// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::api::FrameType;
use crate::color::ChromaSampling::Cs400;
use crate::context::*;
use crate::encoder::FrameInvariants;
use crate::partition::RefType::*;
use crate::predict::PredictionMode::*;
use crate::quantize::*;
use crate::tiling::*;
use crate::util::{clamp, ILog, Pixel};
use crate::DeblockState;
use rayon::iter::*;
use std::cmp;

fn deblock_adjusted_level(
  deblock: &DeblockState, block: &Block, pli: usize, vertical: bool,
) -> usize {
  let idx = if pli == 0 { usize::from(!vertical) } else { pli + 1 };

  let level = if deblock.block_deltas_enabled {
    // By-block filter strength delta, if the feature is active.
    let block_delta = if deblock.block_delta_multi {
      block.deblock_deltas[idx] << deblock.block_delta_shift
    } else {
      block.deblock_deltas[0] << deblock.block_delta_shift
    };

    // Add to frame-specified filter strength (Y-vertical, Y-horizontal, U, V)
    clamp(block_delta + deblock.levels[idx] as i8, 0, MAX_LOOP_FILTER as i8)
      as u8
  } else {
    deblock.levels[idx]
  };

  // if fi.seg_feaure_active {
  // rav1e does not yet support segments or segment features
  // }

  // Are delta modifiers for specific references and modes active?  If so, add them too.
  if deblock.deltas_enabled {
    let mode = block.mode;
    let reference = block.ref_frames[0];
    let mode_type = usize::from(
      mode >= NEARESTMV && mode != GLOBALMV && mode != GLOBAL_GLOBALMV,
    );
    let l5 = level >> 5;
    clamp(
      level as i32
        + ((deblock.ref_deltas[reference.to_index()] as i32) << l5)
        + if reference == INTRA_FRAME {
          0
        } else {
          (deblock.mode_deltas[mode_type] as i32) << l5
        },
      0,
      MAX_LOOP_FILTER as i32,
    ) as usize
  } else {
    level as usize
  }
}

#[inline]
fn deblock_left<'a, T: Pixel>(
  blocks: &'a TileBlocks, in_bo: TileBlockOffset, p: &PlaneRegion<T>,
) -> &'a Block {
  let xdec = p.plane_cfg.xdec;
  let ydec = p.plane_cfg.ydec;

  // subsampled chroma uses odd mi row/col
  // We already know we're not at the upper/left corner, so prev_block is in frame
  &blocks[in_bo.0.y | ydec][(in_bo.0.x | xdec) - (1 << xdec)]
}

#[inline]
fn deblock_up<'a, T: Pixel>(
  blocks: &'a TileBlocks, in_bo: TileBlockOffset, p: &PlaneRegion<T>,
) -> &'a Block {
  let xdec = p.plane_cfg.xdec;
  let ydec = p.plane_cfg.ydec;

  // subsampled chroma uses odd mi row/col
  &blocks[(in_bo.0.y | ydec) - (1 << ydec)][in_bo.0.x | xdec]
}

// Must be called on a tx edge, and not on a frame edge.  This is enforced above the call.
fn deblock_size<T: Pixel>(
  block: &Block, prev_block: &Block, p: &PlaneRegion<T>, pli: usize,
  vertical: bool, block_edge: bool,
) -> usize {
  let xdec = p.plane_cfg.xdec;
  let ydec = p.plane_cfg.ydec;

  // filter application is conditional on skip and block edge
  if !(block_edge
    || !block.skip
    || !prev_block.skip
    || block.ref_frames[0] == INTRA_FRAME
    || prev_block.ref_frames[0] == INTRA_FRAME)
  {
    0
  } else {
    let (txsize, prev_txsize) = if pli == 0 {
      (block.txsize, prev_block.txsize)
    } else {
      (
        block.bsize.largest_chroma_tx_size(xdec, ydec),
        prev_block.bsize.largest_chroma_tx_size(xdec, ydec),
      )
    };
    let (tx_n, prev_tx_n) = if vertical {
      (cmp::max(txsize.width_mi(), 1), cmp::max(prev_txsize.width_mi(), 1))
    } else {
      (cmp::max(txsize.height_mi(), 1), cmp::max(prev_txsize.height_mi(), 1))
    };
    cmp::min(
      if pli == 0 { 14 } else { 6 },
      cmp::min(tx_n, prev_tx_n) << MI_SIZE_LOG2,
    )
  }
}

// Must be called on a tx edge
#[inline]
fn deblock_level(
  deblock: &DeblockState, block: &Block, prev_block: &Block, pli: usize,
  vertical: bool,
) -> usize {
  let level = deblock_adjusted_level(deblock, block, pli, vertical);
  if level == 0 {
    deblock_adjusted_level(deblock, prev_block, pli, vertical)
  } else {
    level
  }
}

// four taps, 4 outputs (two are trivial)
#[inline]
fn filter_narrow2_4(
  p1: i32, p0: i32, q0: i32, q1: i32, shift: usize,
) -> [i32; 4] {
  let filter0 = clamp(p1 - q1, -128 << shift, (128 << shift) - 1);
  let filter1 =
    clamp(filter0 + 3 * (q0 - p0) + 4, -128 << shift, (128 << shift) - 1) >> 3;
  // be certain our optimization removing a clamp is sound
  debug_assert!({
    let base =
      clamp(filter0 + 3 * (q0 - p0), -128 << shift, (128 << shift) - 1);
    let test = clamp(base + 4, -128 << shift, (128 << shift) - 1) >> 3;
    filter1 == test
  });
  let filter2 =
    clamp(filter0 + 3 * (q0 - p0) + 3, -128 << shift, (128 << shift) - 1) >> 3;
  // be certain our optimization removing a clamp is sound
  debug_assert!({
    let base =
      clamp(filter0 + 3 * (q0 - p0), -128 << shift, (128 << shift) - 1);
    let test = clamp(base + 3, -128 << shift, (128 << shift) - 1) >> 3;
    filter2 == test
  });
  [
    p1,
    clamp(p0 + filter2, 0, (256 << shift) - 1),
    clamp(q0 - filter1, 0, (256 << shift) - 1),
    q1,
  ]
}

// six taps, 6 outputs (four are trivial)
#[inline]
fn filter_narrow2_6(
  p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32, shift: usize,
) -> [i32; 6] {
  let x = filter_narrow2_4(p1, p0, q0, q1, shift);
  [p2, x[0], x[1], x[2], x[3], q2]
}

// 12 taps, 12 outputs (ten are trivial)
#[inline]
fn filter_narrow2_12(
  p5: i32, p4: i32, p3: i32, p2: i32, p1: i32, p0: i32, q0: i32, q1: i32,
  q2: i32, q3: i32, q4: i32, q5: i32, shift: usize,
) -> [i32; 12] {
  let x = filter_narrow2_4(p1, p0, q0, q1, shift);
  [p5, p4, p3, p2, x[0], x[1], x[2], x[3], q2, q3, q4, q5]
}

