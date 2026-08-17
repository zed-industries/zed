use std::any::TypeId;
use std::mem::size_of;
use std::ptr::NonNull;
use criterion::{criterion_group, criterion_main, Criterion};
use any_vec::any_value::{AnyValueSizelessRaw, AnyValueRaw};
use any_vec::AnyVec;

const SIZE: usize = 10000;

type Element = (usize, usize);
#[inline]
fn make_element(i: usize) -> Element{
    (i,i)
}

fn vec_push(size: usize){
    let mut vec = Vec::new();
    for i in 0..size{
        vec.push(make_element(i));
    }
}

fn any_vec_push(size: usize){
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    let mut vec = any_vec.downcast_mut::<Element>().unwrap();
    for i in 0..size{
        vec.push(make_element(i));
    }
}

fn any_vec_push_untyped(size: usize){
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    for i in 0..size{
        let value = make_element(i);
        let raw_value = unsafe{
            AnyValueRaw::new(
            NonNull::from(&value).cast::<u8>(),
            size_of::<Element>(),
            TypeId::of::<Element>()
        )};
        any_vec.push(raw_value);
    }
}

fn any_vec_push_untyped_unchecked(size: usize){
    let mut any_vec: AnyVec = AnyVec::new::<Element>();
    for i in 0..size{
        let value = make_element(i);
        let raw_value = unsafe{
            AnyValueSizelessRaw::new(
            NonNull::from(&value).cast::<u8>(),
        )};
        unsafe{
            any_vec.push_unchecked(raw_value);
        }
    }
}

pub fn bench_push(c: &mut Criterion) {
    use criterion::black_box;

    c.bench_function("Vec push", |b|b.iter(||vec_push(black_box(SIZE))));
    c.bench_function("AnyVec push", |b|b.iter(||any_vec_push(black_box(SIZE))));
    c.bench_function("AnyVec push untyped unchecked", |b|b.iter(||any_vec_push_untyped_unchecked(black_box(SIZE))));
    c.bench_function("AnyVec push untyped", |b|b.iter(||any_vec_push_untyped(black_box(SIZE))));
}

criterion_group!(benches, bench_push);
criterion_main!(benches);