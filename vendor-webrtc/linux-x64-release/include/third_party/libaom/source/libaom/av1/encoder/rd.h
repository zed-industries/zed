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

#ifndef AOM_AV1_ENCODER_RD_H_
#define AOM_AV1_ENCODER_RD_H_

#include <limits.h>

#include "aom/aomcx.h"
#include "av1/common/blockd.h"
#include "av1/encoder/block.h"
#include "av1/encoder/context_tree.h"
#include "av1/encoder/cost.h"
#include "av1/encoder/ratectrl.h"
#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

#define RDDIV_BITS 7
#define RD_EPB_SHIFT 6

#define RDCOST(RM, R, D)                                            \
  (ROUND_POWER_OF_TWO(((int64_t)(R)) * (RM), AV1_PROB_COST_SHIFT) + \
   ((D) * (1 << RDDIV_BITS)))

#define RDCOST_NEG_R(RM, R, D) \
  (((D) * (1 << RDDIV_BITS)) - \
   ROUND_POWER_OF_TWO(((int64_t)(R)) * (RM), AV1_PROB_COST_SHIFT))

#define RDCOST_DBL_WITH_NATIVE_BD_DIST(RM, R, D, BD)               \
  (((((double)(R)) * (RM)) / (double)(1 << AV1_PROB_COST_SHIFT)) + \
   ((double)((D) >> (2 * (BD - 8))) * (1 << RDDIV_BITS)))

#define QIDX_SKIP_THRESH 115

#define MV_COST_WEIGHT 108
#define MV_COST_WEIGHT_SUB 120

// The fractional part of rd_thresh factor is stored with 5 bits. The maximum
// factor that we allow is two, which is stored as 2 ** (5+1) = 64
#define RD_THRESH_FAC_FRAC_BITS (5)
#define RD_THRESH_FAC_FRAC_VAL (1 << (RD_THRESH_FAC_FRAC_BITS))
#define RD_THRESH_MAX_FACT ((RD_THRESH_FAC_FRAC_VAL) << 1)
#define RD_THRESH_LOG_DEC_FACTOR (4)
#define RD_THRESH_INC (1)

// Factor to weigh the rate for switchable interp filters.
#define SWITCHABLE_INTERP_RATE_FACTOR 1

// Macros for common video resolutions: width x height
// For example, 720p represents video resolution of 1280x720 pixels.
#define RESOLUTION_288P 352 * 288
#define RESOLUTION_360P 640 * 360
#define RESOLUTION_480P 640 * 480
#define RESOLUTION_720P 1280 * 720
#define RESOLUTION_1080P 1920 * 1080
#define RESOLUTION_1440P 2560 * 1440
#define RESOLUTION_4K 3840 * 2160

#define RTC_REFS 4
static const MV_REFERENCE_FRAME real_time_ref_combos[RTC_REFS][2] = {
  { LAST_FRAME, NONE_FRAME },
  { ALTREF_FRAME, NONE_FRAME },
  { GOLDEN_FRAME, NONE_FRAME },
  { INTRA_FRAME, NONE_FRAME }
};

static inline int mode_offset(const PREDICTION_MODE mode) {
  if (mode >= NEARESTMV) {
    return INTER_OFFSET(mode);
  } else {
    switch (mode) {
      case DC_PRED: return 0;
      case V_PRED: return 1;
      case H_PRED: return 2;
      case SMOOTH_PRED: return 3;
      default: assert(0); return -1;
    }
  }
}

enum {
  // Default initialization when we are not using winner mode framework. e.g.
  // intrabc
  DEFAULT_EVAL = 0,
  // Initialization for selecting winner mode
  MODE_EVAL,
  // Initialization for winner mode evaluation
  WINNER_MODE_EVAL,
  // All mode evaluation types
  MODE_EVAL_TYPES,
} UENUM1BYTE(MODE_EVAL_TYPE);

typedef struct RD_OPT {
  // Thresh_mult is used to set a threshold for the rd score. A higher value
  // means that we will accept the best mode so far more often. This number
  // is used in combination with the current block size, and thresh_freq_fact
  // to pick a threshold.
  int thresh_mult[MAX_MODES];

  int threshes[MAX_SEGMENTS][BLOCK_SIZES_ALL][MAX_MODES];

  int RDMULT;

  double r0;
} RD_OPT;

