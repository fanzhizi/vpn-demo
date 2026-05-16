pub mod stack;
pub mod socks5;
pub mod ffi;

use std::net::SocketAddr;
use std::sync::Arc;
use parking_lot::Mutex;
use tokio::sync::mpsc;

/// Tunnel configuration
#[derive(Clone, Debug)]
pub struct TunnelConfig {
    pub socks5_addr: SocketAddr,
}

/// Tunnel state shared between the packet provider and the networking stack
pub struct TunnelState {
    pub config: TunnelConfig,
    /// Channel to send raw IP packets TO the tunnel (from device)
    pub packet_tx: mpsc::Sender<Vec<u8>>,
    /// Channel to receive raw IP packets FROM the tunnel (to device)
    pub packet_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
    pub running: Arc<std::sync::atomic::AtomicBool>,
}

impl TunnelState {
    pub fn new(config: TunnelConfig) -> (Self, mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) {
        // Packets from TUN device -> our stack
        let (inbound_tx, inbound_rx) = mpsc::channel::<Vec<u8>>(256);
        // Packets from our stack -> TUN device
        let (outbound_tx, outbound_rx) = mpsc::channel::<Vec<u8>>(256);

        let state = TunnelState {
            config,
            packet_tx: inbound_tx,
            packet_rx: Arc::new(Mutex::new(outbound_rx)),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        };

        (state, outbound_tx, inbound_rx)
    }
}
