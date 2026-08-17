//! Benchmark `unescape` functions with [`criterion`].

#![allow(clippy::missing_docs_in_private_items, missing_docs)]

use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
    Throughput,
};
#[allow(clippy::wildcard_imports)]
use htmlize::unescape::internal::*;
use std::time::Duration;

#[macro_use]
mod util;

fn init_group<'a>(
    c: &'a mut Criterion,
    name: &'static str,
) -> BenchmarkGroup<'a, WallTime> {
    let mut group = c.benchmark_group(name);
    group
        .noise_threshold(0.10)
        .significance_level(0.01)
        .confidence_level(0.99)
        .sample_size(300)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(10));
    group
}

#[allow(clippy::significant_drop_tightening, reason = "buggy lint")]
fn benchmarks(c: &mut Criterion) {
    let test_inputs = [
        ("normal", "&lt;"),
        ("bare", "&lta"),
        ("none", "_lta"),
        ("invalid", "&xxa"),
    ];

    let mut group = init_group(c, "unescape");
    for (name, entity) in test_inputs {
        let input = util::inputs::make_sample(128, entity, "a");

        #[cfg(feature = "unescape")]
        util::benchmark_name!(
            group,
            "map",
            (Phf, ContextGeneral),
            &name,
            &input
        );

        #[cfg(feature = "unescape_fast")]
        util::benchmark_name!(
            group,
            "matchgen",
            (Matchgen, ContextGeneral),
            &name,
            &input
        );
    }
    group.finish();

    let mut group = init_group(c, "unescape_attribute");
    for (name, entity) in test_inputs {
        let input = util::inputs::make_sample(128, entity, "a");

        #[cfg(feature = "unescape")]
        util::benchmark_name!(
            group,
            "map",
            (Phf, ContextAttribute),
            &name,
            &input
        );

        #[cfg(feature = "unescape_fast")]
        util::benchmark_name!(
            group,
            "matchgen",
            (Matchgen, ContextAttribute),
            &name,
            &input
        );
    }
    group.finish();
}

criterion_group!(unescape_group, benchmarks);
criterion_main!(unescape_group);
