// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::api::InterConfig;
use crate::context::{
  BlockOffset, PlaneBlockOffset, SuperBlockOffset, TileBlockOffset,
  TileSuperBlockOffset, MAX_SB_SIZE_LOG2, MIB_SIZE_LOG2, MI_SIZE,
  MI_SIZE_LOG2, SB_SIZE,
};
use crate::dist::*;
use crate::frame::*;
use crate::mc::MotionVector;
use crate::partition::*;
use crate::predict::PredictionMode;
use crate::tiling::*;
use crate::util::ILog;
use crate::util::{clamp, Pixel};
use crate::FrameInvariants;

use arrayvec::*;
use std::ops::{Index, IndexMut};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Copy, Clone, Default)]
pub struct MEStats {
  pub mv: MotionVector,
  /// sad value, on the scale of a 128x128 block
  pub normalized_sad: u32,
}

#[derive(Debug, Clone)]
pub struct FrameMEStats {
  stats: Box<[MEStats]>,
  pub cols: usize,
  pub rows: usize,
}

/// cbindgen:ignore
pub type RefMEStats = Arc<RwLock<[FrameMEStats; REF_FRAMES]>>;
/// cbindgen:ignore
pub type ReadGuardMEStats<'a> =
  RwLockReadGuard<'a, [FrameMEStats; REF_FRAMES]>;
/// cbindgen:ignore
pub type WriteGuardMEStats<'a> =
  RwLockWriteGuard<'a, [FrameMEStats; REF_FRAMES]>;

impl FrameMEStats {
  #[inline]
  pub fn rows_iter(&self) -> std::slice::ChunksExact<'_, MEStats> {
    self.stats.chunks_exact(self.cols)
  }

  pub fn new(cols: usize, rows: usize) -> Self {
    Self {
      // dynamic allocation: once per frame
      stats: vec![MEStats::default(); cols * rows].into_boxed_slice(),
      cols,
      rows,
    }
  }
  pub fn new_arc_array(cols: usize, rows: usize) -> RefMEStats {
    Arc::new(RwLock::new([
      FrameMEStats::new(cols, rows),
      FrameMEStats::new(cols, rows),
      FrameMEStats::new(cols, rows),
      FrameMEStats::new(cols, rows),
      FrameMEStats::new(cols, rows),
      FrameMEStats::new(cols, rows),
      FrameMEStats::new(cols, rows),
      FrameMEStats::new(cols, rows),
    ]))
  }
}

impl Index<usize> for FrameMEStats {
  type Output = [MEStats];
  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    &self.stats[index * self.cols..(index + 1) * self.cols]
  }
}

impl IndexMut<usize> for FrameMEStats {
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.stats[index * self.cols..(index + 1) * self.cols]
  }
}

/// Result of motion search.
#[derive(Debug, Copy, Clone)]
pub struct MotionSearchResult {
  /// Motion vector chosen by the motion search.
  pub mv: MotionVector,
  /// Rate distortion data associated with `mv`.
  pub rd: MVCandidateRD,
}

impl MotionSearchResult {
  /// Creates an 'empty' value.
  ///
  /// To be considered empty, cost is set higher than any naturally occurring
  /// cost value. The idea is that comparing to any valid rd output, the search
  /// result will always be replaced.
  #[inline(always)]
  pub fn empty() -> MotionSearchResult {
    MotionSearchResult {
      mv: MotionVector::default(),
      rd: MVCandidateRD::empty(),
    }
  }

  /// Check if the value should be considered to be empty.
  #[inline(always)]
  const fn is_empty(&self) -> bool {
    self.rd.cost == u64::MAX
  }
}

/// Holds data from computing rate distortion of a motion vector.
#[derive(Debug, Copy, Clone)]
pub struct MVCandidateRD {
  /// Rate distortion cost of the motion vector.
  pub cost: u64,
  /// Distortion metric value for the motion vector.
  pub sad: u32,
}

impl MVCandidateRD {
  /// Creates an 'empty' value.
  ///
  /// To be considered empty, cost is set higher than any naturally occurring
  /// cost value. The idea is that comparing to any valid rd output, the search
  /// result will always be replaced.
  #[inline(always)]
  const fn empty() -> MVCandidateRD {
    MVCandidateRD { sad: u32::MAX, cost: u64::MAX }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MVSamplingMode {
  INIT,
  CORNER { right: bool, bottom: bool },
}

pub fn estimate_tile_motion<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>,
  inter_cfg: &InterConfig,
) {
  let init_size = MIB_SIZE_LOG2;

  let mut prev_ssdec: Option<u8> = None;
  for mv_size_in_b_log2 in (2..=init_size).rev() {
    let init = mv_size_in_b_log2 == init_size;

    // Choose subsampling. Pass one is quarter res and pass two is at half res.
    let ssdec = match init_size - mv_size_in_b_log2 {
      0 => 2,
      1 => 1,
      _ => 0,
    };

    let new_subsampling =
      if let Some(prev) = prev_ssdec { prev != ssdec } else { false };
    prev_ssdec = Some(ssdec);

    // 0.5 and 0.125 are a fudge factors
    let lambda = (fi.me_lambda * 256.0 / (1 << (2 * ssdec)) as f64
      * if ssdec == 0 { 0.5 } else { 0.125 }) as u32;

    for sby in 0..ts.sb_height {
      for sbx in 0..ts.sb_width {
        let mut tested_frames_flags = 0;
        for &ref_frame in inter_cfg.allowed_ref_frames() {
          let frame_flag = 1 << fi.ref_frames[ref_frame.to_index()];
          if tested_frames_flags & frame_flag == frame_flag {
            continue;
          }
          tested_frames_flags |= frame_flag;

          let tile_bo =
            TileSuperBlockOffset(SuperBlockOffset { x: sbx, y: sby })
              .block_offset(0, 0);

          if new_subsampling {
            refine_subsampled_sb_motion(
              fi,
              ts,
              ref_frame,
              mv_size_in_b_log2 + 1,
              tile_bo,
              ssdec,
              lambda,
            );
          }

          estimate_sb_motion(
            fi,
            ts,
            ref_frame,
            mv_size_in_b_log2,
            tile_bo,
            init,
            ssdec,
            lambda,
          );
        }
      }
    }
  }
}

fn estimate_sb_motion<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>, ref_frame: RefType,
  mv_size_in_b_log2: usize, tile_bo: TileBlockOffset, init: bool, ssdec: u8,
  lambda: u32,
) {
  let pix_offset = tile_bo.to_luma_plane_offset();
  let sb_h: usize = SB_SIZE.min(ts.height - pix_offset.y as usize);
  let sb_w: usize = SB_SIZE.min(ts.width - pix_offset.x as usize);

  let mv_size = MI_SIZE << mv_size_in_b_log2;

  // Process in blocks, cropping at edges.
  for y in (0..sb_h).step_by(mv_size) {
    for x in (0..sb_w).step_by(mv_size) {
      let corner: MVSamplingMode = if init {
        MVSamplingMode::INIT
      } else {
        // Processing the block a size up produces data that can be used by
        // the right and bottom corners.
        MVSamplingMode::CORNER {
          right: x & mv_size == mv_size,
          bottom: y & mv_size == mv_size,
        }
      };

      let sub_bo = tile_bo
        .with_offset(x as isize >> MI_SIZE_LOG2, y as isize >> MI_SIZE_LOG2);

      // Clamp to frame edge, rounding up in the case of subsampling.
      // The rounding makes some assumptions about how subsampling is done.
      let w = mv_size.min(sb_w - x + (1 << ssdec) - 1) >> ssdec;
      let h = mv_size.min(sb_h - y + (1 << ssdec) - 1) >> ssdec;

      // Run motion estimation.
      // Note that the initial search (init) instructs the called function to
      // perform a more extensive search.
      if let Some(results) = estimate_motion(
        fi,
        ts,
        w,
        h,
        sub_bo,
        ref_frame,
        None,
        corner,
        init,
        ssdec,
        Some(lambda),
      ) {
        // normalize sad to 128x128 block
        let sad = (((results.rd.sad as u64) << (MAX_SB_SIZE_LOG2 * 2))
          / (w * h) as u64) as u32;
        save_me_stats(
          ts,
          mv_size_in_b_log2,
          sub_bo,
          ref_frame,
          MEStats { mv: results.mv, normalized_sad: sad },
        );
      }
    }
  }
}

