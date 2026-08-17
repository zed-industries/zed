// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem;
use std::vec::Vec;

use symphonia_core::audio::{AudioBuffer, Signal};
use symphonia_core::checksum::Md5;
use symphonia_core::io::Monitor;

/// `Validator` computes the MD5 checksum of an audio stream taking into account the peculiarities
/// of FLAC's MD5 validation scheme.
#[derive(Default)]
pub struct Validator {
    state: Md5,
    buf: Vec<u8>,
}

impl Validator {
    /// Processes the audio buffer and updates the state of the validator.
    pub fn update(&mut self, buf: &AudioBuffer<i32>, bps: u32) {
        // The MD5 checksum is calculated on a buffer containing interleaved audio samples of the
        // correct sample width. While FLAC can encode and decode samples of arbitrary bit widths,
        // the samples in the buffer must be a multiple of 8-bits.
        //
        // Additionally, Symphonia's AudioBuffer's are in planar format, and the FLAC decoder works
        // internally on signed 32-bit samples exclusively.
        //
        // Therefore, to compute the checksum, the audio buffer samples must truncated to the
        // correct bit-width, interlaced, and converted to a little-endian byte buffer. The byte
        // buffer can then be passed to the MD5 algorithm for hashing.

        // Round the sample bit width up to the nearest byte.
        let bytes_per_sample = match bps {
            0 => return,
            1..=8 => 1,
            9..=16 => 2,
            17..=24 => 3,
            25..=32 => 4,
            _ => unreachable!(),
        };

        let n_channels = buf.spec().channels.count();
        let n_frames = buf.frames();

        // Calculate the total size of all the samples in bytes.
        let buf_len = n_channels * n_frames * bytes_per_sample;

        // Ensure the byte buffer length can accomodate all the samples.
        if self.buf.len() < buf_len {
            self.buf.resize(buf_len, 0u8);
        }

        // Populate the hash buffer with samples truncated to the correct width. A &[u8] slice of
        // all the samples in hash buffer will be returned.
        let buf_slice = match bytes_per_sample {
            1 => copy_as_i8(buf, &mut self.buf, n_channels, n_frames),
            2 => copy_as_i16(buf, &mut self.buf, n_channels, n_frames),
            3 => copy_as_i24(buf, &mut self.buf, n_channels, n_frames),
            4 => copy_as_i32(buf, &mut self.buf, n_channels, n_frames),
            _ => unreachable!(),
        };

        // Update the MD5 state.
        self.state.process_buf_bytes(buf_slice);
    }

    /// Get the checksum.
    pub fn md5(&mut self) -> [u8; 16] {
        self.state.md5()
    }
}

fn copy_as_i24<'a>(
    samples: &AudioBuffer<i32>,
    buf: &'a mut [u8],
    n_channels: usize,
    n_frames: usize,
) -> &'a [u8] {
    const SIZE_OF_I24: usize = 24 / 8;

    for ch in 0..n_channels {
        for (out, sample) in
            buf.chunks_exact_mut(SIZE_OF_I24).skip(ch).step_by(n_channels).zip(samples.chan(ch))
        {
            out.copy_from_slice(&sample.to_le_bytes()[0..SIZE_OF_I24]);
        }
    }

    &buf[..n_channels * n_frames * SIZE_OF_I24]
}

macro_rules! copy_as {
    ($name:ident, $type:ty) => {
        fn $name<'a>(
            samples: &AudioBuffer<i32>,
            buf: &'a mut [u8],
            n_channels: usize,
            n_frames: usize,
        ) -> &'a [u8] {
            for ch in 0..n_channels {
                for (out, sample) in buf
                    .chunks_exact_mut(mem::size_of::<$type>())
                    .skip(ch)
                    .step_by(n_channels)
                    .zip(samples.chan(ch))
                {
                    out.copy_from_slice(&(*sample as $type).to_le_bytes());
                }
            }

            &buf[..n_channels * n_frames * mem::size_of::<$type>()]
        }
    };
}

copy_as!(copy_as_i8, i8);
copy_as!(copy_as_i16, i16);
copy_as!(copy_as_i32, i32);
