// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of cubic arclength approaches.

#![allow(missing_docs, reason = "criterion emits undocumented functions")]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use kurbo::{CubicBez, ParamCurveArclen};

fn bench_cubic_arclen(cc: &mut Criterion) {
    {
        let mut group = cc.benchmark_group("cubic_arclen");
        for accuracy in [1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9] {
            group.bench_with_input(
                BenchmarkId::from_parameter(accuracy),
                &accuracy,
                |b, &accuracy| {
                    let c =
                        CubicBez::new((0.0, 0.0), (1.0 / 3.0, 0.0), (2.0 / 3.0, 1.0), (1.0, 1.0));
                    b.iter(|| black_box(c).arclen(accuracy));
                },
            );
        }
    }

    cc.bench_function("cubic_arclen degenerate", |b| {
        let c = CubicBez::new((0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0));
        b.iter(|| black_box(c).arclen(1e-6));
    });
}

criterion_group!(benches, bench_cubic_arclen);
criterion_main!(benches);
