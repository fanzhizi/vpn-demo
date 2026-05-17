//! Windows TUN 设备管理，基于 wintun.dll。
//!
//! Wintun 是 WireGuard 项目提供的高性能用户态 TUN 驱动，
//! 不需要安装 TAP-Windows 等传统驱动。
//!
//! 工作流程：
//! 1. 加载 wintun.dll
//! 2. 创建虚拟网络适配器
//! 3. 配置 IP 地址和路由（通过 netsh 命令）
//! 4. 启动 Session 进行数据包收发

#[cfg(windows)]
pub mod win {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use log::{info, warn, error};
    use tokio::sync::mpsc;

    /// TUN 设备配置
    pub struct TunConfig {
        /// TUN 适配器名称（在网络设置中显示）
        pub adapter_name: String,
        /// TUN 设备的 IP 地址
        pub address: String,
        /// 子网掩码位数
        pub prefix_len: u8,
        /// DNS 服务器
        pub dns: Vec<String>,
        /// 需要排除的 IP（SOCKS5 服务器地址，走物理网卡）
        pub excluded_ips: Vec<String>,
    }

    impl Default for TunConfig {
        fn default() -> Self {
            Self {
                adapter_name: "VPNDemo".to_string(),
                address: "10.0.0.2".to_string(),
                prefix_len: 24,
                dns: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
                excluded_ips: vec![],
            }
        }
    }

    /// 创建 wintun 适配器并启动数据包收发。
    ///
    /// 返回：
    /// - inbound_tx: 向 tunnel-core 发送从 TUN 读取的 IP 包
    /// - outbound_rx: 从 tunnel-core 接收要写入 TUN 的 IP 包
    pub fn start_tun(
        config: TunConfig,
        running: Arc<AtomicBool>,
        inbound_tx: mpsc::Sender<Vec<u8>>,
        mut outbound_rx: mpsc::Receiver<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 加载 wintun.dll（必须在可执行文件同目录或 PATH 中）
        let wintun = unsafe { wintun::load()? };
        info!("wintun.dll loaded");

        // 创建适配器（如果同名适配器已存在会复用）
        let adapter = wintun::Adapter::create(
            &wintun,
            &config.adapter_name,
            "VPNDemo Tunnel",
            None, // 不指定 GUID，自动生成
        )?;
        info!("Adapter '{}' created", config.adapter_name);

        // 配置 IP 地址（通过 netsh 命令，最简单可靠的方式）
        let addr_cmd = format!(
            "netsh interface ip set address \"{}\" static {} {} gateway=none",
            config.adapter_name, config.address,
            prefix_to_mask(config.prefix_len)
        );
        run_cmd(&addr_cmd);

        // 配置 DNS
        for (i, dns) in config.dns.iter().enumerate() {
            let dns_cmd = if i == 0 {
                format!(
                    "netsh interface ip set dns \"{}\" static {}",
                    config.adapter_name, dns
                )
            } else {
                format!(
                    "netsh interface ip add dns \"{}\" {} index={}",
                    config.adapter_name, dns, i + 1
                )
            };
            run_cmd(&dns_cmd);
        }

        // 添加默认路由：所有流量走 TUN
        let route_cmd = format!(
            "netsh interface ip add route 0.0.0.0/0 \"{}\" {} metric=5",
            config.adapter_name, config.address
        );
        run_cmd(&route_cmd);

        // 排除 SOCKS5 服务器的 IP（走物理网卡，防止回环）
        // Windows 没有 protect(fd)，需要通过路由表排除
        for ip in &config.excluded_ips {
            // 获取默认网关
            if let Some(gateway) = get_default_gateway() {
                let exclude_cmd = format!(
                    "route add {} mask 255.255.255.255 {} metric 1",
                    ip, gateway
                );
                run_cmd(&exclude_cmd);
            }
        }

        // 启动 wintun session（ring buffer 大小 0x400000 = 4MB）
        let session = Arc::new(adapter.start_session(0x400000)?);
        info!("Wintun session started");

        // 读线程：TUN → tunnel-core
        let session_read = session.clone();
        let running_read = running.clone();
        std::thread::spawn(move || {
            let mut count: u64 = 0;
            while running_read.load(Ordering::SeqCst) {
                // 阻塞等待一个 IP 包（最多等 1 秒）
                match session_read.receive_blocking() {
                    Ok(packet) => {
                        count += 1;
                        let data = packet.bytes().to_vec();
                        if count <= 5 || count % 500 == 0 {
                            info!("TUN read #{}: {} bytes", count, data.len());
                        }
                        let _ = inbound_tx.blocking_send(data);
                    }
                    Err(e) => {
                        if running_read.load(Ordering::SeqCst) {
                            warn!("TUN read error: {}", e);
                        }
                        break;
                    }
                }
            }
            info!("TUN read thread ended, packets={}", count);
        });

        // 写线程：tunnel-core → TUN
        let session_write = session.clone();
        let running_write = running.clone();
        std::thread::spawn(move || {
            let mut count: u64 = 0;
            while running_write.load(Ordering::SeqCst) {
                match outbound_rx.blocking_recv() {
                    Some(data) => {
                        count += 1;
                        if count <= 5 || count % 500 == 0 {
                            info!("TUN write #{}: {} bytes", count, data.len());
                        }
                        // 分配 wintun 发送缓冲区并拷贝数据
                        match session_write.allocate_send_packet(data.len() as u16) {
                            Ok(mut send_packet) => {
                                send_packet.bytes_mut().copy_from_slice(&data);
                                session_write.send_packet(send_packet);
                            }
                            Err(e) => {
                                warn!("TUN write alloc error: {}", e);
                            }
                        }
                    }
                    None => break,
                }
            }
            info!("TUN write thread ended, packets={}", count);
        });

        Ok(())
    }

    /// 前缀长度转子网掩码字符串
    fn prefix_to_mask(prefix: u8) -> String {
        let mask: u32 = if prefix >= 32 {
            0xFFFFFFFF
        } else {
            !((1u32 << (32 - prefix)) - 1)
        };
        format!(
            "{}.{}.{}.{}",
            (mask >> 24) & 0xFF,
            (mask >> 16) & 0xFF,
            (mask >> 8) & 0xFF,
            mask & 0xFF
        )
    }

    /// 执行系统命令
    fn run_cmd(cmd: &str) {
        info!("Running: {}", cmd);
        let output = std::process::Command::new("cmd")
            .args(["/C", cmd])
            .output();
        match output {
            Ok(o) => {
                if !o.status.success() {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    warn!("Command failed: {}", stderr.trim());
                }
            }
            Err(e) => error!("Failed to run command: {}", e),
        }
    }

    /// 获取默认网关 IP（通过解析 route print）
    fn get_default_gateway() -> Option<String> {
        let output = std::process::Command::new("cmd")
            .args(["/C", "route print 0.0.0.0"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        // 解析 route print 输出找默认网关
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == "0.0.0.0" && parts[1] == "0.0.0.0" {
                return Some(parts[2].to_string());
            }
        }
        None
    }
}
