#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    clippy::redundant_pub_crate,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value,
    clippy::too_many_lines,
    // `expect_test` sometimes adds redundant hashes, we just have to live with that
    clippy::needless_raw_string_hashes,
    non_local_definitions,
    missing_docs,
    impl_trait_overcaptures,
)]
// This catches the problem of `clippy::empty_enum` lint triggering.
// This lint is enabled only when `feature(never_type)` is enabled.
#![cfg_attr(nightly, allow(unstable_features), feature(never_type))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod prelude {
    #[cfg(feature = "alloc")]
    pub(crate) use alloc::{
        borrow::ToOwned, boxed::Box, collections::BTreeSet, format, rc::Rc, string::String,
        sync::Arc, vec, vec::Vec,
    };

    pub(crate) use super::assert_debug_eq;
    pub(crate) use bon::{bon, builder, Builder};
    pub(crate) use expect_test::expect;
}

mod builder;
mod ui;

use expect_test::Expect;

/// Approximate number of characters that can fit on a single screen
const COMMON_SCREEN_CHARS_WIDTH: usize = 60;

#[track_caller]
fn assert_debug_eq(actual: impl core::fmt::Debug, expected: Expect) {
    extern crate alloc;

    let snapshot = || {
        let terse = alloc::format!("{actual:?}");

        let width = match terse.lines().map(str::len).max() {
            Some(width) => width,
            _ => return terse,
        };

        if width < COMMON_SCREEN_CHARS_WIDTH {
            return terse;
        }

        alloc::format!("{actual:#?}")
    };

    expected.assert_eq(&snapshot());
}
