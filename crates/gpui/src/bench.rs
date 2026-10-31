/// Defines a Criterion benchmark group for benchmarks annotated with [`gpui::bench`].
///
/// This mirrors `criterion::criterion_group!`, but the group measures with the
/// `gpui::BenchMeasurement` configured by `BENCH_MEASUREMENT` (see
/// `BenchMeasurement::from_env`). By default Criterion analyzes wall time
/// while retired instructions, cycles, IPC, context switches, and the other
/// counters this machine supports are printed per iteration in the GPUI bench
/// report. `BENCH_MEASUREMENT=instructions` makes Criterion analyze
/// process-wide instructions instead, `foreground-instructions` the benchmark
/// thread's alone, and `wall-time` disables all counters. A
/// `config = ...` expression may set any other Criterion option; its
/// measurement is replaced. To measure with something else, call
/// `criterion::criterion_group!` directly with
/// `config = criterion::Criterion::default().with_measurement(gpui::BenchMeasurement::new(...))`.
///
/// [`gpui::bench`]: crate::bench
#[macro_export]
macro_rules! bench_group {
    (name = $name:ident; config = $config:expr; targets = $($target:path),+ $(,)?) => {
        criterion::criterion_group! {
            name = $name;
            config = ($config).with_measurement($crate::BenchMeasurement::from_env_or_exit());
            targets = $($target),+
        }
    };
    ($name:ident, $($target:path),+ $(,)?) => {
        $crate::bench_group! {
            name = $name;
            config = criterion::Criterion::default();
            targets = $($target),+
        }
    };
}

/// Defines the entry point for GPUI Criterion benchmark groups.
///
/// This mirrors `criterion::criterion_main!` so GPUI benchmark files can keep the
/// same shape as ordinary Criterion benchmarks. It also installs
/// [`CountingAllocator`] as the binary's global allocator, so reports include
/// heap allocations per iteration. It wraps the system allocator by default;
/// `allocator = Path;` wraps another unit-struct allocator instead, e.g. to
/// compare allocators, which pay the same small counting cost:
///
/// ```ignore
/// gpui::bench_main!(allocator = mimalloc::MiMalloc; benches);
/// ```
///
/// `allocator = none;` installs no global allocator, leaving Rust's default and
/// dropping the allocation metrics from reports.
#[macro_export]
macro_rules! bench_main {
    (allocator = none; $($groups:tt)*) => {
        criterion::criterion_main!($($groups)*);
    };
    (allocator = $allocator:path; $($groups:tt)*) => {
        #[global_allocator]
        static GPUI_BENCH_ALLOCATOR: $crate::CountingAllocator<$allocator> =
            $crate::CountingAllocator::new($allocator);

        criterion::criterion_main!($($groups)*);
    };
    ($($groups:tt)*) => {
        $crate::bench_main!(allocator = ::std::alloc::System; $($groups)*);
    };
}
