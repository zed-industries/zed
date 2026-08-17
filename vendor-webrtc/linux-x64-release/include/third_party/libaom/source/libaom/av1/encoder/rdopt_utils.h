/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_RDOPT_UTILS_H_
#define AOM_AV1_ENCODER_RDOPT_UTILS_H_

#include "aom/aom_integer.h"
#include "av1/encoder/block.h"
#include "av1/common/cfl.h"
#include "av1/common/pred_common.h"
#include "av1/encoder/rdopt_data_defs.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MAX_REF_MV_SEARCH 3
#define MAX_TX_RD_GATE_LEVEL 5
#define INTER_INTRA_RD_THRESH_SCALE 9
#define INTER_INTRA_RD_THRESH_SHIFT 4

typedef struct {
  PREDICTION_MODE mode;
  MV_REFERENCE_FRAME ref_frame[2];
} MODE_DEFINITION;

// This array defines the mapping from the enums in THR_MODES to the actual
// prediction modes and refrence frames
static const MODE_DEFINITION av1_mode_defs[MAX_MODES] = {
  { NEARESTMV, { LAST_FRAME, NONE_FRAME } },
  { NEARESTMV, { LAST2_FRAME, NONE_FRAME } },
  { NEARESTMV, { LAST3_FRAME, NONE_FRAME } },
  { NEARESTMV, { BWDREF_FRAME, NONE_FRAME } },
  { NEARESTMV, { ALTREF2_FRAME, NONE_FRAME } },
  { NEARESTMV, { ALTREF_FRAME, NONE_FRAME } },
  { NEARESTMV, { GOLDEN_FRAME, NONE_FRAME } },

  { NEWMV, { LAST_FRAME, NONE_FRAME } },
  { NEWMV, { LAST2_FRAME, NONE_FRAME } },
  { NEWMV, { LAST3_FRAME, NONE_FRAME } },
  { NEWMV, { BWDREF_FRAME, NONE_FRAME } },
  { NEWMV, { ALTREF2_FRAME, NONE_FRAME } },
  { NEWMV, { ALTREF_FRAME, NONE_FRAME } },
  { NEWMV, { GOLDEN_FRAME, NONE_FRAME } },

  { NEARMV, { LAST_FRAME, NONE_FRAME } },
  { NEARMV, { LAST2_FRAME, NONE_FRAME } },
  { NEARMV, { LAST3_FRAME, NONE_FRAME } },
  { NEARMV, { BWDREF_FRAME, NONE_FRAME } },
  { NEARMV, { ALTREF2_FRAME, NONE_FRAME } },
  { NEARMV, { ALTREF_FRAME, NONE_FRAME } },
  { NEARMV, { GOLDEN_FRAME, NONE_FRAME } },

  { GLOBALMV, { LAST_FRAME, NONE_FRAME } },
  { GLOBALMV, { LAST2_FRAME, NONE_FRAME } },
  { GLOBALMV, { LAST3_FRAME, NONE_FRAME } },
  { GLOBALMV, { BWDREF_FRAME, NONE_FRAME } },
  { GLOBALMV, { ALTREF2_FRAME, NONE_FRAME } },
  { GLOBALMV, { ALTREF_FRAME, NONE_FRAME } },
  { GLOBALMV, { GOLDEN_FRAME, NONE_FRAME } },

  // TODO(zoeliu): May need to reconsider the order on the modes to check

  { NEAREST_NEARESTMV, { LAST_FRAME, ALTREF_FRAME } },
  { NEAREST_NEARESTMV, { LAST2_FRAME, ALTREF_FRAME } },
  { NEAREST_NEARESTMV, { LAST3_FRAME, ALTREF_FRAME } },
  { NEAREST_NEARESTMV, { GOLDEN_FRAME, ALTREF_FRAME } },
  { NEAREST_NEARESTMV, { LAST_FRAME, BWDREF_FRAME } },
  { NEAREST_NEARESTMV, { LAST2_FRAME, BWDREF_FRAME } },
  { NEAREST_NEARESTMV, { LAST3_FRAME, BWDREF_FRAME } },
  { NEAREST_NEARESTMV, { GOLDEN_FRAME, BWDREF_FRAME } },
  { NEAREST_NEARESTMV, { LAST_FRAME, ALTREF2_FRAME } },
  { NEAREST_NEARESTMV, { LAST2_FRAME, ALTREF2_FRAME } },
  { NEAREST_NEARESTMV, { LAST3_FRAME, ALTREF2_FRAME } },
  { NEAREST_NEARESTMV, { GOLDEN_FRAME, ALTREF2_FRAME } },

  { NEAREST_NEARESTMV, { LAST_FRAME, LAST2_FRAME } },
  { NEAREST_NEARESTMV, { LAST_FRAME, LAST3_FRAME } },
  { NEAREST_NEARESTMV, { LAST_FRAME, GOLDEN_FRAME } },
  { NEAREST_NEARESTMV, { BWDREF_FRAME, ALTREF_FRAME } },

  { NEAR_NEARMV, { LAST_FRAME, BWDREF_FRAME } },
  { NEW_NEWMV, { LAST_FRAME, BWDREF_FRAME } },
  { NEW_NEARESTMV, { LAST_FRAME, BWDREF_FRAME } },
  { NEAREST_NEWMV, { LAST_FRAME, BWDREF_FRAME } },
  { NEW_NEARMV, { LAST_FRAME, BWDREF_FRAME } },
  { NEAR_NEWMV, { LAST_FRAME, BWDREF_FRAME } },
  { GLOBAL_GLOBALMV, { LAST_FRAME, BWDREF_FRAME } },

  { NEAR_NEARMV, { LAST_FRAME, ALTREF_FRAME } },
  { NEW_NEWMV, { LAST_FRAME, ALTREF_FRAME } },
  { NEW_NEARESTMV, { LAST_FRAME, ALTREF_FRAME } },
  { NEAREST_NEWMV, { LAST_FRAME, ALTREF_FRAME } },
  { NEW_NEARMV, { LAST_FRAME, ALTREF_FRAME } },
  { NEAR_NEWMV, { LAST_FRAME, ALTREF_FRAME } },
  { GLOBAL_GLOBALMV, { LAST_FRAME, ALTREF_FRAME } },

  { NEAR_NEARMV, { LAST2_FRAME, ALTREF_FRAME } },
  { NEW_NEWMV, { LAST2_FRAME, ALTREF_FRAME } },
  { NEW_NEARESTMV, { LAST2_FRAME, ALTREF_FRAME } },
  { NEAREST_NEWMV, { LAST2_FRAME, ALTREF_FRAME } },
  { NEW_NEARMV, { LAST2_FRAME, ALTREF_FRAME } },
  { NEAR_NEWMV, { LAST2_FRAME, ALTREF_FRAME } },
  { GLOBAL_GLOBALMV, { LAST2_FRAME, ALTREF_FRAME } },

  { NEAR_NEARMV, { LAST3_FRAME, ALTREF_FRAME } },
  { NEW_NEWMV, { LAST3_FRAME, ALTREF_FRAME } },
  { NEW_NEARESTMV, { LAST3_FRAME, ALTREF_FRAME } },
  { NEAREST_NEWMV, { LAST3_FRAME, ALTREF_FRAME } },
  { NEW_NEARMV, { LAST3_FRAME, ALTREF_FRAME } },
  { NEAR_NEWMV, { LAST3_FRAME, ALTREF_FRAME } },
  { GLOBAL_GLOBALMV, { LAST3_FRAME, ALTREF_FRAME } },

  { NEAR_NEARMV, { GOLDEN_FRAME, ALTREF_FRAME } },
  { NEW_NEWMV, { GOLDEN_FRAME, ALTREF_FRAME } },
  { NEW_NEARESTMV, { GOLDEN_FRAME, ALTREF_FRAME } },
  { NEAREST_NEWMV, { GOLDEN_FRAME, ALTREF_FRAME } },
  { NEW_NEARMV, { GOLDEN_FRAME, ALTREF_FRAME } },
  { NEAR_NEWMV, { GOLDEN_FRAME, ALTREF_FRAME } },
  { GLOBAL_GLOBALMV, { GOLDEN_FRAME, ALTREF_FRAME } },

  { NEAR_NEARMV, { LAST2_FRAME, BWDREF_FRAME } },
  { NEW_NEWMV, { LAST2_FRAME, BWDREF_FRAME } },
  { NEW_NEARESTMV, { LAST2_FRAME, BWDREF_FRAME } },
  { NEAREST_NEWMV, { LAST2_FRAME, BWDREF_FRAME } },
  { NEW_NEARMV, { LAST2_FRAME, BWDREF_FRAME } },
  { NEAR_NEWMV, { LAST2_FRAME, BWDREF_FRAME } },
  { GLOBAL_GLOBALMV, { LAST2_FRAME, BWDREF_FRAME } },

  { NEAR_NEARMV, { LAST3_FRAME, BWDREF_FRAME } },
  { NEW_NEWMV, { LAST3_FRAME, BWDREF_FRAME } },
  { NEW_NEARESTMV, { LAST3_FRAME, BWDREF_FRAME } },
  { NEAREST_NEWMV, { LAST3_FRAME, BWDREF_FRAME } },
  { NEW_NEARMV, { LAST3_FRAME, BWDREF_FRAME } },
  { NEAR_NEWMV, { LAST3_FRAME, BWDREF_FRAME } },
  { GLOBAL_GLOBALMV, { LAST3_FRAME, BWDREF_FRAME } },

  { NEAR_NEARMV, { GOLDEN_FRAME, BWDREF_FRAME } },
  { NEW_NEWMV, { GOLDEN_FRAME, BWDREF_FRAME } },
  { NEW_NEARESTMV, { GOLDEN_FRAME, BWDREF_FRAME } },
  { NEAREST_NEWMV, { GOLDEN_FRAME, BWDREF_FRAME } },
  { NEW_NEARMV, { GOLDEN_FRAME, BWDREF_FRAME } },
  { NEAR_NEWMV, { GOLDEN_FRAME, BWDREF_FRAME } },
  { GLOBAL_GLOBALMV, { GOLDEN_FRAME, BWDREF_FRAME } },

  { NEAR_NEARMV, { LAST_FRAME, ALTREF2_FRAME } },
  { NEW_NEWMV, { LAST_FRAME, ALTREF2_FRAME } },
  { NEW_NEARESTMV, { LAST_FRAME, ALTREF2_FRAME } },
  { NEAREST_NEWMV, { LAST_FRAME, ALTREF2_FRAME } },
  { NEW_NEARMV, { LAST_FRAME, ALTREF2_FRAME } },
  { NEAR_NEWMV, { LAST_FRAME, ALTREF2_FRAME } },
  { GLOBAL_GLOBALMV, { LAST_FRAME, ALTREF2_FRAME } },

  { NEAR_NEARMV, { LAST2_FRAME, ALTREF2_FRAME } },
  { NEW_NEWMV, { LAST2_FRAME, ALTREF2_FRAME } },
  { NEW_NEARESTMV, { LAST2_FRAME, ALTREF2_FRAME } },
  { NEAREST_NEWMV, { LAST2_FRAME, ALTREF2_FRAME } },
  { NEW_NEARMV, { LAST2_FRAME, ALTREF2_FRAME } },
  { NEAR_NEWMV, { LAST2_FRAME, ALTREF2_FRAME } },
  { GLOBAL_GLOBALMV, { LAST2_FRAME, ALTREF2_FRAME } },

  { NEAR_NEARMV, { LAST3_FRAME, ALTREF2_FRAME } },
  { NEW_NEWMV, { LAST3_FRAME, ALTREF2_FRAME } },
  { NEW_NEARESTMV, { LAST3_FRAME, ALTREF2_FRAME } },
  { NEAREST_NEWMV, { LAST3_FRAME, ALTREF2_FRAME } },
  { NEW_NEARMV, { LAST3_FRAME, ALTREF2_FRAME } },
  { NEAR_NEWMV, { LAST3_FRAME, ALTREF2_FRAME } },
  { GLOBAL_GLOBALMV, { LAST3_FRAME, ALTREF2_FRAME } },

  { NEAR_NEARMV, { GOLDEN_FRAME, ALTREF2_FRAME } },
  { NEW_NEWMV, { GOLDEN_FRAME, ALTREF2_FRAME } },
  { NEW_NEARESTMV, { GOLDEN_FRAME, ALTREF2_FRAME } },
  { NEAREST_NEWMV, { GOLDEN_FRAME, ALTREF2_FRAME } },
  { NEW_NEARMV, { GOLDEN_FRAME, ALTREF2_FRAME } },
  { NEAR_NEWMV, { GOLDEN_FRAME, ALTREF2_FRAME } },
  { GLOBAL_GLOBALMV, { GOLDEN_FRAME, ALTREF2_FRAME } },

  { NEAR_NEARMV, { LAST_FRAME, LAST2_FRAME } },
  { NEW_NEWMV, { LAST_FRAME, LAST2_FRAME } },
  { NEW_NEARESTMV, { LAST_FRAME, LAST2_FRAME } },
  { NEAREST_NEWMV, { LAST_FRAME, LAST2_FRAME } },
  { NEW_NEARMV, { LAST_FRAME, LAST2_FRAME } },
  { NEAR_NEWMV, { LAST_FRAME, LAST2_FRAME } },
  { GLOBAL_GLOBALMV, { LAST_FRAME, LAST2_FRAME } },

  { NEAR_NEARMV, { LAST_FRAME, LAST3_FRAME } },
  { NEW_NEWMV, { LAST_FRAME, LAST3_FRAME } },
  { NEW_NEARESTMV, { LAST_FRAME, LAST3_FRAME } },
  { NEAREST_NEWMV, { LAST_FRAME, LAST3_FRAME } },
  { NEW_NEARMV, { LAST_FRAME, LAST3_FRAME } },
  { NEAR_NEWMV, { LAST_FRAME, LAST3_FRAME } },
  { GLOBAL_GLOBALMV, { LAST_FRAME, LAST3_FRAME } },

  { NEAR_NEARMV, { LAST_FRAME, GOLDEN_FRAME } },
  { NEW_NEWMV, { LAST_FRAME, GOLDEN_FRAME } },
  { NEW_NEARESTMV, { LAST_FRAME, GOLDEN_FRAME } },
  { NEAREST_NEWMV, { LAST_FRAME, GOLDEN_FRAME } },
  { NEW_NEARMV, { LAST_FRAME, GOLDEN_FRAME } },
  { NEAR_NEWMV, { LAST_FRAME, GOLDEN_FRAME } },
  { GLOBAL_GLOBALMV, { LAST_FRAME, GOLDEN_FRAME } },

  { NEAR_NEARMV, { BWDREF_FRAME, ALTREF_FRAME } },
  { NEW_NEWMV, { BWDREF_FRAME, ALTREF_FRAME } },
  { NEW_NEARESTMV, { BWDREF_FRAME, ALTREF_FRAME } },
  { NEAREST_NEWMV, { BWDREF_FRAME, ALTREF_FRAME } },
  { NEW_NEARMV, { BWDREF_FRAME, ALTREF_FRAME } },
  { NEAR_NEWMV, { BWDREF_FRAME, ALTREF_FRAME } },
  { GLOBAL_GLOBALMV, { BWDREF_FRAME, ALTREF_FRAME } },

  // intra modes
  { DC_PRED, { INTRA_FRAME, NONE_FRAME } },
  { PAETH_PRED, { INTRA_FRAME, NONE_FRAME } },
  { SMOOTH_PRED, { INTRA_FRAME, NONE_FRAME } },
  { SMOOTH_V_PRED, { INTRA_FRAME, NONE_FRAME } },
  { SMOOTH_H_PRED, { INTRA_FRAME, NONE_FRAME } },
  { H_PRED, { INTRA_FRAME, NONE_FRAME } },
  { V_PRED, { INTRA_FRAME, NONE_FRAME } },
  { D135_PRED, { INTRA_FRAME, NONE_FRAME } },
  { D203_PRED, { INTRA_FRAME, NONE_FRAME } },
  { D157_PRED, { INTRA_FRAME, NONE_FRAME } },
  { D67_PRED, { INTRA_FRAME, NONE_FRAME } },
  { D113_PRED, { INTRA_FRAME, NONE_FRAME } },
  { D45_PRED, { INTRA_FRAME, NONE_FRAME } },
};