// four taps, 4 outputs
#[inline]
fn filter_narrow4_4(
  p1: i32, p0: i32, q0: i32, q1: i32, shift: usize,
) -> [i32; 4] {
  let filter1 =
    clamp(3 * (q0 - p0) + 4, -128 << shift, (128 << shift) - 1) >> 3;
  // be certain our optimization removing a clamp is sound
  debug_assert!({
    let base = clamp(3 * (q0 - p0), -128 << shift, (128 << shift) - 1);
    let test = clamp(base + 4, -128 << shift, (128 << shift) - 1) >> 3;
    filter1 == test
  });
  let filter2 =
    clamp(3 * (q0 - p0) + 3, -128 << shift, (128 << shift) - 1) >> 3;
  // be certain our optimization removing a clamp is sound
  debug_assert!({
    let base = clamp(3 * (q0 - p0), -128 << shift, (128 << shift) - 1);
    let test = clamp(base + 3, -128 << shift, (128 << shift) - 1) >> 3;
    filter2 == test
  });
  let filter3 = (filter1 + 1) >> 1;
  [
    clamp(p1 + filter3, 0, (256 << shift) - 1),
    clamp(p0 + filter2, 0, (256 << shift) - 1),
    clamp(q0 - filter1, 0, (256 << shift) - 1),
    clamp(q1 - filter3, 0, (256 << shift) - 1),
  ]
}

// six taps, 6 outputs (two are trivial)
#[inline]
fn filter_narrow4_6(
  p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32, shift: usize,
) -> [i32; 6] {
  let x = filter_narrow4_4(p1, p0, q0, q1, shift);
  [p2, x[0], x[1], x[2], x[3], q2]
}

// 12 taps, 12 outputs (eight are trivial)
#[inline]
fn filter_narrow4_12(
  p5: i32, p4: i32, p3: i32, p2: i32, p1: i32, p0: i32, q0: i32, q1: i32,
  q2: i32, q3: i32, q4: i32, q5: i32, shift: usize,
) -> [i32; 12] {
  let x = filter_narrow4_4(p1, p0, q0, q1, shift);
  [p5, p4, p3, p2, x[0], x[1], x[2], x[3], q2, q3, q4, q5]
}

// six taps, 4 outputs
#[rustfmt::skip]
#[inline]
const fn filter_wide6_4(
  p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32
) -> [i32; 4] {
  [
    (p2*3 + p1*2 + p0*2 + q0   + (1<<2)) >> 3,
    (p2   + p1*2 + p0*2 + q0*2 + q1   + (1<<2)) >> 3,
           (p1   + p0*2 + q0*2 + q1*2 + q2   + (1<<2)) >> 3,
                  (p0   + q0*2 + q1*2 + q2*3 + (1<<2)) >> 3
  ]
}

// eight taps, 6 outputs
#[rustfmt::skip]
#[inline]
const fn filter_wide8_6(
  p3: i32, p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32, q3: i32
) -> [i32; 6] {
  [
    (p3*3 + p2*2 + p1   + p0   + q0   + (1<<2)) >> 3,
    (p3*2 + p2   + p1*2 + p0   + q0   + q1   + (1<<2)) >> 3,
    (p3   + p2   + p1   + p0*2 + q0   + q1   + q2   +(1<<2)) >> 3,
           (p2   + p1   + p0   + q0*2 + q1   + q2   + q3   + (1<<2)) >> 3,
                  (p1   + p0   + q0   + q1*2 + q2   + q3*2 + (1<<2)) >> 3,
                         (p0   + q0   + q1   + q2*2 + q3*3 + (1<<2)) >> 3
  ]
}

// 12 taps, 12 outputs (six are trivial)
#[inline]
const fn filter_wide8_12(
  p5: i32, p4: i32, p3: i32, p2: i32, p1: i32, p0: i32, q0: i32, q1: i32,
  q2: i32, q3: i32, q4: i32, q5: i32,
) -> [i32; 12] {
  let x = filter_wide8_6(p3, p2, p1, p0, q0, q1, q2, q3);
  [p5, p4, p3, x[0], x[1], x[2], x[3], x[4], x[5], q3, q4, q5]
}

// fourteen taps, 12 outputs
#[rustfmt::skip]
#[inline]
const fn filter_wide14_12(
  p6: i32, p5: i32, p4: i32, p3: i32, p2: i32, p1: i32, p0: i32, q0: i32,
  q1: i32, q2: i32, q3: i32, q4: i32, q5: i32, q6: i32
) -> [i32; 12] {
  [
    (p6*7 + p5*2 + p4*2 + p3   + p2   + p1   + p0   + q0   + (1<<3)) >> 4,
    (p6*5 + p5*2 + p4*2 + p3*2 + p2   + p1   + p0   + q0   + q1   + (1<<3)) >> 4,
    (p6*4 + p5   + p4*2 + p3*2 + p2*2 + p1   + p0   + q0   + q1   + q2   + (1<<3)) >> 4,
    (p6*3 + p5   + p4   + p3*2 + p2*2 + p1*2 + p0   + q0   + q1   + q2   + q3   + (1<<3)) >> 4,
    (p6*2 + p5   + p4   + p3   + p2*2 + p1*2 + p0*2 + q0   + q1   + q2   + q3   + q4   + (1<<3)) >> 4,
    (p6   + p5   + p4   + p3   + p2   + p1*2 + p0*2 + q0*2 + q1   + q2   + q3   + q4   + q5   + (1<<3)) >> 4,
           (p5   + p4   + p3   + p2   + p1   + p0*2 + q0*2 + q1*2 + q2   + q3   + q4   + q5   + q6 + (1<<3)) >> 4,
                  (p4   + p3   + p2   + p1   + p0   + q0*2 + q1*2 + q2*2 + q3   + q4   + q5   + q6*2 + (1<<3)) >> 4,
                         (p3   + p2   + p1   + p0   + q0   + q1*2 + q2*2 + q3*2 + q4   + q5   + q6*3 + (1<<3)) >> 4,
                                (p2   + p1   + p0   + q0   + q1   + q2*2 + q3*2 + q4*2 + q5   + q6*4 + (1<<3)) >> 4,
                                       (p1   + p0   + q0   + q1   + q2   + q3*2 + q4*2 + q5*2 + q6*5 + (1<<3)) >> 4,
                                              (p0   + q0   + q1   + q2   + q3   + q4*2 + q5*2 + q6*7 + (1<<3)) >> 4
  ]
}

#[inline]
fn copy_horizontal<T: Pixel>(
  dst: &mut PlaneRegionMut<'_, T>, x: usize, y: usize, src: &[i32],
) {
  let row = &mut dst[y][x..];
  for (dst, src) in row.iter_mut().take(src.len()).zip(src) {
    *dst = T::cast_from(*src);
  }
}

#[inline]
fn copy_vertical<T: Pixel>(
  dst: &mut PlaneRegionMut<'_, T>, x: usize, y: usize, src: &[i32],
) {
  for (i, v) in src.iter().enumerate() {
    let p = &mut dst[y + i][x];
    *p = T::cast_from(*v);
  }
}

#[inline]
fn stride_sse<const LEN: usize>(a: &[i32; LEN], b: &[i32; LEN]) -> i64 {
  a.iter().zip(b).map(|(a, b)| (a - b) * (a - b)).sum::<i32>() as i64
}

#[inline]
const fn _level_to_limit(level: i32, shift: usize) -> i32 {
  level << shift
}

