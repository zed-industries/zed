// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use self::BlockSize::*;
use self::TxSize::*;
use crate::context::*;
use crate::frame::*;
use crate::predict::*;
use crate::recon_intra::*;
use crate::serialize::{Deserialize, Serialize};
use crate::tiling::*;
use crate::transform::TxSize;
use crate::util::*;
use thiserror::Error;

use std::mem::transmute;
use std::mem::MaybeUninit;

// LAST_FRAME through ALTREF_FRAME correspond to slots 0-6.
#[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug)]
pub enum RefType {
  INTRA_FRAME = 0,
  LAST_FRAME = 1,
  LAST2_FRAME = 2,
  LAST3_FRAME = 3,
  GOLDEN_FRAME = 4,
  BWDREF_FRAME = 5,
  ALTREF2_FRAME = 6,
  ALTREF_FRAME = 7,
  NONE_FRAME = 8,
}

impl RefType {
  /// convert to a ref list index, 0-6 (`INTER_REFS_PER_FRAME`)
  ///
  /// # Panics
  ///
  /// - If the ref type is a None or Intra frame
  #[inline]
  pub fn to_index(self) -> usize {
    match self {
      NONE_FRAME => {
        panic!("Tried to get slot of NONE_FRAME");
      }
      INTRA_FRAME => {
        panic!("Tried to get slot of INTRA_FRAME");
      }
      _ => (self as usize) - 1,
    }
  }
  #[inline]
  pub const fn is_fwd_ref(self) -> bool {
    (self as usize) < 5
  }
  #[inline]
  pub const fn is_bwd_ref(self) -> bool {
    (self as usize) >= 5
  }
}

use self::RefType::*;
use std::fmt;
use std::fmt::Display;

pub const ALL_INTER_REFS: [RefType; 7] = [
  LAST_FRAME,
  LAST2_FRAME,
  LAST3_FRAME,
  GOLDEN_FRAME,
  BWDREF_FRAME,
  ALTREF2_FRAME,
  ALTREF_FRAME,
];

pub const LAST_LAST2_FRAMES: usize = 0; // { LAST_FRAME, LAST2_FRAME }
pub const LAST_LAST3_FRAMES: usize = 1; // { LAST_FRAME, LAST3_FRAME }
pub const LAST_GOLDEN_FRAMES: usize = 2; // { LAST_FRAME, GOLDEN_FRAME }
pub const BWDREF_ALTREF_FRAMES: usize = 3; // { BWDREF_FRAME, ALTREF_FRAME }
pub const LAST2_LAST3_FRAMES: usize = 4; // { LAST2_FRAME, LAST3_FRAME }
pub const LAST2_GOLDEN_FRAMES: usize = 5; // { LAST2_FRAME, GOLDEN_FRAME }
pub const LAST3_GOLDEN_FRAMES: usize = 6; // { LAST3_FRAME, GOLDEN_FRAME }
pub const BWDREF_ALTREF2_FRAMES: usize = 7; // { BWDREF_FRAME, ALTREF2_FRAME }
pub const ALTREF2_ALTREF_FRAMES: usize = 8; // { ALTREF2_FRAME, ALTREF_FRAME }
pub const TOTAL_UNIDIR_COMP_REFS: usize = 9;

// NOTE: UNIDIR_COMP_REFS is the number of uni-directional reference pairs
//       that are explicitly signaled.
pub const UNIDIR_COMP_REFS: usize = BWDREF_ALTREF_FRAMES + 1;

pub const FWD_REFS: usize = 4;
pub const BWD_REFS: usize = 3;
pub const SINGLE_REFS: usize = 7;
pub const TOTAL_REFS_PER_FRAME: usize = 8;
pub const INTER_REFS_PER_FRAME: usize = 7;
pub const TOTAL_COMP_REFS: usize =
  FWD_REFS * BWD_REFS + TOTAL_UNIDIR_COMP_REFS;

pub const REF_FRAMES_LOG2: usize = 3;
pub const REF_FRAMES: usize = 1 << REF_FRAMES_LOG2;

pub const REF_CONTEXTS: usize = 3;
pub const MVREF_ROW_COLS: usize = 3;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
pub enum PartitionType {
  PARTITION_NONE,
  PARTITION_HORZ,
  PARTITION_VERT,
  PARTITION_SPLIT,
  PARTITION_HORZ_A, // HORZ split and the top partition is split again
  PARTITION_HORZ_B, // HORZ split and the bottom partition is split again
  PARTITION_VERT_A, // VERT split and the left partition is split again
  PARTITION_VERT_B, // VERT split and the right partition is split again
  PARTITION_HORZ_4, // 4:1 horizontal partition
  PARTITION_VERT_4, // 4:1 vertical partition
  PARTITION_INVALID,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub enum BlockSize {
  BLOCK_4X4,
  BLOCK_4X8,
  BLOCK_8X4,
  BLOCK_8X8,
  BLOCK_8X16,
  BLOCK_16X8,
  BLOCK_16X16,
  BLOCK_16X32,
  BLOCK_32X16,
  BLOCK_32X32,
  BLOCK_32X64,
  BLOCK_64X32,
  #[cfg_attr(test, default)]
  BLOCK_64X64,
  BLOCK_64X128,
  BLOCK_128X64,
  BLOCK_128X128,
  BLOCK_4X16,
  BLOCK_16X4,
  BLOCK_8X32,
  BLOCK_32X8,
  BLOCK_16X64,
  BLOCK_64X16,
}

#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub struct InvalidBlockSize;

impl Display for InvalidBlockSize {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("invalid block size")
  }
}

