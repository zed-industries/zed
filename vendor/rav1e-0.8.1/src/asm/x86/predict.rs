// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cpu_features::CpuFeatureLevel;
use crate::partition::{BlockSize, IntraEdge};
use crate::predict::{
  rust, IntraEdgeFilterParameters, PredictionMode, PredictionVariant,
};
use crate::tiling::{PlaneRegion, PlaneRegionMut};
use crate::transform::TxSize;
use crate::Pixel;
use std::mem::MaybeUninit;
use v_frame::pixel::PixelType;

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
  rav1e_ipred_h_8bpc_ssse3,
  rav1e_ipred_h_8bpc_avx2,
  rav1e_ipred_h_8bpc_avx512icl,
  rav1e_ipred_v_8bpc_ssse3,
  rav1e_ipred_v_8bpc_avx2,
  rav1e_ipred_v_8bpc_avx512icl,
  rav1e_ipred_dc_8bpc_ssse3,
  rav1e_ipred_dc_8bpc_avx2,
  rav1e_ipred_dc_8bpc_avx512icl,
  rav1e_ipred_dc_left_8bpc_ssse3,
  rav1e_ipred_dc_left_8bpc_avx2,
  rav1e_ipred_dc_left_8bpc_avx512icl,
  rav1e_ipred_dc_128_8bpc_ssse3,
  rav1e_ipred_dc_128_8bpc_avx2,
  rav1e_ipred_dc_128_8bpc_avx512icl,
  rav1e_ipred_dc_top_8bpc_ssse3,
  rav1e_ipred_dc_top_8bpc_avx2,
  rav1e_ipred_dc_top_8bpc_avx512icl,
  rav1e_ipred_smooth_v_8bpc_ssse3,
  rav1e_ipred_smooth_v_8bpc_avx2,
  rav1e_ipred_smooth_v_8bpc_avx512icl,
  rav1e_ipred_smooth_h_8bpc_ssse3,
  rav1e_ipred_smooth_h_8bpc_avx2,
  rav1e_ipred_smooth_h_8bpc_avx512icl,
  rav1e_ipred_smooth_8bpc_ssse3,
  rav1e_ipred_smooth_8bpc_avx2,
  rav1e_ipred_smooth_8bpc_avx512icl,
  rav1e_ipred_z1_8bpc_ssse3,
  rav1e_ipred_z1_8bpc_avx2,
  rav1e_ipred_z3_8bpc_ssse3,
  rav1e_ipred_z3_8bpc_avx2,
  rav1e_ipred_paeth_8bpc_ssse3,
  rav1e_ipred_paeth_8bpc_avx2,
  rav1e_ipred_paeth_8bpc_avx512icl
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
  rav1e_ipred_h_16bpc_ssse3,
  rav1e_ipred_h_16bpc_avx2,
  rav1e_ipred_v_16bpc_ssse3,
  rav1e_ipred_v_16bpc_avx2,
  rav1e_ipred_dc_16bpc_ssse3,
  rav1e_ipred_dc_16bpc_avx2,
  rav1e_ipred_dc_left_16bpc_ssse3,
  rav1e_ipred_dc_left_16bpc_avx2,
  rav1e_ipred_dc_128_16bpc_ssse3,
  rav1e_ipred_dc_128_16bpc_avx2,
  rav1e_ipred_dc_top_16bpc_ssse3,
  rav1e_ipred_dc_top_16bpc_avx2,
  rav1e_ipred_smooth_v_16bpc_ssse3,
  rav1e_ipred_smooth_v_16bpc_avx2,
  rav1e_ipred_smooth_v_16bpc_avx512icl,
  rav1e_ipred_smooth_h_16bpc_ssse3,
  rav1e_ipred_smooth_h_16bpc_avx2,
  rav1e_ipred_smooth_h_16bpc_avx512icl,
  rav1e_ipred_smooth_16bpc_ssse3,
  rav1e_ipred_smooth_16bpc_avx2,
  rav1e_ipred_smooth_16bpc_avx512icl,
  rav1e_ipred_z1_16bpc_ssse3,
  rav1e_ipred_z1_16bpc_avx2,
  rav1e_ipred_z2_16bpc_ssse3,
  rav1e_ipred_z3_16bpc_ssse3,
  rav1e_ipred_z3_16bpc_avx2,
  rav1e_ipred_paeth_16bpc_ssse3,
  rav1e_ipred_paeth_16bpc_avx2,
  rav1e_ipred_paeth_16bpc_avx512icl
}

