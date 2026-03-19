use crate::types::{COMMON_PORTS, EXTENDED_PORTS};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Scan ports on a host (common or extended list)
pub async fn scan_ports(ip: Ipv4Addr, timeout_secs: u64, extended: bool) -> Vec<u16> {
    let timeout_duration = Duration::from_millis(timeout_secs * 200); // Faster timeout for ports
    let ports_to_scan: &[u16] = if extended { EXTENDED_PORTS } else { COMMON_PORTS };

    let mut open_ports = Vec::new();

    // Scan all ports concurrently
    let results: Vec<(u16, bool)> = futures::future::join_all(ports_to_scan.iter().map(|&port| {
        let addr = SocketAddr::new(ip.into(), port);
        async move {
            let is_open = timeout(timeout_duration, TcpStream::connect(addr))
                .await
                .map(|r| r.is_ok())
                .unwrap_or(false);
            (port, is_open)
        }
    }))
    .await;

    for (port, is_open) in results {
        if is_open {
            open_ports.push(port);
        }
    }

    open_ports.sort();
    open_ports
}
