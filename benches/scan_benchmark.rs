// Benchmark Imports

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::net::IpAddr;
use tokio::runtime::Runtime;

use portpeek::scanner::scan_ports;

// Benchmark Scenarios

fn benchmark_concurrency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let target_ip: IpAddr = "127.0.0.1".parse().unwrap();
    let ports: Vec<u16> = (40000..40050).collect();

    let mut group = c.benchmark_group("concurrency_comparison");

    group.bench_function("concurrency_1", |b| {
        b.to_async(&rt).iter(|| async {
            scan_ports(
                black_box(target_ip),
                black_box(ports.clone()),
                black_box(1),
                black_box(100),
                black_box(false),
            )
            .await
        });
    });

    group.bench_function("concurrency_10", |b| {
        b.to_async(&rt).iter(|| async {
            scan_ports(
                black_box(target_ip),
                black_box(ports.clone()),
                black_box(10),
                black_box(100),
                black_box(false),
            )
            .await
        });
    });

    group.bench_function("concurrency_50", |b| {
        b.to_async(&rt).iter(|| async {
            scan_ports(
                black_box(target_ip),
                black_box(ports.clone()),
                black_box(50),
                black_box(100),
                black_box(false),
            )
            .await
        });
    });

    group.finish();
}

// Benchmark Groups

criterion_group!(benches, benchmark_concurrency);
criterion_main!(benches);