fn refine_subsampled_sb_motion<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &mut TileStateMut<'_, T>, ref_frame: RefType,
  mv_size_in_b_log2: usize, tile_bo: TileBlockOffset, ssdec: u8, lambda: u32,
) {
  let pix_offset = tile_bo.to_luma_plane_offset();
  let sb_h: usize = SB_SIZE.min(ts.height - pix_offset.y as usize);
  let sb_w: usize = SB_SIZE.min(ts.width - pix_offset.x as usize);

  let mv_size = MI_SIZE << mv_size_in_b_log2;

  // Process in blocks, cropping at edges.
  for y in (0..sb_h).step_by(mv_size) {
    for x in (0..sb_w).step_by(mv_size) {
      let sub_bo = tile_bo
        .with_offset(x as isize >> MI_SIZE_LOG2, y as isize >> MI_SIZE_LOG2);

      // Clamp to frame edge, rounding up in the case of subsampling.
      // The rounding makes some assumptions about how subsampling is done.
      let w = mv_size.min(sb_w - x + (1 << ssdec) - 1) >> ssdec;
      let h = mv_size.min(sb_h - y + (1 << ssdec) - 1) >> ssdec;

      // Refine the existing motion estimate
      if let Some(results) = refine_subsampled_motion_estimate(
        fi, ts, w, h, sub_bo, ref_frame, ssdec, lambda,
      ) {
        // normalize sad to 128x128 block
        let sad = (((results.rd.sad as u64) << (MAX_SB_SIZE_LOG2 * 2))
          / (w * h) as u64) as u32;
        save_me_stats(
          ts,
          mv_size_in_b_log2,
          sub_bo,
          ref_frame,
          MEStats { mv: results.mv, normalized_sad: sad },
        );
      }
    }
  }
}

fn save_me_stats<T: Pixel>(
  ts: &mut TileStateMut<'_, T>, mv_size_in_b_log2: usize,
  tile_bo: TileBlockOffset, ref_frame: RefType, stats: MEStats,
) {
  let size_in_b = 1 << mv_size_in_b_log2;
  let tile_me_stats = &mut ts.me_stats[ref_frame.to_index()];
  let tile_bo_x_end = (tile_bo.0.x + size_in_b).min(ts.mi_width);
  let tile_bo_y_end = (tile_bo.0.y + size_in_b).min(ts.mi_height);
  for mi_y in tile_bo.0.y..tile_bo_y_end {
    for a in tile_me_stats[mi_y][tile_bo.0.x..tile_bo_x_end].iter_mut() {
      *a = stats;
    }
  }
}

fn get_mv_range(
  w_in_b: usize, h_in_b: usize, bo: PlaneBlockOffset, blk_w: usize,
  blk_h: usize,
) -> (isize, isize, isize, isize) {
  let border_w = 128 + blk_w as isize * 8;
  let border_h = 128 + blk_h as isize * 8;
  let mvx_min = -(bo.0.x as isize) * (8 * MI_SIZE) as isize - border_w;
  let mvx_max = ((w_in_b - bo.0.x) as isize - (blk_w / MI_SIZE) as isize)
    * (8 * MI_SIZE) as isize
    + border_w;
  let mvy_min = -(bo.0.y as isize) * (8 * MI_SIZE) as isize - border_h;
  let mvy_max = ((h_in_b - bo.0.y) as isize - (blk_h / MI_SIZE) as isize)
    * (8 * MI_SIZE) as isize
    + border_h;

  // <https://aomediacodec.github.io/av1-spec/#assign-mv-semantics>
  use crate::context::{MV_LOW, MV_UPP};
  (
    mvx_min.max(MV_LOW as isize + 1),
    mvx_max.min(MV_UPP as isize - 1),
    mvy_min.max(MV_LOW as isize + 1),
    mvy_max.min(MV_UPP as isize - 1),
  )
}

struct MotionEstimationSubsets {
  min_sad: u32,
  median: Option<MotionVector>,
  subset_b: ArrayVec<MotionVector, 5>,
  subset_c: ArrayVec<MotionVector, 5>,
}

impl MotionEstimationSubsets {
  fn all_mvs(&self) -> ArrayVec<MotionVector, 11> {
    let mut all = ArrayVec::new();
    if let Some(median) = self.median {
      all.push(median);
    }

    all.extend(self.subset_b.iter().copied());
    all.extend(self.subset_c.iter().copied());

    all
  }
}

