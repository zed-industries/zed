// Copyright (c) 2017-2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::context::*;
use crate::partition::BlockSize::*;
use crate::partition::*;
use crate::transform::*;

static has_null: &[u8] = &[];

// Tables to store if the top-right reference pixels are available. The flags
// are represented with bits, packed into 8-bit integers. E.g., for the 32x32
// blocks in a 128x128 superblock, the index of the "o" block is 10 (in raster
// order), so its flag is stored at the 3rd bit of the 2nd entry in the table,
// i.e. (table[10 / 8] >> (10 % 8)) & 1.
//       . . . .
//       . . . .
//       . . o .
//       . . . .
#[rustfmt::skip]
static has_tr_4x4: &[u8] = &[
  255, 255, 255, 255, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
  127, 127, 127, 127, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
  255, 127, 255, 127, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
  127, 127, 127, 127, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
  255, 255, 255, 127, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
  127, 127, 127, 127, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
  255, 127, 255, 127, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
  127, 127, 127, 127, 85, 85, 85, 85, 119, 119, 119, 119, 85, 85, 85, 85,
];

static has_tr_4x8: &[u8] = &[
  255, 255, 255, 255, 119, 119, 119, 119, 127, 127, 127, 127, 119, 119, 119,
  119, 255, 127, 255, 127, 119, 119, 119, 119, 127, 127, 127, 127, 119, 119,
  119, 119, 255, 255, 255, 127, 119, 119, 119, 119, 127, 127, 127, 127, 119,
  119, 119, 119, 255, 127, 255, 127, 119, 119, 119, 119, 127, 127, 127, 127,
  119, 119, 119, 119,
];

#[rustfmt::skip]
static has_tr_8x4: &[u8] = &[
  255, 255, 0, 0, 85, 85, 0, 0, 119, 119, 0, 0, 85, 85, 0, 0,
  127, 127, 0, 0, 85, 85, 0, 0, 119, 119, 0, 0, 85, 85, 0, 0,
  255, 127, 0, 0, 85, 85, 0, 0, 119, 119, 0, 0, 85, 85, 0, 0,
  127, 127, 0, 0, 85, 85, 0, 0, 119, 119, 0, 0, 85, 85, 0, 0,
];

#[rustfmt::skip]
static has_tr_8x8: &[u8] = &[
  255, 255, 85, 85, 119, 119, 85, 85, 127, 127, 85, 85, 119, 119, 85, 85,
  255, 127, 85, 85, 119, 119, 85, 85, 127, 127, 85, 85, 119, 119, 85, 85,
];
static has_tr_8x16: &[u8] = &[
  255, 255, 119, 119, 127, 127, 119, 119, 255, 127, 119, 119, 127, 127, 119,
  119,
];
static has_tr_16x8: &[u8] =
  &[255, 0, 85, 0, 119, 0, 85, 0, 127, 0, 85, 0, 119, 0, 85, 0];
static has_tr_16x16: &[u8] = &[255, 85, 119, 85, 127, 85, 119, 85];
static has_tr_16x32: &[u8] = &[255, 119, 127, 119];
static has_tr_32x16: &[u8] = &[15, 5, 7, 5];
static has_tr_32x32: &[u8] = &[95, 87];
static has_tr_32x64: &[u8] = &[127];
static has_tr_64x32: &[u8] = &[19];
static has_tr_64x64: &[u8] = &[7];
static has_tr_64x128: &[u8] = &[3];
static has_tr_128x64: &[u8] = &[1];
static has_tr_128x128: &[u8] = &[1];
static has_tr_4x16: &[u8] = &[
  255, 255, 255, 255, 127, 127, 127, 127, 255, 127, 255, 127, 127, 127, 127,
  127, 255, 255, 255, 127, 127, 127, 127, 127, 255, 127, 255, 127, 127, 127,
  127, 127,
];
static has_tr_16x4: &[u8] = &[
  255, 0, 0, 0, 85, 0, 0, 0, 119, 0, 0, 0, 85, 0, 0, 0, 127, 0, 0, 0, 85, 0,
  0, 0, 119, 0, 0, 0, 85, 0, 0, 0,
];
static has_tr_8x32: &[u8] = &[255, 255, 127, 127, 255, 127, 127, 127];
static has_tr_32x8: &[u8] = &[15, 0, 5, 0, 7, 0, 5, 0];
static has_tr_16x64: &[u8] = &[255, 127];
static has_tr_64x16: &[u8] = &[3, 1];

