// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;

use crate::context::*;
use crate::encoder::*;
use crate::frame::*;
use crate::lrf::{IntegralImageBuffer, SOLVE_IMAGE_SIZE};
use crate::mc::MotionVector;
use crate::me::FrameMEStats;
use crate::me::WriteGuardMEStats;
use crate::partition::{RefType, REF_FRAMES};
use crate::predict::{InterCompoundBuffers, PredictionMode};
use crate::quantize::*;
use crate::rdo::*;
use crate::stats::EncoderStats;
use crate::util::*;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

/// Tiled view of `FrameState`
///
/// Contrary to `PlaneRegionMut` and `TileMut`, there is no const version:
///  - in practice, we don't need it;
///  - it would require to instantiate a const version of every of its inner
///    tiled views recursively.
///
/// # `TileState` fields
///
/// The way the `FrameState` fields are mapped depend on how they are accessed
/// tile-wise and frame-wise.
///
/// Some fields (like `qc`) are only used during tile-encoding, so they are only
/// stored in `TileState`.
///
/// Some other fields (like `input` or `segmentation`) are not written
/// tile-wise, so they just reference the matching field in `FrameState`.
///
/// Some others (like `rec`) are written tile-wise, but must be accessible
/// frame-wise once the tile views vanish (e.g. for deblocking).
#[derive(Debug)]
pub struct TileStateMut<'a, T: Pixel> {
  pub sbo: PlaneSuperBlockOffset,
  pub sb_size_log2: usize,
  pub sb_width: usize,
  pub sb_height: usize,
  pub mi_width: usize,
  pub mi_height: usize,
  pub width: usize,
  pub height: usize,
  pub input: &'a Frame<T>,     // the whole frame
  pub input_tile: Tile<'a, T>, // the current tile
  pub input_hres: &'a Plane<T>,
  pub input_qres: &'a Plane<T>,
  pub deblock: &'a DeblockState,
  pub rec: TileMut<'a, T>,
  pub qc: QuantizationContext,
  pub segmentation: &'a SegmentationState,
  pub restoration: TileRestorationStateMut<'a>,
  pub me_stats: Vec<TileMEStatsMut<'a>>,
  pub coded_block_info: MiTileState,
  pub integral_buffer: IntegralImageBuffer,
  pub inter_compound_buffers: InterCompoundBuffers,
}

/// Contains information for a coded block that is
/// useful to persist. For example, the intra edge
/// filter requires surrounding coded block information.
#[derive(Debug, Clone, Copy)]
pub struct CodedBlockInfo {
  pub luma_mode: PredictionMode,
  pub chroma_mode: PredictionMode,
  pub reference_types: [RefType; 2],
}

impl Default for CodedBlockInfo {
  fn default() -> Self {
    CodedBlockInfo {
      luma_mode: PredictionMode::DC_PRED,
      chroma_mode: PredictionMode::DC_PRED,
      reference_types: [RefType::INTRA_FRAME, RefType::NONE_FRAME],
    }
  }
}

#[derive(Debug, Clone)]
pub struct MiTileState {
  mi_width: usize,
  mi_height: usize,
  mi_block_info: Vec<CodedBlockInfo>,
}

impl MiTileState {
  pub fn new(mi_width: usize, mi_height: usize) -> Self {
    MiTileState {
      mi_width,
      mi_height,
      mi_block_info: vec![CodedBlockInfo::default(); mi_width * mi_height],
    }
  }
}

impl Index<usize> for MiTileState {
  type Output = [CodedBlockInfo];

  #[inline(always)]
  fn index(&self, index: usize) -> &Self::Output {
    &self.mi_block_info[index * self.mi_width..(index + 1) * self.mi_width]
  }
}

impl IndexMut<usize> for MiTileState {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.mi_block_info[index * self.mi_width..(index + 1) * self.mi_width]
  }
}

