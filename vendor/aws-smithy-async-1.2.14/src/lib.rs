/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]

//! Future utilities and runtime-agnostic abstractions for smithy-rs.
//!
//! Async runtime specific code is abstracted behind async traits, and implementations are
//! provided via feature flag. For now, only Tokio runtime implementations are provided.

pub mod future;
pub mod rt;
#[cfg(feature = "test-util")]
pub mod test_util;
pub mod time;

/// Given an `Instant` and a `Duration`, assert time elapsed since `Instant` is equal to `Duration`.
/// This macro allows for a 5ms margin of error.
///
/// # Example
///
/// ```rust,ignore
/// let now = std::time::Instant::now();
/// let _ = some_function_that_always_takes_five_seconds_to_run().await;
/// assert_elapsed!(now, std::time::Duration::from_secs(5));
/// ```
#[macro_export]
macro_rules! assert_elapsed {
    ($start:expr, $dur:expr) => {
        assert_elapsed!($start, $dur, std::time::Duration::from_millis(5));
    };
    ($start:expr, $dur:expr, $margin_of_error:expr) => {{
        let elapsed = $start.elapsed();
        // type ascription improves compiler error when wrong type is passed
        let margin_of_error: std::time::Duration = $margin_of_error;
        let lower: std::time::Duration = $dur - margin_of_error;
        let upper: std::time::Duration = $dur + margin_of_error;

        // Handles ms rounding
        assert!(
            elapsed >= lower && elapsed <= upper,
            "actual = {:?}, expected = {:?}",
            elapsed,
            lower
        );
    }};
}
