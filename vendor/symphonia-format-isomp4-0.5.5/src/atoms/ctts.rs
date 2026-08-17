// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

/// Composition time atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CttsAtom {
    /// Atom header.
    header: AtomHeader,
}

impl Atom for CttsAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(_reader: &mut B, _header: AtomHeader) -> Result<Self> {
        todo!()
    }
}
