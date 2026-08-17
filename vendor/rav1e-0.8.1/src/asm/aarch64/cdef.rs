// Copyright (c) 2020-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cdef::*;
use crate::cpu_features::CpuFeatureLevel;
use crate::frame::*;
use crate::tiling::PlaneRegionMut;
use crate::util::*;

type CdefPaddingFn = unsafe extern fn(
  tmp: *mut u16,
  src: *const u8,
  src_stride: isize,
  left: *const [u8; 2],
  top: *const u8,
  bottom: *const u8,
  h: i32,
  edges: isize,
);

type CdefPaddingHBDFn = unsafe extern fn(
  tmp: *mut u16,
  src: *const u16,
  src_stride: isize,
  left: *const [u16; 2],
  top: *const u16,
  bottom: *const u16,
  h: i32,
  edges: isize,
);

type CdefFilterFn = unsafe extern fn(
  dst: *mut u8,
  dst_stride: isize,
  tmp: *const u16,
  pri_strength: i32,
  sec_strength: i32,
  dir: i32,
  damping: i32,
  h: i32,
  edges: isize,
);

type CdefFilterHBDFn = unsafe extern fn(
  dst: *mut u16,
  dst_stride: isize,
  tmp: *const u16,
  pri_strength: i32,
  sec_strength: i32,
  dir: i32,
  damping: i32,
  h: i32,
  edges: isize,
  bitdepth_max: i32,
);

#[inline(always)]
const fn decimate_index(xdec: usize, ydec: usize) -> usize {
  ((ydec << 1) | xdec) & 3
}

