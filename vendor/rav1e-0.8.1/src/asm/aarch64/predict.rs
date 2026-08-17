// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::context::MAX_TX_SIZE;
use crate::cpu_features::CpuFeatureLevel;
use crate::partition::{BlockSize, IntraEdge};
use crate::predict::rust::{
  dr_intra_derivative, select_ief_strength, select_ief_upsample,
};
use crate::predict::{
  rust, IntraEdgeFilterParameters, PredictionMode, PredictionVariant,
};
use crate::tiling::{PlaneRegion, PlaneRegionMut};
use crate::transform::TxSize;
use crate::{Pixel, PixelType};
use libc;
use libc::{c_int, ptrdiff_t};
use std::mem::MaybeUninit;
use PixelType::{U16, U8};

macro_rules! decl_cfl_ac_fn {
  ($($f:ident),+) => {
    extern {
      $(
        fn $f(
          ac: *mut MaybeUninit<i16>, src: *const u8, stride: libc::ptrdiff_t,
          w_pad: libc::c_int, h_pad: libc::c_int,
          width: libc::c_int, height: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_ac_fn! {
  rav1e_ipred_cfl_ac_420_8bpc_neon,
  rav1e_ipred_cfl_ac_422_8bpc_neon,
  rav1e_ipred_cfl_ac_444_8bpc_neon
}

macro_rules! decl_cfl_ac_hbd_fn {
  ($($f:ident),+) => {
    extern {
      $(
        fn $f(
          ac: *mut MaybeUninit<i16>, src: *const u16, stride: libc::ptrdiff_t,
          w_pad: libc::c_int, h_pad: libc::c_int,
          width: libc::c_int, height: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_ac_hbd_fn! {
  rav1e_ipred_cfl_ac_420_16bpc_neon,
  rav1e_ipred_cfl_ac_422_16bpc_neon,
  rav1e_ipred_cfl_ac_444_16bpc_neon
}

macro_rules! decl_angular_ipred_fn {
  ($($f:ident),+) => {
    extern {
      $(
        fn $f(
          dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
          width: libc::c_int, height: libc::c_int, angle: libc::c_int,
        );
      )*
    }
  };
}

decl_angular_ipred_fn! {
  rav1e_ipred_dc_8bpc_neon,
  rav1e_ipred_dc_128_8bpc_neon,
  rav1e_ipred_dc_left_8bpc_neon,
  rav1e_ipred_dc_top_8bpc_neon,
  rav1e_ipred_v_8bpc_neon,
  rav1e_ipred_h_8bpc_neon,
  rav1e_ipred_smooth_8bpc_neon,
  rav1e_ipred_smooth_v_8bpc_neon,
  rav1e_ipred_smooth_h_8bpc_neon,
  rav1e_ipred_paeth_8bpc_neon
}

macro_rules! decl_angular_ipred_hbd_fn {
  ($($f:ident),+) => {
    extern {
      $(
        fn $f(
          dst: *mut u16, stride: libc::ptrdiff_t, topleft: *const u16,
          width: libc::c_int, height: libc::c_int, angle: libc::c_int,
          max_width: libc::c_int, max_height: libc::c_int,
          bit_depth_max: libc::c_int,
        );
      )*
    }
  };
}

decl_angular_ipred_hbd_fn! {
  rav1e_ipred_dc_16bpc_neon,
  rav1e_ipred_dc_128_16bpc_neon,
  rav1e_ipred_dc_left_16bpc_neon,
  rav1e_ipred_dc_top_16bpc_neon,
  rav1e_ipred_v_16bpc_neon,
  rav1e_ipred_h_16bpc_neon,
  rav1e_ipred_smooth_16bpc_neon,
  rav1e_ipred_smooth_v_16bpc_neon,
  rav1e_ipred_smooth_h_16bpc_neon,
  rav1e_ipred_paeth_16bpc_neon
}

macro_rules! decl_cfl_pred_fn {
  ($($f:ident),+) => {
    extern {
      $(
        fn $f(
          dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
          width: libc::c_int, height: libc::c_int, ac: *const i16,
          alpha: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_pred_fn! {
  rav1e_ipred_cfl_8bpc_neon,
  rav1e_ipred_cfl_128_8bpc_neon,
  rav1e_ipred_cfl_left_8bpc_neon,
  rav1e_ipred_cfl_top_8bpc_neon
}

macro_rules! decl_cfl_pred_hbd_fn {
  ($($f:ident),+) => {
    extern {
      $(
        fn $f(
          dst: *mut u16, stride: libc::ptrdiff_t, topleft: *const u16,
          width: libc::c_int, height: libc::c_int, ac: *const i16,
          alpha: libc::c_int, bit_depth_max: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_pred_hbd_fn! {
  rav1e_ipred_cfl_16bpc_neon,
  rav1e_ipred_cfl_128_16bpc_neon,
  rav1e_ipred_cfl_left_16bpc_neon,
  rav1e_ipred_cfl_top_16bpc_neon
}

extern {
  fn rav1e_ipred_z1_upsample_edge_8bpc_neon(
    out: *mut u8, hsz: c_int, _in: *const u8, end: c_int,
  );
  fn rav1e_ipred_z1_upsample_edge_16bpc_neon(
    out: *mut u16, hsz: c_int, _in: *const u16, end: c_int,
    bit_depth_max: c_int,
  );
  fn rav1e_ipred_z1_filter_edge_8bpc_neon(
    out: *mut u8, sz: c_int, _in: *const u8, end: c_int, strength: c_int,
  );
  fn rav1e_ipred_z1_filter_edge_16bpc_neon(
    out: *mut u16, sz: c_int, _in: *const u16, end: c_int, strength: c_int,
  );
  fn rav1e_ipred_z1_fill1_8bpc_neon(
    dst: *mut u8, stride: ptrdiff_t, top: *const u8, width: c_int,
    height: c_int, dx: c_int, max_base_x: c_int,
  );
  fn rav1e_ipred_z1_fill1_16bpc_neon(
    dst: *mut u16, stride: ptrdiff_t, top: *const u16, width: c_int,
    height: c_int, dx: c_int, max_base_x: c_int,
  );
  fn rav1e_ipred_z1_fill2_8bpc_neon(
    dst: *mut u8, stride: ptrdiff_t, top: *const u8, width: c_int,
    height: c_int, dx: c_int, max_base_x: c_int,
  );
  fn rav1e_ipred_z1_fill2_16bpc_neon(
    dst: *mut u16, stride: ptrdiff_t, top: *const u16, width: c_int,
    height: c_int, dx: c_int, max_base_x: c_int,
  );
  fn rav1e_ipred_z2_upsample_edge_8bpc_neon(
    out: *mut u8, sz: c_int, _in: *const u8,
  );
  fn rav1e_ipred_z2_upsample_edge_16bpc_neon(
    out: *mut u16, sz: c_int, _in: *const u16, bit_depth_max: c_int,
  );
  fn rav1e_ipred_z2_fill1_8bpc_neon(
    dst: *mut u8, stride: ptrdiff_t, top: *const u8, left: *const u8,
    width: c_int, height: c_int, dx: c_int, dy: c_int,
  );
  fn rav1e_ipred_z2_fill1_16bpc_neon(
    dst: *mut u16, stride: ptrdiff_t, top: *const u16, left: *const u16,
    width: c_int, height: c_int, dx: c_int, dy: c_int,
  );
  fn rav1e_ipred_z2_fill2_8bpc_neon(
    dst: *mut u8, stride: ptrdiff_t, top: *const u8, left: *const u8,
    width: c_int, height: c_int, dx: c_int, dy: c_int,
  );
  fn rav1e_ipred_z2_fill2_16bpc_neon(
    dst: *mut u16, stride: ptrdiff_t, top: *const u16, left: *const u16,
    width: c_int, height: c_int, dx: c_int, dy: c_int,
  );
  fn rav1e_ipred_z2_fill3_8bpc_neon(
    dst: *mut u8, stride: ptrdiff_t, top: *const u8, left: *const u8,
    width: c_int, height: c_int, dx: c_int, dy: c_int,
  );
  fn rav1e_ipred_z2_fill3_16bpc_neon(
    dst: *mut u16, stride: ptrdiff_t, top: *const u16, left: *const u16,
    width: c_int, height: c_int, dx: c_int, dy: c_int,
  );
  fn rav1e_ipred_z3_fill1_8bpc_neon(
    dst: *mut u8, stride: ptrdiff_t, left: *const u8, width: c_int,
    height: c_int, dy: c_int, max_base_y: c_int,
  );
  fn rav1e_ipred_z3_fill1_16bpc_neon(
    dst: *mut u16, stride: ptrdiff_t, left: *const u16, width: c_int,
    height: c_int, dy: c_int, max_base_y: c_int,
  );
  fn rav1e_ipred_z3_fill2_8bpc_neon(
    dst: *mut u8, stride: ptrdiff_t, left: *const u8, width: c_int,
    height: c_int, dy: c_int, max_base_y: c_int,
  );
  fn rav1e_ipred_z3_fill2_16bpc_neon(
    dst: *mut u16, stride: ptrdiff_t, left: *const u16, width: c_int,
    height: c_int, dy: c_int, max_base_y: c_int,
  );
  fn rav1e_ipred_reverse_8bpc_neon(dst: *mut u8, src: *const u8, n: c_int);
  fn rav1e_ipred_reverse_16bpc_neon(dst: *mut u16, src: *const u16, n: c_int);
}
#[inline]
unsafe fn ipred_z1_upsample_edge<T: Pixel>(
  o: *mut T, sz: c_int, i: *const T, n: c_int, m: c_int,
) {
  match T::type_enum() {
    U8 => rav1e_ipred_z1_upsample_edge_8bpc_neon(o as _, sz, i as _, n),
    U16 => rav1e_ipred_z1_upsample_edge_16bpc_neon(o as _, sz, i as _, n, m),
  }
}
#[inline]
unsafe fn ipred_z2_upsample_edge<T: Pixel>(
  o: *mut T, sz: c_int, i: *const T, m: c_int,
) {
  match T::type_enum() {
    U8 => rav1e_ipred_z2_upsample_edge_8bpc_neon(o as _, sz, i as _),
    U16 => rav1e_ipred_z2_upsample_edge_16bpc_neon(o as _, sz, i as _, m),
  }
}
#[inline]
unsafe fn ipred_z1_filter_edge<T: Pixel>(
  o: *mut T, sz: c_int, i: *const T, n: c_int, s: c_int,
) {
  match T::type_enum() {
    U8 => rav1e_ipred_z1_filter_edge_8bpc_neon(o as _, sz, i as _, n, s),
    U16 => rav1e_ipred_z1_filter_edge_16bpc_neon(o as _, sz, i as _, n, s),
  }
}
#[inline]
unsafe fn ipred_reverse<T: Pixel>(o: *mut T, i: *const T, n: c_int) {
  match T::type_enum() {
    U8 => rav1e_ipred_reverse_8bpc_neon(o as _, i as _, n),
    U16 => rav1e_ipred_reverse_16bpc_neon(o as _, i as _, n),
  }
}

#[rustfmt::skip]
struct Fill(
  [unsafe extern fn(*mut u8, ptrdiff_t, *const u8, c_int, c_int, c_int, c_int); 2],
  [unsafe extern fn(*mut u16, ptrdiff_t, *const u16, c_int, c_int, c_int, c_int); 2],
);
impl Fill {
  unsafe fn ipred_fill<T: Pixel>(
    self, dst: *mut T, stride: ptrdiff_t, src: *const T, w: c_int, h: c_int,
    d: c_int, max_base: c_int, upsample: bool,
  ) {
    let u = upsample as usize;
    match T::type_enum() {
      U8 => self.0[u](dst as _, stride, src as _, w, h, d, max_base),
      U16 => self.1[u](dst as _, stride, src as _, w, h, d, max_base),
    }
  }
}
const Z1: Fill = Fill(
  [rav1e_ipred_z1_fill1_8bpc_neon, rav1e_ipred_z1_fill2_8bpc_neon],
  [rav1e_ipred_z1_fill1_16bpc_neon, rav1e_ipred_z1_fill2_16bpc_neon],
);
const Z3: Fill = Fill(
  [rav1e_ipred_z3_fill1_8bpc_neon, rav1e_ipred_z3_fill2_8bpc_neon],
  [rav1e_ipred_z3_fill1_16bpc_neon, rav1e_ipred_z3_fill2_16bpc_neon],
);

#[rustfmt::skip]
struct Fill2(
  [unsafe extern fn(*mut u8, ptrdiff_t, *const u8, *const u8, c_int, c_int, c_int, c_int); 3],
  [unsafe extern fn(*mut u16, ptrdiff_t, *const u16, *const u16, c_int, c_int, c_int, c_int); 3],
);
impl Fill2 {
  unsafe fn ipred_fill<T: Pixel>(
    self, dst: *mut T, stride: ptrdiff_t, top: *const T, left: *const T,
    w: c_int, h: c_int, dx: c_int, dy: c_int, upsample_above: bool,
    upsample_left: bool,
  ) {
    let u = if upsample_left { 2 } else { upsample_above as usize };
    match T::type_enum() {
      U8 => self.0[u](dst as _, stride, top as _, left as _, w, h, dx, dy),
      U16 => self.1[u](dst as _, stride, top as _, left as _, w, h, dx, dy),
    }
  }
}
const Z2: Fill2 = Fill2(
  [
    rav1e_ipred_z2_fill1_8bpc_neon,
    rav1e_ipred_z2_fill2_8bpc_neon,
    rav1e_ipred_z2_fill3_8bpc_neon,
  ],
  [
    rav1e_ipred_z2_fill1_16bpc_neon,
    rav1e_ipred_z2_fill2_16bpc_neon,
    rav1e_ipred_z2_fill3_16bpc_neon,
  ],
);

unsafe fn ipred_z1<T: Pixel>(
  dst: *mut T, stride: ptrdiff_t, src: *const T, angle: isize, w: c_int,
  h: c_int, bd_max: c_int, edge_filter: bool, smooth_filter: bool,
) {
  let mut dx = dr_intra_derivative(angle as _) as c_int;
  let mut out = [MaybeUninit::<T>::uninit(); MAX_TX_SIZE * 4 + 15 * 2 + 16];
  let out = out.as_mut_ptr() as *mut T;

  let upsample_above = edge_filter
    && select_ief_upsample(w as _, h as _, smooth_filter, 90 - angle);
  let max_base_x = if upsample_above {
    ipred_z1_upsample_edge(out, w + h, src, w + w.min(h), bd_max);
    dx <<= 1;
    2 * (w + h) - 2
  } else {
    let strength =
      select_ief_strength(w as _, h as _, smooth_filter, 90 - angle) as c_int;
    if strength != 0 {
      ipred_z1_filter_edge(out, w + h, src, w + w.min(h), strength);
      w + h - 1
    } else {
      let n = w + w.min(h);
      out.copy_from_nonoverlapping(src.add(1), n as usize);
      n - 1
    }
  };

  let base_inc = 1 + upsample_above as c_int;
  let pad_pixels = w + 15;
  let fill_pixel = out.add(max_base_x as usize).read();
  let base = out.add(max_base_x as usize + 1);
  for i in 0..(pad_pixels * base_inc) as usize {
    base.add(i).write(fill_pixel);
  }

  Z1.ipred_fill(dst, stride, out, w, h, dx, max_base_x, upsample_above);
}

unsafe fn ipred_z2<T: Pixel>(
  dst: *mut T, stride: ptrdiff_t, src: *const T, angle: isize, w: c_int,
  h: c_int, max_w: c_int, max_h: c_int, bd_max: c_int, edge_filter: bool,
  smooth_filter: bool,
) {
  assert!(angle > 90 && angle < 180);
  let mut dx = dr_intra_derivative((180 - angle) as _) as c_int;
  let mut dy = dr_intra_derivative((angle - 90) as _) as c_int;
  let us_left = edge_filter
    && select_ief_upsample(w as _, h as _, smooth_filter, 180 - angle);
  let us_above = edge_filter
    && select_ief_upsample(w as _, h as _, smooth_filter, angle - 90);
  let mut out = [MaybeUninit::<T>::uninit(); 3 * (MAX_TX_SIZE * 4 + 1)];
  let out = out.as_mut_ptr() as *mut T;
  let left = out.add(2 * (64 + 1));
  let top = out.add(64 + 1);
  let flipped = out;
  if us_above {
    ipred_z2_upsample_edge(top, w, src, bd_max);
    dx <<= 1;
  } else {
    let strength =
      select_ief_strength(w as _, h as _, smooth_filter, angle - 90) as c_int;

    if edge_filter && strength != 0 {
      ipred_z1_filter_edge(top.add(1), max_w.min(w), src, w, strength);
      if max_w < w {
        top.add((1 + max_w) as _).copy_from_nonoverlapping(
          src.add((1 + max_w) as _),
          (w - max_w) as _,
        );
      }
    } else {
      top.add(1).copy_from_nonoverlapping(src.add(1), w as _);
    }
  }
  if us_left {
    flipped.write(src.read());
    ipred_reverse(flipped.add(1), src, h);
    ipred_z2_upsample_edge(left, h, flipped, bd_max);
    dy <<= 1;
  } else {
    let strength =
      select_ief_strength(w as _, h as _, smooth_filter, 180 - angle) as c_int;

    if edge_filter && strength != 0 {
      flipped.write(src.read());
      ipred_reverse(flipped.add(1), src, h);
      ipred_z1_filter_edge(left.add(1), max_h.min(h), flipped, h, strength);
      if max_h < h {
        left.add((1 + max_h) as _).copy_from_nonoverlapping(
          flipped.add((1 + max_h) as _),
          (h - max_h) as _,
        );
      }
    } else {
      ipred_reverse(left.add(1), src, h);
    }
  }
  let top_left = src.read();
  top.write(top_left);
  left.write(top_left);

  assert!(!(us_above && us_left));
  Z2.ipred_fill(dst, stride, top, left, w, h, dx, dy, us_above, us_left);
}

unsafe fn ipred_z3<T: Pixel>(
  dst: *mut T, stride: ptrdiff_t, src: *const T, angle: isize, w: c_int,
  h: c_int, bd_max: c_int, edge_filter: bool, smooth_filter: bool,
) {
  assert!(angle > 180);
  let mut dy = dr_intra_derivative(270 - angle as usize) as c_int;
  let mut tmp = [MaybeUninit::<T>::uninit(); MAX_TX_SIZE * 4 + 16];
  let mut out = [MaybeUninit::<T>::uninit(); MAX_TX_SIZE * 8 + 15 * 2];
  let out = out.as_mut_ptr() as *mut T;
  let tmp = tmp.as_mut_ptr() as *mut T;

  let upsample_left = edge_filter
    && select_ief_upsample(w as _, h as _, smooth_filter, angle - 180);
  let max_base_y = if upsample_left {
    tmp.write(src.read());
    ipred_reverse(tmp.add(1), src, h + w.max(h));
    ipred_z1_upsample_edge(out, w + h, tmp, h + w.min(h), bd_max);
    dy <<= 1;
    2 * (w + h) - 2
  } else {
    let strength =
      select_ief_strength(w as _, h as _, smooth_filter, angle - 180) as c_int;
    if strength != 0 {
      tmp.write(src.read());
      ipred_reverse(tmp.add(1), src, h + w.max(h));
      ipred_z1_filter_edge(out, w + h, tmp, h + w.min(h), strength);
      w + h - 1
    } else {
      let n = w + w.min(h);
      ipred_reverse(out, src, n);
      n - 1
    }
  };

  let base_inc = 1 + upsample_left as c_int;
  let pad_pixels = (h + 15).max(64 - max_base_y - 1);
  let fill_pixel = out.add(max_base_y as usize).read();
  let base = out.add(max_base_y as usize + 1);
  for i in 0..(pad_pixels * base_inc) as usize {
    base.add(i).write(fill_pixel);
  }

  Z3.ipred_fill(dst, stride, out, w, h, dy, max_base_y, upsample_left);
}

#[inline(always)]
pub fn dispatch_predict_intra<T: Pixel>(
  mode: PredictionMode, variant: PredictionVariant,
  dst: &mut PlaneRegionMut<'_, T>, tx_size: TxSize, bit_depth: usize,
  ac: &[i16], angle: isize, ief_params: Option<IntraEdgeFilterParameters>,
  edge_buf: &IntraEdge<T>, cpu: CpuFeatureLevel,
) {
  let call_rust = |dst: &mut PlaneRegionMut<'_, T>| {
    rust::dispatch_predict_intra(
      mode, variant, dst, tx_size, bit_depth, ac, angle, ief_params, edge_buf,
      cpu,
    );
  };

  if cpu < CpuFeatureLevel::NEON {
    return call_rust(dst);
  }

  unsafe {
    let dst_ptr = dst.data_ptr_mut() as *mut _;
    let dst_u16 = dst.data_ptr_mut() as *mut u16;
    let stride = T::to_asm_stride(dst.plane_cfg.stride) as libc::ptrdiff_t;
    let edge_ptr = edge_buf.top_left_ptr() as *const _;
    let edge_u16 = edge_buf.top_left_ptr() as *const u16;
    let w = tx_size.width() as libc::c_int;
    let h = tx_size.height() as libc::c_int;
    let angle = angle as libc::c_int;
    let bd_max = (1 << bit_depth) - 1;
    match T::type_enum() {
      PixelType::U8 => match mode {
        PredictionMode::DC_PRED => {
          (match variant {
            PredictionVariant::NONE => rav1e_ipred_dc_128_8bpc_neon,
            PredictionVariant::LEFT => rav1e_ipred_dc_left_8bpc_neon,
            PredictionVariant::TOP => rav1e_ipred_dc_top_8bpc_neon,
            PredictionVariant::BOTH => rav1e_ipred_dc_8bpc_neon,
          })(dst_ptr, stride, edge_ptr, w, h, angle);
        }
        PredictionMode::V_PRED if angle == 90 => {
          rav1e_ipred_v_8bpc_neon(dst_ptr, stride, edge_ptr, w, h, angle);
        }
        PredictionMode::H_PRED if angle == 180 => {
          rav1e_ipred_h_8bpc_neon(dst_ptr, stride, edge_ptr, w, h, angle);
        }
        PredictionMode::SMOOTH_PRED => {
          rav1e_ipred_smooth_8bpc_neon(dst_ptr, stride, edge_ptr, w, h, angle);
        }
        PredictionMode::SMOOTH_V_PRED => {
          rav1e_ipred_smooth_v_8bpc_neon(
            dst_ptr, stride, edge_ptr, w, h, angle,
          );
        }
        PredictionMode::SMOOTH_H_PRED => {
          rav1e_ipred_smooth_h_8bpc_neon(
            dst_ptr, stride, edge_ptr, w, h, angle,
          );
        }
        PredictionMode::PAETH_PRED => {
          rav1e_ipred_paeth_8bpc_neon(dst_ptr, stride, edge_ptr, w, h, angle);
        }
        PredictionMode::UV_CFL_PRED => {
          let ac_ptr = ac.as_ptr() as *const _;
          (match variant {
            PredictionVariant::NONE => rav1e_ipred_cfl_128_8bpc_neon,
            PredictionVariant::LEFT => rav1e_ipred_cfl_left_8bpc_neon,
            PredictionVariant::TOP => rav1e_ipred_cfl_top_8bpc_neon,
            PredictionVariant::BOTH => rav1e_ipred_cfl_8bpc_neon,
          })(dst_ptr, stride, edge_ptr, w, h, ac_ptr, angle);
        }
        _ => call_rust(dst),
      },
      PixelType::U16 if bit_depth > 8 => match mode {
        PredictionMode::DC_PRED => {
          (match variant {
            PredictionVariant::NONE => rav1e_ipred_dc_128_16bpc_neon,
            PredictionVariant::LEFT => rav1e_ipred_dc_left_16bpc_neon,
            PredictionVariant::TOP => rav1e_ipred_dc_top_16bpc_neon,
            PredictionVariant::BOTH => rav1e_ipred_dc_16bpc_neon,
          })(dst_u16, stride, edge_u16, w, h, angle, 0, 0, bd_max);
        }
        PredictionMode::V_PRED if angle == 90 => {
          rav1e_ipred_v_16bpc_neon(
            dst_u16, stride, edge_u16, w, h, angle, 0, 0, bd_max,
          );
        }
        PredictionMode::H_PRED if angle == 180 => {
          rav1e_ipred_h_16bpc_neon(
            dst_u16, stride, edge_u16, w, h, angle, 0, 0, bd_max,
          );
        }
        PredictionMode::H_PRED
        | PredictionMode::V_PRED
        | PredictionMode::D45_PRED
        | PredictionMode::D135_PRED
        | PredictionMode::D113_PRED
        | PredictionMode::D157_PRED
        | PredictionMode::D203_PRED
        | PredictionMode::D67_PRED => {
          let edge_filter = ief_params.is_some();
          let smooth_filter = ief_params
            .map(IntraEdgeFilterParameters::use_smooth_filter)
            .unwrap_or_default();
          if (90..=180).contains(&angle) {
            // From dav1d, bw and bh are the frame width and height rounded to 8px units
            let (bw, bh) = (
              ((dst.plane_cfg.width + 7) >> 3) << 3,
              ((dst.plane_cfg.height + 7) >> 3) << 3,
            );
            // From dav1d, dx and dy are the distance from the predicted block to the frame edge
            let (dx, dy) = (
              (bw as isize - dst.rect().x) as libc::c_int,
              (bh as isize - dst.rect().y) as libc::c_int,
            );
            return ipred_z2(
              dst.data_ptr_mut(),
              stride,
              edge_buf.top_left_ptr(),
              angle as isize,
              w,
              h,
              dx,
              dy,
              bd_max,
              edge_filter,
              smooth_filter,
            );
          }
          (if angle < 90 { ipred_z1 } else { ipred_z3 })(
            dst.data_ptr_mut(),
            stride,
            edge_buf.top_left_ptr(),
            angle as isize,
            w,
            h,
            bd_max,
            edge_filter,
            smooth_filter,
          );
        }
        PredictionMode::SMOOTH_PRED => {
          rav1e_ipred_smooth_16bpc_neon(
            dst_u16, stride, edge_u16, w, h, angle, 0, 0, bd_max,
          );
        }
        PredictionMode::SMOOTH_V_PRED => {
          rav1e_ipred_smooth_v_16bpc_neon(
            dst_u16, stride, edge_u16, w, h, angle, 0, 0, bd_max,
          );
        }
        PredictionMode::SMOOTH_H_PRED => {
          rav1e_ipred_smooth_h_16bpc_neon(
            dst_u16, stride, edge_u16, w, h, angle, 0, 0, bd_max,
          );
        }
        PredictionMode::PAETH_PRED => {
          rav1e_ipred_paeth_16bpc_neon(
            dst_u16, stride, edge_u16, w, h, angle, 0, 0, bd_max,
          );
        }
        PredictionMode::UV_CFL_PRED => {
          let ac_ptr = ac.as_ptr() as *const _;
          (match variant {
            PredictionVariant::NONE => rav1e_ipred_cfl_128_16bpc_neon,
            PredictionVariant::LEFT => rav1e_ipred_cfl_left_16bpc_neon,
            PredictionVariant::TOP => rav1e_ipred_cfl_top_16bpc_neon,
            PredictionVariant::BOTH => rav1e_ipred_cfl_16bpc_neon,
          })(dst_u16, stride, edge_u16, w, h, ac_ptr, angle, bd_max);
        }
        _ => call_rust(dst),
      },
      _ => call_rust(dst),
    }
  }
}

/// It MUST initialize all `ac` elements.
#[inline(always)]
pub(crate) fn pred_cfl_ac<T: Pixel, const XDEC: usize, const YDEC: usize>(
  ac: &mut [MaybeUninit<i16>], luma: &PlaneRegion<'_, T>, bsize: BlockSize,
  w_pad: usize, h_pad: usize, cpu: CpuFeatureLevel,
) {
  debug_assert_eq!(ac.len(), bsize.area());

  if cpu < CpuFeatureLevel::NEON {
    return rust::pred_cfl_ac::<T, XDEC, YDEC>(
      ac, luma, bsize, w_pad, h_pad, cpu,
    );
  }

  let stride = T::to_asm_stride(luma.plane_cfg.stride) as libc::ptrdiff_t;
  let w = bsize.width() as libc::c_int;
  let h = bsize.height() as libc::c_int;
  let w_pad = w_pad as libc::c_int;
  let h_pad = h_pad as libc::c_int;

  // SAFETY: Calls Assembly code.
  unsafe {
    let ac_ptr = ac.as_mut_ptr();
    match T::type_enum() {
      PixelType::U8 => {
        let luma_ptr = luma.data_ptr() as *const u8;
        (match (XDEC, YDEC) {
          (0, 0) => rav1e_ipred_cfl_ac_444_8bpc_neon,
          (1, 0) => rav1e_ipred_cfl_ac_422_8bpc_neon,
          _ => rav1e_ipred_cfl_ac_420_8bpc_neon,
        })(ac_ptr, luma_ptr, stride, w_pad, h_pad, w, h)
      }
      PixelType::U16 => {
        let luma_ptr = luma.data_ptr() as *const u16;
        (match (XDEC, YDEC) {
          (0, 0) => rav1e_ipred_cfl_ac_444_16bpc_neon,
          (1, 0) => rav1e_ipred_cfl_ac_422_16bpc_neon,
          _ => rav1e_ipred_cfl_ac_420_16bpc_neon,
        })(ac_ptr, luma_ptr, stride, w_pad, h_pad, w, h)
      }
    }
  }
}
