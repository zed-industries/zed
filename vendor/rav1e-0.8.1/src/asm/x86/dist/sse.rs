// Copyright (c) 2020-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cpu_features::CpuFeatureLevel;
use crate::dist::*;
use crate::encoder::IMPORTANCE_BLOCK_SIZE;
use crate::partition::BlockSize;
use crate::rdo::DistortionScale;
use crate::tiling::PlaneRegion;
use crate::util::*;

type WeightedSseFn = unsafe extern fn(
  src: *const u8,
  src_stride: isize,
  dst: *const u8,
  dst_stride: isize,
  scale: *const u32,
  scale_stride: isize,
) -> u64;

type WeightedSseHBDFn = unsafe extern fn(
  src: *const u16,
  src_stride: isize,
  dst: *const u16,
  dst_stride: isize,
  scale: *const u32,
  scale_stride: isize,
) -> u64;

macro_rules! declare_asm_sse_fn {
  ($($name: ident),+) => (
    $(
      extern { fn $name (
        src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize, scale: *const u32, scale_stride: isize
      ) -> u64; }
    )+
  )
}

macro_rules! declare_asm_hbd_sse_fn {
  ($($name: ident),+) => (
    $(
      extern { fn $name (
        src: *const u16, src_stride: isize, dst: *const u16, dst_stride: isize, scale: *const u32, scale_stride: isize
      ) -> u64; }
    )+
  )
}

declare_asm_sse_fn![
  // SSSE3
  rav1e_weighted_sse_4x4_ssse3,
  rav1e_weighted_sse_4x8_ssse3,
  rav1e_weighted_sse_4x16_ssse3,
  rav1e_weighted_sse_8x4_ssse3,
  rav1e_weighted_sse_8x8_ssse3,
  rav1e_weighted_sse_8x16_ssse3,
  rav1e_weighted_sse_8x32_ssse3,
  // AVX2
  rav1e_weighted_sse_16x4_avx2,
  rav1e_weighted_sse_16x8_avx2,
  rav1e_weighted_sse_16x16_avx2,
  rav1e_weighted_sse_16x32_avx2,
  rav1e_weighted_sse_16x64_avx2,
  rav1e_weighted_sse_32x8_avx2,
  rav1e_weighted_sse_32x16_avx2,
  rav1e_weighted_sse_32x32_avx2,
  rav1e_weighted_sse_32x64_avx2,
  rav1e_weighted_sse_64x16_avx2,
  rav1e_weighted_sse_64x32_avx2,
  rav1e_weighted_sse_64x64_avx2,
  rav1e_weighted_sse_64x128_avx2,
  rav1e_weighted_sse_128x64_avx2,
  rav1e_weighted_sse_128x128_avx2
];

declare_asm_hbd_sse_fn![
  // SSE2
  rav1e_weighted_sse_4x4_hbd_sse2
];

/// # Panics
///
/// - If in `check_asm` mode, panics on mismatch between native and ASM results.
#[inline(always)]
#[allow(clippy::let_and_return)]
pub fn get_weighted_sse<T: Pixel>(
  src: &PlaneRegion<'_, T>, dst: &PlaneRegion<'_, T>, scale: &[u32],
  scale_stride: usize, w: usize, h: usize, bit_depth: usize,
  cpu: CpuFeatureLevel,
) -> u64 {
  // Assembly breaks if imp block size changes.
  assert_eq!(IMPORTANCE_BLOCK_SIZE >> 1, 4);

  let bsize_opt = BlockSize::from_width_and_height_opt(w, h);

  let call_rust = || -> u64 {
    rust::get_weighted_sse(dst, src, scale, scale_stride, w, h, bit_depth, cpu)
  };

  #[cfg(feature = "check_asm")]
  let ref_dist = call_rust();

  #[inline]
  const fn size_of_element<T: Sized>(_: &[T]) -> usize {
    std::mem::size_of::<T>()
  }

  let den =
    DistortionScale::new(1, 1 << rust::GET_WEIGHTED_SSE_SHIFT).0 as u64;
  let dist = match (bsize_opt, T::type_enum()) {
    (Err(_), _) => call_rust(),
    (Ok(bsize), PixelType::U8) => {
      match SSE_FNS[cpu.as_index()][to_index(bsize)] {
        // SAFETY: Calls Assembly code.
        Some(func) => unsafe {
          ((func)(
            src.data_ptr() as *const _,
            T::to_asm_stride(src.plane_cfg.stride),
            dst.data_ptr() as *const _,
            T::to_asm_stride(dst.plane_cfg.stride),
            scale.as_ptr(),
            (scale_stride * size_of_element(scale)) as isize,
          ) + (den >> 1))
            / den
        },
        None => call_rust(),
      }
    }
    (Ok(bsize), PixelType::U16) => {
      match SSE_HBD_FNS[cpu.as_index()][to_index(bsize)] {
        // SAFETY: Calls Assembly code.
        Some(func) => unsafe {
          ((func)(
            src.data_ptr() as *const _,
            T::to_asm_stride(src.plane_cfg.stride),
            dst.data_ptr() as *const _,
            T::to_asm_stride(dst.plane_cfg.stride),
            scale.as_ptr(),
            (scale_stride * size_of_element(scale)) as isize,
          ) + (den >> 1))
            / den
        },
        None => call_rust(),
      }
    }
  };

  #[cfg(feature = "check_asm")]
  assert_eq!(
    dist, ref_dist,
    "Weighted SSE {:?}: Assembly doesn't match reference code.",
    bsize_opt
  );

  dist
}

