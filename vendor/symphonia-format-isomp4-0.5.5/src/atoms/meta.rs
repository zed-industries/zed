// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;
use symphonia_core::meta::MetadataRevision;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, IlstAtom};

/// User data atom.
#[allow(dead_code)]
pub struct MetaAtom {
    /// Atom header.
    header: AtomHeader,
    /// Metadata revision.
    pub metadata: Option<MetadataRevision>,
}

impl Debug for MetaAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(redacted)")
    }
}

impl MetaAtom {
    /// If metadata was read, consumes the metadata and returns it.
    pub fn take_metadata(&mut self) -> Option<MetadataRevision> {
        self.metadata.take()
    }
}

impl Atom for MetaAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    #[allow(clippy::single_match)]
    fn read<B: ReadBytes>(reader: &mut B, mut header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        // AtomIterator doesn't know the extra data was read already, so the extra data size must be
        // subtrated from the atom's data length.
        header.data_len -= AtomHeader::EXTRA_DATA_SIZE;

        let mut iter = AtomIterator::new(reader, header);

        let mut metadata = None;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::MetaList => {
                    metadata = Some(iter.read_atom::<IlstAtom>()?.metadata);
                }
                _ => (),
            }
        }

        Ok(MetaAtom { header, metadata })
    }
}
