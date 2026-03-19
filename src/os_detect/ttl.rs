/// Guess operating system based on initial TTL value
///
/// Common initial TTL values:
/// - Linux/Unix: 64
/// - Windows: 128
/// - Cisco/Network devices: 255
pub fn guess_os_from_ttl(ttl: u8) -> &'static str {
    match ttl {
        0 => "Unknown",
        // TTL values close to 64 (Linux/Unix)
        1..=64 => "Linux/Unix",
        // TTL values close to 128 (Windows)
        65..=128 => "Windows",
        // TTL values close to 255 (Network devices)
        129..=255 => "Network Device",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_ttl() {
        assert_eq!(guess_os_from_ttl(64), "Linux/Unix");
        assert_eq!(guess_os_from_ttl(63), "Linux/Unix");
        assert_eq!(guess_os_from_ttl(50), "Linux/Unix");
    }

    #[test]
    fn test_windows_ttl() {
        assert_eq!(guess_os_from_ttl(128), "Windows");
        assert_eq!(guess_os_from_ttl(127), "Windows");
        assert_eq!(guess_os_from_ttl(100), "Windows");
    }

    #[test]
    fn test_network_device_ttl() {
        assert_eq!(guess_os_from_ttl(255), "Network Device");
        assert_eq!(guess_os_from_ttl(200), "Network Device");
    }
}
