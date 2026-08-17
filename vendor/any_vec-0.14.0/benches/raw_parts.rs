use std::mem;
use criterion::{criterion_group, criterion_main, Criterion};
use any_vec::AnyVec;

const SIZE: usize = 10000;

type Element = (usize, usize, usize, usize);
static VALUE: Element = (0,0,0,0);

fn vec_push(){
    let mut vec = Vec::new();
    for _ in 0..SIZE{
        vec.push(VALUE.clone());
    }
}

fn vec_from_raw_parts_push(){
    let mut vec = Vec::new();
    for _ in 0..SIZE{
        let mut vec = unsafe{Vec::from_raw_parts(
            vec.as_mut_ptr(),
            vec.len(),
            vec.capacity()
        )};
            vec.push(VALUE.clone());
        mem::forget(vec);
    }
}

fn any_vec_push(){
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    for _ in 0..SIZE{
        let mut vec = any_vec.downcast_mut::<Element>().unwrap();
        vec.push(VALUE.clone());
    }
}

fn any_vec_from_parts_push(){
    let any_vec: AnyVec = AnyVec::new::<Element>();
    let raw_parts = any_vec.into_raw_parts();

    for _ in 0..SIZE{
        let mut any_vec: AnyVec = unsafe{AnyVec::from_raw_parts(raw_parts.clone())};
            let mut vec = any_vec.downcast_mut::<Element>().unwrap();
            vec.push(VALUE.clone());
        mem::forget(any_vec);
    }

    let any_vec: AnyVec = unsafe{AnyVec::from_raw_parts(raw_parts)};
    drop(any_vec)
}

pub fn bench_push(c: &mut Criterion) {
    c.bench_function("Vec push", |b|b.iter(||vec_push()));
    c.bench_function("Vec from_raw_parts push", |b|b.iter(||vec_from_raw_parts_push()));
    c.bench_function("AnyVec push", |b|b.iter(||any_vec_push()));
    c.bench_function("AnyVec from_raw_parts push", |b|b.iter(||any_vec_from_parts_push()));
}

criterion_group!(benches, bench_push);
criterion_main!(benches);