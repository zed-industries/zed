// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;

pub const MAX_SB_SIZE_LOG2: usize = 7;
const SB_SIZE_LOG2: usize = 6;
pub const SB_SIZE: usize = 1 << SB_SIZE_LOG2;
const SB_SQUARE: usize = SB_SIZE * SB_SIZE;

pub const MI_SIZE_LOG2: usize = 2;
pub const MI_SIZE: usize = 1 << MI_SIZE_LOG2;
pub const MAX_MIB_SIZE_LOG2: usize = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MIB_SIZE_LOG2: usize = SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MIB_SIZE: usize = 1 << MIB_SIZE_LOG2;
pub const MIB_MASK: usize = MIB_SIZE - 1;

pub const SUPERBLOCK_TO_PLANE_SHIFT: usize = SB_SIZE_LOG2;
pub const SUPERBLOCK_TO_BLOCK_SHIFT: usize = MIB_SIZE_LOG2;
pub const BLOCK_TO_PLANE_SHIFT: usize = MI_SIZE_LOG2;
pub const IMPORTANCE_BLOCK_TO_BLOCK_SHIFT: usize = 1;
pub const LOCAL_BLOCK_MASK: usize = (1 << SUPERBLOCK_TO_BLOCK_SHIFT) - 1;

pub const MAX_SB_IN_IMP_B: usize = 1
  << (MAX_SB_SIZE_LOG2
    - IMPORTANCE_BLOCK_TO_BLOCK_SHIFT
    - BLOCK_TO_PLANE_SHIFT);

/// Absolute offset in superblocks, where a superblock is defined
/// to be an `N*N` square where `N == (1 << SUPERBLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuperBlockOffset {
  pub x: usize,
  pub y: usize,
}

/// Absolute offset in superblocks inside a plane, where a superblock is defined
/// to be an `N*N` square where `N == (1 << SUPERBLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneSuperBlockOffset(pub SuperBlockOffset);

/// Absolute offset in superblocks inside a tile, where a superblock is defined
/// to be an `N*N` square where `N == (1 << SUPERBLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileSuperBlockOffset(pub SuperBlockOffset);

impl SuperBlockOffset {
  /// Offset of a block inside the current superblock.
  #[inline]
  const fn block_offset(self, block_x: usize, block_y: usize) -> BlockOffset {
    BlockOffset {
      x: (self.x << SUPERBLOCK_TO_BLOCK_SHIFT) + block_x,
      y: (self.y << SUPERBLOCK_TO_BLOCK_SHIFT) + block_y,
    }
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    PlaneOffset {
      x: (self.x as isize) << (SUPERBLOCK_TO_PLANE_SHIFT - plane.xdec),
      y: (self.y as isize) << (SUPERBLOCK_TO_PLANE_SHIFT - plane.ydec),
    }
  }
}

impl Add for SuperBlockOffset {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

impl PlaneSuperBlockOffset {
  /// Offset of a block inside the current superblock.
  #[inline]
  pub const fn block_offset(
    self, block_x: usize, block_y: usize,
  ) -> PlaneBlockOffset {
    PlaneBlockOffset(self.0.block_offset(block_x, block_y))
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  pub const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    self.0.plane_offset(plane)
  }
}

impl Add for PlaneSuperBlockOffset {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    PlaneSuperBlockOffset(self.0 + rhs.0)
  }
}

impl TileSuperBlockOffset {
  /// Offset of a block inside the current superblock.
  #[inline]
  pub const fn block_offset(
    self, block_x: usize, block_y: usize,
  ) -> TileBlockOffset {
    TileBlockOffset(self.0.block_offset(block_x, block_y))
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  pub const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    self.0.plane_offset(plane)
  }
}

impl Add for TileSuperBlockOffset {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    TileSuperBlockOffset(self.0 + rhs.0)
  }
}
