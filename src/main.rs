// Modules

mod cli;
mod error;
mod output;
mod scanner;
mod services;

// Imports

use clap::Parser;
use std::process;
use std::time::{Duration, Instant};

use cli::{Args, resolve_ports};
use output::render_results;
use scanner::{resolve_target, scan_ports};

// Entry Point

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(err) = args.validate() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    let ports = match resolve_ports(&args) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let target_ip = match resolve_target(&args.target).await {
        Ok(ip) => ip,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let output_format = args.get_output_format();

    loop {
        let start_time = Instant::now();
        let results = scan_ports(
            target_ip,
            ports.clone(),
            args.concurrency,
            args.timeout,
            args.no_service_hints,
        )
        .await;
        let elapsed_secs = start_time.elapsed().as_secs_f64();

        render_results(
            output_format,
            &args.target,
            target_ip,
            &results,
            args.open_only,
            args.quiet,
            elapsed_secs,
        );

        if !args.watch {
            break;
        }

        tokio::time::sleep(Duration::from_secs(args.watch_interval)).await;
    }
}
