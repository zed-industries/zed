//! Contains macros which together define a benchmark harness that can be used
//! in place of the standard benchmark harness. This allows the user to run
//! Criterion.rs benchmarks with `cargo bench`.

/// Macro used to define a function group for the benchmark harness; see the
/// `criterion_main!` macro for more details.
///
/// This is used to define a function group; a collection of functions to call with a common
/// Criterion configuration. Accepts two forms which can be seen below.
///
/// Note that the group name given here is not important, it must simply
/// be unique. Note also that this macro is not related to the `Criterion::benchmark_group` function
/// or the `BenchmarkGroup` type.
///
/// # Examples:
///
/// Complete form:
///
/// ```
/// # #[macro_use]
/// # extern crate criterion;
/// # use criterion::Criterion;
/// # fn bench_method1(c: &mut Criterion) {
/// # }
/// #
/// # fn bench_method2(c: &mut Criterion) {
/// # }
/// #
/// criterion_group!{
///     name = benches;
///     config = Criterion::default();
///     targets = bench_method1, bench_method2
/// }
/// #
/// # fn main() {}
/// ```
///
/// In this form, all of the options are clearly spelled out. This expands to
/// a function named benches, which uses the given config expression to create
/// an instance of the Criterion struct. This is then passed by mutable
/// reference to the targets.
///
/// Compact Form:
///
/// ```
/// # #[macro_use]
/// # extern crate criterion;
/// # use criterion::Criterion;
/// # fn bench_method1(c: &mut Criterion) {
/// # }
/// #
/// # fn bench_method2(c: &mut Criterion) {
/// # }
/// #
/// criterion_group!(benches, bench_method1, bench_method2);
/// #
/// # fn main() {}
/// ```
/// In this form, the first parameter is the name of the group and subsequent
/// parameters are the target methods. The Criterion struct will be created using
/// the `Criterion::default()` function. If you wish to customize the
/// configuration, use the complete form and provide your own configuration
/// function.
#[macro_export]
macro_rules! criterion_group {
    (name = $name:ident; config = $config:expr; targets = $( $target:path ),+ $(,)*) => {
        pub fn $name() {
            let mut criterion: $crate::Criterion<_> = $config
                .configure_from_args();
            $(
                $target(&mut criterion);
            )+
        }
    };
    ($name:ident, $( $target:path ),+ $(,)*) => {
        $crate::criterion_group!{
            name = $name;
            config = $crate::Criterion::default();
            targets = $( $target ),+
        }
    }
}

/// Macro which expands to a benchmark harness.
///
/// Currently, using Criterion.rs requires disabling the benchmark harness
/// generated automatically by rustc. This can be done like so:
///
/// ```toml
/// [[bench]]
/// name = "my_bench"
/// harness = false
/// ```
///
/// In this case, `my_bench` must be a rust file inside the 'benches' directory,
/// like so:
///
/// `benches/my_bench.rs`
///
/// Since we've disabled the default benchmark harness, we need to add our own:
///
/// ```ignore
/// #[macro_use]
/// extern crate criterion;
/// use criterion::Criterion;
/// fn bench_method1(c: &mut Criterion) {
/// }
///
/// fn bench_method2(c: &mut Criterion) {
/// }
///
/// criterion_group!(benches, bench_method1, bench_method2);
/// criterion_main!(benches);
/// ```
///
/// The `criterion_main` macro expands to a `main` function which runs all of the
/// benchmarks in the given groups.
///
#[macro_export]
macro_rules! criterion_main {
    ( $( $group:path ),+ $(,)* ) => {
        fn main() {
            $(
                $group();
            )+

            $crate::Criterion::default()
                .configure_from_args()
                .final_summary();
        }
    }
}
