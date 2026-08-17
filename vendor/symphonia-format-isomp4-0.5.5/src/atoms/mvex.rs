// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, MehdAtom, TrexAtom};

/// Movie extends atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MvexAtom {
    /// Atom header.
    pub header: AtomHeader,
    /// Movie extends header, optional.
    pub mehd: Option<MehdAtom>,
    /// Track extends box, one per track.
    pub trexs: Vec<TrexAtom>,
}

impl Atom for MvexAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut mehd = None;
        let mut trexs = Vec::new();

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::MovieExtendsHeader => {
                    mehd = Some(iter.read_atom::<MehdAtom>()?);
                }
                AtomType::TrackExtends => {
                    let trex = iter.read_atom::<TrexAtom>()?;
                    trexs.push(trex);
                }
                _ => (),
            }
        }

        Ok(MvexAtom { header, mehd, trexs })
    }
}
