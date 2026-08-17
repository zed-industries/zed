// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use criterion::{Criterion, criterion_group, criterion_main};
use polycool::Poly;
use std::hint::black_box;

pub fn eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval");

    let coeffs_2 = [1.0, 2.0, 3.0];
    let coeffs_3 = [1.0, 2.0, 3.0, 4.0];
    let coeffs_4 = [1.0, 2.0, 3.0, 4.0, 5.0];

    group.bench_with_input("eval 2", &2, |b, _| {
        let p2 = Poly::new(coeffs_2);
        b.iter(|| black_box(p2).eval(black_box(1.0)))
    });
    group.bench_with_input("eval 3", &2, |b, _| {
        let p3 = Poly::new(coeffs_3);
        b.iter(|| black_box(p3).eval(black_box(1.0)))
    });
    group.bench_with_input("eval 3 alt", &2, |b, _| {
        let p3 = Poly::new(coeffs_3);
        b.iter(|| black_box(p3).eval_opt(black_box(1.0)))
    });
    group.bench_with_input("eval 2 and 3", &2, |b, _| {
        let p3 = Poly::new(coeffs_3);
        let p2 = Poly::new(coeffs_2);
        b.iter(|| {
            (
                black_box(p3).eval_opt(black_box(1.0)),
                p2.eval(black_box(1.0)),
            )
        })
    });
    group.bench_with_input("eval 2 and 3 alt", &2, |b, _| {
        let p3 = Poly::new(coeffs_3);
        let p2 = Poly::new(coeffs_2);
        b.iter(|| black_box(p3).eval_with_deriv_opt(&p2, black_box(1.0)))
    });
    group.bench_with_input("eval 4", &2, |b, _| {
        let p4 = Poly::new(coeffs_4);
        b.iter(|| black_box(p4).eval(black_box(1.0)))
    });
}

criterion_group!(benches, eval);
criterion_main!(benches);
