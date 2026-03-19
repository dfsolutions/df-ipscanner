use crate::cli::{Args, OutputFormat};
use crate::types::ScanResult;
use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};
use serde::Serialize;

#[derive(Serialize)]
struct JsonOutput {
    subnet: String,
    hosts_found: usize,
    results: Vec<ScanResult>,
}

/// Format and print scan results
pub fn format_results(results: &[ScanResult], args: &Args) {
    match args.output {
        OutputFormat::Table => print_table(results, args),
        OutputFormat::Json => print_json(results, args),
    }
}

fn print_table(results: &[ScanResult], args: &Args) {
    if results.is_empty() {
        println!("\nNo hosts found.");
        return;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    // Build header based on flags
    let mut headers = vec!["IP Address", "MAC Address"];
    if args.detect {
        headers.push("Hostname");
        headers.push("OS");
    }
    if args.port_scan || args.extended_scan {
        headers.push("Open Ports");
    }
    headers.push("Latency");

    table.set_header(headers);

    // Add rows
    for result in results {
        let mut row = vec![
            Cell::new(&result.ip.to_string()),
            Cell::new(result.mac.as_deref().unwrap_or("-")),
        ];

        if args.detect {
            row.push(Cell::new(result.hostname.as_deref().unwrap_or("-")));
            let os_cell = match result.os_guess.as_deref() {
                Some("Windows") => Cell::new("Windows").fg(Color::Cyan),
                Some("Linux/Unix") => Cell::new("Linux/Unix").fg(Color::Green),
                Some("Network Device") => Cell::new("Network Device").fg(Color::Yellow),
                Some(other) => Cell::new(other),
                None => Cell::new("-"),
            };
            row.push(os_cell);
        }

        if args.port_scan || args.extended_scan {
            let ports_str = if result.open_ports.is_empty() {
                "-".to_string()
            } else {
                result
                    .open_ports
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            row.push(Cell::new(ports_str));
        }

        let latency_str = result
            .latency_ms
            .map(|ms| format!("{:.1}ms", ms))
            .unwrap_or_else(|| "-".to_string());
        row.push(Cell::new(latency_str));

        table.add_row(row);
    }

    println!("\n{}", table);
    println!(
        "\nScan completed: {} host(s) found",
        results.len()
    );
}

fn print_json(results: &[ScanResult], args: &Args) {
    let output = JsonOutput {
        subnet: args.subnet.clone(),
        hosts_found: results.len(),
        results: results.to_vec(),
    };

    match serde_json::to_string_pretty(&output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}
