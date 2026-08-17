/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
//! Protocol-agnostic types for smithy-rs.

#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

pub mod base64;
pub mod big_number;
pub mod body;
pub mod byte_stream;
pub mod checksum_config;
/// A typemap for storing configuration.
pub mod config_bag;
pub mod date_time;
pub mod endpoint;
pub mod error;
pub mod event_stream;
pub mod primitive;
pub mod retry;
pub mod timeout;

/// Utilities for type erasure.
pub mod type_erasure;

mod blob;
mod document;
mod number;
pub mod str_bytes;

pub use big_number::{BigDecimal, BigInteger};
pub use blob::Blob;
pub use date_time::DateTime;
pub use document::Document;
pub use number::Number;
