mod utils;

use std::time::{Duration, Instant};
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use any_vec::any_value::{AnyValueCloneable, AnyValueSizeless};
use any_vec::AnyVec;
use any_vec::traits::Cloneable;
use crate::utils::bench_custom;

const SIZE: usize = 10000;

// usize is worst case scenario, since it is copyable.
type Element = usize;
static VALUE: Element = 100;

fn vec_iter_clone() -> Duration {
    let mut vec = Vec::new();
    for _ in 0..SIZE{
        vec.push(VALUE.clone());
    }

    let start = Instant::now();
        let mut sum = 0;
        for i in &vec{
            sum += i.clone();
        }
        black_box(sum);
    start.elapsed()
}

fn any_vec_iter() -> Duration {
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        any_vec.downcast_mut::<Element>().unwrap()
            .push(VALUE.clone());
    }

    let start = Instant::now();
        let mut sum = 0;
        for i in &any_vec{
            sum += unsafe{i.downcast_ref_unchecked::<usize>()};
        }
        black_box(sum);
    start.elapsed()
}

fn any_vec_iter_clone() -> Duration {
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        any_vec.downcast_mut::<Element>().unwrap()
            .push(VALUE.clone());
    }

    let start = Instant::now();
        let mut sum = 0;
        for i in &any_vec{
            sum += unsafe{i.lazy_clone().downcast_ref_unchecked::<usize>()};
        }
        black_box(sum);
    start.elapsed()
}

fn any_vec_typed_iter_clone() -> Duration {
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<Element>();
    let mut vec = any_vec.downcast_mut::<Element>().unwrap();
    for _ in 0..SIZE{
        vec.push(VALUE.clone());
    }

    let start = Instant::now();
        let mut sum = 0;
        for i in vec{
            sum += i.clone();
        }
        black_box(sum);
    start.elapsed()
}


pub fn bench_element_clone(c: &mut Criterion) {
    bench_custom(c, "Vec iter clone", vec_iter_clone);
    bench_custom(c, "AnyVec iter", any_vec_iter);
    bench_custom(c, "AnyVec iter clone", any_vec_iter_clone);
    bench_custom(c, "AnyVecTyped iter clone", any_vec_typed_iter_clone);
}

criterion_group!(benches, bench_element_clone);
criterion_main!(benches);