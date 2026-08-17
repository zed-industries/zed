// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};

use crate::common::{ChannelMode, FrameHeader, Mode};

use super::{common::*, Granule};

use std::cmp::max;
use std::{f32, f64};

use lazy_static::lazy_static;

/// The invalid intensity position for MPEG1 bitstreams.
const INTENSITY_INV_POS_MPEG1: u8 = 7;

/// The invalid intensity position for MPEG2 and MPEG2.5 bitstreams.
///
/// NOTE: Some decoders also consider 7 to be an invalid intensity position in MPEG2 and MPEG2.5.
/// However, this appears wrong. According to the standard, the /maximum/ value for the intensity
/// position is considered invalid. For MPEG1, since the intensity ratios are only specified for
/// is_pos < 7. For MPEG2 the ratios are specified for is_pos < 31. Therefore, it doesn't make sense
/// to consider is_pos == 7 invalid for MPEG2 or 2.5.
const INTENSITY_INV_POS_MPEG2: u8 = 31;

lazy_static! {
    /// (Left, right) channel coefficients for decoding intensity stereo in MPEG2 bitstreams.
    ///
    /// These coefficients are derived from section 2.4.3.2 of ISO/IEC 13818-3.
    ///
    /// As per the specification, for a given intensity position, is_pos (0 <= is_pos < 32), the
    /// channel coefficients, k_l and k_r, may be calculated as per the table below:
    ///
    /// ```text
    /// If...            | k_l                     | k_r
    /// -----------------+-------------------------+-------------------
    /// is_pos     == 0  | 1.0                     | 1.0
    /// is_pos & 1 == 1  | i0 ^ [(is_pos + 1) / 2] | 1.0
    /// is_pos & 1 == 0  | 1.0                     | i0 ^ (is_pos / 2)
    /// ```
    ///
    /// The value of i0 is dependant on the least significant bit of scalefac_compress.
    ///
    /// ```text
    /// scalefac_compress & 1 | i0
    /// ----------------------+---------------------
    /// 0                     | 1 / sqrt(sqrt(2.0))
    /// 1                     | 1 / sqrt(2.0)
    /// ```
    ///
    /// The first dimension of this table is indexed by scalefac_compress & 1 to select i0. The
    /// second dimension is indexed by is_pos to obtain the channel coefficients. Note that
    /// is_pos == 31 is considered an invalid position, but IS included in the table.
    static ref INTENSITY_STEREO_RATIOS_MPEG2: [[(f32, f32); 32]; 2] = {
        let is_scale: [f64; 2] = [
            1.0 / f64::sqrt(f64::consts::SQRT_2),
            f64::consts::FRAC_1_SQRT_2,
        ];

        let mut ratios = [[(0.0, 0.0); 32]; 2];

        for (i, is_pos) in (0..32).enumerate() {
            if is_pos & 1 != 0 {
                // Odd case.
                ratios[0][i] = (is_scale[0].powf(f64::from(is_pos + 1) / 2.0) as f32, 1.0);
                ratios[1][i] = (is_scale[1].powf(f64::from(is_pos + 1) / 2.0) as f32, 1.0);
            }
            else {
                // Even & zero case.
                ratios[0][i] = (1.0, is_scale[0].powf(f64::from(is_pos) / 2.0) as f32);
                ratios[1][i] = (1.0, is_scale[1].powf(f64::from(is_pos) / 2.0) as f32);
            }
        }

        ratios
    };
}

lazy_static! {
    /// (Left, right) channel coeffcients for decoding intensity stereo in MPEG1 bitstreams.
    ///
    /// These coefficients are derived from section 2.4.3.4.9.3 of ISO/IEC 11172-3.
    ///
    /// As per the specification, for a given intensity position, is_pos (0 <= is_pos < 7), a ratio,
    /// is_ratio, is calculated as follows:
    ///
    /// ```text
    /// is_ratio = tan(is_pos * PI/12)
    /// ```
    ///
    /// Then, the channel coefficients, k_l and k_r, are calculated as follows:
    ///
    /// ```text
    /// k_l = is_ratio / (1 + is_ratio)
    /// k_r =        1 / (1 + is_ratio)
    /// ```
    ///
    /// This table is indexed by is_pos. Note that is_pos == 7 is invalid and is NOT included in the
    /// table.
    static ref INTENSITY_STEREO_RATIOS_MPEG1: [(f32, f32); 7] = {
        const PI_12: f64 = f64::consts::PI / 12.0;

        let mut ratios = [(0.0, 0.0); 7];

        for (is_pos, ratio) in ratios.iter_mut().enumerate() {
            let is_ratio = (PI_12 * is_pos as f64).tan();
            *ratio = (
                (is_ratio / (1.0 + is_ratio)) as f32,
                (1.0 / (1.0 + is_ratio)) as f32
            );
        }

        ratios[6] = (1.0, 0.0);

        ratios
    };
}

