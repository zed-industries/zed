use std::any::TypeId;
use std::mem::size_of;
use std::ptr::NonNull;
use criterion::{criterion_group, criterion_main, Criterion};
use any_vec::any_value::{AnyValueRaw};
use any_vec::AnyVec;

const SIZE: usize = 10000;

fn vec_insert_front(){
    let mut vec = Vec::new();
    for i in 0..SIZE{
        vec.insert(0, i);
    }
}

fn any_vec_insert_front(){
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    for i in 0..SIZE{
        let raw_value = unsafe{
            AnyValueRaw::new(
            NonNull::from(&i).cast::<u8>(),
            size_of::<usize>(),
            TypeId::of::<usize>()
        )};
        any_vec.insert(0, raw_value);
    }
}

fn any_vec_typed_insert_front(){
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    for i in 0..SIZE{
        let mut vec = any_vec.downcast_mut::<usize>().unwrap();
        vec.insert(0, i);
    }
}

fn vec_insert_back(){
    let mut vec = Vec::new();
    for i in 0..SIZE{
        vec.insert(i, i);
    }
}

fn any_vec_insert_back(){
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    for i in 0..SIZE{
        let raw_value = unsafe{
            AnyValueRaw::new(
            NonNull::from(&i).cast::<u8>(),
            size_of::<usize>(),
            TypeId::of::<usize>()
        )};
        any_vec.insert(i, raw_value);
    }
}

fn any_vec_typed_insert_back(){
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    for i in 0..SIZE{
        let mut vec = any_vec.downcast_mut::<usize>().unwrap();
        vec.insert(i, i);
    }
}

pub fn bench_push(c: &mut Criterion) {
    c.bench_function("Vec insert front", |b|b.iter(||vec_insert_front()));
    c.bench_function("AnyVec insert front", |b|b.iter(||any_vec_insert_front()));
    c.bench_function("AnyVecTyped insert front", |b|b.iter(||any_vec_typed_insert_front()));
    c.bench_function("Vec insert back", |b|b.iter(||vec_insert_back()));
    c.bench_function("AnyVec insert back", |b|b.iter(||any_vec_insert_back()));
    c.bench_function("AnyVecTyped insert back", |b|b.iter(||any_vec_typed_insert_back()));

}

criterion_group!(benches, bench_push);
criterion_main!(benches);