use criterion::{criterion_group, Criterion};

const SIZE: usize = 1024 * 1024;

fn setup(c: &mut Criterion) {
    c.bench_function("iter_with_setup", |b| {
        b.iter_with_setup(|| (0..SIZE).map(|i| i as u8).collect::<Vec<_>>(), |v| v)
    });
}

criterion_group!(benches, setup);