// Number of winner modes allowed for different values of the speed feature
// multi_winner_mode_type.
static const int winner_mode_count_allowed[MULTI_WINNER_MODE_LEVELS] = {
  1,  // MULTI_WINNER_MODE_OFF
  2,  // MULTI_WINNER_MODE_FAST
  3   // MULTI_WINNER_MODE_DEFAULT
};

static inline void restore_dst_buf(MACROBLOCKD *xd, const BUFFER_SET dst,
                                   const int num_planes) {
  for (int i = 0; i < num_planes; i++) {
    xd->plane[i].dst.buf = dst.plane[i];
    xd->plane[i].dst.stride = dst.stride[i];
  }
}

static inline void swap_dst_buf(MACROBLOCKD *xd, const BUFFER_SET *dst_bufs[2],
                                int num_planes) {
  const BUFFER_SET *buf0 = dst_bufs[0];
  dst_bufs[0] = dst_bufs[1];
  dst_bufs[1] = buf0;
  restore_dst_buf(xd, *dst_bufs[0], num_planes);
}

/* clang-format on */
// Calculate rd threshold based on ref best rd and relevant scaling factors
static inline int64_t get_rd_thresh_from_best_rd(int64_t ref_best_rd,
                                                 int mul_factor,
                                                 int div_factor) {
  int64_t rd_thresh = ref_best_rd;
  if (div_factor != 0) {
    rd_thresh = ref_best_rd < (div_factor * (INT64_MAX / mul_factor))
                    ? ((ref_best_rd / div_factor) * mul_factor)
                    : INT64_MAX;
  }
  return rd_thresh;
}

