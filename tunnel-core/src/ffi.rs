//! C FFI interface for calling from Swift (iOS Network Extension).
//! The PacketTunnelProvider in Swift will call these functions.

use std::ffi::CStr;
use std::net::SocketAddr;
use std::os::raw::c_char;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use log::info;

use crate::stack::NetworkStack;
use crate::TunnelConfig;

/// Global tunnel runtime state
struct TunnelRuntime {
    runtime: Runtime,
    running: Arc<AtomicBool>,
    inbound_tx: mpsc::Sender<Vec<u8>>,
    outbound_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
}

static TUNNEL: OnceCell<TunnelRuntime> = OnceCell::new();

/// Start the tunnel with a SOCKS5 proxy address.
/// Called from Swift when the VPN tunnel starts.
///
/// # Safety
/// `socks5_addr` must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn tunnel_start(socks5_addr: *const c_char) -> bool {
    if socks5_addr.is_null() {
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

    info!("tunnel_start: SOCKS5 addr = {}", socket_addr);

    let config = TunnelConfig {
        socks5_addr: socket_addr,
    };

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return false,
    };

    let running = Arc::new(AtomicBool::new(true));

    // Channels for packet exchange
    let (inbound_tx, inbound_rx) = mpsc::channel::<Vec<u8>>(512);
    let (outbound_tx, outbound_rx) = mpsc::channel::<Vec<u8>>(512);

    let stack = NetworkStack::new(config, running.clone());

    // Spawn the network stack on the runtime
    runtime.spawn(async move {
        stack.run(inbound_rx, outbound_tx).await;
    });

    let _ = TUNNEL.set(TunnelRuntime {
        runtime,
        running,
        inbound_tx,
        outbound_rx: Arc::new(Mutex::new(outbound_rx)),
    });

    true
}

/// Stop the tunnel.
/// Called from Swift when the VPN tunnel stops.
#[no_mangle]
pub extern "C" fn tunnel_stop() {
    if let Some(tunnel) = TUNNEL.get() {
        tunnel.running.store(false, Ordering::SeqCst);
        info!("tunnel_stop: signaled stop");
    }
}

/// Feed a raw IP packet into the tunnel (from TUN device).
/// Called from Swift's `NEPacketTunnelProvider` when it reads packets.
///
/// # Safety
/// `data` must point to `len` valid bytes.
#[no_mangle]
pub unsafe extern "C" fn tunnel_feed_packet(data: *const u8, len: usize) -> bool {
    if data.is_null() || len == 0 {
        return false;
    }

    let packet = std::slice::from_raw_parts(data, len).to_vec();

    if let Some(tunnel) = TUNNEL.get() {
        tunnel.runtime.block_on(async {
            tunnel.inbound_tx.send(packet).await.is_ok()
        })
    } else {
        false
    }
}

/// Try to read an outbound packet (to be written back to TUN device).
/// Returns the number of bytes written to `buf`, or 0 if no packet available.
///
/// # Safety
/// `buf` must point to at least `buf_len` writable bytes.
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

/// Check if the tunnel is currently running.
#[no_mangle]
pub extern "C" fn tunnel_is_running() -> bool {
    TUNNEL.get()
        .map(|t| t.running.load(Ordering::SeqCst))
        .unwrap_or(false)
}
