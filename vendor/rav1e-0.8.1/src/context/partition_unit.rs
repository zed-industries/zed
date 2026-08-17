// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;

// Generates 4 bit field in which each bit set to 1 represents
// a blocksize partition  1111 means we split 64x64, 32x32, 16x16
// and 8x8.  1000 means we just split the 64x64 to 32x32
pub static partition_context_lookup: [[u8; 2]; BlockSize::BLOCK_SIZES_ALL] = [
  [31, 31], // 4X4   - {0b11111, 0b11111}
  [31, 30], // 4X8   - {0b11111, 0b11110}
  [30, 31], // 8X4   - {0b11110, 0b11111}
  [30, 30], // 8X8   - {0b11110, 0b11110}
  [30, 28], // 8X16  - {0b11110, 0b11100}
  [28, 30], // 16X8  - {0b11100, 0b11110}
  [28, 28], // 16X16 - {0b11100, 0b11100}
  [28, 24], // 16X32 - {0b11100, 0b11000}
  [24, 28], // 32X16 - {0b11000, 0b11100}
  [24, 24], // 32X32 - {0b11000, 0b11000}
  [24, 16], // 32X64 - {0b11000, 0b10000}
  [16, 24], // 64X32 - {0b10000, 0b11000}
  [16, 16], // 64X64 - {0b10000, 0b10000}
  [16, 0],  // 64X128- {0b10000, 0b00000}
  [0, 16],  // 128X64- {0b00000, 0b10000}
  [0, 0],   // 128X128-{0b00000, 0b00000}
  [31, 28], // 4X16  - {0b11111, 0b11100}
  [28, 31], // 16X4  - {0b11100, 0b11111}
  [30, 24], // 8X32  - {0b11110, 0b11000}
  [24, 30], // 32X8  - {0b11000, 0b11110}
  [28, 16], // 16X64 - {0b11100, 0b10000}
  [16, 28], // 64X16 - {0b10000, 0b11100}
];

pub const CFL_JOINT_SIGNS: usize = 8;
pub const CFL_ALPHA_CONTEXTS: usize = 6;
pub const CFL_ALPHABET_SIZE: usize = 16;

pub const PARTITION_PLOFFSET: usize = 4;
pub const PARTITION_BLOCK_SIZES: usize = 4 + 1;
const PARTITION_CONTEXTS_PRIMARY: usize =
  PARTITION_BLOCK_SIZES * PARTITION_PLOFFSET;
pub const PARTITION_CONTEXTS: usize = PARTITION_CONTEXTS_PRIMARY;
pub const PARTITION_TYPES: usize = 4;
pub const EXT_PARTITION_TYPES: usize = 10;

pub const SKIP_CONTEXTS: usize = 3;
pub const SKIP_MODE_CONTEXTS: usize = 3;

// partition contexts are at 8x8 granularity, as it is not possible to
// split 4x4 blocks any further than that
pub const PARTITION_CONTEXT_GRANULARITY: usize = 8;
pub const PARTITION_CONTEXT_MAX_WIDTH: usize =
  MAX_TILE_WIDTH / PARTITION_CONTEXT_GRANULARITY;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CFLSign {
  CFL_SIGN_ZERO = 0,
  CFL_SIGN_NEG = 1,
  CFL_SIGN_POS = 2,
}

impl CFLSign {
  pub const fn from_alpha(a: i16) -> CFLSign {
    [CFL_SIGN_NEG, CFL_SIGN_ZERO, CFL_SIGN_POS][(a.signum() + 1) as usize]
  }
}

use crate::context::CFLSign::*;

const CFL_SIGNS: usize = 3;
static cfl_sign_value: [i16; CFL_SIGNS] = [0, -1, 1];

#[derive(Copy, Clone, Debug)]
pub struct CFLParams {
  pub sign: [CFLSign; 2],
  pub scale: [u8; 2],
}

impl Default for CFLParams {
  #[inline]
  fn default() -> Self {
    Self { sign: [CFL_SIGN_NEG, CFL_SIGN_ZERO], scale: [1, 0] }
  }
}