static has_tr_tables: &[&[u8]] = &[
  has_tr_4x4,     // 4x4
  has_tr_4x8,     // 4x8
  has_tr_8x4,     // 8x4
  has_tr_8x8,     // 8x8
  has_tr_8x16,    // 8x16
  has_tr_16x8,    // 16x8
  has_tr_16x16,   // 16x16
  has_tr_16x32,   // 16x32
  has_tr_32x16,   // 32x16
  has_tr_32x32,   // 32x32
  has_tr_32x64,   // 32x64
  has_tr_64x32,   // 64x32
  has_tr_64x64,   // 64x64
  has_tr_64x128,  // 64x128
  has_tr_128x64,  // 128x64
  has_tr_128x128, // 128x128
  has_tr_4x16,    // 4x16
  has_tr_16x4,    // 16x4
  has_tr_8x32,    // 8x32
  has_tr_32x8,    // 32x8
  has_tr_16x64,   // 16x64
  has_tr_64x16,   // 64x16
];

#[rustfmt::skip]
static has_tr_vert_8x8: &[u8] = &[
  255, 255, 0, 0, 119, 119, 0, 0, 127, 127, 0, 0, 119, 119, 0, 0,
  255, 127, 0, 0, 119, 119, 0, 0, 127, 127, 0, 0, 119, 119, 0, 0,
];
static has_tr_vert_16x16: &[u8] = &[255, 0, 119, 0, 127, 0, 119, 0];
static has_tr_vert_32x32: &[u8] = &[15, 7];
static has_tr_vert_64x64: &[u8] = &[3];

// The _vert_* tables are like the ordinary tables above, but describe the
// order we visit square blocks when doing a PARTITION_VERT_A or
// PARTITION_VERT_B. This is the same order as normal except for on the last
// split where we go vertically (TL, BL, TR, BR). We treat the rectangular block
// as a pair of squares, which means that these tables work correctly for both
// mixed vertical partition types.
//
// There are tables for each of the square sizes. Vertical rectangles (like
// BLOCK_16X32) use their respective "non-vert" table
static has_tr_vert_tables: &[&[u8]] = &[
  has_null,          // 4X4
  has_tr_4x8,        // 4X8
  has_null,          // 8X4
  has_tr_vert_8x8,   // 8X8
  has_tr_8x16,       // 8X16
  has_null,          // 16X8
  has_tr_vert_16x16, // 16X16
  has_tr_16x32,      // 16X32
  has_null,          // 32X16
  has_tr_vert_32x32, // 32X32
  has_tr_32x64,      // 32X64
  has_null,          // 64X32
  has_tr_vert_64x64, // 64X64
  has_tr_64x128,     // 64x128
  has_null,          // 128x64
  has_tr_128x128,    // 128x128
];

// TODO: Enable the case for PARTITION_VERT_A/B once they can be encoded by rav1e.
pub fn get_has_tr_table(
  /*partition: PartitionType, */ bsize: BlockSize,
) -> &'static [u8] {
  let ret: &[u8];
  // If this is a mixed vertical partition, look up bsize in orders_vert.
  /*if partition == PartitionType::PARTITION_VERT_A || partition == PartitionType::PARTITION_VERT_B {
    debug_assert!(bsize < BlockSize::BLOCK_SIZES);
    ret = has_tr_vert_tables[bsize as usize];
  } else */
  {
    ret = has_tr_tables[bsize as usize];
  }

  //debug_assert!(ret != ptr::has_null());

  ret
}

