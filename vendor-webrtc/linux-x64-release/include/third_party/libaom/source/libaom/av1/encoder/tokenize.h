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

#ifndef AOM_AV1_ENCODER_TOKENIZE_H_
#define AOM_AV1_ENCODER_TOKENIZE_H_

#include "av1/common/entropy.h"
#include "av1/encoder/block.h"
#include "aom_dsp/bitwriter.h"

#ifdef __cplusplus
extern "C" {
#endif

// The token and color_ctx members of the TokenExtra structure are used
// to store the indices of color and color context of each pixel in
// case of palette mode.
// 1) token can take values in the range of [0, 7] as maximum number of possible
// colors is 8 (PALETTE_COLORS). Hence token requires 3 bits (unsigned).
// 2) The reserved field (1-bit) is positioned such that color_ctx occupies the
// most significant bits and token occupies the least significant bits of the
// byte. Thus accesses to token and color_ctx are optimal. If TokenExtra is
// defined as:
//   typedef struct {
//     int8_t color_ctx : 4;
//     uint8_t token : 3;
//   } TokenExtra;
// then read of color_ctx requires an extra left shift to facilitate sign
// extension and write of token requires an extra masking.
// 3) color_ctx can take 5 (PALETTE_COLOR_INDEX_CONTEXTS) valid values, i.e.,
// from 0 to 4. As per the current implementation it can take values in the
// range of [-1, 4]. Here -1 corresponds to invalid color index context and is
// used for default initialization. Hence color_ctx requires 4 bits (signed).
typedef struct {
  uint8_t token : 3;
  uint8_t reserved : 1;
  int8_t color_ctx : 4;
} TokenExtra;

typedef struct {
  TokenExtra *start;
  unsigned int count;
} TokenList;

typedef struct {
  // Number of tile tokens for which memory is allocated.
  unsigned int tokens_allocated;
  // tile_tok[i][j] is a pointer to the buffer storing palette tokens of the ith
  // tile row, jth tile column.
  TokenExtra *tile_tok[MAX_TILE_ROWS][MAX_TILE_COLS];
  // tplist[i][j][k] holds the start pointer of tile_tok[i][j] and the count of
  // palette tokens for the kth superblock row of the ith tile row, jth tile
  // column.
  TokenList *tplist[MAX_TILE_ROWS][MAX_TILE_COLS];
} TokenInfo;

struct AV1_COMP;
struct ThreadData;
struct FRAME_COUNTS;

enum {
  OUTPUT_ENABLED = 0,
  DRY_RUN_NORMAL,
  DRY_RUN_COSTCOEFFS,
} UENUM1BYTE(RUN_TYPE);

struct tokenize_b_args {
  const struct AV1_COMP *cpi;
  struct ThreadData *td;
  int this_rate;
  uint8_t allow_update_cdf;
  RUN_TYPE dry_run;
};

// Note in all the tokenize functions rate if non NULL is incremented
// with the coefficient token cost only if dry_run = DRY_RUN_COSTCOEFS,
// otherwise rate is not incremented.
void av1_tokenize_sb_vartx(const struct AV1_COMP *cpi, struct ThreadData *td,
                           RUN_TYPE dry_run, BLOCK_SIZE bsize, int *rate,
                           uint8_t allow_update_cdf);

int av1_cost_color_map(const MACROBLOCK *const x, int plane, BLOCK_SIZE bsize,
                       TX_SIZE tx_size, COLOR_MAP_TYPE type);

void av1_tokenize_color_map(const MACROBLOCK *const x, int plane,
                            TokenExtra **t, BLOCK_SIZE bsize, TX_SIZE tx_size,
                            COLOR_MAP_TYPE type, int allow_update_cdf,
                            struct FRAME_COUNTS *counts);

static inline int av1_get_tx_eob(const struct segmentation *seg, int segment_id,
                                 TX_SIZE tx_size) {
  const int eob_max = av1_get_max_eob(tx_size);
  return segfeature_active(seg, segment_id, SEG_LVL_SKIP) ? 0 : eob_max;
}

// Token buffer is only used for palette tokens.
static inline unsigned int get_token_alloc(int mb_rows, int mb_cols,
                                           int sb_size_log2,
                                           const int num_planes) {
  // Calculate the maximum number of max superblocks in the image.
  const int shift = sb_size_log2 - 4;
  const int sb_size = 1 << sb_size_log2;
  const int sb_size_square = sb_size * sb_size;
  const int sb_rows = CEIL_POWER_OF_TWO(mb_rows, shift);
  const int sb_cols = CEIL_POWER_OF_TWO(mb_cols, shift);

  // One palette token for each pixel. There can be palettes on two planes.
  const int sb_palette_toks = AOMMIN(2, num_planes) * sb_size_square;

  return sb_rows * sb_cols * sb_palette_toks;
}

// Allocate memory for token related info.
static inline void alloc_token_info(AV1_COMMON *cm, TokenInfo *token_info,
                                    unsigned int tokens_required) {
  int sb_rows =
      CEIL_POWER_OF_TWO(cm->mi_params.mi_rows, cm->seq_params->mib_size_log2);
  token_info->tokens_allocated = tokens_required;

  CHECK_MEM_ERROR(cm, token_info->tile_tok[0][0],
                  (TokenExtra *)aom_calloc(
                      tokens_required, sizeof(*token_info->tile_tok[0][0])));

  CHECK_MEM_ERROR(
      cm, token_info->tplist[0][0],
      (TokenList *)aom_calloc(sb_rows * MAX_TILE_ROWS * MAX_TILE_COLS,
                              sizeof(*token_info->tplist[0][0])));
}

// Check if memory allocation has been done for token related info.
static inline bool is_token_info_allocated(const TokenInfo *token_info) {
  return ((token_info->tile_tok[0][0] != NULL) &&
          (token_info->tplist[0][0] != NULL));
}

// Free memory from token related variables.
static inline void free_token_info(TokenInfo *token_info) {
  aom_free(token_info->tile_tok[0][0]);
  token_info->tile_tok[0][0] = NULL;

  aom_free(token_info->tplist[0][0]);
  token_info->tplist[0][0] = NULL;

  token_info->tokens_allocated = 0;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_TOKENIZE_H_
