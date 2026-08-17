use criterion::{Criterion, criterion_group, criterion_main};
use saa::Semaphore;

fn acquire_release(c: &mut Criterion) {
    c.bench_function("semaphore-acquire-release", |b| {
        b.iter(|| {
            let semaphore = Semaphore::default();
            semaphore.acquire_sync();
            assert!(semaphore.release());
        });
    });
}

fn acquire_acquire_release_release(c: &mut Criterion) {
    c.bench_function("semaphore-acquire-acquire-release-release", |b| {
        b.iter(|| {
            let semaphore = Semaphore::default();
            semaphore.acquire_sync();
            semaphore.acquire_sync();
            assert!(semaphore.release());
            assert!(semaphore.release());
        });
    });
}

fn acquire_many_release_many(c: &mut Criterion) {
    c.bench_function("semaphore-acquire-many-release-many", |b| {
        b.iter(|| {
            let semaphore = Semaphore::default();
            semaphore.acquire_many_sync(11);
            assert!(semaphore.release_many(11));
        });
    });
}

criterion_group!(
    semaphore,
    acquire_release,
    acquire_acquire_release_release,
    acquire_many_release_many,
);
criterion_main!(semaphore);
