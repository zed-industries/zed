//! Benchmark `escape` functions with [`criterion`].

#![allow(clippy::missing_docs_in_private_items, missing_docs)]

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
#[allow(clippy::wildcard_imports)]
use htmlize::*;
use std::time::Duration;

mod util;

fn benchmarks(c: &mut Criterion) {
    let groups = [
        (
            "clean",
            [
                ("small", util::inputs::CLEAN_SMALL),
                ("medium", util::inputs::CLEAN_MEDIUM),
                ("big", util::inputs::CLEAN_BIG),
            ],
        ),
        (
            "dirty",
            [
                ("small", util::inputs::DIRTY_SMALL),
                ("medium", util::inputs::DIRTY_MEDIUM),
                ("big", util::inputs::DIRTY_BIG),
            ],
        ),
    ];

    for (group_name, inputs) in groups {
        let mut group = c.benchmark_group(group_name);
        group
            .significance_level(0.01)
            .confidence_level(0.99)
            .sample_size(500)
            .warm_up_time(Duration::from_secs(1))
            .measurement_time(Duration::from_secs(5));

        for (size_name, input) in inputs {
            util::benchmark!(group, escape_text, size_name, input);
            util::benchmark!(group, escape_all_quotes, size_name, input);
            util::benchmark!(
                group,
                escape_text_bytes,
                size_name,
                input.as_bytes()
            );
            util::benchmark!(
                group,
                escape_all_quotes_bytes,
                size_name,
                input.as_bytes()
            );
        }

        group.finish();
    }
}

criterion_group!(escape_group, benchmarks);
criterion_main!(escape_group);
