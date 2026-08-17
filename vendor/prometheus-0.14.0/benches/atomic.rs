// Copyright 2018 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::{criterion_group, criterion_main, Criterion};
use prometheus::core::*;

fn bench_atomic_f64(c: &mut Criterion) {
    let val = AtomicF64::new(0.0);
    c.bench_function("atomic_f64", |b| {
        b.iter(|| {
            val.inc_by(12.0);
        })
    });
}

fn bench_atomic_i64(c: &mut Criterion) {
    let val = AtomicI64::new(0);
    c.bench_function("atomic_i64", |b| {
        b.iter(|| {
            val.inc_by(12);
        })
    });
}

criterion_group!(benches, bench_atomic_f64, bench_atomic_i64);
criterion_main!(benches);
