use std::cmp;
use std::ops::Range;
use itertools::assert_equal;
use rand::Rng;
use any_vec::any_value::{AnyValue, AnyValueWrapper};
use any_vec::AnyVec;

const REPEATS: usize = if cfg!(miri){ 4 }else{ 100 };
const SIZE: usize = if cfg!(miri){ 100 }else{ 10000 };

fn rnd_range(max: usize) -> Range<usize> {
    let start = rand::thread_rng().gen_range(0..max);
    let len = rand::thread_rng().gen_range(0..max);
    let end = cmp::min(max, start+len);
    Range{start, end}
}

#[test]
fn any_vec_drain_fuzzy_test() {
for _ in 0..REPEATS{
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut vec = Vec::new();

    // 1. fill
    for i in 0..SIZE {
        any_vec.push(AnyValueWrapper::new(i.to_string()));
        vec.push(i.to_string());
    }

    // 2. do drain
    let drain_range = rnd_range(vec.len());
    let drain = vec.drain(drain_range.clone());
    let any_drain = any_vec.drain(drain_range.clone());
    assert_equal(drain, any_drain.map(|e|e.downcast::<String>().unwrap()));
    assert_equal(vec.iter(), any_vec.downcast_ref::<String>().unwrap());
}
}

#[test]
fn any_vec_splice_fuzzy_test() {
for _ in 0..REPEATS{
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut vec = Vec::new();

    // 1. fill
    for i in 0..SIZE {
        any_vec.push(AnyValueWrapper::new(i.to_string()));
        vec.push(i.to_string());
    }

    // 2. prepare insertion
    let mut vec_insertion = Vec::new();
    let insertion_size = rand::thread_rng().gen_range(0..SIZE);
    for i in 0..insertion_size{
        vec_insertion.push(i.to_string());
    }

    // 3. do splice
    let drain_range = rnd_range(vec.len());
    let drain = vec.splice(
        drain_range.clone(),
        vec_insertion.iter().cloned()
    );
    let any_drain = any_vec.splice(
        drain_range.clone(),
        vec_insertion.iter().cloned().map(|e|AnyValueWrapper::new(e))
    );
    assert_equal(drain, any_drain.map(|e|e.downcast::<String>().unwrap()));
    assert_equal(vec.iter(), any_vec.downcast_ref::<String>().unwrap());
}
}

#[test]
fn any_vec_typed_splice_fuzzy_test() {
for _ in 0..REPEATS{
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    let mut any_vec = any_vec.downcast_mut::<String>().unwrap();
    let mut vec = Vec::new();

    // 1. fill
    for i in 0..SIZE {
        any_vec.push(i.to_string());
        vec.push(i.to_string());
    }

    // 2. prepare insertion
    let mut vec_insertion = Vec::new();
    let insertion_size = rand::thread_rng().gen_range(0..SIZE);
    for i in 0..insertion_size{
        vec_insertion.push(i.to_string());
    }

    // 3. do splice
    let drain_range = rnd_range(vec.len());
    let drain = vec.splice(
        drain_range.clone(),
        vec_insertion.iter().cloned()
    );
    let any_drain = any_vec.splice(
        drain_range.clone(),
        vec_insertion.iter().cloned()
    );
    assert_equal(drain, any_drain);
    assert_equal(vec.iter(), any_vec.iter());
}
}