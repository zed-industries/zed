#![allow(unused)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use terminal::terminal_hyperlinks::find_from_grid_point;

#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn hyperlink_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, hyperlink_benchmark);
criterion_main!(benches);
