/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    // missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]

//! AWS Event Stream frame serialization/deserialization implementation.

#[cfg(feature = "derive-arbitrary")]
pub mod arbitrary;
mod buf;
pub mod error;
pub mod frame;
pub mod message_size_hint;
pub mod smithy;
#[cfg(feature = "test-util")]
pub mod test_util;
