// Integration Tests

use assert_cmd::Command;
use predicates::prelude::*;
use std::net::IpAddr;
use tokio::net::TcpListener;

use portpeek::scanner::{PortStatus, scan_port, scan_ports};

// TCP Server Tests

#[tokio::test]
async fn test_ipv4_open_and_closed_ports() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let open_port = listener.local_addr().unwrap().port();
    let target_ip: IpAddr = "127.0.0.1".parse().unwrap();

    let open_result = scan_port(target_ip, open_port, 500, false).await;
    assert_eq!(open_result.status, PortStatus::Open);
    assert!(open_result.latency_ms.is_some());

    drop(listener);

    let closed_result = scan_port(target_ip, open_port, 500, false).await;
    assert_eq!(closed_result.status, PortStatus::Closed);
}

// IPv6 Tests

#[tokio::test]
async fn test_ipv6_scan() {
    if let Ok(listener) = TcpListener::bind("[::1]:0").await {
        let open_port = listener.local_addr().unwrap().port();
        let target_ip: IpAddr = "::1".parse().unwrap();

        let open_result = scan_port(target_ip, open_port, 500, false).await;
        assert_eq!(open_result.status, PortStatus::Open);

        drop(listener);

        let closed_result = scan_port(target_ip, open_port, 500, false).await;
        assert_eq!(closed_result.status, PortStatus::Closed);
    }
}

// Timeout Integration Test

#[tokio::test]
async fn test_timeout_detection() {
    let target_ip: IpAddr = "192.0.2.1".parse().unwrap();
    let result = scan_port(target_ip, 80, 200, false).await;
    assert_eq!(result.status, PortStatus::Timeout);
}

// Concurrency Sorting Test

#[tokio::test]
async fn test_concurrency_and_sorting() {
    let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
    let ports = vec![30005, 30001, 30003, 30002, 30004];

    let results = scan_ports(target_ip, ports, 5, 200, false).await;
    assert_eq!(results.len(), 5);
    for window in results.windows(2) {
        assert!(window[0].port <= window[1].port);
    }
}

// CLI Integration Tests

#[test]
fn test_cli_quiet_mode() {
    let mut cmd = Command::cargo_bin("portpeek").unwrap();
    cmd.arg("127.0.0.1")
        .arg("--ports")
        .arg("59990")
        .arg("--quiet");

    cmd.assert().success();
}

#[test]
fn test_cli_json_output() {
    let mut cmd = Command::cargo_bin("portpeek").unwrap();
    cmd.arg("127.0.0.1")
        .arg("--ports")
        .arg("59991")
        .arg("--output")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"target\": \"127.0.0.1\""))
        .stdout(predicate::str::contains("\"total_scanned\": 1"));
}

#[test]
fn test_cli_csv_output() {
    let mut cmd = Command::cargo_bin("portpeek").unwrap();
    cmd.arg("127.0.0.1")
        .arg("--ports")
        .arg("59992")
        .arg("--output")
        .arg("csv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "port,protocol,status,service,latency_ms",
        ))
        .stdout(predicate::str::contains("59992,tcp,CLOSED"));
}

#[test]
fn test_cli_invalid_range_error() {
    let mut cmd = Command::cargo_bin("portpeek").unwrap();
    cmd.arg("127.0.0.1").arg("--ports").arg("500-100");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid port range"));
}

#[test]
fn test_cli_no_service_hints() {
    let mut cmd = Command::cargo_bin("portpeek").unwrap();
    cmd.arg("127.0.0.1")
        .arg("--ports")
        .arg("22")
        .arg("--no-service-hints")
        .arg("--output")
        .arg("csv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("22,tcp,CLOSED,,"));
}
