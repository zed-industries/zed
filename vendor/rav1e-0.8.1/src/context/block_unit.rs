// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use std::mem::MaybeUninit;

use super::*;

use crate::predict::PredictionMode;

pub const MAX_PLANES: usize = 3;

pub const BLOCK_SIZE_GROUPS: usize = 4;
pub const MAX_ANGLE_DELTA: usize = 3;
pub const DIRECTIONAL_MODES: usize = 8;
pub const KF_MODE_CONTEXTS: usize = 5;

pub const INTRA_INTER_CONTEXTS: usize = 4;
pub const INTER_MODE_CONTEXTS: usize = 8;
pub const DRL_MODE_CONTEXTS: usize = 3;
pub const COMP_INTER_CONTEXTS: usize = 5;
pub const COMP_REF_TYPE_CONTEXTS: usize = 5;
pub const UNI_COMP_REF_CONTEXTS: usize = 3;

pub const PLANE_TYPES: usize = 2;
const REF_TYPES: usize = 2;

pub const COMP_INDEX_CONTEXTS: usize = 6;
pub const COMP_GROUP_IDX_CONTEXTS: usize = 6;

pub const COEFF_CONTEXT_MAX_WIDTH: usize = MAX_TILE_WIDTH / MI_SIZE;

/// Absolute offset in blocks, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BlockOffset {
  pub x: usize,
  pub y: usize,
}

/// Absolute offset in blocks inside a plane, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneBlockOffset(pub BlockOffset);

/// Absolute offset in blocks inside a tile, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TileBlockOffset(pub BlockOffset);

impl BlockOffset {
  /// Offset of the superblock in which this block is located.
  #[inline]
  const fn sb_offset(self) -> SuperBlockOffset {
    SuperBlockOffset {
      x: self.x >> SUPERBLOCK_TO_BLOCK_SHIFT,
      y: self.y >> SUPERBLOCK_TO_BLOCK_SHIFT,
    }
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    PlaneOffset {
      x: (self.x >> plane.xdec << BLOCK_TO_PLANE_SHIFT) as isize,
      y: (self.y >> plane.ydec << BLOCK_TO_PLANE_SHIFT) as isize,
    }
  }

  /// Convert to plane offset without decimation.
  #[inline]
  const fn to_luma_plane_offset(self) -> PlaneOffset {
    PlaneOffset {
      x: (self.x as isize) << BLOCK_TO_PLANE_SHIFT,
      y: (self.y as isize) << BLOCK_TO_PLANE_SHIFT,
    }
  }

  #[inline]
  const fn y_in_sb(self) -> usize {
    self.y % MIB_SIZE
  }

  #[inline]
  fn with_offset(self, col_offset: isize, row_offset: isize) -> BlockOffset {
    let x = self.x as isize + col_offset;
    let y = self.y as isize + row_offset;
    debug_assert!(x >= 0);
    debug_assert!(y >= 0);

    BlockOffset { x: x as usize, y: y as usize }
  }
}

impl PlaneBlockOffset {
  /// Offset of the superblock in which this block is located.
  #[inline]
  pub const fn sb_offset(self) -> PlaneSuperBlockOffset {
    PlaneSuperBlockOffset(self.0.sb_offset())
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  pub const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    self.0.plane_offset(plane)
  }

  /// Convert to plane offset without decimation.
  #[inline]
  pub const fn to_luma_plane_offset(self) -> PlaneOffset {
    self.0.to_luma_plane_offset()
  }

  #[inline]
  pub const fn y_in_sb(self) -> usize {
    self.0.y_in_sb()
  }

  #[inline]
  pub fn with_offset(
    self, col_offset: isize, row_offset: isize,
  ) -> PlaneBlockOffset {
    Self(self.0.with_offset(col_offset, row_offset))
  }
}

impl TileBlockOffset {
  /// Offset of the superblock in which this block is located.
  #[inline]
  pub const fn sb_offset(self) -> TileSuperBlockOffset {
    TileSuperBlockOffset(self.0.sb_offset())
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  pub const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    self.0.plane_offset(plane)
  }

  /// Convert to plane offset without decimation.
  #[inline]
  pub const fn to_luma_plane_offset(self) -> PlaneOffset {
    self.0.to_luma_plane_offset()
  }

  #[inline]
  pub const fn y_in_sb(self) -> usize {
    self.0.y_in_sb()
  }

  #[inline]
  pub fn with_offset(
    self, col_offset: isize, row_offset: isize,
  ) -> TileBlockOffset {
    Self(self.0.with_offset(col_offset, row_offset))
  }
}

#[derive(Copy, Clone)]
pub struct Block {
  pub mode: PredictionMode,
  pub partition: PartitionType,
  pub skip: bool,
  pub ref_frames: [RefType; 2],
  pub mv: [MotionVector; 2],
  // note: indexes are reflist index, NOT the same as libaom
  pub neighbors_ref_counts: [u8; INTER_REFS_PER_FRAME],
  pub cdef_index: u8,
  pub bsize: BlockSize,
  pub n4_w: u8, /* block width in the unit of mode_info */
  pub n4_h: u8, /* block height in the unit of mode_info */
  pub txsize: TxSize,
  // The block-level deblock_deltas are left-shifted by
  // fi.deblock.block_delta_shift and added to the frame-configured
  // deltas
  pub deblock_deltas: [i8; FRAME_LF_COUNT],
  pub segmentation_idx: u8,
}

impl Block {
  pub fn is_inter(&self) -> bool {
    self.mode >= PredictionMode::NEARESTMV
  }
  pub fn has_second_ref(&self) -> bool {
    self.ref_frames[1] != INTRA_FRAME && self.ref_frames[1] != NONE_FRAME
  }
}

impl Default for Block {
  fn default() -> Block {
    Block {
      mode: PredictionMode::DC_PRED,
      partition: PartitionType::PARTITION_NONE,
      skip: false,
      ref_frames: [INTRA_FRAME; 2],
      mv: [MotionVector::default(); 2],
      neighbors_ref_counts: [0; INTER_REFS_PER_FRAME],
      cdef_index: 0,
      bsize: BLOCK_64X64,
      n4_w: BLOCK_64X64.width_mi() as u8,
      n4_h: BLOCK_64X64.height_mi() as u8,
      txsize: TX_64X64,
      deblock_deltas: [0, 0, 0, 0],
      segmentation_idx: 0,
    }
  }
}

#[derive(Clone)]
pub struct BlockContextCheckpoint {
  x: usize,
  chroma_sampling: ChromaSampling,
  cdef_coded: bool,
  above_partition_context: [u8; MIB_SIZE >> 1],
  // left context is also at 8x8 granularity
  left_partition_context: [u8; MIB_SIZE >> 1],
  above_tx_context: [u8; MIB_SIZE],
  left_tx_context: [u8; MIB_SIZE],
  above_coeff_context: [[u8; MIB_SIZE]; MAX_PLANES],
  left_coeff_context: [[u8; MIB_SIZE]; MAX_PLANES],
}

pub struct BlockContext<'a> {
  pub cdef_coded: bool,
  pub code_deltas: bool,
  pub update_seg: bool,
  pub preskip_segid: bool,
  pub above_partition_context: [u8; PARTITION_CONTEXT_MAX_WIDTH],
  pub left_partition_context: [u8; MIB_SIZE >> 1],
  pub above_tx_context: [u8; COEFF_CONTEXT_MAX_WIDTH],
  pub left_tx_context: [u8; MIB_SIZE],
  pub above_coeff_context: [[u8; COEFF_CONTEXT_MAX_WIDTH]; MAX_PLANES],
  pub left_coeff_context: [[u8; MIB_SIZE]; MAX_PLANES],
  pub blocks: &'a mut TileBlocksMut<'a>,
}

impl<'a> BlockContext<'a> {
  pub fn new(blocks: &'a mut TileBlocksMut<'a>) -> Self {
    BlockContext {
      cdef_coded: false,
      code_deltas: false,
      update_seg: false,
      preskip_segid: false,
      above_partition_context: [0; PARTITION_CONTEXT_MAX_WIDTH],
      left_partition_context: [0; MIB_SIZE >> 1],
      above_tx_context: [0; COEFF_CONTEXT_MAX_WIDTH],
      left_tx_context: [0; MIB_SIZE],
      above_coeff_context: [
        [0; COEFF_CONTEXT_MAX_WIDTH],
        [0; COEFF_CONTEXT_MAX_WIDTH],
        [0; COEFF_CONTEXT_MAX_WIDTH],
      ],
      left_coeff_context: [[0; MIB_SIZE]; MAX_PLANES],
      blocks,
    }
  }

