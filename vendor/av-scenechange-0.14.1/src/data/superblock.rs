use crate::data::{
    block::{BlockOffset, MIB_SIZE_LOG2},
    tile::TileBlockOffset,
};

pub const MAX_SB_SIZE_LOG2: usize = 7;
pub const SUPERBLOCK_TO_BLOCK_SHIFT: usize = MIB_SIZE_LOG2;
pub const SB_SIZE_LOG2: usize = 6;
pub const SB_SIZE: usize = 1 << SB_SIZE_LOG2;
pub const MI_SIZE_LOG2: usize = 2;
pub const MI_SIZE: usize = 1 << MI_SIZE_LOG2;

/// Absolute offset in superblocks inside a tile, where a superblock is defined
/// to be an `N*N` square where `N == (1 << SUPERBLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileSuperBlockOffset(pub SuperBlockOffset);

impl TileSuperBlockOffset {
    /// Offset of a block inside the current superblock.
    pub const fn block_offset(self, block_x: usize, block_y: usize) -> TileBlockOffset {
        TileBlockOffset(self.0.block_offset(block_x, block_y))
    }
}

/// Absolute offset in superblocks inside a plane, where a superblock is defined
/// to be an `N*N` square where `N == (1 << SUPERBLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneSuperBlockOffset(pub SuperBlockOffset);

/// Absolute offset in superblocks, where a superblock is defined
/// to be an `N*N` square where `N == (1 << SUPERBLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuperBlockOffset {
    pub x: usize,
    pub y: usize,
}

impl SuperBlockOffset {
    /// Offset of a block inside the current superblock.
    const fn block_offset(self, block_x: usize, block_y: usize) -> BlockOffset {
        BlockOffset {
            x: (self.x << SUPERBLOCK_TO_BLOCK_SHIFT) + block_x,
            y: (self.y << SUPERBLOCK_TO_BLOCK_SHIFT) + block_y,
        }
    }
}
