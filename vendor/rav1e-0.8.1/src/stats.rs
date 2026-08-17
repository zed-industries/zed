// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::partition::BlockSize;
use crate::predict::PREDICTION_MODES;
use crate::serialize::{Deserialize, Serialize};
use crate::transform::TX_TYPES;

#[cfg(feature = "serialize")]
use serde_big_array::BigArray;

use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncoderStats {
  /// Stores count of pixels belonging to each block size in this frame
  pub block_size_counts: [usize; BlockSize::BLOCK_SIZES_ALL],
  /// Stores count of pixels belonging to skip blocks in this frame
  pub skip_block_count: usize,
  /// Stores count of pixels belonging to each transform type in this frame
  pub tx_type_counts: [usize; TX_TYPES],
  /// Stores count of pixels belonging to each luma prediction mode in this frame
  #[serde(with = "BigArray")]
  pub luma_pred_mode_counts: [usize; PREDICTION_MODES],
  /// Stores count of pixels belonging to each chroma prediction mode in this frame
  #[serde(with = "BigArray")]
  pub chroma_pred_mode_counts: [usize; PREDICTION_MODES],
}

impl Default for EncoderStats {
  fn default() -> Self {
    let luma_pred_mode_counts = [0; PREDICTION_MODES];
    let chroma_pred_mode_counts = [0; PREDICTION_MODES];
    EncoderStats {
      block_size_counts: [0; BlockSize::BLOCK_SIZES_ALL],
      skip_block_count: 0,
      tx_type_counts: [0; TX_TYPES],
      luma_pred_mode_counts,
      chroma_pred_mode_counts,
    }
  }
}

impl Add<&Self> for EncoderStats {
  type Output = Self;

  fn add(self, rhs: &EncoderStats) -> Self::Output {
    let mut lhs = self;
    lhs += rhs;
    lhs
  }
}

impl AddAssign<&Self> for EncoderStats {
  fn add_assign(&mut self, rhs: &EncoderStats) {
    for (s, v) in
      self.block_size_counts.iter_mut().zip(rhs.block_size_counts.iter())
    {
      *s += v;
    }
    for (s, v) in self
      .chroma_pred_mode_counts
      .iter_mut()
      .zip(rhs.chroma_pred_mode_counts.iter())
    {
      *s += v;
    }
    for (s, v) in self
      .luma_pred_mode_counts
      .iter_mut()
      .zip(rhs.luma_pred_mode_counts.iter())
    {
      *s += v;
    }
    for (s, v) in self.tx_type_counts.iter_mut().zip(rhs.tx_type_counts.iter())
    {
      *s += v;
    }
    self.skip_block_count += rhs.skip_block_count;
  }
}
