// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBitsLtr;

use crate::common::{ChannelMode, FrameHeader};

use super::{common::*, FrameData, Granule, GranuleChannel};

/// Pairs of bit lengths for MPEG version 1 scale factors. For MPEG version 1, there are two
/// possible bit lengths for scale factors: slen1 and slen2. The first N of bands have scale factors
/// of bit length slen1, while the remaining bands have length slen2. The value of the switch point,
/// N, is determined by block type.
///
/// This table is indexed by scalefac_compress.
const SCALE_FACTOR_SLEN: [(u32, u32); 16] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (3, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 1),
    (2, 2),
    (2, 3),
    (3, 1),
    (3, 2),
    (3, 3),
    (4, 2),
    (4, 3),
];

/// For MPEG version 2, each scale factor band has a different scale factor. The length in bits of
/// a scale factor (slen) can be one of 4 values. The values in this table indicate the number of
/// scale factors that have length slen[0..4]. Slen[0..4] is calculated from scalefac_compress.
///
/// This table is indexed by channel_mode, scalefac_compress, and block_type.
const SCALE_FACTOR_MPEG2_NSFB: [[[usize; 4]; 3]; 6] = [
    // Intensity stereo channel modes.
    [[7, 7, 7, 0], [12, 12, 12, 0], [6, 15, 12, 0]],
    [[6, 6, 6, 3], [12, 9, 9, 6], [6, 12, 9, 6]],
    [[8, 8, 5, 0], [15, 12, 9, 0], [6, 18, 9, 0]],
    // Other channel modes.
    [[6, 5, 5, 5], [9, 9, 9, 9], [6, 9, 9, 9]],
    [[6, 5, 7, 3], [9, 9, 12, 6], [6, 9, 12, 6]],
    [[11, 10, 0, 0], [18, 18, 0, 0], [15, 18, 0, 0]],
];