#[profiling::function]
fn get_subset_predictors(
  tile_bo: TileBlockOffset, tile_me_stats: &TileMEStats<'_>,
  frame_ref_opt: Option<ReadGuardMEStats<'_>>, ref_frame_id: usize,
  pix_w: usize, pix_h: usize, mvx_min: isize, mvx_max: isize, mvy_min: isize,
  mvy_max: isize, corner: MVSamplingMode, ssdec: u8,
) -> MotionEstimationSubsets {
  let mut min_sad: u32 = u32::MAX;
  let mut subset_b = ArrayVec::<MotionVector, 5>::new();
  let mut subset_c = ArrayVec::<MotionVector, 5>::new();

  // rounded up width in blocks
  let w = ((pix_w << ssdec) + MI_SIZE - 1) >> MI_SIZE_LOG2;
  let h = ((pix_h << ssdec) + MI_SIZE - 1) >> MI_SIZE_LOG2;

  // Get predictors from the same frame.

  let clipped_half_w = (w >> 1).min(tile_me_stats.cols() - 1 - tile_bo.0.x);
  let clipped_half_h = (h >> 1).min(tile_me_stats.rows() - 1 - tile_bo.0.y);

  let mut process_cand = |stats: MEStats| -> MotionVector {
    min_sad = min_sad.min(stats.normalized_sad);
    let mv = stats.mv.quantize_to_fullpel();
    MotionVector {
      col: clamp(mv.col as isize, mvx_min, mvx_max) as i16,
      row: clamp(mv.row as isize, mvy_min, mvy_max) as i16,
    }
  };

  // Sample the middle of all block edges bordering this one.
  // Note: If motion vectors haven't been precomputed to a given blocksize, then
  // the right and bottom edges will be duplicates of the center predictor when
  // processing in raster order.

  // left
  if tile_bo.0.x > 0 {
    subset_b.push(process_cand(
      tile_me_stats[tile_bo.0.y + clipped_half_h][tile_bo.0.x - 1],
    ));
  }
  // top
  if tile_bo.0.y > 0 {
    subset_b.push(process_cand(
      tile_me_stats[tile_bo.0.y - 1][tile_bo.0.x + clipped_half_w],
    ));
  }

  // Sampling far right and far bottom edges was tested, but had worse results
  // without an extensive threshold test (with threshold being applied after
  // checking median and the best of each subset).

  // right
  if let MVSamplingMode::CORNER { right: true, bottom: _ } = corner {
    if tile_bo.0.x + w < tile_me_stats.cols() {
      subset_b.push(process_cand(
        tile_me_stats[tile_bo.0.y + clipped_half_h][tile_bo.0.x + w],
      ));
    }
  }
  // bottom
  if let MVSamplingMode::CORNER { right: _, bottom: true } = corner {
    if tile_bo.0.y + h < tile_me_stats.rows() {
      subset_b.push(process_cand(
        tile_me_stats[tile_bo.0.y + h][tile_bo.0.x + clipped_half_w],
      ));
    }
  }

  let median = if corner != MVSamplingMode::INIT {
    // Sample the center of the current block.
    Some(process_cand(
      tile_me_stats[tile_bo.0.y + clipped_half_h]
        [tile_bo.0.x + clipped_half_w],
    ))
  } else if subset_b.len() != 3 {
    None
  } else {
    let mut rows: ArrayVec<i16, 3> = subset_b.iter().map(|&a| a.row).collect();
    let mut cols: ArrayVec<i16, 3> = subset_b.iter().map(|&a| a.col).collect();
    rows.as_mut_slice().sort_unstable();
    cols.as_mut_slice().sort_unstable();
    Some(MotionVector { row: rows[1], col: cols[1] })
  };

  // Zero motion vector, don't use add_cand since it skips zero vectors.
  subset_b.push(MotionVector::default());

  // EPZS subset C predictors.
  // Sample the middle of bordering side of the left, right, top and bottom
  // blocks of the previous frame.
  // Sample the middle of this block in the previous frame.

  if let Some(frame_me_stats) = frame_ref_opt {
    let prev_frame = &frame_me_stats[ref_frame_id];

    let frame_bo = PlaneBlockOffset(BlockOffset {
      x: tile_me_stats.x() + tile_bo.0.x,
      y: tile_me_stats.y() + tile_bo.0.y,
    });
    let clipped_half_w = (w >> 1).min(prev_frame.cols - 1 - frame_bo.0.x);
    let clipped_half_h = (h >> 1).min(prev_frame.rows - 1 - frame_bo.0.y);

    // left
    if frame_bo.0.x > 0 {
      subset_c.push(process_cand(
        prev_frame[frame_bo.0.y + clipped_half_h][frame_bo.0.x - 1],
      ));
    }
    // top
    if frame_bo.0.y > 0 {
      subset_c.push(process_cand(
        prev_frame[frame_bo.0.y - 1][frame_bo.0.x + clipped_half_w],
      ));
    }
    // right
    if frame_bo.0.x + w < prev_frame.cols {
      subset_c.push(process_cand(
        prev_frame[frame_bo.0.y + clipped_half_h][frame_bo.0.x + w],
      ));
    }
    // bottom
    if frame_bo.0.y + h < prev_frame.rows {
      subset_c.push(process_cand(
        prev_frame[frame_bo.0.y + h][frame_bo.0.x + clipped_half_w],
      ));
    }

    subset_c.push(process_cand(
      prev_frame[frame_bo.0.y + clipped_half_h][frame_bo.0.x + clipped_half_w],
    ));
  }

  // Undo normalization to 128x128 block size
  let min_sad = ((min_sad as u64 * (pix_w * pix_h) as u64)
    >> (MAX_SB_SIZE_LOG2 * 2)) as u32;

  let dec_mv = |mv: MotionVector| MotionVector {
    col: mv.col >> ssdec,
    row: mv.row >> ssdec,
  };
  let median = median.map(dec_mv);
  for mv in subset_b.iter_mut() {
    *mv = dec_mv(*mv);
  }
  for mv in subset_c.iter_mut() {
    *mv = dec_mv(*mv);
  }

  MotionEstimationSubsets { min_sad, median, subset_b, subset_c }
}