#[inline]
const fn limit_to_level(limit: i32, shift: usize) -> i32 {
  (limit + (1 << shift) - 1) >> shift
}

#[inline]
const fn _level_to_blimit(level: i32, shift: usize) -> i32 {
  (3 * level + 4) << shift
}

#[inline]
const fn blimit_to_level(blimit: i32, shift: usize) -> i32 {
  (((blimit + (1 << shift) - 1) >> shift) - 2) / 3
}

#[inline]
const fn _level_to_thresh(level: i32, shift: usize) -> i32 {
  level >> 4 << shift
}

#[inline]
const fn thresh_to_level(thresh: i32, shift: usize) -> i32 {
  (thresh + (1 << shift) - 1) >> shift << 4
}

#[inline]
fn nhev4(p1: i32, p0: i32, q0: i32, q1: i32, shift: usize) -> usize {
  thresh_to_level(cmp::max((p1 - p0).abs(), (q1 - q0).abs()), shift) as usize
}

#[inline]
fn mask4(p1: i32, p0: i32, q0: i32, q1: i32, shift: usize) -> usize {
  cmp::max(
    limit_to_level(cmp::max((p1 - p0).abs(), (q1 - q0).abs()), shift),
    blimit_to_level((p0 - q0).abs() * 2 + (p1 - q1).abs() / 2, shift),
  ) as usize
}

#[inline]
fn deblock_size4_inner(
  [p1, p0, q0, q1]: [i32; 4], level: usize, bd: usize,
) -> Option<[i32; 4]> {
  if mask4(p1, p0, q0, q1, bd - 8) <= level {
    let x = if nhev4(p1, p0, q0, q1, bd - 8) <= level {
      filter_narrow4_4(p1, p0, q0, q1, bd - 8)
    } else {
      filter_narrow2_4(p1, p0, q0, q1, bd - 8)
    };
    Some(x)
  } else {
    None
  }
}

// Assumes rec[0] is set 2 taps back from the edge
fn deblock_v_size4<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for y in 0..4 {
    let p = &rec[y];
    let vals = [p[0].as_(), p[1].as_(), p[2].as_(), p[3].as_()];
    if let Some(data) = deblock_size4_inner(vals, level, bd) {
      copy_horizontal(rec, 0, y, &data);
    }
  }
}

// Assumes rec[0] is set 2 taps back from the edge
fn deblock_h_size4<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for x in 0..4 {
    let vals =
      [rec[0][x].as_(), rec[1][x].as_(), rec[2][x].as_(), rec[3][x].as_()];
    if let Some(data) = deblock_size4_inner(vals, level, bd) {
      copy_vertical(rec, x, 0, &data);
    }
  }
}

// Assumes rec[0] and src[0] are set 2 taps back from the edge.
// Accesses four taps, accumulates four pixels into the tally
fn sse_size4<T: Pixel>(
  rec: &PlaneRegion<'_, T>, src: &PlaneRegion<'_, T>,
  tally: &mut [i64; MAX_LOOP_FILTER + 2], horizontal_p: bool, bd: usize,
) {
  for i in 0..4 {
    let (p1, p0, q0, q1, a) = if horizontal_p {
      (
        rec[0][i].as_(),
        rec[1][i].as_(),
        rec[2][i].as_(),
        rec[3][i].as_(),
        [src[0][i].as_(), src[1][i].as_(), src[2][i].as_(), src[3][i].as_()],
      )
    } else {
      (
        rec[i][0].as_(),
        rec[i][1].as_(),
        rec[i][2].as_(),
        rec[i][3].as_(),
        [src[i][0].as_(), src[i][1].as_(), src[i][2].as_(), src[i][3].as_()],
      )
    };

    // three possibilities: no filter, narrow2 and narrow4
    // All possibilities produce four outputs
    let none: [_; 4] = [p1, p0, q0, q1];
    let narrow2 = filter_narrow2_4(p1, p0, q0, q1, bd - 8);
    let narrow4 = filter_narrow4_4(p1, p0, q0, q1, bd - 8);

    // mask4 sets the dividing line for filter vs no filter
    // nhev4 sets the dividing line between narrow2 and narrow4
    let mask = clamp(mask4(p1, p0, q0, q1, bd - 8), 1, MAX_LOOP_FILTER + 1);
    let nhev = clamp(nhev4(p1, p0, q0, q1, bd - 8), mask, MAX_LOOP_FILTER + 1);

    // sse for each; short-circuit the 'special' no-op cases.
    let sse_none = stride_sse(&a, &none);
    let sse_narrow2 =
      if nhev != mask { stride_sse(&a, &narrow2) } else { sse_none };
    let sse_narrow4 = if nhev <= MAX_LOOP_FILTER {
      stride_sse(&a, &narrow4)
    } else {
      sse_none
    };

    // accumulate possible filter values into the tally
    // level 0 is a special case
    tally[0] += sse_none;
    tally[mask] -= sse_none;
    tally[mask] += sse_narrow2;
    tally[nhev] -= sse_narrow2;
    tally[nhev] += sse_narrow4;
  }
}

#[inline]
fn mask6(
  p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32, shift: usize,
) -> usize {
  cmp::max(
    limit_to_level(
      cmp::max(
        (p2 - p1).abs(),
        cmp::max((p1 - p0).abs(), cmp::max((q2 - q1).abs(), (q1 - q0).abs())),
      ),
      shift,
    ),
    blimit_to_level((p0 - q0).abs() * 2 + (p1 - q1).abs() / 2, shift),
  ) as usize
}

#[inline]
fn flat6(p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32) -> usize {
  cmp::max(
    (p1 - p0).abs(),
    cmp::max((q1 - q0).abs(), cmp::max((p2 - p0).abs(), (q2 - q0).abs())),
  ) as usize
}

#[inline]
fn deblock_size6_inner(
  [p2, p1, p0, q0, q1, q2]: [i32; 6], level: usize, bd: usize,
) -> Option<[i32; 4]> {
  if mask6(p2, p1, p0, q0, q1, q2, bd - 8) <= level {
    let flat = 1 << (bd - 8);
    let x = if flat6(p2, p1, p0, q0, q1, q2) <= flat {
      filter_wide6_4(p2, p1, p0, q0, q1, q2)
    } else if nhev4(p1, p0, q0, q1, bd - 8) <= level {
      filter_narrow4_4(p1, p0, q0, q1, bd - 8)
    } else {
      filter_narrow2_4(p1, p0, q0, q1, bd - 8)
    };
    Some(x)
  } else {
    None
  }
}

// Assumes slice[0] is set 3 taps back from the edge
fn deblock_v_size6<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for y in 0..4 {
    let p = &rec[y];
    let vals =
      [p[0].as_(), p[1].as_(), p[2].as_(), p[3].as_(), p[4].as_(), p[5].as_()];
    if let Some(data) = deblock_size6_inner(vals, level, bd) {
      copy_horizontal(rec, 1, y, &data);
    }
  }
}

// Assumes slice[0] is set 3 taps back from the edge
fn deblock_h_size6<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for x in 0..4 {
    let vals = [
      rec[0][x].as_(),
      rec[1][x].as_(),
      rec[2][x].as_(),
      rec[3][x].as_(),
      rec[4][x].as_(),
      rec[5][x].as_(),
    ];
    if let Some(data) = deblock_size6_inner(vals, level, bd) {
      copy_vertical(rec, x, 1, &data);
    }
  }
}

