use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::compare_functions::fibonaccis,
    benchmarks::external_process::benches,
    benchmarks::iter_with_large_drop::benches,
    benchmarks::iter_with_large_setup::benches,
    benchmarks::iter_with_setup::benches,
    benchmarks::with_inputs::benches,
    benchmarks::special_characters::benches,
    benchmarks::measurement_overhead::benches,
    benchmarks::custom_measurement::benches,
    benchmarks::sampling_mode::benches,
    benchmarks::async_measurement_overhead::benches,
}
