//! Windows TUN 设备管理，基于 wintun.dll。
//!
//! # 整体架构
//!
//! 本模块负责 Windows 平台的 TUN 虚拟网卡管理，核心组件：
//! - **wintun.dll**：WireGuard 项目提供的用户态 TUN 驱动，无需安装 TAP 适配器
//! - **Win32 API**：通过 IP Helper API 配置 IP 地址、路由、DNS
//! - **socket bind**：SOCKS5 连接绑定物理网卡 IP，避免路由回环
//!
//! # 路由绕过策略：为什么用 socket bind 而不是 route 命令
//!
//! 传统方案是用 `route add` 排除 SOCKS5 服务器 IP，但存在严重问题：
//! 1. 程序崩溃时路由残留，导致网络异常
//! 2. SOCKS5 服务器 IP 变化时需要动态更新路由
//! 3. 多个 VPN 客户端的路由可能冲突
//!
//! 本项目采用 **socket bind** 方案（类似 Android 的 protect(fd)）：
//! - 所有 SOCKS5 连接的 socket 绑定到物理网卡的 IP 地址
//! - 内核根据源 IP 地址选择出口接口，天然绕过 TUN
//! - TUN 路由绑定到适配器 LUID，适配器销毁时系统自动清理
//! - 即使程序崩溃，也不会残留任何路由规则
//!
//! # wintun 生命周期
//!
//! ```text
//! load() → create adapter → set IP/DNS/route → start session
//!    → read/write 线程循环 → running=false → drop adapter
//!    → 系统自动清理 TUN 适配器和绑定的路由
//! ```

#[cfg(windows)]
pub mod win {
    use std::net::Ipv4Addr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use log::{info, warn, error};
    use tokio::sync::mpsc;
    // Win32 IP Helper API：提供路由表、IP 地址、接口管理等功能
    use windows::Win32::NetworkManagement::IpHelper::*;
    // NDIS 层定义：NET_LUID_LH 是网络接口的本地唯一标识符
    use windows::Win32::NetworkManagement::Ndis::NET_LUID_LH;
    // WinSock 数据结构：SOCKADDR_INET、IN_ADDR 等网络地址结构体
    use windows::Win32::Networking::WinSock::*;

    /// 物理网卡的 IP 地址，用于 SOCKS5 socket 绑定。
    ///
    /// 使用 RwLock 的原因：
    /// - 写入：start_tun() 在初始化时写入一次（调用 GetBestRoute2 获取物理 IP）
    /// - 读取：main.rs 中的 handle_tcp() 和 forward_udp() 在每次连接时读取
    /// - RwLock 允许多个读者并发访问，适合「写少读多」的场景
    /// - 使用 std::sync::RwLock 而非 tokio 的，因为需要在非 async 上下文中写入
    static BIND_ADDRESS: std::sync::RwLock<Option<Ipv4Addr>> = std::sync::RwLock::new(None);

    /// 获取绑定地址（供 main.rs 中 SOCKS5 连接使用）。
    /// 返回 None 表示尚未初始化或获取物理 IP 失败，此时 SOCKS5 将使用默认路由。
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

