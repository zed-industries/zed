/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_COMMON_DATA_H_
#define AOM_AV1_COMMON_COMMON_DATA_H_

#include "av1/common/enums.h"
#include "aom/aom_integer.h"
#include "aom_dsp/aom_dsp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

// Log 2 conversion lookup tables in units of mode info (4x4).
// The Mi_Width_Log2 table in the spec (Section 9.3. Conversion tables).
static const uint8_t mi_size_wide_log2[BLOCK_SIZES_ALL] = {
  0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 0, 2, 1, 3, 2, 4
};
// The Mi_Height_Log2 table in the spec (Section 9.3. Conversion tables).
static const uint8_t mi_size_high_log2[BLOCK_SIZES_ALL] = {
  0, 1, 0, 1, 2, 1, 2, 3, 2, 3, 4, 3, 4, 5, 4, 5, 2, 0, 3, 1, 4, 2
};

// Width/height lookup tables in units of mode info (4x4).
// The Num_4x4_Blocks_Wide table in the spec (Section 9.3. Conversion tables).
static const uint8_t mi_size_wide[BLOCK_SIZES_ALL] = {
  1, 1, 2, 2, 2, 4, 4, 4, 8, 8, 8, 16, 16, 16, 32, 32, 1, 4, 2, 8, 4, 16
};

// The Num_4x4_Blocks_High table in the spec (Section 9.3. Conversion tables).
static const uint8_t mi_size_high[BLOCK_SIZES_ALL] = {
  1, 2, 1, 2, 4, 2, 4, 8, 4, 8, 16, 8, 16, 32, 16, 32, 4, 1, 8, 2, 16, 4
};

// Width/height lookup tables in units of samples.
// The Block_Width table in the spec (Section 9.3. Conversion tables).
static const uint8_t block_size_wide[BLOCK_SIZES_ALL] = {
  4,  4,  8,  8,   8,   16, 16, 16, 32, 32, 32,
  64, 64, 64, 128, 128, 4,  16, 8,  32, 16, 64
};

// The Block_Height table in the spec (Section 9.3. Conversion tables).
static const uint8_t block_size_high[BLOCK_SIZES_ALL] = {
  4,  8,  4,   8,  16,  8,  16, 32, 16, 32, 64,
  32, 64, 128, 64, 128, 16, 4,  32, 8,  64, 16
};

// Maps a block size to a context.
// The Size_Group table in the spec (Section 9.3. Conversion tables).
// AOMMIN(3, AOMMIN(mi_size_wide_log2(bsize), mi_size_high_log2(bsize)))
static const uint8_t size_group_lookup[BLOCK_SIZES_ALL] = {
  0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 0, 0, 1, 1, 2, 2
};

static const uint8_t num_pels_log2_lookup[BLOCK_SIZES_ALL] = {
  4, 5, 5, 6, 7, 7, 8, 9, 9, 10, 11, 11, 12, 13, 13, 14, 6, 6, 8, 8, 10, 10
};

// A compressed version of the Partition_Subsize table in the spec (9.3.
// Conversion tables), for square block sizes only.
/* clang-format off */
static const BLOCK_SIZE subsize_lookup[EXT_PARTITION_TYPES][SQR_BLOCK_SIZES] = {
  {     // PARTITION_NONE
    BLOCK_4X4, BLOCK_8X8, BLOCK_16X16,
    BLOCK_32X32, BLOCK_64X64, BLOCK_128X128
  }, {  // PARTITION_HORZ
    BLOCK_INVALID, BLOCK_8X4, BLOCK_16X8,
    BLOCK_32X16, BLOCK_64X32, BLOCK_128X64
  }, {  // PARTITION_VERT
    BLOCK_INVALID, BLOCK_4X8, BLOCK_8X16,
    BLOCK_16X32, BLOCK_32X64, BLOCK_64X128
  }, {  // PARTITION_SPLIT
    BLOCK_INVALID, BLOCK_4X4, BLOCK_8X8,
    BLOCK_16X16, BLOCK_32X32, BLOCK_64X64
  }, {  // PARTITION_HORZ_A
    BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X8,
    BLOCK_32X16, BLOCK_64X32, BLOCK_128X64
  }, {  // PARTITION_HORZ_B
    BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X8,
    BLOCK_32X16, BLOCK_64X32, BLOCK_128X64
  }, {  // PARTITION_VERT_A
    BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X16,
    BLOCK_16X32, BLOCK_32X64, BLOCK_64X128
  }, {  // PARTITION_VERT_B
    BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X16,
    BLOCK_16X32, BLOCK_32X64, BLOCK_64X128
  }, {  // PARTITION_HORZ_4
    BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X4,
    BLOCK_32X8, BLOCK_64X16, BLOCK_INVALID
  }, {  // PARTITION_VERT_4
    BLOCK_INVALID, BLOCK_INVALID, BLOCK_4X16,
    BLOCK_8X32, BLOCK_16X64, BLOCK_INVALID
  }
};

