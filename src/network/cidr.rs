use anyhow::{Context, Result};
use ipnetwork::Ipv4Network;
use std::net::Ipv4Addr;

/// Parse CIDR notation and generate list of host IPs
/// Excludes network address and broadcast address
pub fn generate_ip_range(cidr: &str) -> Result<Vec<Ipv4Addr>> {
    let network: Ipv4Network = cidr
        .parse()
        .with_context(|| format!("Invalid CIDR format: {}", cidr))?;

    let size = network.size();

    // For /32, return single host
    if size == 1 {
        return Ok(vec![network.ip()]);
    }

    // For /31, return both IPs (point-to-point link)
    if size == 2 {
        return Ok(network.iter().collect());
    }

    // For larger networks, exclude network address (first) and broadcast (last)
    let hosts: Vec<Ipv4Addr> = network.iter().skip(1).take(size as usize - 2).collect();

    Ok(hosts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_slash_24() {
        let hosts = generate_ip_range("192.168.1.0/24").unwrap();
        assert_eq!(hosts.len(), 254);
        assert_eq!(hosts[0], Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(hosts[253], Ipv4Addr::new(192, 168, 1, 254));
    }

    #[test]
    fn test_parse_slash_32() {
        let hosts = generate_ip_range("192.168.1.1/32").unwrap();
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0], Ipv4Addr::new(192, 168, 1, 1));
    }

    #[test]
    fn test_invalid_cidr() {
        let result = generate_ip_range("invalid");
        assert!(result.is_err());
    }
}
