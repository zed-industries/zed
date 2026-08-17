// Copyright (c) 2018-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::context::*;
use crate::header::PRIMARY_REF_NONE;
use crate::partition::BlockSize;
use crate::rdo::spatiotemporal_scale;
use crate::rdo::DistortionScale;
use crate::tiling::TileStateMut;
use crate::util::Pixel;
use crate::FrameInvariants;
use crate::FrameState;

pub const MAX_SEGMENTS: usize = 8;

#[profiling::function]
pub fn segmentation_optimize<T: Pixel>(
  fi: &FrameInvariants<T>, fs: &mut FrameState<T>,
) {
  assert!(fi.enable_segmentation);
  fs.segmentation.enabled = true;

  if fs.segmentation.enabled {
    fs.segmentation.update_map = true;

    // We don't change the values between frames.
    fs.segmentation.update_data = fi.primary_ref_frame == PRIMARY_REF_NONE;

    // Avoid going into lossless mode by never bringing qidx below 1.
    // Because base_q_idx changes more frequently than the segmentation
    // data, it is still possible for a segment to enter lossless, so
    // enforcement elsewhere is needed.
    let offset_lower_limit = 1 - fi.base_q_idx as i16;

    if !fs.segmentation.update_data {
      let mut min_segment = MAX_SEGMENTS;
      for i in 0..MAX_SEGMENTS {
        if fs.segmentation.features[i][SegLvl::SEG_LVL_ALT_Q as usize]
          && fs.segmentation.data[i][SegLvl::SEG_LVL_ALT_Q as usize]
            >= offset_lower_limit
        {
          min_segment = i;
          break;
        }
      }
      assert_ne!(min_segment, MAX_SEGMENTS);
      fs.segmentation.min_segment = min_segment as u8;
      fs.segmentation.update_threshold(fi.base_q_idx, fi.config.bit_depth);
      return;
    }

    segmentation_optimize_inner(fi, fs, offset_lower_limit);

    /* Figure out parameters */
    fs.segmentation.preskip = false;
    fs.segmentation.last_active_segid = 0;
    for i in 0..MAX_SEGMENTS {
      for j in 0..SegLvl::SEG_LVL_MAX as usize {
        if fs.segmentation.features[i][j] {
          fs.segmentation.last_active_segid = i as u8;
          if j >= SegLvl::SEG_LVL_REF_FRAME as usize {
            fs.segmentation.preskip = true;
          }
        }
      }
    }
  }
}