/// Decorrelates mid and side channels into left and right channels.
///
/// In mid-side (MS) stereo, the left and right channels are encoded as average (mid) and
/// difference (side) components.
///
/// As per ISO/IEC 11172-3, to reconstruct the left and right channels, the following calculation
/// is performed:
///
/// ```text
///      l[i] = (m[i] + s[i]) / sqrt(2)
///      r[i] = (m[i] - s[i]) / sqrt(2)
/// ```
/// where:
///      l[i], and r[i] are the left and right channels, respectively.
///      m[i], and s[i] are the mid and side channels, respectively.
///
/// In the bitstream, m[i] is transmitted in channel 0, while s[i] in channel 1. After decoding,
/// the left channel replaces m[i] in channel 0, and the right channel replaces s[i] in channel
/// 1.
fn process_mid_side(mid: &mut [f32], side: &mut [f32]) {
    debug_assert!(mid.len() == side.len());

    for (m, s) in mid.iter_mut().zip(side) {
        let left = (*m + *s) * f32::consts::FRAC_1_SQRT_2;
        let right = (*m - *s) * f32::consts::FRAC_1_SQRT_2;
        *m = left;
        *s = right;
    }
}

/// Decodes channel 0 of the intensity stereo coded signal into left and right channels.
///
/// As per ISO/IEC 11172-3, the following calculation may be performed to decode the intensity
/// stereo coded signal into left and right channels.
///
/// ```text
///      l[i] = ch0[i] * k_l
///      r[i] = ch0[i] * l_r
/// ```
///
/// where:
///      l[i], and r[i] are the left and right channels, respectively.
///      ch0[i] is the intensity stereo coded signal found in channel 0.
///      k_l, and k_r are the left and right channel ratios, respectively.
fn process_intensity(
    intensity_pos: u8,
    intensity_table: &[(f32, f32)],
    intensity_max: u8,
    mid_side: bool,
    ch0: &mut [f32],
    ch1: &mut [f32],
) {
    if intensity_pos < intensity_max {
        let (ratio_l, ratio_r) = intensity_table[usize::from(intensity_pos)];

        for (l, r) in ch0.iter_mut().zip(ch1) {
            let is = *l;
            *l = ratio_l * is;
            *r = ratio_r * is;
        }
    }
    else if mid_side {
        process_mid_side(ch0, ch1);
    }
}

/// Determines if a band is zeroed.
#[inline(always)]
fn is_zero_band(band: &[f32]) -> bool {
    !band.iter().any(|&x| x != 0.0)
}

