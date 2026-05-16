//! Userspace TCP/IP stack using smoltcp.
//! Receives raw IP packets from the TUN device, reconstructs TCP connections,
//! and forwards them through SOCKS5 proxy.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

use smoltcp::iface::{Config, Interface, SocketHandle, SocketSet};
use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::socket::tcp::{Socket as TcpSocket, State as TcpState};
use smoltcp::time::Instant as SmolInstant;
use smoltcp::wire::{HardwareAddress, IpCidr, IpAddress, Ipv4Packet, IpProtocol};
use smoltcp::wire::TcpPacket;

use tokio::sync::mpsc;
use log::{info, warn, debug};

use crate::socks5::Socks5Connector;
use crate::TunnelConfig;

/// Get current smoltcp timestamp from system clock
fn smol_now() -> SmolInstant {
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    SmolInstant::from_millis(dur.as_millis() as i64)
}

/// Represents a tracked TCP connection
struct TcpConnection {
    handle: SocketHandle,
    dst_addr: SocketAddr,
    /// Whether we've started SOCKS5 forwarding for this connection
    forwarding: bool,
}

/// Virtual device that interfaces with the TUN packet channels
struct VirtualDevice {
    rx_buffer: Vec<Vec<u8>>,
    tx_buffer: Vec<Vec<u8>>,
}

impl VirtualDevice {
    fn new() -> Self {
        Self {
            rx_buffer: Vec::new(),
            tx_buffer: Vec::new(),
        }
    }

    fn inject_packet(&mut self, packet: Vec<u8>) {
        self.rx_buffer.push(packet);
    }

    fn drain_tx(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.tx_buffer)
    }
}

struct VirtualRxToken {
    data: Vec<u8>,
}

impl RxToken for VirtualRxToken {
    fn consume<R, F>(mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.data)
    }
}

struct VirtualTxToken<'a> {
    buffer: &'a mut Vec<Vec<u8>>,
}

impl<'a> TxToken for VirtualTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buf = vec![0u8; len];
        let result = f(&mut buf);
        self.buffer.push(buf);
        result
    }
}

impl Device for VirtualDevice {
    type RxToken<'a> = VirtualRxToken;
    type TxToken<'a> = VirtualTxToken<'a>;

    fn receive(&mut self, _timestamp: SmolInstant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        if self.rx_buffer.is_empty() {
            return None;
        }
        let data = self.rx_buffer.remove(0);
        Some((
            VirtualRxToken { data },
            VirtualTxToken { buffer: &mut self.tx_buffer },
        ))
    }

    fn transmit(&mut self, _timestamp: SmolInstant) -> Option<Self::TxToken<'_>> {
        Some(VirtualTxToken { buffer: &mut self.tx_buffer })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ip;
        caps.max_transmission_unit = 1500;
        caps
    }
}

/// Main network stack processor
pub struct NetworkStack {
    config: TunnelConfig,
    running: Arc<AtomicBool>,
}

impl NetworkStack {
    pub fn new(config: TunnelConfig, running: Arc<AtomicBool>) -> Self {
        Self { config, running }
    }