impl PartialOrd for BlockSize {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    use std::cmp::Ordering::{Equal, Greater, Less};
    match (
      self.width().cmp(&other.width()),
      self.height().cmp(&other.height()),
    ) {
      (Greater, Less) | (Less, Greater) => None,
      (Equal, Equal) => Some(Equal),
      (Greater, _) | (_, Greater) => Some(Greater),
      (Less, _) | (_, Less) => Some(Less),
    }
  }
}

impl BlockSize {
  pub const BLOCK_SIZES_ALL: usize = 22;
  pub const BLOCK_SIZES: usize = BlockSize::BLOCK_SIZES_ALL - 6; // BLOCK_SIZES_ALL minus 4:1 non-squares, six of them

  #[inline]
  /// # Errors
  ///
  /// - Returns `InvalidBlockSize` if the given `w` and `h` do not produce
  ///   a valid block size.
  pub fn from_width_and_height_opt(
    w: usize, h: usize,
  ) -> Result<BlockSize, InvalidBlockSize> {
    match (w, h) {
      (4, 4) => Ok(BLOCK_4X4),
      (4, 8) => Ok(BLOCK_4X8),
      (4, 16) => Ok(BLOCK_4X16),
      (8, 4) => Ok(BLOCK_8X4),
      (8, 8) => Ok(BLOCK_8X8),
      (8, 16) => Ok(BLOCK_8X16),
      (8, 32) => Ok(BLOCK_8X32),
      (16, 4) => Ok(BLOCK_16X4),
      (16, 8) => Ok(BLOCK_16X8),
      (16, 16) => Ok(BLOCK_16X16),
      (16, 32) => Ok(BLOCK_16X32),
      (16, 64) => Ok(BLOCK_16X64),
      (32, 8) => Ok(BLOCK_32X8),
      (32, 16) => Ok(BLOCK_32X16),
      (32, 32) => Ok(BLOCK_32X32),
      (32, 64) => Ok(BLOCK_32X64),
      (64, 16) => Ok(BLOCK_64X16),
      (64, 32) => Ok(BLOCK_64X32),
      (64, 64) => Ok(BLOCK_64X64),
      (64, 128) => Ok(BLOCK_64X128),
      (128, 64) => Ok(BLOCK_128X64),
      (128, 128) => Ok(BLOCK_128X128),
      _ => Err(InvalidBlockSize),
    }
  }

  /// # Panics
  ///
  /// - If the given `w` and `h` do not produce a valid block size.
  pub fn from_width_and_height(w: usize, h: usize) -> BlockSize {
    Self::from_width_and_height_opt(w, h).unwrap()
  }

  #[inline]
  pub fn cfl_allowed(self) -> bool {
    // TODO: fix me when enabling EXT_PARTITION_TYPES
    self <= BlockSize::BLOCK_32X32
  }

  #[inline]
  pub const fn width(self) -> usize {
    1 << self.width_log2()
  }

  /// width * height
  #[inline]
  pub const fn area(self) -> usize {
    self.width() * self.height()
  }

  #[inline]
  pub const fn width_log2(self) -> usize {
    match self {
      BLOCK_4X4 | BLOCK_4X8 | BLOCK_4X16 => 2,
      BLOCK_8X4 | BLOCK_8X8 | BLOCK_8X16 | BLOCK_8X32 => 3,
      BLOCK_16X4 | BLOCK_16X8 | BLOCK_16X16 | BLOCK_16X32 | BLOCK_16X64 => 4,
      BLOCK_32X8 | BLOCK_32X16 | BLOCK_32X32 | BLOCK_32X64 => 5,
      BLOCK_64X16 | BLOCK_64X32 | BLOCK_64X64 | BLOCK_64X128 => 6,
      BLOCK_128X64 | BLOCK_128X128 => 7,
    }
  }

  #[inline]
  pub const fn width_mi_log2(self) -> usize {
    self.width_log2() - 2
  }

  #[inline]
  pub const fn width_mi(self) -> usize {
    self.width() >> MI_SIZE_LOG2
  }

  #[inline]
  pub fn width_imp_b(self) -> usize {
    (self.width() >> (IMPORTANCE_BLOCK_TO_BLOCK_SHIFT + BLOCK_TO_PLANE_SHIFT))
      .max(1)
  }