// Select target quantizers for each segment by fitting to log(scale).
fn segmentation_optimize_inner<T: Pixel>(
  fi: &FrameInvariants<T>, fs: &mut FrameState<T>, offset_lower_limit: i16,
) {
  use crate::quantize::{ac_q, select_ac_qi};
  use crate::util::kmeans;
  use arrayvec::ArrayVec;

  // Minimize the total distance from a small set of values to all scales.
  // Find k-means of log(spatiotemporal scale), k in 3..=8
  let c: ([_; 8], [_; 7], [_; 6], [_; 5], [_; 4], [_; 3]) = {
    let spatiotemporal_scores =
      &fi.coded_frame_data.as_ref().unwrap().spatiotemporal_scores;
    let mut log2_scale_q11 = Vec::with_capacity(spatiotemporal_scores.len());
    log2_scale_q11.extend(spatiotemporal_scores.iter().map(|&s| s.blog16()));
    log2_scale_q11.sort_unstable();
    let l = &log2_scale_q11;
    (kmeans(l), kmeans(l), kmeans(l), kmeans(l), kmeans(l), kmeans(l))
  };

  // Find variance in spacing between successive log(scale)
  let var = |c: &[i16]| {
    let delta = ArrayVec::<_, MAX_SEGMENTS>::from_iter(
      c.iter().skip(1).zip(c).map(|(&a, &b)| b as i64 - a as i64),
    );
    let mean = delta.iter().sum::<i64>() / delta.len() as i64;
    delta.iter().map(|&d| (d - mean).pow(2)).sum::<i64>() as u64
  };
  let variance =
    [var(&c.0), var(&c.1), var(&c.2), var(&c.3), var(&c.4), var(&c.5)];

  // Choose the k value with minimal variance in spacing
  let min_variance = *variance.iter().min().unwrap();
  let position = variance.iter().rposition(|&v| v == min_variance).unwrap();

  // For the selected centroids, derive a target quantizer:
  //   scale Q'^2 = Q^2
  // See `distortion_scale_for` for more information.
  let compute_delta = |centroids: &[i16]| {
    use crate::util::{bexp64, blog64};
    let log2_base_ac_q_q57 =
      blog64(ac_q(fi.base_q_idx, 0, fi.config.bit_depth).get().into());
    centroids
      .iter()
      .rev()
      // Rewrite in log form and exponentiate:
      //   scale Q'^2 = Q^2
      //           Q' = Q / sqrt(scale)
      //      log(Q') = log(Q) - 0.5 log(scale)
      .map(|&log2_scale_q11| {
        bexp64(log2_base_ac_q_q57 - ((log2_scale_q11 as i64) << (57 - 11 - 1)))
      })
      // Find the index of the nearest quantizer to the target,
      // and take the delta from the base quantizer index.
      .map(|q| {
        // Avoid going into lossless mode by never bringing qidx below 1.
        select_ac_qi(q, fi.config.bit_depth).max(1) as i16
          - fi.base_q_idx as i16
      })
      .collect::<ArrayVec<_, MAX_SEGMENTS>>()
  };

  // Compute segment deltas for best value of k
  let seg_delta = match position {
    0 => compute_delta(&c.0),
    1 => compute_delta(&c.1),
    2 => compute_delta(&c.2),
    3 => compute_delta(&c.3),
    4 => compute_delta(&c.4),
    _ => compute_delta(&c.5),
  };

  // Update the segmentation data
  fs.segmentation.min_segment = 0;
  fs.segmentation.max_segment = seg_delta.len() as u8 - 1;
  for (&delta, (features, data)) in seg_delta
    .iter()
    .zip(fs.segmentation.features.iter_mut().zip(&mut fs.segmentation.data))
  {
    features[SegLvl::SEG_LVL_ALT_Q as usize] = true;
    data[SegLvl::SEG_LVL_ALT_Q as usize] = delta.max(offset_lower_limit);
  }

  fs.segmentation.update_threshold(fi.base_q_idx, fi.config.bit_depth);
}

#[profiling::function]
pub fn select_segment<T: Pixel>(
  fi: &FrameInvariants<T>, ts: &TileStateMut<'_, T>, tile_bo: TileBlockOffset,
  bsize: BlockSize, skip: bool,
) -> std::ops::RangeInclusive<u8> {
  // If skip is true or segmentation is turned off, sidx is not coded.
  if skip || !fi.enable_segmentation {
    return 0..=0;
  }

  use crate::api::SegmentationLevel;
  if fi.config.speed_settings.segmentation == SegmentationLevel::Full {
    return ts.segmentation.min_segment..=ts.segmentation.max_segment;
  }

  let frame_bo = ts.to_frame_block_offset(tile_bo);
  let scale = spatiotemporal_scale(fi, frame_bo, bsize);

  let sidx = segment_idx_from_distortion(&ts.segmentation.threshold, scale);

  // Avoid going into lossless mode by never bringing qidx below 1.
  let sidx = sidx.max(ts.segmentation.min_segment);

  if fi.config.speed_settings.segmentation == SegmentationLevel::Complex {
    return sidx..=ts.segmentation.max_segment.min(sidx.saturating_add(1));
  }

  sidx..=sidx
}

fn segment_idx_from_distortion(
  threshold: &[DistortionScale; MAX_SEGMENTS - 1], s: DistortionScale,
) -> u8 {
  threshold.partition_point(|&t| s.0 < t.0) as u8
}
