use criterion::{async_executor::FuturesExecutor, criterion_group, BatchSize, Criterion};

fn some_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("async overhead");
    group.bench_function("iter", |b| b.to_async(FuturesExecutor).iter(|| async { 1 }));
    group.bench_function("iter_with_setup", |b| {
        b.to_async(FuturesExecutor)
            .iter_with_setup(|| (), |_| async { 1 })
    });
    group.bench_function("iter_with_large_setup", |b| {
        b.to_async(FuturesExecutor)
            .iter_with_large_setup(|| (), |_| async { 1 })
    });
    group.bench_function("iter_with_large_drop", |b| {
        b.to_async(FuturesExecutor)
            .iter_with_large_drop(|| async { 1 })
    });
    group.bench_function("iter_batched_small_input", |b| {
        b.to_async(FuturesExecutor)
            .iter_batched(|| (), |_| async { 1 }, BatchSize::SmallInput)
    });
    group.bench_function("iter_batched_large_input", |b| {
        b.to_async(FuturesExecutor)
            .iter_batched(|| (), |_| async { 1 }, BatchSize::LargeInput)
    });
    group.bench_function("iter_batched_per_iteration", |b| {
        b.to_async(FuturesExecutor)
            .iter_batched(|| (), |_| async { 1 }, BatchSize::PerIteration)
    });
    group.bench_function("iter_batched_ref_small_input", |b| {
        b.to_async(FuturesExecutor)
            .iter_batched_ref(|| (), |_| async { 1 }, BatchSize::SmallInput)
    });
    group.bench_function("iter_batched_ref_large_input", |b| {
        b.to_async(FuturesExecutor)
            .iter_batched_ref(|| (), |_| async { 1 }, BatchSize::LargeInput)
    });
    group.bench_function("iter_batched_ref_per_iteration", |b| {
        b.to_async(FuturesExecutor).iter_batched_ref(
            || (),
            |_| async { 1 },
            BatchSize::PerIteration,
        )
    });
    group.finish();
}

criterion_group!(benches, some_benchmark);
