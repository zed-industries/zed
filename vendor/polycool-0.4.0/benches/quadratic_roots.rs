// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

pub fn quadratic_roots(c: &mut Criterion) {
    let poly = polycool::Quadratic::new([-6.0, 11.0, -6.0]);

    c.bench_function("full", |b| b.iter(|| black_box(poly).roots()));
    c.bench_function("positive discriminant", |b| {
        b.iter(|| black_box(poly).positive_discriminant_roots())
    });
}

criterion_group!(benches, quadratic_roots);
criterion_main!(benches);
