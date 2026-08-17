// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

pub fn cubic_roots(c: &mut Criterion) {
    let poly = polycool::Cubic::new([-6.0, 11.0, -6.0, 1.0]);

    let mut group = c.benchmark_group("simple roots");

    for accuracy in [1e-6, 1e-8, 1e-12] {
        group.bench_with_input(
            format!("roots_between {accuracy:?}"),
            &accuracy,
            |b, accuracy| b.iter(|| black_box(poly).roots_between(-1.0, 4.0, *accuracy)),
        );
    }
    for accuracy in [1e-6, 1e-8, 1e-12] {
        group.bench_with_input(
            format!("roots_between with one root {accuracy:?}"),
            &accuracy,
            |b, accuracy| b.iter(|| black_box(poly).roots_between(-1.0, 1.5, *accuracy)),
        );
    }
}

criterion_group!(benches, cubic_roots);
criterion_main!(benches);
