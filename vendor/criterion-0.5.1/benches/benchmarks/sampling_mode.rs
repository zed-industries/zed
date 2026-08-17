use criterion::{criterion_group, Criterion, SamplingMode};
use std::thread::sleep;
use std::time::Duration;

fn sampling_mode_tests(c: &mut Criterion) {
    let mut group = c.benchmark_group("sampling_mode");

    group.sampling_mode(SamplingMode::Auto);
    group.bench_function("Auto", |bencher| {
        bencher.iter(|| sleep(Duration::from_millis(0)))
    });

    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("Linear", |bencher| {
        bencher.iter(|| sleep(Duration::from_millis(0)))
    });

    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("Flat", |bencher| {
        bencher.iter(|| sleep(Duration::from_millis(10)))
    });

    group.finish();
}

criterion_group!(benches, sampling_mode_tests,);