static const TX_SIZE max_txsize_lookup[BLOCK_SIZES_ALL] = {
  //                   4X4
                       TX_4X4,
  // 4X8,    8X4,      8X8
  TX_4X4,    TX_4X4,   TX_8X8,
  // 8X16,   16X8,     16X16
  TX_8X8,    TX_8X8,   TX_16X16,
  // 16X32,  32X16,    32X32
  TX_16X16,  TX_16X16, TX_32X32,
  // 32X64,  64X32,
  TX_32X32,  TX_32X32,
  // 64X64
  TX_64X64,
  // 64x128, 128x64,   128x128
  TX_64X64,  TX_64X64, TX_64X64,
  // 4x16,   16x4,     8x32
  TX_4X4,    TX_4X4,   TX_8X8,
  // 32x8,   16x64     64x16
  TX_8X8,    TX_16X16, TX_16X16
};

static const TX_SIZE max_txsize_rect_lookup[BLOCK_SIZES_ALL] = {
      // 4X4
      TX_4X4,
      // 4X8,    8X4,      8X8
      TX_4X8,    TX_8X4,   TX_8X8,
      // 8X16,   16X8,     16X16
      TX_8X16,   TX_16X8,  TX_16X16,
      // 16X32,  32X16,    32X32
      TX_16X32,  TX_32X16, TX_32X32,
      // 32X64,  64X32,
      TX_32X64,  TX_64X32,
      // 64X64
      TX_64X64,
      // 64x128, 128x64,   128x128
      TX_64X64,  TX_64X64, TX_64X64,
      // 4x16,   16x4,
      TX_4X16,   TX_16X4,
      // 8x32,   32x8
      TX_8X32,   TX_32X8,
      // 16x64,  64x16
      TX_16X64,  TX_64X16
};

static const TX_TYPE_1D vtx_tab[TX_TYPES] = {
  DCT_1D,      ADST_1D, DCT_1D,      ADST_1D,
  FLIPADST_1D, DCT_1D,  FLIPADST_1D, ADST_1D, FLIPADST_1D, IDTX_1D,
  DCT_1D,      IDTX_1D, ADST_1D,     IDTX_1D, FLIPADST_1D, IDTX_1D,
};

static const TX_TYPE_1D htx_tab[TX_TYPES] = {
  DCT_1D,  DCT_1D,      ADST_1D,     ADST_1D,
  DCT_1D,  FLIPADST_1D, FLIPADST_1D, FLIPADST_1D, ADST_1D, IDTX_1D,
  IDTX_1D, DCT_1D,      IDTX_1D,     ADST_1D,     IDTX_1D, FLIPADST_1D,
};

#define TXSIZE_CAT_INVALID (-1)

/* clang-format on */

static const TX_SIZE sub_tx_size_map[TX_SIZES_ALL] = {
  TX_4X4,    // TX_4X4
  TX_4X4,    // TX_8X8
  TX_8X8,    // TX_16X16
  TX_16X16,  // TX_32X32
  TX_32X32,  // TX_64X64
  TX_4X4,    // TX_4X8
  TX_4X4,    // TX_8X4
  TX_8X8,    // TX_8X16
  TX_8X8,    // TX_16X8
  TX_16X16,  // TX_16X32
  TX_16X16,  // TX_32X16
  TX_32X32,  // TX_32X64
  TX_32X32,  // TX_64X32
  TX_4X8,    // TX_4X16
  TX_8X4,    // TX_16X4
  TX_8X16,   // TX_8X32
  TX_16X8,   // TX_32X8
  TX_16X32,  // TX_16X64
  TX_32X16,  // TX_64X16
};

static const TX_SIZE txsize_horz_map[TX_SIZES_ALL] = {
  TX_4X4,    // TX_4X4
  TX_8X8,    // TX_8X8
  TX_16X16,  // TX_16X16
  TX_32X32,  // TX_32X32
  TX_64X64,  // TX_64X64
  TX_4X4,    // TX_4X8
  TX_8X8,    // TX_8X4
  TX_8X8,    // TX_8X16
  TX_16X16,  // TX_16X8
  TX_16X16,  // TX_16X32
  TX_32X32,  // TX_32X16
  TX_32X32,  // TX_32X64
  TX_64X64,  // TX_64X32
  TX_4X4,    // TX_4X16
  TX_16X16,  // TX_16X4
  TX_8X8,    // TX_8X32
  TX_32X32,  // TX_32X8
  TX_16X16,  // TX_16X64
  TX_64X64,  // TX_64X16
};