  #[inline]
  pub const fn height(self) -> usize {
    1 << self.height_log2()
  }

  #[inline]
  pub const fn height_log2(self) -> usize {
    match self {
      BLOCK_4X4 | BLOCK_8X4 | BLOCK_16X4 => 2,
      BLOCK_4X8 | BLOCK_8X8 | BLOCK_16X8 | BLOCK_32X8 => 3,
      BLOCK_4X16 | BLOCK_8X16 | BLOCK_16X16 | BLOCK_32X16 | BLOCK_64X16 => 4,
      BLOCK_8X32 | BLOCK_16X32 | BLOCK_32X32 | BLOCK_64X32 => 5,
      BLOCK_16X64 | BLOCK_32X64 | BLOCK_64X64 | BLOCK_128X64 => 6,
      BLOCK_64X128 | BLOCK_128X128 => 7,
    }
  }

  #[inline]
  pub const fn height_mi_log2(self) -> usize {
    self.height_log2() - 2
  }

  #[inline]
  pub const fn height_mi(self) -> usize {
    self.height() >> MI_SIZE_LOG2
  }

  #[inline]
  pub fn height_imp_b(self) -> usize {
    (self.height() >> (IMPORTANCE_BLOCK_TO_BLOCK_SHIFT + BLOCK_TO_PLANE_SHIFT))
      .max(1)
  }

  #[inline]
  pub const fn tx_size(self) -> TxSize {
    match self {
      BLOCK_4X4 => TX_4X4,
      BLOCK_4X8 => TX_4X8,
      BLOCK_8X4 => TX_8X4,
      BLOCK_8X8 => TX_8X8,
      BLOCK_8X16 => TX_8X16,
      BLOCK_16X8 => TX_16X8,
      BLOCK_16X16 => TX_16X16,
      BLOCK_16X32 => TX_16X32,
      BLOCK_32X16 => TX_32X16,
      BLOCK_32X32 => TX_32X32,
      BLOCK_32X64 => TX_32X64,
      BLOCK_64X32 => TX_64X32,
      BLOCK_4X16 => TX_4X16,
      BLOCK_16X4 => TX_16X4,
      BLOCK_8X32 => TX_8X32,
      BLOCK_32X8 => TX_32X8,
      BLOCK_16X64 => TX_16X64,
      BLOCK_64X16 => TX_64X16,
      _ => TX_64X64,
    }
  }

  /// Source: `Subsampled_Size` (AV1 specification section 5.11.38)
  ///
  /// # Errors
  ///
  /// - Returns `InvalidBlockSize` if the given block size cannot
  ///   be subsampled in the requested way.
  #[inline]
  pub const fn subsampled_size(
    self, xdec: usize, ydec: usize,
  ) -> Result<BlockSize, InvalidBlockSize> {
    Ok(match (xdec, ydec) {
      (0, 0) /* 4:4:4 */ => self,
      (1, 0) /* 4:2:2 */ => match self {
        BLOCK_4X4 | BLOCK_8X4 => BLOCK_4X4,
        BLOCK_8X8 => BLOCK_4X8,
        BLOCK_16X4 => BLOCK_8X4,
        BLOCK_16X8 => BLOCK_8X8,
        BLOCK_16X16 => BLOCK_8X16,
        BLOCK_32X8 => BLOCK_16X8,
        BLOCK_32X16 => BLOCK_16X16,
        BLOCK_32X32 => BLOCK_16X32,
        BLOCK_64X16 => BLOCK_32X16,
        BLOCK_64X32 => BLOCK_32X32,
        BLOCK_64X64 => BLOCK_32X64,
        BLOCK_128X64 => BLOCK_64X64,
        BLOCK_128X128 => BLOCK_64X128,
        _ => return Err(InvalidBlockSize),
      },
      (1, 1) /* 4:2:0 */ => match self {
        BLOCK_4X4 | BLOCK_4X8 | BLOCK_8X4 | BLOCK_8X8 => BLOCK_4X4,
        BLOCK_4X16 | BLOCK_8X16 => BLOCK_4X8,
        BLOCK_8X32 => BLOCK_4X16,
        BLOCK_16X4 | BLOCK_16X8 => BLOCK_8X4,
        BLOCK_16X16 => BLOCK_8X8,
        BLOCK_16X32 => BLOCK_8X16,
        BLOCK_16X64 => BLOCK_8X32,
        BLOCK_32X8 => BLOCK_16X4,
        BLOCK_32X16 => BLOCK_16X8,
        BLOCK_32X32 => BLOCK_16X16,
        BLOCK_32X64 => BLOCK_16X32,
        BLOCK_64X16 => BLOCK_32X8,
        BLOCK_64X32 => BLOCK_32X16,
        BLOCK_64X64 => BLOCK_32X32,
        BLOCK_64X128 => BLOCK_32X64,
        BLOCK_128X64 => BLOCK_64X32,
        BLOCK_128X128 => BLOCK_64X64,
      },
      _ => return Err(InvalidBlockSize),
    })
  }

