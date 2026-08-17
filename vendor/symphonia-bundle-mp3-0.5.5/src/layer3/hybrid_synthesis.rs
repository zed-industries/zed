// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Justification: Some loops are better expressed without a range loop.
#![allow(clippy::needless_range_loop)]

use crate::common::FrameHeader;

use super::{common::*, GranuleChannel};

use std::{convert::TryInto, f64};

use lazy_static::lazy_static;

lazy_static! {
    /// Hybrid synthesesis IMDCT window coefficients for: Long, Start, Short, and End block, in that
    /// order.
    ///
    /// For long blocks:
    ///
    /// ```text
    /// W[ 0..36] = sin(PI/36.0 * (i + 0.5))
    /// ```
    ///
    /// For start blocks:
    ///
    /// ```text
    /// W[ 0..18] = sin(PI/36.0 * (i + 0.5))
    /// W[18..24] = 1.0
    /// W[24..30] = sin(PI/12.0 * ((i - 18) - 0.5))
    /// W[30..36] = 0.0
    /// ```
    ///
    /// For short blocks (to be applied to each 12 sample window):
    ///
    /// ```text
    /// W[ 0..12] = sin(PI/12.0 * (i + 0.5))
    /// W[12..36] = 0.0
    /// ```
    ///
    /// For end blocks:
    ///
    /// ```text
    /// W[ 0..6 ] = 0.0
    /// W[ 6..12] = sin(PI/12.0 * ((i - 6) + 0.5))
    /// W[12..18] = 1.0
    /// W[18..36] = sin(PI/36.0 * (i + 0.5))
    /// ```
    static ref IMDCT_WINDOWS: [[f32; 36]; 4] = {
        const PI_36: f64 = f64::consts::PI / 36.0;
        const PI_12: f64 = f64::consts::PI / 12.0;

        let mut windows = [[0f32; 36]; 4];

        // Window for Long blocks.
        for i in 0..36 {
            windows[0][i] = (PI_36 * (i as f64 + 0.5)).sin() as f32;
        }

        // Window for Start blocks (indicies 30..36 implictly 0.0).
        for i in 0..18 {
            windows[1][i] = (PI_36 * (i as f64 + 0.5)).sin() as f32;
        }
        for i in 18..24 {
            windows[1][i] = 1.0;
        }
        for i in 24..30 {
            windows[1][i] = (PI_12 * ((i - 18) as f64 + 0.5)).sin() as f32;
        }

        // Window for Short blocks.
        for i in 0..12 {
            windows[2][i] = (PI_12 * (i as f64 + 0.5)).sin() as f32;
        }

        // Window for End blocks (indicies 0..6 implicitly 0.0).
        for i in 6..12 {
            windows[3][i] = (PI_12 * ((i - 6) as f64 + 0.5)).sin() as f32;
        }
        for i in 12..18 {
            windows[3][i] = 1.0;
        }
        for i in 18..36 {
            windows[3][i] = (PI_36 * (i as f64 + 0.5)).sin() as f32;
        }

        windows
   };
}

lazy_static! {
    /// Lookup table of cosine coefficients for half of a 12-point IMDCT.
    ///
    /// This table is derived from the general expression:
    ///
    /// ```text
    /// cos12[i][k] = cos(PI/24.0 * (2*i + 1 + N/2) * (2*k + 1))
    /// ```
    /// where:
    ///     `N=12`, `i=N/4..3N/4`, and `k=0..N/2`.
    static ref IMDCT_HALF_COS_12: [[f32; 6]; 6] = {
        const PI_24: f64 = f64::consts::PI / 24.0;

        let mut cos = [[0f32; 6]; 6];

        for (i, cos_i) in cos.iter_mut().enumerate() {
            for (k, cos_ik) in cos_i.iter_mut().enumerate() {
                // Only compute the middle half of the cosine lookup table (i offset by 3).
                let n = (2 * (i + 3) + (12 / 2) + 1) * (2 * k + 1);
                *cos_ik = (PI_24 * n as f64).cos() as f32;
            }
        }

        cos
    };
}

