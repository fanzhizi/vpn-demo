//! Windows TUN 设备管理，基于 wintun.dll。
//!
//! 注意：Windows 上路由修改是系统级的，必须在 VPN 断开时清理，
//! 否则会导致网络不可用（即使程序已退出）。

#[cfg(windows)]
pub mod win {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use log::{info, warn, error};
    use tokio::sync::mpsc;

    pub struct TunConfig {
        pub adapter_name: String,
        pub address: String,
        pub prefix_len: u8,
        pub dns: Vec<String>,
        /// SOCKS5 服务器 IP，需要排除在 TUN 路由之外
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

    /// 记录所有添加的路由，用于清理
    struct RouteCleanup {
        adapter_name: String,
        excluded_ips: Vec<String>,
        default_gateway: Option<String>,
    }

    impl RouteCleanup {
        /// 清理所有添加的路由和适配器配置
        fn cleanup(&self) {
            info!("Cleaning up routes...");

            // 删除 TUN 默认路由
            let cmd = format!(
                "netsh interface ip delete route 0.0.0.0/0 \"{}\"",
                self.adapter_name
            );
            run_cmd(&cmd);

            // 删除排除的 IP 路由
            for ip in &self.excluded_ips {
                let cmd = format!("route delete {} mask 255.255.255.255", ip);
                run_cmd(&cmd);
            }

            // 恢复 DNS（设回 DHCP）
            let cmd = format!(
                "netsh interface ip set dns \"{}\" dhcp",
                self.adapter_name
            );
            run_cmd(&cmd);

            info!("Routes cleaned up");
        }
    }

    impl Drop for RouteCleanup {
        fn drop(&mut self) {
            self.cleanup();
        }
    }

    pub fn start_tun(
        config: TunConfig,
        running: Arc<AtomicBool>,
        inbound_tx: mpsc::Sender<Vec<u8>>,
        mut outbound_rx: mpsc::Receiver<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let wintun = unsafe { wintun::load()? };
        info!("wintun.dll loaded");

        let adapter = wintun::Adapter::create(
            &wintun,
            &config.adapter_name,
            "VPNDemo Tunnel",
            None,
        )?;
        info!("Adapter '{}' created", config.adapter_name);

        // 先获取默认网关（在添加 TUN 路由之前！）
        let default_gateway = get_default_gateway();
        info!("Default gateway: {:?}", default_gateway);

        // 配置 IP 地址
        let cmd = format!(
            "netsh interface ip set address \"{}\" static {} {}",
            config.adapter_name, config.address,
            prefix_to_mask(config.prefix_len)
        );
        run_cmd(&cmd);

        // 配置 DNS
        for (i, dns) in config.dns.iter().enumerate() {
            let cmd = if i == 0 {
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
            run_cmd(&cmd);
        }

        // 排除 SOCKS5 服务器 IP（必须在添加默认路由之前！）
        // 走物理网卡的默认网关，metric 1 优先级高于 TUN 的 metric 5
        if let Some(ref gw) = default_gateway {
            for ip in &config.excluded_ips {
                let cmd = format!(
                    "route add {} mask 255.255.255.255 {} metric 1",
                    ip, gw
                );
                run_cmd(&cmd);
            }
        } else {
            warn!("No default gateway found, SOCKS5 exclusion may not work!");
        }

        // 添加默认路由：所有流量走 TUN（metric 5，低于排除路由的 metric 1）
        let cmd = format!(
            "netsh interface ip add route 0.0.0.0/0 \"{}\" {} metric=5",
            config.adapter_name, config.address
        );
        run_cmd(&cmd);

        // 创建路由清理器（Drop 时自动清理）
        let route_cleanup = Arc::new(std::sync::Mutex::new(Some(RouteCleanup {
            adapter_name: config.adapter_name.clone(),
            excluded_ips: config.excluded_ips.clone(),
            default_gateway: default_gateway.clone(),
        })));

        // 注册 Ctrl+C 清理（防止异常退出时路由残留）
        let cleanup_ctrlc = route_cleanup.clone();
        let _ = ctrlc::set_handler(move || {
            if let Ok(mut guard) = cleanup_ctrlc.lock() {
                if let Some(cleanup) = guard.take() {
                    cleanup.cleanup();
                }
            }
            std::process::exit(0);
        });

        // 启动 wintun session
        let session = Arc::new(adapter.start_session(0x400000)?);
        info!("Wintun session started");

        // 读线程：TUN → tunnel-core
        let session_read = session.clone();
        let running_read = running.clone();
        std::thread::spawn(move || {
            let mut count: u64 = 0;
            while running_read.load(Ordering::SeqCst) {
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
        let cleanup_write = route_cleanup.clone();
        std::thread::spawn(move || {
            let mut count: u64 = 0;
            while running_write.load(Ordering::SeqCst) {
                match outbound_rx.blocking_recv() {
                    Some(data) => {
                        count += 1;
                        if count <= 5 || count % 500 == 0 {
                            info!("TUN write #{}: {} bytes", count, data.len());
                        }
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

            // 写线程结束时清理路由
            if let Ok(mut guard) = cleanup_write.lock() {
                if let Some(cleanup) = guard.take() {
                    cleanup.cleanup();
                }
            }
        });

        Ok(())
    }

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

    fn run_cmd(cmd: &str) {
        info!("Running: {}", cmd);
        let output = std::process::Command::new("cmd")
            .args(["/C", cmd])
            .output();
        match output {
            Ok(o) => {
                if !o.status.success() {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    if !stderr.trim().is_empty() {
                        warn!("Command stderr: {}", stderr.trim());
                    }
                }
            }
            Err(e) => error!("Failed to run command: {}", e),
        }
    }

    fn get_default_gateway() -> Option<String> {
        let output = std::process::Command::new("cmd")
            .args(["/C", "route print 0.0.0.0"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // route print 格式: Network  Netmask  Gateway  Interface  Metric
            if parts.len() >= 5
                && parts[0] == "0.0.0.0"
                && parts[1] == "0.0.0.0"
                && parts[2] != "On-link"
            {
                return Some(parts[2].to_string());
            }
        }
        None
    }
}