  /// # Panics
  ///
  /// Will panic if the subsampling is not possible
  #[inline]
  pub fn largest_chroma_tx_size(self, xdec: usize, ydec: usize) -> TxSize {
    let plane_bsize = self
      .subsampled_size(xdec, ydec)
      .expect("invalid block size for this subsampling mode");

    let chroma_tx_size = max_txsize_rect_lookup[plane_bsize as usize];

    av1_get_coded_tx_size(chroma_tx_size)
  }

  #[inline]
  pub const fn is_sqr(self) -> bool {
    self.width_log2() == self.height_log2()
  }

  #[inline]
  pub const fn is_sub8x8(self, xdec: usize, ydec: usize) -> bool {
    xdec != 0 && self.width_log2() == 2 || ydec != 0 && self.height_log2() == 2
  }

  #[inline]
  pub const fn sub8x8_offset(
    self, xdec: usize, ydec: usize,
  ) -> (isize, isize) {
    let offset_x = if xdec != 0 && self.width_log2() == 2 { -1 } else { 0 };
    let offset_y = if ydec != 0 && self.height_log2() == 2 { -1 } else { 0 };

    (offset_x, offset_y)
  }

  /// # Errors
  ///
  /// - Returns `InvalidBlockSize` if the block size cannot be split
  ///   in the requested way.
  pub const fn subsize(
    self, partition: PartitionType,
  ) -> Result<BlockSize, InvalidBlockSize> {
    use PartitionType::*;

    Ok(match partition {
      PARTITION_NONE => self,
      PARTITION_SPLIT => match self {
        BLOCK_8X8 => BLOCK_4X4,
        BLOCK_16X16 => BLOCK_8X8,
        BLOCK_32X32 => BLOCK_16X16,
        BLOCK_64X64 => BLOCK_32X32,
        BLOCK_128X128 => BLOCK_64X64,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_HORZ | PARTITION_HORZ_A | PARTITION_HORZ_B => match self {
        BLOCK_8X8 => BLOCK_8X4,
        BLOCK_16X16 => BLOCK_16X8,
        BLOCK_32X32 => BLOCK_32X16,
        BLOCK_64X64 => BLOCK_64X32,
        BLOCK_128X128 => BLOCK_128X64,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_VERT | PARTITION_VERT_A | PARTITION_VERT_B => match self {
        BLOCK_8X8 => BLOCK_4X8,
        BLOCK_16X16 => BLOCK_8X16,
        BLOCK_32X32 => BLOCK_16X32,
        BLOCK_64X64 => BLOCK_32X64,
        BLOCK_128X128 => BLOCK_64X128,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_HORZ_4 => match self {
        BLOCK_16X16 => BLOCK_16X4,
        BLOCK_32X32 => BLOCK_32X8,
        BLOCK_64X64 => BLOCK_64X16,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_VERT_4 => match self {
        BLOCK_16X16 => BLOCK_4X16,
        BLOCK_32X32 => BLOCK_8X32,
        BLOCK_64X64 => BLOCK_16X64,
        _ => return Err(InvalidBlockSize),
      },
      _ => return Err(InvalidBlockSize),
    })
  }

  pub const fn is_rect_tx_allowed(self) -> bool {
    !matches!(
      self,
      BLOCK_4X4
        | BLOCK_8X8
        | BLOCK_16X16
        | BLOCK_32X32
        | BLOCK_64X64
        | BLOCK_64X128
        | BLOCK_128X64
        | BLOCK_128X128
    )
  }
}

impl fmt::Display for BlockSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{}",
      match self {
        BlockSize::BLOCK_4X4 => "4x4",
        BlockSize::BLOCK_4X8 => "4x8",
        BlockSize::BLOCK_8X4 => "8x4",
        BlockSize::BLOCK_8X8 => "8x8",
        BlockSize::BLOCK_8X16 => "8x16",
        BlockSize::BLOCK_16X8 => "16x8",
        BlockSize::BLOCK_16X16 => "16x16",
        BlockSize::BLOCK_16X32 => "16x32",
        BlockSize::BLOCK_32X16 => "32x16",
        BlockSize::BLOCK_32X32 => "32x32",
        BlockSize::BLOCK_32X64 => "32x64",
        BlockSize::BLOCK_64X32 => "64x32",
        BlockSize::BLOCK_64X64 => "64x64",
        BlockSize::BLOCK_64X128 => "64x128",
        BlockSize::BLOCK_128X64 => "128x64",
        BlockSize::BLOCK_128X128 => "128x128",
        BlockSize::BLOCK_4X16 => "4x16",
        BlockSize::BLOCK_16X4 => "16x4",
        BlockSize::BLOCK_8X32 => "8x32",
        BlockSize::BLOCK_32X8 => "32x8",
        BlockSize::BLOCK_16X64 => "16x64",
        BlockSize::BLOCK_64X16 => "64x16",
      }
    )
  }
}

pub const NEWMV_MODE_CONTEXTS: usize = 7;
pub const GLOBALMV_MODE_CONTEXTS: usize = 2;
pub const REFMV_MODE_CONTEXTS: usize = 6;
pub const INTER_COMPOUND_MODES: usize = 8;

pub const REFMV_OFFSET: usize = 4;
pub const GLOBALMV_OFFSET: usize = 3;
pub const NEWMV_CTX_MASK: usize = (1 << GLOBALMV_OFFSET) - 1;
pub const GLOBALMV_CTX_MASK: usize =
  (1 << (REFMV_OFFSET - GLOBALMV_OFFSET)) - 1;
pub const REFMV_CTX_MASK: usize = (1 << (8 - REFMV_OFFSET)) - 1;

pub static RAV1E_PARTITION_TYPES: &[PartitionType] = &[
  PartitionType::PARTITION_NONE,
  PartitionType::PARTITION_HORZ,
  PartitionType::PARTITION_VERT,
  PartitionType::PARTITION_SPLIT,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum GlobalMVMode {
  IDENTITY = 0,    // identity transformation, 0-parameter
  TRANSLATION = 1, // translational motion 2-parameter
  ROTZOOM = 2,     // simplified affine with rotation + zoom only, 4-parameter
  AFFINE = 3,      // affine, 6-parameter
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum MvSubpelPrecision {
  MV_SUBPEL_NONE = -1,
  MV_SUBPEL_LOW_PRECISION = 0,
  MV_SUBPEL_HIGH_PRECISION,
}

/* Symbols for coding which components are zero jointly */
pub const MV_JOINTS: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum MvJointType {
  MV_JOINT_ZERO = 0,   /* Zero vector */
  MV_JOINT_HNZVZ = 1,  /* Vert zero, hor nonzero */
  MV_JOINT_HZVNZ = 2,  /* Hor zero, vert nonzero */
  MV_JOINT_HNZVNZ = 3, /* Both components nonzero */
}

fn supersample_chroma_bsize(
  bsize: BlockSize, ss_x: usize, ss_y: usize,
) -> BlockSize {
  debug_assert!(ss_x < 2);
  debug_assert!(ss_y < 2);

  match bsize {
    BLOCK_4X4 => match (ss_x, ss_y) {
      (1, 1) => BLOCK_8X8,
      (1, 0) => BLOCK_8X4,
      (0, 1) => BLOCK_4X8,
      _ => bsize,
    },
    BLOCK_4X8 => match (ss_x, ss_y) {
      (1, 1) => BLOCK_8X8,
      (1, 0) => BLOCK_8X8,
      (0, 1) => BLOCK_4X8,
      _ => bsize,
    },
    BLOCK_8X4 => match (ss_x, ss_y) {
      (1, 1) => BLOCK_8X8,
      (1, 0) => BLOCK_8X4,
      (0, 1) => BLOCK_8X8,
      _ => bsize,
    },
    BLOCK_4X16 => match (ss_x, ss_y) {
      (1, 1) => BLOCK_8X16,
      (1, 0) => BLOCK_8X16,
      (0, 1) => BLOCK_4X16,
      _ => bsize,
    },
    BLOCK_16X4 => match (ss_x, ss_y) {
      (1, 1) => BLOCK_16X8,
      (1, 0) => BLOCK_16X4,
      (0, 1) => BLOCK_16X8,
      _ => bsize,
    },
    _ => bsize,
  }
}

type IntraEdgeBuffer<T> = Aligned<[MaybeUninit<T>; 4 * MAX_TX_SIZE + 1]>;

#[cfg(any(test, feature = "bench"))]
type IntraEdgeMock<T> = Aligned<[T; 4 * MAX_TX_SIZE + 1]>;

pub struct IntraEdge<'a, T: Pixel>(&'a [T], &'a [T], &'a [T]);

impl<'a, T: Pixel> IntraEdge<'a, T> {
  fn new(
    edge_buf: &'a mut IntraEdgeBuffer<T>, init_left: usize, init_above: usize,
  ) -> Self {
    // SAFETY: Initialized in `get_intra_edges`.
    let left = unsafe {
      let begin_left = 2 * MAX_TX_SIZE - init_left;
      let end_above = 2 * MAX_TX_SIZE + 1 + init_above;
      slice_assume_init_mut(&mut edge_buf.data[begin_left..end_above])
    };
    let (left, top_left) = left.split_at(init_left);
    let (top_left, above) = top_left.split_at(1);
    Self(left, top_left, above)
  }

  pub const fn as_slices(&self) -> (&'a [T], &'a [T], &'a [T]) {
    (self.0, self.1, self.2)
  }

  pub const fn top_left_ptr(&self) -> *const T {
    self.1.as_ptr()
  }

  #[cfg(any(test, feature = "bench"))]
  pub fn mock(edge_buf: &'a IntraEdgeMock<T>) -> Self {
    let left = &edge_buf.data[..];
    let (left, top_left) = left.split_at(2 * MAX_TX_SIZE);
    let (top_left, above) = top_left.split_at(1);
    Self(left, top_left, above)
  }
}

pub fn get_intra_edges<'a, T: Pixel>(
  edge_buf: &'a mut IntraEdgeBuffer<T>,
  dst: &PlaneRegion<'_, T>,
  partition_bo: TileBlockOffset, // partition bo, BlockOffset
  bx: usize,
  by: usize,
  partition_size: BlockSize, // partition size, BlockSize
  po: PlaneOffset,
  tx_size: TxSize,
  bit_depth: usize,
  opt_mode: Option<PredictionMode>,
  enable_intra_edge_filter: bool,
  intra_param: IntraParam,
) -> IntraEdge<'a, T> {
  let mut init_left: usize = 0;
  let mut init_above: usize = 0;

