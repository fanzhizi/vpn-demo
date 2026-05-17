//! Windows VPN Demo - 使用 wintun.dll + netstack-smoltcp + SOCKS5
//!
//! # 整体架构
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Windows VPN Demo                          │
//! │                                                             │
//! │  ┌──────────────┐   ┌──────────────┐   ┌───────────────┐   │
//! │  │  Slint UI    │   │  tun.rs      │   │  main.rs      │   │
//! │  │  (共享界面)   │   │  (TUN 管理)   │   │  (核心逻辑)   │   │
//! │  └──────┬───────┘   └──────┬───────┘   └───────┬───────┘   │
//! │         │                  │                    │           │
//! │         │ 回调         wintun.dll          netstack         │
//! │         │                  │              + SOCKS5          │
//! │         ▼                  ▼                    ▼           │
//! │  ┌────────────────────────────────────────────────────┐     │
//! │  │              tokio async runtime                    │     │
//! │  └────────────────────────────────────────────────────┘     │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # 关键设计
//!
//! 1. **共享 UI**：Slint 界面定义在 vpn-app/ui/main.slint，iOS/Android/Windows 共用
//! 2. **socket bind 方案**：SOCKS5 连接绑定到物理网卡 IP，而非用路由排除
//!    - 这是 Windows 版本的「protect」机制，等价于 Android 的 VpnService.protect(fd)
//!    - iOS 通过进程隔离天然避免回环，不需要额外处理
//! 3. **线程模型**：Slint UI 在主线程，VPN 逻辑在子线程 + tokio runtime
//! 4. **定时器轮询**：Slint 不支持跨线程直接操作 UI，用 Timer 轮询状态更新

/// Windows TUN 模块（仅在 Windows 平台编译）
#[cfg(windows)]
mod tun;

/// 引入 Slint 编译生成的 UI 组件（MainWindow 等）
/// build.rs 中 slint_build::compile("../vpn-app/ui/main.slint") 生成
slint::include_modules!();

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::net::SocketAddr;
use log::{info, error};
use tokio::sync::mpsc;
use futures::{SinkExt, StreamExt};

fn main() {
    // 初始化日志，默认级别 info，可通过 RUST_LOG 环境变量覆盖
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    // 创建 Slint 窗口实例（UI 定义在 vpn-app/ui/main.slint）
    let window = MainWindow::new().unwrap();
    // 共享的运行状态标志：true=VPN 运行中，false=已停止
    // 使用 AtomicBool 因为多个线程需要读写：UI 线程、VPN 线程、TUN 读写线程
    let running = Arc::new(AtomicBool::new(false));

    // ========== 连接按钮回调 ==========
    // Slint 的 callback 在 UI 主线程中触发，VPN 启动是阻塞操作，
    // 所以 spawn 一个子线程来运行，避免阻塞 UI
    let running_c = running.clone();
    window.on_connect_clicked(move |addr| {
        let addr_str = addr.to_string();
        // 验证地址格式：必须是 ip:port 格式
        if addr_str.parse::<SocketAddr>().is_err() {
            return;
        }
        let running = running_c.clone();
        running.store(true, Ordering::SeqCst);

        // 在子线程中启动 VPN，避免阻塞 Slint UI 主线程
        std::thread::spawn(move || {
            if let Err(e) = start_vpn(&addr_str, running.clone()) {
                error!("VPN start failed: {}", e);
                running.store(false, Ordering::SeqCst);
            }
        });
    });

    // ========== 断开按钮回调 ==========
    // 只需设置 running=false，所有线程会检测到并自行退出
    // TUN adapter drop → 路由自动清理
    let running_d = running.clone();
    window.on_disconnect_clicked(move || {
        running_d.store(false, Ordering::SeqCst);
    });

    // ========== 测试按钮回调 ==========
    // 不经过 TUN，直接连接 SOCKS5 代理测试连通性
    window.on_test_clicked(move |addr| {
        let addr_str = addr.to_string();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            match rt.block_on(test_socks5(&addr_str)) {
                Ok(s) => info!("Test OK: {}", s),
                Err(e) => error!("Test FAIL: {}", e),
            }
        });
    });

    // ========== 定时器轮询 UI 状态 ==========
    // Slint 的线程安全模型：UI 组件只能在 UI 线程中操作。
    // VPN 状态（running）在其他线程中改变，不能直接调用 set_connected()。
    // 解决方案：用 Slint Timer 每 500ms 轮询 running 状态，在 UI 线程中更新。
    // as_weak() 获取弱引用，避免循环引用导致窗口无法关闭。
    let running_ui = running.clone();
    let window_weak = window.as_weak();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
        if let Some(w) = window_weak.upgrade() {
            let is_running = running_ui.load(Ordering::SeqCst);
            // 只在状态变化时更新 UI，避免不必要的重绘
            if w.get_connected() != is_running {
                w.set_connected(is_running);
                w.set_status_text(if is_running { "Connected".into() } else { "Disconnected".into() });
            }
        }
    });

    // 运行 Slint 事件循环（阻塞直到窗口关闭）
    window.run().unwrap();

    // 窗口关闭后，确保 VPN 停止
    // 这会触发所有线程退出，adapter drop，路由清理
    running.store(false, Ordering::SeqCst);
}

