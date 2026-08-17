// Copyright 2018 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Serialization and deserialization.

/// A serialized positive integer.
#[derive(Copy, Clone)]
pub struct Positive<'a>(untrusted::Input<'a>);

impl<'a> Positive<'a> {
    pub(crate) fn new_non_empty_without_leading_zeros(input: untrusted::Input<'a>) -> Self {
        debug_assert!(!input.is_empty());
        debug_assert!(input.len() == 1 || input.as_slice_less_safe()[0] != 0);
        Self(input)
    }

    /// Returns the value, ordered from significant byte to least significant
    /// byte, without any leading zeros. The result is guaranteed to be
    /// non-empty.
    #[inline]
    #[must_use]
    pub fn big_endian_without_leading_zero(&self) -> &'a [u8] {
        self.big_endian_without_leading_zero_as_input()
            .as_slice_less_safe()
    }

    #[inline]
    pub(crate) fn big_endian_without_leading_zero_as_input(&self) -> untrusted::Input<'a> {
        self.0
    }
}

impl Positive<'_> {
    /// Returns the first byte.
    ///
    /// Will not panic because the value is guaranteed to have at least one
    /// byte.
    #[must_use]
    pub fn first_byte(&self) -> u8 {
        // This won't panic because
        self.0.as_slice_less_safe()[0]
    }
}
