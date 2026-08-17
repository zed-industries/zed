// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, MfhdAtom, TrafAtom};

/// Movie fragment atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MoofAtom {
    /// Atom header.
    header: AtomHeader,
    /// The position of the first byte of this moof atom. This is used as the anchor point for the
    /// subsequent track atoms.
    pub moof_base_pos: u64,
    /// Movie fragment header.
    pub mfhd: MfhdAtom,
    /// Track fragments.
    pub trafs: Vec<TrafAtom>,
}

impl Atom for MoofAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let moof_base_pos = reader.pos() - AtomHeader::HEADER_SIZE;

        let mut mfhd = None;
        let mut trafs = Vec::new();

        let mut iter = AtomIterator::new(reader, header);

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::MovieFragmentHeader => {
                    mfhd = Some(iter.read_atom::<MfhdAtom>()?);
                }
                AtomType::TrackFragment => {
                    let traf = iter.read_atom::<TrafAtom>()?;
                    trafs.push(traf);
                }
                _ => (),
            }
        }

        if mfhd.is_none() {
            return decode_error("isomp4: missing mfhd atom");
        }

        Ok(MoofAtom { header, moof_base_pos, mfhd: mfhd.unwrap(), trafs })
    }
}