static inline THR_MODES get_prediction_mode_idx(
    PREDICTION_MODE this_mode, MV_REFERENCE_FRAME ref_frame,
    MV_REFERENCE_FRAME second_ref_frame) {
  if (this_mode < INTRA_MODE_END) {
    assert(ref_frame == INTRA_FRAME);
    assert(second_ref_frame == NONE_FRAME);
    return intra_to_mode_idx[this_mode - INTRA_MODE_START];
  }
  if (this_mode >= SINGLE_INTER_MODE_START &&
      this_mode < SINGLE_INTER_MODE_END) {
    assert((ref_frame > INTRA_FRAME) && (ref_frame <= ALTREF_FRAME));
    return single_inter_to_mode_idx[this_mode - SINGLE_INTER_MODE_START]
                                   [ref_frame];
  }
  if (this_mode >= COMP_INTER_MODE_START && this_mode < COMP_INTER_MODE_END &&
      second_ref_frame != NONE_FRAME) {
    assert((ref_frame > INTRA_FRAME) && (ref_frame <= ALTREF_FRAME));
    assert((second_ref_frame > INTRA_FRAME) &&
           (second_ref_frame <= ALTREF_FRAME));
    return comp_inter_to_mode_idx[this_mode - COMP_INTER_MODE_START][ref_frame]
                                 [second_ref_frame];
  }
  assert(0);
  return THR_INVALID;
}

