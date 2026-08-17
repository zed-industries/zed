// Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved
// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_camel_case_types)]

use std::fmt;
use std::mem::MaybeUninit;

use arrayvec::*;
use itertools::izip;

use crate::api::*;
use crate::cdef::*;
use crate::context::*;
use crate::cpu_features::CpuFeatureLevel;
use crate::deblock::*;
use crate::dist::*;
use crate::ec::{Writer, WriterCounter, OD_BITRES};
use crate::encode_block_with_modes;
use crate::encoder::{FrameInvariants, IMPORTANCE_BLOCK_SIZE};
use crate::frame::*;
use crate::header::ReferenceMode;
use crate::lrf::*;
use crate::mc::MotionVector;
use crate::me::estimate_motion;
use crate::me::MVSamplingMode;
use crate::me::MotionSearchResult;
use crate::motion_compensate;
use crate::partition::PartitionType::*;
use crate::partition::RefType::*;
use crate::partition::*;
use crate::predict::{
  luma_ac, AngleDelta, IntraEdgeFilterParameters, IntraParam, PredictionMode,
  RAV1E_INTER_COMPOUND_MODES, RAV1E_INTER_MODES_MINIMAL, RAV1E_INTRA_MODES,
};
use crate::rdo_tables::*;
use crate::tiling::*;
use crate::transform::{TxSet, TxSize, TxType, RAV1E_TX_TYPES};
use crate::util::{init_slice_repeat_mut, Aligned, Pixel};
use crate::write_tx_blocks;
use crate::write_tx_tree;
use crate::Tune;
use crate::{encode_block_post_cdef, encode_block_pre_cdef};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RDOType {
  PixelDistRealRate,
  TxDistRealRate,
  TxDistEstRate,
}

impl RDOType {
  #[inline]
  pub const fn needs_tx_dist(self) -> bool {
    match self {
      // Pixel-domain distortion and exact ec rate
      RDOType::PixelDistRealRate => false,
      // Tx-domain distortion and exact ec rate
      RDOType::TxDistRealRate => true,
      // Tx-domain distortion and txdist-based rate
      RDOType::TxDistEstRate => true,
    }
  }
  #[inline]
  pub const fn needs_coeff_rate(self) -> bool {
    match self {
      RDOType::PixelDistRealRate => true,
      RDOType::TxDistRealRate => true,
      RDOType::TxDistEstRate => false,
    }
  }
}

#[derive(Clone)]
pub struct PartitionGroupParameters {
  pub rd_cost: f64,
  pub part_type: PartitionType,
  pub part_modes: ArrayVec<PartitionParameters, 4>,
}

#[derive(Clone, Debug)]
pub struct PartitionParameters {
  pub rd_cost: f64,
  pub bo: TileBlockOffset,
  pub bsize: BlockSize,
  pub pred_mode_luma: PredictionMode,
  pub pred_mode_chroma: PredictionMode,
  pub pred_cfl_params: CFLParams,
  pub angle_delta: AngleDelta,
  pub ref_frames: [RefType; 2],
  pub mvs: [MotionVector; 2],
  pub skip: bool,
  pub has_coeff: bool,
  pub tx_size: TxSize,
  pub tx_type: TxType,
  pub sidx: u8,
}

impl Default for PartitionParameters {
  fn default() -> Self {
    PartitionParameters {
      rd_cost: f64::MAX,
      bo: TileBlockOffset::default(),
      bsize: BlockSize::BLOCK_32X32,
      pred_mode_luma: PredictionMode::default(),
      pred_mode_chroma: PredictionMode::default(),
      pred_cfl_params: CFLParams::default(),
      angle_delta: AngleDelta::default(),
      ref_frames: [RefType::INTRA_FRAME, RefType::NONE_FRAME],
      mvs: [MotionVector::default(); 2],
      skip: false,
      has_coeff: true,
      tx_size: TxSize::TX_4X4,
      tx_type: TxType::DCT_DCT,
      sidx: 0,
    }
  }
}

pub fn estimate_rate(qindex: u8, ts: TxSize, fast_distortion: u64) -> u64 {
  let bs_index = ts as usize;
  let q_bin_idx = (qindex as usize) / RDO_QUANT_DIV;
  let bin_idx_down =
    ((fast_distortion) / RATE_EST_BIN_SIZE).min((RDO_NUM_BINS - 2) as u64);
  let bin_idx_up = (bin_idx_down + 1).min((RDO_NUM_BINS - 1) as u64);
  let x0 = (bin_idx_down * RATE_EST_BIN_SIZE) as i64;
  let x1 = (bin_idx_up * RATE_EST_BIN_SIZE) as i64;
  let y0 = RDO_RATE_TABLE[q_bin_idx][bs_index][bin_idx_down as usize] as i64;
  let y1 = RDO_RATE_TABLE[q_bin_idx][bs_index][bin_idx_up as usize] as i64;
  let slope = ((y1 - y0) << 8) / (x1 - x0);
  (y0 + (((fast_distortion as i64 - x0) * slope) >> 8)).max(0) as u64
}

#[allow(unused)]
pub fn cdef_dist_wxh<T: Pixel, F: Fn(Area, BlockSize) -> DistortionScale>(
  src1: &PlaneRegion<'_, T>, src2: &PlaneRegion<'_, T>, w: usize, h: usize,
  bit_depth: usize, compute_bias: F, cpu: CpuFeatureLevel,
) -> Distortion {
  debug_assert!(src1.plane_cfg.xdec == 0);
  debug_assert!(src1.plane_cfg.ydec == 0);
  debug_assert!(src2.plane_cfg.xdec == 0);
  debug_assert!(src2.plane_cfg.ydec == 0);

  let mut sum = Distortion::zero();
  for y in (0..h).step_by(8) {
    for x in (0..w).step_by(8) {
      let kernel_h = (h - y).min(8);
      let kernel_w = (w - x).min(8);
      let area = Area::StartingAt { x: x as isize, y: y as isize };

      let value = RawDistortion(cdef_dist_kernel(
        &src1.subregion(area),
        &src2.subregion(area),
        kernel_w,
        kernel_h,
        bit_depth,
        cpu,
      ) as u64);

      // cdef is always called on non-subsampled planes, so BLOCK_8X8 is
      // correct here.
      sum += value * compute_bias(area, BlockSize::BLOCK_8X8);
    }
  }
  sum
}

/// Sum of Squared Error for a wxh block
/// Currently limited to w and h of valid blocks
pub fn sse_wxh<T: Pixel, F: Fn(Area, BlockSize) -> DistortionScale>(
  src1: &PlaneRegion<'_, T>, src2: &PlaneRegion<'_, T>, w: usize, h: usize,
  compute_bias: F, bit_depth: usize, cpu: CpuFeatureLevel,
) -> Distortion {
  // See get_weighted_sse in src/dist.rs.
  // Provide a scale to get_weighted_sse for each square region of this size.
  const CHUNK_SIZE: usize = IMPORTANCE_BLOCK_SIZE >> 1;

  // To bias the distortion correctly, compute it in blocks up to the size
  // importance block size in a non-subsampled plane.
  let imp_block_w = CHUNK_SIZE << src1.plane_cfg.xdec;
  let imp_block_h = CHUNK_SIZE << src1.plane_cfg.ydec;

  let imp_bsize = BlockSize::from_width_and_height(imp_block_w, imp_block_h);

  let n_imp_blocks_w = w.div_ceil(CHUNK_SIZE);
  let n_imp_blocks_h = h.div_ceil(CHUNK_SIZE);

  // TODO: Copying biases into a buffer is slow. It would be best if biases were
  // passed directly. To do this, we would need different versions of the
  // weighted sse function for decimated/subsampled data. Also requires
  // eliminating use of unbiased sse.
  // It should also be noted that the current copy code does not auto-vectorize.

  // Copy biases into a buffer.
  let mut buf_storage = Aligned::new(
    [MaybeUninit::<u32>::uninit(); 128 / CHUNK_SIZE * 128 / CHUNK_SIZE],
  );
  let buf_stride = n_imp_blocks_w.next_power_of_two();
  let buf = init_slice_repeat_mut(
    &mut buf_storage.data[..buf_stride * n_imp_blocks_h],
    0,
  );

  for block_y in 0..n_imp_blocks_h {
    for block_x in 0..n_imp_blocks_w {
      let block = Area::StartingAt {
        x: (block_x * CHUNK_SIZE) as isize,
        y: (block_y * CHUNK_SIZE) as isize,
      };
      buf[block_y * buf_stride + block_x] = compute_bias(block, imp_bsize).0;
    }
  }

  Distortion(get_weighted_sse(
    src1, src2, buf, buf_stride, w, h, bit_depth, cpu,
  ))
}

// TODO consider saturating_sub later
#[allow(clippy::implicit_saturating_sub)]
pub const fn clip_visible_bsize(
  frame_w: usize, frame_h: usize, bsize: BlockSize, x: usize, y: usize,
) -> (usize, usize) {
  let blk_w = bsize.width();
  let blk_h = bsize.height();

  let visible_w: usize = if x + blk_w <= frame_w {
    blk_w
  } else if x >= frame_w {
    0
  } else {
    frame_w - x
  };

  let visible_h: usize = if y + blk_h <= frame_h {
    blk_h
  } else if y >= frame_h {
    0
  } else {
    frame_h - y
  };

  (visible_w, visible_h)
}

// Compute the pixel-domain distortion for an encode
fn compute_distortion<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &TileStateMut<'_, T>, bsize: BlockSize,
  is_chroma_block: bool, tile_bo: TileBlockOffset, luma_only: bool,
) -> ScaledDistortion {
  let area = Area::BlockStartingAt { bo: tile_bo.0 };
  let input_region = ts.input_tile.planes[0].subregion(area);
  let rec_region = ts.rec.planes[0].subregion(area);

  // clip a block to have visible pixles only
  let frame_bo = ts.to_frame_block_offset(tile_bo);
  let (visible_w, visible_h) = clip_visible_bsize(
    fi.width,
    fi.height,
    bsize,
    frame_bo.0.x << MI_SIZE_LOG2,
    frame_bo.0.y << MI_SIZE_LOG2,
  );

  if visible_w == 0 || visible_h == 0 {
    return ScaledDistortion::zero();
  }

  let mut distortion = match fi.config.tune {
    Tune::Psychovisual => cdef_dist_wxh(
      &input_region,
      &rec_region,
      visible_w,
      visible_h,
      fi.sequence.bit_depth,
      |bias_area, bsize| {
        distortion_scale(
          fi,
          input_region.subregion(bias_area).frame_block_offset(),
          bsize,
        )
      },
      fi.cpu_feature_level,
    ),
    Tune::Psnr => sse_wxh(
      &input_region,
      &rec_region,
      visible_w,
      visible_h,
      |bias_area, bsize| {
        distortion_scale(
          fi,
          input_region.subregion(bias_area).frame_block_offset(),
          bsize,
        )
      },
      fi.sequence.bit_depth,
      fi.cpu_feature_level,
    ),
  } * fi.dist_scale[0];

  if is_chroma_block
    && !luma_only
    && fi.sequence.chroma_sampling != ChromaSampling::Cs400
  {
    let PlaneConfig { xdec, ydec, .. } = ts.input.planes[1].cfg;
    let chroma_w = if bsize.width() >= 8 || xdec == 0 {
      (visible_w + xdec) >> xdec
    } else {
      (4 + visible_w + xdec) >> xdec
    };
    let chroma_h = if bsize.height() >= 8 || ydec == 0 {
      (visible_h + ydec) >> ydec
    } else {
      (4 + visible_h + ydec) >> ydec
    };

    for p in 1..3 {
      let input_region = ts.input_tile.planes[p].subregion(area);
      let rec_region = ts.rec.planes[p].subregion(area);
      distortion += sse_wxh(
        &input_region,
        &rec_region,
        chroma_w,
        chroma_h,
        |bias_area, bsize| {
          distortion_scale(
            fi,
            input_region.subregion(bias_area).frame_block_offset(),
            bsize,
          )
        },
        fi.sequence.bit_depth,
        fi.cpu_feature_level,
      ) * fi.dist_scale[p];
    }
  }
  distortion
}

