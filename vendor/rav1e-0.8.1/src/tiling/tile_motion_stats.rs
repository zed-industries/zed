// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::mc::MotionVector;
use crate::me::*;

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice;

/// Tiled view of `FrameMEStats`
#[derive(Debug)]
pub struct TileMEStats<'a> {
  data: *const MEStats,
  // expressed in mi blocks
  // private to guarantee borrowing rules
  x: usize,
  y: usize,
  cols: usize,
  rows: usize,
  /// number of cols in the underlying `FrameMEStats`
  stride: usize,
  phantom: PhantomData<&'a MotionVector>,
}

/// Mutable tiled view of `FrameMEStats`
#[derive(Debug)]
pub struct TileMEStatsMut<'a> {
  data: *mut MEStats,
  // expressed in mi blocks
  // private to guarantee borrowing rules
  x: usize,
  y: usize,
  cols: usize,
  rows: usize,
  /// number of cols in the underlying `FrameMEStats`
  stride: usize,
  phantom: PhantomData<&'a mut MotionVector>,
}

// common impl for TileMotionVectors and TileMotionVectorsMut
macro_rules! tile_me_stats_common {
  // $name: TileMEStats or TileMEStatsMut
  // $opt_mut: nothing or mut
  ($name:ident $(,$opt_mut:tt)?) => {
    impl<'a> $name<'a> {

      /// # Panics
      ///
      /// - If the requested dimensions are larger than the frame MV size
      #[inline(always)]
      pub fn new(
        frame_mvs: &'a $($opt_mut)? FrameMEStats,
        x: usize,
        y: usize,
        cols: usize,
        rows: usize,
      ) -> Self {
        assert!(x + cols <= frame_mvs.cols);
        assert!(y + rows <= frame_mvs.rows);
        Self {
          data: & $($opt_mut)? frame_mvs[y][x],
          x,
          y,
          cols,
          rows,
          stride: frame_mvs.cols,
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
    }

    unsafe impl Send for $name<'_> {}
    unsafe impl Sync for $name<'_> {}

    impl Index<usize> for $name<'_> {
      type Output = [MEStats];

      #[inline(always)]
      fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rows);
        // SAFETY: The above assert ensures we do not access OOB data.
        unsafe {
          let ptr = self.data.add(index * self.stride);
          slice::from_raw_parts(ptr, self.cols)
        }
      }
    }
  }
}

tile_me_stats_common!(TileMEStats);
tile_me_stats_common!(TileMEStatsMut, mut);

impl TileMEStatsMut<'_> {
  #[inline(always)]
  pub const fn as_const(&self) -> TileMEStats<'_> {
    TileMEStats {
      data: self.data,
      x: self.x,
      y: self.y,
      cols: self.cols,
      rows: self.rows,
      stride: self.stride,
      phantom: PhantomData,
    }
  }
}

impl IndexMut<usize> for TileMEStatsMut<'_> {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    assert!(index < self.rows);
    // SAFETY: The above assert ensures we do not access OOB data.
    unsafe {
      let ptr = self.data.add(index * self.stride);
      slice::from_raw_parts_mut(ptr, self.cols)
    }
  }
}
