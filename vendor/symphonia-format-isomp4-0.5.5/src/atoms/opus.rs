// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::codecs::{CodecParameters, CODEC_TYPE_OPUS};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::io::ReadBytes;

use crate::atoms::{Atom, AtomHeader};

#[allow(dead_code)]
#[derive(Debug)]
pub struct OpusAtom {
    /// Atom header.
    header: AtomHeader,
    /// Opus extra data (identification header).
    extra_data: Box<[u8]>,
}

impl Atom for OpusAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        const OPUS_MAGIC: &[u8] = b"OpusHead";
        const OPUS_MAGIC_LEN: usize = OPUS_MAGIC.len();

        const MIN_OPUS_EXTRA_DATA_SIZE: usize = OPUS_MAGIC_LEN + 11;
        const MAX_OPUS_EXTRA_DATA_SIZE: usize = MIN_OPUS_EXTRA_DATA_SIZE + 257;

        // Offset of the Opus version number in the extra data.
        const OPUS_EXTRADATA_VERSION_OFFSET: usize = OPUS_MAGIC_LEN;

        // The dops atom contains an Opus identification header excluding the OpusHead magic
        // signature. Therefore, the atom data length should be atleast as long as the shortest
        // Opus identification header.
        let data_len = header.data_len as usize;

        if data_len < MIN_OPUS_EXTRA_DATA_SIZE - OPUS_MAGIC_LEN {
            return decode_error("isomp4 (opus): opus identification header too short");
        }

        if data_len > MAX_OPUS_EXTRA_DATA_SIZE - OPUS_MAGIC_LEN {
            return decode_error("isomp4 (opus): opus identification header too large");
        }

        let mut extra_data = vec![0; OPUS_MAGIC_LEN + data_len].into_boxed_slice();

        // The Opus magic is excluded in the atom, but the extra data must start with it.
        extra_data[..OPUS_MAGIC_LEN].copy_from_slice(OPUS_MAGIC);

        // Read the extra data from the atom.
        reader.read_buf_exact(&mut extra_data[OPUS_MAGIC_LEN..])?;

        // Verify the version number is 0.
        if extra_data[OPUS_EXTRADATA_VERSION_OFFSET] != 0 {
            return unsupported_error("isomp4 (opus): unsupported opus version");
        }

        Ok(OpusAtom { header, extra_data })
    }
}

impl OpusAtom {
    pub fn fill_codec_params(&self, codec_params: &mut CodecParameters) {
        codec_params.for_codec(CODEC_TYPE_OPUS).with_extra_data(self.extra_data.clone());
    }
}