// Assumes rec[0] and src[0] are set 3 taps back from the edge.
// Accesses six taps, accumulates four pixels into the tally
fn sse_size6<T: Pixel>(
  rec: &PlaneRegion<'_, T>, src: &PlaneRegion<'_, T>,
  tally: &mut [i64; MAX_LOOP_FILTER + 2], horizontal_p: bool, bd: usize,
) {
  let flat = 1 << (bd - 8);
  for i in 0..4 {
    let (p2, p1, p0, q0, q1, q2, a) = if horizontal_p {
      // six taps
      (
        rec[0][i].as_(),
        rec[1][i].as_(),
        rec[2][i].as_(),
        rec[3][i].as_(),
        rec[4][i].as_(),
        rec[5][i].as_(),
        // four pixels to compare so offset one forward
        [src[1][i].as_(), src[2][i].as_(), src[3][i].as_(), src[4][i].as_()],
      )
    } else {
      // six taps
      (
        rec[i][0].as_(),
        rec[i][1].as_(),
        rec[i][2].as_(),
        rec[i][3].as_(),
        rec[i][4].as_(),
        rec[i][5].as_(),
        // four pixels to compare so offset one forward
        [src[i][1].as_(), src[i][2].as_(), src[i][3].as_(), src[i][4].as_()],
      )
    };

    // Four possibilities: no filter, wide6, narrow2 and narrow4
    // All possibilities produce four outputs
    let none: [_; 4] = [p1, p0, q0, q1];
    let wide6 = filter_wide6_4(p2, p1, p0, q0, q1, q2);
    let narrow2 = filter_narrow2_4(p1, p0, q0, q1, bd - 8);
    let narrow4 = filter_narrow4_4(p1, p0, q0, q1, bd - 8);

    // mask6 sets the dividing line for filter vs no filter
    // flat6 decides between wide and narrow filters (unrelated to level)
    // nhev4 sets the dividing line between narrow2 and narrow4
    let mask =
      clamp(mask6(p2, p1, p0, q0, q1, q2, bd - 8), 1, MAX_LOOP_FILTER + 1);
    let flatp = flat6(p2, p1, p0, q0, q1, q2) <= flat;
    let nhev = clamp(nhev4(p1, p0, q0, q1, bd - 8), mask, MAX_LOOP_FILTER + 1);

    // sse for each; short-circuit the 'special' no-op cases.
    let sse_none = stride_sse(&a, &none);
    let sse_wide6 = if flatp && mask <= MAX_LOOP_FILTER {
      stride_sse(&a, &wide6)
    } else {
      sse_none
    };
    let sse_narrow2 =
      if !flatp && nhev != mask { stride_sse(&a, &narrow2) } else { sse_none };
    let sse_narrow4 = if !flatp && nhev <= MAX_LOOP_FILTER {
      stride_sse(&a, &narrow4)
    } else {
      sse_none
    };

    // accumulate possible filter values into the tally
    tally[0] += sse_none;
    tally[mask] -= sse_none;
    if flatp {
      tally[mask] += sse_wide6;
    } else {
      tally[mask] += sse_narrow2;
      tally[nhev] -= sse_narrow2;
      tally[nhev] += sse_narrow4;
    }
  }
}

#[inline]
fn mask8(
  p3: i32, p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32, q3: i32,
  shift: usize,
) -> usize {
  cmp::max(
    limit_to_level(
      cmp::max(
        (p3 - p2).abs(),
        cmp::max(
          (p2 - p1).abs(),
          cmp::max(
            (p1 - p0).abs(),
            cmp::max(
              (q3 - q2).abs(),
              cmp::max((q2 - q1).abs(), (q1 - q0).abs()),
            ),
          ),
        ),
      ),
      shift,
    ),
    blimit_to_level((p0 - q0).abs() * 2 + (p1 - q1).abs() / 2, shift),
  ) as usize
}

#[inline]
fn flat8(
  p3: i32, p2: i32, p1: i32, p0: i32, q0: i32, q1: i32, q2: i32, q3: i32,
) -> usize {
  cmp::max(
    (p1 - p0).abs(),
    cmp::max(
      (q1 - q0).abs(),
      cmp::max(
        (p2 - p0).abs(),
        cmp::max((q2 - q0).abs(), cmp::max((p3 - p0).abs(), (q3 - q0).abs())),
      ),
    ),
  ) as usize
}

#[inline]
fn deblock_size8_inner(
  [p3, p2, p1, p0, q0, q1, q2, q3]: [i32; 8], level: usize, bd: usize,
) -> Option<[i32; 6]> {
  if mask8(p3, p2, p1, p0, q0, q1, q2, q3, bd - 8) <= level {
    let flat = 1 << (bd - 8);
    let x = if flat8(p3, p2, p1, p0, q0, q1, q2, q3) <= flat {
      filter_wide8_6(p3, p2, p1, p0, q0, q1, q2, q3)
    } else if nhev4(p1, p0, q0, q1, bd - 8) <= level {
      filter_narrow4_6(p2, p1, p0, q0, q1, q2, bd - 8)
    } else {
      filter_narrow2_6(p2, p1, p0, q0, q1, q2, bd - 8)
    };
    Some(x)
  } else {
    None
  }
}

// Assumes rec[0] is set 4 taps back from the edge
fn deblock_v_size8<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for y in 0..4 {
    let p = &rec[y];
    let vals = [
      p[0].as_(),
      p[1].as_(),
      p[2].as_(),
      p[3].as_(),
      p[4].as_(),
      p[5].as_(),
      p[6].as_(),
      p[7].as_(),
    ];
    if let Some(data) = deblock_size8_inner(vals, level, bd) {
      copy_horizontal(rec, 1, y, &data);
    }
  }
}

// Assumes rec[0] is set 4 taps back from the edge
fn deblock_h_size8<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for x in 0..4 {
    let vals = [
      rec[0][x].as_(),
      rec[1][x].as_(),
      rec[2][x].as_(),
      rec[3][x].as_(),
      rec[4][x].as_(),
      rec[5][x].as_(),
      rec[6][x].as_(),
      rec[7][x].as_(),
    ];
    if let Some(data) = deblock_size8_inner(vals, level, bd) {
      copy_vertical(rec, x, 1, &data);
    }
  }
}

