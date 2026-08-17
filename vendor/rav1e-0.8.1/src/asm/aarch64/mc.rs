// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cpu_features::CpuFeatureLevel;
use crate::frame::*;
use crate::mc::FilterMode::*;
use crate::mc::*;
use crate::tiling::*;
use crate::util::*;

type PutFn = unsafe extern fn(
  dst: *mut u8,
  dst_stride: isize,
  src: *const u8,
  src_stride: isize,
  width: i32,
  height: i32,
  col_frac: i32,
  row_frac: i32,
);

type PutHBDFn = unsafe extern fn(
  dst: *mut u16,
  dst_stride: isize,
  src: *const u16,
  src_stride: isize,
  width: i32,
  height: i32,
  col_frac: i32,
  row_frac: i32,
  bitdepth_max: i32,
);

type PrepFn = unsafe extern fn(
  tmp: *mut i16,
  src: *const u8,
  src_stride: isize,
  width: i32,
  height: i32,
  col_frac: i32,
  row_frac: i32,
);

type PrepHBDFn = unsafe extern fn(
  tmp: *mut i16,
  src: *const u16,
  src_stride: isize,
  width: i32,
  height: i32,
  col_frac: i32,
  row_frac: i32,
  bitdepth_max: i32,
);

type AvgFn = unsafe extern fn(
  dst: *mut u8,
  dst_stride: isize,
  tmp1: *const i16,
  tmp2: *const i16,
  width: i32,
  height: i32,
);

type AvgHBDFn = unsafe extern fn(
  dst: *mut u16,
  dst_stride: isize,
  tmp1: *const i16,
  tmp2: *const i16,
  width: i32,
  height: i32,
  bitdepth_max: i32,
);

// gets an index that can be mapped to a function for a pair of filter modes
#[inline]
const fn get_2d_mode_idx(mode_x: FilterMode, mode_y: FilterMode) -> usize {
  (mode_x as usize + 4 * (mode_y as usize)) & 15
}

#[inline(always)]
pub fn put_8tap<T: Pixel>(
  dst: &mut PlaneRegionMut<'_, T>, src: PlaneSlice<'_, T>, width: usize,
  height: usize, col_frac: i32, row_frac: i32, mode_x: FilterMode,
  mode_y: FilterMode, bit_depth: usize, cpu: CpuFeatureLevel,
) {
  let call_rust = |dst: &mut PlaneRegionMut<'_, T>| {
    rust::put_8tap(
      dst, src, width, height, col_frac, row_frac, mode_x, mode_y, bit_depth,
      cpu,
    );
  };
  #[cfg(feature = "check_asm")]
  let ref_dst = {
    let mut copy = dst.scratch_copy();
    call_rust(&mut copy.as_region_mut());
    copy
  };
  unsafe {
    // SAFETY: The assembly only supports even heights and valid uncropped
    //         widths
    assert_eq!(height & 1, 0);
    assert!(width.is_power_of_two() && (2..=128).contains(&width));

    // SAFETY: Check bounds of dst
    assert!(dst.rect().width >= width && dst.rect().height >= height);

    // SAFETY: Check bounds of src
    assert!(src.accessible(width + 4, height + 4));
    assert!(src.accessible_neg(3, 3));

    match T::type_enum() {
      PixelType::U8 => {
        match PUT_FNS[cpu.as_index()][get_2d_mode_idx(mode_x, mode_y)] {
          Some(func) => (func)(
            dst.data_ptr_mut() as *mut _,
            T::to_asm_stride(dst.plane_cfg.stride),
            src.as_ptr() as *const _,
            T::to_asm_stride(src.plane.cfg.stride),
            width as i32,
            height as i32,
            col_frac,
            row_frac,
          ),
          None => call_rust(dst),
        }
      }
      PixelType::U16 if bit_depth > 8 => {
        match PUT_HBD_FNS[cpu.as_index()][get_2d_mode_idx(mode_x, mode_y)] {
          Some(func) => (func)(
            dst.data_ptr_mut() as *mut _,
            T::to_asm_stride(dst.plane_cfg.stride),
            src.as_ptr() as *const _,
            T::to_asm_stride(src.plane.cfg.stride),
            width as i32,
            height as i32,
            col_frac,
            row_frac,
            (1 << bit_depth) - 1,
          ),
          None => call_rust(dst),
        }
      }
      _ => call_rust(dst),
    }
  }
  #[cfg(feature = "check_asm")]
  {
    for (dst_row, ref_row) in
      dst.rows_iter().zip(ref_dst.as_region().rows_iter())
    {
      for (dst, reference) in dst_row.iter().zip(ref_row) {
        assert_eq!(*dst, *reference);
      }
    }
  }
}