pub fn estimate_motion<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &TileStateMut<'_, T>, w: usize, h: usize,
  tile_bo: TileBlockOffset, ref_frame: RefType,
  pmv: Option<[MotionVector; 2]>, corner: MVSamplingMode,
  extensive_search: bool, ssdec: u8, lambda: Option<u32>,
) -> Option<MotionSearchResult> {
  if let Some(ref rec) =
    fi.rec_buffer.frames[fi.ref_frames[ref_frame.to_index()] as usize]
  {
    let frame_bo = ts.to_frame_block_offset(tile_bo);
    let (mvx_min, mvx_max, mvy_min, mvy_max) =
      get_mv_range(fi.w_in_b, fi.h_in_b, frame_bo, w << ssdec, h << ssdec);

    let lambda = lambda.unwrap_or({
      // 0.5 is a fudge factor
      (fi.me_lambda * 256.0 * 0.5) as u32
    });

    let global_mv = [MotionVector { row: 0, col: 0 }; 2];

    let po = frame_bo.to_luma_plane_offset();
    let (mvx_min, mvx_max, mvy_min, mvy_max) =
      (mvx_min >> ssdec, mvx_max >> ssdec, mvy_min >> ssdec, mvy_max >> ssdec);
    let po = PlaneOffset { x: po.x >> ssdec, y: po.y >> ssdec };
    let p_ref = match ssdec {
      0 => &rec.frame.planes[0],
      1 => &rec.input_hres,
      2 => &rec.input_qres,
      _ => unimplemented!(),
    };

    let org_region = &match ssdec {
      0 => ts.input_tile.planes[0]
        .subregion(Area::BlockStartingAt { bo: tile_bo.0 }),
      1 => ts.input_hres.region(Area::StartingAt { x: po.x, y: po.y }),
      2 => ts.input_qres.region(Area::StartingAt { x: po.x, y: po.y }),
      _ => unimplemented!(),
    };

    let mut best: MotionSearchResult = full_pixel_me(
      fi,
      ts,
      org_region,
      p_ref,
      tile_bo,
      po,
      lambda,
      pmv.unwrap_or(global_mv),
      w,
      h,
      mvx_min,
      mvx_max,
      mvy_min,
      mvy_max,
      ref_frame,
      corner,
      extensive_search,
      ssdec,
    );

    if let Some(pmv) = pmv {
      let use_satd: bool = fi.config.speed_settings.motion.use_satd_subpel;
      if use_satd {
        best.rd = get_fullpel_mv_rd(
          fi,
          po,
          org_region,
          p_ref,
          fi.sequence.bit_depth,
          pmv,
          lambda,
          use_satd,
          mvx_min,
          mvx_max,
          mvy_min,
          mvy_max,
          w,
          h,
          best.mv,
        );
      }

      sub_pixel_me(
        fi, po, org_region, p_ref, lambda, pmv, mvx_min, mvx_max, mvy_min,
        mvy_max, w, h, use_satd, &mut best, ref_frame,
      );
    }

    // Scale motion vectors to full res size
    best.mv = best.mv << ssdec;

    Some(best)
  } else {
    None
  }
}

/// Refine motion estimation that was computed one level of subsampling up.
fn refine_subsampled_motion_estimate<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &TileStateMut<'_, T>, w: usize, h: usize,
  tile_bo: TileBlockOffset, ref_frame: RefType, ssdec: u8, lambda: u32,
) -> Option<MotionSearchResult> {
  if let Some(ref rec) =
    fi.rec_buffer.frames[fi.ref_frames[ref_frame.to_index()] as usize]
  {
    let frame_bo = ts.to_frame_block_offset(tile_bo);
    let (mvx_min, mvx_max, mvy_min, mvy_max) =
      get_mv_range(fi.w_in_b, fi.h_in_b, frame_bo, w << ssdec, h << ssdec);

    let pmv = [MotionVector { row: 0, col: 0 }; 2];

    let po = frame_bo.to_luma_plane_offset();
    let (mvx_min, mvx_max, mvy_min, mvy_max) =
      (mvx_min >> ssdec, mvx_max >> ssdec, mvy_min >> ssdec, mvy_max >> ssdec);
    let po = PlaneOffset { x: po.x >> ssdec, y: po.y >> ssdec };
    let p_ref = match ssdec {
      0 => &rec.frame.planes[0],
      1 => &rec.input_hres,
      2 => &rec.input_qres,
      _ => unimplemented!(),
    };

    let org_region = &match ssdec {
      0 => ts.input_tile.planes[0]
        .subregion(Area::BlockStartingAt { bo: tile_bo.0 }),
      1 => ts.input_hres.region(Area::StartingAt { x: po.x, y: po.y }),
      2 => ts.input_qres.region(Area::StartingAt { x: po.x, y: po.y }),
      _ => unimplemented!(),
    };

    let mv =
      ts.me_stats[ref_frame.to_index()][tile_bo.0.y][tile_bo.0.x].mv >> ssdec;

    // Given a motion vector at 0 at higher subsampling:
    // |  -1   |   0   |   1   |
    // then the vectors at -1 to 2 should be tested at the current subsampling.
    //      |-------------|
    // | -2 -1 |  0  1 |  2  3 |
    // This corresponds to a 4x4 full search.
    let x_lo = po.x + (mv.col as isize / 8 - 1).max(mvx_min / 8);
    let x_hi = po.x + (mv.col as isize / 8 + 2).min(mvx_max / 8);
    let y_lo = po.y + (mv.row as isize / 8 - 1).max(mvy_min / 8);
    let y_hi = po.y + (mv.row as isize / 8 + 2).min(mvy_max / 8);
    let mut results = full_search(
      fi, x_lo, x_hi, y_lo, y_hi, w, h, org_region, p_ref, po, 1, lambda, pmv,
    );

    // Scale motion vectors to full res size
    results.mv = results.mv << ssdec;

    Some(results)
  } else {
    None
  }
}