/// Reads the side_info for a single channel in a granule from a `BitStream`.
fn read_granule_channel_side_info<B: ReadBitsLtr>(
    bs: &mut B,
    channel: &mut GranuleChannel,
    header: &FrameHeader,
) -> Result<()> {
    channel.part2_3_length = bs.read_bits_leq32(12)? as u16;
    channel.big_values = bs.read_bits_leq32(9)? as u16;

    // The maximum number of samples in a granule is 576. One big_value decodes to 2 samples,
    // therefore there can be no more than 288 (576/2) big_values.
    if channel.big_values > 288 {
        return decode_error("mpa: granule big_values > 288");
    }

    channel.global_gain = bs.read_bits_leq32(8)? as u8;

    channel.scalefac_compress =
        if header.is_mpeg1() { bs.read_bits_leq32(4) } else { bs.read_bits_leq32(9) }? as u16;

    let window_switching = bs.read_bool()?;

    if window_switching {
        let block_type_enc = bs.read_bits_leq32(2)?;

        let is_mixed = bs.read_bool()?;

        channel.block_type = match block_type_enc {
            // Only transitional Long blocks (Start, End) are allowed with window switching.
            0b00 => return decode_error("mpa: invalid block_type"),
            0b01 => BlockType::Start,
            0b10 => BlockType::Short { is_mixed },
            0b11 => BlockType::End,
            _ => unreachable!(),
        };

        // When window switching is used, there are only two regions, therefore there are only
        // two table selectors.
        for i in 0..2 {
            channel.table_select[i] = bs.read_bits_leq32(5)? as u8;
        }

        for i in 0..3 {
            channel.subblock_gain[i] = bs.read_bits_leq32(3)? as u8;
        }

        // When using window switching, the boundaries of region[0..3] are set implicitly according
        // to the MPEG version and block type. Below, the boundaries to set as per the applicable
        // standard.
        //
        // If MPEG version 2.5 specifically...
        if header.is_mpeg2p5() {
            // For MPEG2.5, the number of scale-factor bands in region0 depends on the block type.
            // The standard indicates these values as 1 less than the actual value, therefore 1 is
            // added here to both values.
            let region0_count = match channel.block_type {
                BlockType::Short { is_mixed: false } => 5 + 1,
                _ => 7 + 1,
            };

            channel.region1_start = SFB_LONG_BANDS[header.sample_rate_idx][region0_count];
        }
        // If MPEG version 1, OR the block type is Short...
        else if header.is_mpeg1() || block_type_enc == 0b10 {
            // For MPEG1 with transitional LONG blocks, the first 8 LONG scale-factor bands are used
            // for region0. These bands are always [4, 4, 4, 4, 4, 4, 6, 6, ...] regardless of
            // sample rate. These bands sum to 36 samples.
            //
            // For MPEG1 with SHORT blocks, the first 9 SHORT scale-factor bands are used for
            // region0. These band are always [4, 4, 4, 4, 4, 4, 4, 4, 4, ...] regardless of sample
            // rate. These bands also sum to 36 samples.
            //
            // Finally, for MPEG2 with SHORT blocks, the first 9 short scale-factor bands are used
            // for region0. These bands are also always  [4, 4, 4, 4, 4, 4, 4, 4, 4, ...] regardless
            // of sample and thus sum to 36 samples.
            //
            // In all cases, the region0_count is 36.
            //
            // TODO: This is not accurate for MPEG2.5 at 8kHz.
            channel.region1_start = 36;
        }
        // If MPEG version 2 AND the block type is not Short...
        else {
            // For MPEG2 and transitional LONG blocks, the first 8 LONG scale-factor bands are used
            // for region0. These bands are always [6, 6, 6, 6, 6, 6, 8, 10, ...] regardless of
            // sample rate. These bands sum to 54.
            channel.region1_start = 54;
        }

        // The second region, region1, spans the remaining samples. Therefore the third region,
        // region2, isn't used.
        channel.region2_start = 576;
    }
    else {
        // If window switching is not used, the block type is always Long.
        channel.block_type = BlockType::Long;

        for i in 0..3 {
            channel.table_select[i] = bs.read_bits_leq32(5)? as u8;
        }

        // When window switching is not used, only LONG scale-factor bands are used for each region.
        // The number of bands in region0 and region1 are defined in side_info. The stored value is
        // 1 less than the actual value.
        let region0_count = bs.read_bits_leq32(4)? as usize + 1;
        let region0_1_count = bs.read_bits_leq32(3)? as usize + region0_count + 1;

        channel.region1_start = SFB_LONG_BANDS[header.sample_rate_idx][region0_count];

        // The count in region0_1_count may exceed the last band (22) in the LONG bands table.
        // Protect against this.
        channel.region2_start = match region0_1_count {
            0..=22 => SFB_LONG_BANDS[header.sample_rate_idx][region0_1_count],
            _ => 576,
        };
    }

    // For MPEG2, preflag is determined implicitly when reading the scale factors.
    channel.preflag = if header.is_mpeg1() { bs.read_bool()? } else { false };

    channel.scalefac_scale = bs.read_bool()?;
    channel.count1table_select = bs.read_bit()? as u8;

    Ok(())
}

/// Reads the side_info for all channels in a granule from a `BitStream`.
fn read_granule_side_info<B: ReadBitsLtr>(
    bs: &mut B,
    granule: &mut Granule,
    header: &FrameHeader,
) -> Result<()> {
    // Read the side_info for each channel in the granule.
    for channel in &mut granule.channels[..header.channel_mode.count()] {
        read_granule_channel_side_info(bs, channel, header)?;
    }
    Ok(())
}