lazy_static! {
    /// Pair of lookup tables, CS and CA, for alias reduction.
    ///
    /// As per ISO/IEC 11172-3, CS and CA are calculated as follows:
    ///
    /// ```text
    /// cs[i] =  1.0 / sqrt(1.0 + c[i]^2)
    /// ca[i] = c[i] / sqrt(1.0 + c[i]^2)
    /// ```
    ///
    /// where:
    /// ```text
    /// c[i] = [ -0.6, -0.535, -0.33, -0.185, -0.095, -0.041, -0.0142, -0.0037 ]
    /// ```
    static ref ANTIALIAS_CS_CA: ([f32; 8], [f32; 8]) = {
        const C: [f64; 8] = [ -0.6, -0.535, -0.33, -0.185, -0.095, -0.041, -0.0142, -0.0037 ];

        let mut cs = [0f32; 8];
        let mut ca = [0f32; 8];

        for i in 0..8 {
            let sqrt = f64::sqrt(1.0 + (C[i] * C[i]));
            cs[i] = (1.0 / sqrt) as f32;
            ca[i] = (C[i] / sqrt) as f32;
        }

        (cs, ca)
    };
}

/// Reorder samples that are part of short blocks into sub-band order.
pub(super) fn reorder(header: &FrameHeader, channel: &mut GranuleChannel, buf: &mut [f32; 576]) {
    // Only short blocks are reordered.
    if let BlockType::Short { is_mixed } = channel.block_type {
        // Every short block is split into 3 equally sized windows as illustrated below (e.g. for
        // a short scale factor band with win_len=4):
        //
        //    <- Window #1 ->  <- Window #2 ->  <- Window #3 ->
        //   [ 0 | 1 | 2 | 3 ][ 4 | 5 | 6 | 7 ][ 8 | 9 | a | b ]
        //    <-----  3 * Short Scale Factor Band Width  ----->
        //
        // Reordering interleaves the samples of each window as follows:
        //
        //   [ 0 | 4 | 8 | 1 | 5 | 9 | 2 | 6 | a | 3 | 7 | b ]
        //    <----  3 * Short Scale Factor Band Width  ---->
        //
        // Basically, reordering interleaves the 3 windows the same way that 3 planar audio buffers
        // would be interleaved.
        debug_assert!(channel.rzero <= 576);

        // In mixed blocks, only the short bands can be re-ordered. Determine the applicable bands.
        let bands = if is_mixed {
            let switch = SFB_MIXED_SWITCH_POINT[header.sample_rate_idx];
            &SFB_MIXED_BANDS[header.sample_rate_idx][switch..]
        }
        else {
            &SFB_SHORT_BANDS[header.sample_rate_idx]
        };

        let mut reorder_buf = [0f32; 576];

        let start = bands[0];
        let mut i = start;

        for (((s0, s1), s2), s3) in
            bands.iter().zip(&bands[1..]).zip(&bands[2..]).zip(&bands[3..]).step_by(3)
        {
            // Do not reorder short blocks that begin after the rzero partition boundary since
            // they're zeroed.
            if *s0 >= channel.rzero {
                break;
            }

            // The three short sample windows.
            let win0 = &buf[*s0..*s1];
            let win1 = &buf[*s1..*s2];
            let win2 = &buf[*s2..*s3];

            // Interleave the three short sample windows.
            for ((w0, w1), w2) in win0.iter().zip(win1).zip(win2) {
                reorder_buf[i + 0] = *w0;
                reorder_buf[i + 1] = *w1;
                reorder_buf[i + 2] = *w2;
                i += 3;
            }
        }

        // Copy reordered samples from the reorder buffer to the actual sample buffer.
        buf[start..i].copy_from_slice(&reorder_buf[start..i]);

        // After reordering, the start of the rzero partition may no longer be valid. Update it.
        channel.rzero = channel.rzero.max(i);
    }
}

