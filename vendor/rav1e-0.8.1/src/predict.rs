// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem::MaybeUninit;

cfg_if::cfg_if! {
  if #[cfg(nasm_x86_64)] {
    pub use crate::asm::x86::predict::*;
  } else if #[cfg(asm_neon)] {
    pub use crate::asm::aarch64::predict::*;
  } else {
    pub use self::rust::*;
  }
}

use aligned_vec::{avec, ABox};

use crate::context::{TileBlockOffset, MAX_SB_SIZE_LOG2, MAX_TX_SIZE};
use crate::cpu_features::CpuFeatureLevel;
use crate::encoder::FrameInvariants;
use crate::frame::*;
use crate::mc::*;
use crate::partition::*;
use crate::tiling::*;
use crate::transform::*;
use crate::util::*;

pub const ANGLE_STEP: i8 = 3;

// TODO: Review the order of this list.
// The order impacts compression efficiency.
pub static RAV1E_INTRA_MODES: &[PredictionMode] = &[
  PredictionMode::DC_PRED,
  PredictionMode::H_PRED,
  PredictionMode::V_PRED,
  PredictionMode::SMOOTH_PRED,
  PredictionMode::SMOOTH_H_PRED,
  PredictionMode::SMOOTH_V_PRED,
  PredictionMode::PAETH_PRED,
  PredictionMode::D45_PRED,
  PredictionMode::D135_PRED,
  PredictionMode::D113_PRED,
  PredictionMode::D157_PRED,
  PredictionMode::D203_PRED,
  PredictionMode::D67_PRED,
];

pub static RAV1E_INTER_MODES_MINIMAL: &[PredictionMode] =
  &[PredictionMode::NEARESTMV];

pub static RAV1E_INTER_COMPOUND_MODES: &[PredictionMode] = &[
  PredictionMode::GLOBAL_GLOBALMV,
  PredictionMode::NEAREST_NEARESTMV,
  PredictionMode::NEW_NEWMV,
  PredictionMode::NEAREST_NEWMV,
  PredictionMode::NEW_NEARESTMV,
  PredictionMode::NEAR_NEAR0MV,
  PredictionMode::NEAR_NEAR1MV,
  PredictionMode::NEAR_NEAR2MV,
];

// There are more modes than in the spec because every allowed
// drl index for NEAR modes is considered its own mode.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Default)]
pub enum PredictionMode {
  #[default]
  DC_PRED, // Average of above and left pixels
  V_PRED,      // Vertical
  H_PRED,      // Horizontal
  D45_PRED,    // Directional 45  degree
  D135_PRED,   // Directional 135 degree
  D113_PRED,   // Directional 113 degree
  D157_PRED,   // Directional 157 degree
  D203_PRED,   // Directional 203 degree
  D67_PRED,    // Directional 67  degree
  SMOOTH_PRED, // Combination of horizontal and vertical interpolation
  SMOOTH_V_PRED,
  SMOOTH_H_PRED,
  PAETH_PRED,
  UV_CFL_PRED,
  NEARESTMV,
  NEAR0MV,
  NEAR1MV,
  NEAR2MV,
  GLOBALMV,
  NEWMV,
  // Compound ref compound modes
  NEAREST_NEARESTMV,
  NEAR_NEAR0MV,
  NEAR_NEAR1MV,
  NEAR_NEAR2MV,
  NEAREST_NEWMV,
  NEW_NEARESTMV,
  NEAR_NEW0MV,
  NEAR_NEW1MV,
  NEAR_NEW2MV,
  NEW_NEAR0MV,
  NEW_NEAR1MV,
  NEW_NEAR2MV,
  GLOBAL_GLOBALMV,
  NEW_NEWMV,
}

// This is a higher number than in the spec and cannot be used
// for bitstream writing purposes.
pub const PREDICTION_MODES: usize = 34;

#[derive(Copy, Clone, Debug)]
pub enum PredictionVariant {
  NONE,
  LEFT,
  TOP,
  BOTH,
}

impl PredictionVariant {
  #[inline]
  const fn new(x: usize, y: usize) -> Self {
    match (x, y) {
      (0, 0) => PredictionVariant::NONE,
      (_, 0) => PredictionVariant::LEFT,
      (0, _) => PredictionVariant::TOP,
      _ => PredictionVariant::BOTH,
    }
  }
}

pub const fn intra_mode_to_angle(mode: PredictionMode) -> isize {
  match mode {
    PredictionMode::V_PRED => 90,
    PredictionMode::H_PRED => 180,
    PredictionMode::D45_PRED => 45,
    PredictionMode::D135_PRED => 135,
    PredictionMode::D113_PRED => 113,
    PredictionMode::D157_PRED => 157,
    PredictionMode::D203_PRED => 203,
    PredictionMode::D67_PRED => 67,
    _ => 0,
  }
}

impl PredictionMode {
  #[inline]
  pub fn is_compound(self) -> bool {
    self >= PredictionMode::NEAREST_NEARESTMV
  }
  #[inline]
  pub fn has_nearmv(self) -> bool {
    self == PredictionMode::NEAR0MV
      || self == PredictionMode::NEAR1MV
      || self == PredictionMode::NEAR2MV
      || self == PredictionMode::NEAR_NEAR0MV
      || self == PredictionMode::NEAR_NEAR1MV
      || self == PredictionMode::NEAR_NEAR2MV
      || self == PredictionMode::NEAR_NEW0MV
      || self == PredictionMode::NEAR_NEW1MV
      || self == PredictionMode::NEAR_NEW2MV
      || self == PredictionMode::NEW_NEAR0MV
      || self == PredictionMode::NEW_NEAR1MV
      || self == PredictionMode::NEW_NEAR2MV
  }
  #[inline]
  pub fn has_newmv(self) -> bool {
    self == PredictionMode::NEWMV
      || self == PredictionMode::NEW_NEWMV
      || self == PredictionMode::NEAREST_NEWMV
      || self == PredictionMode::NEW_NEARESTMV
      || self == PredictionMode::NEAR_NEW0MV
      || self == PredictionMode::NEAR_NEW1MV
      || self == PredictionMode::NEAR_NEW2MV
      || self == PredictionMode::NEW_NEAR0MV
      || self == PredictionMode::NEW_NEAR1MV
      || self == PredictionMode::NEW_NEAR2MV
  }
  #[inline]
  pub fn ref_mv_idx(self) -> usize {
    if self == PredictionMode::NEAR0MV
      || self == PredictionMode::NEAR1MV
      || self == PredictionMode::NEAR2MV
    {
      self as usize - PredictionMode::NEAR0MV as usize + 1
    } else if self == PredictionMode::NEAR_NEAR0MV
      || self == PredictionMode::NEAR_NEAR1MV
      || self == PredictionMode::NEAR_NEAR2MV
    {
      self as usize - PredictionMode::NEAR_NEAR0MV as usize + 1
    } else {
      1
    }
  }