// Compute the transform-domain distortion for an encode
fn compute_tx_distortion<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &TileStateMut<'_, T>, bsize: BlockSize,
  is_chroma_block: bool, tile_bo: TileBlockOffset, tx_dist: ScaledDistortion,
  skip: bool, luma_only: bool,
) -> ScaledDistortion {
  assert!(fi.config.tune == Tune::Psnr);
  let area = Area::BlockStartingAt { bo: tile_bo.0 };
  let input_region = ts.input_tile.planes[0].subregion(area);
  let rec_region = ts.rec.planes[0].subregion(area);

  let (visible_w, visible_h) = if !skip {
    (bsize.width(), bsize.height())
  } else {
    let frame_bo = ts.to_frame_block_offset(tile_bo);
    clip_visible_bsize(
      fi.width,
      fi.height,
      bsize,
      frame_bo.0.x << MI_SIZE_LOG2,
      frame_bo.0.y << MI_SIZE_LOG2,
    )
  };

  if visible_w == 0 || visible_h == 0 {
    return ScaledDistortion::zero();
  }

  let mut distortion = if skip {
    sse_wxh(
      &input_region,
      &rec_region,
      visible_w,
      visible_h,
      |bias_area, bsize| {
        distortion_scale(
          fi,
          input_region.subregion(bias_area).frame_block_offset(),
          bsize,
        )
      },
      fi.sequence.bit_depth,
      fi.cpu_feature_level,
    ) * fi.dist_scale[0]
  } else {
    tx_dist
  };

  if is_chroma_block
    && !luma_only
    && skip
    && fi.sequence.chroma_sampling != ChromaSampling::Cs400
  {
    let PlaneConfig { xdec, ydec, .. } = ts.input.planes[1].cfg;
    let chroma_w = if bsize.width() >= 8 || xdec == 0 {
      (visible_w + xdec) >> xdec
    } else {
      (4 + visible_w + xdec) >> xdec
    };
    let chroma_h = if bsize.height() >= 8 || ydec == 0 {
      (visible_h + ydec) >> ydec
    } else {
      (4 + visible_h + ydec) >> ydec
    };

    for p in 1..3 {
      let input_region = ts.input_tile.planes[p].subregion(area);
      let rec_region = ts.rec.planes[p].subregion(area);
      distortion += sse_wxh(
        &input_region,
        &rec_region,
        chroma_w,
        chroma_h,
        |bias_area, bsize| {
          distortion_scale(
            fi,
            input_region.subregion(bias_area).frame_block_offset(),
            bsize,
          )
        },
        fi.sequence.bit_depth,
        fi.cpu_feature_level,
      ) * fi.dist_scale[p];
    }
  }
  distortion
}

/// Compute a scaling factor to multiply the distortion of a block by,
/// this factor is determined using temporal RDO.
///
/// # Panics
///
/// - If called with `bsize` of 8x8 or smaller
/// - If the coded frame data doesn't exist on the `FrameInvariants`
pub fn distortion_scale<T: Pixel>(
  fi: &FrameInvariants<T>, frame_bo: PlaneBlockOffset, bsize: BlockSize,
) -> DistortionScale {
  if !fi.config.temporal_rdo() {
    return DistortionScale::default();
  }
  // EncoderConfig::temporal_rdo() should always return false in situations
  // where distortion is computed on > 8x8 blocks, so we should never hit this
  // assert.
  assert!(bsize <= BlockSize::BLOCK_8X8);

  let x = frame_bo.0.x >> IMPORTANCE_BLOCK_TO_BLOCK_SHIFT;
  let y = frame_bo.0.y >> IMPORTANCE_BLOCK_TO_BLOCK_SHIFT;

  let coded_data = fi.coded_frame_data.as_ref().unwrap();
  coded_data.distortion_scales[y * coded_data.w_in_imp_b + x]
}

/// # Panics
///
/// - If the coded frame data doesn't exist on the `FrameInvariants`
pub fn spatiotemporal_scale<T: Pixel>(
  fi: &FrameInvariants<T>, frame_bo: PlaneBlockOffset, bsize: BlockSize,
) -> DistortionScale {
  if !fi.config.temporal_rdo() && fi.config.tune != Tune::Psychovisual {
    return DistortionScale::default();
  }

  let coded_data = fi.coded_frame_data.as_ref().unwrap();

  let x0 = frame_bo.0.x >> IMPORTANCE_BLOCK_TO_BLOCK_SHIFT;
  let y0 = frame_bo.0.y >> IMPORTANCE_BLOCK_TO_BLOCK_SHIFT;
  let x1 = (x0 + bsize.width_imp_b()).min(coded_data.w_in_imp_b);
  let y1 = (y0 + bsize.height_imp_b()).min(coded_data.h_in_imp_b);
  let den = (((x1 - x0) * (y1 - y0)) as u64) << DistortionScale::SHIFT;

  // calling this on each slice individually improves autovectorization
  // compared to using `Iterator::take`
  #[inline(always)]
  fn take_slice<T>(slice: &[T], n: usize) -> &[T] {
    slice.get(..n).unwrap_or(slice)
  }

  let mut sum = 0;
  for y in y0..y1 {
    sum += take_slice(
      &coded_data.distortion_scales[y * coded_data.w_in_imp_b..][x0..x1],
      MAX_SB_IN_IMP_B,
    )
    .iter()
    .zip(
      take_slice(
        &coded_data.activity_scales[y * coded_data.w_in_imp_b..][x0..x1],
        MAX_SB_IN_IMP_B,
      )
      .iter(),
    )
    .map(|(d, a)| d.0 as u64 * a.0 as u64)
    .sum::<u64>();
  }
  DistortionScale(((sum + (den >> 1)) / den) as u32)
}

pub fn distortion_scale_for(
  propagate_cost: f64, intra_cost: f64,
) -> DistortionScale {
  // The mbtree paper \cite{mbtree} uses the following formula:
  //
  //     QP_delta = -strength * log2(1 + (propagate_cost / intra_cost))
  //
  // Since this is H.264, this corresponds to the following quantizer:
  //
  //     Q' = Q * 2^(QP_delta/6)
  //
  // Since lambda is proportial to Q^2, this means we want to minimize:
  //
  //     D + lambda' * R
  //   = D + 2^(QP_delta / 3) * lambda * R
  //
  // If we want to keep lambda fixed, we can instead scale distortion and
  // minimize:
  //
  //     D * scale + lambda * R
  //
  // where:
  //
  //     scale = 2^(QP_delta / -3)
  //           = (1 + (propagate_cost / intra_cost))^(strength / 3)
  //
  //  The original paper empirically chooses strength = 2.0, but strength = 1.0
  //  seems to work best in rav1e currently, this may have something to do with
  //  the fact that they use 16x16 blocks whereas our "importance blocks" are
  //  8x8, but everything should be scale invariant here so that's weird.
  //
  // @article{mbtree,
  //   title={A novel macroblock-tree algorithm for high-performance
  //    optimization of dependent video coding in H.264/AVC},
  //   author={Garrett-Glaser, Jason},
  //   journal={Tech. Rep.},
  //   year={2009},
  //   url={https://pdfs.semanticscholar.org/032f/1ab7d9db385780a02eb2d579af8303b266d2.pdf}
  // }

  if intra_cost == 0. {
    return DistortionScale::default(); // no scaling
  }

  let strength = 1.0; // empirical, see comment above
  let frac = (intra_cost + propagate_cost) / intra_cost;
  frac.powf(strength / 3.0).into()
}

/// Fixed point arithmetic version of distortion scale
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DistortionScale(pub u32);

#[repr(transparent)]
pub struct RawDistortion(u64);

#[repr(transparent)]
pub struct Distortion(pub u64);

#[repr(transparent)]
pub struct ScaledDistortion(u64);

impl DistortionScale {
  /// Bits past the radix point
  const SHIFT: u32 = 14;
  /// Number of bits used. Determines the max value.
  /// 28 bits is quite excessive.
  const BITS: u32 = 28;
  /// Maximum internal value
  const MAX: u64 = (1 << Self::BITS) - 1;

  #[inline]
  pub const fn new(num: u64, den: u64) -> Self {
    let raw = (num << Self::SHIFT).saturating_add(den / 2) / den;
    let mask = (raw <= Self::MAX) as u64;
    Self((mask * raw + (1 - mask) * Self::MAX) as u32)
  }

  pub fn inv_mean(slice: &[Self]) -> Self {
    use crate::util::{bexp64, blog32_q11};
    let sum = slice.iter().map(|&s| blog32_q11(s.0) as i64).sum::<i64>();
    let log_inv_mean_q11 =
      (Self::SHIFT << 11) as i64 - sum / slice.len() as i64;
    Self(
      bexp64((log_inv_mean_q11 + (Self::SHIFT << 11) as i64) << (57 - 11))
        .clamp(1, (1 << Self::BITS) - 1) as u32,
    )
  }

  /// Binary logarithm in Q11
  #[inline]
  pub const fn blog16(self) -> i16 {
    use crate::util::blog32_q11;
    (blog32_q11(self.0) - ((Self::SHIFT as i32) << 11)) as i16
  }

  /// Binary logarithm in Q57
  #[inline]
  pub const fn blog64(self) -> i64 {
    use crate::util::{blog64, q57};
    blog64(self.0 as i64) - q57(Self::SHIFT as i32)
  }

  /// Multiply, round and shift
  /// Internal implementation, so don't use multiply trait.
  #[inline]
  pub const fn mul_u64(self, dist: u64) -> u64 {
    (self.0 as u64 * dist + (1 << Self::SHIFT >> 1)) >> Self::SHIFT
  }
}

impl std::ops::Mul for DistortionScale {
  type Output = Self;

  /// Multiply, round and shift
  #[inline]
  fn mul(self, rhs: Self) -> Self {
    Self(
      (((self.0 as u64 * rhs.0 as u64) + (1 << (Self::SHIFT - 1)))
        >> Self::SHIFT)
        .clamp(1, (1 << Self::BITS) - 1) as u32,
    )
  }
}

impl std::ops::MulAssign for DistortionScale {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

// Default value for DistortionScale is a fixed point 1
impl Default for DistortionScale {
  #[inline]
  fn default() -> Self {
    Self(1 << Self::SHIFT)
  }
}

impl fmt::Debug for DistortionScale {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", f64::from(*self))
  }
}

impl From<f64> for DistortionScale {
  #[inline]
  fn from(scale: f64) -> Self {
    let den = 1 << (Self::SHIFT + 1);
    Self::new((scale * den as f64) as u64, den)
  }
}

impl From<DistortionScale> for f64 {
  #[inline]
  fn from(scale: DistortionScale) -> Self {
    scale.0 as f64 / (1 << DistortionScale::SHIFT) as f64
  }
}

impl RawDistortion {
  #[inline]
  pub const fn new(dist: u64) -> Self {
    Self(dist)
  }
}

impl std::ops::Mul<DistortionScale> for RawDistortion {
  type Output = Distortion;
  #[inline]
  fn mul(self, rhs: DistortionScale) -> Distortion {
    Distortion(rhs.mul_u64(self.0))
  }
}

