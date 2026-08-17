// Copyright (c) 2020-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#[cfg(test)]
pub mod test {
  use crate::config::CpuFeatureLevel;
  use crate::dist::*;
  use crate::frame::*;
  use crate::partition::BlockSize;
  use crate::rdo::DistortionScale;
  use crate::tiling::Area;
  use crate::util::*;
  use rand::{rng, Rng};

  fn random_planes<T: Pixel>(bd: usize) -> (Plane<T>, Plane<T>) {
    let mut rng = rng();

    // Two planes with different strides
    let mut input_plane = Plane::new(640, 480, 0, 0, 128 + 8, 128 + 8);
    let mut rec_plane = Plane::new(640, 480, 0, 0, 2 * 128 + 8, 2 * 128 + 8);

    for rows in input_plane.as_region_mut().rows_iter_mut() {
      for c in rows {
        *c = T::cast_from(rng.random_range(0u16..(1 << bd)));
      }
    }

    for rows in rec_plane.as_region_mut().rows_iter_mut() {
      for c in rows {
        *c = T::cast_from(rng.random_range(0u16..(1 << bd)));
      }
    }

    (input_plane, rec_plane)
  }

  // Create planes with the max difference between the two values.
  fn max_diff_planes<T: Pixel>(bd: usize) -> (Plane<T>, Plane<T>) {
    // Two planes with different strides
    let mut input_plane = Plane::new(640, 480, 0, 0, 128 + 8, 128 + 8);
    let mut rec_plane = Plane::new(640, 480, 0, 0, 2 * 128 + 8, 2 * 128 + 8);

    for rows in input_plane.as_region_mut().rows_iter_mut() {
      for c in rows {
        *c = T::cast_from(0);
      }
    }

    for rows in rec_plane.as_region_mut().rows_iter_mut() {
      for c in rows {
        *c = T::cast_from((1 << bd) - 1);
      }
    }

    (input_plane, rec_plane)
  }

  /// Fill data for scaling of one (i.e. no scaling between blocks)
  fn scaling_one(scales: &mut [u32]) {
    for a in scales.iter_mut() {
      *a = DistortionScale::default().0;
    }
  }

  /// Fill data for scaling of one
  fn scaling_random(scales: &mut [u32]) {
    let mut rng = rng();
    for a in scales.iter_mut() {
      *a = rng.random_range(
        DistortionScale::from(0.5).0..DistortionScale::from(1.5).0,
      );
    }
  }

  /// Fill the max value for scaling
  /// TODO: Pair with max difference test
  fn scaling_large(scales: &mut [u32]) {
    for a in scales.iter_mut() {
      *a = DistortionScale::from(f64::MAX).0;
    }
  }

  #[test]
  fn weighted_sse_simd_no_scaling() {
    weighted_sse_simd_tester(8, scaling_one, random_planes::<u8>);
  }

  #[test]
  fn weighted_sse_simd_random() {
    weighted_sse_simd_tester(8, scaling_random, random_planes::<u8>);
  }

  #[test]
  fn weighted_sse_simd_large() {
    weighted_sse_simd_tester(8, scaling_large, max_diff_planes::<u8>);
  }

  #[test]
  fn weighted_sse_hbd_simd_no_scaling() {
    weighted_sse_simd_tester(12, scaling_one, random_planes::<u16>);
  }

  #[test]
  fn weighted_sse_hbd_simd_random() {
    weighted_sse_simd_tester(12, scaling_random, random_planes::<u16>);
  }

  #[test]
  fn weighted_sse_hbd_simd_large() {
    weighted_sse_simd_tester(12, scaling_large, max_diff_planes::<u16>);
  }

  fn weighted_sse_simd_tester<T: Pixel>(
    bd: usize, fill_scales: fn(scales: &mut [u32]),
    gen_planes: fn(bd: usize) -> (Plane<T>, Plane<T>),
  ) {
    use BlockSize::*;
    let blocks = vec![
      BLOCK_4X4,
      BLOCK_4X8,
      BLOCK_8X4,
      BLOCK_8X8,
      BLOCK_8X16,
      BLOCK_16X8,
      BLOCK_16X16,
      BLOCK_16X32,
      BLOCK_32X16,
      BLOCK_32X32,
      BLOCK_32X64,
      BLOCK_64X32,
      BLOCK_64X64,
      BLOCK_64X128,
      BLOCK_128X64,
      BLOCK_128X128,
      BLOCK_4X16,
      BLOCK_16X4,
      BLOCK_8X32,
      BLOCK_32X8,
      BLOCK_16X64,
      BLOCK_64X16,
    ];

    const SCALE_STRIDE: usize = 256;
    let mut scaling_storage = Aligned::new([0u32; 256 * SCALE_STRIDE]);
    let scaling = &mut scaling_storage.data;
    fill_scales(scaling);

    let (input_plane, rec_plane) = gen_planes(bd);

    for block in blocks {
      // Start at block width to test alignment.
      let area = Area::StartingAt { x: block.width() as isize, y: 40 };

      let input_region = input_plane.region(area);
      let rec_region = rec_plane.region(area);

      let rust = rust::get_weighted_sse(
        &input_region,
        &rec_region,
        scaling,
        SCALE_STRIDE,
        block.width(),
        block.height(),
        bd,
        CpuFeatureLevel::default(),
      );

      let simd = get_weighted_sse(
        &input_region,
        &rec_region,
        scaling,
        SCALE_STRIDE,
        block.width(),
        block.height(),
        bd,
        CpuFeatureLevel::default(),
      );

      assert!(
        simd == rust,
        "Weighted SSE {}: Assembly doesn't match reference code. {} (asm) != {} (ref)",
        block,
        simd,
        rust,
      );
    }
  }
}