/// 启动 VPN 核心逻辑。
///
/// 整体流程：
/// 1. 创建两对 channel（TUN↔netstack 的双向通信）
/// 2. 启动 TUN 设备（Windows）或进入 mock 模式（非 Windows）
/// 3. 启动 netstack 事件循环处理数据包
fn start_vpn(socks5_addr: &str, running: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = socks5_addr.parse()?;
    info!("Starting VPN, SOCKS5: {}", socks5_addr);

    // 创建 tokio 运行时（VPN 核心逻辑使用 async）
    let rt = tokio::runtime::Runtime::new()?;

    // 两对 channel 实现 TUN ↔ netstack 的双向数据传输：
    // tun_tx/tun_rx: TUN 读取的原始 IP 包 → netstack（入站方向）
    // stack_tx/stack_rx: netstack 生成的响应包 → TUN（出站方向）
    let (tun_tx, mut tun_rx) = mpsc::channel::<Vec<u8>>(1024);
    let (stack_tx, stack_rx) = mpsc::channel::<Vec<u8>>(1024);

    // Windows 平台：启动 wintun TUN 设备
    #[cfg(windows)]
    {
        let mut config = tun::win::TunConfig::default();
        config.socks5_host = socket_addr.ip().to_string();
        // start_tun 内部创建适配器、配置 IP/DNS/路由、启动读写线程
        tun::win::start_tun(config, running.clone(), tun_tx, stack_rx)?;
    }

    // 非 Windows 平台：mock 模式（用于开发调试）
    // drop 掉 channel 的发送端，netstack 会立即收到 None
    #[cfg(not(windows))]
    {
        info!("Non-Windows: TUN not available, mock mode");
        drop(tun_tx);
        drop(stack_rx);
    }

    // 阻塞运行 netstack 事件循环，直到 running=false
    let running2 = running.clone();
    rt.block_on(async move {
        if let Err(e) = run_netstack(socket_addr, tun_rx, stack_tx, running2).await {
            error!("Netstack error: {}", e);
        }
    });

    Ok(())
}

