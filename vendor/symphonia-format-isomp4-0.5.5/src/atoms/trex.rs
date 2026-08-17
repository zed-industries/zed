// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

/// Track extends atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct TrexAtom {
    /// Atom header.
    header: AtomHeader,
    /// Track this atom describes.
    pub track_id: u32,
    /// Default sample description index.
    pub default_sample_desc_idx: u32,
    /// Default sample duration.
    pub default_sample_duration: u32,
    /// Default sample size.
    pub default_sample_size: u32,
    /// Default sample flags.
    pub default_sample_flags: u32,
}

impl Atom for TrexAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        Ok(TrexAtom {
            header,
            track_id: reader.read_be_u32()?,
            default_sample_desc_idx: reader.read_be_u32()?,
            default_sample_duration: reader.read_be_u32()?,
            default_sample_size: reader.read_be_u32()?,
            default_sample_flags: reader.read_be_u32()?,
        })
    }
}