impl CFLParams {
  /// # Panics
  ///
  /// - If either current sign is zero
  #[inline]
  pub fn joint_sign(self) -> u32 {
    assert!(self.sign[0] != CFL_SIGN_ZERO || self.sign[1] != CFL_SIGN_ZERO);
    (self.sign[0] as u32) * (CFL_SIGNS as u32) + (self.sign[1] as u32) - 1
  }
  /// # Panics
  ///
  /// - If the sign at index `uv` is zero
  #[inline]
  pub fn context(self, uv: usize) -> usize {
    assert!(self.sign[uv] != CFL_SIGN_ZERO);
    (self.sign[uv] as usize - 1) * CFL_SIGNS + (self.sign[1 - uv] as usize)
  }
  /// # Panics
  ///
  /// - If the sign at index `uv` is zero
  #[inline]
  pub fn index(self, uv: usize) -> u32 {
    assert!(self.sign[uv] != CFL_SIGN_ZERO && self.scale[uv] != 0);
    (self.scale[uv] - 1) as u32
  }
  #[inline]
  pub fn alpha(self, uv: usize) -> i16 {
    cfl_sign_value[self.sign[uv] as usize] * (self.scale[uv] as i16)
  }
  #[inline]
  pub const fn from_alpha(u: i16, v: i16) -> CFLParams {
    CFLParams {
      sign: [CFLSign::from_alpha(u), CFLSign::from_alpha(v)],
      scale: [u.unsigned_abs() as u8, v.unsigned_abs() as u8],
    }
  }
}

