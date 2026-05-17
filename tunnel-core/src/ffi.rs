//! C FFI 接口层 - 同时服务 iOS (Swift 通过 C 桥接) 和 Android (Java 通过 JNI)。
//!
//! 本模块是 tunnel-core 对外暴露的所有 C ABI 函数的实现。
//! iOS 端由 Swift 通过 Bridging Header 直接调用这些 C 函数；
//! Android 端由 jni_ffi.rs 中的 JNI 包装函数间接调用。
//!
//! 核心组件：
//! - android_protect 子模块：Android 独有的 socket 保护机制
//! - TunnelRuntime 结构体：持有 tokio runtime 和数据通道
//! - run_stack()：核心网络栈逻辑（TCP/UDP 转发）
//! - tunnel_start/stop/feed/read：生命周期和数据通道 API

use std::ffi::CStr;
use std::net::SocketAddr;
use std::os::raw::c_char;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use futures::{SinkExt, StreamExt};
use netstack_smoltcp::StackBuilder;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use tokio::io;
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use log::{info, warn, error};

use crate::TunnelConfig;

/// Android Socket 保护模块。
///
/// 为什么需要 protect：
/// Android VPN 通过路由表将所有流量导向 TUN 设备。但我们的 SOCKS5 代理连接本身
/// 也是网络流量——如果不特殊处理，它也会被路由到 TUN，形成死循环：
///   App 流量 → TUN → netstack → SOCKS5 socket → TUN → netstack → SOCKS5 ... (死循环!)
///
/// 解决方案：VpnService.protect(fd) 给 socket 打标记，使其绕过 VPN 路由。
///
/// 实现参考了 leaf 项目 (eycorsican/leaf) 的设计模式：
///   - RwLock<JavaVM>：缓存 JVM 引用，支持从任意 Rust 线程回调 Java
///   - RwLock<GlobalRef>：持有 VpnService 实例，防止 Java GC 回收
///   - attach_current_thread_permanently：将 Rust 线程附加到 JVM（永久的，不自动 detach）
///
/// 完整调用链：
///   Rust socket2::Socket::new()
///   → protect_socket(fd)
///   → JVM.attach_current_thread_permanently()
///   → env.call_method("protectSocket", fd)
///   → Java TunnelVpnService.protectSocket(fd)
///   → Android VpnService.protect(fd)
///   → Linux 内核 SO_MARK
///   → socket 绕过 VPN 路由
///   → 然后才能安全 connect() 到 SOCKS5 代理
#[cfg(target_os = "android")]
pub mod android_protect {
    use std::os::unix::io::RawFd;
    use std::sync::RwLock;
    use jni::objects::{GlobalRef, JValue};
    use jni::JavaVM;
    use log::{info, warn};

    // 全局 JVM 引用，在 JNI_OnLoad 时设置
    // 使用 RwLock 因为：写入只发生一次（JNI_OnLoad），之后都是只读
    static JVM: RwLock<Option<JavaVM>> = RwLock::new(None);

    // 保护回调信息：VpnService 实例 + 方法名
    // 使用 RwLock 同上：写入在 setProtectSocketCallback，之后只读
    static PROTECT_CALLBACK: RwLock<Option<ProtectCallback>> = RwLock::new(None);

    /// 封装 Java 回调所需的信息
    struct ProtectCallback {
        class: GlobalRef,   // VpnService 实例的全局引用（防止 GC）
        method: String,     // 方法名 "protectSocket"
    }

    /// 保存 JVM 引用。由 JNI_OnLoad 调用，整个进程生命周期只调用一次。
    pub fn set_jvm(vm: JavaVM) {
        *JVM.write().unwrap() = Some(vm);
    }

    /// 设置保护回调。由 jni_ffi.rs 中的 setProtectSocketCallback 调用。
    /// 必须在 tunnel_start 之前调用，否则 SOCKS5 连接无法被 protect。
    pub fn set_protect_callback(class: GlobalRef, method: String) {
        info!("set_protect_callback: method={}", method);
        *PROTECT_CALLBACK.write().unwrap() = Some(ProtectCallback { class, method });
    }

