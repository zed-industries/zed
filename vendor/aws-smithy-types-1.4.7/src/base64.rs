/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! A thin wrapper over [`base64-simd`](https://docs.rs/base64-simd/)

use base64_simd::STANDARD;
use std::error::Error;

/// Failure to decode a base64 value.
#[derive(Debug)]
pub struct DecodeError(base64_simd::Error);

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to decode base64")
    }
}

/// Decode `input` from base64 using the standard base64 alphabet
///
/// If input is not a valid base64 encoded string, this function will return `DecodeError`.
pub fn decode(input: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
    STANDARD.decode_to_vec(input.as_ref()).map_err(DecodeError)
}

/// Encode `input` into base64 using the standard base64 alphabet
pub fn encode(input: impl AsRef<[u8]>) -> String {
    STANDARD.encode_to_string(input.as_ref())
}

/// Returns the base64 representation's length for the given `length` of data
pub fn encoded_length(length: usize) -> usize {
    STANDARD.encoded_length(length)
}
