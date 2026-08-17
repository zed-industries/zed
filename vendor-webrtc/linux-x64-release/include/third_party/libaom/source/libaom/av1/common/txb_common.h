/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_TXB_COMMON_H_
#define AOM_AV1_COMMON_TXB_COMMON_H_

#include "av1/common/av1_common_int.h"

extern const int16_t av1_eob_group_start[12];
extern const int16_t av1_eob_offset_bits[12];

extern const int8_t *av1_nz_map_ctx_offset[TX_SIZES_ALL];

typedef struct txb_ctx {
  int txb_skip_ctx;
  int dc_sign_ctx;
} TXB_CTX;

static const int base_level_count_to_index[13] = {
  0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3,
};

static const TX_CLASS tx_type_to_class[TX_TYPES] = {
  TX_CLASS_2D,     // DCT_DCT
  TX_CLASS_2D,     // ADST_DCT
  TX_CLASS_2D,     // DCT_ADST
  TX_CLASS_2D,     // ADST_ADST
  TX_CLASS_2D,     // FLIPADST_DCT
  TX_CLASS_2D,     // DCT_FLIPADST
  TX_CLASS_2D,     // FLIPADST_FLIPADST
  TX_CLASS_2D,     // ADST_FLIPADST
  TX_CLASS_2D,     // FLIPADST_ADST
  TX_CLASS_2D,     // IDTX
  TX_CLASS_VERT,   // V_DCT
  TX_CLASS_HORIZ,  // H_DCT
  TX_CLASS_VERT,   // V_ADST
  TX_CLASS_HORIZ,  // H_ADST
  TX_CLASS_VERT,   // V_FLIPADST
  TX_CLASS_HORIZ,  // H_FLIPADST
};

static inline int get_txb_bhl(TX_SIZE tx_size) {
  tx_size = av1_get_adjusted_tx_size(tx_size);
  return tx_size_high_log2[tx_size];
}

static inline int get_txb_wide(TX_SIZE tx_size) {
  tx_size = av1_get_adjusted_tx_size(tx_size);
  return tx_size_wide[tx_size];
}

static inline int get_txb_high(TX_SIZE tx_size) {
  tx_size = av1_get_adjusted_tx_size(tx_size);
  return tx_size_high[tx_size];
}

static inline uint8_t *set_levels(uint8_t *const levels_buf, const int height) {
  return levels_buf + TX_PAD_TOP * (height + TX_PAD_HOR);
}

static inline int get_padded_idx(const int idx, const int bhl) {
  return idx + ((idx >> bhl) << TX_PAD_HOR_LOG2);
}

static inline int get_br_ctx_2d(const uint8_t *const levels,
                                const int c,  // raster order
                                const int bhl) {
  assert(c > 0);
  const int col = c >> bhl;
  const int row = c - (col << bhl);
  const int stride = (1 << bhl) + TX_PAD_HOR;
  const int pos = col * stride + row;
  int mag = AOMMIN(levels[pos + 1], MAX_BASE_BR_RANGE) +
            AOMMIN(levels[pos + stride], MAX_BASE_BR_RANGE) +
            AOMMIN(levels[pos + 1 + stride], MAX_BASE_BR_RANGE);
  mag = AOMMIN((mag + 1) >> 1, 6);
  //((row | col) < 2) is equivalent to ((row < 2) && (col < 2))
  if ((row | col) < 2) return mag + 7;
  return mag + 14;
}

static AOM_FORCE_INLINE int get_br_ctx_eob(const int c,  // raster order
                                           const int bhl,
                                           const TX_CLASS tx_class) {
  const int col = c >> bhl;
  const int row = c - (col << bhl);
  if (c == 0) return 0;
  if ((tx_class == TX_CLASS_2D && row < 2 && col < 2) ||
      (tx_class == TX_CLASS_HORIZ && col == 0) ||
      (tx_class == TX_CLASS_VERT && row == 0))
    return 7;
  return 14;
}

