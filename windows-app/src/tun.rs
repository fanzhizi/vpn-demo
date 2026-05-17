//! Windows TUN 设备管理，基于 wintun.dll。
//!
//! 路由策略：不修改系统路由表！
//! - TUN 默认路由通过 CreateIpForwardEntry2 添加（绑定 TUN 接口 LUID，
//!   TUN 适配器销毁时系统自动清理）
//! - SOCKS5 连接通过 socket bind 绑定到物理网卡出去（类似 Android protect）
//!
//! 这样即使程序崩溃，也不会残留任何路由。

#[cfg(windows)]
pub mod win {
    use std::net::Ipv4Addr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use log::{info, warn, error};
    use tokio::sync::mpsc;
    use windows::Win32::NetworkManagement::IpHelper::*;
    use windows::Win32::Networking::WinSock::*;

    /// 物理网卡信息，用于 SOCKS5 socket 绑定
    static BIND_ADDRESS: std::sync::RwLock<Option<Ipv4Addr>> = std::sync::RwLock::new(None);

    /// 获取绑定地址（供 main.rs 中 SOCKS5 连接使用）
    pub fn get_bind_address() -> Option<Ipv4Addr> {
        BIND_ADDRESS.read().unwrap().clone()
    }

    pub struct TunConfig {
        pub adapter_name: String,
        pub address: String,
        pub prefix_len: u8,
        pub dns: Vec<String>,
        pub socks5_host: String,
    }

    impl Default for TunConfig {
        fn default() -> Self {
            Self {
                adapter_name: "VPNDemo".to_string(),
                address: "10.0.0.2".to_string(),
                prefix_len: 24,
                dns: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
                socks5_host: String::new(),
            }
        }
    }