  pub fn checkpoint(
    &self, tile_bo: &TileBlockOffset, chroma_sampling: ChromaSampling,
  ) -> BlockContextCheckpoint {
    let x = tile_bo.0.x & (COEFF_CONTEXT_MAX_WIDTH - MIB_SIZE);
    let mut checkpoint = BlockContextCheckpoint {
      x,
      chroma_sampling,
      cdef_coded: self.cdef_coded,
      above_partition_context: [0; MIB_SIZE >> 1],
      left_partition_context: self.left_partition_context,
      above_tx_context: [0; MIB_SIZE],
      left_tx_context: self.left_tx_context,
      above_coeff_context: [[0; MIB_SIZE]; MAX_PLANES],
      left_coeff_context: self.left_coeff_context,
    };
    checkpoint.above_partition_context.copy_from_slice(
      &self.above_partition_context[(x >> 1)..][..(MIB_SIZE >> 1)],
    );
    checkpoint
      .above_tx_context
      .copy_from_slice(&self.above_tx_context[x..][..MIB_SIZE]);
    let num_planes =
      if chroma_sampling == ChromaSampling::Cs400 { 1 } else { 3 };
    for (p, (dst, src)) in checkpoint
      .above_coeff_context
      .iter_mut()
      .zip(self.above_coeff_context.iter())
      .enumerate()
      .take(num_planes)
    {
      let xdec = (p > 0 && chroma_sampling != ChromaSampling::Cs444) as usize;
      dst.copy_from_slice(&src[(x >> xdec)..][..MIB_SIZE]);
    }
    checkpoint
  }

  pub fn rollback(&mut self, checkpoint: &BlockContextCheckpoint) {
    let x = checkpoint.x & (COEFF_CONTEXT_MAX_WIDTH - MIB_SIZE);
    self.cdef_coded = checkpoint.cdef_coded;
    self.above_partition_context[(x >> 1)..][..(MIB_SIZE >> 1)]
      .copy_from_slice(&checkpoint.above_partition_context);
    self.left_partition_context = checkpoint.left_partition_context;
    self.above_tx_context[x..][..MIB_SIZE]
      .copy_from_slice(&checkpoint.above_tx_context);
    self.left_tx_context = checkpoint.left_tx_context;
    let num_planes =
      if checkpoint.chroma_sampling == ChromaSampling::Cs400 { 1 } else { 3 };
    for (p, (dst, src)) in self
      .above_coeff_context
      .iter_mut()
      .zip(checkpoint.above_coeff_context.iter())
      .enumerate()
      .take(num_planes)
    {
      let xdec = (p > 0 && checkpoint.chroma_sampling != ChromaSampling::Cs444)
        as usize;
      dst[(x >> xdec)..][..MIB_SIZE].copy_from_slice(src);
    }
    self.left_coeff_context = checkpoint.left_coeff_context;
  }

  #[inline]
  pub fn set_dc_sign(cul_level: &mut u32, dc_val: i32) {
    if dc_val < 0 {
      *cul_level |= 1 << COEFF_CONTEXT_BITS;
    } else if dc_val > 0 {
      *cul_level += 2 << COEFF_CONTEXT_BITS;
    }
  }

  pub fn set_coeff_context(
    &mut self, plane: usize, bo: TileBlockOffset, tx_size: TxSize,
    xdec: usize, ydec: usize, value: u8,
  ) {
    for above in &mut self.above_coeff_context[plane][(bo.0.x >> xdec)..]
      [..tx_size.width_mi()]
    {
      *above = value;
    }
    let bo_y = bo.y_in_sb();
    for left in &mut self.left_coeff_context[plane][(bo_y >> ydec)..]
      [..tx_size.height_mi()]
    {
      *left = value;
    }
  }

  fn reset_left_coeff_context(&mut self, plane: usize) {
    for c in &mut self.left_coeff_context[plane] {
      *c = 0;
    }
  }

  fn reset_left_partition_context(&mut self) {
    for c in &mut self.left_partition_context {
      *c = 0;
    }
  }

  pub fn update_tx_size_context(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, tx_size: TxSize,
    skip: bool,
  ) {
    let n4_w = bsize.width_mi();
    let n4_h = bsize.height_mi();

    let (tx_w, tx_h) = if skip {
      ((n4_w * MI_SIZE) as u8, (n4_h * MI_SIZE) as u8)
    } else {
      (tx_size.width() as u8, tx_size.height() as u8)
    };

    let above_ctx = &mut self.above_tx_context[bo.0.x..bo.0.x + n4_w];
    let left_ctx =
      &mut self.left_tx_context[bo.y_in_sb()..bo.y_in_sb() + n4_h];

    for v in above_ctx[0..n4_w].iter_mut() {
      *v = tx_w;
    }

    for v in left_ctx[0..n4_h].iter_mut() {
      *v = tx_h;
    }
  }

  fn reset_left_tx_context(&mut self) {
    for c in &mut self.left_tx_context {
      *c = 0;
    }
  }

  pub fn reset_left_contexts(&mut self, planes: usize) {
    for p in 0..planes {
      BlockContext::reset_left_coeff_context(self, p);
    }
    BlockContext::reset_left_partition_context(self);

    BlockContext::reset_left_tx_context(self);
  }

  // The mode info data structure has a one element border above and to the
  // left of the entries corresponding to real macroblocks.
  // The prediction flags in these dummy entries are initialized to 0.
  // 0 - inter/inter, inter/--, --/inter, --/--
  // 1 - intra/inter, inter/intra
  // 2 - intra/--, --/intra
  // 3 - intra/intra
  pub fn intra_inter_context(&self, bo: TileBlockOffset) -> usize {
    let has_above = bo.0.y > 0;
    let has_left = bo.0.x > 0;

    match (has_above, has_left) {
      (true, true) => {
        let above_intra = !self.blocks.above_of(bo).is_inter();
        let left_intra = !self.blocks.left_of(bo).is_inter();
        if above_intra && left_intra {
          3
        } else {
          (above_intra || left_intra) as usize
        }
      }
      (true, false) => {
        if self.blocks.above_of(bo).is_inter() {
          0
        } else {
          2
        }
      }
      (false, true) => {
        if self.blocks.left_of(bo).is_inter() {
          0
        } else {
          2
        }
      }
      _ => 0,
    }
  }

