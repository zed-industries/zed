//! Benchmark targets for Zed crates.
//!
//! Benchmarks live in their own crate so benchmark-only dependencies
//! (Criterion, `gpui_platform`, gpui's `bench` feature, ...) don't weigh down
//! the test builds of the crates being benchmarked. Each file in `benches/`
//! targets one area of the codebase.