#[profiling::function]
fn full_pixel_me<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &TileStateMut<'_, T>,
  org_region: &PlaneRegion<T>, p_ref: &Plane<T>, tile_bo: TileBlockOffset,
  po: PlaneOffset, lambda: u32, pmv: [MotionVector; 2], w: usize, h: usize,
  mvx_min: isize, mvx_max: isize, mvy_min: isize, mvy_max: isize,
  ref_frame: RefType, corner: MVSamplingMode, extensive_search: bool,
  ssdec: u8,
) -> MotionSearchResult {
  let ref_frame_id = ref_frame.to_index();
  let tile_me_stats = &ts.me_stats[ref_frame_id].as_const();
  let frame_ref = fi.rec_buffer.frames[fi.ref_frames[0] as usize]
    .as_ref()
    .map(|frame_ref| frame_ref.frame_me_stats.read().expect("poisoned lock"));
  let subsets = get_subset_predictors(
    tile_bo,
    tile_me_stats,
    frame_ref,
    ref_frame_id,
    w,
    h,
    mvx_min,
    mvx_max,
    mvy_min,
    mvy_max,
    corner,
    ssdec,
  );

  let try_cands = |predictors: &[MotionVector],
                   best: &mut MotionSearchResult| {
    let mut results = get_best_predictor(
      fi,
      po,
      org_region,
      p_ref,
      predictors,
      fi.sequence.bit_depth,
      pmv,
      lambda,
      mvx_min,
      mvx_max,
      mvy_min,
      mvy_max,
      w,
      h,
    );
    fullpel_diamond_search(
      fi,
      po,
      org_region,
      p_ref,
      &mut results,
      fi.sequence.bit_depth,
      pmv,
      lambda,
      mvx_min,
      mvx_max,
      mvy_min,
      mvy_max,
      w,
      h,
    );

    if results.rd.cost < best.rd.cost {
      *best = results;
    }
  };

  let mut best: MotionSearchResult = MotionSearchResult::empty();
  if !extensive_search {
    try_cands(&subsets.all_mvs(), &mut best);
    best
  } else {
    // Perform a more thorough search before resorting to full search.
    // Search the median, the best mvs of neighboring blocks, and motion vectors
    // from the previous frame. Stop once a candidate with a sad less than a
    // threshold is found.

    let thresh = (subsets.min_sad as f32 * 1.2) as u32
      + (((w * h) as u32) << (fi.sequence.bit_depth - 8));

    if let Some(median) = subsets.median {
      try_cands(&[median], &mut best);

      if best.rd.sad < thresh {
        return best;
      }
    }

    try_cands(&subsets.subset_b, &mut best);

    if best.rd.sad < thresh {
      return best;
    }

    try_cands(&subsets.subset_c, &mut best);

    if best.rd.sad < thresh {
      return best;
    }

    // Preform UMH search, either as the last possible search when full search
    // is disabled, or as the last search before resorting to full search.
    uneven_multi_hex_search(
      fi,
      po,
      org_region,
      p_ref,
      &mut best,
      fi.sequence.bit_depth,
      pmv,
      lambda,
      mvx_min,
      mvx_max,
      mvy_min,
      mvy_max,
      w,
      h,
      // Use 24, since it is the largest range that x264 uses.
      24,
    );

    if !fi.config.speed_settings.motion.me_allow_full_search
      || best.rd.sad < thresh
    {
      return best;
    }

    {
      let range_x = (192 * fi.me_range_scale as isize) >> ssdec;
      let range_y = (64 * fi.me_range_scale as isize) >> ssdec;
      let x_lo = po.x + (-range_x).max(mvx_min / 8);
      let x_hi = po.x + (range_x).min(mvx_max / 8);
      let y_lo = po.y + (-range_y).max(mvy_min / 8);
      let y_hi = po.y + (range_y).min(mvy_max / 8);

      let results = full_search(
        fi,
        x_lo,
        x_hi,
        y_lo,
        y_hi,
        w,
        h,
        org_region,
        p_ref,
        po,
        // Full search is run at quarter resolution, except for short edges.
        // When subsampling is lower than usual, the step size is raised so that
        // the number of search locations stays the same.
        4 >> ssdec,
        lambda,
        [MotionVector::default(); 2],
      );

      if results.rd.cost < best.rd.cost {
        results
      } else {
        best
      }
    }
  }
}

fn sub_pixel_me<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  p_ref: &Plane<T>, lambda: u32, pmv: [MotionVector; 2], mvx_min: isize,
  mvx_max: isize, mvy_min: isize, mvy_max: isize, w: usize, h: usize,
  use_satd: bool, best: &mut MotionSearchResult, ref_frame: RefType,
) {
  subpel_diamond_search(
    fi,
    po,
    org_region,
    p_ref,
    fi.sequence.bit_depth,
    pmv,
    lambda,
    mvx_min,
    mvx_max,
    mvy_min,
    mvy_max,
    w,
    h,
    use_satd,
    best,
    ref_frame,
  );
}

#[profiling::function]
fn get_best_predictor<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  p_ref: &Plane<T>, predictors: &[MotionVector], bit_depth: usize,
  pmv: [MotionVector; 2], lambda: u32, mvx_min: isize, mvx_max: isize,
  mvy_min: isize, mvy_max: isize, w: usize, h: usize,
) -> MotionSearchResult {
  let mut best: MotionSearchResult = MotionSearchResult::empty();

  for &init_mv in predictors.iter() {
    let rd = get_fullpel_mv_rd(
      fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
      mvx_max, mvy_min, mvy_max, w, h, init_mv,
    );

    if rd.cost < best.rd.cost {
      best.mv = init_mv;
      best.rd = rd;
    }
  }

  best
}

/// Declares an array of motion vectors in structure of arrays syntax.
/// Compared to [`search_pattern_subpel`], this version creates motion vectors
/// in fullpel resolution (x8).
macro_rules! search_pattern {
    ($field_a:ident: [$($ll_a:expr),*], $field_b:ident: [$($ll_b:expr),*]) => {
      [ $(MotionVector { $field_a: $ll_a << 3, $field_b: $ll_b << 3 } ),*]
    };
}

/// Declares an array of motion vectors in structure of arrays syntax.
macro_rules! search_pattern_subpel {
    ($field_a:ident: [$($ll_a:expr),*], $field_b:ident: [$($ll_b:expr),*]) => {
      [ $(MotionVector { $field_a: $ll_a, $field_b: $ll_b } ),*]
    };
}

/// Diamond pattern of radius 1 as shown below. For fullpel search, use
/// `DIAMOND_R1_PATTERN_FULLPEL` since it has been scaled for fullpel search.
/// ```text
///  X
/// XoX
///  X
/// ```
/// 'X's are motion candidates and the 'o' is the center.
///
const DIAMOND_R1_PATTERN_SUBPEL: [MotionVector; 4] = search_pattern_subpel!(
  col: [  0,  1,  0, -1],
  row: [  1,  0, -1,  0]
);
/// Diamond pattern of radius 1 as shown below. Unlike `DIAMOND_R1_PATTERN`, the
/// vectors have been shifted fullpel scale.
/// ```text
///  X
/// XoX
///  X
/// ```
/// 'X's are motion candidates and the 'o' is the center.
const DIAMOND_R1_PATTERN: [MotionVector; 4] = search_pattern!(
  col: [  0,  1,  0, -1],
  row: [  1,  0, -1,  0]
);

