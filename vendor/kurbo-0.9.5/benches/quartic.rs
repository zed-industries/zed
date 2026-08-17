// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of the quartic equation solver.

#![cfg(nightly)]
#![feature(test)]
extern crate test;
use kurbo::common::solve_quartic;
use test::Bencher;

#[bench]
fn bench_quartic(bb: &mut Bencher) {
    let (x1, x2, x3, x4) = (1.0, 2.0, 3.0, 4.0);
    let a = -(x1 + x2 + x3 + x4);
    let b = x1 * (x2 + x3) + x2 * (x3 + x4) + x4 * (x1 + x3);
    let c = -x1 * x2 * (x3 + x4) - x3 * x4 * (x1 + x2);
    let d = x1 * x2 * x3 * x4;

    bb.iter(|| {
        solve_quartic(
            test::black_box(d),
            test::black_box(c),
            test::black_box(b),
            test::black_box(a),
            1.0,
        )
    })
}