#[inline(always)]
pub fn prep_8tap<T: Pixel>(
  tmp: &mut [i16], src: PlaneSlice<'_, T>, width: usize, height: usize,
  col_frac: i32, row_frac: i32, mode_x: FilterMode, mode_y: FilterMode,
  bit_depth: usize, cpu: CpuFeatureLevel,
) {
  // The assembly only supports even heights and valid uncropped widths
  assert_eq!(height & 1, 0);
  assert!(width.is_power_of_two() && (2 <= width || width <= 128));

  let call_rust = |tmp: &mut [i16]| {
    rust::prep_8tap(
      tmp, src, width, height, col_frac, row_frac, mode_x, mode_y, bit_depth,
      cpu,
    );
  };
  #[cfg(feature = "check_asm")]
  let ref_tmp = {
    let mut copy = vec![0; width * height];
    copy[..].copy_from_slice(&tmp[..width * height]);
    call_rust(&mut copy);
    copy
  };
  unsafe {
    // SAFETY: The assembly only supports even heights and valid uncropped
    //         widths
    assert_eq!(height & 1, 0);
    assert!(width.is_power_of_two() && (2..=128).contains(&width));

    // SAFETY: Check length of tmp
    assert!(tmp.len() >= width * height);

    // SAFETY: Check bounds of src
    assert!(src.accessible(width + 4, height + 4));
    assert!(src.accessible_neg(3, 3));
    match T::type_enum() {
      PixelType::U8 => {
        match PREP_FNS[cpu.as_index()][get_2d_mode_idx(mode_x, mode_y)] {
          Some(func) => (func)(
            tmp.as_mut_ptr(),
            src.as_ptr() as *const _,
            T::to_asm_stride(src.plane.cfg.stride),
            width as i32,
            height as i32,
            col_frac,
            row_frac,
          ),
          None => call_rust(tmp),
        }
      }
      PixelType::U16 if bit_depth > 8 => {
        match PREP_HBD_FNS[cpu.as_index()][get_2d_mode_idx(mode_x, mode_y)] {
          Some(func) => (func)(
            tmp.as_mut_ptr() as *mut _,
            src.as_ptr() as *const _,
            T::to_asm_stride(src.plane.cfg.stride),
            width as i32,
            height as i32,
            col_frac,
            row_frac,
            (1 << bit_depth) - 1,
          ),
          None => call_rust(tmp),
        }
      }
      _ => call_rust(tmp),
    }
  }
  #[cfg(feature = "check_asm")]
  {
    assert_eq!(&tmp[..width * height], &ref_tmp[..]);
  }
}

pub fn mc_avg<T: Pixel>(
  dst: &mut PlaneRegionMut<'_, T>, tmp1: &[i16], tmp2: &[i16], width: usize,
  height: usize, bit_depth: usize, cpu: CpuFeatureLevel,
) {
  // The assembly only supports even heights and valid uncropped widths
  assert_eq!(height & 1, 0);
  assert!(width.is_power_of_two() && (2 <= width || width <= 128));

  let call_rust = |dst: &mut PlaneRegionMut<'_, T>| {
    rust::mc_avg(dst, tmp1, tmp2, width, height, bit_depth, cpu);
  };
  #[cfg(feature = "check_asm")]
  let ref_dst = {
    let mut copy = dst.scratch_copy();
    call_rust(&mut copy.as_region_mut());
    copy
  };
  unsafe {
    // SAFETY: The assembly only supports even heights and valid uncropped
    //         widths
    assert_eq!(height & 1, 0);
    assert!(width.is_power_of_two() && (2..=128).contains(&width));

    // SAFETY: Check bounds of dst
    assert!(dst.rect().width >= width && dst.rect().height >= height);

    // SAFETY: Check length of tmp1 & tmp2
    assert!(tmp1.len() >= width * height);
    assert!(tmp2.len() >= width * height);
    match T::type_enum() {
      PixelType::U8 => match AVG_FNS[cpu.as_index()] {
        Some(func) => (func)(
          dst.data_ptr_mut() as *mut _,
          T::to_asm_stride(dst.plane_cfg.stride),
          tmp1.as_ptr(),
          tmp2.as_ptr(),
          width as i32,
          height as i32,
        ),
        None => call_rust(dst),
      },
      PixelType::U16 if bit_depth > 8 => match AVG_HBD_FNS[cpu.as_index()] {
        Some(func) => (func)(
          dst.data_ptr_mut() as *mut _,
          T::to_asm_stride(dst.plane_cfg.stride),
          tmp1.as_ptr(),
          tmp2.as_ptr(),
          width as i32,
          height as i32,
          (1 << bit_depth) - 1,
        ),
        None => call_rust(dst),
      },
      _ => call_rust(dst),
    }
  }
  #[cfg(feature = "check_asm")]
  {
    for (dst_row, ref_row) in
      dst.rows_iter().zip(ref_dst.as_region().rows_iter())
    {
      for (dst, reference) in dst_row.iter().zip(ref_row) {
        assert_eq!(*dst, *reference);
      }
    }
  }
}