static inline int inter_mode_data_block_idx(BLOCK_SIZE bsize) {
  if (bsize == BLOCK_4X4 || bsize == BLOCK_4X8 || bsize == BLOCK_8X4 ||
      bsize == BLOCK_4X16 || bsize == BLOCK_16X4) {
    return -1;
  }
  return 1;
}

// Get transform block visible dimensions cropped to the MI units.
static inline void get_txb_dimensions(const MACROBLOCKD *xd, int plane,
                                      BLOCK_SIZE plane_bsize, int blk_row,
                                      int blk_col, BLOCK_SIZE tx_bsize,
                                      int *width, int *height,
                                      int *visible_width, int *visible_height) {
  assert(tx_bsize <= plane_bsize);
  const int txb_height = block_size_high[tx_bsize];
  const int txb_width = block_size_wide[tx_bsize];
  const struct macroblockd_plane *const pd = &xd->plane[plane];

  // TODO(aconverse@google.com): Investigate using crop_width/height here rather
  // than the MI size
  if (xd->mb_to_bottom_edge >= 0) {
    *visible_height = txb_height;
  } else {
    const int block_height = block_size_high[plane_bsize];
    const int block_rows =
        (xd->mb_to_bottom_edge >> (3 + pd->subsampling_y)) + block_height;
    *visible_height =
        clamp(block_rows - (blk_row << MI_SIZE_LOG2), 0, txb_height);
  }
  if (height) *height = txb_height;

  if (xd->mb_to_right_edge >= 0) {
    *visible_width = txb_width;
  } else {
    const int block_width = block_size_wide[plane_bsize];
    const int block_cols =
        (xd->mb_to_right_edge >> (3 + pd->subsampling_x)) + block_width;
    *visible_width =
        clamp(block_cols - (blk_col << MI_SIZE_LOG2), 0, txb_width);
  }
  if (width) *width = txb_width;
}

static inline int bsize_to_num_blk(BLOCK_SIZE bsize) {
  int num_blk = 1 << (num_pels_log2_lookup[bsize] - 2 * MI_SIZE_LOG2);
  return num_blk;
}

