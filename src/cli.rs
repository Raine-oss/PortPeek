// CLI Arguments

use clap::{Parser, ValueEnum};
use std::collections::BTreeSet;

use crate::error::PortPeekError;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Terminal,
    Json,
    Csv,
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "portpeek",
    version,
    about = "A fast and lightweight terminal TCP port scanner"
)]
pub struct Args {
    #[arg(help = "Target hostname or IP address (e.g., localhost, 127.0.0.1)")]
    pub target: String,

    #[arg(
        short = 'p',
        long = "ports",
        help = "Ports to scan (e.g. 22,80,443 or 1-1024)"
    )]
    pub ports: Option<String>,

    #[arg(
        short = 't',
        long = "timeout",
        default_value_t = 1000,
        help = "Connection timeout in milliseconds"
    )]
    pub timeout: u64,

    #[arg(
        short = 'c',
        long = "concurrency",
        default_value_t = 100,
        help = "Maximum concurrent connections"
    )]
    pub concurrency: usize,

    #[arg(
        short = 'o',
        long = "output",
        value_enum,
        help = "Output format: terminal, json, csv"
    )]
    pub output: Option<OutputFormat>,

    #[arg(
        long = "json",
        help = "Output results in JSON format (shorthand for --output json)"
    )]
    pub json: bool,

    #[arg(long = "open-only", help = "Display only open ports")]
    pub open_only: bool,

    #[arg(
        short = 'q',
        long = "quiet",
        help = "Quiet output showing only open ports for scripting"
    )]
    pub quiet: bool,

    #[arg(
        short = 'w',
        long = "watch",
        help = "Continuously re-scan target at intervals"
    )]
    pub watch: bool,

    #[arg(
        long = "watch-interval",
        default_value_t = 2,
        help = "Interval in seconds between re-scans in watch mode"
    )]
    pub watch_interval: u64,

    #[arg(
        long = "no-service-hints",
        help = "Disable common service hint lookups"
    )]
    pub no_service_hints: bool,

    #[arg(
        long = "minecraft",
        help = "Preset to quickly inspect common Minecraft ports"
    )]
    pub minecraft: bool,

    #[arg(
        long = "fail-on-closed",
        visible_alias = "fail-if-closed",
        help = "Exit with code 2 if any scanned port is closed or timed out"
    )]
    pub fail_on_closed: bool,

    #[arg(
        long = "fail-if-none-open",
        help = "Exit with code 2 if no scanned ports are open"
    )]
    pub fail_if_none_open: bool,
}

// Helper Implementations

impl Args {
    pub fn get_output_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            self.output.unwrap_or(OutputFormat::Terminal)
        }
    }

    pub fn validate(&self) -> Result<(), PortPeekError> {
        if self.concurrency == 0 {
            return Err(PortPeekError::InvalidConcurrency(self.concurrency));
        }
        if self.timeout == 0 {
            return Err(PortPeekError::InvalidTimeout(self.timeout));
        }
        Ok(())
    }
}

// Port Parsing

pub fn resolve_ports(args: &Args) -> Result<Vec<u16>, PortPeekError> {
    if args.minecraft {
        return Ok(vec![25565]);
    }

    if let Some(ref ports_str) = args.ports {
        let mut port_set = BTreeSet::new();

        for part in ports_str.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if let Some((start_str, end_str)) = part.split_once('-') {
                let start: u16 = start_str
                    .trim()
                    .parse()
                    .map_err(|_| PortPeekError::InvalidPortNumber(start_str.to_string()))?;
                let end: u16 = end_str
                    .trim()
                    .parse()
                    .map_err(|_| PortPeekError::InvalidPortNumber(end_str.to_string()))?;

                if start > end {
                    return Err(PortPeekError::InvalidPortRange { start, end });
                }

                for p in start..=end {
                    port_set.insert(p);
                }
            } else {
                let p: u16 = part
                    .parse()
                    .map_err(|_| PortPeekError::InvalidPortNumber(part.to_string()))?;
                port_set.insert(p);
            }
        }

        if port_set.is_empty() {
            return Err(PortPeekError::NoPortsSpecified);
        }

        Ok(port_set.into_iter().collect())
    } else {
        Ok(vec![
            21, 22, 25, 53, 80, 110, 143, 443, 3000, 3306, 5432, 6379, 8000, 8080, 8443, 25565,
        ])
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_args(ports: Option<String>, minecraft: bool) -> Args {
        Args {
            target: "localhost".to_string(),
            ports,
            timeout: 1000,
            concurrency: 100,
            output: None,
            json: false,
            open_only: false,
            quiet: false,
            watch: false,
            watch_interval: 2,
            no_service_hints: false,
            minecraft,
            fail_on_closed: false,
            fail_if_none_open: false,
        }
    }

    #[test]
    fn test_default_ports() {
        let args = create_test_args(None, false);
        let ports = resolve_ports(&args).unwrap();
        assert!(ports.contains(&22));
        assert!(ports.contains(&80));
        assert!(ports.contains(&25565));
    }

    #[test]
    fn test_single_and_comma_ports() {
        let args = create_test_args(Some("22,80,443".to_string()), false);
        let ports = resolve_ports(&args).unwrap();
        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn test_duplicate_ports_deduplication() {
        let args = create_test_args(Some("80,80,443,80,443,22".to_string()), false);
        let ports = resolve_ports(&args).unwrap();
        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn test_port_range() {
        let args = create_test_args(Some("80-83".to_string()), false);
        let ports = resolve_ports(&args).unwrap();
        assert_eq!(ports, vec![80, 81, 82, 83]);
    }

    #[test]
    fn test_port_overflow_range() {
        let args = create_test_args(Some("65535-65536".to_string()), false);
        let res = resolve_ports(&args);
        assert!(res.is_err());
        match res.unwrap_err() {
            PortPeekError::InvalidPortNumber(val) => assert_eq!(val, "65536"),
            other => panic!("Unexpected error: {:?}", other),
        }
    }

    #[test]
    fn test_minecraft_flag() {
        let args = create_test_args(None, true);
        let ports = resolve_ports(&args).unwrap();
        assert_eq!(ports, vec![25565]);
    }

    #[test]
    fn test_invalid_range() {
        let args = create_test_args(Some("100-50".to_string()), false);
        assert!(resolve_ports(&args).is_err());
    }

    #[test]
    fn test_validation() {
        let mut args = create_test_args(None, false);
        assert!(args.validate().is_ok());

        args.concurrency = 0;
        let err = args.validate().unwrap_err();
        match err {
            PortPeekError::InvalidConcurrency(c) => assert_eq!(c, 0),
            other => panic!("Unexpected error: {:?}", other),
        }

        args.concurrency = 100;
        args.timeout = 0;
        assert!(args.validate().is_err());
    }

    #[test]
    fn test_output_format_fallback() {
        let mut args = create_test_args(None, false);
        assert_eq!(args.get_output_format(), OutputFormat::Terminal);

        args.json = true;
        assert_eq!(args.get_output_format(), OutputFormat::Json);

        args.json = false;
        args.output = Some(OutputFormat::Csv);
        assert_eq!(args.get_output_format(), OutputFormat::Csv);
    }
}
