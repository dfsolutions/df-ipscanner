use std::net::Ipv4Addr;

/// Get MAC address for an IP using ARP
#[cfg(windows)]
pub async fn get_mac_address(ip: Ipv4Addr) -> Option<String> {
    use windows::Win32::NetworkManagement::IpHelper::SendARP;

    let ip_addr: u32 = u32::from(ip).to_be();
    let mut mac: [u8; 6] = [0; 6];
    let mut mac_len: u32 = 6;

    let result = unsafe { SendARP(ip_addr, 0, mac.as_mut_ptr() as *mut _, &mut mac_len) };

    if result == 0 && mac_len == 6 {
        Some(format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
        ))
    } else {
        None
    }
}

/// Get MAC address for an IP using ARP (Unix fallback - reads /proc/net/arp)
#[cfg(not(windows))]
pub async fn get_mac_address(ip: Ipv4Addr) -> Option<String> {
    use std::fs;

    let ip_str = ip.to_string();

    if let Ok(arp_table) = fs::read_to_string("/proc/net/arp") {
        for line in arp_table.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == ip_str {
                let mac = parts[3];
                if mac != "00:00:00:00:00:00" {
                    return Some(mac.to_uppercase());
                }
            }
        }
    }

    None
}
