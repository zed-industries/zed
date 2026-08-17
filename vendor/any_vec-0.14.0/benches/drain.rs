mod utils;

use std::time::{Duration, Instant};
use criterion::{criterion_group, criterion_main, Criterion};
use any_vec::AnyVec;
use crate::utils::bench_custom;

// TODO: drain in both reverse and front.

const SIZE: usize = 10000;
const DRAIN_SIZE: usize = 40;

type Element = String;
static VALUE: Element = String::new();

fn vec_drain() -> Duration {
    let mut vec = Vec::new();
    for _ in 0..SIZE{
        vec.push(VALUE.clone());
    }

    let start = Instant::now();
        while vec.len() >= DRAIN_SIZE {
            vec.drain(0..DRAIN_SIZE);
        }
    start.elapsed()
}

fn any_vec_drain() -> Duration {
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        any_vec.downcast_mut::<Element>().unwrap()
            .push(VALUE.clone());
    }

    let start = Instant::now();
        while any_vec.len() >= DRAIN_SIZE {
            any_vec.drain(0..DRAIN_SIZE);
        }
    start.elapsed()
}

fn any_vec_typed_drain() -> Duration {
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        any_vec.downcast_mut::<Element>().unwrap()
            .push(VALUE.clone());
    }

    let start = Instant::now();
        while any_vec.len() >= DRAIN_SIZE {
            let _ = any_vec.downcast_mut::<Element>().unwrap()
                .drain(0..DRAIN_SIZE);
        }
    start.elapsed()
}

pub fn bench_drain(c: &mut Criterion) {
    bench_custom(c, "Vec drain", vec_drain);
    bench_custom(c, "AnyVec drain", any_vec_drain);
    bench_custom(c, "AnyVecTyped drain", any_vec_typed_drain);
}

criterion_group!(benches, bench_drain);
criterion_main!(benches);