  let plane_cfg = &dst.plane_cfg;

  let base = 128u16 << (bit_depth - 8);

  {
    // left pixels are ordered from bottom to top and right-aligned
    let (left, not_left) = edge_buf.data.split_at_mut(2 * MAX_TX_SIZE);
    let (top_left, above) = not_left.split_at_mut(1);

    let x = po.x as usize;
    let y = po.y as usize;

    let mut needs_left = true;
    let mut needs_topleft = true;
    let mut needs_top = true;
    let mut needs_topright = true;
    let mut needs_bottomleft = true;
    let mut needs_topleft_filter = false;

    if let Some(mut mode) = opt_mode {
      mode = match mode {
        PredictionMode::PAETH_PRED => match (x, y) {
          (0, 0) => PredictionMode::DC_PRED,
          (0, _) => PredictionMode::V_PRED,
          (_, 0) => PredictionMode::H_PRED,
          _ => PredictionMode::PAETH_PRED,
        },
        _ => mode,
      };

      let p_angle = intra_mode_to_angle(mode)
        + match intra_param {
          IntraParam::AngleDelta(val) => (val * ANGLE_STEP) as isize,
          _ => 0,
        };

      let dc_or_cfl =
        mode == PredictionMode::DC_PRED || mode == PredictionMode::UV_CFL_PRED;

      needs_left = (!dc_or_cfl || x != 0) || (p_angle > 90 && p_angle != 180);
      needs_topleft = mode == PredictionMode::PAETH_PRED
        || (mode.is_directional() && p_angle != 90 && p_angle != 180);
      needs_top = (!dc_or_cfl || y != 0) || (p_angle != 90 && p_angle < 180);
      needs_topright = mode.is_directional() && p_angle < 90;
      needs_bottomleft = mode.is_directional() && p_angle > 180;
      needs_topleft_filter =
        enable_intra_edge_filter && p_angle > 90 && p_angle < 180;
    }

    let rect_w =
      dst.rect().width.min(dst.plane_cfg.width - dst.rect().x as usize);
    let rect_h =
      dst.rect().height.min(dst.plane_cfg.height - dst.rect().y as usize);

    // Needs left
    if needs_left {
      let txh = if y + tx_size.height() > rect_h {
        rect_h - y
      } else {
        tx_size.height()
      };
      if x != 0 {
        for i in 0..txh {
          debug_assert!(y + i < rect_h);
          left[2 * MAX_TX_SIZE - 1 - i].write(dst[y + i][x - 1]);
        }
        if txh < tx_size.height() {
          let val = dst[y + txh - 1][x - 1];
          for i in txh..tx_size.height() {
            left[2 * MAX_TX_SIZE - 1 - i].write(val);
          }
        }
      } else {
        let val = if y != 0 { dst[y - 1][0] } else { T::cast_from(base + 1) };
        for v in left[2 * MAX_TX_SIZE - tx_size.height()..].iter_mut() {
          v.write(val);
        }
      }
      init_left += tx_size.height();
    }

    // Needs top
    if needs_top {
      let txw = if x + tx_size.width() > rect_w {
        rect_w - x
      } else {
        tx_size.width()
      };
      if y != 0 {
        above[..txw].copy_from_slice(
          // SAFETY: &[T] and &[MaybeUninit<T>] have the same layout
          unsafe {
            transmute::<&[T], &[MaybeUninit<T>]>(&dst[y - 1][x..x + txw])
          },
        );
        if txw < tx_size.width() {
          let val = dst[y - 1][x + txw - 1];
          for i in txw..tx_size.width() {
            above[i].write(val);
          }
        }
      } else {
        let val = if x != 0 { dst[0][x - 1] } else { T::cast_from(base - 1) };
        for v in above[..tx_size.width()].iter_mut() {
          v.write(val);
        }
      }
      init_above += tx_size.width();
    }

    let bx4 = bx * (tx_size.width() >> MI_SIZE_LOG2); // bx,by are in tx block indices
    let by4 = by * (tx_size.height() >> MI_SIZE_LOG2);

    let have_top = by4 != 0
      || if plane_cfg.ydec != 0 {
        partition_bo.0.y > 1
      } else {
        partition_bo.0.y > 0
      };
    let have_left = bx4 != 0
      || if plane_cfg.xdec != 0 {
        partition_bo.0.x > 1
      } else {
        partition_bo.0.x > 0
      };

    let right_available = x + tx_size.width() < rect_w;
    let bottom_available = y + tx_size.height() < rect_h;

    let scaled_partition_size =
      supersample_chroma_bsize(partition_size, plane_cfg.xdec, plane_cfg.ydec);

    // Needs top right
    if needs_topright {
      debug_assert!(plane_cfg.xdec <= 1 && plane_cfg.ydec <= 1);

      let num_avail = if y != 0
        && has_top_right(
          scaled_partition_size,
          partition_bo,
          have_top,
          right_available,
          tx_size,
          by4,
          bx4,
          plane_cfg.xdec,
          plane_cfg.ydec,
        ) {
        tx_size.width().min(rect_w - x - tx_size.width())
      } else {
        0
      };
      if num_avail > 0 {
        above[tx_size.width()..][..num_avail].copy_from_slice(
          // SAFETY: &[T] and &[MaybeUninit<T>] have the same layout
          unsafe {
            transmute::<&[T], &[MaybeUninit<T>]>(
              &dst[y - 1][x + tx_size.width()..][..num_avail],
            )
          },
        );
      }
      if num_avail < tx_size.height() {
        let val = above[tx_size.width() + num_avail - 1];
        for v in above
          [tx_size.width() + num_avail..tx_size.width() + tx_size.height()]
          .iter_mut()
        {
          *v = val;
        }
      }
      init_above += tx_size.height();
    }

    // SAFETY: The blocks above have initialized the first `init_above` items.
    let above = unsafe { slice_assume_init_mut(&mut above[..init_above]) };

    // Needs bottom left
    if needs_bottomleft {
      debug_assert!(plane_cfg.xdec <= 1 && plane_cfg.ydec <= 1);

      let num_avail = if x != 0
        && has_bottom_left(
          scaled_partition_size,
          partition_bo,
          bottom_available,
          have_left,
          tx_size,
          by4,
          bx4,
          plane_cfg.xdec,
          plane_cfg.ydec,
        ) {
        tx_size.height().min(rect_h - y - tx_size.height())
      } else {
        0
      };
      if num_avail > 0 {
        for i in 0..num_avail {
          left[2 * MAX_TX_SIZE - tx_size.height() - 1 - i]
            .write(dst[y + tx_size.height() + i][x - 1]);
        }
      }
      if num_avail < tx_size.width() {
        let val = left[2 * MAX_TX_SIZE - tx_size.height() - num_avail];
        for v in left[(2 * MAX_TX_SIZE - tx_size.height() - tx_size.width())
          ..(2 * MAX_TX_SIZE - tx_size.height() - num_avail)]
          .iter_mut()
        {
          *v = val;
        }
      }
      init_left += tx_size.width();
    }

    // SAFETY: The blocks above have initialized last `init_left` items.
    let left = unsafe {
      slice_assume_init_mut(&mut left[2 * MAX_TX_SIZE - init_left..])
    };

    // Needs top-left
    if needs_topleft {
      let top_left = top_left[0].write(match (x, y) {
        (0, 0) => T::cast_from(base),
        (_, 0) => dst[0][x - 1],
        (0, _) => dst[y - 1][0],
        _ => dst[y - 1][x - 1],
      });

      let (w, h) = (tx_size.width(), tx_size.height());
      if needs_topleft_filter && w + h >= 24 {
        let (l, a, tl): (u32, u32, u32) =
          (left[left.len() - 1].into(), above[0].into(), (*top_left).into());
        let s = l * 5 + tl * 6 + a * 5;

        *top_left = T::cast_from((s + (1 << 3)) >> 4);
      }
    } else {
      top_left[0].write(T::cast_from(base));
    }
  }
  IntraEdge::new(edge_buf, init_left, init_above)
}

