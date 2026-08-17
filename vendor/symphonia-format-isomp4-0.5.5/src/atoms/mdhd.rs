// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

fn parse_language(code: u16) -> String {
    // An ISO language code outside of these bounds is not valid.
    if code < 0x400 || code > 0x7fff {
        String::new()
    }
    else {
        let chars = [
            ((code >> 10) & 0x1f) as u8 + 0x60,
            ((code >> 5) & 0x1f) as u8 + 0x60,
            ((code >> 0) & 0x1f) as u8 + 0x60,
        ];

        String::from_utf8_lossy(&chars).to_string()
    }
}

/// Media header atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MdhdAtom {
    /// Atom header.
    header: AtomHeader,
    /// Creation time.
    pub ctime: u64,
    /// Modification time.
    pub mtime: u64,
    /// Timescale.
    pub timescale: u32,
    /// Duration of the media in timescale units.
    pub duration: u64,
    /// Language.
    pub language: String,
}

impl Atom for MdhdAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (version, _) = AtomHeader::read_extra(reader)?;

        let mut mdhd = MdhdAtom {
            header,
            ctime: 0,
            mtime: 0,
            timescale: 0,
            duration: 0,
            language: String::new(),
        };

        match version {
            0 => {
                mdhd.ctime = u64::from(reader.read_be_u32()?);
                mdhd.mtime = u64::from(reader.read_be_u32()?);
                mdhd.timescale = reader.read_be_u32()?;
                // 0xffff_ffff is a special case.
                mdhd.duration = match reader.read_be_u32()? {
                    u32::MAX => u64::MAX,
                    duration => u64::from(duration),
                };
            }
            1 => {
                mdhd.ctime = reader.read_be_u64()?;
                mdhd.mtime = reader.read_be_u64()?;
                mdhd.timescale = reader.read_be_u32()?;
                mdhd.duration = reader.read_be_u64()?;
            }
            _ => {
                return decode_error("isomp4: invalid mdhd version");
            }
        }

        mdhd.language = parse_language(reader.read_be_u16()?);

        // Quality
        let _ = reader.read_be_u16()?;

        Ok(mdhd)
    }
}
