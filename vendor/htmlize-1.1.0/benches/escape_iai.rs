//! Benchmark `escape` functions with [`iai`].

#![allow(clippy::missing_docs_in_private_items, missing_docs)]

#[allow(clippy::wildcard_imports)]
use htmlize::*;
use iai::black_box;
use pastey::paste;
use std::borrow::Cow;

mod util;

macro_rules! iai_benchmarks {
    ( $( ($name:ident, $input:expr), )+ ) => {
        paste! {
            $(
                fn [<iai_escape_text_ $name>]() -> Cow<'static, str> {
                    escape_text(black_box($input))
                }
            )+

            $(
                fn [<iai_escape_all_quotes_ $name>]() -> Cow<'static, str> {
                    escape_all_quotes(black_box($input))
                }
            )+

            $(
                fn [<iai_escape_text_bytes_ $name>]() -> Cow<'static, [u8]> {
                    escape_text_bytes(black_box($input.as_bytes()))
                }
            )+

            $(
                fn [<iai_escape_all_quotes_bytes_ $name>]() -> Cow<'static, [u8]> {
                    escape_all_quotes_bytes(black_box($input.as_bytes()))
                }
            )+

            iai::main!(
                $([<iai_escape_text_ $name>],)+
                $([<iai_escape_all_quotes_ $name>],)+
                $([<iai_escape_text_bytes_ $name>],)+
                $([<iai_escape_all_quotes_bytes_ $name>],)+
            );
        }
    }
}

iai_benchmarks! {
    (clean_small, util::inputs::CLEAN_SMALL),
    (clean_medium, util::inputs::CLEAN_MEDIUM),
    (clean_big, util::inputs::CLEAN_BIG),
    (dirty_small, util::inputs::DIRTY_SMALL),
    (dirty_medium, util::inputs::DIRTY_MEDIUM),
    (dirty_big, util::inputs::DIRTY_BIG),
}
