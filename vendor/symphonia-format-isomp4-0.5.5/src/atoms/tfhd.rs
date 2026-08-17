// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

/// Track fragment header atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct TfhdAtom {
    /// Atom header.
    header: AtomHeader,
    pub track_id: u32,
    pub base_data_offset: Option<u64>,
    pub sample_desc_idx: Option<u32>,
    pub default_sample_duration: Option<u32>,
    pub default_sample_size: Option<u32>,
    pub default_sample_flags: Option<u32>,
    /// If true, there are no samples for this time duration.
    pub duration_is_empty: bool,
    /// If true, the base data offset for this track is the first byte of the parent containing moof
    /// atom.
    pub default_base_is_moof: bool,
}

impl Atom for TfhdAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, flags) = AtomHeader::read_extra(reader)?;

        let track_id = reader.read_be_u32()?;

        let base_data_offset = match flags & 0x1 {
            0 => None,
            _ => Some(reader.read_be_u64()?),
        };

        let sample_desc_idx = match flags & 0x2 {
            0 => None,
            _ => Some(reader.read_be_u32()?),
        };

        let default_sample_duration = match flags & 0x8 {
            0 => None,
            _ => Some(reader.read_be_u32()?),
        };

        let default_sample_size = match flags & 0x10 {
            0 => None,
            _ => Some(reader.read_be_u32()?),
        };

        let default_sample_flags = match flags & 0x20 {
            0 => None,
            _ => Some(reader.read_be_u32()?),
        };

        let duration_is_empty = (flags & 0x1_0000) != 0;

        // The default-base-is-moof flag is ignored if the base-data-offset flag is set.
        let default_base_is_moof = (flags & 0x1 == 0) && (flags & 0x2_0000 != 0);

        Ok(TfhdAtom {
            header,
            track_id,
            base_data_offset,
            sample_desc_idx,
            default_sample_duration,
            default_sample_size,
            default_sample_flags,
            duration_is_empty,
            default_base_is_moof,
        })
    }
}
