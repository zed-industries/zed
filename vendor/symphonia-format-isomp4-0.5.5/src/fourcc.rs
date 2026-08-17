// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

/// Four character codes for typical Ftyps (reference: http://ftyps.com/).
#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct FourCc {
    val: [u8; 4],
}

impl FourCc {
    /// Construct a new FourCC code from the given byte array.
    pub fn new(val: [u8; 4]) -> Self {
        Self { val }
    }
}

impl fmt::Debug for FourCc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match std::str::from_utf8(&self.val) {
            Ok(name) => f.write_str(name),
            _ => write!(f, "{:x?}", self.val),
        }
    }
}