static inline int check_txfm_eval(MACROBLOCK *const x, BLOCK_SIZE bsize,
                                  int64_t best_skip_rd, int64_t skip_rd,
                                  int level, int is_luma_only) {
  int eval_txfm = 1;
  // Derive aggressiveness factor for gating the transform search
  // Lower value indicates more aggressiveness. Be more conservative (high
  // value) for (i) low quantizers (ii) regions where prediction is poor
  const int scale[MAX_TX_RD_GATE_LEVEL + 1] = { INT_MAX, 4, 3, 2, 2, 1 };
  const int qslope = 2 * (!is_luma_only);
  const int level_to_qindex_map[MAX_TX_RD_GATE_LEVEL + 1] = { 0,  0,   0,
                                                              80, 100, 140 };
  int aggr_factor = 4;
  assert(level <= MAX_TX_RD_GATE_LEVEL);
  const int pred_qindex_thresh = level_to_qindex_map[level];
  if (!is_luma_only && level <= 2) {
    aggr_factor = 4 * AOMMAX(1, ROUND_POWER_OF_TWO((MAXQ - x->qindex) * qslope,
                                                   QINDEX_BITS));
  }
  if ((best_skip_rd >
       (x->source_variance << (num_pels_log2_lookup[bsize] + RDDIV_BITS))) &&
      (x->qindex >= pred_qindex_thresh))
    aggr_factor *= scale[level];
  // For level setting 1, be more conservative for non-luma-only case even when
  // prediction is good.
  else if ((level <= 1) && !is_luma_only)
    aggr_factor = (aggr_factor >> 2) * 6;

  // Be more conservative for luma only cases (called from compound type rd)
  // since best_skip_rd is computed after and skip_rd is computed (with 8-bit
  // prediction signals blended for WEDGE/DIFFWTD rather than 16-bit) before
  // interpolation filter search
  const int luma_mul[MAX_TX_RD_GATE_LEVEL + 1] = {
    INT_MAX, 32, 29, 17, 17, 17
  };
  int mul_factor = is_luma_only ? luma_mul[level] : 16;
  int64_t rd_thresh =
      (best_skip_rd == INT64_MAX)
          ? best_skip_rd
          : (int64_t)(best_skip_rd * aggr_factor * mul_factor >> 6);
  if (skip_rd > rd_thresh) eval_txfm = 0;
  return eval_txfm;
}

static TX_MODE select_tx_mode(
    const AV1_COMMON *cm, const TX_SIZE_SEARCH_METHOD tx_size_search_method) {
  if (cm->features.coded_lossless) return ONLY_4X4;
  if (tx_size_search_method == USE_LARGESTALL) {
    return TX_MODE_LARGEST;
  } else {
    assert(tx_size_search_method == USE_FULL_RD ||
           tx_size_search_method == USE_FAST_RD);
    return TX_MODE_SELECT;
  }
}

// Checks the conditions to disable winner mode processing
static inline int bypass_winner_mode_processing(const MACROBLOCK *const x,
                                                const SPEED_FEATURES *sf,
                                                int use_txfm_skip,
                                                int actual_txfm_skip,
                                                PREDICTION_MODE best_mode) {
  const int prune_winner_mode_eval_level =
      sf->winner_mode_sf.prune_winner_mode_eval_level;

  // Disable winner mode processing for blocks with low source variance.
  // The aggressiveness of this pruning logic reduces as qindex increases.
  // The threshold decreases linearly from 64 as qindex varies from 0 to 255.
  if (prune_winner_mode_eval_level == 1) {
    const unsigned int src_var_thresh = 64 - 48 * x->qindex / (MAXQ + 1);
    if (x->source_variance < src_var_thresh) return 1;
  } else if (prune_winner_mode_eval_level == 2) {
    // Skip winner mode processing of blocks for which transform turns out to be
    // skip due to nature of eob alone except NEWMV mode.
    if (!have_newmv_in_inter_mode(best_mode) && actual_txfm_skip) return 1;
  } else if (prune_winner_mode_eval_level == 3) {
    // Skip winner mode processing of blocks for which transform turns out to be
    // skip except NEWMV mode and considered based on the quantizer.
    // At high quantizers: Take conservative approach by considering transform
    // skip based on eob alone.
    // At low quantizers: Consider transform skip based on eob nature or RD cost
    // evaluation.
    const int is_txfm_skip =
        x->qindex > 127 ? actual_txfm_skip : actual_txfm_skip || use_txfm_skip;

    if (!have_newmv_in_inter_mode(best_mode) && is_txfm_skip) return 1;
  } else if (prune_winner_mode_eval_level >= 4) {
    // Do not skip winner mode evaluation at low quantizers if normal mode's
    // transform search was too aggressive.
    if (sf->rd_sf.perform_coeff_opt >= 5 && x->qindex <= 70) return 0;

    if (use_txfm_skip || actual_txfm_skip) return 1;
  }

  return 0;
}

