// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::context::*;
use crate::mc::MotionVector;
use crate::partition::*;
use crate::predict::PredictionMode;
use crate::transform::*;

use std::cmp;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice;

/// Tiled view of `FrameBlocks`
#[derive(Debug)]
pub struct TileBlocks<'a> {
  data: *const Block,
  x: usize,
  y: usize,
  cols: usize,
  rows: usize,
  frame_cols: usize,
  frame_rows: usize,
  phantom: PhantomData<&'a Block>,
}

/// Mutable tiled view of `FrameBlocks`
#[derive(Debug)]
pub struct TileBlocksMut<'a> {
  data: *mut Block,
  // private to guarantee borrowing rules
  x: usize,
  y: usize,
  cols: usize,
  rows: usize,
  frame_cols: usize,
  frame_rows: usize,
  phantom: PhantomData<&'a mut Block>,
}

// common impl for TileBlocks and TileBlocksMut
macro_rules! tile_blocks_common {
  // $name: TileBlocks or TileBlocksMut
  // $opt_mut: nothing or mut
  ($name:ident $(,$opt_mut:tt)?) => {
    impl<'a> $name<'a> {

      #[inline(always)]
      pub fn new(
        frame_blocks: &'a $($opt_mut)? FrameBlocks,
        x: usize,
        y: usize,
        cols: usize,
        rows: usize,
      ) -> Self {
        Self {
          data: & $($opt_mut)? frame_blocks[y][x],
          x,
          y,
          cols,
          rows,
          frame_cols: frame_blocks.cols,
          frame_rows: frame_blocks.rows,
          phantom: PhantomData,
        }
      }

      pub fn subregion(
        &mut self,
        x: usize,
        y: usize,
        cols: usize,
        rows: usize,
      ) -> TileBlocks<'_> {
        TileBlocks {
          data: &self[y][x],
          x: self.x+x,
          y: self.y+y,
          cols: cmp::min(cols, self.cols - x),
          rows: cmp::min(rows, self.rows - y),
          frame_cols: self.frame_cols,
          frame_rows: self.frame_rows,
          phantom: PhantomData,
        }
      }

      #[inline(always)]
      pub const fn x(&self) -> usize {
        self.x
      }

      #[inline(always)]
      pub const fn y(&self) -> usize {
        self.y
      }

      #[inline(always)]
      pub const fn cols(&self) -> usize {
        self.cols
      }

      #[inline(always)]
      pub const fn rows(&self) -> usize {
        self.rows
      }

      #[inline(always)]
      pub const fn frame_cols(&self) -> usize {
        self.frame_cols
      }

      #[inline(always)]
      pub const fn frame_rows(&self) -> usize {
        self.frame_rows
      }

      #[inline(always)]
      pub fn above_of(&self, bo: TileBlockOffset) -> &Block {
        &self[bo.0.y - 1][bo.0.x]
      }

      #[inline(always)]
      pub fn left_of(&self, bo: TileBlockOffset) -> &Block {
        &self[bo.0.y][bo.0.x - 1]
      }

      #[inline(always)]
      pub fn above_left_of(&self, bo: TileBlockOffset) -> &Block {
        &self[bo.0.y - 1][bo.0.x - 1]
      }

      pub fn get_cdef(&self, sbo: TileSuperBlockOffset) -> u8 {
        let bo = sbo.block_offset(0, 0).0;
        self[bo.y][bo.x].cdef_index
      }
    }

    unsafe impl Send for $name<'_> {}
    unsafe impl Sync for $name<'_> {}

    impl Index<usize> for $name<'_> {
      type Output = [Block];
      #[inline(always)]
      fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rows);
        // SAFETY: The above assert ensures we do not access OOB data.
        unsafe {
          let ptr = self.data.add(index * self.frame_cols);
          slice::from_raw_parts(ptr, self.cols)
        }
      }
    }

    // for convenience, also index by TileBlockOffset
    impl Index<TileBlockOffset> for $name<'_> {
      type Output = Block;
      #[inline(always)]
      fn index(&self, bo: TileBlockOffset) -> &Self::Output {
        &self[bo.0.y][bo.0.x]
      }
    }
  }
}