/// Run a full pixel diamond search. The search is run on multiple step sizes.
///
/// For each step size, candidate motion vectors are examined for improvement
/// to the current search location. The search location is moved to the best
/// candidate (if any). This is repeated until the search location stops moving.
#[profiling::function]
fn fullpel_diamond_search<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  p_ref: &Plane<T>, current: &mut MotionSearchResult, bit_depth: usize,
  pmv: [MotionVector; 2], lambda: u32, mvx_min: isize, mvx_max: isize,
  mvy_min: isize, mvy_max: isize, w: usize, h: usize,
) {
  // Define the initial and the final scale (log2) of the diamond.
  let (mut diamond_radius_log2, diamond_radius_end_log2) = (1u8, 0u8);

  loop {
    // Find the best candidate from the diamond pattern.
    let mut best_cand: MotionSearchResult = MotionSearchResult::empty();
    for &offset in &DIAMOND_R1_PATTERN {
      let cand_mv = current.mv + (offset << diamond_radius_log2);
      let rd = get_fullpel_mv_rd(
        fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
        mvx_max, mvy_min, mvy_max, w, h, cand_mv,
      );

      if rd.cost < best_cand.rd.cost {
        best_cand.mv = cand_mv;
        best_cand.rd = rd;
      }
    }

    // Continue the search at this scale until the can't find a better candidate
    // to use.
    if current.rd.cost <= best_cand.rd.cost {
      if diamond_radius_log2 == diamond_radius_end_log2 {
        break;
      } else {
        diamond_radius_log2 -= 1;
      }
    } else {
      *current = best_cand;
    }
  }

  assert!(!current.is_empty());
}

/// A hexagon pattern around a center point. The pattern is ordered so that the
/// offsets circle around the center. This is done to allow pruning locations
/// covered by the last iteration.
/// ```text
///   21012
/// 2  X X
/// 1
/// 0 X o X
/// 1
/// 2  X X
/// ```
/// 'X's are motion candidates and the 'o' is the center.
///
/// The illustration below shows the process of a hexagon search.
/// ```text
/// Step 1    Step 2
///  1 1       1 1 2
///
/// 1(0)1  => 1 0(1)2
///
///  1 1       1 1 2
/// ```
/// The search above has gone through the following steps.
/// 1. Search '1' elements for better candidates than the center '0'.
/// 2. Recenter around the best candidate ('(1)') and hexagon candidates that
///    don't overlap with the previous search step (labeled '2').
const HEXAGON_PATTERN: [MotionVector; 6] = search_pattern!(
  col: [  0,  2,  2,  0, -2, -2],
  row: [ -2, -1,  1,  2,  1, -1]
);

/// A small square pattern around a center point.
/// ```text
///   101
/// 1 XXX
/// 0 XoX
/// 1 XXX
/// ```
/// 'X's are motion candidates and the 'o' is the center.
const SQUARE_REFINE_PATTERN: [MotionVector; 8] = search_pattern!(
  col: [ -1,  0,  1, -1,  1, -1,  0,  1],
  row: [  1,  1,  1,  0,  0, -1, -1, -1]
);

/// Perform hexagon search and refine afterwards.
///
/// In the hexagon search stage, candidate motion vectors are examined for
/// improvement to the current search location. The search location is moved to
/// the best candidate (if any). This is repeated until the search location
/// stops moving.
///
/// Refinement uses a square pattern that fits between the hexagon candidates.
///
/// The hexagon pattern is defined by [`HEXAGON_PATTERN`] and the refinement
/// is defined by [`SQUARE_REFINE_PATTERN`].
///
/// `current` provides the initial search location and serves as
/// the output for the final search results.
#[profiling::function]
fn hexagon_search<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  p_ref: &Plane<T>, current: &mut MotionSearchResult, bit_depth: usize,
  pmv: [MotionVector; 2], lambda: u32, mvx_min: isize, mvx_max: isize,
  mvy_min: isize, mvy_max: isize, w: usize, h: usize,
) {
  // The first iteration of hexagon search is implemented separate from
  // subsequent iterations, which overlap with previous iterations.

  // Holds what candidate is used (if any). This is used to determine which
  // candidates have already been tested in a previous iteration and can be
  // skipped.
  let mut best_cand_idx: usize = 0;
  let mut best_cand: MotionSearchResult = MotionSearchResult::empty();

  // First iteration of hexagon search. There are six candidates to consider.
  for i in 0..6 {
    let cand_mv = current.mv + HEXAGON_PATTERN[i];
    let rd = get_fullpel_mv_rd(
      fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
      mvx_max, mvy_min, mvy_max, w, h, cand_mv,
    );

    if rd.cost < best_cand.rd.cost {
      best_cand_idx = i;
      best_cand.mv = cand_mv;
      best_cand.rd = rd;
    }
  }

  // Run additional iterations of hexagon search until the search location
  // doesn't update.
  while best_cand.rd.cost < current.rd.cost {
    // Update the search location.
    *current = best_cand;
    best_cand = MotionSearchResult::empty();

    // Save the index/direction taken in the previous iteration to the current
    // search location.
    let center_cand_idx = best_cand_idx;

    // Look only at candidates that don't overlap with previous iterations. This
    // corresponds with the three offsets (2D) with the closest direction to
    // that traveled by the previous iteration. HEXAGON_PATTERN has clockwise
    // order, so the last direction -1, +0, and +1 (mod 6) give the indices for
    // these offsets.
    for idx_offset_mod6 in 5..=7 {
      let i = (center_cand_idx + idx_offset_mod6) % 6;
      let cand_mv = current.mv + HEXAGON_PATTERN[i];

      let rd = get_fullpel_mv_rd(
        fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
        mvx_max, mvy_min, mvy_max, w, h, cand_mv,
      );

      if rd.cost < best_cand.rd.cost {
        best_cand_idx = i;
        best_cand.mv = cand_mv;
        best_cand.rd = rd;
      }
    }
  }

  // Refine the motion after completing hexagon search.
  let mut best_cand: MotionSearchResult = MotionSearchResult::empty();
  for &offset in &SQUARE_REFINE_PATTERN {
    let cand_mv = current.mv + offset;
    let rd = get_fullpel_mv_rd(
      fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
      mvx_max, mvy_min, mvy_max, w, h, cand_mv,
    );

    if rd.cost < best_cand.rd.cost {
      best_cand.mv = cand_mv;
      best_cand.rd = rd;
    }
  }
  if best_cand.rd.cost < current.rd.cost {
    *current = best_cand;
  }

  assert!(!current.is_empty());
}