  /// # Panics
  ///
  /// - If called on an inter `PredictionMode`
  pub fn predict_intra<T: Pixel>(
    self, tile_rect: TileRect, dst: &mut PlaneRegionMut<'_, T>,
    tx_size: TxSize, bit_depth: usize, ac: &[i16], intra_param: IntraParam,
    ief_params: Option<IntraEdgeFilterParameters>, edge_buf: &IntraEdge<T>,
    cpu: CpuFeatureLevel,
  ) {
    assert!(self.is_intra());
    let &Rect { x: frame_x, y: frame_y, .. } = dst.rect();
    debug_assert!(frame_x >= 0 && frame_y >= 0);
    // x and y are expressed relative to the tile
    let x = frame_x as usize - tile_rect.x;
    let y = frame_y as usize - tile_rect.y;

    let variant = PredictionVariant::new(x, y);

    let alpha = match intra_param {
      IntraParam::Alpha(val) => val,
      _ => 0,
    };
    let angle_delta = match intra_param {
      IntraParam::AngleDelta(val) => val,
      _ => 0,
    };

    let mode = match self {
      PredictionMode::PAETH_PRED => match variant {
        PredictionVariant::NONE => PredictionMode::DC_PRED,
        PredictionVariant::TOP => PredictionMode::V_PRED,
        PredictionVariant::LEFT => PredictionMode::H_PRED,
        PredictionVariant::BOTH => PredictionMode::PAETH_PRED,
      },
      PredictionMode::UV_CFL_PRED if alpha == 0 => PredictionMode::DC_PRED,
      _ => self,
    };

    let angle = match mode {
      PredictionMode::UV_CFL_PRED => alpha as isize,
      _ => intra_mode_to_angle(mode) + (angle_delta * ANGLE_STEP) as isize,
    };

    dispatch_predict_intra::<T>(
      mode, variant, dst, tx_size, bit_depth, ac, angle, ief_params, edge_buf,
      cpu,
    );
  }

  #[inline]
  pub fn is_intra(self) -> bool {
    self < PredictionMode::NEARESTMV
  }

  #[inline]
  pub fn is_cfl(self) -> bool {
    self == PredictionMode::UV_CFL_PRED
  }

  #[inline]
  pub fn is_directional(self) -> bool {
    self >= PredictionMode::V_PRED && self <= PredictionMode::D67_PRED
  }

  #[inline(always)]
  pub const fn angle_delta_count(self) -> i8 {
    match self {
      PredictionMode::V_PRED
      | PredictionMode::H_PRED
      | PredictionMode::D45_PRED
      | PredictionMode::D135_PRED
      | PredictionMode::D113_PRED
      | PredictionMode::D157_PRED
      | PredictionMode::D203_PRED
      | PredictionMode::D67_PRED => 7,
      _ => 1,
    }
  }

  // Used by inter prediction to extract the fractional component of a mv and
  // obtain the correct PlaneSlice to operate on.
  #[inline]
  fn get_mv_params<T: Pixel>(
    rec_plane: &Plane<T>, po: PlaneOffset, mv: MotionVector,
  ) -> (i32, i32, PlaneSlice<T>) {
    let &PlaneConfig { xdec, ydec, .. } = &rec_plane.cfg;
    let row_offset = mv.row as i32 >> (3 + ydec);
    let col_offset = mv.col as i32 >> (3 + xdec);
    let row_frac = ((mv.row as i32) << (1 - ydec)) & 0xf;
    let col_frac = ((mv.col as i32) << (1 - xdec)) & 0xf;
    let qo = PlaneOffset {
      x: po.x + col_offset as isize - 3,
      y: po.y + row_offset as isize - 3,
    };
    (row_frac, col_frac, rec_plane.slice(qo).clamp().subslice(3, 3))
  }

  /// Inter prediction with a single reference (i.e. not compound mode)
  ///
  /// # Panics
  ///
  /// - If called on an intra `PredictionMode`
  pub fn predict_inter_single<T: Pixel>(
    self, fi: &FrameInvariants<T>, tile_rect: TileRect, p: usize,
    po: PlaneOffset, dst: &mut PlaneRegionMut<'_, T>, width: usize,
    height: usize, ref_frame: RefType, mv: MotionVector,
  ) {
    assert!(!self.is_intra());
    let frame_po = tile_rect.to_frame_plane_offset(po);

    let mode = fi.default_filter;

    if let Some(ref rec) =
      fi.rec_buffer.frames[fi.ref_frames[ref_frame.to_index()] as usize]
    {
      let (row_frac, col_frac, src) =
        PredictionMode::get_mv_params(&rec.frame.planes[p], frame_po, mv);
      put_8tap(
        dst,
        src,
        width,
        height,
        col_frac,
        row_frac,
        mode,
        mode,
        fi.sequence.bit_depth,
        fi.cpu_feature_level,
      );
    }
  }

  /// Inter prediction with two references.
  ///
  /// # Panics
  ///
  /// - If called on an intra `PredictionMode`
  pub fn predict_inter_compound<T: Pixel>(
    self, fi: &FrameInvariants<T>, tile_rect: TileRect, p: usize,
    po: PlaneOffset, dst: &mut PlaneRegionMut<'_, T>, width: usize,
    height: usize, ref_frames: [RefType; 2], mvs: [MotionVector; 2],
    buffer: &mut InterCompoundBuffers,
  ) {
    assert!(!self.is_intra());
    let frame_po = tile_rect.to_frame_plane_offset(po);

    let mode = fi.default_filter;

    for i in 0..2 {
      if let Some(ref rec) =
        fi.rec_buffer.frames[fi.ref_frames[ref_frames[i].to_index()] as usize]
      {
        let (row_frac, col_frac, src) = PredictionMode::get_mv_params(
          &rec.frame.planes[p],
          frame_po,
          mvs[i],
        );
        prep_8tap(
          buffer.get_buffer_mut(i),
          src,
          width,
          height,
          col_frac,
          row_frac,
          mode,
          mode,
          fi.sequence.bit_depth,
          fi.cpu_feature_level,
        );
      }
    }
    mc_avg(
      dst,
      buffer.get_buffer(0),
      buffer.get_buffer(1),
      width,
      height,
      fi.sequence.bit_depth,
      fi.cpu_feature_level,
    );
  }

  /// Inter prediction that determines whether compound mode is being used based
  /// on the second [`RefType`] in [`ref_frames`].
  pub fn predict_inter<T: Pixel>(
    self, fi: &FrameInvariants<T>, tile_rect: TileRect, p: usize,
    po: PlaneOffset, dst: &mut PlaneRegionMut<'_, T>, width: usize,
    height: usize, ref_frames: [RefType; 2], mvs: [MotionVector; 2],
    compound_buffer: &mut InterCompoundBuffers,
  ) {
    let is_compound = ref_frames[1] != RefType::INTRA_FRAME
      && ref_frames[1] != RefType::NONE_FRAME;

    if !is_compound {
      self.predict_inter_single(
        fi,
        tile_rect,
        p,
        po,
        dst,
        width,
        height,
        ref_frames[0],
        mvs[0],
      )
    } else {
      self.predict_inter_compound(
        fi,
        tile_rect,
        p,
        po,
        dst,
        width,
        height,
        ref_frames,
        mvs,
        compound_buffer,
      );
    }
  }
}

/// A pair of buffers holding the interpolation of two references. Use for
/// compound inter prediction.
#[derive(Debug)]
pub struct InterCompoundBuffers {
  data: ABox<[i16]>,
}

impl InterCompoundBuffers {
  // Size of one of the two buffers used.
  const BUFFER_SIZE: usize = 1 << (2 * MAX_SB_SIZE_LOG2);

