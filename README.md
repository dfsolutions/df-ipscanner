# DF Solutions IP Scanner

Fast CLI network scanner written in Rust.

## Features

- ICMP ping host discovery
- MAC address via ARP
- OS detection from TTL (Linux/Windows/Network Device)
- Hostname via reverse DNS
- Port scanning: common (15 ports) or extended (100+ infrastructure ports)
- Table and JSON output

## Installation

Download the latest release for your platform from [Releases](https://github.com/dfsolutions/df-ipscanner/releases).

Or build from source:
```bash
cargo build --release
```

## Usage

```bash
# Basic scan (IP + MAC)
ip-scanner 192.168.1.0/24

# With custom timeout (seconds)
ip-scanner 192.168.1.0/24 -t 2

# With hostname and OS detection
ip-scanner 192.168.1.0/24 -d

# With port scan (15 common ports)
ip-scanner 192.168.1.0/24 -p

# Extended port scan (100+ infrastructure ports)
ip-scanner 192.168.1.0/24 -e

# Full scan
ip-scanner 192.168.1.0/24 -t 2 -d -e

# JSON output
ip-scanner 192.168.1.0/24 -d -o json
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `<SUBNET>` | Subnet in CIDR notation | required |
| `-t, --timeout` | Ping timeout in seconds | 1 |
| `-d, --detect` | Enable hostname and OS detection | off |
| `-p, --port-scan` | Scan 15 common ports | off |
| `-e, --extended-scan` | Scan 100+ infrastructure ports | off |
| `-o, --output` | Output format: table, json | table |

## Extended Ports (-e)

Includes ports for: Proxmox, Docker, Kubernetes, Grafana, Prometheus, Elasticsearch, MongoDB, Redis, RabbitMQ, VMware, Synology, Home Assistant, and many more.

## Example Output

```
  DF Solutions Port Scanner

Scanning 254 hosts in 192.168.1.0/24 (timeout: 1s)

┌──────────────┬───────────────────┬──────────────┬─────────┬──────────────┬─────────┐
│ IP Address   │ MAC Address       │ Hostname     │ OS      │ Open Ports   │ Latency │
├──────────────┼───────────────────┼──────────────┼─────────┼──────────────┼─────────┤
│ 192.168.1.1  │ AA:BB:CC:DD:EE:FF │ router.local │ Network │ 80, 443      │ 1.2ms   │
│ 192.168.1.10 │ 11:22:33:44:55:66 │ desktop-pc   │ Windows │ 445, 3389    │ 0.8ms   │
│ 192.168.1.20 │ 77:88:99:AA:BB:CC │ server       │ Linux   │ 22, 80, 443  │ 0.5ms   │
└──────────────┴───────────────────┴──────────────┴─────────┴──────────────┴─────────┘

Scan completed: 3 host(s) found
```

## Platform Notes

### Windows
- Run as **Administrator** for best results (ICMP + ARP)
- Windows Firewall may block some ICMP responses

### macOS / Linux
- May require `sudo` for ICMP raw sockets
- ARP lookup reads from system ARP cache

## License

MIT

## Author

DF Solutions
