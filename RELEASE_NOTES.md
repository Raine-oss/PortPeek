# Release Notes

## v0.2.0

### Summary
Version 0.2.0 introduces structured error handling via `thiserror`, multi-format output capabilities (Terminal, JSON, CSV, Quiet), continuous watch scanning, explicit IPv6 resolution, expanded service hints, comprehensive integration tests, and formal concurrency benchmarks using Criterion.

### Key Changes
- **Structured Error Engine**: Migrated internal and CLI error handling to a typed `PortPeekError` enum, providing explicit diagnostics for invalid ranges, unresolvable targets, invalid concurrency bounds, and network I/O failures.
- **Multi-Format Output (`--output`)**:
  - `terminal`: Colorized, aligned table output with latency metrics and scan summary.
  - `json`: Machine-readable, structured JSON output containing target metadata, counts, duration, and array of port statuses.
  - `csv`: Comma-separated output (`port,protocol,status,service,latency_ms`) suitable for spreadsheet import and Unix data processing pipelines.
- **Scripting Mode (`--quiet`, `-q`)**: Suppresses headers and summaries, printing only open port identifiers (e.g., `8000/tcp`) for direct shell pipeline integration.
- **Continuous Watch Mode (`--watch`, `-w`)**: Enables continuous re-scanning at configurable intervals (`--watch-interval <SECS>`) for live service startup detection.
- **Service Hints Expansion**: Added hints for standard database and messaging services (Kafka 9092, Elasticsearch 9200, RabbitMQ 5672, Memcached 11211, Docker 2375). Added `--no-service-hints` to bypass lookups.
- **IPv6 Support**: Fully verified dual-stack addressing supporting loopback `::1`, bracketed `[::1]`, and public IPv6 destinations.
- **Integration Test Suite**: Added end-to-end integration tests spawning local TCP listeners, verifying open, closed, timeout, and CLI output format behaviors.
- **Criterion Concurrency Benchmarks**: Established standardized benchmark harness measuring scan latency across concurrency thresholds (1, 10, 50).

### Verification
- 35 automated tests passing (13 unit lib, 13 unit bin, 9 integration).
- Zero compiler warnings under `cargo clippy --all-targets -- -D warnings`.

---

## v0.1.0

### Summary
Initial proof of concept release of PortPeek:
- Basic TCP connect scanning engine with configurable timeouts.
- Concurrency limiting via asynchronous semaphore.
- Target hostname and IPv4 resolution.
- Port range and comma-separated port parsing.
- Initial colorized terminal table.