/// Applies the anti-aliasing filter to sub-bands that are not part of short blocks.
pub(super) fn antialias(channel: &mut GranuleChannel, samples: &mut [f32; 576]) {
    // The maximum number of sub-bands to anti-alias depends on block type.
    let sb_limit = match channel.block_type {
        // Short blocks are never anti-aliased.
        BlockType::Short { is_mixed: false } => return,
        // Mixed blocks have a long block span the first 36 samples (2 sub-bands). Therefore, only
        // anti-alias these two sub-bands.
        BlockType::Short { is_mixed: true } => 2,
        // All other block types require all 32 sub-bands to be anti-aliased.
        _ => 32,
    };

    // Amortize the lazy_static fetch over the entire anti-aliasing operation.
    let (cs, ca): &([f32; 8], [f32; 8]) = &ANTIALIAS_CS_CA;

    // The sub-band that intersects the start of the rzero partition. All sub-bands after this one
    // are zeroed and do-not need anti-aliasing.
    let sb_rzero = channel.rzero / 18;

    // The anti-aliasing filter must be applied up-to the last non-zero sub-band. After
    // anti-aliasing, the first zeroed sub-band may have non-zero values "smeared" into it.
    // Therefore, the rzero must be updated.
    channel.rzero = 18 * sb_limit.min(sb_rzero + 2).min(32);

    // Anti-aliasing is performed using 8 butterfly calculations at the boundaries of ADJACENT
    // sub-bands. For each calculation, there are two samples: lower and upper. For each iteration,
    // the lower sample index advances backwards from the boundary, while the upper sample index
    // advances forward from the boundary.
    //
    // For example, let B(li, ui) represent the butterfly calculation where li and ui are the
    // indicies of the lower and upper samples respectively. If j is the index of the first sample
    // of a sub-band, then the iterations are as follows:
    //
    // B(j-1,j), B(j-2,j+1), B(j-3,j+2), B(j-4,j+3), B(j-5,j+4), B(j-6,j+5), B(j-7,j+6), B(j-8,j+7)
    //
    // The butterfly calculation itself can be illustrated as follows:
    //
    //              * cs[i]
    //   l0 -------o------(-)------> l1
    //               \    /                  l1 = l0 * cs[i] - u0 * ca[i]
    //                \  / * ca[i]           u1 = u0 * cs[i] + l0 * ca[i]
    //                 \
    //               /  \  * ca[i]           where:
    //             /     \                       cs[i], ca[i] are constant values for iteration i,
    //   u0 ------o------(+)-------> u1          derived from table B.9 of ISO/IEC 11172-3.
    //             * cs[i]
    //
    // Note that all butterfly calculations only involve two samples, and all iterations are
    // independant of each other. This lends itself well for SIMD processing.
    for sb in (18..channel.rzero).step_by(18) {
        for i in 0..8 {
            let li = sb - 1 - i;
            let ui = sb + i;
            let lower = samples[li];
            let upper = samples[ui];
            samples[li] = lower * cs[i] - upper * ca[i];
            samples[ui] = upper * cs[i] + lower * ca[i];
        }
    }
}