// Assumes rec[0] and src[0] are set 4 taps back from the edge.
// Accesses eight taps, accumulates six pixels into the tally
fn sse_size8<T: Pixel>(
  rec: &PlaneRegion<'_, T>, src: &PlaneRegion<'_, T>,
  tally: &mut [i64; MAX_LOOP_FILTER + 2], horizontal_p: bool, bd: usize,
) {
  let flat = 1 << (bd - 8);

  for i in 0..4 {
    let (p3, p2, p1, p0, q0, q1, q2, q3, a) = if horizontal_p {
      // eight taps
      (
        rec[0][i].as_(),
        rec[1][i].as_(),
        rec[2][i].as_(),
        rec[3][i].as_(),
        rec[4][i].as_(),
        rec[5][i].as_(),
        rec[6][i].as_(),
        rec[7][i].as_(),
        // six pixels to compare so offset one forward
        [
          src[1][i].as_(),
          src[2][i].as_(),
          src[3][i].as_(),
          src[4][i].as_(),
          src[5][i].as_(),
          src[6][i].as_(),
        ],
      )
    } else {
      // eight taps
      (
        rec[i][0].as_(),
        rec[i][1].as_(),
        rec[i][2].as_(),
        rec[i][3].as_(),
        rec[i][4].as_(),
        rec[i][5].as_(),
        rec[i][6].as_(),
        rec[i][7].as_(),
        // six pixels to compare so offset one forward
        [
          src[i][1].as_(),
          src[i][2].as_(),
          src[i][3].as_(),
          src[i][4].as_(),
          src[i][5].as_(),
          src[i][6].as_(),
        ],
      )
    };

    // Four possibilities: no filter, wide8, narrow2 and narrow4
    let none: [_; 6] = [p2, p1, p0, q0, q1, q2];
    let wide8: [_; 6] = filter_wide8_6(p3, p2, p1, p0, q0, q1, q2, q3);
    let narrow2: [_; 6] = filter_narrow2_6(p2, p1, p0, q0, q1, q2, bd - 8);
    let narrow4: [_; 6] = filter_narrow4_6(p2, p1, p0, q0, q1, q2, bd - 8);

    // mask8 sets the dividing line for filter vs no filter
    // flat8 decides between wide and narrow filters (unrelated to level)
    // nhev4 sets the dividing line between narrow2 and narrow4
    let mask = clamp(
      mask8(p3, p2, p1, p0, q0, q1, q2, q3, bd - 8),
      1,
      MAX_LOOP_FILTER + 1,
    );
    let flatp = flat8(p3, p2, p1, p0, q0, q1, q2, q3) <= flat;
    let nhev = clamp(nhev4(p1, p0, q0, q1, bd - 8), mask, MAX_LOOP_FILTER + 1);

    // sse for each; short-circuit the 'special' no-op cases.
    let sse_none = stride_sse(&a, &none);
    let sse_wide8 = if flatp && mask <= MAX_LOOP_FILTER {
      stride_sse(&a, &wide8)
    } else {
      sse_none
    };
    let sse_narrow2 =
      if !flatp && nhev != mask { stride_sse(&a, &narrow2) } else { sse_none };
    let sse_narrow4 = if !flatp && nhev <= MAX_LOOP_FILTER {
      stride_sse(&a, &narrow4)
    } else {
      sse_none
    };

    // accumulate possible filter values into the tally
    tally[0] += sse_none;
    tally[mask] -= sse_none;
    if flatp {
      tally[mask] += sse_wide8;
    } else {
      tally[mask] += sse_narrow2;
      tally[nhev] -= sse_narrow2;
      tally[nhev] += sse_narrow4;
    }
  }
}

#[inline]
fn flat14_outer(
  p6: i32, p5: i32, p4: i32, p0: i32, q0: i32, q4: i32, q5: i32, q6: i32,
) -> usize {
  cmp::max(
    (p4 - p0).abs(),
    cmp::max(
      (q4 - q0).abs(),
      cmp::max(
        (p5 - p0).abs(),
        cmp::max((q5 - q0).abs(), cmp::max((p6 - p0).abs(), (q6 - q0).abs())),
      ),
    ),
  ) as usize
}

#[inline]
fn deblock_size14_inner(
  [p6, p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5, q6]: [i32; 14],
  level: usize, bd: usize,
) -> Option<[i32; 12]> {
  // 'mask' test
  if mask8(p3, p2, p1, p0, q0, q1, q2, q3, bd - 8) <= level {
    let flat = 1 << (bd - 8);
    // inner flatness test
    let x = if flat8(p3, p2, p1, p0, q0, q1, q2, q3) <= flat {
      // outer flatness test
      if flat14_outer(p6, p5, p4, p0, q0, q4, q5, q6) <= flat {
        // sufficient flatness across 14 pixel width; run full-width filter
        filter_wide14_12(
          p6, p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5, q6,
        )
      } else {
        // only flat in inner area, run 8-tap
        filter_wide8_12(p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5)
      }
    } else if nhev4(p1, p0, q0, q1, bd - 8) <= level {
      // not flat, run narrow filter
      filter_narrow4_12(p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5, bd - 8)
    } else {
      filter_narrow2_12(p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5, bd - 8)
    };
    Some(x)
  } else {
    None
  }
}

// Assumes rec[0] is set 7 taps back from the edge
fn deblock_v_size14<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for y in 0..4 {
    let p = &rec[y];
    let vals = [
      p[0].as_(),
      p[1].as_(),
      p[2].as_(),
      p[3].as_(),
      p[4].as_(),
      p[5].as_(),
      p[6].as_(),
      p[7].as_(),
      p[8].as_(),
      p[9].as_(),
      p[10].as_(),
      p[11].as_(),
      p[12].as_(),
      p[13].as_(),
    ];
    if let Some(data) = deblock_size14_inner(vals, level, bd) {
      copy_horizontal(rec, 1, y, &data);
    }
  }
}

// Assumes rec[0] is set 7 taps back from the edge
fn deblock_h_size14<T: Pixel>(
  rec: &mut PlaneRegionMut<'_, T>, level: usize, bd: usize,
) {
  for x in 0..4 {
    let vals = [
      rec[0][x].as_(),
      rec[1][x].as_(),
      rec[2][x].as_(),
      rec[3][x].as_(),
      rec[4][x].as_(),
      rec[5][x].as_(),
      rec[6][x].as_(),
      rec[7][x].as_(),
      rec[8][x].as_(),
      rec[9][x].as_(),
      rec[10][x].as_(),
      rec[11][x].as_(),
      rec[12][x].as_(),
      rec[13][x].as_(),
    ];
    if let Some(data) = deblock_size14_inner(vals, level, bd) {
      copy_vertical(rec, x, 1, &data);
    }
  }
}

