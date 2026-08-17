// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::{AudioBuffer, Signal};
use symphonia_core::errors::Result;
use symphonia_core::io::{BitReaderLtr, BufReader, ReadBitsLtr, ReadBytes};
use symphonia_core::util::bits::sign_extend_leq32_to_i32;

use crate::common::{ChannelMode, FrameHeader, Layer, Mode};
use crate::layer12::LAYER12_SCALEFACTORS;
use crate::synthesis;

struct QuantClass {
    /// C constant.
    c: f32,
    /// D constant.
    d: f32,
    /// Is grouping used?
    grouping: bool,
    /// Bits per raw sample (if grouping is false) or codeword (if grouping is true).
    bits: u8,
    /// Number of levels in a sample.
    nlevels: u16,
}

struct SbQuantInfo {
    /// Bit allocation for the sub-band.
    nbal: u8,
    /// Indicies into quantization class table. Valid for classes 0..2^nbal.
    classes: [u8; 16],
}

struct SbInfo {
    /// The maximum number of sub-bands.
    sblimit: usize,
    /// An index into the sub-band information table for each sub-band. Valid for sub-bands between
    /// 0..sblimit.
    bands: [u8; 32],
}

/// Quantization classes. Derived from ISO/IEC 11172-3 Table 3-B.4.
const QUANT_CLASS: [QuantClass; 17] = [
    QuantClass { c: 1.33333333333, d: 0.50000000000, grouping: true, bits: 5, nlevels: 3 },
    QuantClass { c: 1.60000000000, d: 0.50000000000, grouping: true, bits: 7, nlevels: 5 },
    QuantClass { c: 1.14285714286, d: 0.25000000000, grouping: false, bits: 3, nlevels: 7 },
    QuantClass { c: 1.77777777777, d: 0.50000000000, grouping: true, bits: 10, nlevels: 9 },
    QuantClass { c: 1.06666666666, d: 0.12500000000, grouping: false, bits: 4, nlevels: 15 },
    QuantClass { c: 1.03225806452, d: 0.06250000000, grouping: false, bits: 5, nlevels: 31 },
    QuantClass { c: 1.01587301587, d: 0.03125000000, grouping: false, bits: 6, nlevels: 63 },
    QuantClass { c: 1.00787401575, d: 0.01562500000, grouping: false, bits: 7, nlevels: 127 },
    QuantClass { c: 1.00392156863, d: 0.00781250000, grouping: false, bits: 8, nlevels: 255 },
    QuantClass { c: 1.00195694716, d: 0.00390625000, grouping: false, bits: 9, nlevels: 511 },
    QuantClass { c: 1.00097751711, d: 0.00195312500, grouping: false, bits: 10, nlevels: 1023 },
    QuantClass { c: 1.00048851979, d: 0.00097656250, grouping: false, bits: 11, nlevels: 2047 },
    QuantClass { c: 1.00024420024, d: 0.00048828125, grouping: false, bits: 12, nlevels: 4095 },
    QuantClass { c: 1.00012208522, d: 0.00024414063, grouping: false, bits: 13, nlevels: 8191 },
    QuantClass { c: 1.00006103888, d: 0.00012207031, grouping: false, bits: 14, nlevels: 16383 },
    QuantClass { c: 1.00003051851, d: 0.00006103516, grouping: false, bits: 15, nlevels: 32767 },
    QuantClass { c: 1.00001525902, d: 0.00003051758, grouping: false, bits: 16, nlevels: 65535 },
];

