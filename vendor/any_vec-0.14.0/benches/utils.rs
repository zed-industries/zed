use std::time::Duration;
use criterion::Criterion;

pub fn bench_custom<F: FnMut() -> Duration>(c: &mut Criterion, id: &str, mut routine: F) {
    c.bench_function(id, move |b|b.iter_custom(|iters|
        (0..iters).map(|_|routine()).sum()
    ));
}