// Assumes rec[0] and src[0] are set 7 taps back from the edge.
// Accesses fourteen taps, accumulates twelve pixels into the tally
fn sse_size14<T: Pixel>(
  rec: &PlaneRegion<'_, T>, src: &PlaneRegion<'_, T>,
  tally: &mut [i64; MAX_LOOP_FILTER + 2], horizontal_p: bool, bd: usize,
) {
  let flat = 1 << (bd - 8);
  for i in 0..4 {
    let (p6, p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5, q6, a) =
      if horizontal_p {
        // 14 taps
        (
          rec[0][i].as_(),
          rec[1][i].as_(),
          rec[2][i].as_(),
          rec[3][i].as_(),
          rec[4][i].as_(),
          rec[5][i].as_(),
          rec[6][i].as_(),
          rec[7][i].as_(),
          rec[8][i].as_(),
          rec[9][i].as_(),
          rec[10][i].as_(),
          rec[11][i].as_(),
          rec[12][i].as_(),
          rec[13][i].as_(),
          // 12 pixels to compare so offset one forward
          [
            src[1][i].as_(),
            src[2][i].as_(),
            src[3][i].as_(),
            src[4][i].as_(),
            src[5][i].as_(),
            src[6][i].as_(),
            src[7][i].as_(),
            src[8][i].as_(),
            src[9][i].as_(),
            src[10][i].as_(),
            src[11][i].as_(),
            src[12][i].as_(),
          ],
        )
      } else {
        // 14 taps
        (
          rec[i][0].as_(),
          rec[i][1].as_(),
          rec[i][2].as_(),
          rec[i][3].as_(),
          rec[i][4].as_(),
          rec[i][5].as_(),
          rec[i][6].as_(),
          rec[i][7].as_(),
          rec[i][8].as_(),
          rec[i][9].as_(),
          rec[i][10].as_(),
          rec[i][11].as_(),
          rec[i][12].as_(),
          rec[i][13].as_(),
          // 12 pixels to compare so offset one forward
          [
            src[i][1].as_(),
            src[i][2].as_(),
            src[i][3].as_(),
            src[i][4].as_(),
            src[i][5].as_(),
            src[i][6].as_(),
            src[i][7].as_(),
            src[i][8].as_(),
            src[i][9].as_(),
            src[i][10].as_(),
            src[i][11].as_(),
            src[i][12].as_(),
          ],
        )
      };

    // Five possibilities: no filter, wide14, wide8, narrow2 and narrow4
    let none: [i32; 12] = [p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5];
    let wide14 =
      filter_wide14_12(p6, p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5, q6);
    let wide8 =
      filter_wide8_12(p5, p4, p3, p2, p1, p0, q0, q1, q2, q3, q4, q5);
    let narrow2 = filter_narrow2_12(
      p5,
      p4,
      p3,
      p2,
      p1,
      p0,
      q0,
      q1,
      q2,
      q3,
      q4,
      q5,
      bd - 8,
    );
    let narrow4 = filter_narrow4_12(
      p5,
      p4,
      p3,
      p2,
      p1,
      p0,
      q0,
      q1,
      q2,
      q3,
      q4,
      q5,
      bd - 8,
    );

    // mask8 sets the dividing line for filter vs no filter
    // flat8 decides between wide and narrow filters (unrelated to level)
    // flat14 decides between wide14 and wide8 filters
    // nhev4 sets the dividing line between narrow2 and narrow4
    let mask = clamp(
      mask8(p3, p2, p1, p0, q0, q1, q2, q3, bd - 8),
      1,
      MAX_LOOP_FILTER + 1,
    );
    let flat8p = flat8(p3, p2, p1, p0, q0, q1, q2, q3) <= flat;
    let flat14p = flat14_outer(p6, p5, p4, p0, q0, q4, q5, q6) <= flat;
    let nhev = clamp(nhev4(p1, p0, q0, q1, bd - 8), mask, MAX_LOOP_FILTER + 1);

    // sse for each; short-circuit the 'special' no-op cases.
    let sse_none = stride_sse(&a, &none);
    let sse_wide8 = if flat8p && !flat14p && mask <= MAX_LOOP_FILTER {
      stride_sse(&a, &wide8)
    } else {
      sse_none
    };
    let sse_wide14 = if flat8p && flat14p && mask <= MAX_LOOP_FILTER {
      stride_sse(&a, &wide14)
    } else {
      sse_none
    };
    let sse_narrow2 = if !flat8p && nhev != mask {
      stride_sse(&a, &narrow2)
    } else {
      sse_none
    };
    let sse_narrow4 = if !flat8p && nhev <= MAX_LOOP_FILTER {
      stride_sse(&a, &narrow4)
    } else {
      sse_none
    };

    // accumulate possible filter values into the tally
    tally[0] += sse_none;
    tally[mask] -= sse_none;
    if flat8p {
      if flat14p {
        tally[mask] += sse_wide14;
      } else {
        tally[mask] += sse_wide8;
      }
    } else {
      tally[mask] += sse_narrow2;
      tally[nhev] -= sse_narrow2;
      tally[nhev] += sse_narrow4;
    }
  }
}

fn filter_v_edge<T: Pixel>(
  deblock: &DeblockState, blocks: &TileBlocks, bo: TileBlockOffset,
  p: &mut PlaneRegionMut<T>, pli: usize, bd: usize, xdec: usize, ydec: usize,
) {
  let block = &blocks[bo];
  let txsize = if pli == 0 {
    block.txsize
  } else {
    block.bsize.largest_chroma_tx_size(xdec, ydec)
  };
  let tx_edge = (bo.0.x >> xdec) & (txsize.width_mi() - 1) == 0;
  if tx_edge {
    let prev_block = deblock_left(blocks, bo, &p.as_const());
    let block_edge = bo.0.x & (block.n4_w as usize - 1) == 0;
    let filter_size =
      deblock_size(block, prev_block, &p.as_const(), pli, true, block_edge);
    if filter_size > 0 {
      let level = deblock_level(deblock, block, prev_block, pli, true);
      if level > 0 {
        let po = bo.plane_offset(p.plane_cfg);
        let mut plane_region = p.subregion_mut(Area::Rect {
          x: po.x - (filter_size >> 1) as isize,
          y: po.y,
          width: filter_size,
          height: 4,
        });
        match filter_size {
          4 => {
            deblock_v_size4(&mut plane_region, level, bd);
          }
          6 => {
            deblock_v_size6(&mut plane_region, level, bd);
          }
          8 => {
            deblock_v_size8(&mut plane_region, level, bd);
          }
          14 => {
            deblock_v_size14(&mut plane_region, level, bd);
          }
          _ => unreachable!(),
        }
      }
    }
  }
}

fn sse_v_edge<T: Pixel>(
  blocks: &TileBlocks, bo: TileBlockOffset, rec_plane: &PlaneRegion<T>,
  src_plane: &PlaneRegion<T>, tally: &mut [i64; MAX_LOOP_FILTER + 2],
  pli: usize, bd: usize, xdec: usize, ydec: usize,
) {
  let block = &blocks[bo];
  let txsize = if pli == 0 {
    block.txsize
  } else {
    block.bsize.largest_chroma_tx_size(xdec, ydec)
  };
  let tx_edge = (bo.0.x >> xdec) & (txsize.width_mi() - 1) == 0;
  if tx_edge {
    let prev_block = deblock_left(blocks, bo, rec_plane);
    let block_edge = bo.0.x & (block.n4_w as usize - 1) == 0;
    let filter_size =
      deblock_size(block, prev_block, rec_plane, pli, true, block_edge);
    if filter_size > 0 {
      let po = bo.plane_offset(rec_plane.plane_cfg); // rec and src have identical subsampling
      let rec_region = rec_plane.subregion(Area::Rect {
        x: po.x - (filter_size >> 1) as isize,
        y: po.y,
        width: filter_size,
        height: 4,
      });
      let src_region = src_plane.subregion(Area::Rect {
        x: po.x - (filter_size >> 1) as isize,
        y: po.y,
        width: filter_size,
        height: 4,
      });
      match filter_size {
        4 => {
          sse_size4(&rec_region, &src_region, tally, false, bd);
        }
        6 => {
          sse_size6(&rec_region, &src_region, tally, false, bd);
        }
        8 => {
          sse_size8(&rec_region, &src_region, tally, false, bd);
        }
        14 => {
          sse_size14(&rec_region, &src_region, tally, false, bd);
        }
        _ => unreachable!(),
      }
    }
  }
}

