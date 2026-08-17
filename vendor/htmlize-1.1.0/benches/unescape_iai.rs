//! Benchmark `unescape` functions with [`iai`].

#![allow(clippy::missing_docs_in_private_items, missing_docs)]

#[allow(clippy::wildcard_imports)]
use htmlize::unescape::internal::*;
use iai::black_box;
use pastey::paste;
use std::borrow::Cow;

mod util;

macro_rules! iai_benchmarks {
    ( $( ($name:ident, $input:expr), )+ ) => {
        paste! {
            $(
                #[cfg(feature = "unescape")]
                fn [<iai_map_unescape_ $name>]() -> Cow<'static, str> {
                    unescape_in((Phf, ContextGeneral), black_box($input))
                }

                #[cfg(feature = "unescape")]
                fn [<iai_map_unescape_attribute_ $name>]() -> Cow<'static, str> {
                    unescape_in((Phf, ContextAttribute), black_box($input))
                }

                #[cfg(feature = "unescape_fast")]
                fn [<iai_matchgen_unescape_ $name>]() -> Cow<'static, str> {
                    unescape_in((Matchgen, ContextGeneral), black_box($input))
                }

                #[cfg(feature = "unescape_fast")]
                fn [<iai_matchgen_unescape_attribute_ $name>]() -> Cow<'static, str> {
                    unescape_in((Matchgen, ContextAttribute), black_box($input))
                }
            )+

            #[cfg(all(feature = "unescape", not(feature = "unescape_fast")))]
            iai::main!(
                $(
                    [<iai_map_unescape_ $name>],
                    [<iai_map_unescape_attribute_ $name>],
                )+
            );

            #[cfg(all(feature = "unescape", feature = "unescape_fast"))]
            iai::main!(
                $(
                    [<iai_map_unescape_ $name>],
                    [<iai_map_unescape_attribute_ $name>],
                    [<iai_matchgen_unescape_ $name>],
                    [<iai_matchgen_unescape_attribute_ $name>],
                )+
            );

            #[cfg(all(not(feature = "unescape"), feature = "unescape_fast"))]
            iai::main!(
                $(
                    [<iai_matchgen_unescape_ $name>],
                    [<iai_matchgen_unescape_attribute_ $name>],
                )+
            );
        }
    }
}

// FIXME: we’re benchmarking making the sample too.
iai_benchmarks! {
    (sample_128, util::inputs::make_sample(128, "&lt;", "a")),
    (sample_128_bare, util::inputs::make_sample(128, "&lta", "a")),
    (sample_128_none, util::inputs::make_sample(128, "_lta", "a")),
    (sample_128_invalid, util::inputs::make_sample(128, "&xxa", "a")),
}
