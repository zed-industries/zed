// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of the quartic equation solver.

#![allow(missing_docs, reason = "criterion emits undocumented functions")]

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use kurbo::common::solve_quartic;

fn bench_quartic(cc: &mut Criterion) {
    let (x1, x2, x3, x4) = (1.0, 2.0, 3.0, 4.0);
    let a = -(x1 + x2 + x3 + x4);
    let b = x1 * (x2 + x3) + x2 * (x3 + x4) + x4 * (x1 + x3);
    let c = -x1 * x2 * (x3 + x4) - x3 * x4 * (x1 + x2);
    let d = x1 * x2 * x3 * x4;

    cc.bench_function("quartic roots", |bb| {
        bb.iter(|| solve_quartic(black_box(d), black_box(c), black_box(b), black_box(a), 1.0));
    });
}

criterion_group!(benches, bench_quartic);
criterion_main!(benches);
