mod utils;

use std::time::{Duration, Instant};
use criterion::{criterion_group, criterion_main, Criterion};
use any_vec::AnyVec;
use crate::utils::bench_custom;

const SIZE: usize = 10000;

type Element = String;
static VALUE: Element = String::new();

fn vec_swap_remove() -> Duration {
    let mut vec = Vec::new();
    for _ in 0..SIZE{
        vec.push(VALUE.clone());
    }

    let start = Instant::now();
        for _ in 0..SIZE{
            vec.swap_remove(0);
        }
    start.elapsed()
}

fn any_vec_swap_remove() -> Duration {
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        any_vec.downcast_mut::<Element>().unwrap()
            .push(VALUE.clone());
    }

    let start = Instant::now();
        for _ in 0..SIZE{
            any_vec.swap_remove(0);
        }
    start.elapsed()
}

fn any_vec_typed_swap_remove() -> Duration {
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        any_vec.downcast_mut::<Element>().unwrap()
            .push(VALUE.clone());
    }

    let start = Instant::now();
        for _ in 0..SIZE{
            any_vec.downcast_mut::<Element>().unwrap()
                .swap_remove(0);
        }
    start.elapsed()
}

pub fn bench_swap_remove(c: &mut Criterion) {
    bench_custom(c, "Vec swap_remove", vec_swap_remove);
    bench_custom(c, "AnyVec swap_remove", any_vec_swap_remove);
    bench_custom(c, "AnyVecTyped swap_remove", any_vec_typed_swap_remove);
}

criterion_group!(benches, bench_swap_remove);
criterion_main!(benches);