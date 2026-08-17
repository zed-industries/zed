// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, EsdsAtom};

use super::{AtomIterator, AtomType};

#[allow(dead_code)]
#[derive(Debug)]
pub struct WaveAtom {
    /// Atom header.
    header: AtomHeader,
    pub esds: Option<EsdsAtom>,
}

impl Atom for WaveAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut esds = None;

        while let Some(header) = iter.next()? {
            if header.atype == AtomType::Esds {
                esds = Some(iter.read_atom::<EsdsAtom>()?);
            }
        }

        Ok(WaveAtom { header, esds })
    }
}
