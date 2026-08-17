// Copyright (c) 2022-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::activity::apply_ssim_boost;
use crate::cpu_features::CpuFeatureLevel;
use crate::dist::*;
use crate::tiling::PlaneRegion;
use crate::util::Pixel;
use crate::util::PixelType;
use std::arch::x86_64::*;

type CdefDistKernelFn = unsafe extern fn(
  src: *const u8,
  src_stride: isize,
  dst: *const u8,
  dst_stride: isize,
  ret_ptr: *mut u32,
);

type CdefDistKernelHBDFn = unsafe fn(
  src: *const u16,
  src_stride: isize,
  dst: *const u16,
  dst_stride: isize,
) -> (u32, u32, u32);

extern {
  fn rav1e_cdef_dist_kernel_4x4_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
  fn rav1e_cdef_dist_kernel_4x8_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
  fn rav1e_cdef_dist_kernel_8x4_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
  fn rav1e_cdef_dist_kernel_8x8_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
}

/// # Panics
///
/// - If in `check_asm` mode, panics on mismatch between native and ASM results.
#[allow(clippy::let_and_return)]
pub fn cdef_dist_kernel<T: Pixel>(
  src: &PlaneRegion<'_, T>, dst: &PlaneRegion<'_, T>, w: usize, h: usize,
  bit_depth: usize, cpu: CpuFeatureLevel,
) -> u32 {
  debug_assert!(src.plane_cfg.xdec == 0);
  debug_assert!(src.plane_cfg.ydec == 0);
  debug_assert!(dst.plane_cfg.xdec == 0);
  debug_assert!(dst.plane_cfg.ydec == 0);

  // Limit kernel to 8x8
  debug_assert!(w <= 8);
  debug_assert!(h <= 8);

  let call_rust =
    || -> u32 { rust::cdef_dist_kernel(dst, src, w, h, bit_depth, cpu) };
  #[cfg(feature = "check_asm")]
  let ref_dist = call_rust();

  let (svar, dvar, sse) = match T::type_enum() {
    PixelType::U8 => {
      if let Some(func) =
        CDEF_DIST_KERNEL_FNS[cpu.as_index()][kernel_fn_index(w, h)]
      {
        let mut ret_buf = [0u32; 3];
        // SAFETY: Calls Assembly code.
        unsafe {
          func(
            src.data_ptr() as *const _,
            T::to_asm_stride(src.plane_cfg.stride),
            dst.data_ptr() as *const _,
            T::to_asm_stride(dst.plane_cfg.stride),
            ret_buf.as_mut_ptr(),
          )
        }

        (ret_buf[0], ret_buf[1], ret_buf[2])
      } else {
        return call_rust();
      }
    }
    PixelType::U16 => {
      if let Some(func) =
        CDEF_DIST_KERNEL_HBD_FNS[cpu.as_index()][kernel_fn_index(w, h)]
      {
        // SAFETY: Calls Assembly code.
        unsafe {
          func(
            src.data_ptr() as *const _,
            T::to_asm_stride(src.plane_cfg.stride),
            dst.data_ptr() as *const _,
            T::to_asm_stride(dst.plane_cfg.stride),
          )
        }
      } else {
        return call_rust();
      }
    }
  };

  let dist = apply_ssim_boost(sse, svar, dvar, bit_depth);
  #[cfg(feature = "check_asm")]
  assert_eq!(
    dist, ref_dist,
    "CDEF Distortion {}x{}: Assembly doesn't match reference code.",
    w, h
  );

  dist
}

/// Store functions in a 8x8 grid. Most will be empty.
const CDEF_DIST_KERNEL_FNS_LENGTH: usize = 8 * 8;

const fn kernel_fn_index(w: usize, h: usize) -> usize {
  ((w - 1) << 3) | (h - 1)
}

static CDEF_DIST_KERNEL_FNS_SSE2: [Option<CdefDistKernelFn>;
  CDEF_DIST_KERNEL_FNS_LENGTH] = {
  let mut out: [Option<CdefDistKernelFn>; CDEF_DIST_KERNEL_FNS_LENGTH] =
    [None; CDEF_DIST_KERNEL_FNS_LENGTH];

  out[kernel_fn_index(4, 4)] = Some(rav1e_cdef_dist_kernel_4x4_sse2);
  out[kernel_fn_index(4, 8)] = Some(rav1e_cdef_dist_kernel_4x8_sse2);
  out[kernel_fn_index(8, 4)] = Some(rav1e_cdef_dist_kernel_8x4_sse2);
  out[kernel_fn_index(8, 8)] = Some(rav1e_cdef_dist_kernel_8x8_sse2);

  out
};