impl Distortion {
  #[inline]
  pub const fn zero() -> Self {
    Self(0)
  }
}

impl std::ops::Mul<DistortionScale> for Distortion {
  type Output = ScaledDistortion;
  #[inline]
  fn mul(self, rhs: DistortionScale) -> ScaledDistortion {
    ScaledDistortion(rhs.mul_u64(self.0))
  }
}

impl std::ops::AddAssign for Distortion {
  #[inline]
  fn add_assign(&mut self, other: Self) {
    self.0 += other.0;
  }
}

impl ScaledDistortion {
  #[inline]
  pub const fn zero() -> Self {
    Self(0)
  }
}

impl std::ops::AddAssign for ScaledDistortion {
  #[inline]
  fn add_assign(&mut self, other: Self) {
    self.0 += other.0;
  }
}

pub fn compute_rd_cost<T: Pixel>(
  fi: &FrameInvariants<T>, rate: u32, distortion: ScaledDistortion,
) -> f64 {
  let rate_in_bits = (rate as f64) / ((1 << OD_BITRES) as f64);
  fi.lambda.mul_add(rate_in_bits, distortion.0 as f64)
}

pub fn rdo_tx_size_type<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, bsize: BlockSize, tile_bo: TileBlockOffset,
  luma_mode: PredictionMode, ref_frames: [RefType; 2], mvs: [MotionVector; 2],
  skip: bool,
) -> (TxSize, TxType) {
  let is_inter = !luma_mode.is_intra();
  let mut tx_size = max_txsize_rect_lookup[bsize as usize];

  if fi.enable_inter_txfm_split && is_inter && !skip {
    tx_size = sub_tx_size_map[tx_size as usize]; // Always choose one level split size
  }

  let mut best_tx_type = TxType::DCT_DCT;
  let mut best_tx_size = tx_size;
  let mut best_rd = f64::MAX;

  let do_rdo_tx_size = fi.tx_mode_select
    && fi.config.speed_settings.transform.rdo_tx_decision
    && !is_inter;
  let rdo_tx_depth = if do_rdo_tx_size { 2 } else { 0 };
  let mut cw_checkpoint: Option<ContextWriterCheckpoint> = None;

  for _ in 0..=rdo_tx_depth {
    let tx_set = get_tx_set(tx_size, is_inter, fi.use_reduced_tx_set);

    let do_rdo_tx_type = tx_set > TxSet::TX_SET_DCTONLY
      && fi.config.speed_settings.transform.rdo_tx_decision
      && !is_inter
      && !skip;

    if !do_rdo_tx_size && !do_rdo_tx_type {
      return (best_tx_size, best_tx_type);
    };

    let tx_types =
      if do_rdo_tx_type { RAV1E_TX_TYPES } else { &[TxType::DCT_DCT] };

    // Luma plane transform type decision
    let (tx_type, rd_cost) = rdo_tx_type_decision(
      fi,
      ts,
      cw,
      &mut cw_checkpoint,
      luma_mode,
      ref_frames,
      mvs,
      bsize,
      tile_bo,
      tx_size,
      tx_set,
      tx_types,
      best_rd,
    );

    if rd_cost < best_rd {
      best_tx_size = tx_size;
      best_tx_type = tx_type;
      best_rd = rd_cost;
    }

    debug_assert!(tx_size.width_log2() <= bsize.width_log2());
    debug_assert!(tx_size.height_log2() <= bsize.height_log2());
    debug_assert!(
      tx_size.sqr() <= TxSize::TX_32X32 || tx_type == TxType::DCT_DCT
    );

    let next_tx_size = sub_tx_size_map[tx_size as usize];

    if next_tx_size == tx_size {
      break;
    } else {
      tx_size = next_tx_size;
    };
  }

  (best_tx_size, best_tx_type)
}

#[inline]
const fn dmv_in_range(mv: MotionVector, ref_mv: MotionVector) -> bool {
  let diff_row = mv.row as i32 - ref_mv.row as i32;
  let diff_col = mv.col as i32 - ref_mv.col as i32;
  diff_row >= MV_LOW
    && diff_row <= MV_UPP
    && diff_col >= MV_LOW
    && diff_col <= MV_UPP
}

#[inline]
#[profiling::function]
fn luma_chroma_mode_rdo<T: Pixel>(
  luma_mode: PredictionMode, fi: &FrameInvariants<T>, bsize: BlockSize,
  tile_bo: TileBlockOffset, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, rdo_type: RDOType,
  cw_checkpoint: &ContextWriterCheckpoint, best: &mut PartitionParameters,
  mvs: [MotionVector; 2], ref_frames: [RefType; 2],
  mode_set_chroma: &[PredictionMode], luma_mode_is_intra: bool,
  mode_context: usize, mv_stack: &ArrayVec<CandidateMV, 9>,
  angle_delta: AngleDelta,
) {
  let PlaneConfig { xdec, ydec, .. } = ts.input.planes[1].cfg;

  let is_chroma_block =
    has_chroma(tile_bo, bsize, xdec, ydec, fi.sequence.chroma_sampling);

  if !luma_mode_is_intra {
    let ref_mvs = if mv_stack.is_empty() {
      [MotionVector::default(); 2]
    } else {
      [mv_stack[0].this_mv, mv_stack[0].comp_mv]
    };

    if (luma_mode == PredictionMode::NEWMV
      || luma_mode == PredictionMode::NEW_NEWMV
      || luma_mode == PredictionMode::NEW_NEARESTMV)
      && !dmv_in_range(mvs[0], ref_mvs[0])
    {
      return;
    }

    if (luma_mode == PredictionMode::NEW_NEWMV
      || luma_mode == PredictionMode::NEAREST_NEWMV)
      && !dmv_in_range(mvs[1], ref_mvs[1])
    {
      return;
    }
  }

  // Find the best chroma prediction mode for the current luma prediction mode
  let mut chroma_rdo = |skip: bool| -> bool {
    use crate::segmentation::select_segment;

    let mut zero_distortion = false;

    for sidx in select_segment(fi, ts, tile_bo, bsize, skip) {
      cw.bc.blocks.set_segmentation_idx(tile_bo, bsize, sidx);

      let (tx_size, tx_type) = rdo_tx_size_type(
        fi, ts, cw, bsize, tile_bo, luma_mode, ref_frames, mvs, skip,
      );
      for &chroma_mode in mode_set_chroma.iter() {
        let wr = &mut WriterCounter::new();
        let tell = wr.tell_frac();

        if bsize >= BlockSize::BLOCK_8X8 && bsize.is_sqr() {
          cw.write_partition(
            wr,
            tile_bo,
            PartitionType::PARTITION_NONE,
            bsize,
          );
        }

        // TODO(yushin): luma and chroma would have different decision based on chroma format
        let need_recon_pixel =
          luma_mode_is_intra && tx_size.block_size() != bsize;

        encode_block_pre_cdef(&fi.sequence, ts, cw, wr, bsize, tile_bo, skip);
        let (has_coeff, tx_dist) = encode_block_post_cdef(
          fi,
          ts,
          cw,
          wr,
          luma_mode,
          chroma_mode,
          angle_delta,
          ref_frames,
          mvs,
          bsize,
          tile_bo,
          skip,
          CFLParams::default(),
          tx_size,
          tx_type,
          mode_context,
          mv_stack,
          rdo_type,
          need_recon_pixel,
          None,
        );

        let rate = wr.tell_frac() - tell;
        let distortion = if fi.use_tx_domain_distortion && !need_recon_pixel {
          compute_tx_distortion(
            fi,
            ts,
            bsize,
            is_chroma_block,
            tile_bo,
            tx_dist,
            skip,
            false,
          )
        } else {
          compute_distortion(fi, ts, bsize, is_chroma_block, tile_bo, false)
        };
        let is_zero_dist = distortion.0 == 0;
        let rd = compute_rd_cost(fi, rate, distortion);
        if rd < best.rd_cost {
          //if rd < best.rd_cost || luma_mode == PredictionMode::NEW_NEWMV {
          best.rd_cost = rd;
          best.pred_mode_luma = luma_mode;
          best.pred_mode_chroma = chroma_mode;
          best.angle_delta = angle_delta;
          best.ref_frames = ref_frames;
          best.mvs = mvs;
          best.skip = skip;
          best.has_coeff = has_coeff;
          best.tx_size = tx_size;
          best.tx_type = tx_type;
          best.sidx = sidx;
          zero_distortion = is_zero_dist;
        }

        cw.rollback(cw_checkpoint);
      }
    }

    zero_distortion
  };

  // Don't skip when using intra modes
  let zero_distortion =
    if !luma_mode_is_intra { chroma_rdo(true) } else { false };
  // early skip
  if !zero_distortion {
    chroma_rdo(false);
  }
}

/// RDO-based mode decision
///
/// # Panics
///
/// - If the best RD found is negative.
///   This should never happen and indicates a development error.
#[profiling::function]
pub fn rdo_mode_decision<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, bsize: BlockSize, tile_bo: TileBlockOffset,
  inter_cfg: &InterConfig,
) -> PartitionParameters {
  let PlaneConfig { xdec, ydec, .. } = ts.input.planes[1].cfg;
  let cw_checkpoint = cw.checkpoint(&tile_bo, fi.sequence.chroma_sampling);

  let rdo_type = if fi.use_tx_domain_rate {
    RDOType::TxDistEstRate
  } else if fi.use_tx_domain_distortion {
    RDOType::TxDistRealRate
  } else {
    RDOType::PixelDistRealRate
  };

  let mut best = if fi.frame_type.has_inter() {
    assert!(fi.frame_type != FrameType::KEY);

    inter_frame_rdo_mode_decision(
      fi,
      ts,
      cw,
      bsize,
      tile_bo,
      inter_cfg,
      &cw_checkpoint,
      rdo_type,
    )
  } else {
    PartitionParameters::default()
  };

  let is_chroma_block =
    has_chroma(tile_bo, bsize, xdec, ydec, fi.sequence.chroma_sampling);

  if !best.skip {
    best = intra_frame_rdo_mode_decision(
      fi,
      ts,
      cw,
      bsize,
      tile_bo,
      &cw_checkpoint,
      rdo_type,
      best,
      is_chroma_block,
    );
  }

  if best.pred_mode_luma.is_intra() && is_chroma_block && bsize.cfl_allowed() {
    cw.bc.blocks.set_segmentation_idx(tile_bo, bsize, best.sidx);

    let chroma_mode = PredictionMode::UV_CFL_PRED;
    let cw_checkpoint = cw.checkpoint(&tile_bo, fi.sequence.chroma_sampling);
    let mut wr = WriterCounter::new();
    let angle_delta = AngleDelta { y: best.angle_delta.y, uv: 0 };

    write_tx_blocks(
      fi,
      ts,
      cw,
      &mut wr,
      best.pred_mode_luma,
      best.pred_mode_luma,
      angle_delta,
      tile_bo,
      bsize,
      best.tx_size,
      best.tx_type,
      false,
      CFLParams::default(),
      true,
      rdo_type,
      true,
    );
    cw.rollback(&cw_checkpoint);
    if fi.sequence.chroma_sampling != ChromaSampling::Cs400 {
      if let Some(cfl) = rdo_cfl_alpha(ts, tile_bo, bsize, best.tx_size, fi) {
        let mut wr = WriterCounter::new();
        let tell = wr.tell_frac();

        encode_block_pre_cdef(
          &fi.sequence,
          ts,
          cw,
          &mut wr,
          bsize,
          tile_bo,
          best.skip,
        );
        let (has_coeff, _) = encode_block_post_cdef(
          fi,
          ts,
          cw,
          &mut wr,
          best.pred_mode_luma,
          chroma_mode,
          angle_delta,
          best.ref_frames,
          best.mvs,
          bsize,
          tile_bo,
          best.skip,
          cfl,
          best.tx_size,
          best.tx_type,
          0,
          &[],
          rdo_type,
          true, // For CFL, luma should be always reconstructed.
          None,
        );

        let rate = wr.tell_frac() - tell;

        // For CFL, tx-domain distortion is not an option.
        let distortion =
          compute_distortion(fi, ts, bsize, is_chroma_block, tile_bo, false);
        let rd = compute_rd_cost(fi, rate, distortion);
        if rd < best.rd_cost {
          best.rd_cost = rd;
          best.pred_mode_chroma = chroma_mode;
          best.angle_delta = angle_delta;
          best.has_coeff = has_coeff;
          best.pred_cfl_params = cfl;
        }

        cw.rollback(&cw_checkpoint);
      }
    }
  }

  cw.bc.blocks.set_mode(tile_bo, bsize, best.pred_mode_luma);
  cw.bc.blocks.set_ref_frames(tile_bo, bsize, best.ref_frames);
  cw.bc.blocks.set_motion_vectors(tile_bo, bsize, best.mvs);

  assert!(best.rd_cost >= 0_f64);

  PartitionParameters {
    bo: tile_bo,
    bsize,
    pred_mode_luma: best.pred_mode_luma,
    pred_mode_chroma: best.pred_mode_chroma,
    pred_cfl_params: best.pred_cfl_params,
    angle_delta: best.angle_delta,
    ref_frames: best.ref_frames,
    mvs: best.mvs,
    rd_cost: best.rd_cost,
    skip: best.skip,
    has_coeff: best.has_coeff,
    tx_size: best.tx_size,
    tx_type: best.tx_type,
    sidx: best.sidx,
  }
}

