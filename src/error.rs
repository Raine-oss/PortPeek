// Error Definitions

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortPeekError {
    #[error("Invalid port range: {start}-{end} (start must be <= end)")]
    InvalidPortRange { start: u16, end: u16 },

    #[error("Invalid port number: '{0}'")]
    InvalidPortNumber(String),

    #[error("No valid ports specified")]
    NoPortsSpecified,

    #[error("Failed to resolve target '{target}': {reason}")]
    DnsResolutionFailed { target: String, reason: String },

    #[error("Invalid concurrency value: {0} (must be greater than 0)")]
    InvalidConcurrency(usize),

    #[error("Invalid timeout value: {0} (must be greater than 0)")]
    InvalidTimeout(u64),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
