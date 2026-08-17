pub mod compare_functions;
pub mod custom_measurement;
pub mod external_process;
pub mod iter_with_large_drop;
pub mod iter_with_large_setup;
pub mod iter_with_setup;
pub mod measurement_overhead;
pub mod sampling_mode;
pub mod special_characters;
pub mod with_inputs;

#[cfg(feature = "async_futures")]
pub mod async_measurement_overhead;

#[cfg(not(feature = "async_futures"))]
pub mod async_measurement_overhead {
    use criterion::{criterion_group, Criterion};
    fn some_benchmark(_c: &mut Criterion) {}

    criterion_group!(benches, some_benchmark);
}