impl<'a, T: Pixel> TileStateMut<'a, T> {
  pub fn new(
    fs: &'a mut FrameState<T>, sbo: PlaneSuperBlockOffset,
    sb_size_log2: usize, width: usize, height: usize,
    frame_me_stats: &'a mut [FrameMEStats],
  ) -> Self {
    debug_assert!(
      width % MI_SIZE == 0,
      "Tile width must be a multiple of MI_SIZE"
    );
    debug_assert!(
      height % MI_SIZE == 0,
      "Tile width must be a multiple of MI_SIZE"
    );

    let sb_rounded_width = width.align_power_of_two(sb_size_log2);
    let sb_rounded_height = height.align_power_of_two(sb_size_log2);

    let luma_rect = TileRect {
      x: sbo.0.x << sb_size_log2,
      y: sbo.0.y << sb_size_log2,
      width: sb_rounded_width,
      height: sb_rounded_height,
    };
    let sb_width = width.align_power_of_two_and_shift(sb_size_log2);
    let sb_height = height.align_power_of_two_and_shift(sb_size_log2);

    Self {
      sbo,
      sb_size_log2,
      sb_width,
      sb_height,
      mi_width: width >> MI_SIZE_LOG2,
      mi_height: height >> MI_SIZE_LOG2,
      width,
      height,
      input: &fs.input,
      input_tile: Tile::new(&fs.input, luma_rect),
      input_hres: &fs.input_hres,
      input_qres: &fs.input_qres,
      deblock: &fs.deblock,
      rec: TileMut::new(Arc::make_mut(&mut fs.rec), luma_rect),
      qc: Default::default(),
      segmentation: &fs.segmentation,
      restoration: TileRestorationStateMut::new(
        &mut fs.restoration,
        sbo,
        sb_width,
        sb_height,
      ),
      me_stats: frame_me_stats
        .iter_mut()
        .map(|fmvs| {
          TileMEStatsMut::new(
            fmvs,
            sbo.0.x << (sb_size_log2 - MI_SIZE_LOG2),
            sbo.0.y << (sb_size_log2 - MI_SIZE_LOG2),
            width >> MI_SIZE_LOG2,
            height >> MI_SIZE_LOG2,
          )
        })
        .collect(),
      coded_block_info: MiTileState::new(
        width >> MI_SIZE_LOG2,
        height >> MI_SIZE_LOG2,
      ),
      integral_buffer: IntegralImageBuffer::zeroed(SOLVE_IMAGE_SIZE),
      inter_compound_buffers: InterCompoundBuffers::default(),
    }
  }

  #[inline(always)]
  pub fn tile_rect(&self) -> TileRect {
    TileRect {
      x: self.sbo.0.x << self.sb_size_log2,
      y: self.sbo.0.y << self.sb_size_log2,
      width: self.width,
      height: self.height,
    }
  }

  #[inline(always)]
  pub fn to_frame_block_offset(
    &self, tile_bo: TileBlockOffset,
  ) -> PlaneBlockOffset {
    let bx = self.sbo.0.x << (self.sb_size_log2 - MI_SIZE_LOG2);
    let by = self.sbo.0.y << (self.sb_size_log2 - MI_SIZE_LOG2);
    PlaneBlockOffset(BlockOffset { x: bx + tile_bo.0.x, y: by + tile_bo.0.y })
  }

  #[inline(always)]
  pub fn to_frame_super_block_offset(
    &self, tile_sbo: TileSuperBlockOffset,
  ) -> PlaneSuperBlockOffset {
    PlaneSuperBlockOffset(SuperBlockOffset {
      x: self.sbo.0.x + tile_sbo.0.x,
      y: self.sbo.0.y + tile_sbo.0.y,
    })
  }

  /// Returns above block information for context during prediction.
  /// If there is no above block, returns `None`.
  /// `xdec` and `ydec` are the decimation factors of the targeted plane.
  pub fn above_block_info(
    &self, bo: TileBlockOffset, xdec: usize, ydec: usize,
  ) -> Option<CodedBlockInfo> {
    let (mut bo_x, mut bo_y) = (bo.0.x, bo.0.y);
    if bo_x & 1 == 0 {
      bo_x += xdec
    };
    if bo_y & 1 == 1 {
      bo_y -= ydec
    };
    if bo_y == 0 {
      None
    } else {
      Some(self.coded_block_info[bo_y - 1][bo_x])
    }
  }

  /// Returns left block information for context during prediction.
  /// If there is no left block, returns `None`.
  /// `xdec` and `ydec` are the decimation factors of the targeted plane.
  pub fn left_block_info(
    &self, bo: TileBlockOffset, xdec: usize, ydec: usize,
  ) -> Option<CodedBlockInfo> {
    let (mut bo_x, mut bo_y) = (bo.0.x, bo.0.y);
    if bo_x & 1 == 1 {
      bo_x -= xdec
    };
    if bo_y & 1 == 0 {
      bo_y += ydec
    };
    if bo_x == 0 {
      None
    } else {
      Some(self.coded_block_info[bo_y][bo_x - 1])
    }
  }
}
