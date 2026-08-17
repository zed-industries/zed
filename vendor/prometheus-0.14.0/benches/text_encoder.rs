// Copyright 2020 PingCAP, Inc.
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
use prometheus::{CounterVec, Encoder, HistogramOpts, HistogramVec, Opts, Registry, TextEncoder};

fn bench_text_encoder_without_escaping(c: &mut Criterion) {
    let registry = registry_with_test_metrics(false);
    run_text_encoder(c, "text_encoder_without_escaping", registry)
}

fn bench_text_encoder_with_escaping(c: &mut Criterion) {
    let registry = registry_with_test_metrics(true);
    run_text_encoder(c, "text_encoder_with_escaping", registry)
}

fn registry_with_test_metrics(with_escaping: bool) -> Registry {
    let registry = Registry::new();

    for i in 0..100 {
        let counter = CounterVec::new(
            Opts::new(
                format!("benchmark_counter_{}", i),
                "A counter to benchmark it.",
            ),
            &["one", "two", "three"],
        )
        .unwrap();
        registry.register(Box::new(counter.clone())).unwrap();

        let histogram = HistogramVec::new(
            HistogramOpts::new(
                format!("benchmark_histogram_{}", i),
                "A histogram to benchmark it.",
            ),
            &["one", "two", "three"],
        )
        .unwrap();
        registry.register(Box::new(histogram.clone())).unwrap();

        for j in 0..100 {
            let j_string = j.to_string();
            let label_values = if with_escaping {
                ["ei\\ns\n", "zw\"e\"i", &j_string]
            } else {
                ["eins", "zwei", &j_string]
            };

            counter.with_label_values(&label_values).inc();
            histogram.with_label_values(&label_values).observe(j.into());
        }
    }

    registry
}

fn run_text_encoder(c: &mut Criterion, label: &str, registry: Registry) {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    c.bench_function(label, |b| {
        b.iter(|| encoder.encode(&metric_families, &mut buffer).unwrap());
    });
}

criterion_group!(
    benches,
    bench_text_encoder_without_escaping,
    bench_text_encoder_with_escaping,
);
criterion_main!(benches);