static AOM_FORCE_INLINE int get_br_ctx(const uint8_t *const levels,
                                       const int c,  // raster order
                                       const int bhl, const TX_CLASS tx_class) {
  const int col = c >> bhl;
  const int row = c - (col << bhl);
  const int stride = (1 << bhl) + TX_PAD_HOR;
  const int pos = col * stride + row;
  int mag = levels[pos + 1];
  mag += levels[pos + stride];
  switch (tx_class) {
    case TX_CLASS_2D:
      mag += levels[pos + stride + 1];
      mag = AOMMIN((mag + 1) >> 1, 6);
      if (c == 0) return mag;
      if ((row < 2) && (col < 2)) return mag + 7;
      break;
    case TX_CLASS_HORIZ:
      mag += levels[pos + (stride << 1)];
      mag = AOMMIN((mag + 1) >> 1, 6);
      if (c == 0) return mag;
      if (col == 0) return mag + 7;
      break;
    case TX_CLASS_VERT:
      mag += levels[pos + 2];
      mag = AOMMIN((mag + 1) >> 1, 6);
      if (c == 0) return mag;
      if (row == 0) return mag + 7;
      break;
    default: break;
  }

  return mag + 14;
}

static const uint8_t clip_max3[256] = {
  0, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3
};

static AOM_FORCE_INLINE int get_nz_mag(const uint8_t *const levels,
                                       const int bhl, const TX_CLASS tx_class) {
  int mag;

  // Note: AOMMIN(level, 3) is useless for decoder since level < 3.
  mag = clip_max3[levels[(1 << bhl) + TX_PAD_HOR]];  // { 0, 1 }
  mag += clip_max3[levels[1]];                       // { 1, 0 }

  if (tx_class == TX_CLASS_2D) {
    mag += clip_max3[levels[(1 << bhl) + TX_PAD_HOR + 1]];          // { 1, 1 }
    mag += clip_max3[levels[(2 << bhl) + (2 << TX_PAD_HOR_LOG2)]];  // { 0, 2 }
    mag += clip_max3[levels[2]];                                    // { 2, 0 }
  } else if (tx_class == TX_CLASS_VERT) {
    mag += clip_max3[levels[2]];  // { 2, 0 }
    mag += clip_max3[levels[3]];  // { 3, 0 }
    mag += clip_max3[levels[4]];  // { 4, 0 }
  } else {
    mag += clip_max3[levels[(2 << bhl) + (2 << TX_PAD_HOR_LOG2)]];  // { 0, 2 }
    mag += clip_max3[levels[(3 << bhl) + (3 << TX_PAD_HOR_LOG2)]];  // { 0, 3 }
    mag += clip_max3[levels[(4 << bhl) + (4 << TX_PAD_HOR_LOG2)]];  // { 0, 4 }
  }

  return mag;
}

#define NZ_MAP_CTX_0 SIG_COEF_CONTEXTS_2D
#define NZ_MAP_CTX_5 (NZ_MAP_CTX_0 + 5)
#define NZ_MAP_CTX_10 (NZ_MAP_CTX_0 + 10)

static const int nz_map_ctx_offset_1d[32] = {
  NZ_MAP_CTX_0,  NZ_MAP_CTX_5,  NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10,
  NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10,
  NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10,
  NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10,
  NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10,
  NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10, NZ_MAP_CTX_10,
  NZ_MAP_CTX_10, NZ_MAP_CTX_10,
};