  /// Get the buffer for eith
  #[inline]
  fn get_buffer_mut(&mut self, i: usize) -> &mut [i16] {
    match i {
      0 => &mut self.data[0..Self::BUFFER_SIZE],
      1 => &mut self.data[Self::BUFFER_SIZE..2 * Self::BUFFER_SIZE],
      _ => panic!(),
    }
  }

  #[inline]
  fn get_buffer(&self, i: usize) -> &[i16] {
    match i {
      0 => &self.data[0..Self::BUFFER_SIZE],
      1 => &self.data[Self::BUFFER_SIZE..2 * Self::BUFFER_SIZE],
      _ => panic!(),
    }
  }
}

impl Default for InterCompoundBuffers {
  fn default() -> Self {
    Self { data: avec![0; 2 * Self::BUFFER_SIZE].into_boxed_slice() }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum InterIntraMode {
  II_DC_PRED,
  II_V_PRED,
  II_H_PRED,
  II_SMOOTH_PRED,
  INTERINTRA_MODES,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum CompoundType {
  COMPOUND_AVERAGE,
  COMPOUND_WEDGE,
  COMPOUND_DIFFWTD,
  COMPOUND_TYPES,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum MotionMode {
  SIMPLE_TRANSLATION,
  OBMC_CAUSAL,   // 2-sided OBMC
  WARPED_CAUSAL, // 2-sided WARPED
  MOTION_MODES,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum PaletteSize {
  TWO_COLORS,
  THREE_COLORS,
  FOUR_COLORS,
  FIVE_COLORS,
  SIX_COLORS,
  SEVEN_COLORS,
  EIGHT_COLORS,
  PALETTE_SIZES,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum PaletteColor {
  PALETTE_COLOR_ONE,
  PALETTE_COLOR_TWO,
  PALETTE_COLOR_THREE,
  PALETTE_COLOR_FOUR,
  PALETTE_COLOR_FIVE,
  PALETTE_COLOR_SIX,
  PALETTE_COLOR_SEVEN,
  PALETTE_COLOR_EIGHT,
  PALETTE_COLORS,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum FilterIntraMode {
  FILTER_DC_PRED,
  FILTER_V_PRED,
  FILTER_H_PRED,
  FILTER_D157_PRED,
  FILTER_PAETH_PRED,
  FILTER_INTRA_MODES,
}

#[derive(Copy, Clone, Debug)]
pub enum IntraParam {
  AngleDelta(i8),
  Alpha(i16),
  None,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AngleDelta {
  pub y: i8,
  pub uv: i8,
}

#[derive(Copy, Clone, Default)]
pub struct IntraEdgeFilterParameters {
  pub plane: usize,
  pub above_ref_frame_types: Option<[RefType; 2]>,
  pub left_ref_frame_types: Option<[RefType; 2]>,
  pub above_mode: Option<PredictionMode>,
  pub left_mode: Option<PredictionMode>,
}

impl IntraEdgeFilterParameters {
  pub fn new(
    plane: usize, above_ctx: Option<CodedBlockInfo>,
    left_ctx: Option<CodedBlockInfo>,
  ) -> Self {
    IntraEdgeFilterParameters {
      plane,
      above_mode: match above_ctx {
        Some(bi) => match plane {
          0 => bi.luma_mode,
          _ => bi.chroma_mode,
        }
        .into(),
        None => None,
      },
      left_mode: match left_ctx {
        Some(bi) => match plane {
          0 => bi.luma_mode,
          _ => bi.chroma_mode,
        }
        .into(),
        None => None,
      },
      above_ref_frame_types: above_ctx.map(|bi| bi.reference_types),
      left_ref_frame_types: left_ctx.map(|bi| bi.reference_types),
    }
  }

  /// # Panics
  ///
  /// - If the appropriate ref frame types are not set on `self`
  pub fn use_smooth_filter(self) -> bool {
    let above_smooth = match self.above_mode {
      Some(PredictionMode::SMOOTH_PRED)
      | Some(PredictionMode::SMOOTH_V_PRED)
      | Some(PredictionMode::SMOOTH_H_PRED) => {
        self.plane == 0
          || self.above_ref_frame_types.unwrap()[0] == RefType::INTRA_FRAME
      }
      _ => false,
    };

    let left_smooth = match self.left_mode {
      Some(PredictionMode::SMOOTH_PRED)
      | Some(PredictionMode::SMOOTH_V_PRED)
      | Some(PredictionMode::SMOOTH_H_PRED) => {
        self.plane == 0
          || self.left_ref_frame_types.unwrap()[0] == RefType::INTRA_FRAME
      }
      _ => false,
    };

    above_smooth || left_smooth
  }
}

// Weights are quadratic from '1' to '1 / block_size', scaled by 2^sm_weight_log2_scale.
const sm_weight_log2_scale: u8 = 8;

// Smooth predictor weights
#[rustfmt::skip]
static sm_weight_arrays: [u8; 2 * MAX_TX_SIZE] = [
    // Unused, because we always offset by bs, which is at least 2.
    0, 0,
    // bs = 2
    255, 128,
    // bs = 4
    255, 149, 85, 64,
    // bs = 8
    255, 197, 146, 105, 73, 50, 37, 32,
    // bs = 16
    255, 225, 196, 170, 145, 123, 102, 84, 68, 54, 43, 33, 26, 20, 17, 16,
    // bs = 32
    255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111, 101, 92, 83, 74,
    66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8,
    // bs = 64
    255, 248, 240, 233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156,
    150, 144, 138, 133, 127, 121, 116, 111, 106, 101, 96, 91, 86, 82, 77, 73, 69,
    65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27, 25, 22, 20, 18, 16, 15,
    13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
];

#[inline(always)]
const fn get_scaled_luma_q0(alpha_q3: i16, ac_pred_q3: i16) -> i32 {
  let scaled_luma_q6 = (alpha_q3 as i32) * (ac_pred_q3 as i32);
  let abs_scaled_luma_q0 = (scaled_luma_q6.abs() + 32) >> 6;
  if scaled_luma_q6 < 0 {
    -abs_scaled_luma_q0
  } else {
    abs_scaled_luma_q0
  }
}

/// # Returns
///
/// Initialized luma AC coefficients
///
/// # Panics
///
/// - If the block size is invalid for subsampling
///
pub fn luma_ac<'ac, T: Pixel>(
  ac: &'ac mut [MaybeUninit<i16>], ts: &mut TileStateMut<'_, T>,
  tile_bo: TileBlockOffset, bsize: BlockSize, tx_size: TxSize,
  fi: &FrameInvariants<T>,
) -> &'ac mut [i16] {
  use crate::context::MI_SIZE_LOG2;

  let PlaneConfig { xdec, ydec, .. } = ts.input.planes[1].cfg;
  let plane_bsize = bsize.subsampled_size(xdec, ydec).unwrap();

  // ensure ac has the right length, so there aren't any uninitialized elements at the end
  let ac = &mut ac[..plane_bsize.area()];

  let bo = if bsize.is_sub8x8(xdec, ydec) {
    let offset = bsize.sub8x8_offset(xdec, ydec);
    tile_bo.with_offset(offset.0, offset.1)
  } else {
    tile_bo
  };
  let rec = &ts.rec.planes[0];
  let luma = &rec.subregion(Area::BlockStartingAt { bo: bo.0 });
  let frame_bo = ts.to_frame_block_offset(bo);

  let frame_clipped_bw: usize =
    ((fi.w_in_b - frame_bo.0.x) << MI_SIZE_LOG2).min(bsize.width());
  let frame_clipped_bh: usize =
    ((fi.h_in_b - frame_bo.0.y) << MI_SIZE_LOG2).min(bsize.height());

  // Similar to 'MaxLumaW' and 'MaxLumaH' stated in https://aomediacodec.github.io/av1-spec/#transform-block-semantics
  let max_luma_w = if bsize.width() > BlockSize::BLOCK_8X8.width() {
    let txw_log2 = tx_size.width_log2();
    ((frame_clipped_bw + (1 << txw_log2) - 1) >> txw_log2) << txw_log2
  } else {
    bsize.width()
  };
  let max_luma_h = if bsize.height() > BlockSize::BLOCK_8X8.height() {
    let txh_log2 = tx_size.height_log2();
    ((frame_clipped_bh + (1 << txh_log2) - 1) >> txh_log2) << txh_log2
  } else {
    bsize.height()
  };

  let w_pad = (bsize.width() - max_luma_w) >> (2 + xdec);
  let h_pad = (bsize.height() - max_luma_h) >> (2 + ydec);
  let cpu = fi.cpu_feature_level;

  (match (xdec, ydec) {
    (0, 0) => pred_cfl_ac::<T, 0, 0>,
    (1, 0) => pred_cfl_ac::<T, 1, 0>,
    (_, _) => pred_cfl_ac::<T, 1, 1>,
  })(ac, luma, plane_bsize, w_pad, h_pad, cpu);

  // SAFETY: it relies on individual pred_cfl_ac implementations to initialize the ac
  unsafe { slice_assume_init_mut(ac) }
}

pub(crate) mod rust {
  use super::*;
  use std::mem::size_of;

  #[inline(always)]
  pub fn dispatch_predict_intra<T: Pixel>(
    mode: PredictionMode, variant: PredictionVariant,
    dst: &mut PlaneRegionMut<'_, T>, tx_size: TxSize, bit_depth: usize,
    ac: &[i16], angle: isize, ief_params: Option<IntraEdgeFilterParameters>,
    edge_buf: &IntraEdge<T>, _cpu: CpuFeatureLevel,
  ) {
    let width = tx_size.width();
    let height = tx_size.height();

    // left pixels are ordered from bottom to top and right-aligned
    let (left, top_left, above) = edge_buf.as_slices();

    let above_slice = above;
    let left_slice = &left[left.len().saturating_sub(height)..];
    let left_and_left_below_slice =
      &left[left.len().saturating_sub(width + height)..];

    match mode {
      PredictionMode::DC_PRED => {
        (match variant {
          PredictionVariant::NONE => pred_dc_128,
          PredictionVariant::LEFT => pred_dc_left,
          PredictionVariant::TOP => pred_dc_top,
          PredictionVariant::BOTH => pred_dc,
        })(dst, above_slice, left_slice, width, height, bit_depth)
      }
      PredictionMode::V_PRED if angle == 90 => {
        pred_v(dst, above_slice, width, height)
      }
      PredictionMode::H_PRED if angle == 180 => {
        pred_h(dst, left_slice, width, height)
      }
      PredictionMode::H_PRED
      | PredictionMode::V_PRED
      | PredictionMode::D45_PRED
      | PredictionMode::D135_PRED
      | PredictionMode::D113_PRED
      | PredictionMode::D157_PRED
      | PredictionMode::D203_PRED
      | PredictionMode::D67_PRED => pred_directional(
        dst,
        above_slice,
        left_and_left_below_slice,
        top_left,
        angle as usize,
        width,
        height,
        bit_depth,
        ief_params,
      ),
      PredictionMode::SMOOTH_PRED => {
        pred_smooth(dst, above_slice, left_slice, width, height)
      }
      PredictionMode::SMOOTH_V_PRED => {
        pred_smooth_v(dst, above_slice, left_slice, width, height)
      }
      PredictionMode::SMOOTH_H_PRED => {
        pred_smooth_h(dst, above_slice, left_slice, width, height)
      }
      PredictionMode::PAETH_PRED => {
        pred_paeth(dst, above_slice, left_slice, top_left[0], width, height)
      }
      PredictionMode::UV_CFL_PRED => (match variant {
        PredictionVariant::NONE => pred_cfl_128,
        PredictionVariant::LEFT => pred_cfl_left,
        PredictionVariant::TOP => pred_cfl_top,
        PredictionVariant::BOTH => pred_cfl,
      })(
        dst,
        ac,
        angle as i16,
        above_slice,
        left_slice,
        width,
        height,
        bit_depth,
      ),
      _ => unimplemented!(),
    }
  }

  pub(crate) fn pred_dc<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], left: &[T], width: usize,
    height: usize, _bit_depth: usize,
  ) {
    let edges = left[..height].iter().chain(above[..width].iter());
    let len = (width + height) as u32;
    let avg = (edges.fold(0u32, |acc, &v| {
      let v: u32 = v.into();
      v + acc
    }) + (len >> 1))
      / len;
    let avg = T::cast_from(avg);

    for line in output.rows_iter_mut().take(height) {
      line[..width].fill(avg);
    }
  }

  pub(crate) fn pred_dc_128<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, _above: &[T], _left: &[T],
    width: usize, height: usize, bit_depth: usize,
  ) {
    let v = T::cast_from(128u32 << (bit_depth - 8));
    for line in output.rows_iter_mut().take(height) {
      line[..width].fill(v);
    }
  }

  pub(crate) fn pred_dc_left<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, _above: &[T], left: &[T],
    width: usize, height: usize, _bit_depth: usize,
  ) {
    let sum = left[..].iter().fold(0u32, |acc, &v| {
      let v: u32 = v.into();
      v + acc
    });
    let avg = T::cast_from((sum + (height >> 1) as u32) / height as u32);
    for line in output.rows_iter_mut().take(height) {
      line[..width].fill(avg);
    }
  }

  pub(crate) fn pred_dc_top<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], _left: &[T],
    width: usize, height: usize, _bit_depth: usize,
  ) {
    let sum = above[..width].iter().fold(0u32, |acc, &v| {
      let v: u32 = v.into();
      v + acc
    });
    let avg = T::cast_from((sum + (width >> 1) as u32) / width as u32);
    for line in output.rows_iter_mut().take(height) {
      line[..width].fill(avg);
    }
  }

