// Scanner Data Types

use serde::Serialize;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::{TcpStream, lookup_host};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::timeout;

use crate::error::PortPeekError;
use crate::services::get_service_hint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PortStatus {
    Open,
    Closed,
    Timeout,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub port: u16,
    pub status: PortStatus,
    pub latency_ms: Option<f64>,
    pub service: Option<&'static str>,
}

// Target Resolution

pub async fn resolve_target(target: &str) -> Result<IpAddr, PortPeekError> {
    let clean_target = target.trim_matches(|c| c == '[' || c == ']');
    if let Ok(ip) = clean_target.parse::<IpAddr>() {
        return Ok(ip);
    }

    let socket_str = format!("{}:80", target);
    let mut addrs =
        lookup_host(&socket_str)
            .await
            .map_err(|e| PortPeekError::DnsResolutionFailed {
                target: target.to_string(),
                reason: e.to_string(),
            })?;

    if let Some(socket_addr) = addrs.next() {
        Ok(socket_addr.ip())
    } else {
        Err(PortPeekError::DnsResolutionFailed {
            target: target.to_string(),
            reason: "No IP addresses returned".to_string(),
        })
    }
}

// Single Port Scan

pub async fn scan_port(
    target_ip: IpAddr,
    port: u16,
    timeout_ms: u64,
    no_hints: bool,
) -> ScanResult {
    let socket_addr = SocketAddr::new(target_ip, port);
    let start = Instant::now();

    let connect_future = TcpStream::connect(socket_addr);
    let result = timeout(Duration::from_millis(timeout_ms), connect_future).await;

    match result {
        Ok(Ok(_stream)) => {
            let latency = start.elapsed().as_secs_f64() * 1000.0;
            ScanResult {
                port,
                status: PortStatus::Open,
                latency_ms: Some((latency * 10.0).round() / 10.0),
                service: get_service_hint(port, no_hints),
            }
        }
        Ok(Err(e)) => {
            let status = match e.kind() {
                std::io::ErrorKind::TimedOut => PortStatus::Timeout,
                _ => PortStatus::Closed,
            };
            ScanResult {
                port,
                status,
                latency_ms: None,
                service: get_service_hint(port, no_hints),
            }
        }
        Err(_) => ScanResult {
            port,
            status: PortStatus::Timeout,
            latency_ms: None,
            service: get_service_hint(port, no_hints),
        },
    }
}

// Concurrent Scan Execution

pub async fn scan_ports(
    target_ip: IpAddr,
    ports: Vec<u16>,
    concurrency: usize,
    timeout_ms: u64,
    no_hints: bool,
) -> Vec<ScanResult> {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut join_set = JoinSet::new();

    for port in ports {
        let sem = semaphore.clone();
        join_set.spawn(async move {
            let _permit = sem
                .acquire()
                .await
                .expect("Failed to acquire semaphore permit");
            scan_port(target_ip, port, timeout_ms, no_hints).await
        });
    }

    let mut results = Vec::new();
    while let Some(res) = join_set.join_next().await {
        if let Ok(scan_res) = res {
            results.push(scan_res);
        }
    }

    results.sort_by_key(|r| r.port);
    results
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_resolve_target_ipv4() {
        let ip = resolve_target("127.0.0.1").await.unwrap();
        assert_eq!(ip.to_string(), "127.0.0.1");
    }

    #[tokio::test]
    async fn test_resolve_target_ipv6() {
        let ip = resolve_target("::1").await.unwrap();
        assert_eq!(ip.to_string(), "::1");

        let ip_bracketed = resolve_target("[::1]").await.unwrap();
        assert_eq!(ip_bracketed.to_string(), "::1");
    }

    #[tokio::test]
    async fn test_scan_port_open_and_closed() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        let open_res = scan_port("127.0.0.1".parse().unwrap(), port, 500, false).await;
        assert_eq!(open_res.status, PortStatus::Open);
        assert!(open_res.latency_ms.is_some());

        drop(listener);

        let closed_res = scan_port("127.0.0.1".parse().unwrap(), port, 500, false).await;
        assert_ne!(closed_res.status, PortStatus::Open);
    }
}