pub fn has_top_right(
  bsize: BlockSize, partition_bo: TileBlockOffset, top_available: bool,
  right_available: bool, tx_size: TxSize, row_off: usize, col_off: usize,
  ss_x: usize, _ss_y: usize,
) -> bool {
  if !top_available || !right_available {
    return false;
  };

  let bw_unit = bsize.width_mi();
  let plane_bw_unit = (bw_unit >> ss_x).max(1);
  let top_right_count_unit = tx_size.width_mi();

  let mi_col = partition_bo.0.x;
  let mi_row = partition_bo.0.y;

  if row_off > 0 {
    // Just need to check if enough pixels on the right.
    // 128x128 SB is not supported yet by rav1e
    if bsize.width() > BLOCK_64X64.width() {
      // Special case: For 128x128 blocks, the transform unit whose
      // top-right corner is at the center of the block does in fact have
      // pixels available at its top-right corner.
      if row_off == BLOCK_64X64.height_mi() >> _ss_y
        && col_off + top_right_count_unit == BLOCK_64X64.width_mi() >> ss_x
      {
        return false;
      }
      let plane_bw_unit_64 = BLOCK_64X64.width_mi() >> ss_x;
      let col_off_64 = col_off % plane_bw_unit_64;
      return col_off_64 + top_right_count_unit < plane_bw_unit_64;
    }
    col_off + top_right_count_unit < plane_bw_unit
  } else {
    // All top-right pixels are in the block above, which is already available.
    if col_off + top_right_count_unit < plane_bw_unit {
      return true;
    };

    let bw_in_mi_log2 = bsize.width_log2() - MI_SIZE_LOG2;
    let bh_in_mi_log2 = bsize.height_log2() - MI_SIZE_LOG2;
    let sb_mi_size: usize = 16; // 64x64
    let blk_row_in_sb = (mi_row & (sb_mi_size - 1)) >> bh_in_mi_log2;
    let blk_col_in_sb = (mi_col & (sb_mi_size - 1)) >> bw_in_mi_log2;

    // Top row of superblock: so top-right pixels are in the top and/or
    // top-right superblocks, both of which are already available.
    if blk_row_in_sb == 0 {
      return true;
    };

    // Rightmost column of superblock (and not the top row): so top-right pixels
    // fall in the right superblock, which is not available yet.
    if ((blk_col_in_sb + 1) << bw_in_mi_log2) >= sb_mi_size {
      return false;
    };

    // General case (neither top row nor rightmost column): check if the
    // top-right block is coded before the current block.
    let this_blk_index =
      (blk_row_in_sb << (MAX_MIB_SIZE_LOG2 - bw_in_mi_log2)) + blk_col_in_sb;
    let idx1 = this_blk_index / 8;
    let idx2 = this_blk_index % 8;
    let has_tr_table: &[u8] = get_has_tr_table(/*partition,*/ bsize);

    ((has_tr_table[idx1] >> idx2) & 1) != 0
  }
}

// Similar to the has_tr_* tables, but store if the bottom-left reference
// pixels are available.
static has_bl_4x4: &[u8] = &[
  84, 85, 85, 85, 16, 17, 17, 17, 84, 85, 85, 85, 0, 1, 1, 1, 84, 85, 85, 85,
  16, 17, 17, 17, 84, 85, 85, 85, 0, 0, 1, 0, 84, 85, 85, 85, 16, 17, 17, 17,
  84, 85, 85, 85, 0, 1, 1, 1, 84, 85, 85, 85, 16, 17, 17, 17, 84, 85, 85, 85,
  0, 0, 0, 0, 84, 85, 85, 85, 16, 17, 17, 17, 84, 85, 85, 85, 0, 1, 1, 1, 84,
  85, 85, 85, 16, 17, 17, 17, 84, 85, 85, 85, 0, 0, 1, 0, 84, 85, 85, 85, 16,
  17, 17, 17, 84, 85, 85, 85, 0, 1, 1, 1, 84, 85, 85, 85, 16, 17, 17, 17, 84,
  85, 85, 85, 0, 0, 0, 0,
];
static has_bl_4x8: &[u8] = &[
  16, 17, 17, 17, 0, 1, 1, 1, 16, 17, 17, 17, 0, 0, 1, 0, 16, 17, 17, 17, 0,
  1, 1, 1, 16, 17, 17, 17, 0, 0, 0, 0, 16, 17, 17, 17, 0, 1, 1, 1, 16, 17, 17,
  17, 0, 0, 1, 0, 16, 17, 17, 17, 0, 1, 1, 1, 16, 17, 17, 17, 0, 0, 0, 0,
];
static has_bl_8x4: &[u8] = &[
  254, 255, 84, 85, 254, 255, 16, 17, 254, 255, 84, 85, 254, 255, 0, 1, 254,
  255, 84, 85, 254, 255, 16, 17, 254, 255, 84, 85, 254, 255, 0, 0, 254, 255,
  84, 85, 254, 255, 16, 17, 254, 255, 84, 85, 254, 255, 0, 1, 254, 255, 84,
  85, 254, 255, 16, 17, 254, 255, 84, 85, 254, 255, 0, 0,
];
static has_bl_8x8: &[u8] = &[
  84, 85, 16, 17, 84, 85, 0, 1, 84, 85, 16, 17, 84, 85, 0, 0, 84, 85, 16, 17,
  84, 85, 0, 1, 84, 85, 16, 17, 84, 85, 0, 0,
];
static has_bl_8x16: &[u8] =
  &[16, 17, 0, 1, 16, 17, 0, 0, 16, 17, 0, 1, 16, 17, 0, 0];