#[profiling::function]
fn inter_frame_rdo_mode_decision<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, bsize: BlockSize, tile_bo: TileBlockOffset,
  inter_cfg: &InterConfig, cw_checkpoint: &ContextWriterCheckpoint,
  rdo_type: RDOType,
) -> PartitionParameters {
  let mut best = PartitionParameters::default();

  // we can never have more than 7 reference frame sets
  let mut ref_frames_set = ArrayVec::<_, 7>::new();
  // again, max of 7 ref slots
  let mut ref_slot_set = ArrayVec::<_, 7>::new();
  // our implementation never returns more than 3 at the moment
  let mut mvs_from_me = ArrayVec::<_, 3>::new();
  let mut fwdref = None;
  let mut bwdref = None;

  for i in inter_cfg.allowed_ref_frames().iter().copied() {
    // Don't search LAST3 since it's used only for probs
    if i == LAST3_FRAME {
      continue;
    }

    if !ref_slot_set.contains(&fi.ref_frames[i.to_index()]) {
      if fwdref.is_none() && i.is_fwd_ref() {
        fwdref = Some(ref_frames_set.len());
      }
      if bwdref.is_none() && i.is_bwd_ref() {
        bwdref = Some(ref_frames_set.len());
      }
      ref_frames_set.push([i, NONE_FRAME]);
      let slot_idx = fi.ref_frames[i.to_index()];
      ref_slot_set.push(slot_idx);
    }
  }
  assert!(!ref_frames_set.is_empty());

  let mut inter_mode_set = ArrayVec::<(PredictionMode, usize), 20>::new();
  let mut mvs_set = ArrayVec::<[MotionVector; 2], 20>::new();
  let mut satds = ArrayVec::<u32, 20>::new();
  let mut mv_stacks = ArrayVec::<_, 20>::new();
  let mut mode_contexts = ArrayVec::<_, 7>::new();

  for (i, &ref_frames) in ref_frames_set.iter().enumerate() {
    let mut mv_stack = ArrayVec::<CandidateMV, 9>::new();
    mode_contexts.push(cw.find_mvrefs(
      tile_bo,
      ref_frames,
      &mut mv_stack,
      bsize,
      fi,
      false,
    ));

    let mut pmv = [MotionVector::default(); 2];
    if !mv_stack.is_empty() {
      pmv[0] = mv_stack[0].this_mv;
    }
    if mv_stack.len() > 1 {
      pmv[1] = mv_stack[1].this_mv;
    }

    let res = estimate_motion(
      fi,
      ts,
      bsize.width(),
      bsize.height(),
      tile_bo,
      ref_frames[0],
      Some(pmv),
      MVSamplingMode::CORNER { right: true, bottom: true },
      false,
      0,
      None,
    )
    .unwrap_or_else(MotionSearchResult::empty);
    let b_me = res.mv;

    mvs_from_me.push([b_me, MotionVector::default()]);

    for &x in RAV1E_INTER_MODES_MINIMAL {
      inter_mode_set.push((x, i));
    }
    if !mv_stack.is_empty() {
      inter_mode_set.push((PredictionMode::NEAR0MV, i));
    }
    if mv_stack.len() >= 2 {
      inter_mode_set.push((PredictionMode::GLOBALMV, i));
    }
    let include_near_mvs = fi.config.speed_settings.motion.include_near_mvs;
    if include_near_mvs {
      if mv_stack.len() >= 3 {
        inter_mode_set.push((PredictionMode::NEAR1MV, i));
      }
      if mv_stack.len() >= 4 {
        inter_mode_set.push((PredictionMode::NEAR2MV, i));
      }
    }
    let same_row_col = |x: &CandidateMV| {
      x.this_mv.row == mvs_from_me[i][0].row
        && x.this_mv.col == mvs_from_me[i][0].col
    };
    if !mv_stack
      .iter()
      .take(if include_near_mvs { 4 } else { 2 })
      .any(same_row_col)
      && (mvs_from_me[i][0].row != 0 || mvs_from_me[i][0].col != 0)
    {
      inter_mode_set.push((PredictionMode::NEWMV, i));
    }

    mv_stacks.push(mv_stack);
  }

  let sz = bsize.width_mi().min(bsize.height_mi());

  // To use non single reference modes, block width and height must be greater than 4.
  if fi.reference_mode != ReferenceMode::SINGLE && sz >= 2 {
    // Adding compound candidate
    if let Some(r0) = fwdref {
      if let Some(r1) = bwdref {
        let ref_frames = [ref_frames_set[r0][0], ref_frames_set[r1][0]];
        ref_frames_set.push(ref_frames);
        let mv0 = mvs_from_me[r0][0];
        let mv1 = mvs_from_me[r1][0];
        mvs_from_me.push([mv0, mv1]);
        let mut mv_stack = ArrayVec::<CandidateMV, 9>::new();
        mode_contexts.push(cw.find_mvrefs(
          tile_bo,
          ref_frames,
          &mut mv_stack,
          bsize,
          fi,
          true,
        ));
        for &x in RAV1E_INTER_COMPOUND_MODES {
          // exclude any NEAR mode based on speed setting
          if fi.config.speed_settings.motion.include_near_mvs
            || !x.has_nearmv()
          {
            let mv_stack_idx = ref_frames_set.len() - 1;
            // exclude NEAR modes if the mv_stack is too short
            if !(x.has_nearmv() && x.ref_mv_idx() >= mv_stack.len()) {
              inter_mode_set.push((x, mv_stack_idx));
            }
          }
        }
        mv_stacks.push(mv_stack);
      }
    }
  }

  let num_modes_rdo = if fi.config.speed_settings.prediction.prediction_modes
    >= PredictionModesSetting::ComplexAll
  {
    inter_mode_set.len()
  } else {
    9 // This number is determined by AWCY test
  };

  inter_mode_set.iter().for_each(|&(luma_mode, i)| {
    let mvs = match luma_mode {
      PredictionMode::NEWMV | PredictionMode::NEW_NEWMV => mvs_from_me[i],
      PredictionMode::NEARESTMV | PredictionMode::NEAREST_NEARESTMV => {
        if !mv_stacks[i].is_empty() {
          [mv_stacks[i][0].this_mv, mv_stacks[i][0].comp_mv]
        } else {
          [MotionVector::default(); 2]
        }
      }
      PredictionMode::NEAR0MV | PredictionMode::NEAR_NEAR0MV => {
        if mv_stacks[i].len() > 1 {
          [mv_stacks[i][1].this_mv, mv_stacks[i][1].comp_mv]
        } else {
          [MotionVector::default(); 2]
        }
      }
      PredictionMode::NEAR1MV
      | PredictionMode::NEAR2MV
      | PredictionMode::NEAR_NEAR1MV
      | PredictionMode::NEAR_NEAR2MV => [
        mv_stacks[i][luma_mode.ref_mv_idx()].this_mv,
        mv_stacks[i][luma_mode.ref_mv_idx()].comp_mv,
      ],
      PredictionMode::NEAREST_NEWMV => {
        [mv_stacks[i][0].this_mv, mvs_from_me[i][1]]
      }
      PredictionMode::NEW_NEARESTMV => {
        [mvs_from_me[i][0], mv_stacks[i][0].comp_mv]
      }
      PredictionMode::GLOBALMV | PredictionMode::GLOBAL_GLOBALMV => {
        [MotionVector::default(); 2]
      }
      _ => {
        unimplemented!();
      }
    };
    mvs_set.push(mvs);

    // Calculate SATD for each mode
    if num_modes_rdo != inter_mode_set.len() {
      let tile_rect = ts.tile_rect();
      let rec = &mut ts.rec.planes[0];
      let po = tile_bo.plane_offset(rec.plane_cfg);
      let mut rec_region =
        rec.subregion_mut(Area::BlockStartingAt { bo: tile_bo.0 });

      luma_mode.predict_inter(
        fi,
        tile_rect,
        0,
        po,
        &mut rec_region,
        bsize.width(),
        bsize.height(),
        ref_frames_set[i],
        mvs,
        &mut ts.inter_compound_buffers,
      );

      let plane_org = ts.input_tile.planes[0]
        .subregion(Area::BlockStartingAt { bo: tile_bo.0 });
      let plane_ref = rec_region.as_const();

      let satd = get_satd(
        &plane_org,
        &plane_ref,
        bsize.width(),
        bsize.height(),
        fi.sequence.bit_depth,
        fi.cpu_feature_level,
      );
      satds.push(satd);
    } else {
      satds.push(0);
    }
  });

  let mut sorted =
    izip!(inter_mode_set, mvs_set, satds).collect::<ArrayVec<_, 20>>();
  if num_modes_rdo != sorted.len() {
    sorted.sort_by_key(|((_mode, _i), _mvs, satd)| *satd);
  }

  sorted.iter().take(num_modes_rdo).for_each(
    |&((luma_mode, i), mvs, _satd)| {
      let mode_set_chroma = ArrayVec::from([luma_mode]);

      luma_chroma_mode_rdo(
        luma_mode,
        fi,
        bsize,
        tile_bo,
        ts,
        cw,
        rdo_type,
        cw_checkpoint,
        &mut best,
        mvs,
        ref_frames_set[i],
        &mode_set_chroma,
        false,
        mode_contexts[i],
        &mv_stacks[i],
        AngleDelta::default(),
      );
    },
  );

  best
}

