// Copyright 2016 PingCAP, Inc.
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
use prometheus::{Gauge, GaugeVec, IntGauge, Opts};

fn bench_gauge_with_label_values(c: &mut Criterion) {
    let gauge = GaugeVec::new(
        Opts::new("benchmark_gauge", "A gauge to benchmark it."),
        &["one", "two", "three"],
    )
    .unwrap();
    c.bench_function("gauge_with_label_values", |b| {
        b.iter(|| gauge.with_label_values(&["eins", "zwei", "drei"]).inc())
    });
}

fn bench_gauge_no_labels(c: &mut Criterion) {
    let gauge = Gauge::new("benchmark_gauge", "A gauge to benchmark.").unwrap();
    c.bench_function("gauge_no_labels", |b| b.iter(|| gauge.inc()));
}

fn bench_int_gauge_no_labels(c: &mut Criterion) {
    let gauge = IntGauge::new("benchmark_int_gauge", "A int_gauge to benchmark.").unwrap();
    c.bench_function("int_gauge_no_labels", |b| b.iter(|| gauge.inc()));
}

criterion_group!(
    benches,
    bench_gauge_with_label_values,
    bench_gauge_no_labels,
    bench_int_gauge_no_labels,
);
criterion_main!(benches);
