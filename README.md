# PortPeek

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Release](https://img.shields.io/github/v/release/Raine-oss/PortPeek?color=blue)](https://github.com/Raine-oss/PortPeek/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-success.svg)](https://github.com/Raine-oss/PortPeek/actions)
[![Tests](https://img.shields.io/badge/tests-35%20passed-success.svg)](https://github.com/Raine-oss/PortPeek)

A fast, lightweight terminal TCP port scanner for connectivity checks and service troubleshooting, built with Rust. Uses standard asynchronous TCP connect scans, controlled concurrency via semaphores, latency measurement, and multi-format outputs (Terminal, JSON, CSV).

[![View Releases](https://img.shields.io/badge/GitHub-Releases-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Raine-oss/PortPeek/releases)
[![Report Issue](https://img.shields.io/badge/GitHub-Issues-blue?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Raine-oss/PortPeek/issues)
[![Read Documentation](https://img.shields.io/badge/Docs-Release_Notes-2ea44f?style=for-the-badge&logo=readme&logoColor=white)](RELEASE_NOTES.md)

---

## Project Overview

PortPeek is designed specifically for developers, system administrators, and infrastructure engineers who need an immediate, dependable way to verify listening TCP services without the overhead or complexity of comprehensive security auditing frameworks.

It does not attempt to replace large-scale network mappers like Nmap. Instead, it focuses on common engineering tasks: verifying whether a local database is up, checking if a web application started on port 3000 or 8000, confirming firewall reachability to a remote host, or monitoring a port during service restarts.

### Core Capabilities

- **Real TCP Connect Scanning**: Executes complete three-way TCP handshakes without requiring root, administrative privileges, or raw socket drivers.
- **Controlled Concurrency**: Scales port checks concurrently using asynchronous tasks bounded by an asynchronous semaphore to prevent file descriptor exhaustion.
- **Accurate Connection Classification**: Explicitly distinguishes between `OPEN` (successful handshake), `CLOSED` (connection refused by target host), and `TIMEOUT` (deadline expired without response).
- **Latency Measurement**: Measures connection latency for all responsive open ports.
- **Common Service Hints**: Maps port numbers to common standard services (SSH, HTTP, MySQL, PostgreSQL, Redis, Kafka, Minecraft) with an option to disable lookups.
- **Multiple Output Formats**: Supports human-readable terminal tables, machine-readable JSON, comma-separated values (CSV), and a minimal quiet mode for shell scripting.
- **Continuous Watch Mode**: Re-scans targets at configurable intervals for monitoring application startup and shutdown transitions.
- **Dual-Stack Support**: Resolves and connects transparently over both IPv4 and IPv6 networks.
- **Structured Error Engine**: Employs typed error handling with explicit diagnostic messages.

---

## Platform Support Matrix

| Operating System | Architecture | Binary Support | Status |
| :--- | :--- | :--- | :--- |
| **Linux** | x86_64 | Standalone executable | Verified |
| **Linux** | ARM64 (aarch64) | Standalone executable | Verified |
| **macOS** | Apple Silicon (aarch64) | Standalone executable | Verified |
| **macOS** | Intel (x86_64) | Standalone executable | Verified |
| **Windows** | x86_64 | Standalone executable | Verified |

---

## Explicit Scope and Non-Goals

To maintain high performance, reliability, and architectural simplicity, PortPeek deliberately excludes:
- **UDP Scanning**: UDP scanning is connectionless and requires protocol-specific heuristics or ICMP unreachable parsing.
- **SYN / Stealth Scanning**: Requires raw sockets, administrative privileges (root/sudo), and platform-specific network drivers.
- **Operating System Fingerprinting**: TCP stack fingerprinting introduces substantial complexity and network noise.
- **Banner Grabbing**: Probing for service banners can trigger security alerts and cause unexpected state changes in listening daemons.
- **Vulnerability Assessment**: PortPeek is a connectivity tool, not an exploit or CVE scanner.
- **CIDR / Subnet Scanning**: Scanning arbitrary network blocks is outside the scope of single-target connectivity checks.

---

## Installation

### Method 1: Cargo Install (From Source Repository)

Ensure you have a recent stable Rust toolchain installed:

```bash
cargo install --git https://github.com/Raine-oss/PortPeek.git
```

### Method 2: Download Prebuilt Release Binaries

Download precompiled, standalone binaries from the [GitHub Releases page](https://github.com/Raine-oss/PortPeek/releases):

```bash
# Example for Linux x86_64
curl -LO https://github.com/Raine-oss/PortPeek/releases/download/v0.2.0/portpeek-x86_64-unknown-linux-gnu.tar.gz
tar -xzf portpeek-x86_64-unknown-linux-gnu.tar.gz
chmod +x portpeek
sudo mv portpeek /usr/local/bin/
```

### Method 3: Build From Source

```bash
git clone https://github.com/Raine-oss/PortPeek.git
cd PortPeek
cargo build --release
sudo cp target/release/portpeek /usr/local/bin/
```

---

## Command-Line Interface

### Usage Syntax

```text
portpeek [OPTIONS] <TARGET>
```

### Arguments

| Argument | Type | Description |
| :--- | :--- | :--- |
| `<TARGET>` | String | Destination hostname or IP address (e.g., `localhost`, `127.0.0.1`, `::1`, `192.168.1.50`) |

### Options

| Option | Short | Default | Description |
| :--- | :--- | :--- | :--- |
| `--ports <PORTS>` | `-p` | Common ports list | Ports to scan. Accepts single ports (`80`), lists (`22,80,443`), ranges (`1-1024`), or combinations (`22,80,8000-8010`) |
| `--timeout <MS>` | `-t` | `1000` | Connection timeout in milliseconds |
| `--concurrency <N>` | `-c` | `100` | Maximum simultaneous open TCP connections |
| `--output <FORMAT>` | `-o` | `terminal` | Output format: `terminal`, `json`, or `csv` |
| `--json` | | `false` | Shorthand alias for `--output json` |
| `--open-only` | | `false` | Restricts output exclusively to open ports |
| `--quiet` | `-q` | `false` | Suppresses headers and metrics; outputs only open ports (e.g., `8000/tcp`) |
| `--watch` | `-w` | `false` | Re-executes the scan continuously at timed intervals |
| `--watch-interval <S>` | | `2` | Interval in seconds between scans when running in watch mode |
| `--no-service-hints` | | `false` | Bypasses the port-to-service dictionary lookups |
| `--minecraft` | | `false` | Preset flag scanning common Minecraft ports (25565/tcp) |
| `--help` | `-h` | | Displays help information and options |
| `--version` | `-V` | | Displays application version |

---

## Usage Examples

### 1. Default Inspection

When run without `--ports`, PortPeek checks the top 16 standard development and infrastructure ports:

```bash
portpeek localhost
```

```text
PortPeek
Target: localhost (127.0.0.1)

PORT         STATUS     SERVICE                  LATENCY   
----------------------------------------------------------
21/tcp       CLOSED     FTP                      -         
22/tcp       CLOSED     SSH                      -         
25/tcp       CLOSED     SMTP                     -         
53/tcp       CLOSED     DNS                      -         
80/tcp       CLOSED     HTTP                     -         
110/tcp      CLOSED     POP3                     -         
143/tcp      CLOSED     IMAP                     -         
443/tcp      CLOSED     HTTPS                    -         
3000/tcp     CLOSED     Dev Server               -         
3306/tcp     CLOSED     MySQL                    -         
5432/tcp     CLOSED     PostgreSQL               -         
6379/tcp     CLOSED     Redis                    -         
8000/tcp     OPEN       Dev Server               0.1ms     
8080/tcp     CLOSED     HTTP Proxy / Dev         -         
8443/tcp     CLOSED     HTTPS Alt                -         
25565/tcp    CLOSED     Minecraft Java           -         
----------------------------------------------------------
Scanned 16 ports in 0.00s
1 open ports
```

### 2. Custom Port Lists and Ranges

```bash
# Check specific ports
portpeek localhost -p 22,80,443,3000,8000

# Check an inclusive port range
portpeek localhost -p 8000-8010
```

### 3. Filtering Open Ports Only

```bash
portpeek localhost --open-only
```

```text
PortPeek
Target: localhost (127.0.0.1)

PORT         STATUS     SERVICE                  LATENCY   
----------------------------------------------------------
8000/tcp     OPEN       Dev Server               0.1ms     
----------------------------------------------------------
Scanned 16 ports in 0.00s
1 open ports
```

### 4. Machine-Readable JSON Output

```bash
portpeek localhost -p 22,8000 --output json
```

```json
{
  "target": "localhost",
  "ip": "127.0.0.1",
  "total_scanned": 2,
  "open_count": 1,
  "duration_seconds": 0.0,
  "results": [
    {
      "port": 22,
      "status": "CLOSED",
      "latency_ms": null,
      "service": "SSH"
    },
    {
      "port": 8000,
      "status": "OPEN",
      "latency_ms": 0.1,
      "service": "Dev Server"
    }
  ]
}
```

### 5. CSV Export

```bash
portpeek localhost -p 22,8000 --output csv
```

```text
port,protocol,status,service,latency_ms
22,tcp,CLOSED,SSH,
8000,tcp,OPEN,Dev Server,0.1
```

### 6. Shell Script Integration with Quiet Mode

Quiet mode emits only the open port strings (`<port>/tcp`), making it directly consumable by UNIX pipelines:

```bash
portpeek localhost -q
```

```text
8000/tcp
```

Use in shell loops or conditionals:

```bash
if portpeek localhost -p 5432 -q | grep -q "5432"; then
  echo "PostgreSQL is ready."
fi
```

### 7. Continuous Monitoring with Watch Mode

Monitor application startup during background processes or deployment rollouts:

```bash
portpeek localhost -p 8000 --watch --watch-interval 1
```

### 8. IPv6 Target Inspection

PortPeek resolves and connects to IPv6 addresses directly:

```bash
portpeek "::1" -p 8000,22
```

---

## Technical Architecture

```text
               CLI Arguments (cli.rs)
                         |
                         v
             Target Resolution (scanner.rs)
                         |
           +-------------+-------------+
           |                           |
    Target: localhost            Target: [::1]
      IP: 127.0.0.1                IP: ::1
           |                           |
           +-------------+-------------+
                         |
                         v
             Task Dispatch with Semaphore
                 (Bounded Concurrency)
                         |
         +---------------+---------------+
         |               |               |
       Port 22        Port 80        Port 8000
         |               |               |
         v               v               v
    TcpStream       TcpStream       TcpStream
    Connection      Connection      Connection
         |               |               |
         v               v               v
       CLOSED         TIMEOUT          OPEN
      (Refused)      (Expired)      (Latency)
         |               |               |
         +---------------+---------------+
                         |
                         v
             Service Hint Mapping (services.rs)
                         |
                         v
             Ordered Results (Port Ascending)
                         |
         +---------------+---------------+
         |               |               |
         v               v               v
   Terminal Table      JSON             CSV
    (output.rs)     (output.rs)     (output.rs)
```

### Modular Structure

- **`src/cli.rs`**: Argument definitions using `clap`, port range parsing, input validation, and output format resolution.
- **`src/scanner.rs`**: DNS/IP resolution, single-port connect execution with timeout, and semaphore-bounded concurrent scheduling.
- **`src/services.rs`**: Standard port-to-service hint lookup table.
- **`src/output.rs`**: Formatting and output dispatch for Terminal, JSON, CSV, and Quiet modes.
- **`src/error.rs`**: Structured error definitions powered by `thiserror`.
- **`src/main.rs`**: Execution lifecycle coordination and watch mode event loop.

---

## Performance and Benchmarks

PortPeek incorporates a standardized benchmark suite using the `criterion` framework. Benchmarks evaluate scanning latency over a fixed range of 50 local ports under varying concurrency bounds.

### Measured Results (Criterion v0.5)

Hardware: AMD Ryzen / Linux x86_64, local loopback interface:

| Concurrency Bound | Mean Execution Time | Relative Speedup |
| :--- | :--- | :--- |
| **Concurrency = 1** (Sequential) | **496.46 us** | 1.0x (Baseline) |
| **Concurrency = 10** | **158.73 us** | **3.13x faster** |
| **Concurrency = 50** | **162.93 us** | **3.05x faster** |


### Running Benchmarks Locally

```bash
cargo bench --bench scan_benchmark -- --sample-size 10
```

---

## Test Suite and Verification

The project includes 35 automated tests covering unit logic, integration with live TCP listeners, IPv6 dual-stack handling, and CLI execution.

### Test Breakdown

- **13 Library Unit Tests**: Default ports resolution, range boundaries, single port parsing, validation checks, service hint dictionary integrity, and IPv4/IPv6 address parsing.
- **13 Binary Unit Tests**: Identical coverage on binary target modules.
- **9 Integration Tests**:
  - `test_ipv4_open_and_closed_ports`: Spawns real local `TcpListener`, confirms `OPEN`, drops listener, confirms `CLOSED`.
  - `test_ipv6_scan`: Spawns listener on `[::1]:0`, verifies IPv6 loopback detection.
  - `test_timeout_detection`: Probes non-routable RFC 5737 TEST-NET address, asserts `TIMEOUT` state.
  - `test_concurrency_and_sorting`: Verifies that concurrent tasks always produce numerically sorted results.
  - `test_cli_json_output`: Validates CLI execution with JSON deserialization.
  - `test_cli_csv_output`: Validates CSV formatting across CLI output streams.
  - `test_cli_quiet_mode`: Validates pipeline-friendly quiet output.
  - `test_cli_no_service_hints`: Asserts suppression of service labels.
  - `test_cli_invalid_range_error`: Verifies process exit code and stderr on invalid range input.

### Executing Tests

```bash
cargo test
```

---

## Exit Codes

| Exit Code | Meaning |
| :--- | :--- |
| `0` | Successful scan execution (independent of whether ports were open, closed, or timed out) |
| `1` | Configuration or execution error (unresolvable target, invalid port format, invalid range, invalid concurrency bound) |

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