/// Performs hybrid synthesis (IMDCT and windowing).
pub(super) fn hybrid_synthesis(
    channel: &GranuleChannel,
    overlap: &mut [[f32; 18]; 32],
    samples: &mut [f32; 576],
) {
    // The first sub-band after the rzero partition boundary is the sub-band limit. All sub-bands
    // past this are zeroed.
    let sb_limit = (channel.rzero + 17) / 18;

    // Determine the split point of long and short blocks in terms of a sub-band index.
    //
    // Short blocks process 0 sub-bands as long blocks, mixed blocks process the first 2 sub-bands
    // as long blocks, and all other block types (long, start, end) process all 32 sub-bands as long
    // blocks.
    let sb_split = match channel.block_type {
        BlockType::Short { is_mixed: false } => 0,
        BlockType::Short { is_mixed: true } => 2,
        _ => 32,
    };

    // If the split point is not 0, then some sub-bands need to be processed as long blocks using
    // the 36-point IMDCT.
    if sb_split > 0 {
        // Select the appropriate window given the block type.
        let window: &[f32; 36] = match channel.block_type {
            BlockType::Start => &IMDCT_WINDOWS[1],
            BlockType::End => &IMDCT_WINDOWS[3],
            _ => &IMDCT_WINDOWS[0],
        };

        let sb_long_end = sb_split.min(sb_limit);

        // For each of the sub-bands (18 samples each) in the long block...
        for sb in 0..sb_long_end {
            let start = 18 * sb;

            // Casting to a slice of a known-size lets the compiler elide bounds checks.
            let sub_band: &mut [f32; 18] = (&mut samples[start..(start + 18)]).try_into().unwrap();

            // Perform the 36-point on the entire sub-band.
            imdct36::imdct36(sub_band, window, &mut overlap[sb]);
        }
    }

    // If the split point is less-than 32, then some sub-bands need to be processed as short blocks
    // using the 12-point IMDCT on each of the three windows.
    if sb_split < 32 {
        // Select the short block window.
        let window: &[f32; 36] = &IMDCT_WINDOWS[2];

        let sb_short_begin = sb_split.min(sb_limit);

        // For each of the sub-bands (18 samples each) in the short block...
        for sb in sb_short_begin..sb_limit {
            let start = 18 * sb;

            // Casting to a slice of a known-size lets the compiler elide bounds checks.
            let sub_band: &mut [f32; 18] = (&mut samples[start..(start + 18)]).try_into().unwrap();

            // Perform the 12-point IMDCT on each of the 3 short windows within the sub-band (6
            // samples each).
            imdct12_win(sub_band, window, &mut overlap[sb]);
        }
    }

    // Every sub-band after the the sub-band limit are zeroed, however, the overlap for that
    // sub-band may be non-zero. Therefore, copy it over.
    for sb in sb_limit..32 {
        let start = 18 * sb;
        let sub_band: &mut [f32; 18] = (&mut samples[start..(start + 18)]).try_into().unwrap();

        sub_band.copy_from_slice(&overlap[sb]);
        overlap[sb].fill(0.0);
    }
}

