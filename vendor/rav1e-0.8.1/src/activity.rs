// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::frame::*;
use crate::rdo::DistortionScale;
use crate::tiling::*;
use crate::util::*;
use itertools::izip;

#[derive(Debug, Default, Clone)]
pub struct ActivityMask {
  variances: Box<[u32]>,
}

impl ActivityMask {
  #[profiling::function]
  pub fn from_plane<T: Pixel>(luma_plane: &Plane<T>) -> ActivityMask {
    let PlaneConfig { width, height, .. } = luma_plane.cfg;

    // Width and height are padded to 8×8 block size.
    let w_in_imp_b = width.align_power_of_two_and_shift(3);
    let h_in_imp_b = height.align_power_of_two_and_shift(3);

    let aligned_luma = Rect {
      x: 0_isize,
      y: 0_isize,
      width: w_in_imp_b << 3,
      height: h_in_imp_b << 3,
    };
    let luma = PlaneRegion::new(luma_plane, aligned_luma);

    let mut variances = Vec::with_capacity(w_in_imp_b * h_in_imp_b);

    for y in 0..h_in_imp_b {
      for x in 0..w_in_imp_b {
        let block_rect = Area::Rect {
          x: (x << 3) as isize,
          y: (y << 3) as isize,
          width: 8,
          height: 8,
        };

        let block = luma.subregion(block_rect);
        let variance = variance_8x8(&block);
        variances.push(variance);
      }
    }
    ActivityMask { variances: variances.into_boxed_slice() }
  }

  #[profiling::function]
  pub fn fill_scales(
    &self, bit_depth: usize, activity_scales: &mut Box<[DistortionScale]>,
  ) {
    for (dst, &src) in activity_scales.iter_mut().zip(self.variances.iter()) {
      *dst = ssim_boost(src, src, bit_depth);
    }
  }
}

// Adapted from the source variance calculation in `cdef_dist_wxh_8x8`.
#[inline(never)]
fn variance_8x8<T: Pixel>(src: &PlaneRegion<'_, T>) -> u32 {
  debug_assert!(src.plane_cfg.xdec == 0);
  debug_assert!(src.plane_cfg.ydec == 0);

  // Sum into columns to improve auto-vectorization
  let mut sum_s_cols: [u16; 8] = [0; 8];
  let mut sum_s2_cols: [u32; 8] = [0; 8];

  // Check upfront that 8 rows are available.
  let _row = &src[7];

  for j in 0..8 {
    let row = &src[j][0..8];
    for (sum_s, sum_s2, s) in izip!(&mut sum_s_cols, &mut sum_s2_cols, row) {
      // Don't convert directly to u32 to allow better vectorization
      let s: u16 = u16::cast_from(*s);
      *sum_s += s;

      // Convert to u32 to avoid overflows when multiplying
      let s: u32 = s as u32;
      *sum_s2 += s * s;
    }
  }

  // Sum together the sum of columns
  let sum_s = sum_s_cols.iter().copied().map(u64::from).sum::<u64>();
  let sum_s2 = sum_s2_cols.iter().copied().map(u64::from).sum::<u64>();

  // Use sums to calculate variance
  u32::try_from(sum_s2 - ((sum_s * sum_s + 32) >> 6)).unwrap_or(u32::MAX)
}

/// `rsqrt` result stored in fixed point w/ scaling such that:
///   `rsqrt = output.rsqrt_norm / (1 << output.shift)`
struct RsqrtOutput {
  norm: u16,
  shift: u8,
}

/// Fixed point `rsqrt` for `ssim_boost`
const fn ssim_boost_rsqrt(x: u64) -> RsqrtOutput {
  const INSHIFT: u8 = 16;
  const OUTSHIFT: u8 = 14;

  let k = (x.ilog2() >> 1) as i16;
  /*t is x in the range [0.25, 1) in QINSHIFT, or x*2^(-s).
  Shift by log2(x) - log2(0.25*(1 << INSHIFT)) to ensure 0.25 lower bound.*/
  let s: i16 = 2 * k - (INSHIFT as i16 - 2);
  let t: u16 = if s > 0 { x >> s } else { x << -s } as u16;

  /*We want to express od_rsqrt() in terms of od_rsqrt_norm(), which is
   defined as (2^OUTSHIFT)/sqrt(t*(2^-INSHIFT)) with t=x*(2^-s).
  This simplifies to 2^(OUTSHIFT+(INSHIFT/2)+(s/2))/sqrt(x), so the caller
   needs to shift right by OUTSHIFT + INSHIFT/2 + s/2.*/
  let rsqrt_shift: u8 = (OUTSHIFT as i16 + ((s + INSHIFT as i16) >> 1)) as u8;

  #[inline(always)]
  const fn mult16_16_q15(a: i32, b: i32) -> i32 {
    (a * b) >> 15
  }

  /* Reciprocal sqrt approximation where the input is in the range [0.25,1) in
  Q16 and the output is in the range (1.0, 2.0] in Q14). */

  /* Range of n is [-16384,32767] ([-0.5,1) in Q15). */
  let n: i32 = t as i32 - 32768;
  debug_assert!(n >= -16384);

  /* Get a rough guess for the root.
  The optimal minimax quadratic approximation (using relative error) is
   r = 1.437799046117536+n*(-0.823394375837328+n*0.4096419668459485).
  Coefficients here, and the final result r, are Q14. */
  let rsqrt: i32 = 23557 + mult16_16_q15(n, -13490 + mult16_16_q15(n, 6711));

  debug_assert!(16384 <= rsqrt && rsqrt < 32768);
  RsqrtOutput { norm: rsqrt as u16, shift: rsqrt_shift }
}