tile_blocks_common!(TileBlocks);
tile_blocks_common!(TileBlocksMut, mut);

impl TileBlocksMut<'_> {
  #[inline(always)]
  pub const fn as_const(&self) -> TileBlocks<'_> {
    TileBlocks {
      data: self.data,
      x: self.x,
      y: self.y,
      cols: self.cols,
      rows: self.rows,
      frame_cols: self.frame_cols,
      frame_rows: self.frame_rows,
      phantom: PhantomData,
    }
  }

  pub fn subregion_mut(
    &mut self, x: usize, y: usize, cols: usize, rows: usize,
  ) -> TileBlocksMut<'_> {
    TileBlocksMut {
      data: &mut self[y][x],
      x: self.x + x,
      y: self.y + y,
      cols: cmp::min(cols, self.cols - x),
      rows: cmp::min(rows, self.rows - y),
      frame_cols: self.frame_cols,
      frame_rows: self.frame_rows,
      phantom: PhantomData,
    }
  }

  #[inline(always)]
  pub fn for_each<F>(&mut self, bo: TileBlockOffset, bsize: BlockSize, f: F)
  where
    F: Fn(&mut Block),
  {
    let mut bw = bsize.width_mi();
    let bh = bsize.height_mi();

    if bo.0.x + bw >= self.cols {
      bw = self.cols - bo.0.x;
    }
    for y in 0..bh {
      if bo.0.y + y >= self.rows {
        continue;
      }
      for block in self[bo.0.y + y][bo.0.x..bo.0.x + bw].iter_mut() {
        f(block);
      }
    }
  }

  #[inline(always)]
  pub fn set_mode(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, mode: PredictionMode,
  ) {
    self.for_each(bo, bsize, |block| block.mode = mode);
  }

  #[inline(always)]
  pub fn set_block_size(&mut self, bo: TileBlockOffset, bsize: BlockSize) {
    let n4_w = bsize.width_mi() as u8;
    let n4_h = bsize.height_mi() as u8;
    self.for_each(bo, bsize, |block| {
      block.bsize = bsize;
      block.n4_w = n4_w;
      block.n4_h = n4_h;
    });
  }

  #[inline(always)]
  pub fn set_tx_size(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, tx_size: TxSize,
  ) {
    self.for_each(bo, bsize, |block| block.txsize = tx_size);
  }

  #[inline(always)]
  pub fn set_skip(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, skip: bool,
  ) {
    self.for_each(bo, bsize, |block| block.skip = skip);
  }

  #[inline(always)]
  pub fn set_segmentation_idx(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, idx: u8,
  ) {
    self.for_each(bo, bsize, |block| block.segmentation_idx = idx);
  }

  #[inline(always)]
  pub fn set_ref_frames(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, r: [RefType; 2],
  ) {
    self.for_each(bo, bsize, |block| block.ref_frames = r);
  }

  #[inline(always)]
  pub fn set_motion_vectors(
    &mut self, bo: TileBlockOffset, bsize: BlockSize, mvs: [MotionVector; 2],
  ) {
    self.for_each(bo, bsize, |block| block.mv = mvs);
  }

  #[inline(always)]
  pub fn set_cdef(&mut self, sbo: TileSuperBlockOffset, cdef_index: u8) {
    let bo = sbo.block_offset(0, 0).0;
    // Checkme: Is 16 still the right block unit for 128x128 superblocks?
    let bw = cmp::min(bo.x + MIB_SIZE, self.cols);
    let bh = cmp::min(bo.y + MIB_SIZE, self.rows);
    for y in bo.y..bh {
      for x in bo.x..bw {
        self[y][x].cdef_index = cdef_index;
      }
    }
  }
}

impl IndexMut<usize> for TileBlocksMut<'_> {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    assert!(index < self.rows);
    // SAFETY: The above assert ensures we do not access OOB data.
    unsafe {
      let ptr = self.data.add(index * self.frame_cols);
      slice::from_raw_parts_mut(ptr, self.cols)
    }
  }
}

impl IndexMut<TileBlockOffset> for TileBlocksMut<'_> {
  #[inline(always)]
  fn index_mut(&mut self, bo: TileBlockOffset) -> &mut Self::Output {
    &mut self[bo.0.y][bo.0.x]
  }
}