// Checks the conditions to enable winner mode processing
static inline int is_winner_mode_processing_enabled(const struct AV1_COMP *cpi,
                                                    const MACROBLOCK *const x,
                                                    MB_MODE_INFO *const mbmi,
                                                    int actual_txfm_skip) {
  const SPEED_FEATURES *sf = &cpi->sf;
  const PREDICTION_MODE best_mode = mbmi->mode;

  if (bypass_winner_mode_processing(x, sf, mbmi->skip_txfm, actual_txfm_skip,
                                    best_mode))
    return 0;

  // TODO(any): Move block independent condition checks to frame level
  if (is_inter_block(mbmi)) {
    if (is_inter_mode(best_mode) &&
        (sf->tx_sf.tx_type_search.fast_inter_tx_type_prob_thresh != INT_MAX) &&
        !cpi->oxcf.txfm_cfg.use_inter_dct_only)
      return 1;
  } else {
    if (sf->tx_sf.tx_type_search.fast_intra_tx_type_search &&
        !cpi->oxcf.txfm_cfg.use_intra_default_tx_only &&
        !cpi->oxcf.txfm_cfg.use_intra_dct_only)
      return 1;
  }

  // Check speed feature related to winner mode processing
  if (sf->winner_mode_sf.enable_winner_mode_for_coeff_opt &&
      cpi->optimize_seg_arr[mbmi->segment_id] != NO_TRELLIS_OPT &&
      cpi->optimize_seg_arr[mbmi->segment_id] != FINAL_PASS_TRELLIS_OPT)
    return 1;
  if (sf->winner_mode_sf.enable_winner_mode_for_tx_size_srch) return 1;

  return 0;
}

static inline void set_tx_size_search_method(
    const AV1_COMMON *cm, const WinnerModeParams *winner_mode_params,
    TxfmSearchParams *txfm_params, int enable_winner_mode_for_tx_size_srch,
    int is_winner_mode) {
  // Populate transform size search method/transform mode appropriately
  txfm_params->tx_size_search_method =
      winner_mode_params->tx_size_search_methods[DEFAULT_EVAL];
  if (enable_winner_mode_for_tx_size_srch) {
    if (is_winner_mode)
      txfm_params->tx_size_search_method =
          winner_mode_params->tx_size_search_methods[WINNER_MODE_EVAL];
    else
      txfm_params->tx_size_search_method =
          winner_mode_params->tx_size_search_methods[MODE_EVAL];
  }
  txfm_params->tx_mode_search_type =
      select_tx_mode(cm, txfm_params->tx_size_search_method);
}

static inline void set_tx_type_prune(const SPEED_FEATURES *sf,
                                     TxfmSearchParams *txfm_params,
                                     int winner_mode_tx_type_pruning,
                                     int is_winner_mode) {
  // Populate prune transform mode appropriately
  txfm_params->prune_2d_txfm_mode = sf->tx_sf.tx_type_search.prune_2d_txfm_mode;
  if (!winner_mode_tx_type_pruning) return;

  const int prune_mode[4][2] = { { TX_TYPE_PRUNE_3, TX_TYPE_PRUNE_0 },
                                 { TX_TYPE_PRUNE_4, TX_TYPE_PRUNE_0 },
                                 { TX_TYPE_PRUNE_5, TX_TYPE_PRUNE_2 },
                                 { TX_TYPE_PRUNE_5, TX_TYPE_PRUNE_3 } };
  txfm_params->prune_2d_txfm_mode =
      prune_mode[winner_mode_tx_type_pruning - 1][is_winner_mode];
}

static inline void set_tx_domain_dist_params(
    const WinnerModeParams *winner_mode_params, TxfmSearchParams *txfm_params,
    int enable_winner_mode_for_tx_domain_dist, int is_winner_mode) {
  if (txfm_params->use_qm_dist_metric) {
    // QM-weighted PSNR is computed in transform space, so we need to forcibly
    // enable the use of tx domain distortion.
    txfm_params->use_transform_domain_distortion = 1;
    txfm_params->tx_domain_dist_threshold = 0;
    return;
  }

  if (!enable_winner_mode_for_tx_domain_dist) {
    txfm_params->use_transform_domain_distortion =
        winner_mode_params->use_transform_domain_distortion[DEFAULT_EVAL];
    txfm_params->tx_domain_dist_threshold =
        winner_mode_params->tx_domain_dist_threshold[DEFAULT_EVAL];
    return;
  }

  if (is_winner_mode) {
    txfm_params->use_transform_domain_distortion =
        winner_mode_params->use_transform_domain_distortion[WINNER_MODE_EVAL];
    txfm_params->tx_domain_dist_threshold =
        winner_mode_params->tx_domain_dist_threshold[WINNER_MODE_EVAL];
  } else {
    txfm_params->use_transform_domain_distortion =
        winner_mode_params->use_transform_domain_distortion[MODE_EVAL];
    txfm_params->tx_domain_dist_threshold =
        winner_mode_params->tx_domain_dist_threshold[MODE_EVAL];
  }
}