/// Reads the side_info of a MPEG audio frame from a `BitStream` into `FrameData`.
pub(super) fn read_side_info<B: ReadBitsLtr>(
    bs: &mut B,
    header: &FrameHeader,
    frame_data: &mut FrameData,
) -> Result<usize> {
    // For MPEG version 1...
    if header.is_mpeg1() {
        // First 9 bits is main_data_begin.
        frame_data.main_data_begin = bs.read_bits_leq32(9)? as u16;

        // Next 3 (>1 channel) or 5 (1 channel) bits are private and should be ignored.
        match header.channel_mode {
            ChannelMode::Mono => bs.ignore_bits(5)?,
            _ => bs.ignore_bits(3)?,
        };

        // Next four (or 8, if more than one channel) are the SCFSI bits.
        for scfsi in &mut frame_data.scfsi[..header.n_channels()] {
            for band in scfsi.iter_mut() {
                *band = bs.read_bool()?;
            }
        }
    }
    // For MPEG version 2...
    else {
        // First 8 bits is main_data_begin.
        frame_data.main_data_begin = bs.read_bits_leq32(8)? as u16;

        // Next 1 (1 channel) or 2 (>1 channel) bits are private and should be ignored.
        match header.channel_mode {
            ChannelMode::Mono => bs.ignore_bits(1)?,
            _ => bs.ignore_bits(2)?,
        }
    }

    // Read the side_info for each granule.
    for granule in frame_data.granules_mut(header.version) {
        read_granule_side_info(bs, granule, header)?;
    }

    Ok(header.side_info_len())
}

/// Reads the scale factors for a single channel in a granule in a MPEG version 1 audio frame.
pub(super) fn read_scale_factors_mpeg1<B: ReadBitsLtr>(
    bs: &mut B,
    gr: usize,
    ch: usize,
    frame_data: &mut FrameData,
) -> Result<u32> {
    let mut bits_read = 0;

    let channel = &mut frame_data.granules[gr].channels[ch];

    // For MPEG1, scalefac_compress is a 4-bit index into a scale factor bit length lookup table.
    let (slen1, slen2) = SCALE_FACTOR_SLEN[channel.scalefac_compress as usize];

    // Short or Mixed windows...
    if let BlockType::Short { is_mixed } = channel.block_type {
        // If the block is mixed, there are three total scale factor partitions. The first is a long
        // scale factor partition for bands 0..8 (scalefacs[0..8] with each scale factor being slen1
        // bits long. Following this is a short scale factor partition covering bands 8..11 with a
        // window of 3 (scalefacs[8..17]) and each scale factoring being slen1 bits long.
        //
        // If a block is not mixed, then there are a total of two scale factor partitions. The first
        // is a short scale factor partition for bands 0..6 with a window length of 3
        // (scalefacs[0..18]) and each scale factor being slen1 bits long.
        let n_sfb = if is_mixed { 8 + 3 * 3 } else { 6 * 3 };

        if slen1 > 0 {
            for sfb in 0..n_sfb {
                channel.scalefacs[sfb] = bs.read_bits_leq32(slen1)? as u8;
            }
            bits_read += n_sfb * slen1 as usize;
        }

        // The final scale factor partition is always a a short scale factor window. It covers bands
        // 11..17 (scalefacs[17..35]) if the block is mixed, or bands 6..12 (scalefacs[18..36]) if
        // not. Each band has a window of 3 with each scale factor being slen2 bits long.
        if slen2 > 0 {
            for sfb in n_sfb..(n_sfb + (6 * 3)) {
                channel.scalefacs[sfb] = bs.read_bits_leq32(slen2)? as u8;
            }
            bits_read += 6 * 3 * slen2 as usize;
        }
    }
    // Normal (long, start, end) windows...
    else {
        // For normal windows there are 21 scale factor bands. These bands are divivided into four
        // band ranges. Scale factors in the first two band ranges: [0..6], [6..11], have scale
        // factors that are slen1 bits long, while the last two band ranges: [11..16], [16..21] have
        // scale factors that are slen2 bits long.
        const SCALE_FACTOR_BANDS: [(usize, usize); 4] = [(0, 6), (6, 11), (11, 16), (16, 21)];

        for (i, (start, end)) in SCALE_FACTOR_BANDS.iter().enumerate() {
            let slen = if i < 2 { slen1 } else { slen2 };

            // If this is the second granule, and the scale factor selection information for this
            // channel indicates that the scale factors should be copied from the first granule,
            // do so.
            if gr > 0 && frame_data.scfsi[ch][i] {
                let (granule0, granule1) = frame_data.granules.split_first_mut().unwrap();

                granule1[0].channels[ch].scalefacs[*start..*end]
                    .copy_from_slice(&granule0.channels[ch].scalefacs[*start..*end]);
            }
            // Otherwise, read the scale factors from the bitstream. Since scale factors are already
            // zeroed out by default, don't do anything if slen is 0.
            else if slen > 0 {
                for sfb in *start..*end {
                    frame_data.granules[gr].channels[ch].scalefacs[sfb] =
                        bs.read_bits_leq32(slen)? as u8;
                }
                bits_read += slen as usize * (end - start);
            }
        }
    }

    Ok(bits_read as u32)
}