  pub fn get_txb_ctx(
    &self, plane_bsize: BlockSize, tx_size: TxSize, plane: usize,
    bo: TileBlockOffset, xdec: usize, ydec: usize, frame_clipped_txw: usize,
    frame_clipped_txh: usize,
  ) -> TXB_CTX {
    let mut txb_ctx = TXB_CTX { txb_skip_ctx: 0, dc_sign_ctx: 0 };
    const MAX_TX_SIZE_UNIT: usize = 16;
    const signs: [i8; 3] = [0, -1, 1];
    const dc_sign_contexts: [usize; 4 * MAX_TX_SIZE_UNIT + 1] = [
      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
      1, 1, 1, 1, 1, 1, 1, 1, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    ];
    let mut dc_sign: i16 = 0;

    let above_ctxs = &self.above_coeff_context[plane][(bo.0.x >> xdec)..]
      [..frame_clipped_txw >> 2];
    let left_ctxs = &self.left_coeff_context[plane][(bo.y_in_sb() >> ydec)..]
      [..frame_clipped_txh >> 2];

    // Decide txb_ctx.dc_sign_ctx
    for &ctx in above_ctxs {
      let sign = ctx >> COEFF_CONTEXT_BITS;
      dc_sign += signs[sign as usize] as i16;
    }

    for &ctx in left_ctxs {
      let sign = ctx >> COEFF_CONTEXT_BITS;
      dc_sign += signs[sign as usize] as i16;
    }

    txb_ctx.dc_sign_ctx =
      dc_sign_contexts[(dc_sign + 2 * MAX_TX_SIZE_UNIT as i16) as usize];

    // Decide txb_ctx.txb_skip_ctx
    if plane == 0 {
      if plane_bsize == tx_size.block_size() {
        txb_ctx.txb_skip_ctx = 0;
      } else {
        // This is the algorithm to generate table skip_contexts[min][max].
        //    if (!max)
        //      txb_skip_ctx = 1;
        //    else if (!min)
        //      txb_skip_ctx = 2 + (max > 3);
        //    else if (max <= 3)
        //      txb_skip_ctx = 4;
        //    else if (min <= 3)
        //      txb_skip_ctx = 5;
        //    else
        //      txb_skip_ctx = 6;
        const skip_contexts: [[u8; 5]; 5] = [
          [1, 2, 2, 2, 3],
          [1, 4, 4, 4, 5],
          [1, 4, 4, 4, 5],
          [1, 4, 4, 4, 5],
          [1, 4, 4, 4, 6],
        ];

        let top: u8 = above_ctxs.iter().fold(0, |acc, ctx| acc | *ctx)
          & COEFF_CONTEXT_MASK as u8;

        let left: u8 = left_ctxs.iter().fold(0, |acc, ctx| acc | *ctx)
          & COEFF_CONTEXT_MASK as u8;

        let max = cmp::min(top | left, 4);
        let min = cmp::min(cmp::min(top, left), 4);
        txb_ctx.txb_skip_ctx =
          skip_contexts[min as usize][max as usize] as usize;
      }
    } else {
      let top: u8 = above_ctxs.iter().fold(0, |acc, ctx| acc | *ctx);
      let left: u8 = left_ctxs.iter().fold(0, |acc, ctx| acc | *ctx);
      let ctx_base = (top != 0) as usize + (left != 0) as usize;
      let ctx_offset = if num_pels_log2_lookup[plane_bsize as usize]
        > num_pels_log2_lookup[tx_size.block_size() as usize]
      {
        10
      } else {
        7
      };
      txb_ctx.txb_skip_ctx = ctx_base + ctx_offset;
    }

    txb_ctx
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct NMVComponent {
  pub sign_cdf: [u16; 2],
  pub class0_hp_cdf: [u16; 2],
  pub hp_cdf: [u16; 2],
  pub class0_cdf: [u16; CLASS0_SIZE],
  pub bits_cdf: [[u16; 2]; MV_OFFSET_BITS],

  pub class0_fp_cdf: [[u16; MV_FP_SIZE]; CLASS0_SIZE],
  pub fp_cdf: [u16; MV_FP_SIZE],

  pub classes_cdf: [u16; MV_CLASSES],
  // MV_CLASSES + 5 == 16; pad the last CDF for rollback.
  padding: [u16; 5],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct NMVContext {
  pub joints_cdf: [u16; MV_JOINTS],
  // MV_JOINTS + 12 == 16; pad the last CDF for rollback.
  padding: [u16; 12],
  pub comps: [NMVComponent; 2],
}

// lv_map
pub static default_nmv_context: NMVContext = {
  NMVContext {
    joints_cdf: cdf([4096, 11264, 19328]),
    padding: [0; 12],
    comps: [
      NMVComponent {
        classes_cdf: cdf([
          28672, 30976, 31858, 32320, 32551, 32656, 32740, 32757, 32762, 32767,
        ]),
        class0_fp_cdf: cdf_2d([[16384, 24576, 26624], [12288, 21248, 24128]]),
        fp_cdf: cdf([8192, 17408, 21248]),
        sign_cdf: cdf([128 * 128]),
        class0_hp_cdf: cdf([160 * 128]),
        hp_cdf: cdf([128 * 128]),
        class0_cdf: cdf([216 * 128]),
        bits_cdf: cdf_2d([
          [128 * 136],
          [128 * 140],
          [128 * 148],
          [128 * 160],
          [128 * 176],
          [128 * 192],
          [128 * 224],
          [128 * 234],
          [128 * 234],
          [128 * 240],
        ]),
        padding: [0; 5],
      },
      NMVComponent {
        classes_cdf: cdf([
          28672, 30976, 31858, 32320, 32551, 32656, 32740, 32757, 32762, 32767,
        ]),
        class0_fp_cdf: cdf_2d([[16384, 24576, 26624], [12288, 21248, 24128]]),
        fp_cdf: cdf([8192, 17408, 21248]),
        sign_cdf: cdf([128 * 128]),
        class0_hp_cdf: cdf([160 * 128]),
        hp_cdf: cdf([128 * 128]),
        class0_cdf: cdf([216 * 128]),
        bits_cdf: cdf_2d([
          [128 * 136],
          [128 * 140],
          [128 * 148],
          [128 * 160],
          [128 * 176],
          [128 * 192],
          [128 * 224],
          [128 * 234],
          [128 * 234],
          [128 * 240],
        ]),
        padding: [0; 5],
      },
    ],
  }
};

#[derive(Clone)]
pub struct CandidateMV {
  pub this_mv: MotionVector,
  pub comp_mv: MotionVector,
  pub weight: u32,
}

#[derive(Clone)]
pub struct FrameBlocks {
  blocks: Box<[Block]>,
  pub cols: usize,
  pub rows: usize,
}

impl FrameBlocks {
  pub fn new(cols: usize, rows: usize) -> Self {
    Self {
      blocks: vec![Block::default(); cols * rows].into_boxed_slice(),
      cols,
      rows,
    }
  }

  #[inline(always)]
  pub fn as_tile_blocks(&self) -> TileBlocks<'_> {
    TileBlocks::new(self, 0, 0, self.cols, self.rows)
  }

  #[inline(always)]
  pub fn as_tile_blocks_mut(&mut self) -> TileBlocksMut<'_> {
    TileBlocksMut::new(self, 0, 0, self.cols, self.rows)
  }
}

impl Index<usize> for FrameBlocks {
  type Output = [Block];
  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    &self.blocks[index * self.cols..(index + 1) * self.cols]
  }
}

impl IndexMut<usize> for FrameBlocks {
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.blocks[index * self.cols..(index + 1) * self.cols]
  }
}

// for convenience, also index by BlockOffset

impl Index<PlaneBlockOffset> for FrameBlocks {
  type Output = Block;
  #[inline]
  fn index(&self, bo: PlaneBlockOffset) -> &Self::Output {
    &self[bo.0.y][bo.0.x]
  }
}

impl IndexMut<PlaneBlockOffset> for FrameBlocks {
  #[inline]
  fn index_mut(&mut self, bo: PlaneBlockOffset) -> &mut Self::Output {
    &mut self[bo.0.y][bo.0.x]
  }
}

