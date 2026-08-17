extern crate core;
extern crate criterion;
extern crate seahash;

use core::hash::Hasher;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn describe_benches(c: &mut Criterion) {
    // shared buffers for all tests
    let buf = vec![15; 16 * 1024];

    // shared/n and buffer/n are executed for these sizes
    let sizes = [64, 1024, 4096, 16 * 1024];

    let mut group = c.benchmark_group("buffer");

    for size in &sizes {
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                black_box(seahash::hash(&buf[..size]));
            })
        });
    }

    group.finish();

    let mut group = c.benchmark_group("stream");

    for size in &sizes {
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_with_setup(
                || seahash::SeaHasher::default(),
                |mut h: seahash::SeaHasher| {
                    // use chunks of 32 bytes to simulate some looping on a single hasher value
                    for _ in 0..size / 32 {
                        h.write(&buf[..32]);
                    }
                    // this will mostly be an empty slice, but that is a possible Hasher api usage
                    h.write(&buf[..(size % 32)]);
                    black_box(h.finish())
                },
            )
        });
    }

    group.finish();

    // gigabyte group times are comparable with earlier benchmark values based on
    // d52d115a223a0e81d1600bd8a5e73cb4b24a38c0
    let mut group = c.benchmark_group("gigabyte");
    group.throughput(Throughput::Bytes((1024 * 1024 * 1024) as u64));

    group.bench_function(BenchmarkId::from_parameter("buffer"), |b| {
        b.iter(|| {
            let mut buf = [15; 4096];
            let mut total = 0;
            for _ in 0..250_000 {
                total ^= seahash::hash(&buf);
                buf[0] = buf[0].wrapping_add(1);
            }
            black_box(total)
        })
    });

    group.bench_function(BenchmarkId::from_parameter("stream"), |b| {
        b.iter(|| {
            let mut buf = [15; 4096];
            let mut h = seahash::SeaHasher::default();
            for _ in 0..250_000 {
                h.write(&buf);
                buf[0] = buf[0].wrapping_add(1);
            }
            black_box(h.finish())
        })
    });

    group.finish();
}

criterion_group!(benches, describe_benches);
criterion_main!(benches);