static has_bl_16x8: &[u8] =
  &[254, 84, 254, 16, 254, 84, 254, 0, 254, 84, 254, 16, 254, 84, 254, 0];
static has_bl_16x16: &[u8] = &[84, 16, 84, 0, 84, 16, 84, 0];
static has_bl_16x32: &[u8] = &[16, 0, 16, 0];
static has_bl_32x16: &[u8] = &[78, 14, 78, 14];
static has_bl_32x32: &[u8] = &[4, 4];
static has_bl_32x64: &[u8] = &[0];
static has_bl_64x32: &[u8] = &[34];
static has_bl_64x64: &[u8] = &[0];
static has_bl_64x128: &[u8] = &[0];
static has_bl_128x64: &[u8] = &[0];
static has_bl_128x128: &[u8] = &[0];
static has_bl_4x16: &[u8] = &[
  0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0,
  1, 1, 1, 0, 0, 0, 0,
];
static has_bl_16x4: &[u8] = &[
  254, 254, 254, 84, 254, 254, 254, 16, 254, 254, 254, 84, 254, 254, 254, 0,
  254, 254, 254, 84, 254, 254, 254, 16, 254, 254, 254, 84, 254, 254, 254, 0,
];
static has_bl_8x32: &[u8] = &[0, 1, 0, 0, 0, 1, 0, 0];
static has_bl_32x8: &[u8] = &[238, 78, 238, 14, 238, 78, 238, 14];
static has_bl_16x64: &[u8] = &[0, 0];
static has_bl_64x16: &[u8] = &[42, 42];

static has_bl_tables: &[&[u8]] = &[
  has_bl_4x4,     // 4x4
  has_bl_4x8,     // 4x8
  has_bl_8x4,     // 8x4
  has_bl_8x8,     // 8x8
  has_bl_8x16,    // 8x16
  has_bl_16x8,    // 16x8
  has_bl_16x16,   // 16x16
  has_bl_16x32,   // 16x32
  has_bl_32x16,   // 32x16
  has_bl_32x32,   // 32x32
  has_bl_32x64,   // 32x64
  has_bl_64x32,   // 64x32
  has_bl_64x64,   // 64x64
  has_bl_64x128,  // 64x128
  has_bl_128x64,  // 128x64
  has_bl_128x128, // 128x128
  has_bl_4x16,    // 4x16
  has_bl_16x4,    // 16x4
  has_bl_8x32,    // 8x32
  has_bl_32x8,    // 32x8
  has_bl_16x64,   // 16x64
  has_bl_64x16,   // 64x16
];

#[rustfmt::skip]
static has_bl_vert_8x8: &[u8] = &[
  254, 255, 16, 17, 254, 255, 0, 1, 254, 255, 16, 17, 254, 255, 0, 0,
  254, 255, 16, 17, 254, 255, 0, 1, 254, 255, 16, 17, 254, 255, 0, 0,
];
static has_bl_vert_16x16: &[u8] = &[254, 16, 254, 0, 254, 16, 254, 0];
static has_bl_vert_32x32: &[u8] = &[14, 14];
static has_bl_vert_64x64: &[u8] = &[2];

// The _vert_* tables are like the ordinary tables above, but describe the
// order we visit square blocks when doing a PARTITION_VERT_A or
// PARTITION_VERT_B. This is the same order as normal except for on the last
// split where we go vertically (TL, BL, TR, BR). We treat the rectangular block
// as a pair of squares, which means that these tables work correctly for both
// mixed vertical partition types.
//
// There are tables for each of the square sizes. Vertical rectangles (like
// BLOCK_16X32) use their respective "non-vert" table
static has_bl_vert_tables: &[&[u8]] = &[
  has_null,          // 4x4
  has_bl_4x8,        // 4x8
  has_null,          // 8x4
  has_bl_vert_8x8,   // 8x8
  has_bl_8x16,       // 8x16
  has_null,          // 16x8
  has_bl_vert_16x16, // 16x16
  has_bl_16x32,      // 16x32
  has_null,          // 32x16
  has_bl_vert_32x32, // 32x32
  has_bl_32x64,      // 32x64
  has_null,          // 64x32
  has_bl_vert_64x64, // 64x64
  has_bl_64x128,     // 64x128
  has_null,          // 128x64
  has_bl_128x128,    // 128x128
];