/// Decodes all intensity stereo coded bands within an entire long block and returns the intensity
/// bound.
fn process_intensity_long_block(
    header: &FrameHeader,
    granule: &Granule,
    mid_side: bool,
    max_bound: usize,
    ch0: &mut [f32; 576],
    ch1: &mut [f32; 576],
) -> usize {
    // As per ISO/IEC 11172-3 and ISO/IEC 13818-3, for long blocks that have intensity stereo
    // coding enabled, all bands starting after the last non-zero band in channel 1 may be
    // intensity stereo coded.
    //
    // The scale-factors in channel 1 for those respective bands determine the intensity position.

    // The rzero sample index is the index of last non-zero sample plus 1.
    let rzero = granule.channels[1].rzero;

    // Select the intensity stereo ratios table.
    let (is_table, is_inv_pos) = if header.is_mpeg1() {
        (&INTENSITY_STEREO_RATIOS_MPEG1[..], INTENSITY_INV_POS_MPEG1)
    }
    else {
        let is_scale = granule.channels[1].scalefac_compress & 1;
        (&INTENSITY_STEREO_RATIOS_MPEG2[usize::from(is_scale)][..], INTENSITY_INV_POS_MPEG2)
    };

    let bands = &SFB_LONG_BANDS[header.sample_rate_idx];

    // The intensity positions are stored in the right channel (channel 1) scalefactors. The
    // intensity position for band 21 is not coded and is copied from band 20.
    let mut is_pos = [0; 22];
    is_pos.copy_from_slice(&granule.channels[1].scalefacs[..22]);
    is_pos[21] = is_pos[20];

    // Create an iterator that yields a band start-end pair, and scale-factor.
    let bands_iter = bands.iter().zip(&bands[1..]).zip(is_pos.iter());

    let mut bound = max_bound;

    // Iterate over each band and decode the intensity stereo coding if the band is zero.
    for ((&start, &end), &is_pos) in bands_iter.rev() {
        // Bands starting above rzero are always 0, however bands below it are ambiguous.
        let is_zero_band = start >= rzero || is_zero_band(&ch1[start..end]);

        if is_zero_band {
            process_intensity(
                is_pos,
                is_table,
                is_inv_pos,
                mid_side,
                &mut ch0[start..end],
                &mut ch1[start..end],
            );
        }
        else {
            break;
        }

        // Update the intensity bound to the start of the band since it has now been processed.
        bound = start;
    }

    bound
}

