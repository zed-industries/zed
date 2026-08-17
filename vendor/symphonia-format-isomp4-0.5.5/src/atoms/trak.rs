// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, EdtsAtom, MdiaAtom, TkhdAtom};

/// Track atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct TrakAtom {
    /// Atom header.
    header: AtomHeader,
    /// Track header atom.
    pub tkhd: TkhdAtom,
    /// Optional, edit list atom.
    pub edts: Option<EdtsAtom>,
    /// Media atom.
    pub mdia: MdiaAtom,
}

impl Atom for TrakAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut tkhd = None;
        let mut edts = None;
        let mut mdia = None;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::TrackHeader => {
                    tkhd = Some(iter.read_atom::<TkhdAtom>()?);
                }
                AtomType::Edit => {
                    edts = Some(iter.read_atom::<EdtsAtom>()?);
                }
                AtomType::Media => {
                    mdia = Some(iter.read_atom::<MdiaAtom>()?);
                }
                _ => (),
            }
        }

        if tkhd.is_none() {
            return decode_error("isomp4: missing tkhd atom");
        }

        if mdia.is_none() {
            return decode_error("isomp4: missing mdia atom");
        }

        Ok(TrakAtom { header, tkhd: tkhd.unwrap(), edts, mdia: mdia.unwrap() })
    }
}
