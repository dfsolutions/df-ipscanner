use std::net::Ipv4Addr;
use std::time::Duration;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};

pub struct PingResult {
    pub is_alive: bool,
    pub ttl: Option<u8>,
    pub latency_ms: Option<f64>,
}

/// Ping a single host using ICMP
pub async fn ping_host(ip: Ipv4Addr, timeout_secs: u64) -> PingResult {
    let config = Config::default();

    let client = match Client::new(&config) {
        Ok(c) => c,
        Err(_) => {
            return PingResult {
                is_alive: false,
                ttl: None,
                latency_ms: None,
            }
        }
    };

    let mut pinger = client.pinger(ip.into(), PingIdentifier(rand_id())).await;
    pinger.timeout(Duration::from_secs(timeout_secs));

    match pinger.ping(PingSequence(0), &[]).await {
        Ok((reply, duration)) => {
            // Get TTL from the reply packet
            let ttl = match reply {
                IcmpPacket::V4(packet) => packet.get_ttl(),
                IcmpPacket::V6(_) => None,
            };

            PingResult {
                is_alive: true,
                ttl,
                latency_ms: Some(duration.as_secs_f64() * 1000.0),
            }
        }
        Err(_) => PingResult {
            is_alive: false,
            ttl: None,
            latency_ms: None,
        },
    }
}

/// Generate a random ping identifier
fn rand_id() -> u16 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 65535) as u16
}