#[inline(always)]
pub fn ssim_boost(svar: u32, dvar: u32, bit_depth: usize) -> DistortionScale {
  DistortionScale(apply_ssim_boost(
    DistortionScale::default().0,
    svar,
    dvar,
    bit_depth,
  ))
}

/// Apply ssim boost to a given input
#[inline(always)]
pub const fn apply_ssim_boost(
  input: u32, svar: u32, dvar: u32, bit_depth: usize,
) -> u32 {
  let coeff_shift = bit_depth - 8;

  // Scale dvar and svar to lbd range to prevent overflows.
  let svar = (svar >> (2 * coeff_shift)) as u64;
  let dvar = (dvar >> (2 * coeff_shift)) as u64;

  // The constants are such that when source and destination variance are equal,
  // ssim_boost ~= (x/2)^(-1/3) where x = variance / scale and the scale is
  // (maximum variance / sample range) << (bit depth - 8).
  // C2 is the variance floor, equivalent to a flat block of mean valued samples
  // with a single maximum value sample.
  const C1: u64 = 3355;
  const C2: u64 = 16128;
  const C3: u64 = 12338;
  const RATIO_SHIFT: u8 = 14;
  const RATIO: u64 = (((C1 << (RATIO_SHIFT + 1)) / C3) + 1) >> 1;

  //          C1        (svar + dvar + C2)
  // input * ---- * --------------------------
  //          C3     sqrt(C1^2 + svar * dvar)
  let rsqrt = ssim_boost_rsqrt((C1 * C1) + svar * dvar);
  ((input as u64
    * (((RATIO * (svar + dvar + C2)) * rsqrt.norm as u64) >> RATIO_SHIFT))
    >> rsqrt.shift) as u32
}

#[cfg(test)]
mod ssim_boost_tests {
  use super::*;
  use interpolate_name::interpolate_test;
  use rand::Rng;

  /// Test to make sure extreme values of `ssim_boost` don't overflow.
  #[test]
  fn overflow_test() {
    // Test variance for 8x8 region with a bit depth of 12
    let max_pix_diff = (1 << 12) - 1;
    let max_pix_sse = max_pix_diff * max_pix_diff;
    let max_variance = max_pix_diff * 8 * 8 / 4;
    apply_ssim_boost(max_pix_sse * 8 * 8, max_variance, max_variance, 12);
  }

  /// Floating point reference version of `ssim_boost`
  fn reference_ssim_boost(svar: u32, dvar: u32, bit_depth: usize) -> f64 {
    let coeff_shift = bit_depth - 8;
    let var_scale = 1f64 / (1 << (2 * coeff_shift)) as f64;
    let svar = svar as f64 * var_scale;
    let dvar = dvar as f64 * var_scale;
    // These constants are from ssim boost and need to be updated if the
    //  constants in ssim boost change.
    const C1: f64 = 3355f64;
    const C2: f64 = 16128f64;
    const C3: f64 = 12338f64;
    const RATIO: f64 = C1 / C3;

    RATIO * (svar + dvar + C2) / f64::sqrt(C1.mul_add(C1, svar * dvar))
  }

  /// Test that `ssim_boost` has sufficient accuracy.
  #[test]
  fn accuracy_test() {
    let mut rng = rand::rng();

    let mut max_relative_error = 0f64;
    let bd = 12;

    // Test different log scale ranges for the variance.
    // Each scale is tested multiple times with randomized variances.
    for scale in 0..(bd + 3 * 2 - 2) {
      for _ in 0..40 {
        let svar = rng.random_range(0..(1 << scale));
        let dvar = rng.random_range(0..(1 << scale));

        let float = reference_ssim_boost(svar, dvar, 12);
        let fixed =
          apply_ssim_boost(1 << 23, svar, dvar, 12) as f64 / (1 << 23) as f64;

        // Compare the two versions
        max_relative_error =
          max_relative_error.max(f64::abs(1f64 - fixed / float));
      }
    }

    assert!(
      max_relative_error < 0.05,
      "SSIM boost error too high. Measured max relative error: {}.",
      max_relative_error
    );
  }

  #[interpolate_test(8, 8)]
  #[interpolate_test(10, 10)]
  #[interpolate_test(12, 12)]
  fn reciprocal_cube_root_test(bd: usize) {
    let mut max_relative_error = 0f64;

    let scale = ((1 << bd) - 1) << (6 - 2 + bd - 8);
    for svar in scale..(scale << 2) {
      let float = ((scale << 1) as f64 / svar as f64).cbrt();
      let fixed =
        apply_ssim_boost(1 << 23, svar, svar, bd) as f64 / (1 << 23) as f64;

      // Compare the two versions
      max_relative_error =
        max_relative_error.max(f64::abs(1f64 - fixed / float));
    }

    assert!(
      max_relative_error < 0.0273,
      "SSIM boost error too high. Measured max relative error: {}.",
      max_relative_error
    );
  }
}
