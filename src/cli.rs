use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "ip-scanner")]
#[command(version = "0.1.0")]
#[command(about = "Fast network IP scanner with MAC, hostname, OS detection and port scanning")]
#[command(author = "Federico Modolo")]
pub struct Args {
    /// Subnet in CIDR notation (e.g., 192.168.1.0/24)
    pub subnet: String,

    /// Ping timeout in seconds
    #[arg(short = 't', long = "timeout", default_value = "1")]
    pub timeout: u64,

    /// Enable hostname and OS detection
    #[arg(short = 'd', long = "detect")]
    pub detect: bool,

    /// Enable common port scanning (15 ports)
    #[arg(short = 'p', long = "port-scan")]
    pub port_scan: bool,

    /// Extended port scanning (100+ infrastructure ports)
    #[arg(short = 'e', long = "extended-scan")]
    pub extended_scan: bool,

    /// Output format
    #[arg(short = 'o', long = "output", default_value = "table")]
    pub output: OutputFormat,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}