    /// 获取物理网卡的本地 IP 地址（用于 socket 绑定）
    fn get_physical_adapter_ip() -> Option<Ipv4Addr> {
        let output = std::process::Command::new("cmd")
            .args(["/C", "route print 0.0.0.0"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // route print 格式: Network Netmask Gateway Interface Metric
            if parts.len() >= 5
                && parts[0] == "0.0.0.0"
                && parts[1] == "0.0.0.0"
                && parts[2] != "On-link"
            {
                // parts[3] 是物理网卡的 IP
                if let Ok(ip) = parts[3].parse::<Ipv4Addr>() {
                    return Some(ip);
                }
            }
        }
        None
    }

    /// Win32 API: 添加路由（绑定到 TUN 接口 LUID，接口销毁时自动清理）
    fn add_route_api(dest: Ipv4Addr, prefix_len: u8, next_hop: Ipv4Addr, luid: u64) -> Result<(), String> {
        unsafe {
            let mut row: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
            InitializeIpForwardEntry(&mut row);

            row.InterfaceLuid.Value = luid;
            row.DestinationPrefix.Prefix.si_family = AF_INET;
            row.DestinationPrefix.Prefix.Ipv4.sin_addr = in_addr_from_ipv4(dest);
            row.DestinationPrefix.PrefixLength = prefix_len;
            row.NextHop.si_family = AF_INET;
            row.NextHop.Ipv4.sin_addr = in_addr_from_ipv4(next_hop);
            row.Metric = 5;
            // PROTO_IP_NETMGMT = 3
            row.Protocol = std::mem::transmute(3i32);

            let result = CreateIpForwardEntry2(&row);
            if result.is_ok() {
                Ok(())
            } else {
                Err(format!("CreateIpForwardEntry2 failed: {:?}", result))
            }
        }
    }

    fn in_addr_from_ipv4(ip: Ipv4Addr) -> IN_ADDR {
        let octets = ip.octets();
        IN_ADDR {
            S_un: IN_ADDR_0 {
                S_addr: u32::from_ne_bytes(octets),
            },
        }
    }

    pub fn start_tun(
        config: TunConfig,
        running: Arc<AtomicBool>,
        inbound_tx: mpsc::Sender<Vec<u8>>,
        mut outbound_rx: mpsc::Receiver<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 获取物理网卡 IP（在创建 TUN 之前！）
        let physical_ip = get_physical_adapter_ip();
        info!("Physical adapter IP: {:?}", physical_ip);

        // 保存绑定地址供 SOCKS5 连接使用
        if let Some(ip) = physical_ip {
            *BIND_ADDRESS.write().unwrap() = Some(ip);
        }

        let wintun = unsafe { wintun::load()? };
        info!("wintun.dll loaded");

        let adapter = wintun::Adapter::create(
            &wintun,
            &config.adapter_name,
            "VPNDemo Tunnel",
            None,
        )?;
        info!("Adapter '{}' created", config.adapter_name);

        let adapter_luid = adapter.get_luid();
        let luid_value: u64 = unsafe { std::mem::transmute(adapter_luid) };

        // 配置 TUN IP 地址（netsh，IP 配置 API 太复杂不必要）
        let cmd = format!(
            "netsh interface ip set address \"{}\" static {} {}",
            config.adapter_name, config.address,
            prefix_to_mask(config.prefix_len)
        );
        let _ = std::process::Command::new("cmd").args(["/C", &cmd]).output();

        // 配置 DNS
        for (i, dns) in config.dns.iter().enumerate() {
            let cmd = if i == 0 {
                format!("netsh interface ip set dns \"{}\" static {}", config.adapter_name, dns)
            } else {
                format!("netsh interface ip add dns \"{}\" {} index={}", config.adapter_name, dns, i + 1)
            };
            let _ = std::process::Command::new("cmd").args(["/C", &cmd]).output();
        }

        // 通过 API 添加 TUN 默认路由（绑定 TUN LUID，适配器销毁时自动清理）
        // 用 0.0.0.0/1 + 128.0.0.0/1 代替 0.0.0.0/0，比默认路由更具体但不覆盖它
        let tun_addr: Ipv4Addr = config.address.parse().unwrap_or(Ipv4Addr::new(10, 0, 0, 2));

        match add_route_api(Ipv4Addr::new(0, 0, 0, 0), 1, tun_addr, luid_value) {
            Ok(()) => info!("Route 0.0.0.0/1 via TUN added (API, auto-cleanup)"),
            Err(e) => error!("Failed to add route 0.0.0.0/1: {}", e),
        }
        match add_route_api(Ipv4Addr::new(128, 0, 0, 0), 1, tun_addr, luid_value) {
            Ok(()) => info!("Route 128.0.0.0/1 via TUN added (API, auto-cleanup)"),
            Err(e) => error!("Failed to add route 128.0.0.0/1: {}", e),
        }

        // 不添加任何排除路由！SOCKS5 连接通过 socket bind 绑定到物理网卡。
        info!("No system route exclusions needed - SOCKS5 uses socket bind instead");

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
                            Err(e) => warn!("TUN write alloc error: {}", e),
                        }
                    }
                    None => break,
                }
            }
            info!("TUN write thread ended, packets={}", count);
        });

        // wintun Adapter 在 drop 时自动销毁 TUN 适配器
        // → 绑定到 LUID 的路由被 Windows 自动删除
        // → 无需手动清理路由！

        // 保持 adapter 存活（move 到后台线程）
        let running_adapter = running.clone();
        std::thread::spawn(move || {
            while running_adapter.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            // running 变 false 时，adapter 在这里被 drop → TUN 销毁 → 路由自动清理
            drop(adapter);
            info!("TUN adapter dropped, routes auto-cleaned by OS");
        });

        Ok(())
    }

    fn prefix_to_mask(prefix: u8) -> String {
        let mask: u32 = if prefix >= 32 { 0xFFFFFFFF } else { !((1u32 << (32 - prefix)) - 1) };
        format!("{}.{}.{}.{}", (mask >> 24) & 0xFF, (mask >> 16) & 0xFF, (mask >> 8) & 0xFF, mask & 0xFF)
    }
}
