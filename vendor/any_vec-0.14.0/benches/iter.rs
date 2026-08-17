mod utils;

use std::time::{Duration, Instant};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use any_vec::AnyVec;
use crate::utils::bench_custom;

const SIZE: usize = 10000;

fn vec_iter() -> Duration {
    let mut vec = Vec::new();
    for i in 0..SIZE{
        vec.push(i);
    }

    let start = Instant::now();
        let sum: usize = vec.iter().sum();
        black_box(sum);
    start.elapsed()
}

fn any_vec_typed_iter() -> Duration {
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    let mut any_vec_typed = any_vec.downcast_mut::<usize>().unwrap();
    for i in 0..SIZE{
        any_vec_typed.push(i);
    }

    let start = Instant::now();
        let sum: usize = any_vec_typed.iter().sum();
        black_box(sum);
    start.elapsed()
}

fn any_vec_iter() -> Duration {
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    for i in 0..SIZE{
        any_vec.downcast_mut::<usize>().unwrap()
            .push(i);
    }

    let start = Instant::now();
        let sum: usize = any_vec.iter()
            //.map(|e|e.downcast_ref().unwrap())
            .map(|e|unsafe{e.downcast_ref_unchecked()})
            .sum();
        black_box(sum);
    start.elapsed()
}

pub fn bench_iter(c: &mut Criterion) {
    bench_custom(c, "Vec iter", vec_iter);
    bench_custom(c, "AnyVecTyped iter", any_vec_typed_iter);
    bench_custom(c, "AnyVec iter", any_vec_iter);
}

criterion_group!(benches, bench_iter);
criterion_main!(benches);