static SSE_FNS_SSSE3: [Option<WeightedSseFn>; DIST_FNS_LENGTH] = {
  let mut out: [Option<WeightedSseFn>; DIST_FNS_LENGTH] =
    [None; DIST_FNS_LENGTH];

  use BlockSize::*;
  out[BLOCK_4X4 as usize] = Some(rav1e_weighted_sse_4x4_ssse3);
  out[BLOCK_4X8 as usize] = Some(rav1e_weighted_sse_4x8_ssse3);
  out[BLOCK_4X16 as usize] = Some(rav1e_weighted_sse_4x16_ssse3);
  out[BLOCK_8X4 as usize] = Some(rav1e_weighted_sse_8x4_ssse3);
  out[BLOCK_8X8 as usize] = Some(rav1e_weighted_sse_8x8_ssse3);
  out[BLOCK_8X16 as usize] = Some(rav1e_weighted_sse_8x16_ssse3);
  out[BLOCK_8X32 as usize] = Some(rav1e_weighted_sse_8x32_ssse3);

  out
};

static SSE_FNS_AVX2: [Option<WeightedSseFn>; DIST_FNS_LENGTH] = {
  let mut out: [Option<WeightedSseFn>; DIST_FNS_LENGTH] = SSE_FNS_SSSE3;

  use BlockSize::*;
  out[BLOCK_16X4 as usize] = Some(rav1e_weighted_sse_16x4_avx2);
  out[BLOCK_16X8 as usize] = Some(rav1e_weighted_sse_16x8_avx2);
  out[BLOCK_16X16 as usize] = Some(rav1e_weighted_sse_16x16_avx2);
  out[BLOCK_16X32 as usize] = Some(rav1e_weighted_sse_16x32_avx2);
  out[BLOCK_16X64 as usize] = Some(rav1e_weighted_sse_16x64_avx2);
  out[BLOCK_32X8 as usize] = Some(rav1e_weighted_sse_32x8_avx2);
  out[BLOCK_32X16 as usize] = Some(rav1e_weighted_sse_32x16_avx2);
  out[BLOCK_32X32 as usize] = Some(rav1e_weighted_sse_32x32_avx2);
  out[BLOCK_32X64 as usize] = Some(rav1e_weighted_sse_32x64_avx2);
  out[BLOCK_64X16 as usize] = Some(rav1e_weighted_sse_64x16_avx2);
  out[BLOCK_64X32 as usize] = Some(rav1e_weighted_sse_64x32_avx2);
  out[BLOCK_64X64 as usize] = Some(rav1e_weighted_sse_64x64_avx2);
  out[BLOCK_64X128 as usize] = Some(rav1e_weighted_sse_64x128_avx2);
  out[BLOCK_128X64 as usize] = Some(rav1e_weighted_sse_128x64_avx2);
  out[BLOCK_128X128 as usize] = Some(rav1e_weighted_sse_128x128_avx2);

  out
};

static SSE_HBD_FNS_SSE2: [Option<WeightedSseHBDFn>; DIST_FNS_LENGTH] = {
  let mut out: [Option<WeightedSseHBDFn>; DIST_FNS_LENGTH] =
    [None; DIST_FNS_LENGTH];

  use BlockSize::*;
  out[BLOCK_4X4 as usize] = Some(rav1e_weighted_sse_4x4_hbd_sse2);

  out
};

cpu_function_lookup_table!(
  SSE_FNS: [[Option<WeightedSseFn>; DIST_FNS_LENGTH]],
  default: [None; DIST_FNS_LENGTH],
  [SSSE3, AVX2]
);

cpu_function_lookup_table!(
  SSE_HBD_FNS: [[Option<WeightedSseHBDFn>; DIST_FNS_LENGTH]],
  default: [None; DIST_FNS_LENGTH],
  [SSE2]
);