    /// 保护一个 socket fd，使其绕过 VPN 路由。
    ///
    /// 调用时机：在 socket connect() 之前。顺序很重要！
    ///   new socket → protect(fd) → connect()
    /// 如果先 connect 再 protect，SYN 包已经走了 TUN，protect 就晚了。
    ///
    /// 关于 attach_current_thread_permanently：
    /// - Rust 的 tokio 线程不是 Java 线程，不能直接调用 JNI
    /// - attach 将当前线程注册到 JVM，获得 JNIEnv
    /// - "permanently" 表示不会自动 detach（detach 开销大且在 tokio 中不可控）
    /// - 已 attach 的线程再次调用 attach 是空操作（安全）
    pub fn protect_socket(fd: RawFd) -> bool {
        let jvm_guard = JVM.read().unwrap();
        let Some(vm) = jvm_guard.as_ref() else {
            warn!("protect_socket: JVM not set");
            return false;
        };

        let cb_guard = PROTECT_CALLBACK.read().unwrap();
        let Some(cb) = cb_guard.as_ref() else {
            warn!("protect_socket: callback not set");
            return false;
        };

        // 将当前 Rust/tokio 线程附加到 JVM
        // permanently 版本不会在作用域结束时 detach，适合反复调用的场景
        let mut env = match vm.attach_current_thread_permanently() {
            Ok(env) => env,
            Err(e) => {
                warn!("protect_socket: attach thread failed: {}", e);
                return false;
            }
        };

        // 通过 JNI 反射调用 Java 方法：VpnService.protectSocket(int fd) -> boolean
        // "(I)Z" 是 JNI 方法签名：参数 int (I)，返回 boolean (Z)
        match env.call_method(&cb.class, &cb.method, "(I)Z", &[JValue::Int(fd as i32)]) {
            Ok(result) => result.z().unwrap_or(false),
            Err(e) => {
                warn!("protect_socket: call_method failed: {}", e);
                // 调用失败时清除 Java 异常，否则后续 JNI 调用会直接失败
                let _ = env.exception_clear();
                false
            }
        }
    }

    /// JNI_OnLoad - 动态库被 System.loadLibrary() 加载时由 JVM 自动调用。
    ///
    /// 这是获取 JavaVM 引用的最佳时机：
    /// - 每个进程只有一个 JavaVM 实例
    /// - 保存后可从任意线程通过 attach 获取 JNIEnv
    /// - 返回 JNI_VERSION_1_6 告诉 JVM 我们需要的 JNI 版本
    #[no_mangle]
    pub extern "system" fn JNI_OnLoad(
        vm: jni::JavaVM,
        _: *mut std::ffi::c_void,
    ) -> jni::sys::jint {
        set_jvm(vm);
        jni::sys::JNI_VERSION_1_6
    }
}

/// 跨平台 protect_socket 封装。
/// Android：调用 android_protect 模块通过 JNI 回调 VpnService.protect()
/// 非 Android（iOS/桌面）：直接返回 true（iOS 使用 NEPacketTunnelProvider 的进程隔离，不需要 protect）
#[cfg(target_os = "android")]
pub fn protect_socket(fd: i32) -> bool {
    android_protect::protect_socket(fd)
}

#[cfg(not(target_os = "android"))]
pub fn protect_socket(_fd: i32) -> bool {
    true // iOS/桌面平台不需要 protect，直接返回成功
}

/// 初始化日志系统（全局只执行一次）。
/// 根据目标平台选择不同的日志后端：
/// - iOS: oslog（输出到 Console.app，可通过 Xcode 查看）
/// - Android: android_logger（输出到 logcat，tag 为 "tunnel-core"）
/// - 桌面: env_logger（输出到 stderr，可通过 RUST_LOG 环境变量控制级别）
fn init_logger() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        #[cfg(target_os = "ios")]
        {
            let _ = oslog::OsLogger::new("com.vpndemo.app.tunnel.rust")
                .level_filter(log::LevelFilter::Debug)
                .init();
        }
        #[cfg(target_os = "android")]
        {
            // android_logger 将 log crate 的输出转发到 Android logcat
            // tag "tunnel-core" 方便过滤：adb logcat -s tunnel-core
            android_logger::init_once(
                android_logger::Config::default()
                    .with_max_level(log::LevelFilter::Debug)
                    .with_tag("tunnel-core"),
            );
        }
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            let _ = env_logger::try_init();
        }
    });
}

