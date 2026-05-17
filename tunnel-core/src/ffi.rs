//! C FFI interface for calling from Swift (iOS Network Extension).

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

/// Protect a socket fd from VPN routing (Android only).
/// Follows the same pattern as leaf (eycorsican/leaf):
///   RwLock<JavaVM> + RwLock<GlobalRef> + attach_current_thread_permanently
#[cfg(target_os = "android")]
pub mod android_protect {
    use std::os::unix::io::RawFd;
    use std::sync::RwLock;
    use jni::objects::{GlobalRef, JValue};
    use jni::JavaVM;
    use log::{info, warn};

    static JVM: RwLock<Option<JavaVM>> = RwLock::new(None);
    static PROTECT_CALLBACK: RwLock<Option<ProtectCallback>> = RwLock::new(None);

    struct ProtectCallback {
        class: GlobalRef,
        method: String,
    }

    pub fn set_jvm(vm: JavaVM) {
        *JVM.write().unwrap() = Some(vm);
    }

    pub fn set_protect_callback(class: GlobalRef, method: String) {
        info!("set_protect_callback: method={}", method);
        *PROTECT_CALLBACK.write().unwrap() = Some(ProtectCallback { class, method });
    }

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

        let mut env = match vm.attach_current_thread_permanently() {
            Ok(env) => env,
            Err(e) => {
                warn!("protect_socket: attach thread failed: {}", e);
                return false;
            }
        };

        match env.call_method(&cb.class, &cb.method, "(I)Z", &[JValue::Int(fd as i32)]) {
            Ok(result) => result.z().unwrap_or(false),
            Err(e) => {
                warn!("protect_socket: call_method failed: {}", e);
                let _ = env.exception_clear();
                false
            }
        }
    }

    /// Called by JNI when the library is loaded. Only saves JVM reference.
    #[no_mangle]
    pub extern "system" fn JNI_OnLoad(
        vm: jni::JavaVM,
        _: *mut std::ffi::c_void,
    ) -> jni::sys::jint {
        set_jvm(vm);
        jni::sys::JNI_VERSION_1_6
    }
}

#[cfg(target_os = "android")]
pub fn protect_socket(fd: i32) -> bool {
    android_protect::protect_socket(fd)
}

#[cfg(not(target_os = "android"))]
pub fn protect_socket(_fd: i32) -> bool {
    true // no-op on non-Android
}

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

struct TunnelRuntime {
    runtime: Runtime,
    running: Arc<AtomicBool>,
    inbound_tx: mpsc::Sender<Vec<u8>>,
    outbound_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
}

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

/// Forward a UDP packet directly to destination and return the response.
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

    #[cfg(target_os = "android")]
    let socket = {
        use std::os::unix::io::AsRawFd;
        let domain = if dst_addr.is_ipv4() {
            socket2::Domain::IPV4
        } else {
            socket2::Domain::IPV6
        };
        let sock = socket2::Socket::new(domain, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
        let fd = sock.as_raw_fd();
        protect_socket(fd);
        sock.bind(&socket2::SockAddr::from(local_addr))?;
        sock.set_nonblocking(true)?;
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

    // On Android: create socket -> protect it -> then connect to SOCKS5
    // Must protect BEFORE connect, otherwise the SYN goes through TUN
    #[cfg(target_os = "android")]
    let mut socks5_stream = {
        use fast_socks5::Socks5Command;
        use fast_socks5::util::target_addr::TargetAddr;
        use std::os::unix::io::{AsRawFd, IntoRawFd, FromRawFd};

        // Step 1: Create raw socket
        let domain = if socks5_addr.is_ipv4() {
            socket2::Domain::IPV4
        } else {
            socket2::Domain::IPV6
        };
        let sock = socket2::Socket::new(domain, socket2::Type::STREAM, Some(socket2::Protocol::TCP))?;

        // Step 2: Protect BEFORE connecting
        let fd = sock.as_raw_fd();
        let protected = protect_socket(fd);
        info!("protect_socket(fd={}) = {}", fd, protected);
        if !protected {
            warn!("Failed to protect SOCKS5 TCP socket fd={}", fd);
        }

        // Step 3: Connect (this now bypasses VPN because fd is protected)
        sock.connect(&socket2::SockAddr::from(socks5_addr))?;
        sock.set_nonblocking(true)?;

        // Step 4: Convert to tokio TcpStream
        let std_stream: std::net::TcpStream = sock.into();
        let tokio_stream = tokio::net::TcpStream::from_std(std_stream)?;

        let mut stream = Socks5Stream::use_stream(
            tokio_stream,
            None,
            Socks5Config::default(),
        )
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

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
