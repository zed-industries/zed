// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, TfhdAtom, TrunAtom};

/// Track fragment atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct TrafAtom {
    /// Atom header.
    header: AtomHeader,
    /// Track fragment header.
    pub tfhd: TfhdAtom,
    /// Track fragment sample runs.
    pub truns: Vec<TrunAtom>,
    /// The total number of samples in this track fragment.
    pub total_sample_count: u32,
}

impl Atom for TrafAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut tfhd = None;
        let mut truns = Vec::new();

        let mut iter = AtomIterator::new(reader, header);

        let mut total_sample_count = 0;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::TrackFragmentHeader => {
                    tfhd = Some(iter.read_atom::<TfhdAtom>()?);
                }
                AtomType::TrackFragmentRun => {
                    let trun = iter.read_atom::<TrunAtom>()?;

                    // Increment the total sample count.
                    total_sample_count += trun.sample_count;

                    truns.push(trun);
                }
                _ => (),
            }
        }

        // Tfhd is mandatory.
        if tfhd.is_none() {
            return decode_error("isomp4: missing tfhd atom");
        }

        Ok(TrafAtom { header, tfhd: tfhd.unwrap(), truns, total_sample_count })
    }
}