static inline void av1_init_rd_stats(RD_STATS *rd_stats) {
#if CONFIG_RD_DEBUG
  int plane;
#endif
  rd_stats->rate = 0;
  rd_stats->dist = 0;
  rd_stats->rdcost = 0;
  rd_stats->sse = 0;
  rd_stats->skip_txfm = 1;
  rd_stats->zero_rate = 0;
#if CONFIG_RD_DEBUG
  // This may run into problems when monochrome video is
  // encoded, as there will only be 1 plane
  for (plane = 0; plane < MAX_MB_PLANE; ++plane) {
    rd_stats->txb_coeff_cost[plane] = 0;
  }
#endif
}

static inline void av1_invalid_rd_stats(RD_STATS *rd_stats) {
#if CONFIG_RD_DEBUG
  int plane;
#endif
  rd_stats->rate = INT_MAX;
  rd_stats->dist = INT64_MAX;
  rd_stats->rdcost = INT64_MAX;
  rd_stats->sse = INT64_MAX;
  rd_stats->skip_txfm = 0;
  rd_stats->zero_rate = 0;
#if CONFIG_RD_DEBUG
  // This may run into problems when monochrome video is
  // encoded, as there will only be 1 plane
  for (plane = 0; plane < MAX_MB_PLANE; ++plane) {
    rd_stats->txb_coeff_cost[plane] = INT_MAX;
  }
#endif
}

static inline void av1_merge_rd_stats(RD_STATS *rd_stats_dst,
                                      const RD_STATS *rd_stats_src) {
  if (rd_stats_dst->rate == INT_MAX || rd_stats_src->rate == INT_MAX) {
    // If rd_stats_dst or rd_stats_src has invalid rate, we will make
    // rd_stats_dst invalid.
    av1_invalid_rd_stats(rd_stats_dst);
    return;
  }
  rd_stats_dst->rate = (int)AOMMIN(
      ((int64_t)rd_stats_dst->rate + (int64_t)rd_stats_src->rate), INT_MAX);
  if (!rd_stats_dst->zero_rate)
    rd_stats_dst->zero_rate = rd_stats_src->zero_rate;
  rd_stats_dst->dist += rd_stats_src->dist;
  if (rd_stats_dst->sse < INT64_MAX && rd_stats_src->sse < INT64_MAX) {
    rd_stats_dst->sse += rd_stats_src->sse;
  }
  rd_stats_dst->skip_txfm &= rd_stats_src->skip_txfm;
#if CONFIG_RD_DEBUG
  // This may run into problems when monochrome video is
  // encoded, as there will only be 1 plane
  for (int plane = 0; plane < MAX_MB_PLANE; ++plane) {
    rd_stats_dst->txb_coeff_cost[plane] += rd_stats_src->txb_coeff_cost[plane];
  }
#endif
}

static inline void av1_accumulate_rd_stats(RD_STATS *rd_stats, int64_t dist,
                                           int rate, int skip_txfm, int64_t sse,
                                           int zero_rate) {
  assert(rd_stats->rate != INT_MAX && rate != INT_MAX);
  rd_stats->rate += rate;
  if (!rd_stats->zero_rate) rd_stats->zero_rate = zero_rate;
  rd_stats->dist += dist;
  rd_stats->skip_txfm &= skip_txfm;
  rd_stats->sse += sse;
}

static inline int64_t av1_calculate_rd_cost(int mult, int rate, int64_t dist) {
  assert(mult >= 0);
  if (rate >= 0) {
    return RDCOST(mult, rate, dist);
  }
  return RDCOST_NEG_R(mult, -rate, dist);
}

static inline void av1_rd_cost_update(int mult, RD_STATS *rd_cost) {
  if (rd_cost->rate < INT_MAX && rd_cost->dist < INT64_MAX &&
      rd_cost->rdcost < INT64_MAX) {
    rd_cost->rdcost = av1_calculate_rd_cost(mult, rd_cost->rate, rd_cost->dist);
  } else {
    av1_invalid_rd_stats(rd_cost);
  }
}