/// Uneven multi-hexagon search pattern around a center point. Used for locating
/// irregular movement.
/// ```text
///      X
///    X   X
///  X       X
///  X       X
///  X   o   X
///  X       X
///  X       X
///    X   X
///      X
/// ```
/// 'X's are motion candidates and the 'o' is the center.
const UMH_PATTERN: [MotionVector; 16] = search_pattern!(
  col: [ -2, -1,  0,  1,  2,  3,  4,  3,  2,  1,  0, -1, -2,  3, -4, -3],
  row: [  4,  4,  4,  4,  4,  2,  0, -2, -4, -4, -4, -4, -4, -2,  0,  2]
);

/// Perform an uneven multi-hexagon search. There are 4 stages:
/// 1. Unsymmetrical-cross search: Search the horizontal and vertical directions
///   for the general direction of the motion.
/// 2. A 5x5 full search is done to refine the current candidate.
/// 3. Uneven multi-hexagon search. See [`UMH_PATTERN`].
/// 4. Refinement using standard hexagon search.
///
/// `current` provides the initial search location and serves as
/// the output for the final search results.
///
/// `me_range` parameter determines how far these stages can search.
#[profiling::function]
fn uneven_multi_hex_search<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  p_ref: &Plane<T>, current: &mut MotionSearchResult, bit_depth: usize,
  pmv: [MotionVector; 2], lambda: u32, mvx_min: isize, mvx_max: isize,
  mvy_min: isize, mvy_max: isize, w: usize, h: usize, me_range: i16,
) {
  assert!(!current.is_empty());

  // Search in a cross pattern to obtain a rough approximate of motion.
  // The cross is split into a horizontal and vertical component. Video content
  // tends to have more horizontal motion, so the horizontal part of the cross
  // is twice as large as the vertical half.
  //        X        -
  //                 | <- me_range/2
  //        X        |
  // X X X XoX X X X -
  //        X
  //
  //        X
  // |------|
  //     \
  //    me_range
  let center = current.mv;

  // The larger, horizontal, part of the cross search.
  for i in (1..=me_range).step_by(2) {
    const HORIZONTAL_LINE: [MotionVector; 2] = search_pattern!(
      col: [ 0, 0],
      row: [-1, 1]
    );

    for &offset in &HORIZONTAL_LINE {
      let cand_mv = center + offset * i;
      let rd = get_fullpel_mv_rd(
        fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
        mvx_max, mvy_min, mvy_max, w, h, cand_mv,
      );

      if rd.cost < current.rd.cost {
        current.mv = cand_mv;
        current.rd = rd;
      }
    }
  }

  // The smaller, vertical, part of the cross search
  for i in (1..=me_range >> 1).step_by(2) {
    const VERTICAL_LINE: [MotionVector; 2] = search_pattern!(
      col: [-1, 1],
      row: [ 0, 0]
    );

    for &offset in &VERTICAL_LINE {
      let cand_mv = center + offset * i;
      let rd = get_fullpel_mv_rd(
        fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
        mvx_max, mvy_min, mvy_max, w, h, cand_mv,
      );

      if rd.cost < current.rd.cost {
        current.mv = cand_mv;
        current.rd = rd;
      }
    }
  }

  // 5x5 full search. Search a 5x5 square region around the current best mv.
  let center = current.mv;
  for row in -2..=2 {
    for col in -2..=2 {
      if row == 0 && col == 0 {
        continue;
      }
      let cand_mv = center + MotionVector { row, col };
      let rd = get_fullpel_mv_rd(
        fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
        mvx_max, mvy_min, mvy_max, w, h, cand_mv,
      );

      if rd.cost < current.rd.cost {
        current.mv = cand_mv;
        current.rd = rd;
      }
    }
  }

  // Run the hexagons in uneven multi-hexagon. The hexagonal pattern is tested
  // around the best vector at multiple scales.
  // Example of the UMH pattern run on a scale of 1 and 2:
  //         2         -
  //                   | <- me_range
  //     2       2     |
  //                   |
  // 2       1       2 |
  //       1   1       |
  // 2   1       1   2 |
  //     1       1     |
  // 2   1   o   1   2 |
  //     1       1     |
  // 2   1       1   2 |
  //       1   1       |
  // 2       1       2 |
  //                   |
  //     2       2     |
  //                   |
  //         2         -
  // |---------------|
  //        \
  //       me_range
  let center = current.mv;

  // Divide by 4, the radius of the UMH's hexagon.
  let iterations = me_range >> 2;
  for i in 1..=iterations {
    for &offset in &UMH_PATTERN {
      let cand_mv = center + offset * i;
      let rd = get_fullpel_mv_rd(
        fi, po, org_region, p_ref, bit_depth, pmv, lambda, false, mvx_min,
        mvx_max, mvy_min, mvy_max, w, h, cand_mv,
      );

      if rd.cost < current.rd.cost {
        current.mv = cand_mv;
        current.rd = rd;
      }
    }
  }

  // Refine the search results using a 'normal' hexagon search.
  hexagon_search(
    fi, po, org_region, p_ref, current, bit_depth, pmv, lambda, mvx_min,
    mvx_max, mvy_min, mvy_max, w, h,
  );
}

/// Run a subpixel diamond search. The search is run on multiple step sizes.
///
/// For each step size, candidate motion vectors are examined for improvement
/// to the current search location. The search location is moved to the best
/// candidate (if any). This is repeated until the search location stops moving.
#[profiling::function]
fn subpel_diamond_search<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  _p_ref: &Plane<T>, bit_depth: usize, pmv: [MotionVector; 2], lambda: u32,
  mvx_min: isize, mvx_max: isize, mvy_min: isize, mvy_max: isize, w: usize,
  h: usize, use_satd: bool, current: &mut MotionSearchResult,
  ref_frame: RefType,
) {
  use crate::util::Aligned;

  // Motion compensation assembly has special requirements for edges
  let mc_w = w.next_power_of_two();
  let mc_h = (h + 1) & !1;

  // Metadata for subpel scratch pad.
  let cfg = PlaneConfig::new(mc_w, mc_h, 0, 0, 0, 0, std::mem::size_of::<T>());
  // Stack allocation for subpel scratch pad.
  // SAFETY: We write to the array below before reading from it.
  let mut buf: Aligned<[T; 128 * 128]> = unsafe { Aligned::uninitialized() };
  let mut tmp_region = PlaneRegionMut::from_slice(
    &mut buf.data,
    &cfg,
    Rect { x: 0, y: 0, width: cfg.width, height: cfg.height },
  );

  // start at 1/2 pel and end at 1/4 or 1/8 pel
  let (mut diamond_radius_log2, diamond_radius_end_log2) =
    (2u8, u8::from(!fi.allow_high_precision_mv));

  loop {
    // Find the best candidate from the diamond pattern.
    let mut best_cand: MotionSearchResult = MotionSearchResult::empty();
    for &offset in &DIAMOND_R1_PATTERN_SUBPEL {
      let cand_mv = current.mv + (offset << diamond_radius_log2);

      let rd = get_subpel_mv_rd(
        fi,
        po,
        org_region,
        bit_depth,
        pmv,
        lambda,
        use_satd,
        mvx_min,
        mvx_max,
        mvy_min,
        mvy_max,
        w,
        h,
        cand_mv,
        &mut tmp_region,
        ref_frame,
      );

      if rd.cost < best_cand.rd.cost {
        best_cand.mv = cand_mv;
        best_cand.rd = rd;
      }
    }

    // Continue the search at this scale until a better candidate isn't found.
    if current.rd.cost <= best_cand.rd.cost {
      if diamond_radius_log2 == diamond_radius_end_log2 {
        break;
      } else {
        diamond_radius_log2 -= 1;
      }
    } else {
      *current = best_cand;
    }
  }

  assert!(!current.is_empty());
}