// This function sets mode parameters for different mode evaluation stages
static inline void set_mode_eval_params(const struct AV1_COMP *cpi,
                                        MACROBLOCK *x,
                                        MODE_EVAL_TYPE mode_eval_type) {
  const AV1_COMMON *cm = &cpi->common;
  const SPEED_FEATURES *sf = &cpi->sf;
  const WinnerModeParams *winner_mode_params = &cpi->winner_mode_params;
  TxfmSearchParams *txfm_params = &x->txfm_search_params;

  txfm_params->use_qm_dist_metric =
      cpi->oxcf.tune_cfg.dist_metric == AOM_DIST_METRIC_QM_PSNR;

  switch (mode_eval_type) {
    case DEFAULT_EVAL:
      txfm_params->default_inter_tx_type_prob_thresh = INT_MAX;
      txfm_params->use_default_intra_tx_type = 0;
      txfm_params->skip_txfm_level =
          winner_mode_params->skip_txfm_level[DEFAULT_EVAL];
      txfm_params->predict_dc_level =
          winner_mode_params->predict_dc_level[DEFAULT_EVAL];
      // Set default transform domain distortion type
      set_tx_domain_dist_params(winner_mode_params, txfm_params, 0, 0);

      // Get default threshold for R-D optimization of coefficients
      get_rd_opt_coeff_thresh(winner_mode_params->coeff_opt_thresholds,
                              txfm_params, 0, 0);

      // Set default transform size search method
      set_tx_size_search_method(cm, winner_mode_params, txfm_params, 0, 0);
      // Set default transform type prune
      set_tx_type_prune(sf, txfm_params, 0, 0);
      break;
    case MODE_EVAL:
      txfm_params->use_default_intra_tx_type =
          (cpi->sf.tx_sf.tx_type_search.fast_intra_tx_type_search ||
           cpi->oxcf.txfm_cfg.use_intra_default_tx_only);
      txfm_params->default_inter_tx_type_prob_thresh =
          cpi->sf.tx_sf.tx_type_search.fast_inter_tx_type_prob_thresh;
      txfm_params->skip_txfm_level =
          winner_mode_params->skip_txfm_level[MODE_EVAL];
      txfm_params->predict_dc_level =
          winner_mode_params->predict_dc_level[MODE_EVAL];
      // Set transform domain distortion type for mode evaluation
      set_tx_domain_dist_params(
          winner_mode_params, txfm_params,
          sf->winner_mode_sf.enable_winner_mode_for_use_tx_domain_dist, 0);

      // Get threshold for R-D optimization of coefficients during mode
      // evaluation
      get_rd_opt_coeff_thresh(
          winner_mode_params->coeff_opt_thresholds, txfm_params,
          sf->winner_mode_sf.enable_winner_mode_for_coeff_opt, 0);

      // Set the transform size search method for mode evaluation
      set_tx_size_search_method(
          cm, winner_mode_params, txfm_params,
          sf->winner_mode_sf.enable_winner_mode_for_tx_size_srch, 0);
      // Set transform type prune for mode evaluation
      set_tx_type_prune(sf, txfm_params,
                        sf->tx_sf.tx_type_search.winner_mode_tx_type_pruning,
                        0);
      break;
    case WINNER_MODE_EVAL:
      txfm_params->default_inter_tx_type_prob_thresh = INT_MAX;
      txfm_params->use_default_intra_tx_type = 0;
      txfm_params->skip_txfm_level =
          winner_mode_params->skip_txfm_level[WINNER_MODE_EVAL];
      txfm_params->predict_dc_level =
          winner_mode_params->predict_dc_level[WINNER_MODE_EVAL];

      // Set transform domain distortion type for winner mode evaluation
      set_tx_domain_dist_params(
          winner_mode_params, txfm_params,
          sf->winner_mode_sf.enable_winner_mode_for_use_tx_domain_dist, 1);

      // Get threshold for R-D optimization of coefficients for winner mode
      // evaluation
      get_rd_opt_coeff_thresh(
          winner_mode_params->coeff_opt_thresholds, txfm_params,
          sf->winner_mode_sf.enable_winner_mode_for_coeff_opt, 1);

      // Set the transform size search method for winner mode evaluation
      set_tx_size_search_method(
          cm, winner_mode_params, txfm_params,
          sf->winner_mode_sf.enable_winner_mode_for_tx_size_srch, 1);
      // Set default transform type prune mode for winner mode evaluation
      set_tx_type_prune(sf, txfm_params,
                        sf->tx_sf.tx_type_search.winner_mode_tx_type_pruning,
                        1);
      break;
    default: assert(0);
  }

  // Rd record collected at a specific mode evaluation stage can not be used
  // across other evaluation stages as the transform parameters are different.
  // Hence, reset mb rd record whenever mode evaluation stage type changes.
  if (txfm_params->mode_eval_type != mode_eval_type)
    reset_mb_rd_record(x->txfm_search_info.mb_rd_record);

  txfm_params->mode_eval_type = mode_eval_type;
}

// Similar to store_cfl_required(), but for use during the RDO process,
// where we haven't yet determined whether this block uses CfL.
static inline CFL_ALLOWED_TYPE store_cfl_required_rdo(const AV1_COMMON *cm,
                                                      const MACROBLOCK *x) {
  const MACROBLOCKD *xd = &x->e_mbd;

  if (cm->seq_params->monochrome || !xd->is_chroma_ref) return CFL_DISALLOWED;

  if (!xd->is_chroma_ref) {
    // For non-chroma-reference blocks, we should always store the luma pixels,
    // in case the corresponding chroma-reference block uses CfL.
    // Note that this can only happen for block sizes which are <8 on
    // their shortest side, as otherwise they would be chroma reference
    // blocks.
    return CFL_ALLOWED;
  }

  // For chroma reference blocks, we should store data in the encoder iff we're
  // allowed to try out CfL.
  return is_cfl_allowed(xd);
}

static inline void init_sbuv_mode(MB_MODE_INFO *const mbmi) {
  mbmi->uv_mode = UV_DC_PRED;
  mbmi->palette_mode_info.palette_size[1] = 0;
}