pub fn has_tr(bo: TileBlockOffset, bsize: BlockSize) -> bool {
  let sb_mi_size = BLOCK_64X64.width_mi(); /* Assume 64x64 for now */
  let mask_row = bo.0.y & LOCAL_BLOCK_MASK;
  let mask_col = bo.0.x & LOCAL_BLOCK_MASK;
  let target_n4_w = bsize.width_mi();
  let target_n4_h = bsize.height_mi();

  let mut bs = target_n4_w.max(target_n4_h);

  if bs > BLOCK_64X64.width_mi() {
    return false;
  }

  let mut has_tr = !((mask_row & bs) != 0 && (mask_col & bs) != 0);

  /* TODO: assert its a power of two */

  while bs < sb_mi_size {
    if (mask_col & bs) != 0 {
      if (mask_col & (2 * bs) != 0) && (mask_row & (2 * bs) != 0) {
        has_tr = false;
        break;
      }
    } else {
      break;
    }
    bs <<= 1;
  }

  /* The left hand of two vertical rectangles always has a top right (as the
   * block above will have been decoded) */
  if (target_n4_w < target_n4_h) && (bo.0.x & target_n4_w) == 0 {
    has_tr = true;
  }

  /* The bottom of two horizontal rectangles never has a top right (as the block
   * to the right won't have been decoded) */
  if (target_n4_w > target_n4_h) && (bo.0.y & target_n4_h) != 0 {
    has_tr = false;
  }

  /* The bottom left square of a Vertical A (in the old format) does
   * not have a top right as it is decoded before the right hand
   * rectangle of the partition */
  /*
    if blk.partition == PartitionType::PARTITION_VERT_A {
      if blk.n4_w == blk.n4_h {
        if (mask_row & bs) != 0 {
          has_tr = false;
        }
      }
    }
  */

  has_tr
}

