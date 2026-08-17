// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of the cubic equation solver.

#![allow(missing_docs, reason = "criterion emits undocumented functions")]

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use kurbo::common::solve_cubic;

fn bench_cubic(cc: &mut Criterion) {
    let (x1, x2, x3) = (1.0, 2.0, 3.0);
    let a = -(x1 + x2 + x3);
    let b = x1 * x2 + x1 * x3 + x2 * x3;
    let c = -x1 * x2 * x3;

    cc.bench_function("cubic roots", |bb| {
        bb.iter(|| solve_cubic(black_box(c), black_box(b), black_box(a), 1.0));
    });
}

criterion_group!(benches, bench_cubic);
criterion_main!(benches);