fn filter_h_edge<T: Pixel>(
  deblock: &DeblockState, blocks: &TileBlocks, bo: TileBlockOffset,
  p: &mut PlaneRegionMut<T>, pli: usize, bd: usize, xdec: usize, ydec: usize,
) {
  let block = &blocks[bo];
  let txsize = if pli == 0 {
    block.txsize
  } else {
    block.bsize.largest_chroma_tx_size(xdec, ydec)
  };
  let tx_edge = (bo.0.y >> ydec) & (txsize.height_mi() - 1) == 0;
  if tx_edge {
    let prev_block = deblock_up(blocks, bo, &p.as_const());
    let block_edge = bo.0.y & (block.n4_h as usize - 1) == 0;
    let filter_size =
      deblock_size(block, prev_block, &p.as_const(), pli, false, block_edge);
    if filter_size > 0 {
      let level = deblock_level(deblock, block, prev_block, pli, false);
      if level > 0 {
        let po = bo.plane_offset(p.plane_cfg);
        let mut plane_region = p.subregion_mut(Area::Rect {
          x: po.x,
          y: po.y - (filter_size >> 1) as isize,
          width: 4,
          height: filter_size,
        });
        match filter_size {
          4 => {
            deblock_h_size4(&mut plane_region, level, bd);
          }
          6 => {
            deblock_h_size6(&mut plane_region, level, bd);
          }
          8 => {
            deblock_h_size8(&mut plane_region, level, bd);
          }
          14 => {
            deblock_h_size14(&mut plane_region, level, bd);
          }
          _ => unreachable!(),
        }
      }
    }
  }
}

fn sse_h_edge<T: Pixel>(
  blocks: &TileBlocks, bo: TileBlockOffset, rec_plane: &PlaneRegion<T>,
  src_plane: &PlaneRegion<T>, tally: &mut [i64; MAX_LOOP_FILTER + 2],
  pli: usize, bd: usize, xdec: usize, ydec: usize,
) {
  let block = &blocks[bo];
  let txsize = if pli == 0 {
    block.txsize
  } else {
    block.bsize.largest_chroma_tx_size(xdec, ydec)
  };
  let tx_edge = (bo.0.y >> ydec) & (txsize.height_mi() - 1) == 0;
  if tx_edge {
    let prev_block = deblock_up(blocks, bo, rec_plane);
    let block_edge = bo.0.y & (block.n4_h as usize - 1) == 0;
    let filter_size =
      deblock_size(block, prev_block, rec_plane, pli, true, block_edge);
    if filter_size > 0 {
      let po = bo.plane_offset(rec_plane.plane_cfg); // rec and src have identical subsampling
      let rec_region = rec_plane.subregion(Area::Rect {
        x: po.x,
        y: po.y - (filter_size >> 1) as isize,
        width: 4,
        height: filter_size,
      });
      let src_region = src_plane.subregion(Area::Rect {
        x: po.x,
        y: po.y - (filter_size >> 1) as isize,
        width: 4,
        height: filter_size,
      });

      match filter_size {
        4 => {
          sse_size4(&rec_region, &src_region, tally, true, bd);
        }
        6 => {
          sse_size6(&rec_region, &src_region, tally, true, bd);
        }
        8 => {
          sse_size8(&rec_region, &src_region, tally, true, bd);
        }
        14 => {
          sse_size14(&rec_region, &src_region, tally, true, bd);
        }
        _ => unreachable!(),
      }
    }
  }
}

// Deblocks all edges, vertical and horizontal, in a single plane
#[profiling::function]
pub fn deblock_plane<T: Pixel>(
  deblock: &DeblockState, p: &mut PlaneRegionMut<T>, pli: usize,
  blocks: &TileBlocks, crop_w: usize, crop_h: usize, bd: usize,
) {
  let xdec = p.plane_cfg.xdec;
  let ydec = p.plane_cfg.ydec;
  assert!(xdec <= 1 && ydec <= 1);

  match pli {
    0 => {
      if deblock.levels[0] == 0 && deblock.levels[1] == 0 {
        return;
      }
    }
    1 => {
      if deblock.levels[2] == 0 {
        return;
      }
    }
    2 => {
      if deblock.levels[3] == 0 {
        return;
      }
    }
    _ => return,
  }

  let rect = p.rect();
  let cols = (cmp::min(
    blocks.cols(),
    ((crop_w - rect.x as usize) + MI_SIZE - 1) >> MI_SIZE_LOG2,
  ) + (1 << xdec >> 1))
    >> xdec
    << xdec; // Clippy can go suck an egg
  let rows = (cmp::min(
    blocks.rows(),
    ((crop_h - rect.y as usize) + MI_SIZE - 1) >> MI_SIZE_LOG2,
  ) + (1 << ydec >> 1))
    >> ydec
    << ydec; // Clippy can go suck an egg

  // vertical edge filtering leads horizontal by one full MI-sized
  // row (and horizontal filtering doesn't happen along the upper
  // edge).  Unroll to avoid corner-cases.
  if rows > 0 {
    for x in (1 << xdec..cols).step_by(1 << xdec) {
      filter_v_edge(
        deblock,
        blocks,
        TileBlockOffset(BlockOffset { x, y: 0 }),
        p,
        pli,
        bd,
        xdec,
        ydec,
      );
    }
    if rows > 1 << ydec {
      for x in (1 << xdec..cols).step_by(1 << xdec) {
        filter_v_edge(
          deblock,
          blocks,
          TileBlockOffset(BlockOffset { x, y: 1 << ydec }),
          p,
          pli,
          bd,
          xdec,
          ydec,
        );
      }
    }
  }

  // filter rows where vertical and horizontal edge filtering both
  // happen (horizontal edge filtering lags vertical by one row).
  for y in ((2 << ydec)..rows).step_by(1 << ydec) {
    // Check for vertical edge at first MI block boundary on this row
    if cols > 1 << xdec {
      filter_v_edge(
        deblock,
        blocks,
        TileBlockOffset(BlockOffset { x: 1 << xdec, y }),
        p,
        pli,
        bd,
        xdec,
        ydec,
      );
    }
    // run the rest of the row with both vertical and horizontal edge filtering.
    // Horizontal lags vertical edge by one row and two columns.
    for x in (2 << xdec..cols).step_by(1 << xdec) {
      filter_v_edge(
        deblock,
        blocks,
        TileBlockOffset(BlockOffset { x, y }),
        p,
        pli,
        bd,
        xdec,
        ydec,
      );
      filter_h_edge(
        deblock,
        blocks,
        TileBlockOffset(BlockOffset {
          x: x - (2 << xdec),
          y: y - (1 << ydec),
        }),
        p,
        pli,
        bd,
        xdec,
        ydec,
      );
    }
    // ..and the last two horizontal edges for the row
    if cols >= 2 << xdec {
      filter_h_edge(
        deblock,
        blocks,
        TileBlockOffset(BlockOffset {
          x: cols - (2 << xdec),
          y: y - (1 << ydec),
        }),
        p,
        pli,
        bd,
        xdec,
        ydec,
      );
    }
    if cols >= 1 << xdec {
      filter_h_edge(
        deblock,
        blocks,
        TileBlockOffset(BlockOffset {
          x: cols - (1 << xdec),
          y: y - (1 << ydec),
        }),
        p,
        pli,
        bd,
        xdec,
        ydec,
      );
    }
  }

  // Last horizontal row, vertical is already complete
  if rows > 1 << ydec {
    for x in (0..cols).step_by(1 << xdec) {
      filter_h_edge(
        deblock,
        blocks,
        TileBlockOffset(BlockOffset { x, y: rows - (1 << ydec) }),
        p,
        pli,
        bd,
        xdec,
        ydec,
      );
    }
  }
}