// Store best mode stats for winner mode processing
static inline void store_winner_mode_stats(
    const AV1_COMMON *const cm, MACROBLOCK *x, const MB_MODE_INFO *mbmi,
    RD_STATS *rd_cost, RD_STATS *rd_cost_y, RD_STATS *rd_cost_uv,
    THR_MODES mode_index, uint8_t *color_map, BLOCK_SIZE bsize, int64_t this_rd,
    int multi_winner_mode_type, int txfm_search_done) {
  WinnerModeStats *winner_mode_stats = x->winner_mode_stats;
  int mode_idx = 0;
  int is_palette_mode = mbmi->palette_mode_info.palette_size[PLANE_TYPE_Y] > 0;
  // Mode stat is not required when multiwinner mode processing is disabled
  if (multi_winner_mode_type == MULTI_WINNER_MODE_OFF) return;
  // Ignore mode with maximum rd
  if (this_rd == INT64_MAX) return;
  // TODO(any): Winner mode processing is currently not applicable for palette
  // mode in Inter frames. Clean-up the following code, once support is added
  if (!frame_is_intra_only(cm) && is_palette_mode) return;

  int max_winner_mode_count = winner_mode_count_allowed[multi_winner_mode_type];
  assert(x->winner_mode_count >= 0 &&
         x->winner_mode_count <= max_winner_mode_count);

  if (x->winner_mode_count) {
    // Find the mode which has higher rd cost than this_rd
    for (mode_idx = 0; mode_idx < x->winner_mode_count; mode_idx++)
      if (winner_mode_stats[mode_idx].rd > this_rd) break;

    if (mode_idx == max_winner_mode_count) {
      // No mode has higher rd cost than this_rd
      return;
    } else if (mode_idx < max_winner_mode_count - 1) {
      // Create a slot for current mode and move others to the next slot
      memmove(
          &winner_mode_stats[mode_idx + 1], &winner_mode_stats[mode_idx],
          (max_winner_mode_count - mode_idx - 1) * sizeof(*winner_mode_stats));
    }
  }
  // Add a mode stat for winner mode processing
  winner_mode_stats[mode_idx].mbmi = *mbmi;
  winner_mode_stats[mode_idx].rd = this_rd;
  winner_mode_stats[mode_idx].mode_index = mode_index;

  // Update rd stats required for inter frame
  if (!frame_is_intra_only(cm) && rd_cost && rd_cost_y && rd_cost_uv) {
    const MACROBLOCKD *xd = &x->e_mbd;
    const int skip_ctx = av1_get_skip_txfm_context(xd);
    const int is_intra_mode = av1_mode_defs[mode_index].mode < INTRA_MODE_END;
    const int skip_txfm = mbmi->skip_txfm && !is_intra_mode;

    winner_mode_stats[mode_idx].rd_cost = *rd_cost;
    if (txfm_search_done) {
      winner_mode_stats[mode_idx].rate_y =
          rd_cost_y->rate +
          x->mode_costs
              .skip_txfm_cost[skip_ctx][rd_cost->skip_txfm || skip_txfm];
      winner_mode_stats[mode_idx].rate_uv = rd_cost_uv->rate;
    }
  }

  if (color_map) {
    // Store color_index_map for palette mode
    const MACROBLOCKD *const xd = &x->e_mbd;
    int block_width, block_height;
    av1_get_block_dimensions(bsize, AOM_PLANE_Y, xd, &block_width,
                             &block_height, NULL, NULL);
    memcpy(winner_mode_stats[mode_idx].color_index_map, color_map,
           block_width * block_height * sizeof(color_map[0]));
  }

  x->winner_mode_count =
      AOMMIN(x->winner_mode_count + 1, max_winner_mode_count);
}

unsigned int av1_get_perpixel_variance(const AV1_COMP *cpi,
                                       const MACROBLOCKD *xd,
                                       const struct buf_2d *ref,
                                       BLOCK_SIZE bsize, int plane,
                                       int use_hbd);

unsigned int av1_get_perpixel_variance_facade(const struct AV1_COMP *cpi,
                                              const MACROBLOCKD *xd,
                                              const struct buf_2d *ref,
                                              BLOCK_SIZE bsize, int plane);

static inline int is_mode_intra(PREDICTION_MODE mode) {
  return mode < INTRA_MODE_END;
}

// This function will copy usable ref_mv_stack[ref_frame][4] and
// weight[ref_frame][4] information from ref_mv_stack[ref_frame][8] and
// weight[ref_frame][8].
static inline void av1_copy_usable_ref_mv_stack_and_weight(
    const MACROBLOCKD *xd, MB_MODE_INFO_EXT *const mbmi_ext,
    MV_REFERENCE_FRAME ref_frame) {
  memcpy(mbmi_ext->weight[ref_frame], xd->weight[ref_frame],
         USABLE_REF_MV_STACK_SIZE * sizeof(xd->weight[0][0]));
  memcpy(mbmi_ext->ref_mv_stack[ref_frame], xd->ref_mv_stack[ref_frame],
         USABLE_REF_MV_STACK_SIZE * sizeof(xd->ref_mv_stack[0][0]));
}

// Get transform rd gate level for the given transform search case.
static inline int get_txfm_rd_gate_level(
    const int is_masked_compound_enabled,
    const int txfm_rd_gate_level[TX_SEARCH_CASES], BLOCK_SIZE bsize,
    TX_SEARCH_CASE tx_search_case, int eval_motion_mode) {
  assert(tx_search_case < TX_SEARCH_CASES);
  if (tx_search_case == TX_SEARCH_MOTION_MODE && !eval_motion_mode &&
      num_pels_log2_lookup[bsize] > 8)
    return txfm_rd_gate_level[TX_SEARCH_MOTION_MODE];
  // Enable aggressive gating of transform search only when masked compound type
  // is enabled.
  else if (tx_search_case == TX_SEARCH_COMP_TYPE_MODE &&
           is_masked_compound_enabled)
    return txfm_rd_gate_level[TX_SEARCH_COMP_TYPE_MODE];

  return txfm_rd_gate_level[TX_SEARCH_DEFAULT];
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_RDOPT_UTILS_H_
