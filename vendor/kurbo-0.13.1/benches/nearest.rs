// Copyright 2022 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of the quartic equation solver.

#![allow(missing_docs, reason = "criterion emits undocumented functions")]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use kurbo::{CubicBez, ParamCurveNearest as _, Point, QuadBez};

fn bench_nearest_quadratic(cc: &mut Criterion) {
    let q = QuadBez {
        p0: (-1.0, -1.0).into(),
        p1: (0.0, 2.0).into(),
        p2: (1.0, -1.0).into(),
    };
    let p = Point::new(0.0, 0.0);

    for acc in [1e-3, 1e-6, 1e-12] {
        cc.bench_with_input(
            BenchmarkId::new("quadratic nearest point", acc),
            &acc,
            |bb, acc| {
                bb.iter(|| black_box(q).nearest(black_box(p), *acc));
            },
        );
    }
}

fn bench_nearest_cubic(cc: &mut Criterion) {
    let c = CubicBez {
        p0: (-1.0, -1.0).into(),
        p1: (0.0, 2.0).into(),
        p2: (1.0, -1.0).into(),
        p3: (2.0, 2.0).into(),
    };
    let p = Point::new(0.0, 0.0);

    for acc in [1e-3, 1e-6, 1e-12] {
        cc.bench_with_input(
            BenchmarkId::new("cubic nearest point", acc),
            &acc,
            |bb, acc| {
                bb.iter(|| black_box(c).nearest(black_box(p), *acc));
            },
        );
    }
}

criterion_group!(benches, bench_nearest_quadratic, bench_nearest_cubic);
criterion_main!(benches);