/// Performs the 12-point IMDCT, and windowing for each of the 3 short windows of a short block, and
/// then overlap-adds the result.
fn imdct12_win(x: &mut [f32; 18], window: &[f32; 36], overlap: &mut [f32; 18]) {
    let cos12: &[[f32; 6]; 6] = &IMDCT_HALF_COS_12;

    let mut tmp = [0.0; 36];

    for w in 0..3 {
        for i in 0..3 {
            // Compute the 12-point IMDCT for each of the 3 short windows using a half-size IMDCT
            // followed by post-processing.
            //
            // In general, the IMDCT is defined as:
            //
            //        (N/2)-1
            // y[i] =   SUM   { x[k] * cos(PI/2N * (2i + 1 + N/2) * (2k + 1)) }
            //          k=0
            //
            // For N=12, the IMDCT becomes:
            //
            //         5
            // y[i] = SUM { x[k] * cos(PI/24 * (2i + 7) * (2k + 1)) }
            //        k=0
            //
            // The cosine twiddle factors are easily indexable by i and k, and are therefore
            // pre-computed and placed into a look-up table.
            //
            // Further, y[3..0] = -y[3..6], and y[12..9] = y[6..9] which reduces the amount of work
            // by half.
            //
            // Therefore, it is possible to split the half-size IMDCT computation into two halves.
            // In the calculations below, yl is the left-half output of the half-size IMDCT, and yr
            // is the right-half.

            let yl = (x[w] * cos12[i][0])
                + (x[3 * 1 + w] * cos12[i][1])
                + (x[3 * 2 + w] * cos12[i][2])
                + (x[3 * 3 + w] * cos12[i][3])
                + (x[3 * 4 + w] * cos12[i][4])
                + (x[3 * 5 + w] * cos12[i][5]);

            let yr = (x[w] * cos12[i + 3][0])
                + (x[3 * 1 + w] * cos12[i + 3][1])
                + (x[3 * 2 + w] * cos12[i + 3][2])
                + (x[3 * 3 + w] * cos12[i + 3][3])
                + (x[3 * 4 + w] * cos12[i + 3][4])
                + (x[3 * 5 + w] * cos12[i + 3][5]);

            // Each adjacent 12-point IMDCT windows are overlapped and added in the output, with the
            // first and last 6 samples of the output always being 0.
            //
            // Each sample from the 12-point IMDCT is multiplied by the appropriate window function
            // as specified in ISO/IEC 11172-3. The values of the window function are pre-computed
            // and given by window[0..12].
            //
            // Since there are 3 IMDCT windows (indexed by w), y[0..12] is computed 3 times.
            // For the purpose of the diagram below, we label these IMDCT windows as: y0[0..12],
            // y1[0..12], and y2[0..12], for IMDCT windows 0..3 respectively.
            //
            // Therefore, the overlap-and-add operation can be visualized as below:
            //
            // 0             6           12           18           24           30            36
            // +-------------+------------+------------+------------+------------+-------------+
            // |      0      |  y0[..6]   |  y0[..6]   |  y1[6..]   |  y2[6..]   |      0      |
            // |     (6)     |            |  + y1[6..] |  + y2[..6] |            |     (6)     |
            // +-------------+------------+------------+------------+------------+-------------+
            // .             .            .            .            .            .             .
            // .             +-------------------------+            .            .             .
            // .             |      IMDCT #1 (y0)      |            .            .             .
            // .             +-------------------------+            .            .             .
            // .             .            +-------------------------+            .             .
            // .             .            |      IMDCT #2 (y1)      |            .             .
            // .             .            +-------------------------+            .             .
            // .             .            .            +-------------------------+             .
            // .             .            .            |      IMDCT #3 (y2)      |             .
            // .             .            .            +-------------------------+             .
            // .             .            .            .            .            .             .
            //
            // Since the 12-point IMDCT was decomposed into a half-size IMDCT and post-processing
            // operations, and further split into left and right halves, each iteration of this loop
            // produces 4 output samples.

            tmp[6 + 6 * w + 3 - i - 1] += -yl * window[3 - i - 1];
            tmp[6 + 6 * w + i + 3] += yl * window[i + 3];
            tmp[6 + 6 * w + i + 6] += yr * window[i + 6];
            tmp[6 + 6 * w + 12 - i - 1] += yr * window[12 - i - 1];
        }
    }

    // Overlap-add.
    for i in 0..18 {
        x[i] = tmp[i] + overlap[i];
        overlap[i] = tmp[i + 18];
    }
}