static const TX_SIZE txsize_vert_map[TX_SIZES_ALL] = {
  TX_4X4,    // TX_4X4
  TX_8X8,    // TX_8X8
  TX_16X16,  // TX_16X16
  TX_32X32,  // TX_32X32
  TX_64X64,  // TX_64X64
  TX_8X8,    // TX_4X8
  TX_4X4,    // TX_8X4
  TX_16X16,  // TX_8X16
  TX_8X8,    // TX_16X8
  TX_32X32,  // TX_16X32
  TX_16X16,  // TX_32X16
  TX_64X64,  // TX_32X64
  TX_32X32,  // TX_64X32
  TX_16X16,  // TX_4X16
  TX_4X4,    // TX_16X4
  TX_32X32,  // TX_8X32
  TX_8X8,    // TX_32X8
  TX_64X64,  // TX_16X64
  TX_16X16,  // TX_64X16
};

#define TX_SIZE_W_MIN 4

// Transform block width in pixels
static const int tx_size_wide[TX_SIZES_ALL] = {
  4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64,
};

#define TX_SIZE_H_MIN 4

// Transform block height in pixels
static const int tx_size_high[TX_SIZES_ALL] = {
  4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16,
};

// Transform block width in unit
static const int tx_size_wide_unit[TX_SIZES_ALL] = {
  1, 2, 4, 8, 16, 1, 2, 2, 4, 4, 8, 8, 16, 1, 4, 2, 8, 4, 16,
};

// Transform block height in unit
static const int tx_size_high_unit[TX_SIZES_ALL] = {
  1, 2, 4, 8, 16, 2, 1, 4, 2, 8, 4, 16, 8, 4, 1, 8, 2, 16, 4,
};

// Transform block width in log2
static const int tx_size_wide_log2[TX_SIZES_ALL] = {
  2, 3, 4, 5, 6, 2, 3, 3, 4, 4, 5, 5, 6, 2, 4, 3, 5, 4, 6,
};

// Transform block width in log2 unit
static const int tx_size_wide_unit_log2[TX_SIZES_ALL] = {
  0, 1, 2, 3, 4, 0, 1, 1, 2, 2, 3, 3, 4, 0, 2, 1, 3, 2, 4,
};

// Transform block height in log2
static const int tx_size_high_log2[TX_SIZES_ALL] = {
  2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4,
};

// Transform block height in log2 unit
static const int tx_size_high_unit_log2[TX_SIZES_ALL] = {
  0, 1, 2, 3, 4, 1, 0, 2, 1, 3, 2, 4, 3, 2, 0, 3, 1, 4, 2,
};

static const int tx_size_2d[TX_SIZES_ALL + 1] = {
  16,  64,   256,  1024, 4096, 32,  32,  128,  128,  512,
  512, 2048, 2048, 64,   64,   256, 256, 1024, 1024,
};

static const BLOCK_SIZE txsize_to_bsize[TX_SIZES_ALL] = {
  BLOCK_4X4,    // TX_4X4
  BLOCK_8X8,    // TX_8X8
  BLOCK_16X16,  // TX_16X16
  BLOCK_32X32,  // TX_32X32
  BLOCK_64X64,  // TX_64X64
  BLOCK_4X8,    // TX_4X8
  BLOCK_8X4,    // TX_8X4
  BLOCK_8X16,   // TX_8X16
  BLOCK_16X8,   // TX_16X8
  BLOCK_16X32,  // TX_16X32
  BLOCK_32X16,  // TX_32X16
  BLOCK_32X64,  // TX_32X64
  BLOCK_64X32,  // TX_64X32
  BLOCK_4X16,   // TX_4X16
  BLOCK_16X4,   // TX_16X4
  BLOCK_8X32,   // TX_8X32
  BLOCK_32X8,   // TX_32X8
  BLOCK_16X64,  // TX_16X64
  BLOCK_64X16,  // TX_64X16
};

static const TX_SIZE txsize_sqr_map[TX_SIZES_ALL] = {
  TX_4X4,    // TX_4X4
  TX_8X8,    // TX_8X8
  TX_16X16,  // TX_16X16
  TX_32X32,  // TX_32X32
  TX_64X64,  // TX_64X64
  TX_4X4,    // TX_4X8
  TX_4X4,    // TX_8X4
  TX_8X8,    // TX_8X16
  TX_8X8,    // TX_16X8
  TX_16X16,  // TX_16X32
  TX_16X16,  // TX_32X16
  TX_32X32,  // TX_32X64
  TX_32X32,  // TX_64X32
  TX_4X4,    // TX_4X16
  TX_4X4,    // TX_16X4
  TX_8X8,    // TX_8X32
  TX_8X8,    // TX_32X8
  TX_16X16,  // TX_16X64
  TX_16X16,  // TX_64X16
};