/// 隧道运行时状态 - 持有 tokio runtime 和数据通道的两端。
///
/// 数据流模型：
///   TUN 读线程 → inbound_tx → [channel] → inbound_rx → netstack → 协议处理
///   netstack → outbound_tx → [channel] → outbound_rx → TUN 写线程
///
/// 为什么用 mpsc channel：
/// - 生产者（TUN 线程）和消费者（tokio 任务）在不同线程
/// - channel 提供天然的背压（满了就丢包/阻塞）
/// - 解耦 Java 的阻塞 I/O 线程和 Rust 的 async 运行时
struct TunnelRuntime {
    runtime: Runtime,                              // tokio 多线程运行时
    running: Arc<AtomicBool>,                      // 全局运行标志
    inbound_tx: mpsc::Sender<Vec<u8>>,             // TUN → Rust 方向的发送端
    outbound_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>, // Rust → TUN 方向的接收端（Mutex 因为 try_recv 需要 &mut）
}

// OnceCell 保证 TunnelRuntime 只初始化一次，全局单例
static TUNNEL: OnceCell<TunnelRuntime> = OnceCell::new();

#[no_mangle]
pub unsafe extern "C" fn tunnel_start(socks5_addr: *const c_char) -> bool {
    init_logger();

    info!("tunnel_start called");

    if socks5_addr.is_null() {
        error!("socks5_addr is null");
        return false;
    }

    let addr_str = match CStr::from_ptr(socks5_addr).to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let socket_addr: SocketAddr = match addr_str.parse() {
        Ok(a) => a,
        Err(_) => return false,
    };

    let config = TunnelConfig {
        socks5_addr: socket_addr,
    };

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return false,
    };

    let running = Arc::new(AtomicBool::new(true));

    let (inbound_tx, inbound_rx) = mpsc::channel::<Vec<u8>>(1024);
    let (outbound_tx, outbound_rx) = mpsc::channel::<Vec<u8>>(1024);

    let running_clone = running.clone();

    runtime.spawn(async move {
        if let Err(e) = run_stack(config, inbound_rx, outbound_tx, running_clone).await {
            error!("Stack error: {}", e);
        }
    });

    let _ = TUNNEL.set(TunnelRuntime {
        runtime,
        running,
        inbound_tx,
        outbound_rx: Arc::new(Mutex::new(outbound_rx)),
    });

    true
}

