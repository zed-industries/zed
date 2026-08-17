// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use criterion::{criterion_group, criterion_main, Criterion};
use pest::Stack;

fn snapshot_push_restore<T: Clone>(elements: impl Iterator<Item = T> + Clone) {
    let mut stack = Stack::<T>::new();
    for elem in elements {
        stack.snapshot();
        stack.push(elem.clone());
        stack.restore();
        stack.push(elem);
    }
}

fn snapshot_push_clear_snapshot<T: Clone>(elements: impl Iterator<Item = T> + Clone) {
    let mut stack = Stack::<T>::new();
    for elem in elements {
        stack.snapshot();
        stack.push(elem);
        stack.clear_snapshot();
    }
}

fn snapshot_pop_restore<T: Clone>(elements: impl Iterator<Item = T>) {
    let mut stack = Stack::<T>::new();
    for elem in elements {
        stack.push(elem);
    }
    while !stack.is_empty() {
        stack.snapshot();
        stack.pop();
        stack.restore();
        stack.pop();
    }
}

fn snapshot_pop_clear<T: Clone>(elements: impl Iterator<Item = T>) {
    let mut stack = Stack::<T>::new();
    for elem in elements {
        stack.push(elem);
    }
    while !stack.is_empty() {
        stack.snapshot();
        stack.pop();
        stack.clear_snapshot();
    }
}

fn benchmark(b: &mut Criterion) {
    use core::iter::repeat;
    // use criterion::black_box;
    let times = 10000usize;
    let small = 0..times;
    let medium = ("", 0usize, 1usize);
    let medium = repeat(medium).take(times);
    let large = [""; 64];
    let large = repeat(large).take(times);
    macro_rules! test_series {
        ($kind:ident) => {
            b.bench_function(stringify!(push - restore - $kind), |b| {
                b.iter(|| snapshot_push_restore($kind.clone()))
            })
            .bench_function(stringify!(push - clear - $kind), |b| {
                b.iter(|| snapshot_push_clear_snapshot($kind.clone()))
            })
            .bench_function(stringify!(pop - restore - $kind), |b| {
                b.iter(|| snapshot_pop_restore($kind.clone()))
            })
            .bench_function(stringify!(pop - clear - $kind), |b| {
                b.iter(|| snapshot_pop_clear($kind.clone()))
            })
        };
    }
    test_series!(small);
    test_series!(medium);
    test_series!(large);
}

criterion_group!(benchmarks, benchmark);
criterion_main!(benchmarks);
