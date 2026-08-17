// Copyright 2022 the Raw Resource Handle Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::Blob;

/// Owned shareable font resource.
#[derive(Clone, PartialEq, Debug)]
pub struct FontData {
    /// Blob containing the content of the font file.
    pub data: Blob<u8>,
    /// Index of the font in a collection, or 0 for a single font.
    pub index: u32,
}

impl FontData {
    /// Creates a new font with the given data and collection index.
    #[must_use]
    pub fn new(data: Blob<u8>, index: u32) -> Self {
        Self { data, index }
    }
}