// sse count of all edges in a single plane, accumulates into vertical and horizontal counts
fn sse_plane<T: Pixel>(
  rec: &PlaneRegion<T>, src: &PlaneRegion<T>,
  v_sse: &mut [i64; MAX_LOOP_FILTER + 2],
  h_sse: &mut [i64; MAX_LOOP_FILTER + 2], pli: usize, blocks: &TileBlocks,
  crop_w: usize, crop_h: usize, bd: usize,
) {
  let xdec = rec.plane_cfg.xdec;
  let ydec = rec.plane_cfg.ydec;
  assert!(xdec <= 1 && ydec <= 1);
  let rect = rec.rect();
  let cols = (cmp::min(
    blocks.cols(),
    (crop_w - rect.x as usize + MI_SIZE - 1) >> MI_SIZE_LOG2,
  ) + (1 << xdec >> 1))
    >> xdec
    << xdec; // Clippy can go suck an egg
  let rows = (cmp::min(
    blocks.rows(),
    (crop_h - rect.y as usize + MI_SIZE - 1) >> MI_SIZE_LOG2,
  ) + (1 << ydec >> 1))
    >> ydec
    << ydec; // Clippy can go suck an egg

  // No horizontal edge filtering along top of frame
  for x in (1 << xdec..cols).step_by(1 << xdec) {
    sse_v_edge(
      blocks,
      TileBlockOffset(BlockOffset { x, y: 0 }),
      rec,
      src,
      v_sse,
      pli,
      bd,
      xdec,
      ydec,
    );
  }

  // Unlike actual filtering, we're counting horizontal and vertical
  // as separable cases.  No need to lag the horizontal processing
  // behind vertical.
  for y in (1 << ydec..rows).step_by(1 << ydec) {
    // No vertical filtering along left edge of frame
    sse_h_edge(
      blocks,
      TileBlockOffset(BlockOffset { x: 0, y }),
      rec,
      src,
      h_sse,
      pli,
      bd,
      xdec,
      ydec,
    );
    for x in (1 << xdec..cols).step_by(1 << xdec) {
      sse_v_edge(
        blocks,
        TileBlockOffset(BlockOffset { x, y }),
        rec,
        src,
        v_sse,
        pli,
        bd,
        xdec,
        ydec,
      );
      sse_h_edge(
        blocks,
        TileBlockOffset(BlockOffset { x, y }),
        rec,
        src,
        h_sse,
        pli,
        bd,
        xdec,
        ydec,
      );
    }
  }
}

// Deblocks all edges in all planes of a frame
#[profiling::function]
pub fn deblock_filter_frame<T: Pixel>(
  deblock: &DeblockState, tile: &mut TileMut<T>, blocks: &TileBlocks,
  crop_w: usize, crop_h: usize, bd: usize, planes: usize,
) {
  tile.planes[..planes].par_iter_mut().enumerate().for_each(|(pli, plane)| {
    deblock_plane(deblock, plane, pli, blocks, crop_w, crop_h, bd);
  });
}

fn sse_optimize<T: Pixel>(
  rec: &Tile<T>, input: &Tile<T>, blocks: &TileBlocks, crop_w: usize,
  crop_h: usize, bd: usize, monochrome: bool,
) -> [u8; 4] {
  // i64 allows us to accumulate a total of ~ 35 bits worth of pixels
  assert!(
    ILog::ilog(input.planes[0].plane_cfg.width)
      + ILog::ilog(input.planes[0].plane_cfg.height)
      < 35
  );
  let mut level = [0; 4];
  let planes = if monochrome { 1 } else { MAX_PLANES };

  for pli in 0..planes {
    let mut v_tally: [i64; MAX_LOOP_FILTER + 2] = [0; MAX_LOOP_FILTER + 2];
    let mut h_tally: [i64; MAX_LOOP_FILTER + 2] = [0; MAX_LOOP_FILTER + 2];

    sse_plane(
      &rec.planes[pli],
      &input.planes[pli],
      &mut v_tally,
      &mut h_tally,
      pli,
      blocks,
      crop_w,
      crop_h,
      bd,
    );

    for i in 1..=MAX_LOOP_FILTER {
      v_tally[i] += v_tally[i - 1];
      h_tally[i] += h_tally[i - 1];
    }

    match pli {
      0 => {
        let mut best_v = 999;
        let mut best_h = 999;
        for i in 0..=MAX_LOOP_FILTER {
          if best_v == 999 || v_tally[best_v] > v_tally[i] {
            best_v = i;
          };
          if best_h == 999 || h_tally[best_h] > h_tally[i] {
            best_h = i;
          };
        }
        level[0] = best_v as u8;
        level[1] = best_h as u8;
      }
      1 | 2 => {
        let mut best = 999;
        for i in 0..=MAX_LOOP_FILTER {
          if best == 999
            || v_tally[best] + h_tally[best] > v_tally[i] + h_tally[i]
          {
            best = i;
          };
        }
        level[pli + 1] = best as u8;
      }
      _ => unreachable!(),
    }
  }
  level
}

#[profiling::function]
pub fn deblock_filter_optimize<T: Pixel, U: Pixel>(
  fi: &FrameInvariants<T>, rec: &Tile<U>, input: &Tile<U>,
  blocks: &TileBlocks, crop_w: usize, crop_h: usize,
) -> [u8; 4] {
  if fi.config.speed_settings.fast_deblock {
    let q = ac_q(fi.base_q_idx, 0, fi.sequence.bit_depth).get() as i32;
    let level = clamp(
      match fi.sequence.bit_depth {
        8 => {
          if fi.frame_type == FrameType::KEY {
            (q * 17563 - 421_574 + (1 << 18 >> 1)) >> 18
          } else {
            (q * 6017 + 650_707 + (1 << 18 >> 1)) >> 18
          }
        }
        10 => {
          if fi.frame_type == FrameType::KEY {
            ((q * 20723 + 4_060_632 + (1 << 20 >> 1)) >> 20) - 4
          } else {
            (q * 20723 + 4_060_632 + (1 << 20 >> 1)) >> 20
          }
        }
        12 => {
          if fi.frame_type == FrameType::KEY {
            ((q * 20723 + 16_242_526 + (1 << 22 >> 1)) >> 22) - 4
          } else {
            (q * 20723 + 16_242_526 + (1 << 22 >> 1)) >> 22
          }
        }
        _ => unreachable!(),
      },
      0,
      MAX_LOOP_FILTER as i32,
    ) as u8;
    [level; 4]
  } else {
    // Deblocking happens in 4x4 (luma) units; luma x,y are clipped to
    // the *crop frame* of the entire frame by 4x4 block.
    sse_optimize(
      rec,
      input,
      blocks,
      crop_w,
      crop_h,
      fi.sequence.bit_depth,
      fi.sequence.chroma_sampling == Cs400,
    )
  }
}