macro_rules! decl_mc_fns {
  ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
    extern {
      $(
        fn $func_name(
          dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
          w: i32, h: i32, mx: i32, my: i32,
        );
      )*
    }

    static PUT_FNS_NEON: [Option<PutFn>; 16] = {
      let mut out: [Option<PutFn>; 16] = [None; 16];
      $(
        out[get_2d_mode_idx($mode_x, $mode_y)] = Some($func_name);
      )*
      out
    };
  }
}

decl_mc_fns!(
  (REGULAR, REGULAR, rav1e_put_8tap_regular_8bpc_neon),
  (REGULAR, SMOOTH, rav1e_put_8tap_regular_smooth_8bpc_neon),
  (REGULAR, SHARP, rav1e_put_8tap_regular_sharp_8bpc_neon),
  (SMOOTH, REGULAR, rav1e_put_8tap_smooth_regular_8bpc_neon),
  (SMOOTH, SMOOTH, rav1e_put_8tap_smooth_8bpc_neon),
  (SMOOTH, SHARP, rav1e_put_8tap_smooth_sharp_8bpc_neon),
  (SHARP, REGULAR, rav1e_put_8tap_sharp_regular_8bpc_neon),
  (SHARP, SMOOTH, rav1e_put_8tap_sharp_smooth_8bpc_neon),
  (SHARP, SHARP, rav1e_put_8tap_sharp_8bpc_neon),
  (BILINEAR, BILINEAR, rav1e_put_bilin_8bpc_neon)
);

cpu_function_lookup_table!(
  PUT_FNS: [[Option<PutFn>; 16]],
  default: [None; 16],
  [NEON]
);

macro_rules! decl_mc_hbd_fns {
  ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
    extern {
      $(
        fn $func_name(
          dst: *mut u16, dst_stride: isize, src: *const u16, src_stride: isize,
          w: i32, h: i32, mx: i32, my: i32, bitdepth_max: i32,
        );
      )*
    }

    static PUT_HBD_FNS_NEON: [Option<PutHBDFn>; 16] = {
      let mut out: [Option<PutHBDFn>; 16] = [None; 16];
      $(
        out[get_2d_mode_idx($mode_x, $mode_y)] = Some($func_name);
      )*
      out
    };
  }
}

decl_mc_hbd_fns!(
  (REGULAR, REGULAR, rav1e_put_8tap_regular_16bpc_neon),
  (REGULAR, SMOOTH, rav1e_put_8tap_regular_smooth_16bpc_neon),
  (REGULAR, SHARP, rav1e_put_8tap_regular_sharp_16bpc_neon),
  (SMOOTH, REGULAR, rav1e_put_8tap_smooth_regular_16bpc_neon),
  (SMOOTH, SMOOTH, rav1e_put_8tap_smooth_16bpc_neon),
  (SMOOTH, SHARP, rav1e_put_8tap_smooth_sharp_16bpc_neon),
  (SHARP, REGULAR, rav1e_put_8tap_sharp_regular_16bpc_neon),
  (SHARP, SMOOTH, rav1e_put_8tap_sharp_smooth_16bpc_neon),
  (SHARP, SHARP, rav1e_put_8tap_sharp_16bpc_neon),
  (BILINEAR, BILINEAR, rav1e_put_bilin_16bpc_neon)
);