/// netstack 事件循环：接收 TUN 数据包，解析 TCP/UDP，转发到 SOCKS5 或直接发送。
///
/// # 数据流
///
/// ```text
/// TUN → tun_rx → [to_stack] → netstack → TCP listener / UDP socket
///                                  │
///                                  ▼
///                             handle_tcp() → socket bind → SOCKS5 代理
///                             forward_udp() → socket bind → 直接转发
///                                  │
///                                  ▼
///                             netstack → [from_stack] → tun_tx → TUN
/// ```
async fn run_netstack(
    socks5_addr: SocketAddr,
    mut tun_rx: mpsc::Receiver<Vec<u8>>,
    tun_tx: mpsc::Sender<Vec<u8>>,
    running: Arc<AtomicBool>,
) -> std::io::Result<()> {
    use netstack_smoltcp::StackBuilder;

    // 构建 netstack：一个用户态 TCP/IP 协议栈
    // 它解析 TUN 收到的原始 IP 包，提取出 TCP 流和 UDP 数据报
    let (stack, runner, udp_socket, tcp_listener) = StackBuilder::default()
        .enable_tcp(true)       // 启用 TCP 协议处理
        .enable_udp(true)       // 启用 UDP 协议处理
        .enable_icmp(true)      // 启用 ICMP（ping 等）
        .tcp_buffer_size(65535) // TCP 收发缓冲区大小
        .udp_buffer_size(65535) // UDP 收发缓冲区大小
        .stack_buffer_size(1024)// 内部数据包缓冲队列长度
        .build()?;

    let mut tcp_listener = tcp_listener.unwrap();
    let udp_socket = udp_socket.unwrap();
    let (mut udp_rx, mut udp_tx) = udp_socket.split();

    // runner 负责 netstack 内部的定时器和状态机驱动
    if let Some(runner) = runner {
        tokio::spawn(async move { let _ = runner.await; });
    }

    // 中间 channel：用于在 TUN channel 和 netstack 之间桥接
    // 因为 netstack 的 send/next 接口是 Stream/Sink trait，
    // 需要通过 select! 驱动，不能直接用 mpsc channel
    let (to_stack_tx, mut to_stack_rx) = mpsc::channel::<Vec<u8>>(1024);
    let (from_stack_tx, mut from_stack_rx) = mpsc::channel::<Vec<u8>>(1024);

    // Stack 双向驱动任务：
    // - 从 to_stack_rx 接收数据包，送入 stack（入站）
    // - 从 stack 读取处理后的数据包，送到 from_stack_tx（出站）
    // biased: 优先处理入站方向，确保输入不积压
    tokio::spawn(async move {
        let mut stack = stack;
        loop {
            tokio::select! {
                biased;
                Some(pkt) = to_stack_rx.recv() => {
                    let _ = stack.send(pkt).await;
                }
                result = stack.next() => {
                    match result {
                        Some(Ok(pkt)) => { if from_stack_tx.send(pkt).await.is_err() { break; } }
                        Some(Err(_)) => {}
                        None => break,
                    }
                }
            }
        }
    });

    // TUN → Stack 桥接：将 TUN 读取的原始 IP 包转发到 netstack
    let running2 = running.clone();
    tokio::spawn(async move {
        while running2.load(Ordering::SeqCst) {
            match tun_rx.recv().await {
                Some(pkt) => { let _ = to_stack_tx.send(pkt).await; }
                None => break,
            }
        }
    });

    // Stack → TUN 桥接：将 netstack 生成的响应包写回 TUN
    let running3 = running.clone();
    tokio::spawn(async move {
        while running3.load(Ordering::SeqCst) {
            match from_stack_rx.recv().await {
                Some(pkt) => { let _ = tun_tx.send(pkt).await; }
                None => break,
            }
        }
    });

    // ========== UDP 转发 ==========
    // UDP 不经过 SOCKS5，直接绑定物理 IP 转发
    // 回复通道：forward_udp 的响应包 → udp_tx → netstack → TUN
    let (udp_reply_tx, mut udp_reply_rx) = mpsc::channel::<(Vec<u8>, SocketAddr, SocketAddr)>(256);
    tokio::spawn(async move {
        while let Some(msg) = udp_reply_rx.recv().await { let _ = udp_tx.send(msg).await; }
    });
    // 接收 netstack 解析出的 UDP 数据报，每个 spawn 一个任务转发
    tokio::spawn(async move {
        while let Some((payload, src, dst)) = udp_rx.next().await {
            let tx = udp_reply_tx.clone();
            tokio::spawn(async move {
                if let Ok(resp) = forward_udp(&payload, dst).await {
                    // 响应包的 src/dst 需要交换（从目标服务器回到原始客户端）
                    let _ = tx.send((resp, dst, src)).await;
                }
            });
        }
    });

    // ========== TCP → SOCKS5 转发 ==========
    // tcp_listener.next() 返回 netstack 解析出的 TCP 连接
    // 每个连接 spawn 一个任务，通过 SOCKS5 代理转发
    while running.load(Ordering::SeqCst) {
        match tcp_listener.next().await {
            Some((stream, src, dst)) => {
                info!("TCP: {} -> {}", src, dst);
                let s = socks5_addr;
                tokio::spawn(async move { let _ = handle_tcp(stream, dst, s).await; });
            }
            None => break,
        }
    }
    Ok(())
}

