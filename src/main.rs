mod cli;
mod error;
mod network;
mod os_detect;
mod output;
mod scanner;
mod types;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Semaphore;

use network::cidr::generate_ip_range;
use output::formatter::format_results;
use scanner::{arp::get_mac_address, dns::reverse_lookup, ping::ping_host, port::scan_ports};
use types::ScanResult;

const MAX_CONCURRENT: usize = 256;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Parse CIDR and generate IP list
    let hosts = generate_ip_range(&args.subnet)?;
    let total_hosts = hosts.len();

    if total_hosts == 0 {
        println!("No hosts to scan in the specified subnet.");
        return Ok(());
    }

    println!("\n  DF Solutions Port Scanner\n");
    println!(
        "Scanning {} hosts in {} (timeout: {}s)",
        total_hosts, args.subnet, args.timeout
    );

    // Setup progress bar
    let pb = ProgressBar::new(total_hosts as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Semaphore for rate limiting
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    let timeout_secs = args.timeout;
    let detect_host = args.detect;
    let scan_ports_flag = args.port_scan || args.extended_scan;
    let extended_scan = args.extended_scan;

    // Parallel scan
    let results: Vec<ScanResult> = stream::iter(hosts)
        .map(|ip| {
            let sem = semaphore.clone();
            let pb = pb.clone();
            async move {
                let _permit = sem.acquire().await.unwrap();

                // Ping host
                let ping_result = ping_host(ip, timeout_secs).await;
                pb.inc(1);

                if !ping_result.is_alive {
                    return None;
                }

                // Get MAC address
                let mac = get_mac_address(ip).await;

                // Hostname and OS detection (if -d flag)
                let (hostname, os_guess) = if detect_host {
                    let hostname = reverse_lookup(ip).await;
                    let os_guess = ping_result
                        .ttl
                        .map(|ttl| os_detect::ttl::guess_os_from_ttl(ttl).to_string());
                    (hostname, os_guess)
                } else {
                    (None, None)
                };

                // Port scan (if -p or -e flag)
                let open_ports = if scan_ports_flag {
                    scan_ports(ip, timeout_secs, extended_scan).await
                } else {
                    vec![]
                };

                Some(ScanResult {
                    ip,
                    mac,
                    hostname,
                    os_guess,
                    ttl: ping_result.ttl,
                    open_ports,
                    latency_ms: ping_result.latency_ms,
                })
            }
        })
        .buffer_unordered(MAX_CONCURRENT)
        .filter_map(|r| async { r })
        .collect()
        .await;

    pb.finish_and_clear();

    // Output results
    format_results(&results, &args);

    Ok(())
}