async fn run_stack(
    config: TunnelConfig,
    mut inbound_rx: mpsc::Receiver<Vec<u8>>,
    outbound_tx: mpsc::Sender<Vec<u8>>,
    running: Arc<AtomicBool>,
) -> std::io::Result<()> {
    info!("Building netstack, SOCKS5 proxy: {}", config.socks5_addr);

    let (stack, runner, udp_socket, tcp_listener) = StackBuilder::default()
        .enable_tcp(true)
        .enable_udp(true)
        .enable_icmp(true)
        .tcp_buffer_size(65535)
        .udp_buffer_size(65535)
        .stack_buffer_size(1024)
        .build()?;

    let mut tcp_listener = tcp_listener.expect("TCP listener should be enabled");
    let udp_socket = udp_socket.expect("UDP socket should be enabled");
    let (mut udp_rx, mut udp_tx) = udp_socket.split();

    // Drive the smoltcp internals (handles ICMP + internal polling)
    if let Some(runner) = runner {
        tokio::spawn(async move {
            if let Err(e) = runner.await {
                error!("Runner error: {}", e);
            }
        });
    }

    // Split stack into two halves via channels
    let (to_stack_tx, mut to_stack_rx) = mpsc::channel::<Vec<u8>>(1024);
    let (from_stack_tx, mut from_stack_rx) = mpsc::channel::<Vec<u8>>(1024);

    // Task: own the Stack, feed packets in and pull packets out
    tokio::spawn(async move {
        // We need to pin the stack for Stream/Sink usage
        let mut stack = stack;
        loop {
            tokio::select! {
                biased;
                // Packets from TUN -> feed into stack
                Some(pkt) = to_stack_rx.recv() => {
                    if let Err(e) = stack.send(pkt).await {
                        warn!("Stack send error: {}", e);
                    }
                }
                // Packets from stack -> send to TUN
                result = stack.next() => {
                    match result {
                        Some(Ok(pkt)) => {
                            if from_stack_tx.send(pkt).await.is_err() {
                                break;
                            }
                        }
                        Some(Err(e)) => {
                            warn!("Stack next error: {}", e);
                        }
                        None => break,
                    }
                }
            }
        }
        info!("Stack driver loop ended");
    });

    // Task: Swift inbound_rx -> to_stack_tx
    let running2 = running.clone();
    tokio::spawn(async move {
        while running2.load(Ordering::SeqCst) {
            match inbound_rx.recv().await {
                Some(pkt) => {
                    if to_stack_tx.send(pkt).await.is_err() {
                        break;
                    }
                }
                None => break,
            }
        }
        info!("Inbound forwarder ended");
    });

    // Task: from_stack_rx -> Swift outbound_tx
    let running3 = running.clone();
    tokio::spawn(async move {
        while running3.load(Ordering::SeqCst) {
            match from_stack_rx.recv().await {
                Some(pkt) => {
                    if outbound_tx.send(pkt).await.is_err() {
                        break;
                    }
                }
                None => break,
            }
        }
        info!("Outbound forwarder ended");
    });

    // Task: UDP forwarding (especially DNS)
    // Forward UDP packets directly to destination (not through SOCKS5)
    let (udp_reply_tx, mut udp_reply_rx) = mpsc::channel::<(Vec<u8>, SocketAddr, SocketAddr)>(256);

    // Sub-task: write UDP replies back into the stack
    tokio::spawn(async move {
        while let Some(msg) = udp_reply_rx.recv().await {
            if let Err(e) = udp_tx.send(msg).await {
                warn!("UDP reply write error: {}", e);
            }
        }
    });

    tokio::spawn(async move {
        info!("UDP forwarder started");
        while let Some((payload, src_addr, dst_addr)) = udp_rx.next().await {
            info!("UDP: {} -> {} ({} bytes)", src_addr, dst_addr, payload.len());
            let reply_tx = udp_reply_tx.clone();
            tokio::spawn(async move {
                match forward_udp(&payload, src_addr, dst_addr).await {
                    Ok(response) => {
                        info!("UDP reply from {}: {} bytes", dst_addr, response.len());
                        // Send response back: swap src/dst
                        if let Err(e) = reply_tx.send((response, dst_addr, src_addr)).await {
                            warn!("UDP reply channel error: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("UDP forward error {} -> {}: {}", src_addr, dst_addr, e);
                    }
                }
            });
        }
        info!("UDP forwarder ended");
    });

    // Main loop: Accept TCP connections and forward via SOCKS5
    let socks5_addr = config.socks5_addr;
    info!("Waiting for TCP connections...");
    while running.load(Ordering::SeqCst) {
        match tcp_listener.next().await {
            Some((tcp_stream, src_addr, dst_addr)) => {
                info!("New TCP: {} -> {}", src_addr, dst_addr);
                let socks5 = socks5_addr;
                tokio::spawn(async move {
                    if let Err(e) = handle_tcp_via_socks5(tcp_stream, dst_addr, socks5).await {
                        warn!("SOCKS5 relay error {} -> {}: {}", src_addr, dst_addr, e);
                    }
                });
            }
            None => break,
        }
    }

    Ok(())
}

/// 直接转发 UDP 数据包到目标地址并等待响应。
/// 不经过 SOCKS5 代理（大多数 SOCKS5 代理不支持 UDP，且 DNS 查询需要低延迟）。
///
/// Android 特殊处理：
/// 必须先创建 socket → protect → bind → 才能发送数据。
/// 如果不 protect，UDP socket 的流量也会被 TUN 捕获，形成死循环。
async fn forward_udp(
    payload: &[u8],
    _src_addr: SocketAddr,
    dst_addr: SocketAddr,
) -> std::io::Result<Vec<u8>> {
    let local_addr: SocketAddr = if dst_addr.is_ipv4() {
        "0.0.0.0:0".parse().unwrap()
    } else {
        "[::]:0".parse().unwrap()
    };

    // Android: 使用 socket2 分步创建 UDP socket
    // 步骤顺序严格：new → protect → bind → set_nonblocking → 转为 tokio socket
    // 如果顺序错误（比如先 bind 再 protect），可能导致流量走 TUN 或 bind 失败
    #[cfg(target_os = "android")]
    let socket = {
        use std::os::unix::io::AsRawFd;
        let domain = if dst_addr.is_ipv4() {
            socket2::Domain::IPV4
        } else {
            socket2::Domain::IPV6
        };
        // 第 1 步：创建原始 socket（此时还没有绑定地址）
        let sock = socket2::Socket::new(domain, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
        // 第 2 步：protect！必须在 bind/connect 之前
        let fd = sock.as_raw_fd();
        protect_socket(fd);
        // 第 3 步：绑定到本地随机端口
        sock.bind(&socket2::SockAddr::from(local_addr))?;
        // 第 4 步：设为非阻塞（tokio 要求）
        sock.set_nonblocking(true)?;
        // 第 5 步：转换为 tokio UdpSocket
        let std_socket: std::net::UdpSocket = sock.into();
        TokioUdpSocket::from_std(std_socket)?
    };

    #[cfg(not(target_os = "android"))]
    let socket = TokioUdpSocket::bind(local_addr).await?;
    socket.send_to(payload, dst_addr).await?;

    let mut buf = vec![0u8; 4096];
    let (len, _) = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        socket.recv_from(&mut buf),
    )
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "UDP timeout"))??;

    buf.truncate(len);
    Ok(buf)
}