impl ContextWriter<'_> {
  fn partition_gather_horz_alike(
    out: &mut [u16; 2], cdf_in: &[u16], _bsize: BlockSize,
  ) {
    out[0] = 32768;
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_HORZ as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_SPLIT as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_HORZ_A as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_HORZ_B as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_VERT_A as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_HORZ_4 as usize,
    );
    out[0] = 32768 - out[0];
    out[1] = 0;
  }

  fn partition_gather_vert_alike(
    out: &mut [u16; 2], cdf_in: &[u16], _bsize: BlockSize,
  ) {
    out[0] = 32768;
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_VERT as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_SPLIT as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_HORZ_A as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_VERT_A as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_VERT_B as usize,
    );
    out[0] -= ContextWriter::cdf_element_prob(
      cdf_in,
      PartitionType::PARTITION_VERT_4 as usize,
    );
    out[0] = 32768 - out[0];
    out[1] = 0;
  }

  #[inline]
  pub fn write_skip<W: Writer>(
    &mut self, w: &mut W, bo: TileBlockOffset, skip: bool,
  ) {
    let ctx = self.bc.skip_context(bo);
    let cdf = &self.fc.skip_cdfs[ctx];
    symbol_with_update!(self, w, skip as u32, cdf);
  }

  pub fn get_segment_pred(
    &self, bo: TileBlockOffset, last_active_segid: u8,
  ) -> (u8, u8) {
    let mut prev_ul = -1;
    let mut prev_u = -1;
    let mut prev_l = -1;
    if bo.0.x > 0 && bo.0.y > 0 {
      prev_ul = self.bc.blocks.above_left_of(bo).segmentation_idx as i8;
    }
    if bo.0.y > 0 {
      prev_u = self.bc.blocks.above_of(bo).segmentation_idx as i8;
    }
    if bo.0.x > 0 {
      prev_l = self.bc.blocks.left_of(bo).segmentation_idx as i8;
    }

    /* Pick CDF index based on number of matching/out-of-bounds segment IDs. */
    let cdf_index: u8;
    if prev_ul < 0 || prev_u < 0 || prev_l < 0 {
      /* Edge case */
      cdf_index = 0;
    } else if (prev_ul == prev_u) && (prev_ul == prev_l) {
      cdf_index = 2;
    } else if (prev_ul == prev_u) || (prev_ul == prev_l) || (prev_u == prev_l)
    {
      cdf_index = 1;
    } else {
      cdf_index = 0;
    }

    /* If 2 or more are identical returns that as predictor, otherwise prev_l. */
    let r: i8;
    if prev_u == -1 {
      /* edge case */
      r = if prev_l == -1 { 0 } else { prev_l };
    } else if prev_l == -1 {
      /* edge case */
      r = prev_u;
    } else {
      r = if prev_ul == prev_u { prev_u } else { prev_l };
    }

    ((r as u8).min(last_active_segid), cdf_index)
  }

  pub fn write_cfl_alphas<W: Writer>(&mut self, w: &mut W, cfl: CFLParams) {
    symbol_with_update!(self, w, cfl.joint_sign(), &self.fc.cfl_sign_cdf);
    for uv in 0..2 {
      if cfl.sign[uv] != CFL_SIGN_ZERO {
        symbol_with_update!(
          self,
          w,
          cfl.index(uv),
          &self.fc.cfl_alpha_cdf[cfl.context(uv)]
        );
      }
    }
  }

  /// # Panics
  ///
  /// - If called with an 8x8 or larger `bsize`
  /// - If called with a `PartitionType` incompatible with the current block.
  pub fn write_partition(
    &mut self, w: &mut impl Writer, bo: TileBlockOffset, p: PartitionType,
    bsize: BlockSize,
  ) {
    debug_assert!(bsize.is_sqr());
    assert!(bsize >= BlockSize::BLOCK_8X8);
    let hbs = bsize.width_mi() / 2;
    let has_cols = (bo.0.x + hbs) < self.bc.blocks.cols();
    let has_rows = (bo.0.y + hbs) < self.bc.blocks.rows();
    let ctx = self.bc.partition_plane_context(bo, bsize);
    assert!(ctx < PARTITION_CONTEXTS);

    if !has_rows && !has_cols {
      return;
    }

    if has_rows && has_cols {
      if ctx < PARTITION_TYPES {
        let cdf = &self.fc.partition_w8_cdf[ctx];
        symbol_with_update!(self, w, p as u32, cdf);
      } else if ctx < 4 * PARTITION_TYPES {
        let cdf = &self.fc.partition_cdf[ctx - PARTITION_TYPES];
        symbol_with_update!(self, w, p as u32, cdf);
      } else {
        let cdf = &self.fc.partition_w128_cdf[ctx - 4 * PARTITION_TYPES];
        symbol_with_update!(self, w, p as u32, cdf);
      }
    } else if !has_rows && has_cols {
      assert!(
        p == PartitionType::PARTITION_SPLIT
          || p == PartitionType::PARTITION_HORZ
      );
      assert!(bsize > BlockSize::BLOCK_8X8);
      let mut cdf = [0u16; 2];
      if ctx < PARTITION_TYPES {
        let partition_cdf = &self.fc.partition_w8_cdf[ctx];
        ContextWriter::partition_gather_vert_alike(
          &mut cdf,
          partition_cdf,
          bsize,
        );
      } else if ctx < 4 * PARTITION_TYPES {
        let partition_cdf = &self.fc.partition_cdf[ctx - PARTITION_TYPES];
        ContextWriter::partition_gather_vert_alike(
          &mut cdf,
          partition_cdf,
          bsize,
        );
      } else {
        let partition_cdf =
          &self.fc.partition_w128_cdf[ctx - 4 * PARTITION_TYPES];
        ContextWriter::partition_gather_vert_alike(
          &mut cdf,
          partition_cdf,
          bsize,
        );
      }
      w.symbol((p == PartitionType::PARTITION_SPLIT) as u32, &cdf);
    } else {
      assert!(
        p == PartitionType::PARTITION_SPLIT
          || p == PartitionType::PARTITION_VERT
      );
      assert!(bsize > BlockSize::BLOCK_8X8);
      let mut cdf = [0u16; 2];
      if ctx < PARTITION_TYPES {
        let partition_cdf = &self.fc.partition_w8_cdf[ctx];
        ContextWriter::partition_gather_horz_alike(
          &mut cdf,
          partition_cdf,
          bsize,
        );
      } else if ctx < 4 * PARTITION_TYPES {
        let partition_cdf = &self.fc.partition_cdf[ctx - PARTITION_TYPES];
        ContextWriter::partition_gather_horz_alike(
          &mut cdf,
          partition_cdf,
          bsize,
        );
      } else {
        let partition_cdf =
          &self.fc.partition_w128_cdf[ctx - 4 * PARTITION_TYPES];
        ContextWriter::partition_gather_horz_alike(
          &mut cdf,
          partition_cdf,
          bsize,
        );
      }
      w.symbol((p == PartitionType::PARTITION_SPLIT) as u32, &cdf);
    }
  }

  fn neg_interleave(x: i32, r: i32, max: i32) -> i32 {
    assert!(x < max);
    if r == 0 {
      return x;
    } else if r >= (max - 1) {
      return -x + max - 1;
    }
    let diff = x - r;
    if 2 * r < max {
      if diff.abs() <= r {
        if diff > 0 {
          return (diff << 1) - 1;
        } else {
          return (-diff) << 1;
        }
      }
      x
    } else {
      if diff.abs() < (max - r) {
        if diff > 0 {
          return (diff << 1) - 1;
        } else {
          return (-diff) << 1;
        }
      }
      (max - x) - 1
    }
  }

  pub fn write_segmentation<W: Writer>(
    &mut self, w: &mut W, bo: TileBlockOffset, bsize: BlockSize, skip: bool,
    last_active_segid: u8,
  ) {
    let (pred, cdf_index) = self.get_segment_pred(bo, last_active_segid);
    if skip {
      self.bc.blocks.set_segmentation_idx(bo, bsize, pred);
      return;
    }
    let seg_idx = self.bc.blocks[bo].segmentation_idx;
    let coded_id = Self::neg_interleave(
      seg_idx as i32,
      pred as i32,
      (last_active_segid + 1) as i32,
    );
    symbol_with_update!(
      self,
      w,
      coded_id as u32,
      &self.fc.spatial_segmentation_cdfs[cdf_index as usize]
    );
  }
}