/// Decodes all intensity stereo coded bands within an entire short block and returns the intensity
/// bound.
fn process_intensity_short_block(
    header: &FrameHeader,
    granule: &Granule,
    is_mixed: bool,
    mid_side: bool,
    max_bound: usize,
    ch0: &mut [f32; 576],
    ch1: &mut [f32; 576],
) -> usize {
    // For short, non-mixed, blocks, each band is composed of 3 windows (windows 0 thru 2). Windows
    // are interleaved in each band.
    //
    // +--------------+--------------+--------------+-------+
    // |     sfb0     |     sfb1     |     sfb2     |  ...  |
    // +--------------+--------------+--------------+-------+
    // | w0 | w1 | w2 | w0 | w1 | w2 | w0 | w1 | w2 |  ...  |
    // +--------------+--------------+--------------+-------+
    //
    // However, each window of the same index is logically contiguous as depicted below.
    //
    // +------+------+------+------+
    // | sfb0 | sfb1 | sfb2 | .... |
    // +------+------+------+------+
    // |  w0  |  w0  |  w0  | .... |
    // +-------------+------+------+
    // |  w1  |  w1  |  w1  | .... |
    // +-------------+------+------+
    // |  w2  |  w2  |  w2  | .... |
    // +------+------+------+------+
    //
    // Each logically contiguous window may have it's own intensity bound. For example, in the
    // example below, the intensity bound for window 0 is sfb0, for window 1 it's sfb2, and for
    // window 2 it's sfb1.
    //
    //      +------+------+------+------+
    //      | sfb0 | sfb1 | sfb2 | .... |
    //      +------+------+------+------+
    //  w0  | 0000 | 0000 | 0000 | 0... |
    //      +-------------+------+------+
    //  w1  | abcd | xyzw | 0000 | 0... |
    //      +-------------+------+------+
    //  w2  | xyz0 | 0000 | 0000 | 0... |
    //      +------+------+------+------+
    //
    // For short blocks that are mixed, the long bands at the start follow the same rules as long
    // blocks (see above). For example, for the block below, if sfb1 is the intensity bound, then
    // all samples from sfb1 onwards must be zero. If the intensity bound is not within the long
    // bands then the rules stated above are followed whereby each window has it's own intensity
    // bound.
    //
    // |> Long bands        |> Short bands (3 windows)
    // +------+------+------+--------+--------+------+
    // | sfb0 | sfb1 | .... | sfbN-2 | sfbN-1 | sfbN |
    // |------+------+------+--------+--------+------+
    // |      |      |      |   w0   |   w0   |  w0  |
    // |      |      |      +--------+--------+------+
    // |      |      | .... |   w1   |   w1   |  w1  |
    // |      |      |      +--------+--------+------+
    // |      |      |      |   w2   |   w2   |  w2  |
    // +------+------+------+--------+--------+------+
    //

    // First, if the short block is mixed, the get pair of short and long bands. Otherwise, if the
    // block is not mixed, get the short bands. In both cases, the index of the last scale-factor is
    // also returned.
    let (short_bands, long_bands, mut sfi) = if is_mixed {
        let bands = SFB_MIXED_BANDS[header.sample_rate_idx];
        let switch = SFB_MIXED_SWITCH_POINT[header.sample_rate_idx];
        // Variable number of short and long scalefactor bands based on the switch point.
        (&bands[switch..], Some(&bands[..switch + 1]), bands.len() - 1)
    }
    else {
        // 39 scalefactors from 13 scalefactor bands with 3 short windows per band.
        (&SFB_SHORT_BANDS[header.sample_rate_idx][..], None, 39)
    };

    // Select the intensity stereo ratios table based on the bitstream version.
    let (is_table, is_inv_pos) = if header.is_mpeg1() {
        (&INTENSITY_STEREO_RATIOS_MPEG1[..], INTENSITY_INV_POS_MPEG1)
    }
    else {
        let is_scale = granule.channels[1].scalefac_compress & 1;
        (&INTENSITY_STEREO_RATIOS_MPEG2[usize::from(is_scale)][..], INTENSITY_INV_POS_MPEG2)
    };

    // The intensity position for the final band (last three short windows) is not coded and is
    // copied from the previous band.
    let mut is_pos = [0; 39];
    is_pos[..36].copy_from_slice(&granule.channels[1].scalefacs[..36]);
    is_pos[36..].copy_from_slice(&granule.channels[1].scalefacs[33..36]);

    let mut window_is_zero = [true; 3];

    let mut bound = max_bound;
    let mut found_bound = false;

    // Process the short bands.
    for (((&s0, &s1), &s2), &s3) in short_bands
        .iter()
        .zip(&short_bands[1..])
        .zip(&short_bands[2..])
        .zip(&short_bands[3..])
        .step_by(3)
        .rev()
    {
        // For each short band, the following logic is repeated for each of the three windows.
        //
        // First, if the corresponding window in the previous band was zeroed, check if the
        // window in this band is also zeroed. Note that if the window is non-zero, this statement
        // short-circuits and avoids the costly zero-check.
        window_is_zero[2] = window_is_zero[2] && is_zero_band(&ch1[s2..s3]);

        // If the window is zeroed, process it with intensity stereo.
        if window_is_zero[2] {
            process_intensity(
                is_pos[sfi - 1],
                is_table,
                is_inv_pos,
                mid_side,
                &mut ch0[s2..s3],
                &mut ch1[s2..s3],
            );
        }
        else if mid_side {
            // If the window is non-zeroed, process it with mid-side stereo.
            process_mid_side(&mut ch0[s2..s3], &mut ch1[s2..s3]);
        }

        // Decrement the scalefactor (intensity position) index to advance to the next window.
        sfi -= 1;

        // Repeat the same process for the second window.
        window_is_zero[1] = window_is_zero[1] && is_zero_band(&ch1[s1..s2]);

        if window_is_zero[1] {
            process_intensity(
                is_pos[sfi - 1],
                is_table,
                is_inv_pos,
                mid_side,
                &mut ch0[s1..s2],
                &mut ch1[s1..s2],
            );
        }
        else if mid_side {
            process_mid_side(&mut ch0[s1..s2], &mut ch1[s1..s2]);
        }

        sfi -= 1;

        // Repeat the same process for the third window.
        window_is_zero[0] = window_is_zero[0] && is_zero_band(&ch1[s0..s1]);

        if window_is_zero[0] {
            process_intensity(
                is_pos[sfi - 1],
                is_table,
                is_inv_pos,
                mid_side,
                &mut ch0[s0..s1],
                &mut ch1[s0..s1],
            );
        }
        else if mid_side {
            process_mid_side(&mut ch0[s0..s1], &mut ch1[s0..s1]);
        }

        sfi -= 1;

        // Update the intensity bound to the start of the first window since all three windows have
        // now been processed by either intensity or mid-side stereo. Note that this is the "final"
        // intensity bound of all the windows in the short bands. Individual windows may have
        // reached their intensity bound earlier. Those windows are processed with mid-side stereo.
        bound = s0;

        // Determine if all windows non-zero.
        found_bound = !window_is_zero[0] && !window_is_zero[1] && !window_is_zero[2];

        // If all windows are non-zero then the all the remaining bands should be processed with
        // mid-side stereo. Break out early in this case.
        if found_bound {
            break;
        }
    }

    // If the final intensity bound was not found within the short bands, then it may be found
    // within the long bands if the short block is mixed.
    if !found_bound {
        // If the short block is mixed, the long bands will not be None.
        if let Some(long_bands) = long_bands {
            // Process the long bands exactly as if it were a long block.
            for (&start, &end) in long_bands.iter().zip(&long_bands[1..]).rev() {
                let is_zero_band = is_zero_band(&ch1[start..end]);

                if is_zero_band {
                    process_intensity(
                        is_pos[sfi - 1],
                        is_table,
                        is_inv_pos,
                        mid_side,
                        &mut ch0[start..end],
                        &mut ch1[start..end],
                    );
                }
                else {
                    break;
                }

                sfi -= 1;

                bound = start;
            }
        }
    }

    // Return the intensity bound.
    bound
}