static AOM_FORCE_INLINE int get_nz_map_ctx_from_stats(
    const int stats,
    const int coeff_idx,  // raster order
    const int bhl, const TX_SIZE tx_size, const TX_CLASS tx_class) {
  // tx_class == 0(TX_CLASS_2D)
  if ((tx_class | coeff_idx) == 0) return 0;
  int ctx = (stats + 1) >> 1;
  ctx = AOMMIN(ctx, 4);
  switch (tx_class) {
    case TX_CLASS_2D: {
      // This is the algorithm to generate av1_nz_map_ctx_offset[][]
      //   const int width = tx_size_wide[tx_size];
      //   const int height = tx_size_high[tx_size];
      //   if (width < height) {
      //     if (row < 2) return 11 + ctx;
      //   } else if (width > height) {
      //     if (col < 2) return 16 + ctx;
      //   }
      //   if (row + col < 2) return ctx + 1;
      //   if (row + col < 4) return 5 + ctx + 1;
      //   return 21 + ctx;
      return ctx + av1_nz_map_ctx_offset[tx_size][coeff_idx];
    }
    case TX_CLASS_HORIZ: {
      const int col = coeff_idx >> bhl;
      return ctx + nz_map_ctx_offset_1d[col];
    }
    case TX_CLASS_VERT: {
      const int col = coeff_idx >> bhl;
      const int row = coeff_idx - (col << bhl);
      return ctx + nz_map_ctx_offset_1d[row];
    }
    default: break;
  }
  return 0;
}

typedef aom_cdf_prob (*base_cdf_arr)[CDF_SIZE(4)];
typedef aom_cdf_prob (*br_cdf_arr)[CDF_SIZE(BR_CDF_SIZE)];

static inline int get_lower_levels_ctx_eob(int bhl, int width, int scan_idx) {
  if (scan_idx == 0) return 0;
  if (scan_idx <= (width << bhl) / 8) return 1;
  if (scan_idx <= (width << bhl) / 4) return 2;
  return 3;
}

static inline int get_lower_levels_ctx_2d(const uint8_t *levels, int coeff_idx,
                                          int bhl, TX_SIZE tx_size) {
  assert(coeff_idx > 0);
  int mag;
  // Note: AOMMIN(level, 3) is useless for decoder since level < 3.
  levels = levels + get_padded_idx(coeff_idx, bhl);
  mag = AOMMIN(levels[(1 << bhl) + TX_PAD_HOR], 3);               // { 0, 1 }
  mag += AOMMIN(levels[1], 3);                                    // { 1, 0 }
  mag += AOMMIN(levels[(1 << bhl) + TX_PAD_HOR + 1], 3);          // { 1, 1 }
  mag += AOMMIN(levels[(2 << bhl) + (2 << TX_PAD_HOR_LOG2)], 3);  // { 0, 2 }
  mag += AOMMIN(levels[2], 3);                                    // { 2, 0 }

  const int ctx = AOMMIN((mag + 1) >> 1, 4);
  return ctx + av1_nz_map_ctx_offset[tx_size][coeff_idx];
}
static AOM_FORCE_INLINE int get_lower_levels_ctx(const uint8_t *levels,
                                                 int coeff_idx, int bhl,
                                                 TX_SIZE tx_size,
                                                 TX_CLASS tx_class) {
  const int stats =
      get_nz_mag(levels + get_padded_idx(coeff_idx, bhl), bhl, tx_class);
  return get_nz_map_ctx_from_stats(stats, coeff_idx, bhl, tx_size, tx_class);
}

static inline int get_lower_levels_ctx_general(int is_last, int scan_idx,
                                               int bhl, int width,
                                               const uint8_t *levels,
                                               int coeff_idx, TX_SIZE tx_size,
                                               TX_CLASS tx_class) {
  if (is_last) {
    if (scan_idx == 0) return 0;
    if (scan_idx <= (width << bhl) >> 3) return 1;
    if (scan_idx <= (width << bhl) >> 2) return 2;
    return 3;
  }
  return get_lower_levels_ctx(levels, coeff_idx, bhl, tx_size, tx_class);
}

static inline void set_dc_sign(int *cul_level, int dc_val) {
  if (dc_val < 0)
    *cul_level |= 1 << COEFF_CONTEXT_BITS;
  else if (dc_val > 0)
    *cul_level += 2 << COEFF_CONTEXT_BITS;
}

