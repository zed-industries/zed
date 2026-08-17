// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::{AudioBuffer, Signal};
use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::{BitReaderLtr, BufReader, ReadBitsLtr, ReadBytes};
use symphonia_core::util::bits::sign_extend_leq32_to_i32;

use crate::common::*;
use crate::layer12::LAYER12_SCALEFACTORS;
use crate::synthesis;

use lazy_static::lazy_static;

lazy_static! {
    static ref FACTOR: [f32; 16] = {
        let mut factor = [0f32; 16];

        for (i, factor) in factor.iter_mut().enumerate().skip(2) {
            // As per ISO/IEC 11172-3, given the nb-bit signed raw sample, val, dequantization is
            // defined as follows.
            //
            // fractional = val / 2^(nb - 1)
            // dequantized = (2^nb) / (2^nb - 1) * (fractional * 2^(-nb + 1))
            //
            // After combining, expanding, and simplifying the above equations, the complete
            // calculation can be expressed as below.
            //
            // [(2^nb) / ((2^nb) - 1)] * 2^(-nb + 1) * (val + 1)
            // -------------------------------------
            //                 factor
            //
            // Therefore, dequantization can be reduced to a single multiplication and addition.
            // This lookup table generator computes factor for nb-bits between 2..15, inclusive.
            let a = 1 << i;
            let b = 1 << (i - 1);

            *factor = (a as f32 / (a - 1) as f32) * (b as f32).recip();
        }

        factor
    };
}

/// Dequantize a sample, `raw`, of length `bits` bits.
#[inline(always)]
fn dequantize(bits: u32, factor: f32, raw: u32) -> f32 {
    // Invert the most significant bit.
    let inv = raw ^ 1 << (bits - 1);

    // Sign extend the sample.
    let a = sign_extend_leq32_to_i32(inv, bits);

    // Dequantize the sample.
    factor * (a + 1) as f32
}

pub struct Layer1 {
    pub synthesis: [synthesis::SynthesisState; 2],
}

impl Layer1 {
    pub fn new() -> Self {
        Self { synthesis: Default::default() }
    }
}

impl Layer for Layer1 {
    fn decode(
        &mut self,
        reader: &mut BufReader<'_>,
        header: &FrameHeader,
        out: &mut AudioBuffer<f32>,
    ) -> Result<()> {
        // Ignore the CRC.
        let _crc = if header.has_crc { Some(reader.read_be_u16()?) } else { None };

        let mut bs = BitReaderLtr::new(reader.read_buf_bytes_available_ref());

        let mut alloc = [[0; 32]; 2];
        let mut scalefacs = [[0.0; 32]; 2];

        let num_channels = header.n_channels();

        let bound = match header.channel_mode {
            ChannelMode::JointStereo(Mode::Intensity { bound }) => bound as usize,
            ChannelMode::JointStereo(Mode::Layer3 { .. }) => {
                // This mode extension is exclusively used for layer 3, it is a fundamental error
                // in the decoder logic if layer 1 or 2 stereo encodings are being decoded with
                // this function.
                panic!("invalid mode extension for layer 1 stereo decoding")
            }
            _ => 32,
        };

        // Read bit allocations for each non-intensity coded sub-bands.
        for sb in 0..bound {
            for chan in &mut alloc {
                let bits = bs.read_bits_leq32(4)? as u8;

                if bits > 0xe {
                    return decode_error("mp1: invalid bit allocation");
                }

                chan[sb] = if bits != 0 { bits + 1 } else { 0 };
            }
        }

        // Read bit allocations for the intensity coded sub-bands.
        for sb in bound..32 {
            let bits = bs.read_bits_leq32(4)? as u8;

            if bits > 0xe {
                return decode_error("mp1: invalid bit allocation");
            }

            let ba = if bits != 0 { bits + 1 } else { 0 };

            alloc[0][sb] = ba;
            alloc[1][sb] = ba;
        }

        // Read scalefactors for each sub-band.
        for sb in 0..32 {
            for ch in 0..num_channels {
                if alloc[ch][sb] != 0 {
                    let index = bs.read_bits_leq32(6)? as usize;

                    scalefacs[ch][sb] = LAYER12_SCALEFACTORS[index];
                }
            }
        }

        let factor = &FACTOR;

        // Decode samples.
        let mut samples = [[0f32; 384]; 2];

        for s in 0..12 {
            // Non-intensity coded sub-bands.
            for sb in 0..bound {
                for ch in 0..num_channels {
                    let bits = u32::from(alloc[ch][sb]);

                    if bits != 0 {
                        // Read the raw sample value from the bistream.
                        let raw = bs.read_bits_leq32(bits)?;

                        // Dequantize the raw sample.
                        let sample = dequantize(bits, factor[bits as usize], raw);

                        // Unscale the sample.
                        samples[ch][12 * sb + s] = scalefacs[ch][sb] * sample;
                    }
                }
            }

            // Intensity coded sub-bands.
            for sb in bound..32 {
                let bits = u32::from(alloc[0][sb]);

                if bits != 0 {
                    // Read the raw sample value from the bistream.
                    let raw = bs.read_bits_leq32(bits)?;

                    // Dequantize the raw sample.
                    let sample = dequantize(bits, factor[bits as usize], raw);

                    // Unscale the sample and copy it into both channels.
                    for ch in 0..num_channels {
                        samples[ch][12 * sb + s] = scalefacs[ch][sb] * sample;
                    }
                }
            }
        }

        // Each packet will yield 384 audio frames. After reserving frames, all steps must be
        // infalliable.
        out.render_reserved(Some(384));

        for (ch, samples) in samples.iter().enumerate().take(num_channels) {
            // Perform polyphase synthesis and generate PCM samples.
            synthesis::synthesis(&mut self.synthesis[ch], 12, samples, out.chan_mut(ch));
        }

        Ok(())
    }
}
