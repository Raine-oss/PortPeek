// Output Formatter

use colored::Colorize;
use serde::Serialize;
use std::net::IpAddr;

use crate::cli::OutputFormat;
use crate::scanner::{PortStatus, ScanResult};

// JSON Structures

#[derive(Serialize)]
pub struct JsonReport<'a> {
    pub target: &'a str,
    pub ip: String,
    pub total_scanned: usize,
    pub open_count: usize,
    pub duration_seconds: f64,
    pub results: &'a [ScanResult],
}

// Terminal Table Rendering

pub fn render_terminal(
    target: &str,
    target_ip: IpAddr,
    results: &[ScanResult],
    open_only: bool,
    elapsed_secs: f64,
) {
    let display_results: Vec<&ScanResult> = if open_only {
        results
            .iter()
            .filter(|r| r.status == PortStatus::Open)
            .collect()
    } else {
        results.iter().collect()
    };

    let open_count = results
        .iter()
        .filter(|r| r.status == PortStatus::Open)
        .count();

    println!();
    println!("{}", "PortPeek".bold().cyan());
    println!(
        "Target: {} ({})",
        target.bold(),
        target_ip.to_string().dimmed()
    );
    println!();

    println!(
        "{:<12} {:<10} {:<24} {:<10}",
        "PORT".bold(),
        "STATUS".bold(),
        "SERVICE".bold(),
        "LATENCY".bold()
    );
    println!("{}", "─".repeat(58).dimmed());

    for r in &display_results {
        let port_str = format!("{}/tcp", r.port);
        let raw_status = match r.status {
            PortStatus::Open => "OPEN",
            PortStatus::Closed => "CLOSED",
            PortStatus::Timeout => "TIMEOUT",
        };
        let padded_status = format!("{:<10}", raw_status);
        let status_str = match r.status {
            PortStatus::Open => padded_status.green().bold().to_string(),
            PortStatus::Closed => padded_status.red().to_string(),
            PortStatus::Timeout => padded_status.yellow().to_string(),
        };

        let service_str = r.service.unwrap_or("-");
        let latency_str = match r.latency_ms {
            Some(lat) => format!("{:.1}ms", lat),
            None => "-".to_string(),
        };

        println!(
            "{:<12} {} {:<24} {:<10}",
            port_str, status_str, service_str, latency_str
        );
    }

    println!("{}", "─".repeat(58).dimmed());
    println!("Scanned {} ports in {:.2}s", results.len(), elapsed_secs);
    println!(
        "{} open ports",
        if open_count > 0 {
            open_count.to_string().green().bold()
        } else {
            open_count.to_string().normal()
        }
    );
    println!();
}

// JSON Output Rendering

pub fn render_json(
    target: &str,
    target_ip: IpAddr,
    results: &[ScanResult],
    open_only: bool,
    elapsed_secs: f64,
) {
    let filtered_results: Vec<ScanResult> = if open_only {
        results
            .iter()
            .filter(|r| r.status == PortStatus::Open)
            .cloned()
            .collect()
    } else {
        results.to_vec()
    };

    let open_count = results
        .iter()
        .filter(|r| r.status == PortStatus::Open)
        .count();

    let report = JsonReport {
        target,
        ip: target_ip.to_string(),
        total_scanned: results.len(),
        open_count,
        duration_seconds: (elapsed_secs * 100.0).round() / 100.0,
        results: &filtered_results,
    };

    if let Ok(json_string) = serde_json::to_string_pretty(&report) {
        println!("{}", json_string);
    }
}

// CSV Output Rendering

pub fn render_csv(results: &[ScanResult], open_only: bool) {
    println!("port,protocol,status,service,latency_ms");

    for r in results {
        if open_only && r.status != PortStatus::Open {
            continue;
        }

        let status_str = match r.status {
            PortStatus::Open => "OPEN",
            PortStatus::Closed => "CLOSED",
            PortStatus::Timeout => "TIMEOUT",
        };

        let service_str = r.service.unwrap_or("");
        let latency_str = match r.latency_ms {
            Some(lat) => format!("{:.1}", lat),
            None => String::new(),
        };

        println!(
            "{},tcp,{},{},{}",
            r.port, status_str, service_str, latency_str
        );
    }
}

// Quiet Output Rendering

pub fn render_quiet(results: &[ScanResult]) {
    for r in results {
        if r.status == PortStatus::Open {
            println!("{}/tcp", r.port);
        }
    }
}

// Unified Render Dispatcher

pub fn render_results(
    format: OutputFormat,
    target: &str,
    target_ip: IpAddr,
    results: &[ScanResult],
    open_only: bool,
    quiet: bool,
    elapsed_secs: f64,
) {
    if quiet {
        render_quiet(results);
        return;
    }

    match format {
        OutputFormat::Terminal => {
            render_terminal(target, target_ip, results, open_only, elapsed_secs);
        }
        OutputFormat::Json => {
            render_json(target, target_ip, results, open_only, elapsed_secs);
        }
        OutputFormat::Csv => {
            render_csv(results, open_only);
        }
    }
}