pub fn has_bl(bo: TileBlockOffset, bsize: BlockSize) -> bool {
  let sb_mi_size = BLOCK_64X64.width_mi(); /* Assume 64x64 for now */
  let mask_row = bo.0.y & LOCAL_BLOCK_MASK;
  let mask_col = bo.0.x & LOCAL_BLOCK_MASK;
  let target_n4_w = bsize.width_mi();
  let target_n4_h = bsize.height_mi();

  let mut bs = target_n4_w.max(target_n4_h);

  if bs > BLOCK_64X64.width_mi() {
    return false;
  }

  let mut has_bl =
    (mask_row & bs) == 0 && (mask_col & bs) == 0 && bs < sb_mi_size;

  /* TODO: assert its a power of two */

  while 2 * bs < sb_mi_size {
    if (mask_col & bs) == 0 {
      if (mask_col & (2 * bs) == 0) && (mask_row & (2 * bs) == 0) {
        has_bl = true;
        break;
      }
    } else {
      break;
    }
    bs <<= 1;
  }

  /* The right hand of two vertical rectangles never has a bottom left (as the
   * block below won't have been decoded) */
  if (target_n4_w < target_n4_h) && (bo.0.x & target_n4_w) != 0 {
    has_bl = false;
  }

  /* The top of two horizontal rectangles always has a bottom left (as the block
   * to the left will have been decoded) */
  if (target_n4_w > target_n4_h) && (bo.0.y & target_n4_h) == 0 {
    has_bl = true;
  }

  /* The bottom left square of a Vertical A (in the old format) does
   * not have a top right as it is decoded before the right hand
   * rectangle of the partition */
  /*
    if blk.partition == PartitionType::PARTITION_VERT_A {
      if blk.n4_w == blk.n4_h {
        if (mask_row & bs) != 0 {
          has_tr = false;
        }
      }
    }
  */

  has_bl
}