static inline void av1_rd_stats_subtraction(int mult,
                                            const RD_STATS *const left,
                                            const RD_STATS *const right,
                                            RD_STATS *result) {
  if (left->rate == INT_MAX || right->rate == INT_MAX ||
      left->dist == INT64_MAX || right->dist == INT64_MAX ||
      left->rdcost == INT64_MAX || right->rdcost == INT64_MAX) {
    av1_invalid_rd_stats(result);
  } else {
    result->rate = left->rate - right->rate;
    result->dist = left->dist - right->dist;
    result->rdcost = av1_calculate_rd_cost(mult, result->rate, result->dist);
  }
}

struct TileInfo;
struct TileDataEnc;
struct AV1_COMP;
struct macroblock;

/*!\brief Compute rdmult based on q index and frame update type
 *
 * \param[in]       bit_depth       bit depth
 * \param[in]       update_type     frame update type
 * \param[in]       qindex          q index
 * \param[in]       tuning          visual tuning metric
 *
 * \return rdmult
 */
int av1_compute_rd_mult_based_on_qindex(aom_bit_depth_t bit_depth,
                                        FRAME_UPDATE_TYPE update_type,
                                        int qindex, aom_tune_metric tuning);

int av1_compute_rd_mult(const int qindex, const aom_bit_depth_t bit_depth,
                        const FRAME_UPDATE_TYPE update_type,
                        const int layer_depth, const int boost_index,
                        const FRAME_TYPE frame_type,
                        const int use_fixed_qp_offsets,
                        const int is_stat_consumption_stage,
                        const aom_tune_metric tuning);

void av1_initialize_rd_consts(struct AV1_COMP *cpi);

// Sets the multiplier to convert mv cost to l1 error during motion search.
void av1_set_sad_per_bit(const struct AV1_COMP *cpi, int *sadperbit,
                         int qindex);

void av1_model_rd_from_var_lapndz(int64_t var, unsigned int n,
                                  unsigned int qstep, int *rate, int64_t *dist);

void av1_model_rd_curvfit(BLOCK_SIZE bsize, double sse_norm, double xqr,
                          double *rate_f, double *distbysse_f);

int av1_get_switchable_rate(const MACROBLOCK *x, const MACROBLOCKD *xd,
                            InterpFilter interp_filter, int dual_filter);

YV12_BUFFER_CONFIG *av1_get_scaled_ref_frame(const struct AV1_COMP *cpi,
                                             int ref_frame);

void av1_init_me_luts(void);

void av1_set_mvcost(MACROBLOCK *x, int ref, int ref_mv_idx);

void av1_get_entropy_contexts(BLOCK_SIZE plane_bsize,
                              const struct macroblockd_plane *pd,
                              ENTROPY_CONTEXT t_above[MAX_MIB_SIZE],
                              ENTROPY_CONTEXT t_left[MAX_MIB_SIZE]);

void av1_set_rd_speed_thresholds(struct AV1_COMP *cpi);

void av1_update_rd_thresh_fact(const AV1_COMMON *const cm,
                               int (*fact)[MAX_MODES], int rd_thresh,
                               BLOCK_SIZE bsize, THR_MODES best_mode_index,
                               THR_MODES inter_mode_start,
                               THR_MODES inter_mode_end,
                               THR_MODES intra_mode_start,
                               THR_MODES intra_mode_end);

static inline void reset_thresh_freq_fact(MACROBLOCK *const x) {
  for (int i = 0; i < BLOCK_SIZES_ALL; ++i) {
    for (int j = 0; j < MAX_MODES; ++j) {
      x->thresh_freq_fact[i][j] = RD_THRESH_FAC_FRAC_VAL;
    }
  }
}

static inline int rd_less_than_thresh(int64_t best_rd, int64_t thresh,
                                      int thresh_fact) {
  return best_rd < (thresh * thresh_fact >> 5) || thresh == INT_MAX;
}