/// Perform joint stereo decoding on the channel pair.
pub(super) fn stereo(
    header: &FrameHeader,
    granule: &mut Granule,
    ch: &mut [[f32; 576]; 2],
) -> Result<()> {
    // Determine whether mid-side, and/or intensity stereo coding is used.
    let (mid_side, intensity) = match header.channel_mode {
        ChannelMode::JointStereo(Mode::Layer3 { mid_side, intensity }) => (mid_side, intensity),
        ChannelMode::JointStereo(Mode::Intensity { .. }) => {
            // This function only supports decoding Layer 3 stereo encodings, it is a fundamental
            // error in the decoder logic if layer 1 or 2 stereo encodings are being decoded with
            // this function.
            panic!("invalid mode extension for layer 3 stereo decoding")
        }
        _ => return Ok(()),
    };

    // The block types must be the same.
    if granule.channels[0].block_type != granule.channels[1].block_type {
        return decode_error("mpa: stereo channel pair block_type mismatch");
    }

    // Split the sample buffer into two channels.
    let (ch0, ch1) = {
        let (ch0, ch1) = ch.split_first_mut().unwrap();
        (ch0, &mut ch1[0])
    };

    // Joint stereo processing as specified in layer 3 is a combination of mid-side, and intensity
    // encoding schemes. Each scale-factor band may use either mid-side, intensity, or no stereo
    // encoding. The type of encoding used for each scale-factor band is determined by the MPEG
    // bitstream version, the mode extension, the block type, and the content of the scale-factor
    // bands.
    let end = max(granule.channels[0].rzero, granule.channels[1].rzero);

    // Decode intensity stereo coded bands if it is enabled and get the intensity bound.
    let is_bound = if intensity {
        // Decode intensity stereo coded bands based on bitstream version and block type.
        match granule.channels[1].block_type {
            BlockType::Short { is_mixed } => {
                process_intensity_short_block(header, granule, is_mixed, mid_side, end, ch0, ch1)
            }
            _ => process_intensity_long_block(header, granule, mid_side, end, ch0, ch1),
        }
    }
    // If intensity stereo coding is not enabled, then all samples are processed with mid-side
    // stereo decoding. In other words, there are no samples encoded with intensity stereo and
    // therefore the intensity bound is equal to the end of the non-zero portion of the samples.
    else {
        end
    };

    // If mid-side stereo coding is enabled, all samples up to the intensity bound should be
    // decoded as mid-side stereo.
    if mid_side && is_bound > 0 {
        process_mid_side(&mut ch0[0..is_bound], &mut ch1[0..is_bound]);
    }

    // With joint stereo encoding, there is usually a mismatch between the number of samples
    // initially read from the bitstream for each channel. This count is stored as the rzero sample
    // index. However, after joint stereo decoding, both channels will have the same number of
    // samples. Update rzero for both channels with the actual number of samples.
    if intensity || mid_side {
        granule.channels[0].rzero = end;
        granule.channels[1].rzero = end;
    }

    Ok(())
}