/// 处理单个 TCP 连接：通过 SOCKS5 代理转发到目标服务器。
///
/// # Windows Socket Bind 方案（核心！）
///
/// 问题：SOCKS5 连接本身也是 TCP，如果不做处理，它的 SYN 包会被 TUN 捕获，
/// 导致路由回环（SOCKS5 → TUN → netstack → SOCKS5 → TUN → ...）
///
/// 解决：使用 socket2 库手动创建 socket，分三步操作：
/// 1. `Socket::new()` - 创建原始 TCP socket
/// 2. `socket.bind(physical_ip:0)` - 绑定到物理网卡 IP（端口 0 = 系统分配）
///    绑定源 IP 后，内核根据源地址选择出口接口 → 走物理网卡而非 TUN
/// 3. `socket.connect(socks5)` - 连接到 SOCKS5 代理
///
/// 为什么不能用 Socks5Stream::connect()？
/// 因为它内部直接 TcpStream::connect()，无法在 connect 前 bind 源地址。
/// 所以用 `Socks5Stream::use_stream()` 在已连接的 socket 上执行 SOCKS5 握手，
/// 再用 `request()` 发送 CONNECT 命令。
async fn handle_tcp(mut local: netstack_smoltcp::TcpStream, dst: SocketAddr, socks5: SocketAddr) -> std::io::Result<()> {
    use fast_socks5::client::{Config, Socks5Stream};

    // ========== Windows: socket bind 方案绕过 TUN ==========
    #[cfg(windows)]
    {
        use std::net::TcpStream as StdTcpStream;

        if let Some(bind_ip) = tun::win::get_bind_address() {
            // 构造绑定地址：物理网卡 IP + 端口 0（让系统自动分配端口）
            let bind_addr: SocketAddr = format!("{}:0", bind_ip).parse().unwrap();

            // 第一步：创建原始 TCP socket（还没连接任何地方）
            let socket = socket2::Socket::new(
                socket2::Domain::IPV4,
                socket2::Type::STREAM,
                Some(socket2::Protocol::TCP),
            )?;
            // 第二步：绑定到物理网卡 IP
            // 这一步决定了此 socket 的出口接口：内核看到源 IP 属于物理网卡，
            // 就会通过物理网卡发送，不走 TUN 设备
            socket.bind(&socket2::SockAddr::from(bind_addr))?;
            // 第三步：连接到 SOCKS5 代理
            // 此时 SYN 包从物理网卡出去，不会被 TUN 捕获
            socket.connect(&socket2::SockAddr::from(socks5))?;
            // 设置非阻塞模式，因为后续要交给 tokio 的 async 运行时管理
            socket.set_nonblocking(true)?;

            // 将 socket2::Socket → std TcpStream → tokio TcpStream
            let std_stream: StdTcpStream = socket.into();
            let tokio_stream = tokio::net::TcpStream::from_std(std_stream)?;

            // 在已连接的 socket 上执行 SOCKS5 握手
            // use_stream: 不创建新连接，而是在已有的 TCP 流上运行 SOCKS5 协议
            // 参数 None: 不使用认证
            let mut socks5_stream = fast_socks5::client::Socks5Stream::use_stream(
                tokio_stream,
                None,
                Config::default(),
            )
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            // 发送 SOCKS5 CONNECT 请求，告诉代理连接到目标服务器
            let target = fast_socks5::util::target_addr::TargetAddr::Ip(dst);
            socks5_stream.request(fast_socks5::Socks5Command::TCPConnect, target)
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            // 双向数据复制：local(netstack) ↔ socks5_stream(代理)
            let _ = tokio::io::copy_bidirectional(&mut local, &mut socks5_stream).await;
            return Ok(());
        }
    }

    // 非 Windows 或无绑定地址时：直接使用 Socks5Stream::connect（不需要 bind）
    let mut remote = Socks5Stream::connect(socks5.to_string(), dst.ip().to_string(), dst.port(), Config::default())
        .await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let _ = tokio::io::copy_bidirectional(&mut local, &mut remote).await;
    Ok(())
}

