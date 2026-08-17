// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

/// Chunk offset atom (64-bit version).
#[allow(dead_code)]
#[derive(Debug)]
pub struct Co64Atom {
    /// Atom header.
    header: AtomHeader,
    pub chunk_offsets: Vec<u64>,
}

impl Atom for Co64Atom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        let entry_count = reader.read_be_u32()?;

        // TODO: Apply a limit.
        let mut chunk_offsets = Vec::with_capacity(entry_count as usize);

        for _ in 0..entry_count {
            chunk_offsets.push(reader.read_be_u64()?);
        }

        Ok(Co64Atom { header, chunk_offsets })
    }
}