#[profiling::function]
fn intra_frame_rdo_mode_decision<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, bsize: BlockSize, tile_bo: TileBlockOffset,
  cw_checkpoint: &ContextWriterCheckpoint, rdo_type: RDOType,
  mut best: PartitionParameters, is_chroma_block: bool,
) -> PartitionParameters {
  let mut modes = ArrayVec::<_, INTRA_MODES>::new();

  // Reduce number of prediction modes at higher speed levels
  let num_modes_rdo = if (fi.frame_type == FrameType::KEY
    && fi.config.speed_settings.prediction.prediction_modes
      >= PredictionModesSetting::ComplexKeyframes)
    || (fi.frame_type.has_inter()
      && fi.config.speed_settings.prediction.prediction_modes
        >= PredictionModesSetting::ComplexAll)
  {
    7
  } else {
    3
  };

  let intra_mode_set = RAV1E_INTRA_MODES;

  // Find mode with lowest rate cost
  {
    use crate::ec::cdf_to_pdf;

    let probs_all = cdf_to_pdf(if fi.frame_type.has_inter() {
      cw.get_cdf_intra_mode(bsize)
    } else {
      cw.get_cdf_intra_mode_kf(tile_bo)
    });

    modes.try_extend_from_slice(intra_mode_set).unwrap();
    modes.sort_by_key(|&a| !probs_all[a as usize]);
  }

  // If tx partition (i.e. fi.tx_mode_select) is enabled, the below intra prediction screening
  // may be improved by emulating prediction for each tx block.
  {
    let satds = {
      // FIXME: If tx partition is used, this whole sads block should be fixed
      let tx_size = bsize.tx_size();
      let mut edge_buf = Aligned::uninit_array();
      let edge_buf = {
        let rec = &ts.rec.planes[0].as_const();
        let po = tile_bo.plane_offset(rec.plane_cfg);
        // FIXME: If tx partition is used, get_intra_edges() should be called for each tx block
        get_intra_edges(
          &mut edge_buf,
          rec,
          tile_bo,
          0,
          0,
          bsize,
          po,
          tx_size,
          fi.sequence.bit_depth,
          None,
          fi.sequence.enable_intra_edge_filter,
          IntraParam::None,
        )
      };

      let ief_params = if fi.sequence.enable_intra_edge_filter {
        let above_block_info = ts.above_block_info(tile_bo, 0, 0);
        let left_block_info = ts.left_block_info(tile_bo, 0, 0);
        Some(IntraEdgeFilterParameters::new(
          0,
          above_block_info,
          left_block_info,
        ))
      } else {
        None
      };

      let mut satds_all = [0; INTRA_MODES];
      for &luma_mode in modes.iter().skip(num_modes_rdo / 2) {
        let tile_rect = ts.tile_rect();
        let rec = &mut ts.rec.planes[0];
        let mut rec_region =
          rec.subregion_mut(Area::BlockStartingAt { bo: tile_bo.0 });
        // FIXME: If tx partition is used, luma_mode.predict_intra() should be called for each tx block
        luma_mode.predict_intra(
          tile_rect,
          &mut rec_region,
          tx_size,
          fi.sequence.bit_depth,
          &[0i16; 2],
          IntraParam::None,
          if luma_mode.is_directional() { ief_params } else { None },
          &edge_buf,
          fi.cpu_feature_level,
        );

        let plane_org = ts.input_tile.planes[0]
          .subregion(Area::BlockStartingAt { bo: tile_bo.0 });
        let plane_ref = rec_region.as_const();

        satds_all[luma_mode as usize] = get_satd(
          &plane_org,
          &plane_ref,
          tx_size.width(),
          tx_size.height(),
          fi.sequence.bit_depth,
          fi.cpu_feature_level,
        );
      }
      satds_all
    };

    modes[num_modes_rdo / 2..].sort_by_key(|&a| satds[a as usize]);
  }

  debug_assert!(num_modes_rdo >= 1);

  modes.iter().take(num_modes_rdo).for_each(|&luma_mode| {
    let mvs = [MotionVector::default(); 2];
    let ref_frames = [INTRA_FRAME, NONE_FRAME];
    let mut mode_set_chroma = ArrayVec::<_, 2>::new();
    mode_set_chroma.push(luma_mode);
    if is_chroma_block && luma_mode != PredictionMode::DC_PRED {
      mode_set_chroma.push(PredictionMode::DC_PRED);
    }
    luma_chroma_mode_rdo(
      luma_mode,
      fi,
      bsize,
      tile_bo,
      ts,
      cw,
      rdo_type,
      cw_checkpoint,
      &mut best,
      mvs,
      ref_frames,
      &mode_set_chroma,
      true,
      0,
      &ArrayVec::<CandidateMV, 9>::new(),
      AngleDelta::default(),
    );
  });

  if fi.config.speed_settings.prediction.fine_directional_intra
    && bsize >= BlockSize::BLOCK_8X8
  {
    // Find the best angle delta for the current best prediction mode
    let luma_deltas = best.pred_mode_luma.angle_delta_count();
    let chroma_deltas = best.pred_mode_chroma.angle_delta_count();

    let mvs = [MotionVector::default(); 2];
    let ref_frames = [INTRA_FRAME, NONE_FRAME];
    let mode_set_chroma = [best.pred_mode_chroma];
    let mv_stack = ArrayVec::<_, 9>::new();
    let mut best_angle_delta = best.angle_delta;
    let mut angle_delta_rdo = |y, uv| -> AngleDelta {
      if best.angle_delta.y != y || best.angle_delta.uv != uv {
        luma_chroma_mode_rdo(
          best.pred_mode_luma,
          fi,
          bsize,
          tile_bo,
          ts,
          cw,
          rdo_type,
          cw_checkpoint,
          &mut best,
          mvs,
          ref_frames,
          &mode_set_chroma,
          true,
          0,
          &mv_stack,
          AngleDelta { y, uv },
        );
      }
      best.angle_delta
    };

    for i in 0..luma_deltas {
      let angle_delta_y =
        if luma_deltas == 1 { 0 } else { i - MAX_ANGLE_DELTA as i8 };
      best_angle_delta = angle_delta_rdo(angle_delta_y, best_angle_delta.uv);
    }
    for j in 0..chroma_deltas {
      let angle_delta_uv =
        if chroma_deltas == 1 { 0 } else { j - MAX_ANGLE_DELTA as i8 };
      best_angle_delta = angle_delta_rdo(best_angle_delta.y, angle_delta_uv);
    }
  }

  best
}

/// # Panics
///
/// - If the block size is invalid for subsampling.
#[profiling::function]
pub fn rdo_cfl_alpha<T: Pixel>(
  ts: &mut TileStateMut<'_, T>, tile_bo: TileBlockOffset, bsize: BlockSize,
  luma_tx_size: TxSize, fi: &FrameInvariants<T>,
) -> Option<CFLParams> {
  let PlaneConfig { xdec, ydec, .. } = ts.input.planes[1].cfg;
  let uv_tx_size = bsize.largest_chroma_tx_size(xdec, ydec);
  debug_assert!(
    bsize.subsampled_size(xdec, ydec).unwrap() == uv_tx_size.block_size()
  );

  let frame_bo = ts.to_frame_block_offset(tile_bo);
  let (visible_tx_w, visible_tx_h) = clip_visible_bsize(
    (fi.width + xdec) >> xdec,
    (fi.height + ydec) >> ydec,
    uv_tx_size.block_size(),
    (frame_bo.0.x << MI_SIZE_LOG2) >> xdec,
    (frame_bo.0.y << MI_SIZE_LOG2) >> ydec,
  );

  if visible_tx_w == 0 || visible_tx_h == 0 {
    return None;
  };
  let mut ac = Aligned::<[MaybeUninit<i16>; 32 * 32]>::uninit_array();
  let ac = luma_ac(&mut ac.data, ts, tile_bo, bsize, luma_tx_size, fi);
  let best_alpha: ArrayVec<i16, 2> = (1..3)
    .map(|p| {
      let &PlaneConfig { xdec, ydec, .. } = ts.rec.planes[p].plane_cfg;
      let tile_rect = ts.tile_rect().decimated(xdec, ydec);
      let rec = &mut ts.rec.planes[p];
      let input = &ts.input_tile.planes[p];
      let po = tile_bo.plane_offset(rec.plane_cfg);
      let mut edge_buf = Aligned::uninit_array();
      let edge_buf = get_intra_edges(
        &mut edge_buf,
        &rec.as_const(),
        tile_bo,
        0,
        0,
        bsize,
        po,
        uv_tx_size,
        fi.sequence.bit_depth,
        Some(PredictionMode::UV_CFL_PRED),
        fi.sequence.enable_intra_edge_filter,
        IntraParam::None,
      );
      let mut alpha_cost = |alpha: i16| -> u64 {
        let mut rec_region =
          rec.subregion_mut(Area::BlockStartingAt { bo: tile_bo.0 });
        PredictionMode::UV_CFL_PRED.predict_intra(
          tile_rect,
          &mut rec_region,
          uv_tx_size,
          fi.sequence.bit_depth,
          ac,
          IntraParam::Alpha(alpha),
          None,
          &edge_buf,
          fi.cpu_feature_level,
        );
        sse_wxh(
          &input.subregion(Area::BlockStartingAt { bo: tile_bo.0 }),
          &rec_region.as_const(),
          visible_tx_w,
          visible_tx_h,
          |_, _| DistortionScale::default(), // We're not doing RDO here.
          fi.sequence.bit_depth,
          fi.cpu_feature_level,
        )
        .0
      };
      let mut best = (alpha_cost(0), 0);
      let mut count = 2;
      for alpha in 1i16..=16i16 {
        let cost = (alpha_cost(alpha), alpha_cost(-alpha));
        if cost.0 < best.0 {
          best = (cost.0, alpha);
          count += 2;
        }
        if cost.1 < best.0 {
          best = (cost.1, -alpha);
          count += 2;
        }
        if count < alpha {
          break;
        }
      }
      best.1
    })
    .collect();

  if best_alpha[0] == 0 && best_alpha[1] == 0 {
    None
  } else {
    Some(CFLParams::from_alpha(best_alpha[0], best_alpha[1]))
  }
}