/// 转发单个 UDP 数据报：直接发送到目标地址（不经过 SOCKS5）。
///
/// UDP 转发同样需要 socket bind 绕过 TUN，原理与 TCP 相同：
/// 绑定物理网卡 IP → 内核按源地址选出口接口 → 不走 TUN
///
/// 注意：UDP 不经过 SOCKS5 代理，而是直接转发。
/// 这是因为 SOCKS5 的 UDP ASSOCIATE 比较复杂，且大多数场景下
/// DNS 查询（主要的 UDP 流量）可以直接转发。
async fn forward_udp(payload: &[u8], dst: SocketAddr) -> std::io::Result<Vec<u8>> {
    // Windows: 绑定物理网卡 IP，防止 UDP 走 TUN 回环
    // 逻辑与 TCP 的 socket bind 完全一致，只是 Type::DGRAM（UDP）
    #[cfg(windows)]
    let socket = {
        if let Some(bind_ip) = tun::win::get_bind_address() {
            let bind_addr: SocketAddr = format!("{}:0", bind_ip).parse().unwrap();
            let sock = socket2::Socket::new(
                socket2::Domain::IPV4,
                socket2::Type::DGRAM,       // UDP 数据报类型
                Some(socket2::Protocol::UDP),
            )?;
            sock.bind(&socket2::SockAddr::from(bind_addr))?;
            sock.set_nonblocking(true)?;
            // socket2 → std UdpSocket → tokio UdpSocket
            let std_socket: std::net::UdpSocket = sock.into();
            tokio::net::UdpSocket::from_std(std_socket)?
        } else {
            tokio::net::UdpSocket::bind("0.0.0.0:0").await?
        }
    };

    // 非 Windows：直接绑定 0.0.0.0:0，不需要特殊处理
    #[cfg(not(windows))]
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;

    socket.send_to(payload, dst).await?;
    let mut buf = vec![0u8; 4096];
    let (n, _) = tokio::time::timeout(std::time::Duration::from_secs(5), socket.recv_from(&mut buf))
        .await.map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout"))??;
    buf.truncate(n);
    Ok(buf)
}

async fn test_socks5(addr: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use fast_socks5::client::{Config, Socks5Stream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut s = Socks5Stream::connect(addr.to_string(), "httpbin.org".to_string(), 80, Config::default()).await?;
    s.write_all(b"GET /ip HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n").await?;
    let mut buf = vec![0u8; 4096];
    let n = tokio::time::timeout(std::time::Duration::from_secs(10), s.read(&mut buf)).await.map_err(|_| "timeout")??;
    Ok(String::from_utf8_lossy(&buf[..n]).lines().next().unwrap_or("").to_string())
}