impl BlockContext<'_> {
  /// # Panics
  ///
  /// - If called with a non-square `bsize`
  pub fn partition_plane_context(
    &self, bo: TileBlockOffset, bsize: BlockSize,
  ) -> usize {
    // TODO: this should be way simpler without sub8x8
    let above_ctx = self.above_partition_context[bo.0.x >> 1];
    let left_ctx = self.left_partition_context[bo.y_in_sb() >> 1];
    let bsl = bsize.width_log2() - BLOCK_8X8.width_log2();
    let above = (above_ctx >> bsl) & 1;
    let left = (left_ctx >> bsl) & 1;

    assert!(bsize.is_sqr());

    (left * 2 + above) as usize + bsl * PARTITION_PLOFFSET
  }

  /// # Panics
  ///
  /// - If the block size is invalid for subsampling
  pub fn reset_skip_context(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, xdec: usize,
    ydec: usize, cs: ChromaSampling,
  ) {
    let num_planes = if cs == ChromaSampling::Cs400 { 1 } else { 3 };
    let nplanes = if bsize >= BLOCK_8X8 {
      num_planes
    } else {
      1 + (num_planes - 1) * has_chroma(bo, bsize, xdec, ydec, cs) as usize
    };

    for plane in 0..nplanes {
      let xdec2 = if plane == 0 { 0 } else { xdec };
      let ydec2 = if plane == 0 { 0 } else { ydec };

      let plane_bsize = if plane == 0 {
        bsize
      } else {
        bsize.subsampled_size(xdec2, ydec2).unwrap()
      };
      let bw = plane_bsize.width_mi();
      let bh = plane_bsize.height_mi();

      for above in
        &mut self.above_coeff_context[plane][(bo.0.x >> xdec2)..][..bw]
      {
        *above = 0;
      }

      let bo_y = bo.y_in_sb();
      for left in &mut self.left_coeff_context[plane][(bo_y >> ydec2)..][..bh]
      {
        *left = 0;
      }
    }
  }

  pub fn skip_context(&self, bo: TileBlockOffset) -> usize {
    let above_skip = bo.0.y > 0 && self.blocks.above_of(bo).skip;
    let left_skip = bo.0.x > 0 && self.blocks.left_of(bo).skip;
    above_skip as usize + left_skip as usize
  }

  /// # Panics
  ///
  /// - If called with a non-square `bsize`
  pub fn update_partition_context(
    &mut self, bo: TileBlockOffset, subsize: BlockSize, bsize: BlockSize,
  ) {
    assert!(bsize.is_sqr());

    let bw = bsize.width_mi();
    let bh = bsize.height_mi();

    let above_ctx =
      &mut self.above_partition_context[bo.0.x >> 1..(bo.0.x + bw) >> 1];
    let left_ctx = &mut self.left_partition_context
      [bo.y_in_sb() >> 1..(bo.y_in_sb() + bh) >> 1];

    // update the partition context at the end notes. set partition bits
    // of block sizes larger than the current one to be one, and partition
    // bits of smaller block sizes to be zero.
    for above in &mut above_ctx[..bw >> 1] {
      *above = partition_context_lookup[subsize as usize][0];
    }

    for left in &mut left_ctx[..bh >> 1] {
      *left = partition_context_lookup[subsize as usize][1];
    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn cdf_map() {
    use super::*;

    let cdf = CDFContext::new(8);
    let cdf_map = FieldMap { map: cdf.build_map() };
    let f = &cdf.partition_cdf[2];
    cdf_map.lookup(f.as_ptr() as usize);
  }

  use super::CFLSign;
  use super::CFLSign::*;

  static cfl_alpha_signs: [[CFLSign; 2]; 8] = [
    [CFL_SIGN_ZERO, CFL_SIGN_NEG],
    [CFL_SIGN_ZERO, CFL_SIGN_POS],
    [CFL_SIGN_NEG, CFL_SIGN_ZERO],
    [CFL_SIGN_NEG, CFL_SIGN_NEG],
    [CFL_SIGN_NEG, CFL_SIGN_POS],
    [CFL_SIGN_POS, CFL_SIGN_ZERO],
    [CFL_SIGN_POS, CFL_SIGN_NEG],
    [CFL_SIGN_POS, CFL_SIGN_POS],
  ];

  static cfl_context: [[usize; 8]; 2] =
    [[0, 0, 0, 1, 2, 3, 4, 5], [0, 3, 0, 1, 4, 0, 2, 5]];

  #[test]
  fn cfl_joint_sign() {
    use super::*;

    let mut cfl = CFLParams::default();
    for (joint_sign, &signs) in cfl_alpha_signs.iter().enumerate() {
      cfl.sign = signs;
      assert!(cfl.joint_sign() as usize == joint_sign);
      for uv in 0..2 {
        if signs[uv] != CFL_SIGN_ZERO {
          assert!(cfl.context(uv) == cfl_context[uv][joint_sign]);
        }
      }
    }
  }
}