/// RDO-based transform type decision
/// If `cw_checkpoint` is `None`, a checkpoint for cw's (`ContextWriter`) current
/// state is created and stored for later use.
///
/// # Panics
///
/// - If a writer checkpoint is never created before or within the function.
///   This should never happen and indicates a development error.
/// - If the best RD found is negative.
///   This should never happen and indicates a development error.
pub fn rdo_tx_type_decision<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, cw_checkpoint: &mut Option<ContextWriterCheckpoint>,
  mode: PredictionMode, ref_frames: [RefType; 2], mvs: [MotionVector; 2],
  bsize: BlockSize, tile_bo: TileBlockOffset, tx_size: TxSize, tx_set: TxSet,
  tx_types: &[TxType], cur_best_rd: f64,
) -> (TxType, f64) {
  let mut best_type = TxType::DCT_DCT;
  let mut best_rd = f64::MAX;

  let PlaneConfig { xdec, ydec, .. } = ts.input.planes[1].cfg;
  let is_chroma_block =
    has_chroma(tile_bo, bsize, xdec, ydec, fi.sequence.chroma_sampling);

  let is_inter = !mode.is_intra();

  if cw_checkpoint.is_none() {
    // Only run the first call
    // Prevents creating multiple checkpoints for own version of cw
    *cw_checkpoint =
      Some(cw.checkpoint(&tile_bo, fi.sequence.chroma_sampling));
  }

  let rdo_type = if fi.use_tx_domain_distortion {
    RDOType::TxDistRealRate
  } else {
    RDOType::PixelDistRealRate
  };
  let need_recon_pixel = tx_size.block_size() != bsize && !is_inter;

  let mut first_iteration = true;
  for &tx_type in tx_types {
    // Skip unsupported transform types
    if av1_tx_used[tx_set as usize][tx_type as usize] == 0 {
      continue;
    }

    if is_inter {
      motion_compensate(
        fi, ts, cw, mode, ref_frames, mvs, bsize, tile_bo, true,
      );
    }

    let mut wr = WriterCounter::new();
    let tell = wr.tell_frac();
    let (_, tx_dist) = if is_inter {
      write_tx_tree(
        fi,
        ts,
        cw,
        &mut wr,
        mode,
        0,
        tile_bo,
        bsize,
        tx_size,
        tx_type,
        false,
        true,
        rdo_type,
        need_recon_pixel,
      )
    } else {
      write_tx_blocks(
        fi,
        ts,
        cw,
        &mut wr,
        mode,
        mode,
        AngleDelta::default(),
        tile_bo,
        bsize,
        tx_size,
        tx_type,
        false,
        CFLParams::default(), // Unused.
        true,
        rdo_type,
        need_recon_pixel,
      )
    };

    let rate = wr.tell_frac() - tell;
    let distortion = if fi.use_tx_domain_distortion {
      compute_tx_distortion(
        fi,
        ts,
        bsize,
        is_chroma_block,
        tile_bo,
        tx_dist,
        false,
        true,
      )
    } else {
      compute_distortion(fi, ts, bsize, is_chroma_block, tile_bo, true)
    };
    cw.rollback(cw_checkpoint.as_ref().unwrap());

    let rd = compute_rd_cost(fi, rate, distortion);

    if first_iteration {
      // We use an optimization to early exit after testing the first
      // transform type if the cost is higher than the existing best.
      // The idea is that if this transform size is not better than he
      // previous size, it is not worth testing remaining modes for this size.
      if rd > cur_best_rd {
        break;
      }
      first_iteration = false;
    }

    if rd < best_rd {
      best_rd = rd;
      best_type = tx_type;
    }
  }

  assert!(best_rd >= 0_f64);

  (best_type, best_rd)
}

pub fn get_sub_partitions(
  four_partitions: &[TileBlockOffset; 4], partition: PartitionType,
) -> ArrayVec<TileBlockOffset, 4> {
  let mut partition_offsets = ArrayVec::<TileBlockOffset, 4>::new();

  partition_offsets.push(four_partitions[0]);

  if partition == PARTITION_NONE {
    return partition_offsets;
  }
  if partition == PARTITION_VERT || partition == PARTITION_SPLIT {
    partition_offsets.push(four_partitions[1]);
  };
  if partition == PARTITION_HORZ || partition == PARTITION_SPLIT {
    partition_offsets.push(four_partitions[2]);
  };
  if partition == PARTITION_SPLIT {
    partition_offsets.push(four_partitions[3]);
  };

  partition_offsets
}

#[inline(always)]
fn rdo_partition_none<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, bsize: BlockSize, tile_bo: TileBlockOffset,
  inter_cfg: &InterConfig, child_modes: &mut ArrayVec<PartitionParameters, 4>,
) -> f64 {
  debug_assert!(tile_bo.0.x < ts.mi_width && tile_bo.0.y < ts.mi_height);

  let mode = rdo_mode_decision(fi, ts, cw, bsize, tile_bo, inter_cfg);
  let cost = mode.rd_cost;

  child_modes.push(mode);

  cost
}

// VERTICAL, HORIZONTAL or simple SPLIT
#[inline(always)]
fn rdo_partition_simple<T: Pixel, W: Writer>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, w_pre_cdef: &mut W, w_post_cdef: &mut W,
  bsize: BlockSize, tile_bo: TileBlockOffset, inter_cfg: &InterConfig,
  partition: PartitionType, rdo_type: RDOType, best_rd: f64,
  child_modes: &mut ArrayVec<PartitionParameters, 4>,
) -> Option<f64> {
  debug_assert!(tile_bo.0.x < ts.mi_width && tile_bo.0.y < ts.mi_height);
  let subsize = bsize.subsize(partition).unwrap();

  let cost = if bsize >= BlockSize::BLOCK_8X8 {
    let w: &mut W = if cw.bc.cdef_coded { w_post_cdef } else { w_pre_cdef };
    let tell = w.tell_frac();
    cw.write_partition(w, tile_bo, partition, bsize);
    compute_rd_cost(fi, w.tell_frac() - tell, ScaledDistortion::zero())
  } else {
    0.0
  };

  let hbsw = subsize.width_mi(); // Half the block size width in blocks
  let hbsh = subsize.height_mi(); // Half the block size height in blocks
  let four_partitions = [
    tile_bo,
    TileBlockOffset(BlockOffset { x: tile_bo.0.x + hbsw, y: tile_bo.0.y }),
    TileBlockOffset(BlockOffset { x: tile_bo.0.x, y: tile_bo.0.y + hbsh }),
    TileBlockOffset(BlockOffset {
      x: tile_bo.0.x + hbsw,
      y: tile_bo.0.y + hbsh,
    }),
  ];

  let partitions = get_sub_partitions(&four_partitions, partition);

  let mut rd_cost_sum = 0.0;

  for offset in partitions {
    let hbs = subsize.width_mi() >> 1;
    let has_cols = offset.0.x + hbs < ts.mi_width;
    let has_rows = offset.0.y + hbs < ts.mi_height;

    if has_cols && has_rows {
      let mode_decision =
        rdo_mode_decision(fi, ts, cw, subsize, offset, inter_cfg);

      rd_cost_sum += mode_decision.rd_cost;

      if fi.enable_early_exit && rd_cost_sum > best_rd {
        return None;
      }
      if subsize >= BlockSize::BLOCK_8X8 && subsize.is_sqr() {
        let w: &mut W =
          if cw.bc.cdef_coded { w_post_cdef } else { w_pre_cdef };
        cw.write_partition(w, offset, PartitionType::PARTITION_NONE, subsize);
      }
      encode_block_with_modes(
        fi,
        ts,
        cw,
        w_pre_cdef,
        w_post_cdef,
        subsize,
        offset,
        &mode_decision,
        rdo_type,
        None,
      );
      child_modes.push(mode_decision);
    } else {
      //rd_cost_sum += f64::MAX;
      return None;
    }
  }

  Some(cost + rd_cost_sum)
}

/// RDO-based single level partitioning decision
///
/// # Panics
///
/// - If the best RD found is negative.
///   This should never happen, and indicates a development error.
#[profiling::function]
pub fn rdo_partition_decision<T: Pixel, W: Writer>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  cw: &mut ContextWriter, w_pre_cdef: &mut W, w_post_cdef: &mut W,
  bsize: BlockSize, tile_bo: TileBlockOffset,
  cached_block: &PartitionGroupParameters, partition_types: &[PartitionType],
  rdo_type: RDOType, inter_cfg: &InterConfig,
) -> PartitionGroupParameters {
  let mut best_partition = cached_block.part_type;
  let mut best_rd = cached_block.rd_cost;
  let mut best_pred_modes = cached_block.part_modes.clone();

  let cw_checkpoint = cw.checkpoint(&tile_bo, fi.sequence.chroma_sampling);
  let w_pre_checkpoint = w_pre_cdef.checkpoint();
  let w_post_checkpoint = w_post_cdef.checkpoint();

  for &partition in partition_types {
    // Do not re-encode results we already have
    if partition == cached_block.part_type {
      continue;
    }

    let mut child_modes = ArrayVec::<_, 4>::new();

    let cost = match partition {
      PARTITION_NONE if bsize <= BlockSize::BLOCK_64X64 => {
        Some(rdo_partition_none(
          fi,
          ts,
          cw,
          bsize,
          tile_bo,
          inter_cfg,
          &mut child_modes,
        ))
      }
      PARTITION_SPLIT | PARTITION_HORZ | PARTITION_VERT => {
        rdo_partition_simple(
          fi,
          ts,
          cw,
          w_pre_cdef,
          w_post_cdef,
          bsize,
          tile_bo,
          inter_cfg,
          partition,
          rdo_type,
          best_rd,
          &mut child_modes,
        )
      }
      _ => {
        unreachable!();
      }
    };

    if let Some(rd) = cost {
      if rd < best_rd {
        best_rd = rd;
        best_partition = partition;
        best_pred_modes.clone_from(&child_modes);
      }
    }
    cw.rollback(&cw_checkpoint);
    w_pre_cdef.rollback(&w_pre_checkpoint);
    w_post_cdef.rollback(&w_post_checkpoint);
  }

  assert!(best_rd >= 0_f64);

  PartitionGroupParameters {
    rd_cost: best_rd,
    part_type: best_partition,
    part_modes: best_pred_modes,
  }
}

#[profiling::function]
fn rdo_loop_plane_error<T: Pixel>(
  base_sbo: TileSuperBlockOffset, offset_sbo: TileSuperBlockOffset,
  sb_w: usize, sb_h: usize, fi: &FrameInvariants<T>, ts: &TileStateMut<'_, T>,
  blocks: &TileBlocks<'_>, test: &Frame<T>, src: &Tile<'_, T>, pli: usize,
) -> ScaledDistortion {
  let sb_w_blocks =
    if fi.sequence.use_128x128_superblock { 16 } else { 8 } * sb_w;
  let sb_h_blocks =
    if fi.sequence.use_128x128_superblock { 16 } else { 8 } * sb_h;
  // Each direction block is 8x8 in y, potentially smaller if subsampled in chroma
  // accumulating in-frame and unpadded
  let mut err = Distortion::zero();
  for by in 0..sb_h_blocks {
    for bx in 0..sb_w_blocks {
      let loop_bo = offset_sbo.block_offset(bx << 1, by << 1);
      if loop_bo.0.x < blocks.cols() && loop_bo.0.y < blocks.rows() {
        let src_plane = &src.planes[pli];
        let test_plane = &test.planes[pli];
        let PlaneConfig { xdec, ydec, .. } = *src_plane.plane_cfg;
        debug_assert_eq!(xdec, test_plane.cfg.xdec);
        debug_assert_eq!(ydec, test_plane.cfg.ydec);

        // Unfortunately, our distortion biases are only available via
        // Frame-absolute addressing, so we need a block offset
        // relative to the full frame origin (not the tile or analysis
        // area)
        let frame_bo = (base_sbo + offset_sbo).block_offset(bx << 1, by << 1);
        let bias = distortion_scale(
          fi,
          ts.to_frame_block_offset(frame_bo),
          BlockSize::BLOCK_8X8,
        );

        let src_region =
          src_plane.subregion(Area::BlockStartingAt { bo: loop_bo.0 });
        let test_region =
          test_plane.region(Area::BlockStartingAt { bo: loop_bo.0 });

        err += if pli == 0 {
          // For loop filters, We intentionally use cdef_dist even with
          // `--tune Psnr`. Using SSE instead gives no PSNR gain but has a
          // significant negative impact on other metrics and visual quality.
          RawDistortion(cdef_dist_kernel(
            &src_region,
            &test_region,
            8,
            8,
            fi.sequence.bit_depth,
            fi.cpu_feature_level,
          ) as u64)
            * bias
        } else {
          sse_wxh(
            &src_region,
            &test_region,
            8 >> xdec,
            8 >> ydec,
            |_, _| bias,
            fi.sequence.bit_depth,
            fi.cpu_feature_level,
          )
        };
      }
    }
  }
  err * fi.dist_scale[pli]
}