pub fn get_has_bl_table(
  /*partition: PartitionType, */ bsize: BlockSize,
) -> &'static [u8] {
  let ret: &[u8];
  // If this is a mixed vertical partition, look up bsize in orders_vert.
  /*if (partition == PARTITION_VERT_A || partition == PARTITION_VERT_B) {
    //assert(bsize < BLOCK_SIZES);
    ret = has_bl_vert_tables[bsize as usize];
  } else*/
  {
    ret = has_bl_tables[bsize as usize];
  }
  //debug_assert!(ret != ptr::has_null());
  ret
}

pub fn has_bottom_left(
  bsize: BlockSize, partition_bo: TileBlockOffset, bottom_available: bool,
  left_available: bool, tx_size: TxSize, row_off: usize, col_off: usize,
  _ss_x: usize, ss_y: usize,
) -> bool {
  if !bottom_available || !left_available {
    return false;
  };

  // Special case for 128x* blocks, when col_off is half the block width.
  // This is needed because 128x* superblocks are divided into 64x* blocks in
  // raster order
  // 128x128 SB is not supported yet by rav1e
  if bsize.width() > BLOCK_64X64.width() && col_off > 0 {
    let plane_bw_unit_64 = BLOCK_64X64.width_mi() >> _ss_x;
    let col_off_64 = col_off % plane_bw_unit_64;
    if col_off_64 == 0 {
      // We are at the left edge of top-right or bottom-right 64x* block.
      let plane_bh_unit_64 = BLOCK_64X64.height_mi() >> ss_y;
      let row_off_64 = row_off % plane_bh_unit_64;
      let plane_bh_unit = (bsize.height_mi() >> ss_y).min(plane_bh_unit_64);
      // Check if all bottom-left pixels are in the left 64x* block (which is
      // already coded).
      return row_off_64 + tx_size.height_mi() < plane_bh_unit;
    }
  }

  if col_off > 0 {
    // Bottom-left pixels are in the bottom-left block, which is not available.
    false
  } else {
    let bh_unit = bsize.height_mi();
    let plane_bh_unit = (bh_unit >> ss_y).max(1);
    let bottom_left_count_unit = tx_size.height_mi();

    let mi_col = partition_bo.0.x;
    let mi_row = partition_bo.0.y;

    // All bottom-left pixels are in the left block, which is already available.
    if row_off + bottom_left_count_unit < plane_bh_unit {
      return true;
    };

    let bw_in_mi_log2 = bsize.width_log2() - MI_SIZE_LOG2;
    let bh_in_mi_log2 = bsize.height_log2() - MI_SIZE_LOG2;
    let sb_mi_size: usize = 16; // 64x64
    let blk_row_in_sb = (mi_row & (sb_mi_size - 1)) >> bh_in_mi_log2;
    let blk_col_in_sb = (mi_col & (sb_mi_size - 1)) >> bw_in_mi_log2;

    // Leftmost column of superblock: so bottom-left pixels maybe in the left
    // and/or bottom-left superblocks. But only the left superblock is
    // available, so check if all required pixels fall in that superblock.
    if blk_col_in_sb == 0 {
      let blk_start_row_off = blk_row_in_sb << bh_in_mi_log2 >> ss_y;
      let row_off_in_sb = blk_start_row_off + row_off;
      let sb_height_unit = sb_mi_size >> ss_y;
      return row_off_in_sb + bottom_left_count_unit < sb_height_unit;
      //return row_off_in_sb + (bottom_left_count_unit << 1) < sb_height_unit;  // Don't it need tx height? again?
    }

    // Bottom row of superblock (and not the leftmost column): so bottom-left
    // pixels fall in the bottom superblock, which is not available yet.
    if ((blk_row_in_sb + 1) << bh_in_mi_log2) >= sb_mi_size {
      return false;
    };

    // General case (neither leftmost column nor bottom row): check if the
    // bottom-left block is coded before the current block.
    let this_blk_index =
      (blk_row_in_sb << (MAX_MIB_SIZE_LOG2 - bw_in_mi_log2)) + blk_col_in_sb;
    let idx1 = this_blk_index / 8;
    let idx2 = this_blk_index % 8;
    let has_bl_table: &[u8] = get_has_bl_table(/*partition,*/ bsize);

    ((has_bl_table[idx1] >> idx2) & 1) != 0
  }
}
