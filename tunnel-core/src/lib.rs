pub mod ffi;
#[cfg(target_os = "android")]
pub mod jni_ffi;

use std::net::SocketAddr;

/// Tunnel configuration
#[derive(Clone, Debug)]
pub struct TunnelConfig {
    pub socks5_addr: SocketAddr,
}