    /// 通过 Win32 API (GetBestRoute2) 获取物理网卡的本地 IP 地址。
    ///
    /// # 原理
    ///
    /// GetBestRoute2 是 Windows IP Helper API，功能是查询到达指定目标的「最佳路由」。
    /// 除了返回路由信息（MIB_IPFORWARD_ROW2），还会返回**源地址**（best_source），
    /// 即系统选择的出口接口的 IP 地址。
    ///
    /// # 为什么查询 8.8.8.8
    ///
    /// 8.8.8.8 是 Google DNS，几乎所有网络环境都有到它的路由。
    /// 查询它的最佳路由 → 系统返回连接互联网的物理网卡 IP。
    /// 这个 IP 就是我们 socket bind 的目标：绑定到这个 IP 的 socket
    /// 会直接走物理网卡出去，不经过 TUN 设备。
    ///
    /// # 调用时机
    ///
    /// 必须在创建 TUN 适配器**之前**调用！因为 TUN 创建后会添加默认路由，
    /// 此时 GetBestRoute2 可能返回 TUN 的地址而非物理网卡地址。
    fn get_physical_adapter_ip() -> Option<Ipv4Addr> {
        unsafe {
            // 构造目标地址：8.8.8.8（Google DNS）
            // SOCKADDR_INET 是 Windows 的通用网络地址结构，包含 IPv4/IPv6 联合体
            let dest_addr = SOCKADDR_INET {
                Ipv4: SOCKADDR_IN {
                    sin_family: AF_INET,    // AF_INET = IPv4
                    sin_addr: in_addr_from_ipv4(Ipv4Addr::new(8, 8, 8, 8)),
                    sin_port: 0,            // 端口对路由查询无意义
                    sin_zero: [0; 8],       // 填充字节，必须为零
                },
            };

            // 输出参数：最佳路由条目和对应的源地址
            let mut best_route: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
            let mut best_source: SOCKADDR_INET = std::mem::zeroed();

            // 调用 GetBestRoute2
            // 参数说明：
            //   InterfaceLuid = None: 不限定接口，让系统选择最佳出口
            //   InterfaceIndex = 0: 同上，不限定
            //   SourceAddress = None: 不限定源地址
            //   DestinationAddress = 8.8.8.8: 查询到此地址的路由
            //   Flags = 0: 无特殊标志
            let result = GetBestRoute2(
                None,                   // 不指定接口 LUID，让系统选最佳
                0,                      // InterfaceIndex = 0
                None,                   // 不指定源地址
                &dest_addr,             // 目标：8.8.8.8
                0,                      // 无标志
                &mut best_route,        // [输出] 最佳路由条目
                &mut best_source,       // [输出] 系统选择的源 IP（就是物理网卡 IP）
            );

            if result.is_ok() {
                // 从 SOCKADDR_INET 中提取 IPv4 地址
                // S_un.S_addr 是网络字节序的 u32，用 to_ne_bytes 转为 4 字节
                let src = best_source.Ipv4.sin_addr.S_un.S_addr;
                let octets = src.to_ne_bytes();
                let ip = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);
                info!("GetBestRoute2: physical IP = {}", ip);
                Some(ip)
            } else {
                warn!("GetBestRoute2 failed: {:?}", result);
                None
            }
        }
    }

    /// Win32 API: 通过 CreateUnicastIpAddressEntry 设置适配器 IP 地址。
    ///
    /// # IP 配置原理
    ///
    /// Windows 中配置网络接口 IP 有两种方式：
    /// 1. netsh 命令（需要解析字符串输出，不够可靠）
    /// 2. CreateUnicastIpAddressEntry API（程序化、可靠、推荐）
    ///
    /// # 参数说明
    ///
    /// - `luid`: 网络接口的本地唯一标识符（从 wintun adapter 获取）
    /// - `ip`: 要分配的 IP 地址（例如 10.0.0.2）
    /// - `prefix_len`: 子网前缀长度（例如 24 = 255.255.255.0）
    ///
    /// # MIB_UNICASTIPADDRESS_ROW 关键字段
    ///
    /// - `InterfaceLuid`: 绑定到哪个网络接口
    /// - `Address`: IP 地址（SOCKADDR_INET 格式）
    /// - `OnLinkPrefixLength`: 子网掩码（CIDR 前缀长度）
    /// - `DadState`: 重复地址检测状态，设为 Preferred 表示直接使用，跳过 DAD
    fn set_adapter_ip(luid: u64, ip: Ipv4Addr, prefix_len: u8) -> Result<(), String> {
        unsafe {
            let mut row: MIB_UNICASTIPADDRESS_ROW = std::mem::zeroed();
            // 必须先调用 InitializeUnicastIpAddressEntry 初始化结构体默认值
            InitializeUnicastIpAddressEntry(&mut row);

            row.InterfaceLuid.Value = luid;         // 指定目标网络接口
            row.Address.si_family = AF_INET;        // IPv4 地址族
            row.Address.Ipv4.sin_addr = in_addr_from_ipv4(ip);  // 设置 IP 地址
            row.OnLinkPrefixLength = prefix_len;    // 子网前缀长度，如 24
            row.DadState = IpDadStatePreferred;     // 跳过重复地址检测，直接使用

            let result = CreateUnicastIpAddressEntry(&row);
            if result.is_ok() {
                Ok(())
            } else {
                Err(format!("CreateUnicastIpAddressEntry failed: {:?}", result))
            }
        }
    }

    /// Win32 API: 设置适配器 DNS（通过注册表，Windows 没有专门的 DNS 配置 API）。
    ///
    /// # 为什么 DNS 只能走注册表
    ///
    /// Windows 的 IP Helper API 提供了 IP 地址和路由的管理接口，
    /// 但**没有提供直接设置 DNS 的 API**。DNS 配置存储在注册表中：
    /// `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{GUID}\NameServer`
    ///
    /// # 注册表路径格式
    ///
    /// 每个网络接口在注册表中有一个子键，键名是接口的 GUID（不是 LUID）。
    /// 因此需要先通过 ConvertInterfaceLuidToGuid 将 LUID 转为 GUID。
    ///
    /// 完整路径示例：
    /// `HKLM\SYSTEM\...\Interfaces\{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}\NameServer`
    ///
    /// NameServer 值格式：逗号分隔的 IP 地址字符串，如 "8.8.8.8,8.8.4.4"
    fn set_adapter_dns(luid: u64, dns_servers: &[String]) -> Result<(), String> {
        unsafe {
            // 第一步：将 LUID 转换为 GUID
            // LUID（本地唯一标识符）是 u64 数值，在内存中唯一
            // GUID（全局唯一标识符）是 128 位 UUID，用于注册表键名
            let mut luid_val: NET_LUID_LH = std::mem::zeroed();
            luid_val.Value = luid;

            let mut guid = windows::core::GUID::zeroed();
            let result = ConvertInterfaceLuidToGuid(&luid_val, &mut guid);
            if result.is_err() {
                return Err(format!("ConvertInterfaceLuidToGuid failed: {:?}", result));
            }

            // 将 GUID 格式化为 Windows 标准格式：{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}
            let guid_str = format!(
                "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
                guid.data1, guid.data2, guid.data3,
                guid.data4[0], guid.data4[1], guid.data4[2], guid.data4[3],
                guid.data4[4], guid.data4[5], guid.data4[6], guid.data4[7],
            );

            // 构造注册表路径：每个网络接口的 TCP/IP 参数都在这个路径下
            let reg_path = format!(
                "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\\{}",
                guid_str
            );

            // 将 DNS 服务器列表用逗号拼接：["8.8.8.8", "8.8.4.4"] → "8.8.8.8,8.8.4.4"
            let dns_value = dns_servers.join(",");

            // 打开注册表并写入 NameServer 值
            let hkey = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
            let subkey = hkey.open_subkey_with_flags(&reg_path, winreg::enums::KEY_WRITE)
                .map_err(|e| format!("RegKey open failed: {}", e))?;
            subkey.set_value("NameServer", &dns_value)
                .map_err(|e| format!("RegKey set DNS failed: {}", e))?;

            info!("DNS set via registry: {} -> {}", guid_str, dns_value);
            Ok(())
        }
    }

    /// Win32 API: 通过 CreateIpForwardEntry2 添加路由条目。
    ///
    /// # LUID 绑定与自动清理
    ///
    /// 这是本项目路由管理的核心设计：
    /// - 路由条目通过 `InterfaceLuid` 绑定到 TUN 适配器
    /// - 当 TUN 适配器被销毁（adapter drop）时，Windows 内核**自动删除**
    ///   所有绑定到该 LUID 的路由条目
    /// - 这意味着即使程序崩溃，路由也不会残留在系统中
    ///
    /// 对比 `route add` 命令：用命令添加的路由是全局的，不绑定到特定接口，
    /// 程序崩溃后路由仍然存在，需要手动 `route delete` 清理。
    ///
    /// # MIB_IPFORWARD_ROW2 关键字段
    ///
    /// - `InterfaceLuid`: 绑定到哪个网络接口（路由的出口）
    /// - `DestinationPrefix`: 目标网段（IP + 前缀长度）
    /// - `NextHop`: 下一跳地址（TUN 的 IP 地址）
    /// - `Metric`: 路由优先级，值越小优先级越高
    /// - `Protocol`: 路由来源，3 = PROTO_IP_NETMGMT（手动管理）
    fn add_route_api(dest: Ipv4Addr, prefix_len: u8, next_hop: Ipv4Addr, luid: u64) -> Result<(), String> {
        unsafe {
            let mut row: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
            // 必须先初始化，填充默认值
            InitializeIpForwardEntry(&mut row);

            // 绑定到 TUN 接口 —— 这是自动清理的关键
            row.InterfaceLuid.Value = luid;
            // 目标网段：例如 0.0.0.0/1 表示 0.0.0.0～127.255.255.255
            row.DestinationPrefix.Prefix.si_family = AF_INET;
            row.DestinationPrefix.Prefix.Ipv4.sin_addr = in_addr_from_ipv4(dest);
            row.DestinationPrefix.PrefixLength = prefix_len;
            // 下一跳：TUN 适配器自己的 IP 地址
            row.NextHop.si_family = AF_INET;
            row.NextHop.Ipv4.sin_addr = in_addr_from_ipv4(next_hop);
            // 低 metric 值 = 高优先级，确保流量优先走 TUN
            row.Metric = 5;
            // PROTO_IP_NETMGMT = 3，表示这是通过网络管理程序添加的路由
            row.Protocol = std::mem::transmute(3i32);

            let result = CreateIpForwardEntry2(&row);
            if result.is_ok() {
                Ok(())
            } else {
                Err(format!("CreateIpForwardEntry2 failed: {:?}", result))
            }
        }
    }

    /// 将 Rust 的 Ipv4Addr 转换为 Windows 的 IN_ADDR 结构体。
    ///
    /// IN_ADDR 是 WinSock 中表示 IPv4 地址的结构体，内部用 u32 存储。
    /// S_un.S_addr 使用网络字节序（大端序），而 Ipv4Addr::octets() 返回的
    /// 字节数组已经是网络字节序，所以用 from_ne_bytes（本机字节序）直接转换。
    /// 这是因为 octets [a, b, c, d] 在内存中的布局恰好就是网络字节序。
    fn in_addr_from_ipv4(ip: Ipv4Addr) -> IN_ADDR {
        let octets = ip.octets();
        IN_ADDR {
            S_un: IN_ADDR_0 {
                S_addr: u32::from_ne_bytes(octets),
            },
        }
    }

    /// 启动 TUN 设备，配置 IP/DNS/路由，并启动读写线程。
    ///
    /// # 参数
    ///
    /// - `config`: TUN 配置（适配器名、IP、DNS 等）
    /// - `running`: 共享的运行状态标志，设为 false 时所有线程退出
    /// - `inbound_tx`: TUN 读取的数据包发送到此 channel（TUN → netstack）
    /// - `outbound_rx`: 从此 channel 接收要写入 TUN 的数据包（netstack → TUN）
    ///
    /// # 初始化顺序（顺序很重要！）
    ///
    /// 1. 获取物理网卡 IP（必须在 TUN 创建前，否则路由被 TUN 影响）
    /// 2. 加载 wintun.dll
    /// 3. 创建 TUN 适配器
    /// 4. 配置 IP 地址（CreateUnicastIpAddressEntry）
    /// 5. 配置 DNS（注册表）
    /// 6. 添加默认路由（CreateIpForwardEntry2，绑定 LUID）
    /// 7. 启动 wintun session 和读写线程
    pub fn start_tun(
        config: TunConfig,
        running: Arc<AtomicBool>,
        inbound_tx: mpsc::Sender<Vec<u8>>,
        mut outbound_rx: mpsc::Receiver<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // ========== 第一步：获取物理网卡 IP ==========
        // 必须在创建 TUN 之前调用！创建 TUN 后会添加默认路由，
        // GetBestRoute2 的结果会被新路由影响
        let physical_ip = get_physical_adapter_ip();
        info!("Physical adapter IP: {:?}", physical_ip);

        // 将物理 IP 保存到全局 BIND_ADDRESS，供 main.rs 中的 SOCKS5 连接使用
        if let Some(ip) = physical_ip {
            *BIND_ADDRESS.write().unwrap() = Some(ip);
        }

        // ========== 第二步：加载 wintun.dll ==========
        // wintun.dll 必须和 exe 在同一目录（或系统 PATH 中）
        // load() 内部调用 LoadLibrary 加载 DLL，获取函数指针
        let wintun = unsafe { wintun::load()? };
        info!("wintun.dll loaded");

        // ========== 第三步：创建 TUN 适配器 ==========
        // Adapter::create 会在系统中创建一个虚拟网络接口
        // 需要管理员权限（UAC）才能成功
        let adapter = wintun::Adapter::create(
            &wintun,
            &config.adapter_name,   // 适配器名称，显示在"网络连接"中
            "VPNDemo Tunnel",       // 隧道类型标识
            None,                   // 不指定 GUID，让系统自动分配
        )?;
        info!("Adapter '{}' created", config.adapter_name);

        // 获取适配器的 LUID（本地唯一标识符），后续 API 调用都需要它
        // LUID 是 Windows 内核为每个网络接口分配的 64 位标识符
        let adapter_luid = adapter.get_luid();
        let luid_value: u64 = unsafe { std::mem::transmute(adapter_luid) };

        // ========== 第四步：通过 API 配置 TUN IP 地址 ==========
        let tun_ip: Ipv4Addr = config.address.parse().unwrap_or(Ipv4Addr::new(10, 0, 0, 2));
        if let Err(e) = set_adapter_ip(luid_value, tun_ip, config.prefix_len) {
            error!("Failed to set adapter IP: {}", e);
        } else {
            info!("Adapter IP set to {}/{} (API)", tun_ip, config.prefix_len);
        }

        // ========== 第五步：通过注册表配置 DNS ==========
        // DNS 配置没有专门的 Win32 API，只能通过注册表设置
        // 如果注册表方式失败（权限不足），回退到 netsh 命令
        if let Err(e) = set_adapter_dns(luid_value, &config.dns) {
            warn!("DNS API failed ({}), falling back to netsh", e);
            for (i, dns) in config.dns.iter().enumerate() {
                let cmd = if i == 0 {
                    // 第一个 DNS：set dns（替换现有配置）
                    format!("netsh interface ip set dns \"{}\" static {}", config.adapter_name, dns)
                } else {
                    // 后续 DNS：add dns（追加到列表）
                    format!("netsh interface ip add dns \"{}\" {} index={}", config.adapter_name, dns, i + 1)
                };
                let _ = std::process::Command::new("cmd").args(["/C", &cmd]).output();
            }
        } else {
            info!("DNS configured via API");
        }

        // ========== 第六步：通过 API 添加 TUN 默认路由 ==========
        // 路由策略：用 0.0.0.0/1 + 128.0.0.0/1 代替 0.0.0.0/0
        // 原因：/1 的路由比 /0 更具体（前缀更长），路由匹配时优先级更高
        // 这两条路由加起来覆盖整个 IPv4 地址空间，但不会覆盖系统原有的默认路由
        // 效果：所有流量走 TUN，但原始默认路由保留作为回退
        let tun_addr: Ipv4Addr = config.address.parse().unwrap_or(Ipv4Addr::new(10, 0, 0, 2));

        match add_route_api(Ipv4Addr::new(0, 0, 0, 0), 1, tun_addr, luid_value) {
            Ok(()) => info!("Route 0.0.0.0/1 via TUN added (API, auto-cleanup)"),
            Err(e) => error!("Failed to add route 0.0.0.0/1: {}", e),
        }
        match add_route_api(Ipv4Addr::new(128, 0, 0, 0), 1, tun_addr, luid_value) {
            Ok(()) => info!("Route 128.0.0.0/1 via TUN added (API, auto-cleanup)"),
            Err(e) => error!("Failed to add route 128.0.0.0/1: {}", e),
        }

        // 关键设计：不添加任何排除路由！
        // SOCKS5 连接通过 socket bind 绑定到物理网卡 IP，内核按源地址选接口，
        // 天然绕过 TUN，无需排除路由。
        info!("No system route exclusions needed - SOCKS5 uses socket bind instead");

        // ========== 第七步：启动 wintun session ==========
        // session 是 wintun 的数据包读写接口
        // 0x400000 (4MB) 是环形缓冲区大小，用于存储待读取的数据包
        // session 需要 Arc 共享，因为读线程和写线程都需要访问
        let session = Arc::new(adapter.start_session(0x400000)?);
        info!("Wintun session started");

        // ========== 读线程：TUN → netstack ==========
        // receive_blocking() 阻塞等待 TUN 设备的数据包
        // 收到的数据包是原始 IP 包（没有以太网头），直接送入 netstack 处理
        let session_read = session.clone();
        let running_read = running.clone();
        std::thread::spawn(move || {
            let mut count: u64 = 0;
            while running_read.load(Ordering::SeqCst) {
                match session_read.receive_blocking() {
                    Ok(packet) => {
                        count += 1;
                        let data = packet.bytes().to_vec();
                        // 日志采样：前 5 个包全部打印，之后每 500 个包打印一次
                        if count <= 5 || count % 500 == 0 {
                            info!("TUN read #{}: {} bytes", count, data.len());
                        }
                        // blocking_send 因为这是同步线程，不能用 async send
                        let _ = inbound_tx.blocking_send(data);
                    }
                    Err(e) => {
                        // session 关闭时会返回错误，只有 running=true 时才报警
                        if running_read.load(Ordering::SeqCst) {
                            warn!("TUN read error: {}", e);
                        }
                        break;
                    }
                }
            }
            info!("TUN read thread ended, packets={}", count);
        });

        // ========== 写线程：netstack → TUN ==========
        // 从 channel 接收 netstack 处理后的响应包，写入 TUN 设备
        // wintun 写入流程：allocate_send_packet → 填充数据 → send_packet
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
                        // 两步写入：先分配缓冲区，再填充数据并发送
                        match session_write.allocate_send_packet(data.len() as u16) {
                            Ok(mut send_packet) => {
                                send_packet.bytes_mut().copy_from_slice(&data);
                                session_write.send_packet(send_packet);
                            }
                            Err(e) => warn!("TUN write alloc error: {}", e),
                        }
                    }
                    None => break, // channel 关闭，退出
                }
            }
            info!("TUN write thread ended, packets={}", count);
        });

        // ========== 适配器生命周期管理 ==========
        // wintun::Adapter 实现了 Drop trait：
        //   drop(adapter) → 销毁 TUN 虚拟网卡 → Windows 内核自动删除绑定到该 LUID 的所有路由
        // 这就是「自动清理」的关键：路由是绑定到适配器 LUID 的，适配器消失路由也消失。
        //
        // adapter 需要 move 到后台线程中保持存活，
        // 当 running 标志变为 false 时，线程退出，adapter 被 drop，触发清理。
        let running_adapter = running.clone();
        std::thread::spawn(move || {
            while running_adapter.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            // running 变 false → adapter 在此 drop → TUN 销毁 → 路由自动清理
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