/// Sub-band quantization class information. Derived from ISO/IEC 11172-3 Tables 3-B.2a-d.
const SB_QUANT_INFO: [SbQuantInfo; 8] = [
    SbQuantInfo { nbal: 2, classes: [0, 0, 1, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    SbQuantInfo { nbal: 2, classes: [0, 0, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    SbQuantInfo { nbal: 3, classes: [0, 0, 1, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0] },
    SbQuantInfo { nbal: 3, classes: [0, 0, 1, 2, 3, 4, 5, 16, 0, 0, 0, 0, 0, 0, 0, 0] },
    SbQuantInfo { nbal: 4, classes: [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] },
    SbQuantInfo { nbal: 4, classes: [0, 0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] },
    SbQuantInfo { nbal: 4, classes: [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16] },
    SbQuantInfo { nbal: 4, classes: [0, 0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] },
];

/// Sub-band information table.
const SB_INFO: [SbInfo; 5] = [
    // Derived from ISO/IEC 11172-3 Table 3-B.2a.
    SbInfo {
        sblimit: 27,
        bands: [
            7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 6, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
    },
    // Derived from ISO/IEC 11172-3 Table 3-B.2b.
    SbInfo {
        sblimit: 30,
        bands: [
            7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 6, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
    },
    // Derived from ISO/IEC 11172-3 Table 3-B.2c.
    SbInfo {
        sblimit: 8,
        bands: [
            5, 5, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
    },
    // Derived from ISO/IEC 11172-3 Table 3-B.2d.
    SbInfo {
        sblimit: 12,
        bands: [
            5, 5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
    },
    // Derived from ISO/IEC 13818-3 Table 3-B.1.
    SbInfo {
        sblimit: 30,
        bands: [
            4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 0,
        ],
    },
];

/// Get quantization class for a specific class index.
#[inline(always)]
fn find_quant_class(sb_quant_info: &SbQuantInfo, class: u8) -> &'static QuantClass {
    &QUANT_CLASS[usize::from(sb_quant_info.classes[usize::from(class)])]
}

/// Get quantization information for a specific sub-band.
#[inline(always)]
fn find_sb_quant_info(sb_info: &SbInfo, sb: usize) -> &'static SbQuantInfo {
    &SB_QUANT_INFO[usize::from(sb_info.bands[sb])]
}

/// Get sub-band information for a frame with a given header.
fn find_sb_info(header: &FrameHeader) -> &'static SbInfo {
    let idx = if header.is_mpeg1() {
        // MPEG1 switches quantization tables based on bitrate per channel and sample rate.
        let num_channels = if header.channel_mode == ChannelMode::Mono { 1 } else { 2 };

        let bitrate_per_channel = header.bitrate / num_channels;

        if bitrate_per_channel <= 48_000 {
            // Table 3-B.2c and 3-B.2d are only used for bitrates <= 48 kbit/s.
            if header.sample_rate == 32_000 {
                3
            }
            else {
                2
            }
        }
        else if bitrate_per_channel <= 80_000 {
            // Table 3-B.2a is always used for 48 kbit/s < bitrates <= 80 kbits/s.
            0
        }
        else {
            // Table 3-B.2a and 3-B.2b as always used for bitrates > 80 kbit/s.
            // TODO: Free format also used this case, but we don't support it.
            usize::from(header.sample_rate != 48_000)
        }
    }
    else {
        // MPEG2 & MPEG2.5 use the same table regardless of bitrate and sample rate.
        4
    };

    &SB_INFO[idx]
}

/// Dequantize a sample, `raw`, of length `bits` bits.
#[inline]
fn dequantize(bs: &mut BitReaderLtr<'_>, class: &QuantClass) -> Result<[f32; 3]> {
    let mut raw = [0; 3];

    let bits = if class.grouping {
        // Read a packed (grouped) codeword from the bitstream, and unpack the codeword into 3
        // raw samples.
        let mut c = bs.read_bits_leq32(u32::from(class.bits))?;

        let nlevels = u32::from(class.nlevels);

        for item in &mut raw {
            *item = c % nlevels;
            c /= nlevels;
        }

        // Each raw sample is in the range 0..nlevels. Therefore, the bit width of an individual
        // raw sample is the minimum number of bits it takes to represent nlevels.
        nlevels.next_power_of_two().trailing_zeros()
    }
    else {
        // Read individial raw samples from the bitstream.
        let bits = u32::from(class.bits);

        for item in &mut raw {
            *item = bs.read_bits_leq32(bits)?;
        }

        bits
    };

    // The divisor for samples of `bits` width. Used to convert the raw integer sample into a
    // floating point sample.
    let divisor = (1 << (bits - 1)) as f32;

    let mut samples = [0.0; 3];

    for i in 0..3 {
        // Invert the most significant bit.
        let inv = raw[i] ^ 1 << (bits - 1);

        // Sign extend the sample.
        let a = sign_extend_leq32_to_i32(inv, bits);

        // Convert the sample into a fraction.
        let s = a as f32 / divisor;

        // Dequantize the sample.
        samples[i] = class.c * (s + class.d);
    }

    Ok(samples)
}

pub struct Layer2 {
    pub synthesis: [synthesis::SynthesisState; 2],
}

impl Layer2 {
    pub fn new() -> Self {
        Self { synthesis: Default::default() }
    }
}

impl Layer for Layer2 {
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
        let mut scfsi = [[0; 32]; 2];
        let mut scalefacs = [[[0; 32]; 3]; 2];

        let num_channels = header.n_channels();

        let sb_info = find_sb_info(header);

        let bound = match header.channel_mode {
            ChannelMode::JointStereo(Mode::Intensity { bound }) => bound as usize,
            ChannelMode::JointStereo(Mode::Layer3 { .. }) => {
                // This mode extension is exclusively used for layer 3, it is a fundamental error
                // in the decoder logic if layer 1 or 2 stereo encodings are being decoded with
                // this function.
                panic!("invalid mode extension for layer 2 stereo decoding")
            }
            _ => 32,
        }
        .min(sb_info.sblimit);

        // Read the class index (allocation in the standard) for each non-intensity coded sub-band.
        for sb in 0..bound {
            let nbal = find_sb_quant_info(sb_info, sb).nbal;

            for chan in &mut alloc[..num_channels] {
                chan[sb] = bs.read_bits_leq32(u32::from(nbal))? as u8;
            }
        }

        // Read the class index (allocation in the standard) for each intensity coded sub-band.
        for sb in bound..sb_info.sblimit {
            let nbal = find_sb_quant_info(sb_info, sb).nbal;

            let value = bs.read_bits_leq32(u32::from(nbal))? as u8;

            alloc[0][sb] = value;
            alloc[1][sb] = value;
        }

        // Read scale factor selection information.
        for sb in 0..sb_info.sblimit {
            for ch in 0..num_channels {
                if alloc[ch][sb] != 0 {
                    scfsi[ch][sb] = bs.read_bits_leq32(2)? as u8;
                }
            }
        }

        // Read scale factors.
        for sb in 0..sb_info.sblimit {
            for ch in 0..num_channels {
                if alloc[ch][sb] != 0 {
                    let mut indicies = [bs.read_bits_leq32(6)? as u8; 3];

                    match scfsi[ch][sb] {
                        0 => {
                            indicies[1] = bs.read_bits_leq32(6)? as u8;
                            indicies[2] = bs.read_bits_leq32(6)? as u8;
                        }
                        1 => {
                            indicies[2] = bs.read_bits_leq32(6)? as u8;
                        }
                        2 => (),
                        3 => {
                            indicies[1] = bs.read_bits_leq32(6)? as u8;
                            indicies[2] = indicies[1];
                        }
                        _ => unreachable!(),
                    }

                    scalefacs[ch][0][sb] = indicies[0];
                    scalefacs[ch][1][sb] = indicies[1];
                    scalefacs[ch][2][sb] = indicies[2];
                }
            }
        }

        // Decode samples.
        let mut samples = [[0f32; 1152]; 2];

        for gr in 0..12 {
            // Non-intensity coded sub-bands.
            for sb in 0..bound {
                let sb_quant_info = find_sb_quant_info(sb_info, sb);

                for ch in 0..num_channels {
                    let class_idx = alloc[ch][sb];

                    if class_idx != 0 {
                        let quant_class = find_quant_class(sb_quant_info, class_idx);

                        // Samples within a sub-band are decoded in-order. Dequantize the next group
                        // of three samples for the sub-band.
                        let triplet = dequantize(&mut bs, quant_class)?;

                        // A sub-band is divided into three partitions of 12 samples each. Each
                        // partition has its own scalefactor. Therefore, the partition index can be
                        // calculated by 3 * gr / 12, or simplified, gr / 4.
                        let scalefac = LAYER12_SCALEFACTORS[usize::from(scalefacs[ch][gr / 4][sb])];

                        // Unpack and unscale the samples.
                        samples[ch][36 * sb + 3 * gr + 0] = scalefac * triplet[0];
                        samples[ch][36 * sb + 3 * gr + 1] = scalefac * triplet[1];
                        samples[ch][36 * sb + 3 * gr + 2] = scalefac * triplet[2];
                    }
                }
            }

            // Intensity coded sub-bands.
            for sb in bound..sb_info.sblimit {
                // Same decode procedure as non-intensity coded sub-bands, but the same pre-scaled
                // sample value is used for both channels.
                let class_idx = alloc[0][sb];

                if class_idx != 0 {
                    let quant_class = find_quant_class(find_sb_quant_info(sb_info, sb), class_idx);

                    let triplet = dequantize(&mut bs, quant_class)?;

                    for ch in 0..num_channels {
                        let scalefac = LAYER12_SCALEFACTORS[usize::from(scalefacs[ch][gr / 4][sb])];

                        samples[ch][36 * sb + 3 * gr + 0] = scalefac * triplet[0];
                        samples[ch][36 * sb + 3 * gr + 1] = scalefac * triplet[1];
                        samples[ch][36 * sb + 3 * gr + 2] = scalefac * triplet[2];
                    }
                }
            }
        }

        // Each packet will yield 1152 audio frames. After reserving frames, all steps must be
        // infalliable.
        out.render_reserved(Some(1152));

        for (ch, samples) in samples.iter().enumerate().take(num_channels) {
            // Perform polyphase synthesis and generate PCM samples.
            synthesis::synthesis(&mut self.synthesis[ch], 36, samples, out.chan_mut(ch));
        }

        Ok(())
    }
}
