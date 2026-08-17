// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, SmhdAtom, StblAtom};

/// Media information atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MinfAtom {
    /// Atom header.
    header: AtomHeader,
    /// Sound media header atom.
    pub smhd: Option<SmhdAtom>,
    /// Sample table atom.
    pub stbl: StblAtom,
}

impl Atom for MinfAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut smhd = None;
        let mut stbl = None;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::SoundMediaHeader => {
                    smhd = Some(iter.read_atom::<SmhdAtom>()?);
                }
                AtomType::SampleTable => {
                    stbl = Some(iter.read_atom::<StblAtom>()?);
                }
                _ => (),
            }
        }

        if stbl.is_none() {
            return decode_error("isomp4: missing stbl atom");
        }

        Ok(MinfAtom { header, smhd, stbl: stbl.unwrap() })
    }
}