// For z2 prediction, we need to provide extra parameters, dx and dy, which indicate
// the distance between the predicted block's top-left pixel and the frame's edge.
// It is required for the intra edge filtering process.
extern {

  fn rav1e_ipred_z2_8bpc_ssse3(
    dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
    width: libc::c_int, height: libc::c_int, angle: libc::c_int,
    dx: libc::c_int, dy: libc::c_int,
  );

  fn rav1e_ipred_z2_8bpc_avx2(
    dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
    width: libc::c_int, height: libc::c_int, angle: libc::c_int,
    dx: libc::c_int, dy: libc::c_int,
  );

  fn rav1e_ipred_z2_16bpc_avx2(
    dst: *mut u16, stride: libc::ptrdiff_t, topleft: *const u16,
    width: libc::c_int, height: libc::c_int, angle: libc::c_int,
    dx: libc::c_int, dy: libc::c_int, bit_depth_max: libc::c_int,
  );
}

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
  rav1e_ipred_cfl_ac_420_8bpc_avx2,
  rav1e_ipred_cfl_ac_420_8bpc_ssse3,
  rav1e_ipred_cfl_ac_422_8bpc_avx2,
  rav1e_ipred_cfl_ac_422_8bpc_ssse3,
  rav1e_ipred_cfl_ac_444_8bpc_avx2,
  rav1e_ipred_cfl_ac_444_8bpc_ssse3
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
  rav1e_ipred_cfl_ac_420_16bpc_ssse3,
  rav1e_ipred_cfl_ac_420_16bpc_avx2,
  rav1e_ipred_cfl_ac_422_16bpc_ssse3,
  rav1e_ipred_cfl_ac_422_16bpc_avx2,
  rav1e_ipred_cfl_ac_444_16bpc_ssse3,
  rav1e_ipred_cfl_ac_444_16bpc_avx2
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
  rav1e_ipred_cfl_8bpc_ssse3,
  rav1e_ipred_cfl_8bpc_avx2,
  rav1e_ipred_cfl_left_8bpc_ssse3,
  rav1e_ipred_cfl_left_8bpc_avx2,
  rav1e_ipred_cfl_top_8bpc_ssse3,
  rav1e_ipred_cfl_top_8bpc_avx2,
  rav1e_ipred_cfl_128_8bpc_ssse3,
  rav1e_ipred_cfl_128_8bpc_avx2
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
  rav1e_ipred_cfl_16bpc_ssse3,
  rav1e_ipred_cfl_16bpc_avx2,
  rav1e_ipred_cfl_128_16bpc_ssse3,
  rav1e_ipred_cfl_128_16bpc_avx2,
  rav1e_ipred_cfl_left_16bpc_ssse3,
  rav1e_ipred_cfl_left_16bpc_avx2,
  rav1e_ipred_cfl_top_16bpc_ssse3,
  rav1e_ipred_cfl_top_16bpc_avx2
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

  // SAFETY: Calls Assembly code.
  unsafe {
    let stride = T::to_asm_stride(dst.plane_cfg.stride) as libc::ptrdiff_t;
    let w = tx_size.width() as libc::c_int;
    let h = tx_size.height() as libc::c_int;
    let angle = angle as libc::c_int;

    match T::type_enum() {
      PixelType::U8 => {
        let dst_ptr = dst.data_ptr_mut() as *mut _;
        let edge_ptr = edge_buf.top_left_ptr() as *const _;
        if cpu >= CpuFeatureLevel::AVX512ICL {
          match mode {
            PredictionMode::DC_PRED => {
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_dc_128_8bpc_avx512icl,
                PredictionVariant::LEFT => rav1e_ipred_dc_left_8bpc_avx512icl,
                PredictionVariant::TOP => rav1e_ipred_dc_top_8bpc_avx512icl,
                PredictionVariant::BOTH => rav1e_ipred_dc_8bpc_avx512icl,
              })(dst_ptr, stride, edge_ptr, w, h, angle);
            }
            PredictionMode::V_PRED if angle == 90 => {
              rav1e_ipred_v_8bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::H_PRED if angle == 180 => {
              rav1e_ipred_h_8bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::V_PRED
            | PredictionMode::H_PRED
            | PredictionMode::D45_PRED
            | PredictionMode::D135_PRED
            | PredictionMode::D113_PRED
            | PredictionMode::D157_PRED
            | PredictionMode::D203_PRED
            | PredictionMode::D67_PRED => {
              let (enable_ief, ief_smooth_filter) =
                if let Some(params) = ief_params {
                  (
                    true as libc::c_int,
                    params.use_smooth_filter() as libc::c_int,
                  )
                } else {
                  (false as libc::c_int, false as libc::c_int)
                };

              // dav1d assembly uses the unused integer bits to hold IEF parameters
              let angle_arg =
                angle | (enable_ief << 10) | (ief_smooth_filter << 9);

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

              if angle <= 90 {
                rav1e_ipred_z1_8bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg,
                );
              } else if angle < 180 {
                rav1e_ipred_z2_8bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, dx, dy,
                );
              } else {
                rav1e_ipred_z3_8bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg,
                );
              }
            }
            PredictionMode::SMOOTH_PRED => {
              rav1e_ipred_smooth_8bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::SMOOTH_V_PRED => {
              rav1e_ipred_smooth_v_8bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::SMOOTH_H_PRED => {
              rav1e_ipred_smooth_h_8bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::PAETH_PRED => {
              rav1e_ipred_paeth_8bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::UV_CFL_PRED => {
              let ac_ptr = ac.as_ptr() as *const _;
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_cfl_128_8bpc_avx2,
                PredictionVariant::LEFT => rav1e_ipred_cfl_left_8bpc_avx2,
                PredictionVariant::TOP => rav1e_ipred_cfl_top_8bpc_avx2,
                PredictionVariant::BOTH => rav1e_ipred_cfl_8bpc_avx2,
              })(dst_ptr, stride, edge_ptr, w, h, ac_ptr, angle);
            }
            _ => call_rust(dst),
          }
        } else if cpu >= CpuFeatureLevel::AVX2 {
          match mode {
            PredictionMode::DC_PRED => {
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_dc_128_8bpc_avx2,
                PredictionVariant::LEFT => rav1e_ipred_dc_left_8bpc_avx2,
                PredictionVariant::TOP => rav1e_ipred_dc_top_8bpc_avx2,
                PredictionVariant::BOTH => rav1e_ipred_dc_8bpc_avx2,
              })(dst_ptr, stride, edge_ptr, w, h, angle);
            }
            PredictionMode::V_PRED if angle == 90 => {
              rav1e_ipred_v_8bpc_avx2(dst_ptr, stride, edge_ptr, w, h, angle);
            }
            PredictionMode::H_PRED if angle == 180 => {
              rav1e_ipred_h_8bpc_avx2(dst_ptr, stride, edge_ptr, w, h, angle);
            }
            PredictionMode::V_PRED
            | PredictionMode::H_PRED
            | PredictionMode::D45_PRED
            | PredictionMode::D135_PRED
            | PredictionMode::D113_PRED
            | PredictionMode::D157_PRED
            | PredictionMode::D203_PRED
            | PredictionMode::D67_PRED => {
              let (enable_ief, ief_smooth_filter) =
                if let Some(params) = ief_params {
                  (
                    true as libc::c_int,
                    params.use_smooth_filter() as libc::c_int,
                  )
                } else {
                  (false as libc::c_int, false as libc::c_int)
                };

              // dav1d assembly uses the unused integer bits to hold IEF parameters
              let angle_arg =
                angle | (enable_ief << 10) | (ief_smooth_filter << 9);

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

              if angle <= 90 {
                rav1e_ipred_z1_8bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg,
                );
              } else if angle < 180 {
                rav1e_ipred_z2_8bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, dx, dy,
                );
              } else {
                rav1e_ipred_z3_8bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg,
                );
              }
            }
            PredictionMode::SMOOTH_PRED => {
              rav1e_ipred_smooth_8bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::SMOOTH_V_PRED => {
              rav1e_ipred_smooth_v_8bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::SMOOTH_H_PRED => {
              rav1e_ipred_smooth_h_8bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::PAETH_PRED => {
              rav1e_ipred_paeth_8bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::UV_CFL_PRED => {
              let ac_ptr = ac.as_ptr() as *const _;
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_cfl_128_8bpc_avx2,
                PredictionVariant::LEFT => rav1e_ipred_cfl_left_8bpc_avx2,
                PredictionVariant::TOP => rav1e_ipred_cfl_top_8bpc_avx2,
                PredictionVariant::BOTH => rav1e_ipred_cfl_8bpc_avx2,
              })(dst_ptr, stride, edge_ptr, w, h, ac_ptr, angle);
            }
            _ => call_rust(dst),
          }
        } else if cpu >= CpuFeatureLevel::SSSE3 {
          match mode {
            PredictionMode::DC_PRED => {
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_dc_128_8bpc_ssse3,
                PredictionVariant::LEFT => rav1e_ipred_dc_left_8bpc_ssse3,
                PredictionVariant::TOP => rav1e_ipred_dc_top_8bpc_ssse3,
                PredictionVariant::BOTH => rav1e_ipred_dc_8bpc_ssse3,
              })(dst_ptr, stride, edge_ptr, w, h, angle);
            }
            PredictionMode::V_PRED if angle == 90 => {
              rav1e_ipred_v_8bpc_ssse3(dst_ptr, stride, edge_ptr, w, h, angle);
            }
            PredictionMode::H_PRED if angle == 180 => {
              rav1e_ipred_h_8bpc_ssse3(dst_ptr, stride, edge_ptr, w, h, angle);
            }
            PredictionMode::V_PRED
            | PredictionMode::H_PRED
            | PredictionMode::D45_PRED
            | PredictionMode::D135_PRED
            | PredictionMode::D113_PRED
            | PredictionMode::D157_PRED
            | PredictionMode::D203_PRED
            | PredictionMode::D67_PRED => {
              let (enable_ief, ief_smooth_filter) =
                if let Some(params) = ief_params {
                  (
                    true as libc::c_int,
                    params.use_smooth_filter() as libc::c_int,
                  )
                } else {
                  (false as libc::c_int, false as libc::c_int)
                };

              // dav1d assembly uses the unused integer bits to hold IEF parameters
              let angle_arg =
                angle | (enable_ief << 10) | (ief_smooth_filter << 9);

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

              if angle <= 90 {
                rav1e_ipred_z1_8bpc_ssse3(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg,
                );
              } else if angle < 180 {
                rav1e_ipred_z2_8bpc_ssse3(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, dx, dy,
                );
              } else {
                rav1e_ipred_z3_8bpc_ssse3(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg,
                );
              }
            }
            PredictionMode::SMOOTH_PRED => {
              rav1e_ipred_smooth_8bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::SMOOTH_V_PRED => {
              rav1e_ipred_smooth_v_8bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::SMOOTH_H_PRED => {
              rav1e_ipred_smooth_h_8bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::PAETH_PRED => {
              rav1e_ipred_paeth_8bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle,
              );
            }
            PredictionMode::UV_CFL_PRED => {
              let ac_ptr = ac.as_ptr() as *const _;
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_cfl_128_8bpc_ssse3,
                PredictionVariant::LEFT => rav1e_ipred_cfl_left_8bpc_ssse3,
                PredictionVariant::TOP => rav1e_ipred_cfl_top_8bpc_ssse3,
                PredictionVariant::BOTH => rav1e_ipred_cfl_8bpc_ssse3,
              })(dst_ptr, stride, edge_ptr, w, h, ac_ptr, angle);
            }
            _ => call_rust(dst),
          }
        } else {
          call_rust(dst)
        }
      }
      PixelType::U16 => {
        let dst_ptr = dst.data_ptr_mut() as *mut _;
        let edge_ptr = edge_buf.top_left_ptr() as *const _;
        let bd_max = (1 << bit_depth) - 1;
        if cpu >= CpuFeatureLevel::AVX512ICL {
          match mode {
            PredictionMode::DC_PRED => {
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_dc_128_16bpc_avx2,
                PredictionVariant::LEFT => rav1e_ipred_dc_left_16bpc_avx2,
                PredictionVariant::TOP => rav1e_ipred_dc_top_16bpc_avx2,
                PredictionVariant::BOTH => rav1e_ipred_dc_16bpc_avx2,
              })(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max
              );
            }
            PredictionMode::V_PRED if angle == 90 => {
              rav1e_ipred_v_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::H_PRED if angle == 180 => {
              rav1e_ipred_h_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::V_PRED
            | PredictionMode::H_PRED
            | PredictionMode::D45_PRED
            | PredictionMode::D135_PRED
            | PredictionMode::D113_PRED
            | PredictionMode::D157_PRED
            | PredictionMode::D203_PRED
            | PredictionMode::D67_PRED => {
              let (enable_ief, ief_smooth_filter) =
                if let Some(params) = ief_params {
                  (
                    true as libc::c_int,
                    params.use_smooth_filter() as libc::c_int,
                  )
                } else {
                  (false as libc::c_int, false as libc::c_int)
                };

              // dav1d assembly uses the unused integer bits to hold IEF parameters
              let angle_arg =
                angle | (enable_ief << 10) | (ief_smooth_filter << 9);

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

              if angle <= 90 {
                rav1e_ipred_z1_16bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, 0, 0, bd_max,
                );
              } else if angle < 180 {
                rav1e_ipred_z2_16bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, dx, dy, bd_max,
                );
              } else {
                rav1e_ipred_z3_16bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, 0, 0, bd_max,
                );
              }
            }
            PredictionMode::SMOOTH_PRED => {
              rav1e_ipred_smooth_16bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::SMOOTH_V_PRED => {
              rav1e_ipred_smooth_v_16bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::SMOOTH_H_PRED => {
              rav1e_ipred_smooth_h_16bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::PAETH_PRED => {
              rav1e_ipred_paeth_16bpc_avx512icl(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::UV_CFL_PRED => {
              let ac_ptr = ac.as_ptr() as *const _;
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_cfl_128_16bpc_avx2,
                PredictionVariant::LEFT => rav1e_ipred_cfl_left_16bpc_avx2,
                PredictionVariant::TOP => rav1e_ipred_cfl_top_16bpc_avx2,
                PredictionVariant::BOTH => rav1e_ipred_cfl_16bpc_avx2,
              })(
                dst_ptr, stride, edge_ptr, w, h, ac_ptr, angle, bd_max
              );
            }
            _ => call_rust(dst),
          }
        } else if cpu >= CpuFeatureLevel::AVX2 {
          match mode {
            PredictionMode::DC_PRED => {
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_dc_128_16bpc_avx2,
                PredictionVariant::LEFT => rav1e_ipred_dc_left_16bpc_avx2,
                PredictionVariant::TOP => rav1e_ipred_dc_top_16bpc_avx2,
                PredictionVariant::BOTH => rav1e_ipred_dc_16bpc_avx2,
              })(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max
              );
            }
            PredictionMode::V_PRED if angle == 90 => {
              rav1e_ipred_v_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::H_PRED if angle == 180 => {
              rav1e_ipred_h_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::V_PRED
            | PredictionMode::H_PRED
            | PredictionMode::D45_PRED
            | PredictionMode::D135_PRED
            | PredictionMode::D113_PRED
            | PredictionMode::D157_PRED
            | PredictionMode::D203_PRED
            | PredictionMode::D67_PRED => {
              let (enable_ief, ief_smooth_filter) =
                if let Some(params) = ief_params {
                  (
                    true as libc::c_int,
                    params.use_smooth_filter() as libc::c_int,
                  )
                } else {
                  (false as libc::c_int, false as libc::c_int)
                };

              // dav1d assembly uses the unused integer bits to hold IEF parameters
              let angle_arg =
                angle | (enable_ief << 10) | (ief_smooth_filter << 9);

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

              if angle <= 90 {
                rav1e_ipred_z1_16bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, 0, 0, bd_max,
                );
              } else if angle < 180 {
                rav1e_ipred_z2_16bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, dx, dy, bd_max,
                );
              } else {
                rav1e_ipred_z3_16bpc_avx2(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, 0, 0, bd_max,
                );
              }
            }
            PredictionMode::SMOOTH_PRED => {
              rav1e_ipred_smooth_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::SMOOTH_V_PRED => {
              rav1e_ipred_smooth_v_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::SMOOTH_H_PRED => {
              rav1e_ipred_smooth_h_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::PAETH_PRED => {
              rav1e_ipred_paeth_16bpc_avx2(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::UV_CFL_PRED => {
              let ac_ptr = ac.as_ptr() as *const _;
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_cfl_128_16bpc_avx2,
                PredictionVariant::LEFT => rav1e_ipred_cfl_left_16bpc_avx2,
                PredictionVariant::TOP => rav1e_ipred_cfl_top_16bpc_avx2,
                PredictionVariant::BOTH => rav1e_ipred_cfl_16bpc_avx2,
              })(
                dst_ptr, stride, edge_ptr, w, h, ac_ptr, angle, bd_max
              );
            }
            _ => call_rust(dst),
          }
        } else if cpu >= CpuFeatureLevel::SSSE3 {
          match mode {
            PredictionMode::DC_PRED => {
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_dc_128_16bpc_ssse3,
                PredictionVariant::LEFT => rav1e_ipred_dc_left_16bpc_ssse3,
                PredictionVariant::TOP => rav1e_ipred_dc_top_16bpc_ssse3,
                PredictionVariant::BOTH => rav1e_ipred_dc_16bpc_ssse3,
              })(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max
              );
            }
            PredictionMode::V_PRED if angle == 90 => {
              rav1e_ipred_v_16bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::H_PRED if angle == 180 => {
              rav1e_ipred_h_16bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::V_PRED
            | PredictionMode::H_PRED
            | PredictionMode::D45_PRED
            | PredictionMode::D135_PRED
            | PredictionMode::D113_PRED
            | PredictionMode::D157_PRED
            | PredictionMode::D203_PRED
            | PredictionMode::D67_PRED => {
              let (enable_ief, ief_smooth_filter) =
                if let Some(params) = ief_params {
                  (
                    true as libc::c_int,
                    params.use_smooth_filter() as libc::c_int,
                  )
                } else {
                  (false as libc::c_int, false as libc::c_int)
                };

              // dav1d assembly uses the unused integer bits to hold IEF parameters
              let angle_arg =
                angle | (enable_ief << 10) | (ief_smooth_filter << 9);

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

              if angle <= 90 {
                rav1e_ipred_z1_16bpc_ssse3(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, 0, 0, bd_max,
                );
              } else if angle < 180 {
                rav1e_ipred_z2_16bpc_ssse3(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, dx, dy, bd_max,
                );
              } else {
                rav1e_ipred_z3_16bpc_ssse3(
                  dst_ptr, stride, edge_ptr, w, h, angle_arg, 0, 0, bd_max,
                );
              }
            }
            PredictionMode::SMOOTH_PRED => {
              rav1e_ipred_smooth_16bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::SMOOTH_V_PRED => {
              rav1e_ipred_smooth_v_16bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::SMOOTH_H_PRED => {
              rav1e_ipred_smooth_h_16bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::PAETH_PRED => {
              rav1e_ipred_paeth_16bpc_ssse3(
                dst_ptr, stride, edge_ptr, w, h, angle, 0, 0, bd_max,
              );
            }
            PredictionMode::UV_CFL_PRED => {
              let ac_ptr = ac.as_ptr() as *const _;
              (match variant {
                PredictionVariant::NONE => rav1e_ipred_cfl_128_16bpc_ssse3,
                PredictionVariant::LEFT => rav1e_ipred_cfl_left_16bpc_ssse3,
                PredictionVariant::TOP => rav1e_ipred_cfl_top_16bpc_ssse3,
                PredictionVariant::BOTH => rav1e_ipred_cfl_16bpc_ssse3,
              })(
                dst_ptr, stride, edge_ptr, w, h, ac_ptr, angle, bd_max
              );
            }
            _ => call_rust(dst),
          }
        } else {
          call_rust(dst)
        }
      }
    }
  }
}

// The implementation MUST inititialize all `ac` elements
#[inline(always)]
pub(crate) fn pred_cfl_ac<T: Pixel, const XDEC: usize, const YDEC: usize>(
  ac: &mut [MaybeUninit<i16>], luma: &PlaneRegion<'_, T>, bsize: BlockSize,
  w_pad: usize, h_pad: usize, cpu: CpuFeatureLevel,
) {
  debug_assert_eq!(ac.len(), bsize.area());

  let call_rust = |ac: &mut [MaybeUninit<i16>]| {
    rust::pred_cfl_ac::<T, XDEC, YDEC>(ac, luma, bsize, w_pad, h_pad, cpu);
  };

  let stride = T::to_asm_stride(luma.plane_cfg.stride) as libc::ptrdiff_t;
  let w = bsize.width() as libc::c_int;
  let h = bsize.height() as libc::c_int;
  let w_pad = w_pad as libc::c_int;
  let h_pad = h_pad as libc::c_int;

  // SAFETY: Calls Assembly code.
  unsafe {
    let ac_ptr = ac.as_mut_ptr();
    match T::type_enum() {
      PixelType::U8 if cpu >= CpuFeatureLevel::SSSE3 => {
        let luma_ptr = luma.data_ptr() as *const u8;
        (if cpu >= CpuFeatureLevel::AVX2 {
          match (XDEC, YDEC) {
            (0, 0) => rav1e_ipred_cfl_ac_444_8bpc_avx2,
            (1, 0) => rav1e_ipred_cfl_ac_422_8bpc_avx2,
            _ => rav1e_ipred_cfl_ac_420_8bpc_avx2,
          }
        } else {
          match (XDEC, YDEC) {
            (0, 0) => rav1e_ipred_cfl_ac_444_8bpc_ssse3,
            (1, 0) => rav1e_ipred_cfl_ac_422_8bpc_ssse3,
            _ => rav1e_ipred_cfl_ac_420_8bpc_ssse3,
          }
        })(ac_ptr, luma_ptr, stride, w_pad, h_pad, w, h)
      }
      PixelType::U16 if cpu >= CpuFeatureLevel::SSSE3 => {
        let luma_ptr = luma.data_ptr() as *const u16;
        (if cpu >= CpuFeatureLevel::AVX2 {
          match (XDEC, YDEC) {
            (0, 0) => rav1e_ipred_cfl_ac_444_16bpc_avx2,
            (1, 0) => rav1e_ipred_cfl_ac_422_16bpc_avx2,
            _ => rav1e_ipred_cfl_ac_420_16bpc_avx2,
          }
        } else {
          match (XDEC, YDEC) {
            (0, 0) => rav1e_ipred_cfl_ac_444_16bpc_ssse3,
            (1, 0) => rav1e_ipred_cfl_ac_422_16bpc_ssse3,
            _ => rav1e_ipred_cfl_ac_420_16bpc_ssse3,
          }
        })(ac_ptr, luma_ptr, stride, w_pad, h_pad, w, h)
      }
      _ => call_rust(ac),
    }
  }
}
