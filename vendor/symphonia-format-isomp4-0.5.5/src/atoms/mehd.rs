// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

/// Movie extends header atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MehdAtom {
    /// Atom header.
    header: AtomHeader,
    /// Fragment duration.
    pub fragment_duration: u64,
}

impl Atom for MehdAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (version, _) = AtomHeader::read_extra(reader)?;

        let fragment_duration = match version {
            0 => u64::from(reader.read_be_u32()?),
            1 => reader.read_be_u64()?,
            _ => {
                return decode_error("isomp4: invalid mehd version");
            }
        };

        Ok(MehdAtom { header, fragment_duration })
    }
}
