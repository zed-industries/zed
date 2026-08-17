// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;

use crate::{
    atoms::{Atom, AtomHeader},
    fourcc::FourCc,
};

use log::warn;

/// Handler type.
#[derive(Debug, PartialEq, Eq)]
pub enum HandlerType {
    /// Video handler.
    Video,
    /// Audio handler.
    Sound,
    /// Subtitle handler.
    Subtitle,
    /// Metadata handler.
    Metadata,
    /// Text handler.
    Text,
    /// Unknown handler type.
    Other([u8; 4]),
}

/// Handler atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct HdlrAtom {
    /// Atom header.
    header: AtomHeader,
    /// Handler type.
    pub handler_type: HandlerType,
    /// Human-readable handler name.
    pub name: String,
}

impl Atom for HdlrAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        // Always 0 for MP4, but for Quicktime this contains the component type.
        let _ = reader.read_quad_bytes()?;

        let handler_type = match &reader.read_quad_bytes()? {
            b"vide" => HandlerType::Video,
            b"soun" => HandlerType::Sound,
            b"meta" => HandlerType::Metadata,
            b"subt" => HandlerType::Subtitle,
            b"text" => HandlerType::Text,
            &hdlr => {
                warn!("unknown handler type {:?}", FourCc::new(hdlr));
                HandlerType::Other(hdlr)
            }
        };

        // These bytes are reserved for MP4, but for QuickTime they contain the component
        // manufacturer, flags, and flags mask.
        reader.ignore_bytes(4 * 3)?;

        // Human readable UTF-8 string of the track type.
        let buf = reader.read_boxed_slice_exact((header.data_len - 24) as usize)?;
        let name = String::from_utf8_lossy(&buf).to_string();

        Ok(HdlrAtom { header, handler_type, name })
    }
}