    /// Run the network stack event loop.
    /// - `inbound_rx`: raw IP packets coming from the TUN device
    /// - `outbound_tx`: raw IP packets going back to the TUN device
    pub async fn run(
        self,
        mut inbound_rx: mpsc::Receiver<Vec<u8>>,
        outbound_tx: mpsc::Sender<Vec<u8>>,
    ) {
        info!("NetworkStack starting, SOCKS5 proxy: {}", self.config.socks5_addr);

        let mut device = VirtualDevice::new();

        let iface_config = Config::new(HardwareAddress::Ip);
        let mut iface = Interface::new(iface_config, &mut device, smol_now());

        // Configure the interface with a virtual IP
        iface.update_ip_addrs(|addrs| {
            addrs.push(IpCidr::new(IpAddress::v4(10, 0, 0, 1), 24)).ok();
        });

        let mut sockets = SocketSet::new(vec![]);
        let mut connections: HashMap<(Ipv4Addr, u16, Ipv4Addr, u16), TcpConnection> = HashMap::new();
        let socks5_addr = self.config.socks5_addr;

        self.running.store(true, Ordering::SeqCst);

        while self.running.load(Ordering::SeqCst) {
            // Try to receive packets with a timeout
            let packet = tokio::select! {
                p = inbound_rx.recv() => p,
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(10)) => None,
            };

            if let Some(pkt) = packet {
                // Parse the packet to identify TCP connections
                if let Some(info) = parse_ip_packet(&pkt) {
                    debug!("Packet: {}:{} -> {}:{} proto={:?}",
                        info.src_ip, info.src_port, info.dst_ip, info.dst_port, info.protocol);

                    if info.protocol == IpProtocol::Tcp {
                        let key = (info.src_ip, info.src_port, info.dst_ip, info.dst_port);

                        if !connections.contains_key(&key) {
                            // New TCP connection - create a socket and start SOCKS5 forwarding
                            let tcp_socket = TcpSocket::new(
                                smoltcp::socket::tcp::SocketBuffer::new(vec![0; 65535]),
                                smoltcp::socket::tcp::SocketBuffer::new(vec![0; 65535]),
                            );
                            let handle = sockets.add(tcp_socket);

                            let dst_addr = SocketAddr::new(
                                IpAddr::V4(info.dst_ip),
                                info.dst_port,
                            );

                            connections.insert(key, TcpConnection {
                                handle,
                                dst_addr,
                                forwarding: false,
                            });

                            info!("New TCP connection tracked: {:?} -> {}", key, dst_addr);
                        }
                    }

                    // Inject into smoltcp
                    device.inject_packet(pkt);
                }
            }

            // Poll the interface
            iface.poll(smol_now(), &mut device, &mut sockets);

            // Process TCP sockets - forward data via SOCKS5
            let mut to_remove = Vec::new();
            for (key, conn) in connections.iter_mut() {
                let socket = sockets.get_mut::<TcpSocket>(conn.handle);

                match socket.state() {
                    TcpState::Established => {
                        if !conn.forwarding {
                            conn.forwarding = true;
                            info!("Starting SOCKS5 forwarding for {:?} -> {}", key, conn.dst_addr);

                            // Read available data and forward
                            let mut buf = vec![0u8; 4096];
                            if socket.can_recv() {
                                if let Ok(n) = socket.recv_slice(&mut buf) {
                                    let data = buf[..n].to_vec();
                                    let socks_addr_clone = socks5_addr;
                                    let dst_clone = conn.dst_addr;
                                    tokio::spawn(async move {
                                        if let Err(e) = Socks5Connector::forward_data(
                                            socks_addr_clone,
                                            dst_clone,
                                            &data,
                                        ).await {
                                            warn!("SOCKS5 forward error: {}", e);
                                        }
                                    });
                                }
                            }
                        } else if socket.can_recv() {
                            let mut buf = vec![0u8; 4096];
                            if let Ok(n) = socket.recv_slice(&mut buf) {
                                let data = buf[..n].to_vec();
                                let socks_addr_clone = socks5_addr;
                                let dst_clone = conn.dst_addr;
                                tokio::spawn(async move {
                                    if let Err(e) = Socks5Connector::forward_data(
                                        socks_addr_clone,
                                        dst_clone,
                                        &data,
                                    ).await {
                                        warn!("SOCKS5 forward error: {}", e);
                                    }
                                });
                            }
                        }
                    }
                    TcpState::Closed | TcpState::TimeWait => {
                        to_remove.push(*key);
                    }
                    _ => {}
                }
            }

            for key in to_remove {
                if let Some(conn) = connections.remove(&key) {
                    sockets.remove(conn.handle);
                    info!("TCP connection closed: {:?}", key);
                }
            }

            // Send outbound packets back to TUN
            for pkt in device.drain_tx() {
                if outbound_tx.send(pkt).await.is_err() {
                    warn!("Failed to send packet to TUN");
                    break;
                }
            }
        }

        info!("NetworkStack stopped");
    }
}

/// Parsed IP packet info
struct PacketInfo {
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
    src_port: u16,
    dst_port: u16,
    protocol: IpProtocol,
}

fn parse_ip_packet(data: &[u8]) -> Option<PacketInfo> {
    if data.is_empty() {
        return None;
    }

    let version = (data[0] >> 4) & 0x0F;
    if version != 4 {
        return None; // Only handle IPv4 for now
    }

    let ipv4 = Ipv4Packet::new_checked(data).ok()?;
    let protocol = ipv4.next_header();
    let src_ip = Ipv4Addr::from(ipv4.src_addr().0);
    let dst_ip = Ipv4Addr::from(ipv4.dst_addr().0);

    let (src_port, dst_port) = match protocol {
        IpProtocol::Tcp => {
            let tcp = TcpPacket::new_checked(ipv4.payload()).ok()?;
            (tcp.src_port(), tcp.dst_port())
        }
        _ => (0, 0),
    };

    Some(PacketInfo {
        src_ip,
        dst_ip,
        src_port,
        dst_port,
        protocol,
    })
}
