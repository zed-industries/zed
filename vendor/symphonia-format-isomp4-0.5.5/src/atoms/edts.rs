// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, ElstAtom};

/// Edits atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EdtsAtom {
    header: AtomHeader,
    pub elst: Option<ElstAtom>,
}

impl Atom for EdtsAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    #[allow(clippy::single_match)]
    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut elst = None;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::EditList => {
                    elst = Some(iter.read_atom::<ElstAtom>()?);
                }
                _ => (),
            }
        }

        Ok(EdtsAtom { header, elst })
    }
}
