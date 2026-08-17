use criterion::{criterion_group, BatchSize, Criterion};

fn some_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("overhead");
    group.bench_function("iter", |b| b.iter(|| 1));
    group.bench_function("iter_with_setup", |b| b.iter_with_setup(|| (), |_| 1));
    group.bench_function("iter_with_large_setup", |b| {
        b.iter_batched(|| (), |_| 1, BatchSize::NumBatches(1))
    });
    group.bench_function("iter_with_large_drop", |b| b.iter_with_large_drop(|| 1));
    group.bench_function("iter_batched_small_input", |b| {
        b.iter_batched(|| (), |_| 1, BatchSize::SmallInput)
    });
    group.bench_function("iter_batched_large_input", |b| {
        b.iter_batched(|| (), |_| 1, BatchSize::LargeInput)
    });
    group.bench_function("iter_batched_per_iteration", |b| {
        b.iter_batched(|| (), |_| 1, BatchSize::PerIteration)
    });
    group.bench_function("iter_batched_ref_small_input", |b| {
        b.iter_batched_ref(|| (), |_| 1, BatchSize::SmallInput)
    });
    group.bench_function("iter_batched_ref_large_input", |b| {
        b.iter_batched_ref(|| (), |_| 1, BatchSize::LargeInput)
    });
    group.bench_function("iter_batched_ref_per_iteration", |b| {
        b.iter_batched_ref(|| (), |_| 1, BatchSize::PerIteration)
    });
    group.finish();
}

criterion_group!(benches, some_benchmark);