void av1_mv_pred(const struct AV1_COMP *cpi, MACROBLOCK *x,
                 uint8_t *ref_y_buffer, int ref_y_stride, int ref_frame,
                 BLOCK_SIZE block_size);

// Sets the multiplier to convert mv cost to l2 error during motion search.
static inline void av1_set_error_per_bit(int *errorperbit, int rdmult) {
  *errorperbit = AOMMAX(rdmult >> RD_EPB_SHIFT, 1);
}

// Get the threshold for R-D optimization of coefficients depending upon mode
// decision/winner mode processing
static inline void get_rd_opt_coeff_thresh(
    const uint32_t (*const coeff_opt_threshold)[2],
    TxfmSearchParams *txfm_params, int enable_winner_mode_for_coeff_opt,
    int is_winner_mode) {
  if (!enable_winner_mode_for_coeff_opt) {
    // Default initialization of threshold
    txfm_params->coeff_opt_thresholds[0] = coeff_opt_threshold[DEFAULT_EVAL][0];
    txfm_params->coeff_opt_thresholds[1] = coeff_opt_threshold[DEFAULT_EVAL][1];
    return;
  }
  // TODO(any): Experiment with coeff_opt_dist_threshold values when
  // enable_winner_mode_for_coeff_opt is ON
  // TODO(any): Skip the winner mode processing for blocks with lower residual
  // energy as R-D optimization of coefficients would have been enabled during
  // mode decision

  // Use conservative threshold during mode decision and perform R-D
  // optimization of coeffs always for winner modes
  if (is_winner_mode) {
    txfm_params->coeff_opt_thresholds[0] =
        coeff_opt_threshold[WINNER_MODE_EVAL][0];
    txfm_params->coeff_opt_thresholds[1] =
        coeff_opt_threshold[WINNER_MODE_EVAL][1];
  } else {
    txfm_params->coeff_opt_thresholds[0] = coeff_opt_threshold[MODE_EVAL][0];
    txfm_params->coeff_opt_thresholds[1] = coeff_opt_threshold[MODE_EVAL][1];
  }
}

// Used to reset the state of mb rd hash information
static inline void reset_mb_rd_record(MB_RD_RECORD *const mb_rd_record) {
  if (!mb_rd_record) return;

  // Reset the state for use_mb_rd_hash
  mb_rd_record->num = mb_rd_record->index_start = 0;
}

void av1_setup_pred_block(const MACROBLOCKD *xd,
                          struct buf_2d dst[MAX_MB_PLANE],
                          const YV12_BUFFER_CONFIG *src,
                          const struct scale_factors *scale,
                          const struct scale_factors *scale_uv,
                          const int num_planes);

int av1_get_intra_cost_penalty(int qindex, int qdelta,
                               aom_bit_depth_t bit_depth);

void av1_fill_mode_rates(AV1_COMMON *const cm, ModeCosts *mode_costs,
                         FRAME_CONTEXT *fc);

#if !CONFIG_REALTIME_ONLY
void av1_fill_lr_rates(ModeCosts *mode_costs, FRAME_CONTEXT *fc);
#endif

void av1_fill_coeff_costs(CoeffCosts *coeff_costs, FRAME_CONTEXT *fc,
                          const int num_planes);

void av1_fill_mv_costs(const nmv_context *nmvc, int integer_mv, int usehp,
                       MvCosts *mv_costs);

void av1_fill_dv_costs(const nmv_context *ndvc, IntraBCMVCosts *dv_costs);

#if !CONFIG_REALTIME_ONLY
int av1_get_adaptive_rdmult(const struct AV1_COMP *cpi, double beta);
#endif

int av1_get_deltaq_offset(aom_bit_depth_t bit_depth, int qindex, double beta);

/*!\brief Adjust current superblock's q_index based on delta q resolution
 *
 * \param[in]       delta_q_res       delta q resolution
 * \param[in]       prev_qindex       previous superblock's q index
 * \param[in]       curr_qindex       current superblock's q index
 *
 * \return the current superblock's adjusted q_index
 */
int av1_adjust_q_from_delta_q_res(int delta_q_res, int prev_qindex,
                                  int curr_qindex);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_RD_H_