async fn handle_tcp_via_socks5(
    mut local_stream: netstack_smoltcp::TcpStream,
    dst_addr: SocketAddr,
    socks5_addr: SocketAddr,
) -> std::io::Result<()> {
    use fast_socks5::client::{Config as Socks5Config, Socks5Stream};

    info!("Connecting via SOCKS5 {} to {}", socks5_addr, dst_addr);

    // Android TCP socket 保护流程：
    // 为什么不能直接用 Socks5Stream::connect()：
    //   connect() 内部会自己创建 TcpStream，我们拿不到 fd 来 protect
    //   所以必须手动分步：创建 socket → protect → connect → 包装为 Socks5Stream
    //
    // 关键：protect 必须在 connect 之前！
    // 因为 connect() 发送的 SYN 包如果走了 TUN → netstack → SOCKS5 → 死循环
    #[cfg(target_os = "android")]
    let mut socks5_stream = {
        use fast_socks5::Socks5Command;
        use fast_socks5::util::target_addr::TargetAddr;
        use std::os::unix::io::{AsRawFd, IntoRawFd, FromRawFd};

        // 第 1 步：创建原始 TCP socket（还未连接任何地址）
        let domain = if socks5_addr.is_ipv4() {
            socket2::Domain::IPV4
        } else {
            socket2::Domain::IPV6
        };
        let sock = socket2::Socket::new(domain, socket2::Type::STREAM, Some(socket2::Protocol::TCP))?;

        // 第 2 步：protect！在 connect 之前标记 socket 绕过 VPN 路由
        // protect 通过 JNI 回调 Java 的 VpnService.protect(fd)
        let fd = sock.as_raw_fd();
        let protected = protect_socket(fd);
        info!("protect_socket(fd={}) = {}", fd, protected);
        if !protected {
            warn!("Failed to protect SOCKS5 TCP socket fd={}", fd);
        }

        // 第 3 步：connect 到 SOCKS5 代理服务器
        // 此时 socket 已被 protect，SYN 包走物理网卡而非 TUN
        sock.connect(&socket2::SockAddr::from(socks5_addr))?;
        sock.set_nonblocking(true)?;

        // 第 4 步：将 socket2::Socket 转换为 tokio TcpStream
        // socket2 → std::net::TcpStream → tokio::net::TcpStream
        let std_stream: std::net::TcpStream = sock.into();
        let tokio_stream = tokio::net::TcpStream::from_std(std_stream)?;

        // 第 5 步：在已建立的 TCP 连接上执行 SOCKS5 握手
        // use_stream() 复用已有连接（而非创建新连接）
        let mut stream = Socks5Stream::use_stream(
            tokio_stream,
            None,
            Socks5Config::default(),
        )
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        // 第 6 步：通过 SOCKS5 协议请求连接到实际目标（如 google.com:443）
        let target = TargetAddr::Ip(dst_addr);
        stream.request(Socks5Command::TCPConnect, target)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        stream
    };

    #[cfg(not(target_os = "android"))]
    let mut socks5_stream = Socks5Stream::connect(
        socks5_addr.to_string(),
        dst_addr.ip().to_string(),
        dst_addr.port(),
        Socks5Config::default(),
    )
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    info!("SOCKS5 tunnel established: -> {}", dst_addr);

    match io::copy_bidirectional(&mut local_stream, &mut socks5_stream).await {
        Ok((up, down)) => {
            info!("Connection {} closed: up={} down={}", dst_addr, up, down);
        }
        Err(e) => {
            warn!("copy_bidirectional error {}: {}", dst_addr, e);
        }
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn tunnel_stop() {
    if let Some(tunnel) = TUNNEL.get() {
        tunnel.running.store(false, Ordering::SeqCst);
        info!("tunnel_stop: signaled stop");
    }
}

#[no_mangle]
pub unsafe extern "C" fn tunnel_feed_packet(data: *const u8, len: usize) -> bool {
    if data.is_null() || len == 0 {
        return false;
    }

    let packet = std::slice::from_raw_parts(data, len).to_vec();

    if let Some(tunnel) = TUNNEL.get() {
        tunnel.inbound_tx.try_send(packet).is_ok()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn tunnel_read_packet(buf: *mut u8, buf_len: usize) -> usize {
    if buf.is_null() || buf_len == 0 {
        return 0;
    }

    if let Some(tunnel) = TUNNEL.get() {
        let mut rx = tunnel.outbound_rx.lock();
        match rx.try_recv() {
            Ok(packet) => {
                let copy_len = packet.len().min(buf_len);
                std::ptr::copy_nonoverlapping(packet.as_ptr(), buf, copy_len);
                copy_len
            }
            Err(_) => 0,
        }
    } else {
        0
    }
}

/// Test SOCKS5 connectivity directly (no TUN involved).
/// Returns 1 on success, 0 on failure. Writes result message to `out_buf`.
///
/// # Safety
/// `socks5_addr` must be a valid C string. `out_buf` must have `out_len` bytes.
#[no_mangle]
pub unsafe extern "C" fn tunnel_test_socks5(
    socks5_addr: *const c_char,
    out_buf: *mut u8,
    out_len: usize,
) -> i32 {
    init_logger();

    if socks5_addr.is_null() {
        return write_result(out_buf, out_len, "socks5_addr is null");
    }

    let addr_str = match CStr::from_ptr(socks5_addr).to_str() {
        Ok(s) => s,
        Err(_) => return write_result(out_buf, out_len, "invalid UTF-8"),
    };

    info!("test_socks5: testing connection to {}", addr_str);

    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => return write_result(out_buf, out_len, &format!("runtime error: {}", e)),
    };

    let result = rt.block_on(async {
        test_socks5_impl(addr_str).await
    });

    match result {
        Ok(msg) => {
            info!("test_socks5 SUCCESS: {}", msg);
            write_result(out_buf, out_len, &format!("OK: {}", msg));
            1
        }
        Err(msg) => {
            error!("test_socks5 FAILED: {}", msg);
            write_result(out_buf, out_len, &format!("FAIL: {}", msg));
            0
        }
    }
}

async fn test_socks5_impl(socks5_addr: &str) -> Result<String, String> {
    use fast_socks5::client::{Config as Socks5Config, Socks5Stream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Step 1: Test raw TCP connectivity to SOCKS5 server
    let tcp_result = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        tokio::net::TcpStream::connect(socks5_addr),
    ).await;

    match &tcp_result {
        Ok(Ok(_)) => info!("TCP connect to {} OK", socks5_addr),
        Ok(Err(e)) => return Err(format!("TCP connect failed: {}", e)),
        Err(_) => return Err(format!("TCP connect timeout to {}", socks5_addr)),
    }
    drop(tcp_result);

    // Step 2: Test SOCKS5 handshake + connect to httpbin.org:80
    let stream_result = tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        Socks5Stream::connect(
            socks5_addr.to_string(),
            "httpbin.org".to_string(),
            80,
            Socks5Config::default(),
        ),
    ).await;

    let mut stream = match stream_result {
        Ok(Ok(s)) => { info!("SOCKS5 handshake OK"); s }
        Ok(Err(e)) => return Err(format!("SOCKS5 handshake failed: {}", e)),
        Err(_) => return Err("SOCKS5 handshake timeout".to_string()),
    };

    // Step 3: Send HTTP request through SOCKS5
    let req = "GET /ip HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
    if let Err(e) = stream.write_all(req.as_bytes()).await {
        return Err(format!("HTTP write error: {}", e));
    }

    let mut response = vec![0u8; 4096];
    let n = match tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        stream.read(&mut response),
    ).await {
        Ok(Ok(n)) => n,
        Ok(Err(e)) => return Err(format!("HTTP read error: {}", e)),
        Err(_) => return Err("HTTP read timeout".to_string()),
    };

    let body = String::from_utf8_lossy(&response[..n]);
    // Extract just the status line
    let status_line = body.lines().next().unwrap_or("(empty)");
    Ok(format!("{} ({} bytes)", status_line, n))
}

unsafe fn write_result(buf: *mut u8, len: usize, msg: &str) -> i32 {
    if buf.is_null() || len == 0 {
        return 0;
    }
    let bytes = msg.as_bytes();
    let copy_len = bytes.len().min(len - 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, copy_len);
    *buf.add(copy_len) = 0; // null terminate
    0
}

#[no_mangle]
pub extern "C" fn tunnel_is_running() -> bool {
    TUNNEL.get()
        .map(|t| t.running.load(Ordering::SeqCst))
        .unwrap_or(false)
}