#[cfg(test)]
mod tests {
  use crate::partition::BlockSize::*;
  use crate::partition::{BlockSize, InvalidBlockSize};

  #[test]
  fn from_wh_matches_naive() {
    fn from_wh_opt_naive(
      w: usize, h: usize,
    ) -> Result<BlockSize, InvalidBlockSize> {
      match (w, h) {
        (4, 4) => Ok(BLOCK_4X4),
        (4, 8) => Ok(BLOCK_4X8),
        (8, 4) => Ok(BLOCK_8X4),
        (8, 8) => Ok(BLOCK_8X8),
        (8, 16) => Ok(BLOCK_8X16),
        (16, 8) => Ok(BLOCK_16X8),
        (16, 16) => Ok(BLOCK_16X16),
        (16, 32) => Ok(BLOCK_16X32),
        (32, 16) => Ok(BLOCK_32X16),
        (32, 32) => Ok(BLOCK_32X32),
        (32, 64) => Ok(BLOCK_32X64),
        (64, 32) => Ok(BLOCK_64X32),
        (64, 64) => Ok(BLOCK_64X64),
        (64, 128) => Ok(BLOCK_64X128),
        (128, 64) => Ok(BLOCK_128X64),
        (128, 128) => Ok(BLOCK_128X128),
        (4, 16) => Ok(BLOCK_4X16),
        (16, 4) => Ok(BLOCK_16X4),
        (8, 32) => Ok(BLOCK_8X32),
        (32, 8) => Ok(BLOCK_32X8),
        (16, 64) => Ok(BLOCK_16X64),
        (64, 16) => Ok(BLOCK_64X16),
        _ => Err(InvalidBlockSize),
      }
    }

    for w in 0..256 {
      for h in 0..256 {
        let a = BlockSize::from_width_and_height_opt(w, h);
        let b = from_wh_opt_naive(w, h);

        assert_eq!(a, b);
      }
    }
  }
}
