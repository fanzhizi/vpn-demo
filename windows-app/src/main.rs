//! Windows VPN Demo - 使用 wintun.dll + netstack-smoltcp + SOCKS5
//!
//! 共用 vpn-app/ui/main.slint 界面，Windows 特定逻辑在 tun.rs 中。
//! Windows 通过 route 命令排除 SOCKS5 服务器 IP，防止路由回环。

#[cfg(windows)]
mod tun;

slint::include_modules!();

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::net::SocketAddr;
use log::{info, error};
use tokio::sync::mpsc;
use futures::{SinkExt, StreamExt};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    let window = MainWindow::new().unwrap();
    let running = Arc::new(AtomicBool::new(false));

    // Connect
    let running_c = running.clone();
    window.on_connect_clicked(move |addr| {
        let addr_str = addr.to_string();
        if addr_str.parse::<SocketAddr>().is_err() {
            return;
        }
        let running = running_c.clone();
        running.store(true, Ordering::SeqCst);

        std::thread::spawn(move || {
            if let Err(e) = start_vpn(&addr_str, running.clone()) {
                error!("VPN start failed: {}", e);
                running.store(false, Ordering::SeqCst);
            }
        });
    });

    // Disconnect
    let running_d = running.clone();
    window.on_disconnect_clicked(move || {
        running_d.store(false, Ordering::SeqCst);
    });

    // Test SOCKS5
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

    // 用定时器轮询 running 状态更新 UI
    let running_ui = running.clone();
    let window_weak = window.as_weak();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
        if let Some(w) = window_weak.upgrade() {
            let is_running = running_ui.load(Ordering::SeqCst);
            if w.get_connected() != is_running {
                w.set_connected(is_running);
                w.set_status_text(if is_running { "Connected".into() } else { "Disconnected".into() });
            }
        }
    });

    window.run().unwrap();

    // 退出时停止 VPN
    running.store(false, Ordering::SeqCst);
}

fn start_vpn(socks5_addr: &str, running: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = socks5_addr.parse()?;
    info!("Starting VPN, SOCKS5: {}", socks5_addr);

    let rt = tokio::runtime::Runtime::new()?;

    let (tun_tx, mut tun_rx) = mpsc::channel::<Vec<u8>>(1024);
    let (stack_tx, stack_rx) = mpsc::channel::<Vec<u8>>(1024);

    #[cfg(windows)]
    {
        let mut config = tun::win::TunConfig::default();
        config.socks5_host = socket_addr.ip().to_string();
        tun::win::start_tun(config, running.clone(), tun_tx, stack_rx)?;
    }

    #[cfg(not(windows))]
    {
        info!("Non-Windows: TUN not available, mock mode");
        drop(tun_tx);
        drop(stack_rx);
    }

    let running2 = running.clone();
    rt.block_on(async move {
        if let Err(e) = run_netstack(socket_addr, tun_rx, stack_tx, running2).await {
            error!("Netstack error: {}", e);
        }
    });

    Ok(())
}

