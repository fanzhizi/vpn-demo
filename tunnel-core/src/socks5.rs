//! SOCKS5 proxy connector using fast-socks5.
//! Establishes connections through a SOCKS5 proxy server.

use std::net::SocketAddr;
use fast_socks5::client::{Config as Socks5Config, Socks5Stream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::info;

pub struct Socks5Connector;

impl Socks5Connector {
    /// Connect to destination through SOCKS5 and forward data, returning the response.
    pub async fn forward_data(
        socks5_addr: SocketAddr,
        dst_addr: SocketAddr,
        data: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let config = Socks5Config::default();

        let mut stream = Socks5Stream::connect(
            socks5_addr.to_string(),
            dst_addr.ip().to_string(),
            dst_addr.port(),
            config,
        )
        .await?;

        info!("SOCKS5 tunnel established to {}", dst_addr);

        // Send the data
        stream.write_all(data).await?;

        // Read response (with timeout)
        let mut response = vec![0u8; 65535];
        let n = tokio::time::timeout(
            tokio::time::Duration::from_secs(10),
            stream.read(&mut response),
        )
        .await
        .unwrap_or(Ok(0))?;

        Ok(response[..n].to_vec())
    }

    /// Create a persistent SOCKS5 connection for bidirectional forwarding.
    pub async fn connect(
        socks5_addr: SocketAddr,
        dst_addr: SocketAddr,
    ) -> Result<Socks5Stream<tokio::net::TcpStream>, Box<dyn std::error::Error + Send + Sync>> {
        let config = Socks5Config::default();

        let stream = Socks5Stream::connect(
            socks5_addr.to_string(),
            dst_addr.ip().to_string(),
            dst_addr.port(),
            config,
        )
        .await?;

        info!("SOCKS5 persistent connection to {} established", dst_addr);
        Ok(stream)
    }
}
