// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};
use crate::fp::FpI8;

/// Sound header atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct SmhdAtom {
    /// Atom header.
    header: AtomHeader,
    /// Stereo balance.
    pub balance: FpI8,
}

impl Atom for SmhdAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        // Stereo balance
        let balance = FpI8::parse_raw(reader.read_be_u16()? as i16);

        // Reserved.
        let _ = reader.read_be_u16()?;

        Ok(SmhdAtom { header, balance })
    }
}