/// Reads the scale factors for a single channel in a granule in a MPEG version 2 audio frame.
pub(super) fn read_scale_factors_mpeg2<B: ReadBitsLtr>(
    bs: &mut B,
    is_intensity_stereo: bool,
    channel: &mut GranuleChannel,
) -> Result<u32> {
    let mut bits_read = 0;

    let block_index = match channel.block_type {
        BlockType::Short { is_mixed: true } => 2,
        BlockType::Short { is_mixed: false } => 1,
        _ => 0,
    };

    let (slen_table, nsfb_table) = if is_intensity_stereo {
        // The actual value of scalefac_compress is a 9-bit unsigned integer (0..512) for MPEG2. A
        // left shift reduces it to an 8-bit value (0..256).
        let sfc = u32::from(channel.scalefac_compress) >> 1;

        match sfc {
            0..=179 => (
                [
                    (sfc / 36),     //
                    (sfc % 36) / 6, //
                    (sfc % 36) % 6, //
                    0,              //
                ],
                &SCALE_FACTOR_MPEG2_NSFB[0][block_index],
            ),
            180..=243 => (
                [
                    ((sfc - 180) % 64) >> 4, //
                    ((sfc - 180) % 16) >> 2, //
                    ((sfc - 180) % 4),       //
                    0,                       //
                ],
                &SCALE_FACTOR_MPEG2_NSFB[1][block_index],
            ),
            244..=255 => (
                [
                    (sfc - 244) / 3, //
                    (sfc - 244) % 3, //
                    0,               //
                    0,               //
                ],
                &SCALE_FACTOR_MPEG2_NSFB[2][block_index],
            ),
            _ => unreachable!(),
        }
    }
    else {
        // The actual value of scalefac_compress is a 9-bit unsigned integer (0..512) for MPEG2.
        let sfc = u32::from(channel.scalefac_compress);

        // Preflag is set only if scalefac_compress >= 500 and this is not the intensity stereo
        // channel. See ISO/IEC 13818-3 section 2.4.3.4.
        channel.preflag = sfc >= 500;

        match sfc {
            0..=399 => (
                [
                    (sfc >> 4) / 5,  //
                    (sfc >> 4) % 5,  //
                    (sfc % 16) >> 2, //
                    (sfc % 4),       //
                ],
                &SCALE_FACTOR_MPEG2_NSFB[3][block_index],
            ),
            400..=499 => (
                [
                    ((sfc - 400) >> 2) / 5, //
                    ((sfc - 400) >> 2) % 5, //
                    (sfc - 400) % 4,        //
                    0,                      //
                ],
                &SCALE_FACTOR_MPEG2_NSFB[4][block_index],
            ),
            500..=512 => (
                [
                    (sfc - 500) / 3, //
                    (sfc - 500) % 3, //
                    0,               //
                    0,               //
                ],
                &SCALE_FACTOR_MPEG2_NSFB[5][block_index],
            ),
            _ => unreachable!(),
        }
    };

    let mut start = 0;

    for (&slen, &n_sfb) in slen_table.iter().zip(nsfb_table.iter()) {
        // If slen > 0, read n_sfb scale factors with each scale factor being slen bits long. If
        // slen == 0, but n_sfb > 0, then the those scale factors should be set to 0. Since all
        // scalefacs are preinitialized to 0, this process may be skipped.
        if slen > 0 {
            for sfb in start..(start + n_sfb) {
                channel.scalefacs[sfb] = bs.read_bits_leq32(slen)? as u8;
            }
            bits_read += slen * n_sfb as u32;
        }

        start += n_sfb;
    }

    Ok(bits_read)
}
