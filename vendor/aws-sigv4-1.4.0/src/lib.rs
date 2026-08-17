/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
//! Provides functions for calculating Sigv4 signing keys, signatures, and
//! optional utilities for signing HTTP requests and Event Stream messages.

#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

use std::fmt;

pub mod sign;

mod date_time;

#[cfg(feature = "sign-eventstream")]
pub mod event_stream;

#[cfg(feature = "sign-http")]
pub mod http_request;

/// The version of the signing algorithm to use
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum SignatureVersion {
    /// The SigV4 signing algorithm.
    V4,
    /// The SigV4a signing algorithm.
    V4a,
}

impl fmt::Display for SignatureVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureVersion::V4 => write!(f, "SigV4"),
            SignatureVersion::V4a => write!(f, "SigV4a"),
        }
    }
}

/// Container for the signed output and the signature.
///
/// This is returned by signing functions, and the signed output will be
/// different based on what is being signed (for example, an event stream
/// message, or an HTTP request).
#[derive(Debug)]
pub struct SigningOutput<T> {
    output: T,
    signature: String,
}

impl<T> SigningOutput<T> {
    /// Creates a new [`SigningOutput`]
    pub fn new(output: T, signature: String) -> Self {
        Self { output, signature }
    }

    /// Returns the signed output
    pub fn output(&self) -> &T {
        &self.output
    }

    /// Returns the signature as a lowercase hex string
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Decomposes the `SigningOutput` into a tuple of the signed output and the signature
    pub fn into_parts(self) -> (T, String) {
        (self.output, self.signature)
    }
}
