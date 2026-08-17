// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of cubic arclength approaches.

#![cfg(nightly)]
#![feature(test)]
extern crate test;
use test::Bencher;

use kurbo::{CubicBez, ParamCurveArclen};

#[bench]
fn bench_cubic_arclen_1e_4(b: &mut Bencher) {
    let c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 1.0), (1.0, 1.0));
    b.iter(|| test::black_box(c).arclen(1e-4))
}

#[bench]
fn bench_cubic_arclen_1e_5(b: &mut Bencher) {
    let c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 1.0), (1.0, 1.0));
    b.iter(|| test::black_box(c).arclen(1e-5))
}

#[bench]
fn bench_cubic_arclen_1e_6(b: &mut Bencher) {
    let c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 1.0), (1.0, 1.0));
    b.iter(|| test::black_box(c).arclen(1e-6))
}

#[bench]
fn bench_cubic_arclen_degenerate(b: &mut Bencher) {
    let c = CubicBez::new((0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0));
    b.iter(|| test::black_box(c).arclen(1e-6))
}

#[bench]
fn bench_cubic_arclen_1e_7(b: &mut Bencher) {
    let c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 1.0), (1.0, 1.0));
    b.iter(|| test::black_box(c).arclen(1e-7))
}

#[bench]
fn bench_cubic_arclen_1e_8(b: &mut Bencher) {
    let c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 1.0), (1.0, 1.0));
    b.iter(|| test::black_box(c).arclen(1e-8))
}

#[bench]
fn bench_cubic_arclen_1e_9(b: &mut Bencher) {
    let c = CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 1.0), (1.0, 1.0));
    b.iter(|| test::black_box(c).arclen(1e-9))
}