/// Passed in a superblock offset representing the upper left corner of
/// the LRU area we're optimizing.  This area covers the largest LRU in
/// any of the present planes, but may consist of a number of
/// superblocks and full, smaller LRUs in the other planes
///
/// # Panics
///
/// - If both CDEF and LRF are disabled.
#[profiling::function]
pub fn rdo_loop_decision<T: Pixel, W: Writer>(
  base_sbo: TileSuperBlockOffset, fi: &FrameInvariants<T>,
  ts: &mut TileStateMut<'_, T>, cw: &mut ContextWriter, w: &mut W,
  deblock_p: bool,
) {
  let planes = if fi.sequence.chroma_sampling == ChromaSampling::Cs400 {
    1
  } else {
    MAX_PLANES
  };
  assert!(fi.sequence.enable_cdef || fi.sequence.enable_restoration);
  // Determine area of optimization: Which plane has the largest LRUs?
  // How many LRUs for each?
  let mut sb_w = 1; // how many superblocks wide the largest LRU
                    // is/how many SBs we're processing (same thing)
  let mut sb_h = 1; // how many superblocks wide the largest LRU
                    // is/how many SBs we're processing (same thing)
  let mut lru_w = [0; MAX_PLANES]; // how many LRUs we're processing
  let mut lru_h = [0; MAX_PLANES]; // how many LRUs we're processing
  for pli in 0..planes {
    let sb_h_shift = ts.restoration.planes[pli].rp_cfg.sb_h_shift;
    let sb_v_shift = ts.restoration.planes[pli].rp_cfg.sb_v_shift;
    if sb_w < (1 << sb_h_shift) {
      sb_w = 1 << sb_h_shift;
    }
    if sb_h < (1 << sb_v_shift) {
      sb_h = 1 << sb_v_shift;
    }
  }
  for pli in 0..planes {
    let sb_h_shift = ts.restoration.planes[pli].rp_cfg.sb_h_shift;
    let sb_v_shift = ts.restoration.planes[pli].rp_cfg.sb_v_shift;
    lru_w[pli] = sb_w / (1 << sb_h_shift);
    lru_h[pli] = sb_h / (1 << sb_v_shift);
  }

  // The superblock width/height determinations may be calling for us
  // to compute over superblocks that do not actually exist in the
  // frame (off the right or lower edge).  Trim sb width/height down
  // to actual superblocks.  Note that these last superblocks on the
  // right/bottom may themselves still span the edge of the frame, but
  // they do hold at least some visible pixels.
  sb_w = sb_w.min(ts.sb_width - base_sbo.0.x);
  sb_h = sb_h.min(ts.sb_height - base_sbo.0.y);

  // We have need to know the Y visible pixel limits as well (the
  // sb_w/sb_h figures above can be used to determine how many
  // allocated pixels, possibly beyond the visible frame, exist).
  let crop_w =
    fi.width - ((ts.sbo.0.x + base_sbo.0.x) << SUPERBLOCK_TO_PLANE_SHIFT);
  let crop_h =
    fi.height - ((ts.sbo.0.y + base_sbo.0.y) << SUPERBLOCK_TO_PLANE_SHIFT);
  let pixel_w = crop_w.min(sb_w << SUPERBLOCK_TO_PLANE_SHIFT);
  let pixel_h = crop_h.min(sb_h << SUPERBLOCK_TO_PLANE_SHIFT);

  // Based on `RestorationState::new`
  const MAX_SB_SHIFT: usize = 4;
  const MAX_SB_SIZE: usize = 1 << MAX_SB_SHIFT;
  const MAX_LRU_SIZE: usize = MAX_SB_SIZE;

  // Static allocation relies on the "minimal LRU area for all N planes" invariant.
  let mut best_index = [-1; MAX_SB_SIZE * MAX_SB_SIZE];
  let mut best_lrf =
    [[RestorationFilter::None; MAX_PLANES]; MAX_LRU_SIZE * MAX_LRU_SIZE];

  // due to imprecision in the reconstruction parameter solver, we
  // need to make sure we don't fall into a limit cycle.  Track our
  // best cost at LRF so that we can break if we get a solution that doesn't
  // improve at the reconstruction stage.
  let mut best_lrf_cost = [[-1.0; MAX_PLANES]; MAX_LRU_SIZE * MAX_LRU_SIZE];

  // sub-setted region of the TileBlocks for our working frame area.
  // Note that the size of this subset is what signals CDEF as to the
  // actual coded size.
  let mut tileblocks_subset = cw.bc.blocks.subregion_mut(
    base_sbo.block_offset(0, 0).0.x,
    base_sbo.block_offset(0, 0).0.y,
    sb_w << SUPERBLOCK_TO_BLOCK_SHIFT,
    sb_h << SUPERBLOCK_TO_BLOCK_SHIFT,
  );

  // cdef doesn't run on superblocks that are completely skipped.
  // Determine which super blocks are marked as skipped so we can avoid running
  // them. If all blocks are skipped, we can avoid some of the overhead related
  // to setting up for cdef.
  let mut cdef_skip = [true; MAX_SB_SIZE * MAX_SB_SIZE];
  let mut cdef_skip_all = true;
  if fi.sequence.enable_cdef {
    for sby in 0..sb_h {
      for sbx in 0..sb_w {
        let blocks = tileblocks_subset.subregion(16 * sbx, 16 * sby, 16, 16);
        let mut skip = true;
        for y in 0..blocks.rows() {
          for block in blocks[y].iter() {
            skip &= block.skip;
          }
        }
        cdef_skip[sby * MAX_SB_SIZE + sbx] = skip;
        cdef_skip_all &= skip;
      }
    }
  }

  // Unlike cdef, loop restoration will run regardless of whether blocks are
  // skipped or not. At the same time, the most significant improvement will
  // generally be from un-skipped blocks, so lru is only performed if there are
  // un-skipped blocks.
  // This should be the same as `cdef_skip_all`, except when cdef is disabled.
  let mut lru_skip_all = true;
  let mut lru_skip = [[true; MAX_PLANES]; MAX_LRU_SIZE * MAX_LRU_SIZE];
  if fi.sequence.enable_restoration {
    if fi.config.speed_settings.lru_on_skip {
      lru_skip_all = false;
      lru_skip = [[false; MAX_PLANES]; MAX_LRU_SIZE * MAX_LRU_SIZE];
    } else {
      for pli in 0..planes {
        // width, in sb, of an LRU in this plane
        let lru_sb_w = 1 << ts.restoration.planes[pli].rp_cfg.sb_h_shift;
        // height, in sb, of an LRU in this plane
        let lru_sb_h = 1 << ts.restoration.planes[pli].rp_cfg.sb_v_shift;
        for lru_y in 0..lru_h[pli] {
          // number of LRUs vertically
          for lru_x in 0..lru_w[pli] {
            // number of LRUs horizontally

            let loop_sbo = TileSuperBlockOffset(SuperBlockOffset {
              x: lru_x * lru_sb_w,
              y: lru_y * lru_sb_h,
            });

            if !ts.restoration.has_restoration_unit(
              base_sbo + loop_sbo,
              pli,
              false,
            ) {
              continue;
            }

            let start = loop_sbo.block_offset(0, 0).0;
            let size = TileSuperBlockOffset(SuperBlockOffset {
              x: lru_sb_w,
              y: lru_sb_h,
            })
            .block_offset(0, 0)
            .0;

            let blocks =
              tileblocks_subset.subregion(start.x, start.y, size.x, size.y);
            let mut skip = true;
            for y in 0..blocks.rows() {
              for block in blocks[y].iter() {
                skip &= block.skip;
              }
            }
            lru_skip[lru_y * MAX_LRU_SIZE + lru_x][pli] = skip;
            lru_skip_all &= skip;
          }
        }
      }
    }
  }

  // Return early if all blocks are skipped for lru and cdef.
  if lru_skip_all && cdef_skip_all {
    return;
  }

  // Loop filter RDO is an iterative process and we need temporary
  // scratch data to hold the results of deblocking, cdef, and the
  // loop reconstruction filter so that each can be partially updated
  // without recomputing the entire stack.  Construct
  // largest-LRU-sized frames for each, accounting for padding
  // required by deblocking, cdef and [optionally] LR.
  let mut rec_subset = ts
    .rec
    .subregion(Area::BlockRect {
      bo: base_sbo.block_offset(0, 0).0,
      width: (pixel_w + 7) >> 3 << 3,
      height: (pixel_h + 7) >> 3 << 3,
    })
    .scratch_copy();

  // const, no need to copy, just need the subregion (but do zero the
  // origin to match the other copies/new backing frames).
  let src_subset = ts
    .input_tile
    .subregion(Area::BlockRect {
      bo: base_sbo.block_offset(0, 0).0,
      width: (pixel_w + 7) >> 3 << 3,
      height: (pixel_h + 7) >> 3 << 3,
    })
    .home();

  if deblock_p {
    // Find a good deblocking filter solution for the passed in area.
    // This is not RDO of deblocking itself, merely a solution to get
    // better results from CDEF/LRF RDO.
    let deblock_levels = deblock_filter_optimize(
      fi,
      &rec_subset.as_tile(),
      &src_subset,
      &tileblocks_subset.as_const(),
      crop_w,
      crop_h,
    );

    // Deblock the contents of our reconstruction copy.
    if deblock_levels[0] != 0 || deblock_levels[1] != 0 {
      // copy ts.deblock because we need to set some of our own values here
      let mut deblock_copy = *ts.deblock;
      deblock_copy.levels = deblock_levels;

      // finally, deblock the temp frame
      deblock_filter_frame(
        &deblock_copy,
        &mut rec_subset.as_tile_mut(),
        &tileblocks_subset.as_const(),
        crop_w,
        crop_h,
        fi.sequence.bit_depth,
        planes,
      );
    }
  }

  let mut cdef_work =
    if !cdef_skip_all { Some(rec_subset.clone()) } else { None };
  let mut lrf_work = if !lru_skip_all {
    Some(Frame {
      planes: {
        let new_plane = |pli: usize| {
          let PlaneConfig { xdec, ydec, width, height, .. } =
            rec_subset.planes[pli].cfg;
          Plane::new(width, height, xdec, ydec, 0, 0)
        };
        [new_plane(0), new_plane(1), new_plane(2)]
      },
    })
  } else {
    None
  };

  // Precompute directional analysis for CDEF
  let cdef_data = {
    if cdef_work.is_some() {
      Some((
        &rec_subset,
        cdef_analyze_superblock_range(
          fi,
          &rec_subset,
          &tileblocks_subset.as_const(),
          sb_w,
          sb_h,
        ),
      ))
    } else {
      None
    }
  };

  // CDEF/LRF decision iteration
  // Start with a default of CDEF 0 and RestorationFilter::None
  // Try all CDEF options for each sb with current LRF; if new CDEF+LRF choice is better, select it.
  // Then try all LRF options with current CDEFs; if new CDEFs+LRF choice is better, select it.
  // If LRF choice changed for any plane, repeat until no changes
  // Limit iterations and where we break based on speed setting (in the TODO list ;-)
  let mut cdef_change = true;
  let mut lrf_change = true;
  while cdef_change || lrf_change {
    // search for improved cdef indices, superblock by superblock, if cdef is enabled.
    if let (Some((rec_copy, cdef_dirs)), Some(cdef_ref)) =
      (&cdef_data, &mut cdef_work.as_mut())
    {
      for sby in 0..sb_h {
        for sbx in 0..sb_w {
          // determine whether this superblock can be skipped
          if cdef_skip[sby * MAX_SB_SIZE + sbx] {
            continue;
          }

          let prev_best_index = best_index[sby * sb_w + sbx];
          let mut best_cost = -1.;
          let mut best_new_index = -1i8;

          /* offset of the superblock we're currently testing within the larger
          analysis area */
          let loop_sbo =
            TileSuperBlockOffset(SuperBlockOffset { x: sbx, y: sby });

          /* cdef index testing loop */
          for cdef_index in 0..(1 << fi.cdef_bits) {
            let mut err = ScaledDistortion::zero();
            let mut rate = 0;

            cdef_filter_superblock(
              fi,
              &rec_subset,
              &mut cdef_ref.as_tile_mut(),
              &tileblocks_subset.as_const(),
              loop_sbo,
              cdef_index,
              &cdef_dirs[sby * sb_w + sbx],
            );
            // apply LRF if any
            for pli in 0..planes {
              // We need the cropped-to-visible-frame area of this SB
              let wh =
                if fi.sequence.use_128x128_superblock { 128 } else { 64 };
              let PlaneConfig { xdec, ydec, .. } = cdef_ref.planes[pli].cfg;
              let vis_width = (wh >> xdec).min(
                (crop_w >> xdec)
                  - loop_sbo.plane_offset(&cdef_ref.planes[pli].cfg).x
                    as usize,
              );
              let vis_height = (wh >> ydec).min(
                (crop_h >> ydec)
                  - loop_sbo.plane_offset(&cdef_ref.planes[pli].cfg).y
                    as usize,
              );
              // which LRU are we currently testing against?
              if let (Some((lru_x, lru_y)), Some(lrf_ref)) = {
                let rp = &ts.restoration.planes[pli];
                (
                  rp.restoration_unit_offset(base_sbo, loop_sbo, false),
                  &mut lrf_work,
                )
              } {
                // We have a valid LRU, apply LRF, compute error
                match best_lrf[lru_y * lru_w[pli] + lru_x][pli] {
                  RestorationFilter::None => {
                    err += rdo_loop_plane_error(
                      base_sbo,
                      loop_sbo,
                      1,
                      1,
                      fi,
                      ts,
                      &tileblocks_subset.as_const(),
                      cdef_ref,
                      &src_subset,
                      pli,
                    );
                    rate += if fi.sequence.enable_restoration {
                      cw.fc.count_lrf_switchable(
                        w,
                        &ts.restoration.as_const(),
                        best_lrf[lru_y * lru_w[pli] + lru_x][pli],
                        pli,
                      )
                    } else {
                      0 // no relative cost differeneces to different
                        // CDEF params.  If cdef is on, it's a wash.
                    };
                  }
                  RestorationFilter::Sgrproj { set, xqd } => {
                    // only run on this single superblock
                    let loop_po =
                      loop_sbo.plane_offset(&cdef_ref.planes[pli].cfg);
                    // todo: experiment with borrowing border pixels
                    // rather than edge-extending. Right now this is
                    // hard-clipping to the superblock boundary.
                    setup_integral_image(
                      &mut ts.integral_buffer,
                      SOLVE_IMAGE_STRIDE,
                      vis_width,
                      vis_height,
                      vis_width,
                      vis_height,
                      &cdef_ref.planes[pli].slice(loop_po),
                      &cdef_ref.planes[pli].slice(loop_po),
                    );
                    sgrproj_stripe_filter(
                      set,
                      xqd,
                      fi,
                      &ts.integral_buffer,
                      SOLVE_IMAGE_STRIDE,
                      &cdef_ref.planes[pli].slice(loop_po),
                      &mut lrf_ref.planes[pli].region_mut(Area::Rect {
                        x: loop_po.x,
                        y: loop_po.y,
                        width: vis_width,
                        height: vis_height,
                      }),
                    );
                    err += rdo_loop_plane_error(
                      base_sbo,
                      loop_sbo,
                      1,
                      1,
                      fi,
                      ts,
                      &tileblocks_subset.as_const(),
                      lrf_ref,
                      &src_subset,
                      pli,
                    );
                    rate += cw.fc.count_lrf_switchable(
                      w,
                      &ts.restoration.as_const(),
                      best_lrf[lru_y * lru_w[pli] + lru_x][pli],
                      pli,
                    );
                  }
                  RestorationFilter::Wiener { .. } => unreachable!(), // coming soon
                }
              } else {
                // No actual LRU here, compute error directly from CDEF output.
                err += rdo_loop_plane_error(
                  base_sbo,
                  loop_sbo,
                  1,
                  1,
                  fi,
                  ts,
                  &tileblocks_subset.as_const(),
                  cdef_ref,
                  &src_subset,
                  pli,
                );
                // no relative cost differeneces to different
                // CDEF params.  If cdef is on, it's a wash.
                // rate += 0;
              }
            }

            let cost = compute_rd_cost(fi, rate, err);
            if best_cost < 0. || cost < best_cost {
              best_cost = cost;
              best_new_index = cdef_index as i8;
            }
          }

          // Did we change any preexisting choices?
          if best_new_index != prev_best_index {
            cdef_change = true;
            best_index[sby * sb_w + sbx] = best_new_index;
            tileblocks_subset.set_cdef(loop_sbo, best_new_index as u8);
          }

          let mut cdef_ref_tm = TileMut::new(
            cdef_ref,
            TileRect {
              x: 0,
              y: 0,
              width: cdef_ref.planes[0].cfg.width,
              height: cdef_ref.planes[0].cfg.height,
            },
          );

          // Keep cdef output up to date; we need it for restoration
          // both below and above (padding)
          cdef_filter_superblock(
            fi,
            rec_copy,
            &mut cdef_ref_tm,
            &tileblocks_subset.as_const(),
            loop_sbo,
            best_index[sby * sb_w + sbx] as u8,
            &cdef_dirs[sby * sb_w + sbx],
          );
        }
      }
    }

    if !cdef_change {
      break;
    }
    cdef_change = false;
    lrf_change = false;

    // search for improved restoration filter parameters if restoration is enabled
    if let Some(lrf_ref) = &mut lrf_work.as_mut() {
      let lrf_input = if cdef_work.is_some() {
        // When CDEF is enabled, we pull from the CDEF output
        cdef_work.as_ref().unwrap()
      } else {
        // When CDEF is disabled, we pull from the [optionally
        // deblocked] reconstruction
        &rec_subset
      };
      for pli in 0..planes {
        // Nominal size of LRU in pixels before clipping to visible frame
        let unit_size = ts.restoration.planes[pli].rp_cfg.unit_size;
        // width, in sb, of an LRU in this plane
        let lru_sb_w = 1 << ts.restoration.planes[pli].rp_cfg.sb_h_shift;
        // height, in sb, of an LRU in this plane
        let lru_sb_h = 1 << ts.restoration.planes[pli].rp_cfg.sb_v_shift;
        let PlaneConfig { xdec, ydec, .. } = lrf_ref.planes[pli].cfg;
        for lru_y in 0..lru_h[pli] {
          // number of LRUs vertically
          for lru_x in 0..lru_w[pli] {
            // number of LRUs horizontally

            // determine whether this lru should be skipped
            if lru_skip[lru_y * MAX_LRU_SIZE + lru_x][pli] {
              continue;
            }

            let loop_sbo = TileSuperBlockOffset(SuperBlockOffset {
              x: lru_x * lru_sb_w,
              y: lru_y * lru_sb_h,
            });
            if ts.restoration.has_restoration_unit(
              base_sbo + loop_sbo,
              pli,
              false,
            ) {
              let src_plane = &src_subset.planes[pli]; // uncompressed input for reference
              let lrf_in_plane = &lrf_input.planes[pli];
              let lrf_po = loop_sbo.plane_offset(src_plane.plane_cfg);
              let mut best_new_lrf = best_lrf[lru_y * lru_w[pli] + lru_x][pli];
              let mut best_cost =
                best_lrf_cost[lru_y * lru_w[pli] + lru_x][pli];

              // Check the no filter option
              {
                let err = rdo_loop_plane_error(
                  base_sbo,
                  loop_sbo,
                  lru_sb_w,
                  lru_sb_h,
                  fi,
                  ts,
                  &tileblocks_subset.as_const(),
                  lrf_input,
                  &src_subset,
                  pli,
                );
                let rate = cw.fc.count_lrf_switchable(
                  w,
                  &ts.restoration.as_const(),
                  best_new_lrf,
                  pli,
                );

                let cost = compute_rd_cost(fi, rate, err);
                // Was this choice actually an improvement?
                if best_cost < 0. || cost < best_cost {
                  best_cost = cost;
                  best_lrf_cost[lru_y * lru_w[pli] + lru_x][pli] = cost;
                  best_new_lrf = RestorationFilter::None;
                }
              }

              // Look for a self guided filter
              // We need the cropped-to-visible-frame computation area of this LRU
              let vis_width = unit_size.min(
                (crop_w >> xdec)
                  - loop_sbo.plane_offset(&lrf_ref.planes[pli].cfg).x as usize,
              );
              let vis_height = unit_size.min(
                (crop_h >> ydec)
                  - loop_sbo.plane_offset(&lrf_ref.planes[pli].cfg).y as usize,
              );

              // todo: experiment with borrowing border pixels
              // rather than edge-extending. Right now this is
              // hard-clipping to the superblock boundary.
              setup_integral_image(
                &mut ts.integral_buffer,
                SOLVE_IMAGE_STRIDE,
                vis_width,
                vis_height,
                vis_width,
                vis_height,
                &lrf_in_plane.slice(lrf_po),
                &lrf_in_plane.slice(lrf_po),
              );

              for &set in get_sgr_sets(fi.config.speed_settings.sgr_complexity)
              {
                let (xqd0, xqd1) = sgrproj_solve(
                  set,
                  fi,
                  &ts.integral_buffer,
                  &src_plane
                    .subregion(Area::StartingAt { x: lrf_po.x, y: lrf_po.y }),
                  &lrf_in_plane.slice(lrf_po),
                  vis_width,
                  vis_height,
                );
                let current_lrf =
                  RestorationFilter::Sgrproj { set, xqd: [xqd0, xqd1] };
                if let RestorationFilter::Sgrproj { set, xqd } = current_lrf {
                  sgrproj_stripe_filter(
                    set,
                    xqd,
                    fi,
                    &ts.integral_buffer,
                    SOLVE_IMAGE_STRIDE,
                    &lrf_in_plane.slice(lrf_po),
                    &mut lrf_ref.planes[pli].region_mut(Area::Rect {
                      x: lrf_po.x,
                      y: lrf_po.y,
                      width: vis_width,
                      height: vis_height,
                    }),
                  );
                }
                let err = rdo_loop_plane_error(
                  base_sbo,
                  loop_sbo,
                  lru_sb_w,
                  lru_sb_h,
                  fi,
                  ts,
                  &tileblocks_subset.as_const(),
                  lrf_ref,
                  &src_subset,
                  pli,
                );
                let rate = cw.fc.count_lrf_switchable(
                  w,
                  &ts.restoration.as_const(),
                  current_lrf,
                  pli,
                );
                let cost = compute_rd_cost(fi, rate, err);
                if cost < best_cost {
                  best_cost = cost;
                  best_lrf_cost[lru_y * lru_w[pli] + lru_x][pli] = cost;
                  best_new_lrf = current_lrf;
                }
              }

              if best_lrf[lru_y * lru_w[pli] + lru_x][pli]
                .notequal(best_new_lrf)
              {
                best_lrf[lru_y * lru_w[pli] + lru_x][pli] = best_new_lrf;
                lrf_change = true;
                if let Some(ru) = ts.restoration.planes[pli]
                  .restoration_unit_mut(base_sbo + loop_sbo)
                {
                  ru.filter = best_new_lrf;
                }
              }
            }
          }
        }
      }
    }
  }
}

#[test]
fn estimate_rate_test() {
  assert_eq!(estimate_rate(0, TxSize::TX_4X4, 0), RDO_RATE_TABLE[0][0][0]);
}