static const TX_SIZE txsize_sqr_up_map[TX_SIZES_ALL] = {
  TX_4X4,    // TX_4X4
  TX_8X8,    // TX_8X8
  TX_16X16,  // TX_16X16
  TX_32X32,  // TX_32X32
  TX_64X64,  // TX_64X64
  TX_8X8,    // TX_4X8
  TX_8X8,    // TX_8X4
  TX_16X16,  // TX_8X16
  TX_16X16,  // TX_16X8
  TX_32X32,  // TX_16X32
  TX_32X32,  // TX_32X16
  TX_64X64,  // TX_32X64
  TX_64X64,  // TX_64X32
  TX_16X16,  // TX_4X16
  TX_16X16,  // TX_16X4
  TX_32X32,  // TX_8X32
  TX_32X32,  // TX_32X8
  TX_64X64,  // TX_16X64
  TX_64X64,  // TX_64X16
};

static const int8_t txsize_log2_minus4[TX_SIZES_ALL] = {
  0,  // TX_4X4
  2,  // TX_8X8
  4,  // TX_16X16
  6,  // TX_32X32
  6,  // TX_64X64
  1,  // TX_4X8
  1,  // TX_8X4
  3,  // TX_8X16
  3,  // TX_16X8
  5,  // TX_16X32
  5,  // TX_32X16
  6,  // TX_32X64
  6,  // TX_64X32
  2,  // TX_4X16
  2,  // TX_16X4
  4,  // TX_8X32
  4,  // TX_32X8
  5,  // TX_16X64
  5,  // TX_64X16
};

static const TX_SIZE tx_mode_to_biggest_tx_size[TX_MODES] = {
  TX_4X4,    // ONLY_4X4
  TX_64X64,  // TX_MODE_LARGEST
  TX_64X64,  // TX_MODE_SELECT
};

// The Subsampled_Size table in the spec (Section 5.11.38. Get plane residual
// size function).
extern const BLOCK_SIZE av1_ss_size_lookup[BLOCK_SIZES_ALL][2][2];

// Generates 5 bit field in which each bit set to 1 represents
// a blocksize partition  11111 means we split 128x128, 64x64, 32x32, 16x16
// and 8x8.  10000 means we just split the 128x128 to 64x64
/* clang-format off */
static const struct {
  PARTITION_CONTEXT above;
  PARTITION_CONTEXT left;
} partition_context_lookup[BLOCK_SIZES_ALL] = {
  { 31, 31 },  // 4X4   - {0b11111, 0b11111}
  { 31, 30 },  // 4X8   - {0b11111, 0b11110}
  { 30, 31 },  // 8X4   - {0b11110, 0b11111}
  { 30, 30 },  // 8X8   - {0b11110, 0b11110}
  { 30, 28 },  // 8X16  - {0b11110, 0b11100}
  { 28, 30 },  // 16X8  - {0b11100, 0b11110}
  { 28, 28 },  // 16X16 - {0b11100, 0b11100}
  { 28, 24 },  // 16X32 - {0b11100, 0b11000}
  { 24, 28 },  // 32X16 - {0b11000, 0b11100}
  { 24, 24 },  // 32X32 - {0b11000, 0b11000}
  { 24, 16 },  // 32X64 - {0b11000, 0b10000}
  { 16, 24 },  // 64X32 - {0b10000, 0b11000}
  { 16, 16 },  // 64X64 - {0b10000, 0b10000}
  { 16, 0 },   // 64X128- {0b10000, 0b00000}
  { 0, 16 },   // 128X64- {0b00000, 0b10000}
  { 0, 0 },    // 128X128-{0b00000, 0b00000}
  { 31, 28 },  // 4X16  - {0b11111, 0b11100}
  { 28, 31 },  // 16X4  - {0b11100, 0b11111}
  { 30, 24 },  // 8X32  - {0b11110, 0b11000}
  { 24, 30 },  // 32X8  - {0b11000, 0b11110}
  { 28, 16 },  // 16X64 - {0b11100, 0b10000}
  { 16, 28 },  // 64X16 - {0b10000, 0b11100}
};
/* clang-format on */

static const int intra_mode_context[INTRA_MODES] = {
  0, 1, 2, 3, 4, 4, 4, 4, 3, 0, 1, 2, 0,
};

// Note: this is also used in unit tests. So whenever one changes the table,
// the unit tests need to be changed accordingly.
static const int quant_dist_weight[4][2] = {
  { 2, 3 }, { 2, 5 }, { 2, 7 }, { 1, MAX_FRAME_DISTANCE }
};

static const int quant_dist_lookup_table[4][2] = {
  { 9, 7 },
  { 11, 5 },
  { 12, 4 },
  { 13, 3 },
};

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_COMMON_DATA_H_
