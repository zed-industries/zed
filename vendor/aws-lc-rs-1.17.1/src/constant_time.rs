// Copyright 2015-2022 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Constant-time operations.

use crate::aws_lc::CRYPTO_memcmp;
use crate::error;

/// Returns `Ok(())` if `a == b` and `Err(error::Unspecified)` otherwise.
///
/// The comparison of `a` and `b` is done in constant time with respect to the
/// contents of each, but NOT in constant time with respect to the lengths of
/// `a` and `b`.
///
/// # Errors
/// `error::Unspecified` when `a` and `b` differ.
#[inline]
pub fn verify_slices_are_equal(a: &[u8], b: &[u8]) -> Result<(), error::Unspecified> {
    if a.len() != b.len() {
        return Err(error::Unspecified);
    }
    let result = unsafe { CRYPTO_memcmp(a.as_ptr().cast(), b.as_ptr().cast(), a.len()) };
    match result {
        0 => Ok(()),
        _ => Err(error::Unspecified),
    }
}