cpu_function_lookup_table!(
  CDEF_DIST_KERNEL_FNS:
    [[Option<CdefDistKernelFn>; CDEF_DIST_KERNEL_FNS_LENGTH]],
  default: [None; CDEF_DIST_KERNEL_FNS_LENGTH],
  [SSE2]
);

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn mm256_sum_i32(ymm: __m256i) -> i32 {
  // We split the vector in half and then add the two halves and sum.
  let m1 = _mm256_extracti128_si256(ymm, 1);
  let m2 = _mm256_castsi256_si128(ymm);
  let m2 = _mm_add_epi32(m2, m1);
  let m1 = _mm_shuffle_epi32(m2, 0b11_10_11_10);
  let m2 = _mm_add_epi32(m2, m1);
  let m1 = _mm_shuffle_epi32(m2, 0b01_01_01_01);
  let m2 = _mm_add_epi32(m2, m1);
  _mm_cvtsi128_si32(m2)
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn rav1e_cdef_dist_kernel_8x8_hbd_avx2(
  src: *const u16, src_stride: isize, dst: *const u16, dst_stride: isize,
) -> (u32, u32, u32) {
  let src = src as *const u8;
  let dst = dst as *const u8;

  unsafe fn sum16(src: *const u8, src_stride: isize) -> u32 {
    let h = 8;
    let res = (0..h)
      .map(|row| _mm_load_si128(src.offset(row * src_stride) as *const _))
      .reduce(|a, b| _mm_add_epi16(a, b))
      .unwrap();

    let m32 = _mm256_cvtepi16_epi32(res);
    mm256_sum_i32(m32) as u32
  }
  unsafe fn mpadd32(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
  ) -> u32 {
    let h = 8;
    let res = (0..h / 2)
      .map(|row| {
        let s = _mm256_loadu2_m128i(
          src.offset(2 * row * src_stride) as *const _,
          src.offset((2 * row + 1) * src_stride) as *const _,
        );

        let d = _mm256_loadu2_m128i(
          dst.offset(2 * row * dst_stride) as *const _,
          dst.offset((2 * row + 1) * dst_stride) as *const _,
        );

        _mm256_madd_epi16(s, d)
      })
      .reduce(|a, b| _mm256_add_epi32(a, b))
      .unwrap();
    mm256_sum_i32(res) as u32
  }

  let sum_s = sum16(src, src_stride);
  let sum_d = sum16(dst, dst_stride);
  let sum_s2 = mpadd32(src, src_stride, src, src_stride);
  let sum_d2 = mpadd32(dst, dst_stride, dst, dst_stride);
  let sum_sd = mpadd32(src, src_stride, dst, dst_stride);

  // To get the distortion, compute sum of squared error and apply a weight
  // based on the variance of the two planes.
  let sse = sum_d2 + sum_s2 - 2 * sum_sd;

  // Convert to 64-bits to avoid overflow when squaring
  let sum_s = sum_s as u64;
  let sum_d = sum_d as u64;

  let svar = (sum_s2 as u64 - (sum_s * sum_s + 32) / 64) as u32;
  let dvar = (sum_d2 as u64 - (sum_d * sum_d + 32) / 64) as u32;

  (svar, dvar, sse)
}

static CDEF_DIST_KERNEL_HBD_FNS_AVX2: [Option<CdefDistKernelHBDFn>;
  CDEF_DIST_KERNEL_FNS_LENGTH] = {
  let mut out: [Option<CdefDistKernelHBDFn>; CDEF_DIST_KERNEL_FNS_LENGTH] =
    [None; CDEF_DIST_KERNEL_FNS_LENGTH];

  out[kernel_fn_index(8, 8)] = Some(rav1e_cdef_dist_kernel_8x8_hbd_avx2);

  out
};

cpu_function_lookup_table!(
  CDEF_DIST_KERNEL_HBD_FNS:
    [[Option<CdefDistKernelHBDFn>; CDEF_DIST_KERNEL_FNS_LENGTH]],
  default: [None; CDEF_DIST_KERNEL_FNS_LENGTH],
  [AVX2]
);
