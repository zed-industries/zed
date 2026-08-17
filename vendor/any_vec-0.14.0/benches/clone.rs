mod utils;

use std::time::{Duration, Instant};
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use any_vec::AnyVec;
use any_vec::traits::Cloneable;
use crate::utils::bench_custom;

const SIZE: usize = 10000;

// usize is worst case scenario, since it is copyable.
type Element = usize;
static VALUE: Element = 100;

fn vec_clone() -> Duration {
    let mut vec = Vec::new();
    for _ in 0..SIZE{
        vec.push(VALUE.clone());
    }

    let start = Instant::now();
        let other = vec.clone();
        black_box(other);
    start.elapsed()
}


fn any_vec_clone() -> Duration {
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        any_vec.downcast_mut::<Element>().unwrap()
            .push(VALUE.clone());
    }

    let start = Instant::now();
        let other = any_vec.clone();
        black_box(other);
    start.elapsed()
}


pub fn bench_clone(c: &mut Criterion) {
    bench_custom(c, "Vec clone", vec_clone);
    bench_custom(c, "AnyVec clone", any_vec_clone);
}

criterion_group!(benches, bench_clone);
criterion_main!(benches);