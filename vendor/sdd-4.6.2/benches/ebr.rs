use criterion::{Criterion, criterion_group, criterion_main};
use sdd::Guard;

fn guard_accelerate(c: &mut Criterion) {
    let _guard = Guard::new();
    c.bench_function("EBR: accelerate", |b| {
        b.iter(|| {
            let guard = Guard::new();
            guard.accelerate();
        })
    });
}

fn guard_single(c: &mut Criterion) {
    c.bench_function("EBR: guard", |b| {
        b.iter(|| {
            let _guard = Guard::new();
        })
    });
}

fn guard_superposed(c: &mut Criterion) {
    let _guard = Guard::new();
    c.bench_function("EBR: superposed guard", |b| {
        b.iter(|| {
            let _guard = Guard::new();
        })
    });
}

criterion_group!(ebr, guard_accelerate, guard_single, guard_superposed);
criterion_main!(ebr);