impl ContextWriter<'_> {
  pub fn get_cdf_intra_mode_kf(
    &self, bo: TileBlockOffset,
  ) -> &[u16; INTRA_MODES] {
    static intra_mode_context: [usize; INTRA_MODES] =
      [0, 1, 2, 3, 4, 4, 4, 4, 3, 0, 1, 2, 0];
    let above_mode = if bo.0.y > 0 {
      self.bc.blocks.above_of(bo).mode
    } else {
      PredictionMode::DC_PRED
    };
    let left_mode = if bo.0.x > 0 {
      self.bc.blocks.left_of(bo).mode
    } else {
      PredictionMode::DC_PRED
    };
    let above_ctx = intra_mode_context[above_mode as usize];
    let left_ctx = intra_mode_context[left_mode as usize];
    &self.fc.kf_y_cdf[above_ctx][left_ctx]
  }

  pub fn write_intra_mode_kf<W: Writer>(
    &mut self, w: &mut W, bo: TileBlockOffset, mode: PredictionMode,
  ) {
    static intra_mode_context: [usize; INTRA_MODES] =
      [0, 1, 2, 3, 4, 4, 4, 4, 3, 0, 1, 2, 0];
    let above_mode = if bo.0.y > 0 {
      self.bc.blocks.above_of(bo).mode
    } else {
      PredictionMode::DC_PRED
    };
    let left_mode = if bo.0.x > 0 {
      self.bc.blocks.left_of(bo).mode
    } else {
      PredictionMode::DC_PRED
    };
    let above_ctx = intra_mode_context[above_mode as usize];
    let left_ctx = intra_mode_context[left_mode as usize];
    let cdf = &self.fc.kf_y_cdf[above_ctx][left_ctx];
    symbol_with_update!(self, w, mode as u32, cdf);
  }

  pub fn get_cdf_intra_mode(&self, bsize: BlockSize) -> &[u16; INTRA_MODES] {
    &self.fc.y_mode_cdf[size_group_lookup[bsize as usize] as usize]
  }

  #[inline]
  pub fn write_intra_mode<W: Writer>(
    &mut self, w: &mut W, bsize: BlockSize, mode: PredictionMode,
  ) {
    let cdf = &self.fc.y_mode_cdf[size_group_lookup[bsize as usize] as usize];
    symbol_with_update!(self, w, mode as u32, cdf);
  }

  #[inline]
  pub fn write_intra_uv_mode<W: Writer>(
    &mut self, w: &mut W, uv_mode: PredictionMode, y_mode: PredictionMode,
    bs: BlockSize,
  ) {
    if bs.cfl_allowed() {
      let cdf = &self.fc.uv_mode_cfl_cdf[y_mode as usize];
      symbol_with_update!(self, w, uv_mode as u32, cdf);
    } else {
      let cdf = &self.fc.uv_mode_cdf[y_mode as usize];
      symbol_with_update!(self, w, uv_mode as u32, cdf);
    }
  }

  #[inline]
  pub fn write_angle_delta<W: Writer>(
    &mut self, w: &mut W, angle: i8, mode: PredictionMode,
  ) {
    symbol_with_update!(
      self,
      w,
      (angle + MAX_ANGLE_DELTA as i8) as u32,
      &self.fc.angle_delta_cdf
        [mode as usize - PredictionMode::V_PRED as usize]
    );
  }

  pub fn write_use_filter_intra<W: Writer>(
    &mut self, w: &mut W, enable: bool, block_size: BlockSize,
  ) {
    let cdf = &self.fc.filter_intra_cdfs[block_size as usize];
    symbol_with_update!(self, w, enable as u32, cdf);
  }

  /// # Panics
  ///
  /// - If called with `enable: true` (not yet implemented
  pub fn write_use_palette_mode<W: Writer>(
    &mut self, w: &mut W, enable: bool, bsize: BlockSize, bo: TileBlockOffset,
    luma_mode: PredictionMode, chroma_mode: PredictionMode, xdec: usize,
    ydec: usize, cs: ChromaSampling,
  ) {
    if enable {
      unimplemented!(); // TODO
    }

    let (ctx_luma, ctx_chroma) = (0, 0); // TODO: increase based on surrounding block info

    if luma_mode == PredictionMode::DC_PRED {
      let bsize_ctx = bsize.width_mi_log2() + bsize.height_mi_log2() - 2;
      let cdf = &self.fc.palette_y_mode_cdfs[bsize_ctx][ctx_luma];
      symbol_with_update!(self, w, enable as u32, cdf);
    }

    if has_chroma(bo, bsize, xdec, ydec, cs)
      && chroma_mode == PredictionMode::DC_PRED
    {
      let cdf = &self.fc.palette_uv_mode_cdfs[ctx_chroma];
      symbol_with_update!(self, w, enable as u32, cdf);
    }
  }

  fn find_valid_row_offs(
    row_offset: isize, mi_row: usize, mi_rows: usize,
  ) -> isize {
    cmp::min(
      cmp::max(row_offset, -(mi_row as isize)),
      (mi_rows - mi_row - 1) as isize,
    )
  }

  fn find_valid_col_offs(
    col_offset: isize, mi_col: usize, mi_cols: usize,
  ) -> isize {
    cmp::min(
      cmp::max(col_offset, -(mi_col as isize)),
      (mi_cols - mi_col - 1) as isize,
    )
  }

  fn find_matching_mv(
    mv: MotionVector, mv_stack: &mut ArrayVec<CandidateMV, 9>,
  ) -> bool {
    for mv_cand in mv_stack {
      if mv.row == mv_cand.this_mv.row && mv.col == mv_cand.this_mv.col {
        return true;
      }
    }
    false
  }

  fn find_matching_mv_and_update_weight(
    mv: MotionVector, mv_stack: &mut ArrayVec<CandidateMV, 9>, weight: u32,
  ) -> bool {
    for mv_cand in mv_stack {
      if mv.row == mv_cand.this_mv.row && mv.col == mv_cand.this_mv.col {
        mv_cand.weight += weight;
        return true;
      }
    }
    false
  }

  fn find_matching_comp_mv_and_update_weight(
    mvs: [MotionVector; 2], mv_stack: &mut ArrayVec<CandidateMV, 9>,
    weight: u32,
  ) -> bool {
    for mv_cand in mv_stack {
      if mvs[0].row == mv_cand.this_mv.row
        && mvs[0].col == mv_cand.this_mv.col
        && mvs[1].row == mv_cand.comp_mv.row
        && mvs[1].col == mv_cand.comp_mv.col
      {
        mv_cand.weight += weight;
        return true;
      }
    }
    false
  }

  fn add_ref_mv_candidate(
    ref_frames: [RefType; 2], blk: &Block,
    mv_stack: &mut ArrayVec<CandidateMV, 9>, weight: u32,
    newmv_count: &mut usize, is_compound: bool,
  ) -> bool {
    if !blk.is_inter() {
      /* For intrabc */
      false
    } else if is_compound {
      if blk.ref_frames[0] == ref_frames[0]
        && blk.ref_frames[1] == ref_frames[1]
      {
        let found_match = Self::find_matching_comp_mv_and_update_weight(
          blk.mv, mv_stack, weight,
        );

        if !found_match && mv_stack.len() < MAX_REF_MV_STACK_SIZE {
          let mv_cand =
            CandidateMV { this_mv: blk.mv[0], comp_mv: blk.mv[1], weight };

          mv_stack.push(mv_cand);
        }

        if blk.mode.has_newmv() {
          *newmv_count += 1;
        }

        true
      } else {
        false
      }
    } else {
      let mut found = false;
      for i in 0..2 {
        if blk.ref_frames[i] == ref_frames[0] {
          let found_match = Self::find_matching_mv_and_update_weight(
            blk.mv[i], mv_stack, weight,
          );

          if !found_match && mv_stack.len() < MAX_REF_MV_STACK_SIZE {
            let mv_cand = CandidateMV {
              this_mv: blk.mv[i],
              comp_mv: MotionVector::default(),
              weight,
            };

            mv_stack.push(mv_cand);
          }

          if blk.mode.has_newmv() {
            *newmv_count += 1;
          }

          found = true;
        }
      }
      found
    }
  }

  fn add_extra_mv_candidate<T: Pixel>(
    blk: &Block, ref_frames: [RefType; 2],
    mv_stack: &mut ArrayVec<CandidateMV, 9>, fi: &FrameInvariants<T>,
    is_compound: bool, ref_id_count: &mut [usize; 2],
    ref_id_mvs: &mut [[MotionVector; 2]; 2], ref_diff_count: &mut [usize; 2],
    ref_diff_mvs: &mut [[MotionVector; 2]; 2],
  ) {
    if is_compound {
      for cand_list in 0..2 {
        let cand_ref = blk.ref_frames[cand_list];
        if cand_ref != INTRA_FRAME && cand_ref != NONE_FRAME {
          for list in 0..2 {
            let mut cand_mv = blk.mv[cand_list];
            if cand_ref == ref_frames[list] && ref_id_count[list] < 2 {
              ref_id_mvs[list][ref_id_count[list]] = cand_mv;
              ref_id_count[list] += 1;
            } else if ref_diff_count[list] < 2 {
              if fi.ref_frame_sign_bias[cand_ref.to_index()]
                != fi.ref_frame_sign_bias[ref_frames[list].to_index()]
              {
                cand_mv.row = -cand_mv.row;
                cand_mv.col = -cand_mv.col;
              }
              ref_diff_mvs[list][ref_diff_count[list]] = cand_mv;
              ref_diff_count[list] += 1;
            }
          }
        }
      }
    } else {
      for cand_list in 0..2 {
        let cand_ref = blk.ref_frames[cand_list];
        if cand_ref != INTRA_FRAME && cand_ref != NONE_FRAME {
          let mut mv = blk.mv[cand_list];
          if fi.ref_frame_sign_bias[cand_ref.to_index()]
            != fi.ref_frame_sign_bias[ref_frames[0].to_index()]
          {
            mv.row = -mv.row;
            mv.col = -mv.col;
          }

          if !Self::find_matching_mv(mv, mv_stack) {
            let mv_cand = CandidateMV {
              this_mv: mv,
              comp_mv: MotionVector::default(),
              weight: 2,
            };
            mv_stack.push(mv_cand);
          }
        }
      }
    }
  }

  fn scan_row_mbmi(
    &self, bo: TileBlockOffset, row_offset: isize, max_row_offs: isize,
    processed_rows: &mut isize, ref_frames: [RefType; 2],
    mv_stack: &mut ArrayVec<CandidateMV, 9>, newmv_count: &mut usize,
    bsize: BlockSize, is_compound: bool,
  ) -> bool {
    let bc = &self.bc;
    let target_n4_w = bsize.width_mi();

    let end_mi = cmp::min(
      cmp::min(target_n4_w, bc.blocks.cols() - bo.0.x),
      BLOCK_64X64.width_mi(),
    );
    let n4_w_8 = BLOCK_8X8.width_mi();
    let n4_w_16 = BLOCK_16X16.width_mi();
    let mut col_offset = 0;

    if row_offset.abs() > 1 {
      col_offset = 1;
      if ((bo.0.x & 0x01) != 0) && (target_n4_w < n4_w_8) {
        col_offset -= 1;
      }
    }

    let use_step_16 = target_n4_w >= 16;

    let mut found_match = false;

    let mut i = 0;
    while i < end_mi {
      let cand =
        &bc.blocks[bo.with_offset(col_offset + i as isize, row_offset)];

      let n4_w = cand.n4_w as usize;
      let mut len = cmp::min(target_n4_w, n4_w);
      if use_step_16 {
        len = cmp::max(n4_w_16, len);
      } else if row_offset.abs() > 1 {
        len = cmp::max(len, n4_w_8);
      }

      let mut weight: u32 = 2;
      if target_n4_w >= n4_w_8 && target_n4_w <= n4_w {
        let inc = cmp::min(-max_row_offs + row_offset + 1, cand.n4_h as isize);
        assert!(inc >= 0);
        weight = cmp::max(weight, inc as u32);
        *processed_rows = inc - row_offset - 1;
      }

      if Self::add_ref_mv_candidate(
        ref_frames,
        cand,
        mv_stack,
        len as u32 * weight,
        newmv_count,
        is_compound,
      ) {
        found_match = true;
      }

      i += len;
    }

    found_match
  }

  fn scan_col_mbmi(
    &self, bo: TileBlockOffset, col_offset: isize, max_col_offs: isize,
    processed_cols: &mut isize, ref_frames: [RefType; 2],
    mv_stack: &mut ArrayVec<CandidateMV, 9>, newmv_count: &mut usize,
    bsize: BlockSize, is_compound: bool,
  ) -> bool {
    let bc = &self.bc;

    let target_n4_h = bsize.height_mi();

    let end_mi = cmp::min(
      cmp::min(target_n4_h, bc.blocks.rows() - bo.0.y),
      BLOCK_64X64.height_mi(),
    );
    let n4_h_8 = BLOCK_8X8.height_mi();
    let n4_h_16 = BLOCK_16X16.height_mi();
    let mut row_offset = 0;

    if col_offset.abs() > 1 {
      row_offset = 1;
      if ((bo.0.y & 0x01) != 0) && (target_n4_h < n4_h_8) {
        row_offset -= 1;
      }
    }

    let use_step_16 = target_n4_h >= 16;

    let mut found_match = false;

    let mut i = 0;
    while i < end_mi {
      let cand =
        &bc.blocks[bo.with_offset(col_offset, row_offset + i as isize)];
      let n4_h = cand.n4_h as usize;
      let mut len = cmp::min(target_n4_h, n4_h);
      if use_step_16 {
        len = cmp::max(n4_h_16, len);
      } else if col_offset.abs() > 1 {
        len = cmp::max(len, n4_h_8);
      }

      let mut weight: u32 = 2;
      if target_n4_h >= n4_h_8 && target_n4_h <= n4_h {
        let inc = cmp::min(-max_col_offs + col_offset + 1, cand.n4_w as isize);
        assert!(inc >= 0);
        weight = cmp::max(weight, inc as u32);
        *processed_cols = inc - col_offset - 1;
      }

      if Self::add_ref_mv_candidate(
        ref_frames,
        cand,
        mv_stack,
        len as u32 * weight,
        newmv_count,
        is_compound,
      ) {
        found_match = true;
      }

      i += len;
    }

    found_match
  }

  fn scan_blk_mbmi(
    &self, bo: TileBlockOffset, ref_frames: [RefType; 2],
    mv_stack: &mut ArrayVec<CandidateMV, 9>, newmv_count: &mut usize,
    is_compound: bool,
  ) -> bool {
    if bo.0.x >= self.bc.blocks.cols() || bo.0.y >= self.bc.blocks.rows() {
      return false;
    }

    let weight = 2 * BLOCK_8X8.width_mi() as u32;
    /* Always assume its within a tile, probably wrong */
    Self::add_ref_mv_candidate(
      ref_frames,
      &self.bc.blocks[bo],
      mv_stack,
      weight,
      newmv_count,
      is_compound,
    )
  }

  fn add_offset(mv_stack: &mut ArrayVec<CandidateMV, 9>) {
    for cand_mv in mv_stack {
      cand_mv.weight += REF_CAT_LEVEL;
    }
  }

  #[profiling::function]
  fn setup_mvref_list<T: Pixel>(
    &self, bo: TileBlockOffset, ref_frames: [RefType; 2],
    mv_stack: &mut ArrayVec<CandidateMV, 9>, bsize: BlockSize,
    fi: &FrameInvariants<T>, is_compound: bool,
  ) -> usize {
    let (_rf, _rf_num) = (INTRA_FRAME, 1);

    let target_n4_h = bsize.height_mi();
    let target_n4_w = bsize.width_mi();

    let mut max_row_offs: isize = 0;
    let row_adj =
      (target_n4_h < BLOCK_8X8.height_mi()) && (bo.0.y & 0x01) != 0x0;

    let mut max_col_offs: isize = 0;
    let col_adj =
      (target_n4_w < BLOCK_8X8.width_mi()) && (bo.0.x & 0x01) != 0x0;

    let mut processed_rows: isize = 0;
    let mut processed_cols: isize = 0;

    let up_avail = bo.0.y > 0;
    let left_avail = bo.0.x > 0;

    if up_avail {
      max_row_offs = -2 * MVREF_ROW_COLS as isize + row_adj as isize;

      // limit max offset for small blocks
      if target_n4_h < BLOCK_8X8.height_mi() {
        max_row_offs = -2 * 2 + row_adj as isize;
      }

      let rows = self.bc.blocks.rows();
      max_row_offs = Self::find_valid_row_offs(max_row_offs, bo.0.y, rows);
    }

    if left_avail {
      max_col_offs = -2 * MVREF_ROW_COLS as isize + col_adj as isize;

      // limit max offset for small blocks
      if target_n4_w < BLOCK_8X8.width_mi() {
        max_col_offs = -2 * 2 + col_adj as isize;
      }

      let cols = self.bc.blocks.cols();
      max_col_offs = Self::find_valid_col_offs(max_col_offs, bo.0.x, cols);
    }

    let mut row_match = false;
    let mut col_match = false;
    let mut newmv_count: usize = 0;

    if max_row_offs.abs() >= 1 {
      let found_match = self.scan_row_mbmi(
        bo,
        -1,
        max_row_offs,
        &mut processed_rows,
        ref_frames,
        mv_stack,
        &mut newmv_count,
        bsize,
        is_compound,
      );
      row_match |= found_match;
    }
    if max_col_offs.abs() >= 1 {
      let found_match = self.scan_col_mbmi(
        bo,
        -1,
        max_col_offs,
        &mut processed_cols,
        ref_frames,
        mv_stack,
        &mut newmv_count,
        bsize,
        is_compound,
      );
      col_match |= found_match;
    }
    if has_tr(bo, bsize) && bo.0.y > 0 {
      let found_match = self.scan_blk_mbmi(
        bo.with_offset(target_n4_w as isize, -1),
        ref_frames,
        mv_stack,
        &mut newmv_count,
        is_compound,
      );
      row_match |= found_match;
    }

    let nearest_match = usize::from(row_match) + usize::from(col_match);

    Self::add_offset(mv_stack);

    /* Scan the second outer area. */
    let mut far_newmv_count: usize = 0; // won't be used

    let found_match = bo.0.x > 0
      && bo.0.y > 0
      && self.scan_blk_mbmi(
        bo.with_offset(-1, -1),
        ref_frames,
        mv_stack,
        &mut far_newmv_count,
        is_compound,
      );
    row_match |= found_match;

    for idx in 2..=MVREF_ROW_COLS {
      let row_offset = -2 * idx as isize + 1 + row_adj as isize;
      let col_offset = -2 * idx as isize + 1 + col_adj as isize;

      if row_offset.abs() <= max_row_offs.abs()
        && row_offset.abs() > processed_rows
      {
        let found_match = self.scan_row_mbmi(
          bo,
          row_offset,
          max_row_offs,
          &mut processed_rows,
          ref_frames,
          mv_stack,
          &mut far_newmv_count,
          bsize,
          is_compound,
        );
        row_match |= found_match;
      }

      if col_offset.abs() <= max_col_offs.abs()
        && col_offset.abs() > processed_cols
      {
        let found_match = self.scan_col_mbmi(
          bo,
          col_offset,
          max_col_offs,
          &mut processed_cols,
          ref_frames,
          mv_stack,
          &mut far_newmv_count,
          bsize,
          is_compound,
        );
        col_match |= found_match;
      }
    }

    let total_match = usize::from(row_match) + usize::from(col_match);

    assert!(total_match >= nearest_match);

    // mode_context contains both newmv_context and refmv_context, where newmv_context
    // lies in the REF_MVOFFSET least significant bits
    let mode_context = match nearest_match {
      0 => cmp::min(total_match, 1) + (total_match << REFMV_OFFSET),
      1 => 3 - cmp::min(newmv_count, 1) + ((2 + total_match) << REFMV_OFFSET),
      _ => 5 - cmp::min(newmv_count, 1) + (5 << REFMV_OFFSET),
    };

    /* TODO: Find nearest match and assign nearest and near mvs */

    // 7.10.2.11 Sort MV stack according to weight
    mv_stack.sort_by(|a, b| b.weight.cmp(&a.weight));

    if mv_stack.len() < 2 {
      // 7.10.2.12 Extra search process

      let w4 = bsize.width_mi().min(16).min(self.bc.blocks.cols() - bo.0.x);
      let h4 = bsize.height_mi().min(16).min(self.bc.blocks.rows() - bo.0.y);
      let num4x4 = w4.min(h4);

      let passes = i32::from(!up_avail)..=i32::from(left_avail);

      let mut ref_id_count: [usize; 2] = [0; 2];
      let mut ref_diff_count: [usize; 2] = [0; 2];
      let mut ref_id_mvs = [[MotionVector::default(); 2]; 2];
      let mut ref_diff_mvs = [[MotionVector::default(); 2]; 2];

      for pass in passes {
        let mut idx = 0;
        while idx < num4x4 && mv_stack.len() < 2 {
          let rbo = if pass == 0 {
            bo.with_offset(idx as isize, -1)
          } else {
            bo.with_offset(-1, idx as isize)
          };

          let blk = &self.bc.blocks[rbo];
          Self::add_extra_mv_candidate(
            blk,
            ref_frames,
            mv_stack,
            fi,
            is_compound,
            &mut ref_id_count,
            &mut ref_id_mvs,
            &mut ref_diff_count,
            &mut ref_diff_mvs,
          );

          idx += if pass == 0 { blk.n4_w } else { blk.n4_h } as usize;
        }
      }

      if is_compound {
        let mut combined_mvs = [[MotionVector::default(); 2]; 2];

        for list in 0..2 {
          let mut comp_count = 0;
          for idx in 0..ref_id_count[list] {
            combined_mvs[comp_count][list] = ref_id_mvs[list][idx];
            comp_count += 1;
          }
          for idx in 0..ref_diff_count[list] {
            if comp_count < 2 {
              combined_mvs[comp_count][list] = ref_diff_mvs[list][idx];
              comp_count += 1;
            }
          }
        }

        if mv_stack.len() == 1 {
          let mv_cand = if combined_mvs[0][0].row == mv_stack[0].this_mv.row
            && combined_mvs[0][0].col == mv_stack[0].this_mv.col
            && combined_mvs[0][1].row == mv_stack[0].comp_mv.row
            && combined_mvs[0][1].col == mv_stack[0].comp_mv.col
          {
            CandidateMV {
              this_mv: combined_mvs[1][0],
              comp_mv: combined_mvs[1][1],
              weight: 2,
            }
          } else {
            CandidateMV {
              this_mv: combined_mvs[0][0],
              comp_mv: combined_mvs[0][1],
              weight: 2,
            }
          };
          mv_stack.push(mv_cand);
        } else {
          for idx in 0..2 {
            let mv_cand = CandidateMV {
              this_mv: combined_mvs[idx][0],
              comp_mv: combined_mvs[idx][1],
              weight: 2,
            };
            mv_stack.push(mv_cand);
          }
        }

        assert!(mv_stack.len() == 2);
      }
    }

    /* TODO: Handle single reference frame extension */

    let frame_bo = PlaneBlockOffset(BlockOffset {
      x: self.bc.blocks.x() + bo.0.x,
      y: self.bc.blocks.y() + bo.0.y,
    });
    // clamp mvs
    for mv in mv_stack {
      let blk_w = bsize.width();
      let blk_h = bsize.height();
      let border_w = 128 + blk_w as isize * 8;
      let border_h = 128 + blk_h as isize * 8;
      let mvx_min =
        -(frame_bo.0.x as isize) * (8 * MI_SIZE) as isize - border_w;
      let mvx_max = ((self.bc.blocks.frame_cols() - frame_bo.0.x) as isize
        - (blk_w / MI_SIZE) as isize)
        * (8 * MI_SIZE) as isize
        + border_w;
      let mvy_min =
        -(frame_bo.0.y as isize) * (8 * MI_SIZE) as isize - border_h;
      let mvy_max = ((self.bc.blocks.frame_rows() - frame_bo.0.y) as isize
        - (blk_h / MI_SIZE) as isize)
        * (8 * MI_SIZE) as isize
        + border_h;
      mv.this_mv.row =
        (mv.this_mv.row as isize).clamp(mvy_min, mvy_max) as i16;
      mv.this_mv.col =
        (mv.this_mv.col as isize).clamp(mvx_min, mvx_max) as i16;
      mv.comp_mv.row =
        (mv.comp_mv.row as isize).clamp(mvy_min, mvy_max) as i16;
      mv.comp_mv.col =
        (mv.comp_mv.col as isize).clamp(mvx_min, mvx_max) as i16;
    }

    mode_context
  }

  /// # Panics
  ///
  /// - If the first ref frame is not set (`NONE_FRAME`)
  pub fn find_mvrefs<T: Pixel>(
    &self, bo: TileBlockOffset, ref_frames: [RefType; 2],
    mv_stack: &mut ArrayVec<CandidateMV, 9>, bsize: BlockSize,
    fi: &FrameInvariants<T>, is_compound: bool,
  ) -> usize {
    assert!(ref_frames[0] != NONE_FRAME);
    if ref_frames[0] != NONE_FRAME {
      // TODO: If ref_frames[0] != INTRA_FRAME, convert global mv to an mv;
      // otherwise, set the global mv ref to invalid.
    }

    if ref_frames[0] != INTRA_FRAME {
      /* TODO: Set zeromv ref to the converted global motion vector */
    } else {
      /* TODO: Set the zeromv ref to 0 */
      return 0;
    }

    self.setup_mvref_list(bo, ref_frames, mv_stack, bsize, fi, is_compound)
  }

  pub fn fill_neighbours_ref_counts(&mut self, bo: TileBlockOffset) {
    let mut ref_counts = [0; INTER_REFS_PER_FRAME];

    if bo.0.y > 0 {
      let above_b = self.bc.blocks.above_of(bo);
      if above_b.is_inter() {
        ref_counts[above_b.ref_frames[0].to_index()] += 1;
        if above_b.has_second_ref() {
          ref_counts[above_b.ref_frames[1].to_index()] += 1;
        }
      }
    }

    if bo.0.x > 0 {
      let left_b = self.bc.blocks.left_of(bo);
      if left_b.is_inter() {
        ref_counts[left_b.ref_frames[0].to_index()] += 1;
        if left_b.has_second_ref() {
          ref_counts[left_b.ref_frames[1].to_index()] += 1;
        }
      }
    }
    self.bc.blocks[bo].neighbors_ref_counts = ref_counts;
  }

  #[inline]
  pub const fn ref_count_ctx(counts0: u8, counts1: u8) -> usize {
    if counts0 < counts1 {
      0
    } else if counts0 == counts1 {
      1
    } else {
      2
    }
  }

  #[inline]
  pub fn get_pred_ctx_brfarf2_or_arf(&self, bo: TileBlockOffset) -> usize {
    let ref_counts = self.bc.blocks[bo].neighbors_ref_counts;

    let brfarf2_count = ref_counts[BWDREF_FRAME.to_index()]
      + ref_counts[ALTREF2_FRAME.to_index()];
    let arf_count = ref_counts[ALTREF_FRAME.to_index()];

    ContextWriter::ref_count_ctx(brfarf2_count, arf_count)
  }

  #[inline]
  pub fn get_pred_ctx_ll2_or_l3gld(&self, bo: TileBlockOffset) -> usize {
    let ref_counts = self.bc.blocks[bo].neighbors_ref_counts;

    let l_l2_count =
      ref_counts[LAST_FRAME.to_index()] + ref_counts[LAST2_FRAME.to_index()];
    let l3_gold_count =
      ref_counts[LAST3_FRAME.to_index()] + ref_counts[GOLDEN_FRAME.to_index()];

    ContextWriter::ref_count_ctx(l_l2_count, l3_gold_count)
  }

  #[inline]
  pub fn get_pred_ctx_last_or_last2(&self, bo: TileBlockOffset) -> usize {
    let ref_counts = self.bc.blocks[bo].neighbors_ref_counts;

    let l_count = ref_counts[LAST_FRAME.to_index()];
    let l2_count = ref_counts[LAST2_FRAME.to_index()];

    ContextWriter::ref_count_ctx(l_count, l2_count)
  }

  #[inline]
  pub fn get_pred_ctx_last3_or_gold(&self, bo: TileBlockOffset) -> usize {
    let ref_counts = self.bc.blocks[bo].neighbors_ref_counts;

    let l3_count = ref_counts[LAST3_FRAME.to_index()];
    let gold_count = ref_counts[GOLDEN_FRAME.to_index()];

    ContextWriter::ref_count_ctx(l3_count, gold_count)
  }

  #[inline]
  pub fn get_pred_ctx_brf_or_arf2(&self, bo: TileBlockOffset) -> usize {
    let ref_counts = self.bc.blocks[bo].neighbors_ref_counts;

    let brf_count = ref_counts[BWDREF_FRAME.to_index()];
    let arf2_count = ref_counts[ALTREF2_FRAME.to_index()];

    ContextWriter::ref_count_ctx(brf_count, arf2_count)
  }

  pub fn get_comp_mode_ctx(&self, bo: TileBlockOffset) -> usize {
    let avail_left = bo.0.x > 0;
    let avail_up = bo.0.y > 0;
    let (left0, left1) = if avail_left {
      let bo_left = bo.with_offset(-1, 0);
      let ref_frames = &self.bc.blocks[bo_left].ref_frames;
      (ref_frames[0], ref_frames[1])
    } else {
      (INTRA_FRAME, NONE_FRAME)
    };
    let (above0, above1) = if avail_up {
      let bo_up = bo.with_offset(0, -1);
      let ref_frames = &self.bc.blocks[bo_up].ref_frames;
      (ref_frames[0], ref_frames[1])
    } else {
      (INTRA_FRAME, NONE_FRAME)
    };
    let left_single = left1 == NONE_FRAME;
    let above_single = above1 == NONE_FRAME;
    let left_intra = left0 == INTRA_FRAME;
    let above_intra = above0 == INTRA_FRAME;
    let left_backward = left0.is_bwd_ref();
    let above_backward = above0.is_bwd_ref();

    if avail_left && avail_up {
      if above_single && left_single {
        (above_backward ^ left_backward) as usize
      } else if above_single {
        2 + (above_backward || above_intra) as usize
      } else if left_single {
        2 + (left_backward || left_intra) as usize
      } else {
        4
      }
    } else if avail_up {
      if above_single {
        above_backward as usize
      } else {
        3
      }
    } else if avail_left {
      if left_single {
        left_backward as usize
      } else {
        3
      }
    } else {
      1
    }
  }

  pub fn get_comp_ref_type_ctx(&self, bo: TileBlockOffset) -> usize {
    fn is_samedir_ref_pair(ref0: RefType, ref1: RefType) -> bool {
      (ref0.is_bwd_ref() && ref0 != NONE_FRAME)
        == (ref1.is_bwd_ref() && ref1 != NONE_FRAME)
    }

    let avail_left = bo.0.x > 0;
    let avail_up = bo.0.y > 0;
    let (left0, left1) = if avail_left {
      let bo_left = bo.with_offset(-1, 0);
      let ref_frames = &self.bc.blocks[bo_left].ref_frames;
      (ref_frames[0], ref_frames[1])
    } else {
      (INTRA_FRAME, NONE_FRAME)
    };
    let (above0, above1) = if avail_up {
      let bo_up = bo.with_offset(0, -1);
      let ref_frames = &self.bc.blocks[bo_up].ref_frames;
      (ref_frames[0], ref_frames[1])
    } else {
      (INTRA_FRAME, NONE_FRAME)
    };
    let left_single = left1 == NONE_FRAME;
    let above_single = above1 == NONE_FRAME;
    let left_intra = left0 == INTRA_FRAME;
    let above_intra = above0 == INTRA_FRAME;
    let above_comp_inter = avail_up && !above_intra && !above_single;
    let left_comp_inter = avail_left && !left_intra && !left_single;
    let above_uni_comp =
      above_comp_inter && is_samedir_ref_pair(above0, above1);
    let left_uni_comp = left_comp_inter && is_samedir_ref_pair(left0, left1);

    if avail_up && !above_intra && avail_left && !left_intra {
      let samedir = is_samedir_ref_pair(above0, left0) as usize;

      if !above_comp_inter && !left_comp_inter {
        1 + 2 * samedir
      } else if !above_comp_inter {
        if !left_uni_comp {
          1
        } else {
          3 + samedir
        }
      } else if !left_comp_inter {
        if !above_uni_comp {
          1
        } else {
          3 + samedir
        }
      } else if !above_uni_comp && !left_uni_comp {
        0
      } else if !above_uni_comp || !left_uni_comp {
        2
      } else {
        3 + ((above0 == BWDREF_FRAME) == (left0 == BWDREF_FRAME)) as usize
      }
    } else if avail_up && avail_left {
      if above_comp_inter {
        1 + 2 * above_uni_comp as usize
      } else if left_comp_inter {
        1 + 2 * left_uni_comp as usize
      } else {
        2
      }
    } else if above_comp_inter {
      4 * above_uni_comp as usize
    } else if left_comp_inter {
      4 * left_uni_comp as usize
    } else {
      2
    }
  }

  /// # Panics
  ///
  /// - If `mode` is not an inter mode
  pub fn write_compound_mode<W: Writer>(
    &mut self, w: &mut W, mode: PredictionMode, ctx: usize,
  ) {
    let newmv_ctx = ctx & NEWMV_CTX_MASK;
    let refmv_ctx = (ctx >> REFMV_OFFSET) & REFMV_CTX_MASK;

    let ctx = if refmv_ctx < 2 {
      newmv_ctx.min(1)
    } else if refmv_ctx < 4 {
      (newmv_ctx + 1).min(4)
    } else {
      (newmv_ctx.max(1) + 3).min(7)
    };

    assert!(mode >= PredictionMode::NEAREST_NEARESTMV);
    let val = match mode {
      PredictionMode::NEAREST_NEARESTMV => 0,
      PredictionMode::NEAR_NEAR0MV
      | PredictionMode::NEAR_NEAR1MV
      | PredictionMode::NEAR_NEAR2MV => 1,
      PredictionMode::NEAREST_NEWMV => 2,
      PredictionMode::NEW_NEARESTMV => 3,
      PredictionMode::NEAR_NEW0MV
      | PredictionMode::NEAR_NEW1MV
      | PredictionMode::NEAR_NEW2MV => 4,
      PredictionMode::NEW_NEAR0MV
      | PredictionMode::NEW_NEAR1MV
      | PredictionMode::NEW_NEAR2MV => 5,
      PredictionMode::GLOBAL_GLOBALMV => 6,
      PredictionMode::NEW_NEWMV => 7,
      _ => unreachable!(),
    };
    symbol_with_update!(self, w, val, &self.fc.compound_mode_cdf[ctx]);
  }

  pub fn write_inter_mode<W: Writer>(
    &mut self, w: &mut W, mode: PredictionMode, ctx: usize,
  ) {
    use PredictionMode::{GLOBALMV, NEARESTMV, NEWMV};
    let newmv_ctx = ctx & NEWMV_CTX_MASK;
    let cdf = &self.fc.newmv_cdf[newmv_ctx];
    symbol_with_update!(self, w, (mode != NEWMV) as u32, cdf);
    if mode != NEWMV {
      let zeromv_ctx = (ctx >> GLOBALMV_OFFSET) & GLOBALMV_CTX_MASK;
      let cdf = &self.fc.zeromv_cdf[zeromv_ctx];
      symbol_with_update!(self, w, (mode != GLOBALMV) as u32, cdf);
      if mode != GLOBALMV {
        let refmv_ctx = (ctx >> REFMV_OFFSET) & REFMV_CTX_MASK;
        let cdf = &self.fc.refmv_cdf[refmv_ctx];
        symbol_with_update!(self, w, (mode != NEARESTMV) as u32, cdf);
      }
    }
  }

  #[inline]
  pub fn write_drl_mode<W: Writer>(
    &mut self, w: &mut W, drl_mode: bool, ctx: usize,
  ) {
    let cdf = &self.fc.drl_cdfs[ctx];
    symbol_with_update!(self, w, drl_mode as u32, cdf);
  }

  /// # Panics
  ///
  /// - If the MV is invalid
  pub fn write_mv<W: Writer>(
    &mut self, w: &mut W, mv: MotionVector, ref_mv: MotionVector,
    mv_precision: MvSubpelPrecision,
  ) {
    // <https://aomediacodec.github.io/av1-spec/#assign-mv-semantics>
    assert!(mv.is_valid());

    let diff =
      MotionVector { row: mv.row - ref_mv.row, col: mv.col - ref_mv.col };
    let j: MvJointType = av1_get_mv_joint(diff);

    let cdf = &self.fc.nmv_context.joints_cdf;
    symbol_with_update!(self, w, j as u32, cdf);

    if mv_joint_vertical(j) {
      self.encode_mv_component(w, diff.row as i32, 0, mv_precision);
    }
    if mv_joint_horizontal(j) {
      self.encode_mv_component(w, diff.col as i32, 1, mv_precision);
    }
  }

  pub fn write_block_deblock_deltas<W: Writer>(
    &mut self, w: &mut W, bo: TileBlockOffset, multi: bool, planes: usize,
  ) {
    let block = &self.bc.blocks[bo];
    let deltas_count = if multi { FRAME_LF_COUNT + planes - 3 } else { 1 };
    let deltas = &block.deblock_deltas[..deltas_count];

    for (i, &delta) in deltas.iter().enumerate() {
      let abs = delta.unsigned_abs() as u32;
      let cdf = if multi {
        &self.fc.deblock_delta_multi_cdf[i]
      } else {
        &self.fc.deblock_delta_cdf
      };

      symbol_with_update!(self, w, cmp::min(abs, DELTA_LF_SMALL), cdf);

      if abs >= DELTA_LF_SMALL {
        let bits = msb(abs as i32 - 1) as u32;
        w.literal(3, bits - 1);
        w.literal(bits as u8, abs - (1 << bits) - 1);
      }
      if abs > 0 {
        w.bool(delta < 0, 16384);
      }
    }
  }

  pub fn write_is_inter<W: Writer>(
    &mut self, w: &mut W, bo: TileBlockOffset, is_inter: bool,
  ) {
    let ctx = self.bc.intra_inter_context(bo);
    let cdf = &self.fc.intra_inter_cdfs[ctx];
    symbol_with_update!(self, w, is_inter as u32, cdf);
  }

  pub fn write_coeffs_lv_map<T: Coefficient, W: Writer>(
    &mut self, w: &mut W, plane: usize, bo: TileBlockOffset, coeffs_in: &[T],
    eob: u16, pred_mode: PredictionMode, tx_size: TxSize, tx_type: TxType,
    plane_bsize: BlockSize, xdec: usize, ydec: usize,
    use_reduced_tx_set: bool, frame_clipped_txw: usize,
    frame_clipped_txh: usize,
  ) -> bool {
    debug_assert!(frame_clipped_txw != 0);
    debug_assert!(frame_clipped_txh != 0);

    let is_inter = pred_mode >= PredictionMode::NEARESTMV;

    // Note: Both intra and inter mode uses inter scan order. Surprised?
    let scan: &[u16] = &av1_scan_orders[tx_size as usize][tx_type as usize]
      .scan[..usize::from(eob)];
    let height = av1_get_coded_tx_size(tx_size).height();

    // Create a slice with coeffs in scan order
    let mut coeffs_storage: Aligned<ArrayVec<T, { 32 * 32 }>> =
      Aligned::new(ArrayVec::new());
    let coeffs = &mut coeffs_storage.data;
    coeffs.extend(scan.iter().map(|&scan_idx| coeffs_in[scan_idx as usize]));

    let cul_level: u32 = coeffs.iter().map(|c| u32::cast_from(c.abs())).sum();

    let txs_ctx = Self::get_txsize_entropy_ctx(tx_size);
    let txb_ctx = self.bc.get_txb_ctx(
      plane_bsize,
      tx_size,
      plane,
      bo,
      xdec,
      ydec,
      frame_clipped_txw,
      frame_clipped_txh,
    );

    {
      let cdf = &self.fc.txb_skip_cdf[txs_ctx][txb_ctx.txb_skip_ctx];
      symbol_with_update!(self, w, (eob == 0) as u32, cdf);
    }

    if eob == 0 {
      self.bc.set_coeff_context(plane, bo, tx_size, xdec, ydec, 0);
      return false;
    }

    let mut levels_buf = [0u8; TX_PAD_2D];
    let levels: &mut [u8] =
      &mut levels_buf[TX_PAD_TOP * (height + TX_PAD_HOR)..];

    self.txb_init_levels(coeffs_in, height, levels, height + TX_PAD_HOR);

    let tx_class = tx_type_to_class[tx_type as usize];
    let plane_type = usize::from(plane != 0);

    // Signal tx_type for luma plane only
    if plane == 0 {
      self.write_tx_type(
        w,
        tx_size,
        tx_type,
        pred_mode,
        is_inter,
        use_reduced_tx_set,
      );
    }

    self.encode_eob(eob, tx_size, tx_class, txs_ctx, plane_type, w);
    self.encode_coeffs(
      coeffs, levels, scan, eob, tx_size, tx_class, txs_ctx, plane_type, w,
    );
    let cul_level =
      self.encode_coeff_signs(coeffs, w, plane_type, txb_ctx, cul_level);
    self.bc.set_coeff_context(plane, bo, tx_size, xdec, ydec, cul_level as u8);
    true
  }

  fn encode_eob<W: Writer>(
    &mut self, eob: u16, tx_size: TxSize, tx_class: TxClass, txs_ctx: usize,
    plane_type: usize, w: &mut W,
  ) {
    let (eob_pt, eob_extra) = Self::get_eob_pos_token(eob);
    let eob_multi_size: usize = tx_size.area_log2() - 4;
    let eob_multi_ctx: usize = usize::from(tx_class != TX_CLASS_2D);

    match eob_multi_size {
      0 => {
        let cdf = &self.fc.eob_flag_cdf16[plane_type][eob_multi_ctx];
        symbol_with_update!(self, w, eob_pt - 1, cdf);
      }
      1 => {
        let cdf = &self.fc.eob_flag_cdf32[plane_type][eob_multi_ctx];
        symbol_with_update!(self, w, eob_pt - 1, cdf);
      }
      2 => {
        let cdf = &self.fc.eob_flag_cdf64[plane_type][eob_multi_ctx];
        symbol_with_update!(self, w, eob_pt - 1, cdf);
      }
      3 => {
        let cdf = &self.fc.eob_flag_cdf128[plane_type][eob_multi_ctx];
        symbol_with_update!(self, w, eob_pt - 1, cdf);
      }
      4 => {
        let cdf = &self.fc.eob_flag_cdf256[plane_type][eob_multi_ctx];
        symbol_with_update!(self, w, eob_pt - 1, cdf);
      }
      5 => {
        let cdf = &self.fc.eob_flag_cdf512[plane_type][eob_multi_ctx];
        symbol_with_update!(self, w, eob_pt - 1, cdf);
      }
      _ => {
        let cdf = &self.fc.eob_flag_cdf1024[plane_type][eob_multi_ctx];
        symbol_with_update!(self, w, eob_pt - 1, cdf);
      }
    }

    let eob_offset_bits = k_eob_offset_bits[eob_pt as usize];

    if eob_offset_bits > 0 {
      let mut eob_shift = eob_offset_bits - 1;
      let mut bit: u32 = u32::from((eob_extra & (1 << eob_shift)) != 0);
      let cdf =
        &self.fc.eob_extra_cdf[txs_ctx][plane_type][(eob_pt - 3) as usize];
      symbol_with_update!(self, w, bit, cdf);
      for i in 1..eob_offset_bits {
        eob_shift = eob_offset_bits - 1 - i;
        bit = u32::from((eob_extra & (1 << eob_shift)) != 0);
        w.bit(bit as u16);
      }
    }
  }

  fn encode_coeffs<T: Coefficient, W: Writer>(
    &mut self, coeffs: &[T], levels: &mut [u8], scan: &[u16], eob: u16,
    tx_size: TxSize, tx_class: TxClass, txs_ctx: usize, plane_type: usize,
    w: &mut W,
  ) {
    let mut coeff_contexts =
      Aligned::<[MaybeUninit<i8>; MAX_CODED_TX_SQUARE]>::uninit_array();

    // get_nz_map_contexts sets coeff_contexts contiguously as a parallel array for scan, not in scan order
    let coeff_contexts = self.get_nz_map_contexts(
      levels,
      scan,
      eob,
      tx_size,
      tx_class,
      &mut coeff_contexts.data,
    );

    let bhl = Self::get_txb_bhl(tx_size);

    let scan_with_ctx =
      scan.iter().copied().zip(coeff_contexts.iter().copied());
    for (c, ((pos, coeff_ctx), v)) in
      scan_with_ctx.zip(coeffs.iter().copied()).enumerate().rev()
    {
      let pos = pos as usize;
      let coeff_ctx = coeff_ctx as usize;
      let level = v.abs();

      if c == usize::from(eob) - 1 {
        symbol_with_update!(
          self,
          w,
          cmp::min(u32::cast_from(level), 3) - 1,
          &self.fc.coeff_base_eob_cdf[txs_ctx][plane_type][coeff_ctx]
        );
      } else {
        symbol_with_update!(
          self,
          w,
          cmp::min(u32::cast_from(level), 3),
          &self.fc.coeff_base_cdf[txs_ctx][plane_type][coeff_ctx]
        );
      }

      if level > T::cast_from(NUM_BASE_LEVELS) {
        let base_range = level - T::cast_from(1 + NUM_BASE_LEVELS);
        let br_ctx = Self::get_br_ctx(levels, pos, bhl, tx_class);
        let mut idx: T = T::cast_from(0);

        loop {
          if idx >= T::cast_from(COEFF_BASE_RANGE) {
            break;
          }
          let k = cmp::min(base_range - idx, T::cast_from(BR_CDF_SIZE - 1));
          let cdf = &self.fc.coeff_br_cdf
            [txs_ctx.min(TxSize::TX_32X32 as usize)][plane_type][br_ctx];
          symbol_with_update!(self, w, u32::cast_from(k), cdf);
          if k < T::cast_from(BR_CDF_SIZE - 1) {
            break;
          }
          idx += T::cast_from(BR_CDF_SIZE - 1);
        }
      }
    }
  }

  fn encode_coeff_signs<T: Coefficient, W: Writer>(
    &mut self, coeffs: &[T], w: &mut W, plane_type: usize, txb_ctx: TXB_CTX,
    orig_cul_level: u32,
  ) -> u32 {
    // Loop to code all signs in the transform block,
    // starting with the sign of DC (if applicable)
    for (c, &v) in coeffs.iter().enumerate() {
      if v == T::cast_from(0) {
        continue;
      }

      let level = v.abs();
      let sign = u32::from(v < T::cast_from(0));
      if c == 0 {
        let cdf = &self.fc.dc_sign_cdf[plane_type][txb_ctx.dc_sign_ctx];
        symbol_with_update!(self, w, sign, cdf);
      } else {
        w.bit(sign as u16);
      }
      // save extra golomb codes for separate loop
      if level > T::cast_from(COEFF_BASE_RANGE + NUM_BASE_LEVELS) {
        w.write_golomb(u32::cast_from(
          level - T::cast_from(COEFF_BASE_RANGE + NUM_BASE_LEVELS + 1),
        ));
      }
    }

    let mut new_cul_level =
      cmp::min(COEFF_CONTEXT_MASK as u32, orig_cul_level);

    BlockContext::set_dc_sign(&mut new_cul_level, i32::cast_from(coeffs[0]));

    new_cul_level
  }
}
