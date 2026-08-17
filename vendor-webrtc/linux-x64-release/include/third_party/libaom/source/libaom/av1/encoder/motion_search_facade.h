/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_MOTION_SEARCH_H_
#define AOM_AV1_ENCODER_MOTION_SEARCH_H_

#include "av1/encoder/encoder.h"

#ifdef __cplusplus
extern "C" {
#endif

#define NUM_JOINT_ME_REFINE_ITER 2
#define REDUCED_JOINT_ME_REFINE_ITER 1
// TODO(any): rename this struct to something else. There is already another
// struct called inter_modes_info, which makes this terribly confusing.
typedef struct {
  int drl_cost;
  int_mv full_search_mv;
  int full_mv_rate;
  int full_mv_bestsme;
  int skip;
} inter_mode_info;

struct HandleInterModeArgs;
void av1_single_motion_search(const AV1_COMP *const cpi, MACROBLOCK *x,
                              BLOCK_SIZE bsize, int ref_idx, int *rate_mv,
                              int search_range, inter_mode_info *mode_info,
                              int_mv *best_mv,
                              struct HandleInterModeArgs *const args);

int av1_joint_motion_search(const AV1_COMP *cpi, MACROBLOCK *x,
                            BLOCK_SIZE bsize, int_mv *cur_mv,
                            const uint8_t *mask, int mask_stride, int *rate_mv,
                            int allow_second_mv, int joint_me_num_refine_iter);

int av1_interinter_compound_motion_search(const AV1_COMP *const cpi,
                                          MACROBLOCK *x,
                                          const int_mv *const cur_mv,
                                          const BLOCK_SIZE bsize,
                                          const PREDICTION_MODE this_mode);

int av1_compound_single_motion_search(const AV1_COMP *cpi, MACROBLOCK *x,
                                      BLOCK_SIZE bsize, MV *this_mv,
                                      const uint8_t *second_pred,
                                      const uint8_t *mask, int mask_stride,
                                      int *rate_mv, int ref_idx);

// Performs a motion search in SIMPLE_TRANSLATION mode using reference frame
// ref and calculates the sse and var of the residue. Note that this sets the
// offset of mbmi, so we will need to reset it after calling this function.
int_mv av1_simple_motion_search_sse_var(struct AV1_COMP *cpi, MACROBLOCK *x,
                                        int mi_row, int mi_col,
                                        BLOCK_SIZE bsize, int ref,
                                        const FULLPEL_MV start_mv,
                                        int num_planes, int use_subpixel,
                                        unsigned int *sse, unsigned int *var);

static inline const search_site_config *av1_get_search_site_config(
    const AV1_COMP *cpi, MACROBLOCK *x, SEARCH_METHODS search_method) {
  const int ref_stride = x->e_mbd.plane[0].pre[0].stride;

  // AV1_COMP::mv_search_params.search_site_config is a compressor level cache
  // that's shared by multiple threads. In most cases where all frames have the
  // same resolution, the cache contains the search site config that we need.
  const MotionVectorSearchParams *mv_search_params = &cpi->mv_search_params;
  if (ref_stride == mv_search_params->search_site_cfg[SS_CFG_SRC]->stride) {
    return mv_search_params->search_site_cfg[SS_CFG_SRC];
  } else if (ref_stride ==
             mv_search_params->search_site_cfg[SS_CFG_LOOKAHEAD]->stride) {
    return mv_search_params->search_site_cfg[SS_CFG_LOOKAHEAD];
  }

  // If the cache does not contain the correct stride, then we will need to rely
  // on the thread level config MACROBLOCK::search_site_cfg_buf. If even the
  // thread level config doesn't match, then we need to update it.
  search_method = search_method_lookup[search_method];
  assert(search_method_lookup[search_method] == search_method &&
         "The search_method_lookup table should be idempotent.");
  if (ref_stride != x->search_site_cfg_buf[search_method].stride) {
    av1_refresh_search_site_config(x->search_site_cfg_buf, search_method,
                                   ref_stride);
  }

  return x->search_site_cfg_buf;
}

static inline SEARCH_METHODS av1_get_faster_search_method(
    SEARCH_METHODS search_method) {
  // Note on search method's accuracy:
  //  1. NSTEP
  //  2. DIAMOND
  //  3. BIGDIA \approx SQUARE
  //  4. HEX.
  //  5. FAST_HEX \approx FAST_DIAMOND
  switch (search_method) {
    case NSTEP: return DIAMOND;
    case NSTEP_8PT: return DIAMOND;
    case DIAMOND: return BIGDIA;
    case CLAMPED_DIAMOND: return BIGDIA;
    case BIGDIA: return HEX;
    case SQUARE: return HEX;
    case HEX: return FAST_HEX;
    case FAST_HEX: return FAST_HEX;
    case FAST_DIAMOND: return VFAST_DIAMOND;
    case FAST_BIGDIA: return FAST_BIGDIA;
    case VFAST_DIAMOND: return VFAST_DIAMOND;
    default: assert(0 && "Invalid search method!"); return DIAMOND;
  }
}

static inline SEARCH_METHODS av1_get_default_mv_search_method(
    const MACROBLOCK *x, const MV_SPEED_FEATURES *mv_sf, BLOCK_SIZE bsize) {
  SEARCH_METHODS search_method = mv_sf->search_method;
  const int sf_blk_search_method = mv_sf->use_bsize_dependent_search_method;
  const int min_dim = AOMMIN(block_size_wide[bsize], block_size_high[bsize]);
  const int qband = x->qindex >> (QINDEX_BITS - 2);
  const bool use_faster_search_method =
      (sf_blk_search_method == 1 && min_dim >= 32) ||
      (sf_blk_search_method >= 2 && min_dim >= 16 &&
       x->content_state_sb.source_sad_nonrd <= kMedSad && qband < 3);

  if (use_faster_search_method) {
    search_method = av1_get_faster_search_method(search_method);
  }
  return search_method;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_MOTION_SEARCH_H_