pub(crate) unsafe fn cdef_filter_block<T: Pixel>(
  dst: &mut PlaneRegionMut<'_, T>, src: *const T, src_stride: isize,
  pri_strength: i32, sec_strength: i32, dir: usize, damping: i32,
  bit_depth: usize, xdec: usize, ydec: usize, edges: u8, cpu: CpuFeatureLevel,
) {
  let call_rust = |dst: &mut PlaneRegionMut<T>| {
    rust::cdef_filter_block(
      dst,
      src,
      src_stride,
      pri_strength,
      sec_strength,
      dir,
      damping,
      bit_depth,
      xdec,
      ydec,
      edges,
      cpu,
    );
  };

  #[cfg(feature = "check_asm")]
  let ref_dst = {
    let mut copy = dst.scratch_copy();
    call_rust(&mut copy.as_region_mut());
    copy
  };
  match T::type_enum() {
    PixelType::U8 => {
      match (
        CDEF_PAD_FNS[cpu.as_index()][decimate_index(xdec, ydec)],
        CDEF_FILTER_FNS[cpu.as_index()][decimate_index(xdec, ydec)],
      ) {
        (Some(pad), Some(func)) => {
          let h = if ydec == 1 { 4 } else { 8 };
          let tmpstride = if xdec == 1 { 8 } else { 16 } as isize;
          const MAXTMPSTRIDE: isize = 16;
          const TMPSIZE: usize = (12 * MAXTMPSTRIDE + 8) as usize;
          let mut tmp: Aligned<[u16; TMPSIZE]> =
            Aligned::new([CDEF_VERY_LARGE; TMPSIZE]);
          let top = src.offset(-2 * src_stride);
          let bottom = src.offset(h as isize * src_stride);
          let mut left: Aligned<[[u8; 2]; 8]> = Aligned::new([[0; 2]; 8]);

          // Rather than modify the dav1d assembly, just swallow our
          // pride and copy relevant portions of src into a left
          // array.  The array is a monolithic, packed [x=2][y=8],
          // though it is aligned to start on a 16-byte boundary.
          if edges & CDEF_HAVE_LEFT != 0 {
            let mut wptr = src.offset(-2) as *const u8;
            for i in 0..h {
              left.data[i as usize][0] = *wptr;
              left.data[i as usize][1] = *wptr.add(1);
              wptr = wptr.offset(src_stride);
            }
          }

          // dav1d's implicit indexing as of this version: tmp array
          // working pointer points to the upper-left of the current
          // coded block, not the upper-left of the tmp array storage,
          // with an adjustment to ensure UL of the block is 16-byte
          // aligned.  src, similarly, points to upper left of src
          // block being coded.  top points to coded block minus two
          // rows (that is, src.x, src.y-2).  It does _not_ point to
          // src.x-2, src.y-2.
          (pad)(
            tmp.data.as_mut_ptr().offset(2 * tmpstride + 8),
            src as *const u8,
            T::to_asm_stride(src_stride as usize),
            left.data.as_ptr(),
            top as *const u8,
            bottom as *const u8,
            8 >> ydec,
            edges.into(),
          );

          (func)(
            dst.data_ptr_mut() as *mut u8,
            T::to_asm_stride(dst.plane_cfg.stride),
            tmp.data.as_ptr().offset(2 * tmpstride + 8),
            pri_strength,
            sec_strength,
            dir as i32,
            damping,
            8 >> ydec,
            edges.into(),
          );
        }
        _ => call_rust(dst),
      }
    }
    PixelType::U16 => {
      match (
        CDEF_PAD_HBD_FNS[cpu.as_index()][decimate_index(xdec, ydec)],
        CDEF_FILTER_HBD_FNS[cpu.as_index()][decimate_index(xdec, ydec)],
      ) {
        // almost exactly as above, but the call convention isn't
        // quite what we'd need to roll 8 bit and HBD together in one
        // clause using Rust macros.  See comments above for
        // indexing/addressing notes.
        (Some(pad), Some(func)) => {
          let h = if ydec == 1 { 4 } else { 8 };
          let tmpstride = if xdec == 1 { 8 } else { 16 } as isize;
          const MAXTMPSTRIDE: isize = 16;
          const TMPSIZE: usize = (12 * MAXTMPSTRIDE + 8) as usize;
          let mut tmp: Aligned<[u16; TMPSIZE]> =
            Aligned::new([CDEF_VERY_LARGE; TMPSIZE]);
          let top = src.offset(-2 * src_stride);
          let bottom = src.offset(h as isize * src_stride);
          let mut left: Aligned<[[u16; 2]; 8]> = Aligned::new([[0; 2]; 8]);

          if edges & CDEF_HAVE_LEFT != 0 {
            let mut wptr = src.offset(-2) as *const u16;
            for i in 0..h {
              left.data[i as usize][0] = *wptr;
              left.data[i as usize][1] = *wptr.add(1);
              wptr = wptr.offset(src_stride);
            }
          }

          (pad)(
            tmp.data.as_mut_ptr().offset(2 * tmpstride + 8),
            src as *const u16,
            T::to_asm_stride(src_stride as usize),
            left.data.as_ptr(),
            top as *const u16,
            bottom as *const u16,
            8 >> ydec,
            edges.into(),
          );

          (func)(
            dst.data_ptr_mut() as *mut u16,
            T::to_asm_stride(dst.plane_cfg.stride),
            tmp.data.as_ptr().offset(2 * tmpstride + 8),
            pri_strength,
            sec_strength,
            dir as i32,
            damping,
            8 >> ydec,
            edges.into(),
            (1 << bit_depth) - 1,
          );
        }
        _ => call_rust(dst),
      }
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

extern {
  fn rav1e_cdef_filter4_8bpc_neon(
    dst: *mut u8, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize,
  );

  fn rav1e_cdef_padding4_8bpc_neon(
    tmp: *mut u16, src: *const u8, src_stride: isize, left: *const [u8; 2],
    top: *const u8, bottom: *const u8, h: i32, edges: isize,
  );

  fn rav1e_cdef_filter8_8bpc_neon(
    dst: *mut u8, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize,
  );

  fn rav1e_cdef_padding8_8bpc_neon(
    tmp: *mut u16, src: *const u8, src_stride: isize, left: *const [u8; 2],
    top: *const u8, bottom: *const u8, h: i32, edges: isize,
  );

  fn rav1e_cdef_filter4_16bpc_neon(
    dst: *mut u16, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize, bd: i32,
  );

  fn rav1e_cdef_padding4_16bpc_neon(
    tmp: *mut u16, src: *const u16, src_stride: isize, left: *const [u16; 2],
    top: *const u16, bottom: *const u16, h: i32, edges: isize,
  );

  fn rav1e_cdef_filter8_16bpc_neon(
    dst: *mut u16, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize, bd: i32,
  );

  fn rav1e_cdef_padding8_16bpc_neon(
    tmp: *mut u16, src: *const u16, src_stride: isize, left: *const [u16; 2],
    top: *const u16, bottom: *const u16, h: i32, edges: isize,
  );
}

static CDEF_PAD_FNS_NEON: [Option<CdefPaddingFn>; 4] = {
  let mut out: [Option<CdefPaddingFn>; 4] = [None; 4];
  out[decimate_index(1, 1)] = Some(rav1e_cdef_padding4_8bpc_neon);
  out[decimate_index(1, 0)] = Some(rav1e_cdef_padding4_8bpc_neon);
  out[decimate_index(0, 0)] = Some(rav1e_cdef_padding8_8bpc_neon);
  out
};

static CDEF_FILTER_FNS_NEON: [Option<CdefFilterFn>; 4] = {
  let mut out: [Option<CdefFilterFn>; 4] = [None; 4];
  out[decimate_index(1, 1)] = Some(rav1e_cdef_filter4_8bpc_neon);
  out[decimate_index(1, 0)] = Some(rav1e_cdef_filter4_8bpc_neon);
  out[decimate_index(0, 0)] = Some(rav1e_cdef_filter8_8bpc_neon);
  out
};

static CDEF_PAD_HBD_FNS_NEON: [Option<CdefPaddingHBDFn>; 4] = {
  let mut out: [Option<CdefPaddingHBDFn>; 4] = [None; 4];
  out[decimate_index(1, 1)] = Some(rav1e_cdef_padding4_16bpc_neon);
  out[decimate_index(1, 0)] = Some(rav1e_cdef_padding4_16bpc_neon);
  out[decimate_index(0, 0)] = Some(rav1e_cdef_padding8_16bpc_neon);
  out
};

static CDEF_FILTER_HBD_FNS_NEON: [Option<CdefFilterHBDFn>; 4] = {
  let mut out: [Option<CdefFilterHBDFn>; 4] = [None; 4];
  out[decimate_index(1, 1)] = Some(rav1e_cdef_filter4_16bpc_neon);
  out[decimate_index(1, 0)] = Some(rav1e_cdef_filter4_16bpc_neon);
  out[decimate_index(0, 0)] = Some(rav1e_cdef_filter8_16bpc_neon);
  out
};

cpu_function_lookup_table!(
  CDEF_PAD_FNS: [[Option<CdefPaddingFn>; 4]],
  default: [None; 4],
  [NEON]
);

cpu_function_lookup_table!(
  CDEF_FILTER_FNS: [[Option<CdefFilterFn>; 4]],
  default: [None; 4],
  [NEON]
);

cpu_function_lookup_table!(
  CDEF_PAD_HBD_FNS: [[Option<CdefPaddingHBDFn>; 4]],
  default: [None; 4],
  [NEON]
);

cpu_function_lookup_table!(
  CDEF_FILTER_HBD_FNS: [[Option<CdefFilterHBDFn>; 4]],
  default: [None; 4],
  [NEON]
);

type CdefDirLBDFn =
  unsafe extern fn(tmp: *const u8, tmp_stride: isize, var: *mut u32) -> i32;
type CdefDirHBDFn = unsafe extern fn(
  tmp: *const u16,
  tmp_stride: isize,
  var: *mut u32,
  bitdepth_max: i32,
) -> i32;

#[inline(always)]
#[allow(clippy::let_and_return)]
pub(crate) fn cdef_find_dir<T: Pixel>(
  img: &PlaneSlice<'_, T>, var: &mut u32, coeff_shift: usize,
  cpu: CpuFeatureLevel,
) -> i32 {
  let call_rust =
    |var: &mut u32| rust::cdef_find_dir::<T>(img, var, coeff_shift, cpu);

  #[cfg(feature = "check_asm")]
  let (ref_dir, ref_var) = {
    let mut var: u32 = 0;
    let dir = call_rust(&mut var);
    (dir, var)
  };

  let dir = match T::type_enum() {
    PixelType::U8 => {
      if let Some(func) = CDEF_DIR_LBD_FNS[cpu.as_index()] {
        unsafe {
          (func)(
            img.as_ptr() as *const _,
            T::to_asm_stride(img.plane.cfg.stride),
            var as *mut u32,
          )
        }
      } else {
        call_rust(var)
      }
    }
    PixelType::U16 if coeff_shift > 0 => {
      if let Some(func) = CDEF_DIR_HBD_FNS[cpu.as_index()] {
        unsafe {
          (func)(
            img.as_ptr() as *const _,
            T::to_asm_stride(img.plane.cfg.stride),
            var as *mut u32,
            (1 << (coeff_shift + 8)) - 1,
          )
        }
      } else {
        call_rust(var)
      }
    }
    _ => call_rust(var),
  };

  #[cfg(feature = "check_asm")]
  {
    assert_eq!(dir, ref_dir);
    assert_eq!(*var, ref_var);
  }

  dir
}

extern {
  fn rav1e_cdef_find_dir_8bpc_neon(
    tmp: *const u8, tmp_stride: isize, var: *mut u32,
  ) -> i32;
}

extern {
  fn rav1e_cdef_find_dir_16bpc_neon(
    tmp: *const u16, tmp_stride: isize, var: *mut u32, max_bitdepth: i32,
  ) -> i32;
}

cpu_function_lookup_table!(
  CDEF_DIR_LBD_FNS: [Option<CdefDirLBDFn>],
  default: None,
  [(NEON, Some(rav1e_cdef_find_dir_8bpc_neon))]
);

cpu_function_lookup_table!(
  CDEF_DIR_HBD_FNS: [Option<CdefDirHBDFn>],
  default: None,
  [(NEON, Some(rav1e_cdef_find_dir_16bpc_neon))]
);
