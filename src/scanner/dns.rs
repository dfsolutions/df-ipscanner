use dns_lookup::lookup_addr;
use std::net::Ipv4Addr;

/// Perform reverse DNS lookup to get hostname
pub async fn reverse_lookup(ip: Ipv4Addr) -> Option<String> {
    // Run blocking DNS lookup in a separate thread
    let result = tokio::task::spawn_blocking(move || lookup_addr(&ip.into())).await;

    match result {
        Ok(Ok(hostname)) => Some(hostname),
        _ => None,
    }
}