async fn run_netstack(
    socks5_addr: SocketAddr,
    mut tun_rx: mpsc::Receiver<Vec<u8>>,
    tun_tx: mpsc::Sender<Vec<u8>>,
    running: Arc<AtomicBool>,
) -> std::io::Result<()> {
    use netstack_smoltcp::StackBuilder;

    let (stack, runner, udp_socket, tcp_listener) = StackBuilder::default()
        .enable_tcp(true)
        .enable_udp(true)
        .enable_icmp(true)
        .tcp_buffer_size(65535)
        .udp_buffer_size(65535)
        .stack_buffer_size(1024)
        .build()?;

    let mut tcp_listener = tcp_listener.unwrap();
    let udp_socket = udp_socket.unwrap();
    let (mut udp_rx, mut udp_tx) = udp_socket.split();

    if let Some(runner) = runner {
        tokio::spawn(async move { let _ = runner.await; });
    }

    let (to_stack_tx, mut to_stack_rx) = mpsc::channel::<Vec<u8>>(1024);
    let (from_stack_tx, mut from_stack_rx) = mpsc::channel::<Vec<u8>>(1024);

    // Stack 双向驱动
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

    // TUN → Stack
    let running2 = running.clone();
    tokio::spawn(async move {
        while running2.load(Ordering::SeqCst) {
            match tun_rx.recv().await {
                Some(pkt) => { let _ = to_stack_tx.send(pkt).await; }
                None => break,
            }
        }
    });

    // Stack → TUN
    let running3 = running.clone();
    tokio::spawn(async move {
        while running3.load(Ordering::SeqCst) {
            match from_stack_rx.recv().await {
                Some(pkt) => { let _ = tun_tx.send(pkt).await; }
                None => break,
            }
        }
    });

    // UDP 转发
    let (udp_reply_tx, mut udp_reply_rx) = mpsc::channel::<(Vec<u8>, SocketAddr, SocketAddr)>(256);
    tokio::spawn(async move {
        while let Some(msg) = udp_reply_rx.recv().await { let _ = udp_tx.send(msg).await; }
    });
    tokio::spawn(async move {
        while let Some((payload, src, dst)) = udp_rx.next().await {
            let tx = udp_reply_tx.clone();
            tokio::spawn(async move {
                if let Ok(resp) = forward_udp(&payload, dst).await {
                    let _ = tx.send((resp, dst, src)).await;
                }
            });
        }
    });

    // TCP → SOCKS5
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

async fn handle_tcp(mut local: netstack_smoltcp::TcpStream, dst: SocketAddr, socks5: SocketAddr) -> std::io::Result<()> {
    use fast_socks5::client::{Config, Socks5Stream};

    // Windows: 绑定到物理网卡 IP，防止 SOCKS5 连接走 TUN 回环
    #[cfg(windows)]
    {
        use std::net::TcpStream as StdTcpStream;

        if let Some(bind_ip) = tun::win::get_bind_address() {
            let bind_addr: SocketAddr = format!("{}:0", bind_ip).parse().unwrap();
            let socket = socket2::Socket::new(
                socket2::Domain::IPV4,
                socket2::Type::STREAM,
                Some(socket2::Protocol::TCP),
            )?;
            socket.bind(&socket2::SockAddr::from(bind_addr))?;
            socket.connect(&socket2::SockAddr::from(socks5))?;
            socket.set_nonblocking(true)?;

            let std_stream: StdTcpStream = socket.into();
            let tokio_stream = tokio::net::TcpStream::from_std(std_stream)?;

            let mut socks5_stream = fast_socks5::client::Socks5Stream::use_stream(
                tokio_stream,
                None,
                Config::default(),
            )
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let target = fast_socks5::util::target_addr::TargetAddr::Ip(dst);
            socks5_stream.request(fast_socks5::Socks5Command::TCPConnect, target)
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let _ = tokio::io::copy_bidirectional(&mut local, &mut socks5_stream).await;
            return Ok(());
        }
    }

    // 非 Windows 或无绑定地址时：直接连接
    let mut remote = Socks5Stream::connect(socks5.to_string(), dst.ip().to_string(), dst.port(), Config::default())
        .await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let _ = tokio::io::copy_bidirectional(&mut local, &mut remote).await;
    Ok(())
}

async fn forward_udp(payload: &[u8], dst: SocketAddr) -> std::io::Result<Vec<u8>> {
    // Windows: 绑定到物理网卡 IP，防止 UDP 走 TUN 回环
    #[cfg(windows)]
    let socket = {
        if let Some(bind_ip) = tun::win::get_bind_address() {
            let bind_addr: SocketAddr = format!("{}:0", bind_ip).parse().unwrap();
            let sock = socket2::Socket::new(
                socket2::Domain::IPV4,
                socket2::Type::DGRAM,
                Some(socket2::Protocol::UDP),
            )?;
            sock.bind(&socket2::SockAddr::from(bind_addr))?;
            sock.set_nonblocking(true)?;
            let std_socket: std::net::UdpSocket = sock.into();
            tokio::net::UdpSocket::from_std(std_socket)?
        } else {
            tokio::net::UdpSocket::bind("0.0.0.0:0").await?
        }
    };

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