cpu_function_lookup_table!(
  PUT_HBD_FNS: [[Option<PutHBDFn>; 16]],
  default: [None; 16],
  [NEON]
);

macro_rules! decl_mct_fns {
  ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
    extern {
      $(
        fn $func_name(
          tmp: *mut i16, src: *const u8, src_stride: libc::ptrdiff_t, w: i32,
          h: i32, mx: i32, my: i32,
        );
      )*
    }

    static PREP_FNS_NEON: [Option<PrepFn>; 16] = {
      let mut out: [Option<PrepFn>; 16] = [None; 16];
      $(
        out[get_2d_mode_idx($mode_x, $mode_y)] = Some($func_name);
      )*
      out
    };
  }
}

decl_mct_fns!(
  (REGULAR, REGULAR, rav1e_prep_8tap_regular_8bpc_neon),
  (REGULAR, SMOOTH, rav1e_prep_8tap_regular_smooth_8bpc_neon),
  (REGULAR, SHARP, rav1e_prep_8tap_regular_sharp_8bpc_neon),
  (SMOOTH, REGULAR, rav1e_prep_8tap_smooth_regular_8bpc_neon),
  (SMOOTH, SMOOTH, rav1e_prep_8tap_smooth_8bpc_neon),
  (SMOOTH, SHARP, rav1e_prep_8tap_smooth_sharp_8bpc_neon),
  (SHARP, REGULAR, rav1e_prep_8tap_sharp_regular_8bpc_neon),
  (SHARP, SMOOTH, rav1e_prep_8tap_sharp_smooth_8bpc_neon),
  (SHARP, SHARP, rav1e_prep_8tap_sharp_8bpc_neon),
  (BILINEAR, BILINEAR, rav1e_prep_bilin_8bpc_neon)
);

cpu_function_lookup_table!(
  PREP_FNS: [[Option<PrepFn>; 16]],
  default: [None; 16],
  [NEON]
);

macro_rules! decl_mct_hbd_fns {
  ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
    extern {
      $(
        fn $func_name(
          tmp: *mut i16, src: *const u16, src_stride: libc::ptrdiff_t, w: i32,
          h: i32, mx: i32, my: i32, bitdepth_max: i32,
        );
      )*
    }

    static PREP_HBD_FNS_NEON: [Option<PrepHBDFn>; 16] = {
      let mut out: [Option<PrepHBDFn>; 16] = [None; 16];
      $(
        out[get_2d_mode_idx($mode_x, $mode_y)] = Some($func_name);
      )*
      out
    };
  }
}

decl_mct_hbd_fns!(
  (REGULAR, REGULAR, rav1e_prep_8tap_regular_16bpc_neon),
  (REGULAR, SMOOTH, rav1e_prep_8tap_regular_smooth_16bpc_neon),
  (REGULAR, SHARP, rav1e_prep_8tap_regular_sharp_16bpc_neon),
  (SMOOTH, REGULAR, rav1e_prep_8tap_smooth_regular_16bpc_neon),
  (SMOOTH, SMOOTH, rav1e_prep_8tap_smooth_16bpc_neon),
  (SMOOTH, SHARP, rav1e_prep_8tap_smooth_sharp_16bpc_neon),
  (SHARP, REGULAR, rav1e_prep_8tap_sharp_regular_16bpc_neon),
  (SHARP, SMOOTH, rav1e_prep_8tap_sharp_smooth_16bpc_neon),
  (SHARP, SHARP, rav1e_prep_8tap_sharp_16bpc_neon),
  (BILINEAR, BILINEAR, rav1e_prep_bilin_16bpc_neon)
);

cpu_function_lookup_table!(
  PREP_HBD_FNS: [[Option<PrepHBDFn>; 16]],
  default: [None; 16],
  [NEON]
);

extern {
  fn rav1e_avg_8bpc_neon(
    dst: *mut u8, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32,
  );
}

cpu_function_lookup_table!(
  AVG_FNS: [Option<AvgFn>],
  default: None,
  [(NEON, Some(rav1e_avg_8bpc_neon))]
);

extern {
  fn rav1e_avg_16bpc_neon(
    dst: *mut u16, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32, bitdepth_max: i32,
  );
}

cpu_function_lookup_table!(
  AVG_HBD_FNS: [Option<AvgHBDFn>],
  default: None,
  [(NEON, Some(rav1e_avg_16bpc_neon))]
);
