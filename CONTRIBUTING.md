# Contributing to PortPeek

Thank you for your interest in contributing to PortPeek. This document outlines the process for proposing changes, reporting bugs, and submitting pull requests.

---

## Project Philosophy and Scope

PortPeek is designed strictly as a lightweight, fast terminal TCP connectivity troubleshooting utility. To preserve its identity and simplicity, the following features are explicitly non-goals:
- UDP scanning
- SYN/stealth scanning
- OS fingerprinting
- Vulnerability assessment
- Banner grabbing
- Subnet/CIDR mass scanning

Pull requests introducing features outside the connectivity troubleshooting scope will be closed.

---

## Development Setup

### Prerequisites
- Rust 1.80.0 or newer (Edition 2024 compatible)
- Cargo package manager
- Git

### Building the Project
```bash
git clone https://github.com/Raine-oss/PortPeek.git
cd PortPeek
cargo build
```

### Running Tests
All changes must pass unit and integration tests:
```bash
cargo test
```

### Running Benchmarks
```bash
cargo bench --bench scan_benchmark -- --sample-size 10
```

### Code Formatting and Lints
The codebase enforces standard formatting and zero Clippy warnings:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

---

## Contribution Workflow

1. Fork the repository on GitHub.
2. Create a feature branch off `main` or `master` (`git checkout -b feature/your-feature`).
3. Implement your changes adhering to existing code conventions.
4. Ensure all unit and integration tests pass.
5. Format the code with `cargo fmt`.
6. Commit your changes using descriptive commit messages.
7. Push the branch to your fork and open a Pull Request.

---

## Pull Request Guidelines

- Keep PRs focused on a single change, bug fix, or feature.
- Include corresponding unit or integration tests for any logic modifications.
- Update documentation in `README.md` if command-line options or behaviors change.