static void get_txb_ctx_general(const BLOCK_SIZE plane_bsize,
                                const TX_SIZE tx_size, const int plane,
                                const ENTROPY_CONTEXT *const a,
                                const ENTROPY_CONTEXT *const l,
                                TXB_CTX *const txb_ctx) {
#define MAX_TX_SIZE_UNIT 16
  static const int8_t signs[3] = { 0, -1, 1 };
  static const int8_t dc_sign_contexts[4 * MAX_TX_SIZE_UNIT + 1] = {
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
  };
  const int txb_w_unit = tx_size_wide_unit[tx_size];
  const int txb_h_unit = tx_size_high_unit[tx_size];
  int dc_sign = 0;
  int k = 0;

  do {
    const unsigned int sign = ((uint8_t)a[k]) >> COEFF_CONTEXT_BITS;
    assert(sign <= 2);
    dc_sign += signs[sign];
  } while (++k < txb_w_unit);

  k = 0;
  do {
    const unsigned int sign = ((uint8_t)l[k]) >> COEFF_CONTEXT_BITS;
    assert(sign <= 2);
    dc_sign += signs[sign];
  } while (++k < txb_h_unit);

  txb_ctx->dc_sign_ctx = dc_sign_contexts[dc_sign + 2 * MAX_TX_SIZE_UNIT];

  if (plane == 0) {
    if (plane_bsize == txsize_to_bsize[tx_size]) {
      txb_ctx->txb_skip_ctx = 0;
    } else {
      // This is the algorithm to generate table skip_contexts[top][left].
      //    const int max = AOMMIN(top | left, 4);
      //    const int min = AOMMIN(AOMMIN(top, left), 4);
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
      static const uint8_t skip_contexts[5][5] = { { 1, 2, 2, 2, 3 },
                                                   { 2, 4, 4, 4, 5 },
                                                   { 2, 4, 4, 4, 5 },
                                                   { 2, 4, 4, 4, 5 },
                                                   { 3, 5, 5, 5, 6 } };
      // For top and left, we only care about which of the following three
      // categories they belong to: { 0 }, { 1, 2, 3 }, or { 4, 5, ... }. The
      // spec calculates top and left with the Max() function. We can calculate
      // an approximate max with bitwise OR because the real max and the
      // approximate max belong to the same category.
      int top = 0;
      int left = 0;

      k = 0;
      do {
        top |= a[k];
      } while (++k < txb_w_unit);
      top &= COEFF_CONTEXT_MASK;
      top = AOMMIN(top, 4);

      k = 0;
      do {
        left |= l[k];
      } while (++k < txb_h_unit);
      left &= COEFF_CONTEXT_MASK;
      left = AOMMIN(left, 4);

      txb_ctx->txb_skip_ctx = skip_contexts[top][left];
    }
  } else {
    const int ctx_base = get_entropy_context(tx_size, a, l);
    const int ctx_offset = (num_pels_log2_lookup[plane_bsize] >
                            num_pels_log2_lookup[txsize_to_bsize[tx_size]])
                               ? 10
                               : 7;
    txb_ctx->txb_skip_ctx = ctx_base + ctx_offset;
  }
}

