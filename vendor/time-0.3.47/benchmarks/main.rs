//! Benchmarks for `time`.
//!
//! These benchmarks are not very precise, but they're good enough to catch major performance
//! regressions. Run them if you think that may be the case. CI **does not** run benchmarks.

#![allow(
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::alloc_instead_of_core,
    reason = "irrelevant for benchmarks"
)]
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "may be removed in the future"
)]

#[cfg(not(all(
    feature = "default",
    feature = "alloc",
    feature = "formatting",
    feature = "large-dates",
    feature = "local-offset",
    feature = "macros",
    feature = "parsing",
    feature = "quickcheck",
    feature = "serde-human-readable",
    feature = "serde-well-known",
    feature = "std",
    feature = "rand",
    feature = "serde",
    bench,
)))]
compile_error!(
    "benchmarks must be run as `RUSTFLAGS=\"--cfg bench\" cargo criterion --all-features`"
);

macro_rules! setup_benchmark {
    (
        $group_prefix:literal,
        $(
            $(#[$fn_attr:meta])*
            fn $fn_name:ident ($bencher:ident : $bencher_type:ty)
            $code:block
        )*
    ) => {
        $(
            $(#[$fn_attr])*
            fn $fn_name(
                c: &mut ::criterion::Criterion
            ) {
                c.bench_function(
                    concat!($group_prefix, ": ", stringify!($fn_name)),
                    |$bencher: $bencher_type| $code
                );
            }
        )*

        ::criterion::criterion_group! {
            name = benches;
            config = ::criterion::Criterion::default()
                // Set a stricter statistical significance threshold ("p-value")
                // for deciding what's an actual performance change vs. noise.
                // The more benchmarks, the lower this needs to be in order to
                // not get lots of false positives.
                .significance_level(0.0001)
                // Ignore any performance change less than this (0.05 = 5%) as
                // noise, regardless of statistical significance.
                .noise_threshold(0.05)
                // Reduce the time taken to run each benchmark
                .warm_up_time(::std::time::Duration::from_millis(100))
                .measurement_time(::std::time::Duration::from_millis(500));
            targets = $($fn_name,)*
        }
    };
}

macro_rules! iter_batched_ref {
    ($ben:ident, $initializer:expr,[$($routine:expr),+ $(,)?]) => {$(
        $ben.iter_batched_ref(
            $initializer,
            $routine,
            ::criterion::BatchSize::SmallInput,
        );
    )+};
}

macro_rules! mods {
    ($(mod $mod:ident;)+) => {
        $(mod $mod;)+
        ::criterion::criterion_main!($($mod::benches),+);
    }
}

mods![
    mod date;
    mod duration;
    mod formatting;
    mod instant;
    mod month;
    mod offset_date_time;
    mod parsing;
    mod primitive_date_time;
    mod rand08;
    mod rand09;
    mod time;
    mod utc_offset;
    mod util;
    mod weekday;
];

/// Shuffle a slice in a random but deterministic manner.
fn shuffle<T, const N: usize>(mut slice: [T; N]) -> [T; N] {
    use ::rand09::prelude::*;

    let mut seed = SmallRng::seed_from_u64(0);
    slice.shuffle(&mut seed);
    slice
}
