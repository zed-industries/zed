// Copyright (c) 2022-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#[cfg(test)]
pub mod test {
  use crate::cpu_features::CpuFeatureLevel;
  use crate::dist::*;
  use crate::frame::*;
  use crate::tiling::Area;
  use crate::util::Pixel;
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

  /// Create planes with the max values for pixels.
  fn max_planes<T: Pixel>(bd: usize) -> (Plane<T>, Plane<T>) {
    // Two planes with different strides
    let mut input_plane = Plane::new(640, 480, 0, 0, 128 + 8, 128 + 8);
    let mut rec_plane = Plane::new(640, 480, 0, 0, 2 * 128 + 8, 2 * 128 + 8);

    for rows in input_plane.as_region_mut().rows_iter_mut() {
      for c in rows {
        *c = T::cast_from((1 << bd) - 1);
      }
    }

    for rows in rec_plane.as_region_mut().rows_iter_mut() {
      for c in rows {
        *c = T::cast_from((1 << bd) - 1);
      }
    }

    (input_plane, rec_plane)
  }

  /// Create planes with the max difference between the two values.
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

  #[test]
  fn cdef_dist_simd_random() {
    cdef_diff_tester(8, random_planes::<u8>);
  }

  #[test]
  fn cdef_dist_simd_random_hbd() {
    cdef_diff_tester(10, random_planes::<u16>);
    cdef_diff_tester(12, random_planes::<u16>);
  }

  #[test]
  fn cdef_dist_simd_large() {
    cdef_diff_tester(8, max_planes::<u8>);
  }

  #[test]
  fn cdef_dist_simd_large_hbd() {
    cdef_diff_tester(10, max_planes::<u16>);
    cdef_diff_tester(12, max_planes::<u16>);
  }

  #[test]
  fn cdef_dist_simd_large_diff() {
    cdef_diff_tester(8, max_diff_planes::<u8>);
  }

  #[test]
  fn cdef_dist_simd_large_diff_hbd() {
    cdef_diff_tester(10, max_diff_planes::<u16>);
    cdef_diff_tester(12, max_diff_planes::<u16>);
  }

  fn cdef_diff_tester<T: Pixel>(
    bd: usize, gen_planes: fn(bd: usize) -> (Plane<T>, Plane<T>),
  ) {
    let (src_plane, dst_plane) = gen_planes(bd);

    let mut fail = false;

    for w in 1..=8 {
      for h in 1..=8 {
        // Test alignment by choosing starting location based on width.
        let area = Area::StartingAt { x: if w <= 4 { 4 } else { 8 }, y: 40 };

        let src_region = src_plane.region(area);
        let dst_region = dst_plane.region(area);

        let rust = rust::cdef_dist_kernel(
          &src_region,
          &dst_region,
          w,
          h,
          bd,
          CpuFeatureLevel::default(),
        );

        let simd = cdef_dist_kernel(
          &src_region,
          &dst_region,
          w,
          h,
          bd,
          CpuFeatureLevel::default(),
        );

        if simd != rust {
          eprintln!(
            "CDEF Distortion {}x{}: Assembly doesn't match reference code \
          \t {} (asm) != {} (ref)",
            w, h, simd, rust
          );
          fail = true;
        }
      }

      if fail {
        panic!();
      }
    }
  }
}