#define SPECIALIZE_GET_TXB_CTX(w, h)                                          \
  static void get_txb_ctx_##w##x##h(                                          \
      const BLOCK_SIZE plane_bsize, const int plane,                          \
      const ENTROPY_CONTEXT *const a, const ENTROPY_CONTEXT *const l,         \
      TXB_CTX *const txb_ctx) {                                               \
    static const int8_t signs[3] = { 0, -1, 1 };                              \
    static const int8_t dc_sign_contexts[4 * MAX_TX_SIZE_UNIT + 1] = {        \
      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,       \
      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,       \
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2           \
    };                                                                        \
    const TX_SIZE tx_size = TX_##w##X##h;                                     \
    const int txb_w_unit = tx_size_wide_unit[tx_size];                        \
    const int txb_h_unit = tx_size_high_unit[tx_size];                        \
    int dc_sign = 0;                                                          \
    int k = 0;                                                                \
                                                                              \
    do {                                                                      \
      const unsigned int sign = ((uint8_t)a[k]) >> COEFF_CONTEXT_BITS;        \
      assert(sign <= 2);                                                      \
      dc_sign += signs[sign];                                                 \
    } while (++k < txb_w_unit);                                               \
                                                                              \
    k = 0;                                                                    \
    do {                                                                      \
      const unsigned int sign = ((uint8_t)l[k]) >> COEFF_CONTEXT_BITS;        \
      assert(sign <= 2);                                                      \
      dc_sign += signs[sign];                                                 \
    } while (++k < txb_h_unit);                                               \
                                                                              \
    txb_ctx->dc_sign_ctx = dc_sign_contexts[dc_sign + 2 * MAX_TX_SIZE_UNIT];  \
                                                                              \
    if (plane == 0) {                                                         \
      if (plane_bsize == txsize_to_bsize[tx_size]) {                          \
        txb_ctx->txb_skip_ctx = 0;                                            \
      } else {                                                                \
        static const uint8_t skip_contexts[5][5] = { { 1, 2, 2, 2, 3 },       \
                                                     { 2, 4, 4, 4, 5 },       \
                                                     { 2, 4, 4, 4, 5 },       \
                                                     { 2, 4, 4, 4, 5 },       \
                                                     { 3, 5, 5, 5, 6 } };     \
        int top = 0;                                                          \
        int left = 0;                                                         \
                                                                              \
        k = 0;                                                                \
        do {                                                                  \
          top |= a[k];                                                        \
        } while (++k < txb_w_unit);                                           \
        top &= COEFF_CONTEXT_MASK;                                            \
        top = AOMMIN(top, 4);                                                 \
                                                                              \
        k = 0;                                                                \
        do {                                                                  \
          left |= l[k];                                                       \
        } while (++k < txb_h_unit);                                           \
        left &= COEFF_CONTEXT_MASK;                                           \
        left = AOMMIN(left, 4);                                               \
                                                                              \
        txb_ctx->txb_skip_ctx = skip_contexts[top][left];                     \
      }                                                                       \
    } else {                                                                  \
      const int ctx_base = get_entropy_context(tx_size, a, l);                \
      const int ctx_offset = (num_pels_log2_lookup[plane_bsize] >             \
                              num_pels_log2_lookup[txsize_to_bsize[tx_size]]) \
                                 ? 10                                         \
                                 : 7;                                         \
      txb_ctx->txb_skip_ctx = ctx_base + ctx_offset;                          \
    }                                                                         \
  }

SPECIALIZE_GET_TXB_CTX(4, 4)
SPECIALIZE_GET_TXB_CTX(8, 8)
SPECIALIZE_GET_TXB_CTX(16, 16)
SPECIALIZE_GET_TXB_CTX(32, 32)

// Wrapper for get_txb_ctx that calls the specialized version of get_txb_ctc_*
// so that the compiler can compile away the while loops.
static inline void get_txb_ctx(const BLOCK_SIZE plane_bsize,
                               const TX_SIZE tx_size, const int plane,
                               const ENTROPY_CONTEXT *const a,
                               const ENTROPY_CONTEXT *const l,
                               TXB_CTX *const txb_ctx) {
  switch (tx_size) {
    case TX_4X4: get_txb_ctx_4x4(plane_bsize, plane, a, l, txb_ctx); break;
    case TX_8X8: get_txb_ctx_8x8(plane_bsize, plane, a, l, txb_ctx); break;
    case TX_16X16: get_txb_ctx_16x16(plane_bsize, plane, a, l, txb_ctx); break;
    case TX_32X32: get_txb_ctx_32x32(plane_bsize, plane, a, l, txb_ctx); break;
    default:
      get_txb_ctx_general(plane_bsize, tx_size, plane, a, l, txb_ctx);
      break;
  }
}
#undef MAX_TX_SIZE_UNIT

#endif  // AOM_AV1_COMMON_TXB_COMMON_H_
