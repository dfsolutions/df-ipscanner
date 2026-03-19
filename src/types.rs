use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub ip: Ipv4Addr,
    pub mac: Option<String>,
    pub hostname: Option<String>,
    pub os_guess: Option<String>,
    pub ttl: Option<u8>,
    pub open_ports: Vec<u16>,
    pub latency_ms: Option<f64>,
}

/// Common ports to scan when -p flag is used
pub const COMMON_PORTS: &[u16] = &[
    21,   // FTP
    22,   // SSH
    23,   // Telnet
    25,   // SMTP
    53,   // DNS
    80,   // HTTP
    110,  // POP3
    143,  // IMAP
    443,  // HTTPS
    445,  // SMB
    3306, // MySQL
    3389, // RDP
    5432, // PostgreSQL
    8080, // HTTP Proxy
    8443, // HTTPS Alt
];

/// Extended ports to scan when -e flag is used (includes infrastructure services)
pub const EXTENDED_PORTS: &[u16] = &[
    // Standard services
    21,    // FTP
    22,    // SSH
    23,    // Telnet
    25,    // SMTP
    53,    // DNS
    67,    // DHCP Server
    68,    // DHCP Client
    69,    // TFTP
    80,    // HTTP
    110,   // POP3
    111,   // RPCbind
    123,   // NTP
    135,   // MS RPC
    137,   // NetBIOS Name
    138,   // NetBIOS Datagram
    139,   // NetBIOS Session
    143,   // IMAP
    161,   // SNMP
    162,   // SNMP Trap
    389,   // LDAP
    443,   // HTTPS
    445,   // SMB
    464,   // Kerberos
    514,   // Syslog
    515,   // LPD Print
    587,   // SMTP Submission
    636,   // LDAPS
    873,   // Rsync
    902,   // VMware ESXi
    993,   // IMAPS
    995,   // POP3S
    1080,  // SOCKS Proxy
    1194,  // OpenVPN
    1433,  // MSSQL
    1434,  // MSSQL Browser
    1521,  // Oracle DB
    1723,  // PPTP VPN
    1883,  // MQTT
    2049,  // NFS
    2082,  // cPanel
    2083,  // cPanel SSL
    2181,  // Zookeeper
    2375,  // Docker API
    2376,  // Docker API SSL
    3000,  // Grafana / Node.js
    3128,  // Squid Proxy
    3268,  // AD Global Catalog
    3269,  // AD Global Catalog SSL
    3306,  // MySQL
    3389,  // RDP
    4443,  // Pharos
    5000,  // Synology DSM / Flask
    5001,  // Synology DSM SSL
    5432,  // PostgreSQL
    5672,  // RabbitMQ
    5900,  // VNC
    5901,  // VNC :1
    5985,  // WinRM HTTP
    5986,  // WinRM HTTPS
    6379,  // Redis
    6443,  // Kubernetes API
    7443,  // WSO2
    8006,  // Proxmox VE
    8008,  // HTTP Alt
    8080,  // HTTP Proxy / Tomcat
    8081,  // HTTP Alt
    8083,  // Mosquitto WS
    8086,  // InfluxDB
    8123,  // Home Assistant
    8200,  // Vault
    8443,  // HTTPS Alt
    8444,  // HTTPS Alt
    8500,  // Consul
    8834,  // Nessus
    8888,  // HTTP Alt
    9000,  // Portainer / SonarQube
    9090,  // Prometheus / Cockpit
    9091,  // Transmission
    9100,  // Printer JetDirect
    9200,  // Elasticsearch
    9300,  // Elasticsearch
    9418,  // Git
    9443,  // VMware vSphere
    10000, // Webmin
    10250, // Kubelet
    10443, // HTTPS Alt
    11211, // Memcached
    15672, // RabbitMQ Management
    16443, // K8s Alt
    27017, // MongoDB
    27018, // MongoDB
    28017, // MongoDB Web
    32400, // Plex
    50000, // Jenkins
    51820, // WireGuard
];