/// Inverts odd samples in odd sub-bands.
pub fn frequency_inversion(samples: &mut [f32; 576]) {
    // There are 32 sub-bands spanning 576 samples:
    //
    //        0    18    36    54    72    90   108       558    576
    //        +-----+-----+-----+-----+-----+-----+ . . . . +------+
    // s[i] = | sb0 | sb1 | sb2 | sb3 | sb4 | sb5 | . . . . | sb31 |
    //        +-----+-----+-----+-----+-----+-----+ . . . . +------+
    //
    // The odd sub-bands are thusly:
    //
    //      sb1  -> s[ 18.. 36]
    //      sb3  -> s[ 54.. 72]
    //      sb5  -> s[ 90..108]
    //      ...
    //      sb31 -> s[558..576]
    //
    // Each odd sample in the aforementioned sub-bands must be negated.
    for i in (18..576).step_by(36) {
        // Sample negation is unrolled into a 2x4 + 1 (9) operation to improve vectorization.
        for j in (i..i + 16).step_by(8) {
            samples[j + 1] = -samples[j + 1];
            samples[j + 3] = -samples[j + 3];
            samples[j + 5] = -samples[j + 5];
            samples[j + 7] = -samples[j + 7];
        }
        samples[i + 18 - 1] = -samples[i + 18 - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::imdct12_win;
    use super::IMDCT_WINDOWS;
    use std::f64;

    fn imdct12_analytical(x: &[f32; 6]) -> [f32; 12] {
        const PI_24: f64 = f64::consts::PI / 24.0;

        let mut result = [0f32; 12];

        for i in 0..12 {
            let mut sum = 0.0;
            for k in 0..6 {
                sum +=
                    (x[k] as f64) * (PI_24 * ((2 * i + (12 / 2) + 1) * (2 * k + 1)) as f64).cos();
            }
            result[i] = sum as f32;
        }

        result
    }

    #[test]
    fn verify_imdct12_win() {
        const TEST_VECTOR: [f32; 18] = [
            0.0976, 0.9321, 0.6138, 0.0857, 0.0433, 0.4855, 0.2144, 0.8488, //
            0.6889, 0.2983, 0.1957, 0.7037, 0.0052, 0.0197, 0.3188, 0.5123, //
            0.2994, 0.7157,
        ];

        let window = &IMDCT_WINDOWS[2];

        let mut actual = TEST_VECTOR;
        let mut overlap = [0.0; 18];
        imdct12_win(&mut actual, window, &mut overlap);

        // The following block performs 3 analytical 12-point IMDCTs over the test vector, and then
        // windows and overlaps the results to generate the final result.
        let expected = {
            let mut expected = [0f32; 36];

            let mut x0 = [0f32; 6];
            let mut x1 = [0f32; 6];
            let mut x2 = [0f32; 6];

            for i in 0..6 {
                x0[i] = TEST_VECTOR[3 * i + 0];
                x1[i] = TEST_VECTOR[3 * i + 1];
                x2[i] = TEST_VECTOR[3 * i + 2];
            }

            let imdct0 = imdct12_analytical(&x0);
            let imdct1 = imdct12_analytical(&x1);
            let imdct2 = imdct12_analytical(&x2);

            for i in 0..12 {
                expected[6 + i] += imdct0[i] * window[i];
                expected[12 + i] += imdct1[i] * window[i];
                expected[18 + i] += imdct2[i] * window[i];
            }

            expected
        };

        for i in 0..18 {
            assert!((expected[i] - actual[i]).abs() < 0.00001);
            assert!((expected[i + 18] - overlap[i]).abs() < 0.00001);
        }
    }
}

mod imdct36 {
    /// Performs an Inverse Modified Discrete Cosine Transform (IMDCT) transforming 18
    /// frequency-domain input samples, into 36 time-domain output samples.
    ///
    /// This is a straight-forward implementation of the IMDCT using Szu-Wei Lee's algorithm
    /// published in article [1].
    ///
    /// [1] Szu-Wei Lee, "Improved algorithm for efficient computation of the forward and backward
    /// MDCT in MPEG audio coder", IEEE Transactions on Circuits and Systems II: Analog and Digital
    /// Signal Processing, vol. 48, no. 10, pp. 990-994, 2001.
    ///
    /// https://ieeexplore.ieee.org/document/974789
    pub fn imdct36(x: &mut [f32; 18], window: &[f32; 36], overlap: &mut [f32; 18]) {
        let mut dct = [0f32; 18];

        dct_iv(x, &mut dct);

        // Mapping of DCT-IV to IMDCT
        //
        //  0            9                       27           36
        //  +------------+------------------------+------------+
        //  | dct[9..18] | -dct[0..18].rev()      | -dct[0..9] |
        //  +------------+------------------------+------------+
        //
        // where dct[] is the DCT-IV of x.

        // First 9 IMDCT values are values 9..18 in the DCT-IV.
        for i in 0..9 {
            x[i] = overlap[i] + dct[9 + i] * window[i];
        }

        // Next 18 IMDCT values are negated and /reversed/ values 0..18 in the DCT-IV.
        for i in 9..18 {
            x[i] = overlap[i] - dct[27 - i - 1] * window[i];
        }

        for i in 18..27 {
            overlap[i - 18] = -dct[27 - i - 1] * window[i];
        }

        // Last 9 IMDCT values are negated values 0..9 in the DCT-IV.
        for i in 27..36 {
            overlap[i - 18] = -dct[i - 27] * window[i];
        }
    }

    /// Continutation of `imdct36`.
    ///
    /// Step 2: Mapping N/2-point DCT-IV to N/2-point SDCT-II.
    fn dct_iv(x: &[f32; 18], y: &mut [f32; 18]) {
        // Scale factors for input samples. Computed from (16).
        // 2 * cos(PI * (2*m + 1) / (2*36)
        const SCALE: [f32; 18] = [
            1.998_096_443_163_715_6, // m=0
            1.982_889_722_747_620_8, // m=1
            1.952_592_014_239_866_7, // m=2
            1.907_433_901_496_453_9, // m=3
            1.847_759_065_022_573_5, // m=4
            1.774_021_666_356_443_4, // m=5
            1.686_782_891_625_771_4, // m=6
            1.586_706_680_582_470_6, // m=7
            1.474_554_673_620_247_9, // m=8
            1.351_180_415_231_320_7, // m=9
            1.217_522_858_017_441_3, // m=10
            1.074_599_216_693_647_8, // m=11
            0.923_497_226_470_067_7, // m=12
            0.765_366_864_730_179_7, // m=13
            0.601_411_599_008_546_1, // m=14
            0.432_879_227_876_205_8, // m=15
            0.261_052_384_440_103_0, // m=16
            0.087_238_774_730_672_0, // m=17
        ];

        let samples = [
            SCALE[0] * x[0],
            SCALE[1] * x[1],
            SCALE[2] * x[2],
            SCALE[3] * x[3],
            SCALE[4] * x[4],
            SCALE[5] * x[5],
            SCALE[6] * x[6],
            SCALE[7] * x[7],
            SCALE[8] * x[8],
            SCALE[9] * x[9],
            SCALE[10] * x[10],
            SCALE[11] * x[11],
            SCALE[12] * x[12],
            SCALE[13] * x[13],
            SCALE[14] * x[14],
            SCALE[15] * x[15],
            SCALE[16] * x[16],
            SCALE[17] * x[17],
        ];

        sdct_ii_18(&samples, y);

        y[0] /= 2.0;
        for i in 1..17 {
            y[i] = (y[i] / 2.0) - y[i - 1];
        }
        y[17] = (y[17] / 2.0) - y[16];
    }

    /// Continutation of `imdct36`.
    ///
    /// Step 3: Decompose N/2-point SDCT-II into two N/4-point SDCT-IIs.
    fn sdct_ii_18(x: &[f32; 18], y: &mut [f32; 18]) {
        // Scale factors for odd input samples. Computed from (23).
        // 2 * cos(PI * (2*m + 1) / 36)
        const SCALE: [f32; 9] = [
            1.992_389_396_183_491_1,  // m=0
            1.931_851_652_578_136_6,  // m=1
            1.812_615_574_073_299_9,  // m=2
            1.638_304_088_577_983_6,  // m=3
            std::f32::consts::SQRT_2, // m=4
            1.147_152_872_702_092_3,  // m=5
            0.845_236_523_481_398_9,  // m=6
            0.517_638_090_205_041_9,  // m=7
            0.174_311_485_495_316_3,  // m=8
        ];

        let even = [
            x[0] + x[18 - 1],
            x[1] + x[18 - 2],
            x[2] + x[18 - 3],
            x[3] + x[18 - 4],
            x[4] + x[18 - 5],
            x[5] + x[18 - 6],
            x[6] + x[18 - 7],
            x[7] + x[18 - 8],
            x[8] + x[18 - 9],
        ];

        sdct_ii_9(&even, y);

        let odd = [
            SCALE[0] * (x[0] - x[18 - 1]),
            SCALE[1] * (x[1] - x[18 - 2]),
            SCALE[2] * (x[2] - x[18 - 3]),
            SCALE[3] * (x[3] - x[18 - 4]),
            SCALE[4] * (x[4] - x[18 - 5]),
            SCALE[5] * (x[5] - x[18 - 6]),
            SCALE[6] * (x[6] - x[18 - 7]),
            SCALE[7] * (x[7] - x[18 - 8]),
            SCALE[8] * (x[8] - x[18 - 9]),
        ];

        sdct_ii_9(&odd, &mut y[1..]);

        y[3] -= y[3 - 2];
        y[5] -= y[5 - 2];
        y[7] -= y[7 - 2];
        y[9] -= y[9 - 2];
        y[11] -= y[11 - 2];
        y[13] -= y[13 - 2];
        y[15] -= y[15 - 2];
        y[17] -= y[17 - 2];
    }

    /// Continutation of `imdct36`.
    ///
    /// Step 4: Computation of 9-point (N/4) SDCT-II.
    fn sdct_ii_9(x: &[f32; 9], y: &mut [f32]) {
        const D: [f32; 7] = [
            -1.732_050_807_568_877_2, // -sqrt(3.0)
            1.879_385_241_571_816_6,  // -2.0 * cos(8.0 * PI / 9.0)
            -0.347_296_355_333_860_8, // -2.0 * cos(4.0 * PI / 9.0)
            -1.532_088_886_237_956_0, // -2.0 * cos(2.0 * PI / 9.0)
            -0.684_040_286_651_337_8, // -2.0 * sin(8.0 * PI / 9.0)
            -1.969_615_506_024_416_0, // -2.0 * sin(4.0 * PI / 9.0)
            -1.285_575_219_373_078_5, // -2.0 * sin(2.0 * PI / 9.0)
        ];

        let a01 = x[3] + x[5];
        let a02 = x[3] - x[5];
        let a03 = x[6] + x[2];
        let a04 = x[6] - x[2];
        let a05 = x[1] + x[7];
        let a06 = x[1] - x[7];
        let a07 = x[8] + x[0];
        let a08 = x[8] - x[0];

        let a09 = x[4] + a05;
        let a10 = a01 + a03;
        let a11 = a10 + a07;
        let a12 = a03 - a07;
        let a13 = a01 - a07;
        let a14 = a01 - a03;
        let a15 = a02 - a04;
        let a16 = a15 + a08;
        let a17 = a04 + a08;
        let a18 = a02 - a08;
        let a19 = a02 + a04;
        let a20 = 2.0 * x[4] - a05;

        let m1 = D[0] * a06;
        let m2 = D[1] * a12;
        let m3 = D[2] * a13;
        let m4 = D[3] * a14;
        let m5 = D[0] * a16;
        let m6 = D[4] * a17;
        let m7 = D[5] * a18; // Note: the cited paper has an error, a1 should be a18.
        let m8 = D[6] * a19;

        let a21 = a20 + m2;
        let a22 = a20 - m2;
        let a23 = a20 + m3;
        let a24 = m1 + m6;
        let a25 = m1 - m6;
        let a26 = m1 + m7;

        y[0] = a09 + a11;
        y[2] = m8 - a26;
        y[4] = m4 - a21;
        y[6] = m5;
        y[8] = a22 - m3;
        y[10] = a25 - m7;
        y[12] = a11 - 2.0 * a09;
        y[14] = a24 + m8;
        y[16] = a23 + m4;
    }

    #[cfg(test)]
    mod tests {
        use super::imdct36;
        use std::f64;

        fn imdct36_analytical(x: &[f32; 18]) -> [f32; 36] {
            let mut result = [0f32; 36];

            const PI_72: f64 = f64::consts::PI / 72.0;

            for i in 0..36 {
                let mut sum = 0.0;
                for j in 0..18 {
                    sum +=
                        (x[j] as f64) * (PI_72 * (((2 * i) + 1 + 18) * ((2 * j) + 1)) as f64).cos();
                }
                result[i] = sum as f32;
            }
            result
        }

        #[test]
        fn verify_imdct36() {
            const TEST_VECTOR: [f32; 18] = [
                0.0976, 0.9321, 0.6138, 0.0857, 0.0433, 0.4855, 0.2144, 0.8488, //
                0.6889, 0.2983, 0.1957, 0.7037, 0.0052, 0.0197, 0.3188, 0.5123, //
                0.2994, 0.7157,
            ];

            const WINDOW: [f32; 36] = [1.0; 36];

            let mut actual = TEST_VECTOR;
            let mut overlap = [0.0; 18];
            imdct36(&mut actual, &WINDOW, &mut overlap);

            let expected = imdct36_analytical(&TEST_VECTOR);

            for i in 0..18 {
                assert!((expected[i] - actual[i]).abs() < 0.00001);
                assert!((expected[i + 18] - overlap[i]).abs() < 0.00001);
            }
        }
    }
}
