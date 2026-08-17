use criterion::{criterion_group, BatchSize, Criterion, Throughput};
use std::time::Duration;

const SIZE: usize = 1024 * 1024;

fn large_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_with_large_setup");
    group.throughput(Throughput::Bytes(SIZE as u64));
    group.bench_function("large_setup", |b| {
        b.iter_batched(
            || (0..SIZE).map(|i| i as u8).collect::<Vec<_>>(),
            |v| v,
            BatchSize::NumBatches(1),
        )
    });
}

fn small_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_with_large_setup");
    group.bench_function("small_setup", |b| {
        b.iter_batched(|| SIZE, |size| size, BatchSize::NumBatches(1))
    });
}

fn short_warmup() -> Criterion {
    Criterion::default().warm_up_time(Duration::new(1, 0))
}

criterion_group! {
    name = benches;
    config = short_warmup();
    targets = large_setup, small_setup
}