  pub(crate) fn pred_h<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, left: &[T], width: usize,
    height: usize,
  ) {
    for (line, l) in output.rows_iter_mut().zip(left[..height].iter().rev()) {
      line[..width].fill(*l);
    }
  }

  pub(crate) fn pred_v<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], width: usize,
    height: usize,
  ) {
    for line in output.rows_iter_mut().take(height) {
      line[..width].copy_from_slice(&above[..width])
    }
  }

  pub(crate) fn pred_paeth<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], left: &[T],
    above_left: T, width: usize, height: usize,
  ) {
    for r in 0..height {
      let row = &mut output[r];
      for c in 0..width {
        // Top-left pixel is fixed in libaom
        let raw_top_left: i32 = above_left.into();
        let raw_left: i32 = left[height - 1 - r].into();
        let raw_top: i32 = above[c].into();

        let p_base = raw_top + raw_left - raw_top_left;
        let p_left = (p_base - raw_left).abs();
        let p_top = (p_base - raw_top).abs();
        let p_top_left = (p_base - raw_top_left).abs();

        // Return nearest to base of left, top and top_left
        if p_left <= p_top && p_left <= p_top_left {
          row[c] = T::cast_from(raw_left);
        } else if p_top <= p_top_left {
          row[c] = T::cast_from(raw_top);
        } else {
          row[c] = T::cast_from(raw_top_left);
        }
      }
    }
  }

  pub(crate) fn pred_smooth<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], left: &[T], width: usize,
    height: usize,
  ) {
    let below_pred = left[0]; // estimated by bottom-left pixel
    let right_pred = above[width - 1]; // estimated by top-right pixel
    let sm_weights_w = &sm_weight_arrays[width..];
    let sm_weights_h = &sm_weight_arrays[height..];

    let log2_scale = 1 + sm_weight_log2_scale;
    let scale = 1_u16 << sm_weight_log2_scale;

    // Weights sanity checks
    assert!((sm_weights_w[0] as u16) < scale);
    assert!((sm_weights_h[0] as u16) < scale);
    assert!((scale - sm_weights_w[width - 1] as u16) < scale);
    assert!((scale - sm_weights_h[height - 1] as u16) < scale);
    // ensures no overflow when calculating predictor
    assert!(log2_scale as usize + size_of::<T>() < 31);

    for r in 0..height {
      let row = &mut output[r];
      for c in 0..width {
        let pixels = [above[c], below_pred, left[height - 1 - r], right_pred];

        let weights = [
          sm_weights_h[r] as u16,
          scale - sm_weights_h[r] as u16,
          sm_weights_w[c] as u16,
          scale - sm_weights_w[c] as u16,
        ];

        assert!(
          scale >= (sm_weights_h[r] as u16)
            && scale >= (sm_weights_w[c] as u16)
        );

        // Sum up weighted pixels
        let mut this_pred: u32 = weights
          .iter()
          .zip(pixels.iter())
          .map(|(w, p)| {
            let p: u32 = (*p).into();
            (*w as u32) * p
          })
          .sum();
        this_pred = (this_pred + (1 << (log2_scale - 1))) >> log2_scale;

        row[c] = T::cast_from(this_pred);
      }
    }
  }

  pub(crate) fn pred_smooth_h<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], left: &[T], width: usize,
    height: usize,
  ) {
    let right_pred = above[width - 1]; // estimated by top-right pixel
    let sm_weights = &sm_weight_arrays[width..];

    let log2_scale = sm_weight_log2_scale;
    let scale = 1_u16 << sm_weight_log2_scale;

    // Weights sanity checks
    assert!((sm_weights[0] as u16) < scale);
    assert!((scale - sm_weights[width - 1] as u16) < scale);
    // ensures no overflow when calculating predictor
    assert!(log2_scale as usize + size_of::<T>() < 31);

    for r in 0..height {
      let row = &mut output[r];
      for c in 0..width {
        let pixels = [left[height - 1 - r], right_pred];
        let weights = [sm_weights[c] as u16, scale - sm_weights[c] as u16];

        assert!(scale >= sm_weights[c] as u16);

        let mut this_pred: u32 = weights
          .iter()
          .zip(pixels.iter())
          .map(|(w, p)| {
            let p: u32 = (*p).into();
            (*w as u32) * p
          })
          .sum();
        this_pred = (this_pred + (1 << (log2_scale - 1))) >> log2_scale;

        row[c] = T::cast_from(this_pred);
      }
    }
  }

  pub(crate) fn pred_smooth_v<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], left: &[T], width: usize,
    height: usize,
  ) {
    let below_pred = left[0]; // estimated by bottom-left pixel
    let sm_weights = &sm_weight_arrays[height..];

    let log2_scale = sm_weight_log2_scale;
    let scale = 1_u16 << sm_weight_log2_scale;

    // Weights sanity checks
    assert!((sm_weights[0] as u16) < scale);
    assert!((scale - sm_weights[height - 1] as u16) < scale);
    // ensures no overflow when calculating predictor
    assert!(log2_scale as usize + size_of::<T>() < 31);

    for r in 0..height {
      let row = &mut output[r];
      for c in 0..width {
        let pixels = [above[c], below_pred];
        let weights = [sm_weights[r] as u16, scale - sm_weights[r] as u16];

        assert!(scale >= sm_weights[r] as u16);

        let mut this_pred: u32 = weights
          .iter()
          .zip(pixels.iter())
          .map(|(w, p)| {
            let p: u32 = (*p).into();
            (*w as u32) * p
          })
          .sum();
        this_pred = (this_pred + (1 << (log2_scale - 1))) >> log2_scale;

        row[c] = T::cast_from(this_pred);
      }
    }
  }

  pub(crate) fn pred_cfl_ac<T: Pixel, const XDEC: usize, const YDEC: usize>(
    ac: &mut [MaybeUninit<i16>], luma: &PlaneRegion<'_, T>,
    plane_bsize: BlockSize, w_pad: usize, h_pad: usize, _cpu: CpuFeatureLevel,
  ) {
    let max_luma_w = (plane_bsize.width() - w_pad * 4) << XDEC;
    let max_luma_h = (plane_bsize.height() - h_pad * 4) << YDEC;
    let max_luma_x: usize = max_luma_w.max(8) - (1 << XDEC);
    let max_luma_y: usize = max_luma_h.max(8) - (1 << YDEC);
    let mut sum: i32 = 0;

    let ac = &mut ac[..plane_bsize.area()];

    for (sub_y, ac_rows) in
      ac.chunks_exact_mut(plane_bsize.width()).enumerate()
    {
      for (sub_x, ac_item) in ac_rows.iter_mut().enumerate() {
        // Refer to https://aomediacodec.github.io/av1-spec/#predict-chroma-from-luma-process
        let luma_y = sub_y << YDEC;
        let luma_x = sub_x << XDEC;
        let y = luma_y.min(max_luma_y);
        let x = luma_x.min(max_luma_x);
        let mut sample: i16 = i16::cast_from(luma[y][x]);
        if XDEC != 0 {
          sample += i16::cast_from(luma[y][x + 1]);
        }
        if YDEC != 0 {
          debug_assert!(XDEC != 0);
          sample += i16::cast_from(luma[y + 1][x])
            + i16::cast_from(luma[y + 1][x + 1]);
        }
        sample <<= 3 - XDEC - YDEC;
        ac_item.write(sample);
        sum += sample as i32;
      }
    }
    // SAFETY: the loop above has initialized all items
    let ac = unsafe { slice_assume_init_mut(ac) };
    let shift = plane_bsize.width_log2() + plane_bsize.height_log2();
    let average = ((sum + (1 << (shift - 1))) >> shift) as i16;

    for val in ac {
      *val -= average;
    }
  }

  pub(crate) fn pred_cfl_inner<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, ac: &[i16], alpha: i16, width: usize,
    height: usize, bit_depth: usize,
  ) {
    if alpha == 0 {
      return;
    }
    debug_assert!(ac.len() >= width * height);
    assert!(output.plane_cfg.stride >= width);
    assert!(output.rows_iter().len() >= height);

    let sample_max = (1 << bit_depth) - 1;
    let avg: i32 = output[0][0].into();

    for (line, luma) in
      output.rows_iter_mut().zip(ac.chunks_exact(width)).take(height)
    {
      for (v, &l) in line[..width].iter_mut().zip(luma[..width].iter()) {
        *v = T::cast_from(
          (avg + get_scaled_luma_q0(alpha, l)).clamp(0, sample_max),
        );
      }
    }
  }

  pub(crate) fn pred_cfl<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, ac: &[i16], alpha: i16, above: &[T],
    left: &[T], width: usize, height: usize, bit_depth: usize,
  ) {
    pred_dc(output, above, left, width, height, bit_depth);
    pred_cfl_inner(output, ac, alpha, width, height, bit_depth);
  }

  pub(crate) fn pred_cfl_128<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, ac: &[i16], alpha: i16, above: &[T],
    left: &[T], width: usize, height: usize, bit_depth: usize,
  ) {
    pred_dc_128(output, above, left, width, height, bit_depth);
    pred_cfl_inner(output, ac, alpha, width, height, bit_depth);
  }

  pub(crate) fn pred_cfl_left<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, ac: &[i16], alpha: i16, above: &[T],
    left: &[T], width: usize, height: usize, bit_depth: usize,
  ) {
    pred_dc_left(output, above, left, width, height, bit_depth);
    pred_cfl_inner(output, ac, alpha, width, height, bit_depth);
  }

  pub(crate) fn pred_cfl_top<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, ac: &[i16], alpha: i16, above: &[T],
    left: &[T], width: usize, height: usize, bit_depth: usize,
  ) {
    pred_dc_top(output, above, left, width, height, bit_depth);
    pred_cfl_inner(output, ac, alpha, width, height, bit_depth);
  }

  #[allow(clippy::collapsible_if)]
  #[allow(clippy::collapsible_else_if)]
  #[allow(clippy::needless_return)]
  pub(crate) const fn select_ief_strength(
    width: usize, height: usize, smooth_filter: bool, angle_delta: isize,
  ) -> u8 {
    let block_wh = width + height;
    let abs_delta = angle_delta.unsigned_abs();

    if smooth_filter {
      if block_wh <= 8 {
        if abs_delta >= 64 {
          return 2;
        }
        if abs_delta >= 40 {
          return 1;
        }
      } else if block_wh <= 16 {
        if abs_delta >= 48 {
          return 2;
        }
        if abs_delta >= 20 {
          return 1;
        }
      } else if block_wh <= 24 {
        if abs_delta >= 4 {
          return 3;
        }
      } else {
        return 3;
      }
    } else {
      if block_wh <= 8 {
        if abs_delta >= 56 {
          return 1;
        }
      } else if block_wh <= 16 {
        if abs_delta >= 40 {
          return 1;
        }
      } else if block_wh <= 24 {
        if abs_delta >= 32 {
          return 3;
        }
        if abs_delta >= 16 {
          return 2;
        }
        if abs_delta >= 8 {
          return 1;
        }
      } else if block_wh <= 32 {
        if abs_delta >= 32 {
          return 3;
        }
        if abs_delta >= 4 {
          return 2;
        }
        return 1;
      } else {
        return 3;
      }
    }

    return 0;
  }

  pub(crate) const fn select_ief_upsample(
    width: usize, height: usize, smooth_filter: bool, angle_delta: isize,
  ) -> bool {
    let block_wh = width + height;
    let abs_delta = angle_delta.unsigned_abs();

    if abs_delta == 0 || abs_delta >= 40 {
      false
    } else if smooth_filter {
      block_wh <= 8
    } else {
      block_wh <= 16
    }
  }

  pub(crate) fn filter_edge<T: Pixel>(
    size: usize, strength: u8, edge: &mut [T],
  ) {
    const INTRA_EDGE_KERNEL: [[u32; 5]; 3] =
      [[0, 4, 8, 4, 0], [0, 5, 6, 5, 0], [2, 4, 4, 4, 2]];

    if strength == 0 {
      return;
    }

    // Copy the edge buffer to avoid predicting from
    // just-filtered samples.
    let mut edge_filtered = [MaybeUninit::<T>::uninit(); MAX_TX_SIZE * 4 + 1];
    let edge_filtered =
      init_slice_repeat_mut(&mut edge_filtered[..edge.len()], T::zero());
    edge_filtered.copy_from_slice(&edge[..edge.len()]);

    for i in 1..size {
      let mut s = 0;

      for j in 0..INTRA_EDGE_KERNEL[0].len() {
        let k = (i + j).saturating_sub(2).min(size - 1);
        s += INTRA_EDGE_KERNEL[(strength - 1) as usize][j]
          * edge[k].to_u32().unwrap();
      }

      edge_filtered[i] = T::cast_from((s + 8) >> 4);
    }
    edge.copy_from_slice(edge_filtered);
  }

  pub(crate) fn upsample_edge<T: Pixel>(
    size: usize, edge: &mut [T], bit_depth: usize,
  ) {
    // The input edge should be valid in the -1..size range,
    // where the -1 index is the top-left edge pixel. Since
    // negative indices are unsafe in Rust, the caller is
    // expected to globally offset it by 1, which makes the
    // input range 0..=size.
    let mut dup = [MaybeUninit::<T>::uninit(); MAX_TX_SIZE];
    let dup = init_slice_repeat_mut(&mut dup[..size + 3], T::zero());
    dup[0] = edge[0];
    dup[1..=size + 1].copy_from_slice(&edge[0..=size]);
    dup[size + 2] = edge[size];

    // Past here the edge is being filtered, and its
    // effective range is shifted from -1..size to
    // -2..2*size-1. Again, because this is safe Rust,
    // we cannot use negative indices, and the actual range
    // will be 0..=2*size. The caller is expected to adjust
    // its indices on receipt of the filtered edge.
    edge[0] = dup[0];

    for i in 0..size {
      let mut s = -dup[i].to_i32().unwrap()
        + (9 * dup[i + 1].to_i32().unwrap())
        + (9 * dup[i + 2].to_i32().unwrap())
        - dup[i + 3].to_i32().unwrap();
      s = ((s + 8) / 16).clamp(0, (1 << bit_depth) - 1);

      edge[2 * i + 1] = T::cast_from(s);
      edge[2 * i + 2] = dup[i + 2];
    }
  }

  pub(crate) const fn dr_intra_derivative(p_angle: usize) -> usize {
    match p_angle {
      3 => 1023,
      6 => 547,
      9 => 372,
      14 => 273,
      17 => 215,
      20 => 178,
      23 => 151,
      26 => 132,
      29 => 116,
      32 => 102,
      36 => 90,
      39 => 80,
      42 => 71,
      45 => 64,
      48 => 57,
      51 => 51,
      54 => 45,
      58 => 40,
      61 => 35,
      64 => 31,
      67 => 27,
      70 => 23,
      73 => 19,
      76 => 15,
      81 => 11,
      84 => 7,
      87 => 3,
      _ => 0,
    }
  }

  pub(crate) fn pred_directional<T: Pixel>(
    output: &mut PlaneRegionMut<'_, T>, above: &[T], left: &[T],
    top_left: &[T], p_angle: usize, width: usize, height: usize,
    bit_depth: usize, ief_params: Option<IntraEdgeFilterParameters>,
  ) {
    let sample_max = (1 << bit_depth) - 1;

    let max_x = output.plane_cfg.width as isize - 1;
    let max_y = output.plane_cfg.height as isize - 1;

    let mut upsample_above = false;
    let mut upsample_left = false;

    let mut above_edge: &[T] = above;
    let mut left_edge: &[T] = left;
    let top_left_edge: T = top_left[0];

    let enable_edge_filter = ief_params.is_some();

    // Initialize above and left edge buffers of the largest possible needed size if upsampled
    // The first value is the top left pixel, also mutable and indexed at -1 in the spec
    let mut above_filtered = [MaybeUninit::<T>::uninit(); MAX_TX_SIZE * 4 + 1];
    let above_filtered = init_slice_repeat_mut(
      &mut above_filtered[..=(width + height) * 2],
      T::zero(),
    );
    let mut left_filtered = [MaybeUninit::<T>::uninit(); MAX_TX_SIZE * 4 + 1];
    let left_filtered = init_slice_repeat_mut(
      &mut left_filtered[..=(width + height) * 2],
      T::zero(),
    );

    if enable_edge_filter {
      let above_len = above.len().min(above_filtered.len() - 1);
      let left_len = left.len().min(left_filtered.len() - 1);
      above_filtered[1..=above_len].clone_from_slice(&above[..above_len]);
      for i in 1..=left_len {
        left_filtered[i] = left[left.len() - i];
      }

      let smooth_filter = ief_params.unwrap().use_smooth_filter();

      if p_angle != 90 && p_angle != 180 {
        above_filtered[0] = top_left_edge;
        left_filtered[0] = top_left_edge;

        let num_px = (
          width.min((max_x - output.rect().x + 1).try_into().unwrap())
            + if p_angle < 90 { height } else { 0 }
            + 1, // above
          height.min((max_y - output.rect().y + 1).try_into().unwrap())
            + if p_angle > 180 { width } else { 0 }
            + 1, // left
        );

        let filter_strength = select_ief_strength(
          width,
          height,
          smooth_filter,
          p_angle as isize - 90,
        );
        filter_edge(num_px.0, filter_strength, above_filtered);
        let filter_strength = select_ief_strength(
          width,
          height,
          smooth_filter,
          p_angle as isize - 180,
        );
        filter_edge(num_px.1, filter_strength, left_filtered);
      }

      let num_px = (
        width + if p_angle < 90 { height } else { 0 }, // above
        height + if p_angle > 180 { width } else { 0 }, // left
      );

      upsample_above = select_ief_upsample(
        width,
        height,
        smooth_filter,
        p_angle as isize - 90,
      );
      if upsample_above {
        upsample_edge(num_px.0, above_filtered, bit_depth);
      }
      upsample_left = select_ief_upsample(
        width,
        height,
        smooth_filter,
        p_angle as isize - 180,
      );
      if upsample_left {
        upsample_edge(num_px.1, left_filtered, bit_depth);
      }

      left_filtered.reverse();
      above_edge = above_filtered;
      left_edge = left_filtered;
    }

    let dx = if p_angle < 90 {
      dr_intra_derivative(p_angle)
    } else if p_angle > 90 && p_angle < 180 {
      dr_intra_derivative(180 - p_angle)
    } else {
      0 // undefined
    };

    let dy = if p_angle > 90 && p_angle < 180 {
      dr_intra_derivative(p_angle - 90)
    } else if p_angle > 180 {
      dr_intra_derivative(270 - p_angle)
    } else {
      0 // undefined
    };

    // edge buffer index offsets applied due to the fact
    // that we cannot safely use negative indices in Rust
    let upsample_above = upsample_above as usize;
    let upsample_left = upsample_left as usize;
    let offset_above = (enable_edge_filter as usize) << upsample_above;
    let offset_left = (enable_edge_filter as usize) << upsample_left;

    if p_angle < 90 {
      for i in 0..height {
        let row = &mut output[i];
        for j in 0..width {
          let idx = (i + 1) * dx;
          let base = (idx >> (6 - upsample_above)) + (j << upsample_above);
          let shift = (((idx << upsample_above) >> 1) & 31) as i32;
          let max_base_x = (height + width - 1) << upsample_above;
          let v = (if base < max_base_x {
            let a: i32 = above_edge[base + offset_above].into();
            let b: i32 = above_edge[base + 1 + offset_above].into();
            round_shift(a * (32 - shift) + b * shift, 5)
          } else {
            let c: i32 = above_edge[max_base_x + offset_above].into();
            c
          })
          .clamp(0, sample_max);
          row[j] = T::cast_from(v);
        }
      }
    } else if p_angle > 90 && p_angle < 180 {
      for i in 0..height {
        let row = &mut output[i];
        for j in 0..width {
          let idx = (j << 6) as isize - ((i + 1) * dx) as isize;
          let base = idx >> (6 - upsample_above);
          if base >= -(1 << upsample_above) {
            let shift = (((idx << upsample_above) >> 1) & 31) as i32;
            let a: i32 = if !enable_edge_filter && base < 0 {
              top_left_edge
            } else {
              above_edge[(base + offset_above as isize) as usize]
            }
            .into();
            let b: i32 =
              above_edge[(base + 1 + offset_above as isize) as usize].into();
            let v = round_shift(a * (32 - shift) + b * shift, 5)
              .clamp(0, sample_max);
            row[j] = T::cast_from(v);
          } else {
            let idx = (i << 6) as isize - ((j + 1) * dy) as isize;
            let base = idx >> (6 - upsample_left);
            let shift = (((idx << upsample_left) >> 1) & 31) as i32;
            let l = left_edge.len() - 1;
            let a: i32 = if !enable_edge_filter && base < 0 {
              top_left_edge
            } else if (base + offset_left as isize) == -2 {
              left_edge[0]
            } else {
              left_edge[l - (base + offset_left as isize) as usize]
            }
            .into();
            let b: i32 = if (base + offset_left as isize) == -2 {
              left_edge[1]
            } else {
              left_edge[l - (base + offset_left as isize + 1) as usize]
            }
            .into();
            let v = round_shift(a * (32 - shift) + b * shift, 5)
              .clamp(0, sample_max);
            row[j] = T::cast_from(v);
          }
        }
      }
    } else if p_angle > 180 {
      for i in 0..height {
        let row = &mut output[i];
        for j in 0..width {
          let idx = (j + 1) * dy;
          let base = (idx >> (6 - upsample_left)) + (i << upsample_left);
          let shift = (((idx << upsample_left) >> 1) & 31) as i32;
          let l = left_edge.len() - 1;
          let a: i32 = left_edge[l.saturating_sub(base + offset_left)].into();
          let b: i32 =
            left_edge[l.saturating_sub(base + offset_left + 1)].into();
          let v =
            round_shift(a * (32 - shift) + b * shift, 5).clamp(0, sample_max);
          row[j] = T::cast_from(v);
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::predict::rust::*;
  use num_traits::*;

  #[test]
  fn pred_matches_u8() {
    let edge_buf =
      Aligned::from_fn(|i| (i + 32).saturating_sub(MAX_TX_SIZE * 2).as_());
    let (all_left, top_left, above) = IntraEdge::mock(&edge_buf).as_slices();
    let left = &all_left[all_left.len() - 4..];

    let mut output = Plane::from_slice(&[0u8; 4 * 4], 4);

    pred_dc(&mut output.as_region_mut(), above, left, 4, 4, 8);
    assert_eq!(&output.data[..], [32u8; 16]);

    pred_dc_top(&mut output.as_region_mut(), above, left, 4, 4, 8);
    assert_eq!(&output.data[..], [35u8; 16]);

    pred_dc_left(&mut output.as_region_mut(), above, left, 4, 4, 8);
    assert_eq!(&output.data[..], [30u8; 16]);

    pred_dc_128(&mut output.as_region_mut(), above, left, 4, 4, 8);
    assert_eq!(&output.data[..], [128u8; 16]);

    pred_v(&mut output.as_region_mut(), above, 4, 4);
    assert_eq!(
      &output.data[..],
      [33, 34, 35, 36, 33, 34, 35, 36, 33, 34, 35, 36, 33, 34, 35, 36]
    );

    pred_h(&mut output.as_region_mut(), left, 4, 4);
    assert_eq!(
      &output.data[..],
      [31, 31, 31, 31, 30, 30, 30, 30, 29, 29, 29, 29, 28, 28, 28, 28]
    );

    pred_paeth(&mut output.as_region_mut(), above, left, top_left[0], 4, 4);
    assert_eq!(
      &output.data[..],
      [32, 34, 35, 36, 30, 32, 32, 36, 29, 32, 32, 32, 28, 28, 32, 32]
    );

    pred_smooth(&mut output.as_region_mut(), above, left, 4, 4);
    assert_eq!(
      &output.data[..],
      [32, 34, 35, 35, 30, 32, 33, 34, 29, 31, 32, 32, 29, 30, 32, 32]
    );

    pred_smooth_h(&mut output.as_region_mut(), above, left, 4, 4);
    assert_eq!(
      &output.data[..],
      [31, 33, 34, 35, 30, 33, 34, 35, 29, 32, 34, 34, 28, 31, 33, 34]
    );

    pred_smooth_v(&mut output.as_region_mut(), above, left, 4, 4);
    assert_eq!(
      &output.data[..],
      [33, 34, 35, 36, 31, 31, 32, 33, 30, 30, 30, 31, 29, 30, 30, 30]
    );

    let left = &all_left[all_left.len() - 8..];
    let angles = [
      3, 6, 9, 14, 17, 20, 23, 26, 29, 32, 36, 39, 42, 45, 48, 51, 54, 58, 61,
      64, 67, 70, 73, 76, 81, 84, 87,
    ];
    let expected = [
      [40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40],
      [40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40],
      [39, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40],
      [37, 38, 39, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40],
      [36, 37, 38, 39, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40],
      [36, 37, 38, 39, 39, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40],
      [35, 36, 37, 38, 38, 39, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40],
      [35, 36, 37, 38, 37, 38, 39, 40, 39, 40, 40, 40, 40, 40, 40, 40],
      [35, 36, 37, 38, 37, 38, 39, 40, 38, 39, 40, 40, 40, 40, 40, 40],
      [35, 36, 37, 38, 36, 37, 38, 39, 38, 39, 40, 40, 39, 40, 40, 40],
      [34, 35, 36, 37, 36, 37, 38, 39, 37, 38, 39, 40, 39, 40, 40, 40],
      [34, 35, 36, 37, 36, 37, 38, 39, 37, 38, 39, 40, 38, 39, 40, 40],
      [34, 35, 36, 37, 35, 36, 37, 38, 36, 37, 38, 39, 37, 38, 39, 40],
      [34, 35, 36, 37, 35, 36, 37, 38, 36, 37, 38, 39, 37, 38, 39, 40],
      [34, 35, 36, 37, 35, 36, 37, 38, 36, 37, 38, 39, 37, 38, 39, 40],
      [34, 35, 36, 37, 35, 36, 37, 38, 35, 36, 37, 38, 36, 37, 38, 39],
      [34, 35, 36, 37, 34, 35, 36, 37, 35, 36, 37, 38, 36, 37, 38, 39],
      [34, 35, 36, 37, 34, 35, 36, 37, 35, 36, 37, 38, 36, 37, 38, 39],
      [34, 35, 36, 37, 34, 35, 36, 37, 35, 36, 37, 38, 35, 36, 37, 38],
      [33, 34, 35, 36, 34, 35, 36, 37, 34, 35, 36, 37, 35, 36, 37, 38],
      [33, 34, 35, 36, 34, 35, 36, 37, 34, 35, 36, 37, 35, 36, 37, 38],
      [33, 34, 35, 36, 34, 35, 36, 37, 34, 35, 36, 37, 34, 35, 36, 37],
      [33, 34, 35, 36, 34, 35, 36, 37, 34, 35, 36, 37, 34, 35, 36, 37],
      [33, 34, 35, 36, 33, 34, 35, 36, 34, 35, 36, 37, 34, 35, 36, 37],
      [33, 34, 35, 36, 33, 34, 35, 36, 34, 35, 36, 37, 34, 35, 36, 37],
      [33, 34, 35, 36, 33, 34, 35, 36, 33, 34, 35, 36, 33, 34, 35, 36],
      [33, 34, 35, 36, 33, 34, 35, 36, 33, 34, 35, 36, 33, 34, 35, 36],
    ];
    for (&angle, expected) in angles.iter().zip(expected.iter()) {
      pred_directional(
        &mut output.as_region_mut(),
        above,
        left,
        top_left,
        angle,
        4,
        4,
        8,
        None,
      );
      assert_eq!(&output.data[..], expected);
    }
  }

  #[test]
  fn pred_max() {
    let max12bit = 4096 - 1;
    let above = [max12bit; 32];
    let left = [max12bit; 32];

    let mut o = Plane::from_slice(&vec![0u16; 32 * 32], 32);

    pred_dc(&mut o.as_region_mut(), &above[..4], &left[..4], 4, 4, 16);

    for l in o.data.chunks(32).take(4) {
      for v in l[..4].iter() {
        assert_eq!(*v, max12bit);
      }
    }

    pred_h(&mut o.as_region_mut(), &left[..4], 4, 4);

    for l in o.data.chunks(32).take(4) {
      for v in l[..4].iter() {
        assert_eq!(*v, max12bit);
      }
    }

    pred_v(&mut o.as_region_mut(), &above[..4], 4, 4);

    for l in o.data.chunks(32).take(4) {
      for v in l[..4].iter() {
        assert_eq!(*v, max12bit);
      }
    }

    let above_left = max12bit;

    pred_paeth(
      &mut o.as_region_mut(),
      &above[..4],
      &left[..4],
      above_left,
      4,
      4,
    );

    for l in o.data.chunks(32).take(4) {
      for v in l[..4].iter() {
        assert_eq!(*v, max12bit);
      }
    }

    pred_smooth(&mut o.as_region_mut(), &above[..4], &left[..4], 4, 4);

    for l in o.data.chunks(32).take(4) {
      for v in l[..4].iter() {
        assert_eq!(*v, max12bit);
      }
    }

    pred_smooth_h(&mut o.as_region_mut(), &above[..4], &left[..4], 4, 4);

    for l in o.data.chunks(32).take(4) {
      for v in l[..4].iter() {
        assert_eq!(*v, max12bit);
      }
    }

    pred_smooth_v(&mut o.as_region_mut(), &above[..4], &left[..4], 4, 4);

    for l in o.data.chunks(32).take(4) {
      for v in l[..4].iter() {
        assert_eq!(*v, max12bit);
      }
    }
  }
}
