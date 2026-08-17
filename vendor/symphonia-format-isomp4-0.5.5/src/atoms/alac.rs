// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::codecs::{CodecParameters, CODEC_TYPE_ALAC};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

#[allow(dead_code)]
#[derive(Debug)]
pub struct AlacAtom {
    /// Atom header.
    header: AtomHeader,
    /// ALAC extra data (magic cookie).
    extra_data: Box<[u8]>,
}

impl Atom for AlacAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (version, flags) = AtomHeader::read_extra(reader)?;

        if version != 0 {
            return unsupported_error("isomp4 (alac): unsupported alac version");
        }

        if flags != 0 {
            return decode_error("isomp4 (alac): flags not zero");
        }

        if header.data_len <= AtomHeader::EXTRA_DATA_SIZE {
            return decode_error("isomp4 (alac): invalid alac atom length");
        }

        // The ALAC magic cookie (aka extra data) is either 24 or 48 bytes long.
        let magic_len = match header.data_len - AtomHeader::EXTRA_DATA_SIZE {
            len @ 24 | len @ 48 => len as usize,
            _ => return decode_error("isomp4 (alac): invalid magic cookie length"),
        };

        // Read the magic cookie.
        let extra_data = reader.read_boxed_slice_exact(magic_len)?;

        Ok(AlacAtom { header, extra_data })
    }
}

impl AlacAtom {
    pub fn fill_codec_params(&self, codec_params: &mut CodecParameters) {
        codec_params.for_codec(CODEC_TYPE_ALAC).with_extra_data(self.extra_data.clone());
    }
}