#[inline]
fn get_fullpel_mv_rd<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  p_ref: &Plane<T>, bit_depth: usize, pmv: [MotionVector; 2], lambda: u32,
  use_satd: bool, mvx_min: isize, mvx_max: isize, mvy_min: isize,
  mvy_max: isize, w: usize, h: usize, cand_mv: MotionVector,
) -> MVCandidateRD {
  if (cand_mv.col as isize) < mvx_min
    || (cand_mv.col as isize) > mvx_max
    || (cand_mv.row as isize) < mvy_min
    || (cand_mv.row as isize) > mvy_max
  {
    return MVCandidateRD::empty();
  }

  // Convert the motion vector into an full pixel offset.
  let plane_ref = p_ref.region(Area::StartingAt {
    x: po.x + (cand_mv.col / 8) as isize,
    y: po.y + (cand_mv.row / 8) as isize,
  });
  compute_mv_rd(
    fi, pmv, lambda, use_satd, bit_depth, w, h, cand_mv, org_region,
    &plane_ref,
  )
}

fn get_subpel_mv_rd<T: Pixel>(
  fi: &FrameInvariants<T>, po: PlaneOffset, org_region: &PlaneRegion<T>,
  bit_depth: usize, pmv: [MotionVector; 2], lambda: u32, use_satd: bool,
  mvx_min: isize, mvx_max: isize, mvy_min: isize, mvy_max: isize, w: usize,
  h: usize, cand_mv: MotionVector, tmp_region: &mut PlaneRegionMut<T>,
  ref_frame: RefType,
) -> MVCandidateRD {
  if (cand_mv.col as isize) < mvx_min
    || (cand_mv.col as isize) > mvx_max
    || (cand_mv.row as isize) < mvy_min
    || (cand_mv.row as isize) > mvy_max
  {
    return MVCandidateRD::empty();
  }

  let tmp_width = tmp_region.rect().width;
  let tmp_height = tmp_region.rect().height;
  let tile_rect =
    TileRect { x: 0, y: 0, width: tmp_width, height: tmp_height };

  PredictionMode::NEWMV.predict_inter_single(
    fi, tile_rect, 0, po, tmp_region,
    // motion comp's w & h on edges can be different than distortion's
    tmp_width, tmp_height, ref_frame, cand_mv,
  );
  let plane_ref = tmp_region.as_const();
  compute_mv_rd(
    fi, pmv, lambda, use_satd, bit_depth, w, h, cand_mv, org_region,
    &plane_ref,
  )
}

/// Compute the rate distortion stats for a motion vector.
#[inline(always)]
fn compute_mv_rd<T: Pixel>(
  fi: &FrameInvariants<T>, pmv: [MotionVector; 2], lambda: u32,
  use_satd: bool, bit_depth: usize, w: usize, h: usize, cand_mv: MotionVector,
  plane_org: &PlaneRegion<'_, T>, plane_ref: &PlaneRegion<'_, T>,
) -> MVCandidateRD {
  let sad = if use_satd {
    get_satd(plane_org, plane_ref, w, h, bit_depth, fi.cpu_feature_level)
  } else {
    get_sad(plane_org, plane_ref, w, h, bit_depth, fi.cpu_feature_level)
  };

  let rate1 = get_mv_rate(cand_mv, pmv[0], fi.allow_high_precision_mv);
  let rate2 = get_mv_rate(cand_mv, pmv[1], fi.allow_high_precision_mv);
  let rate = rate1.min(rate2 + 1);

  MVCandidateRD { cost: 256 * sad as u64 + rate as u64 * lambda as u64, sad }
}

#[profiling::function]
fn full_search<T: Pixel>(
  fi: &FrameInvariants<T>, x_lo: isize, x_hi: isize, y_lo: isize, y_hi: isize,
  w: usize, h: usize, org_region: &PlaneRegion<T>, p_ref: &Plane<T>,
  po: PlaneOffset, step: usize, lambda: u32, pmv: [MotionVector; 2],
) -> MotionSearchResult {
  let search_region = p_ref.region(Area::Rect {
    x: x_lo,
    y: y_lo,
    width: (x_hi - x_lo) as usize + w,
    height: (y_hi - y_lo) as usize + h,
  });

  let mut best: MotionSearchResult = MotionSearchResult::empty();

  // Select rectangular regions within search region with vert+horz windows
  for vert_window in search_region.vert_windows(h).step_by(step) {
    for ref_window in vert_window.horz_windows(w).step_by(step) {
      let &Rect { x, y, .. } = ref_window.rect();

      let mv = MotionVector {
        row: 8 * (y as i16 - po.y as i16),
        col: 8 * (x as i16 - po.x as i16),
      };

      let rd = compute_mv_rd(
        fi,
        pmv,
        lambda,
        false,
        fi.sequence.bit_depth,
        w,
        h,
        mv,
        org_region,
        &ref_window,
      );

      if rd.cost < best.rd.cost {
        best.rd = rd;
        best.mv = mv;
      }
    }
  }

  best
}

#[inline(always)]
fn get_mv_rate(
  a: MotionVector, b: MotionVector, allow_high_precision_mv: bool,
) -> u32 {
  #[inline(always)]
  fn diff_to_rate(diff: i16, allow_high_precision_mv: bool) -> u32 {
    let d = if allow_high_precision_mv { diff } else { diff >> 1 };
    2 * ILog::ilog(d.abs()) as u32
  }

  diff_to_rate(a.row - b.row, allow_high_precision_mv)
    + diff_to_rate(a.col - b.col, allow_high_precision_mv)
}
