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

#ifndef AOM_AV1_ENCODER_ENCODEFRAME_UTILS_H_
#define AOM_AV1_ENCODER_ENCODEFRAME_UTILS_H_

#include "aom_ports/aom_timer.h"

#include "av1/common/reconinter.h"

#include "av1/encoder/encoder.h"
#include "av1/encoder/rdopt.h"

#ifdef __cplusplus
extern "C" {
#endif

#define WRITE_FEATURE_TO_FILE 0

#define FEATURE_SIZE_SMS_SPLIT_FAST 6
#define FEATURE_SIZE_SMS_SPLIT 17
#define FEATURE_SIZE_SMS_PRUNE_PART 25
#define FEATURE_SIZE_SMS_TERM_NONE 28
#define FEATURE_SIZE_FP_SMS_TERM_NONE 20
#define FEATURE_SIZE_MAX_MIN_PART_PRED 13
#define MAX_NUM_CLASSES_MAX_MIN_PART_PRED 4

#define FEATURE_SMS_NONE_FLAG 1
#define FEATURE_SMS_SPLIT_FLAG (1 << 1)
#define FEATURE_SMS_RECT_FLAG (1 << 2)

#define FEATURE_SMS_PRUNE_PART_FLAG \
  (FEATURE_SMS_NONE_FLAG | FEATURE_SMS_SPLIT_FLAG | FEATURE_SMS_RECT_FLAG)
#define FEATURE_SMS_SPLIT_MODEL_FLAG \
  (FEATURE_SMS_NONE_FLAG | FEATURE_SMS_SPLIT_FLAG)

// Number of sub-partitions in rectangular partition types.
#define SUB_PARTITIONS_RECT 2

// Number of sub-partitions in split partition type.
#define SUB_PARTITIONS_SPLIT 4

// Number of sub-partitions in AB partition types.
#define SUB_PARTITIONS_AB 3

// Number of sub-partitions in 4-way partition types.
#define SUB_PARTITIONS_PART4 4

// 4part partition types.
enum { HORZ4 = 0, VERT4, NUM_PART4_TYPES } UENUM1BYTE(PART4_TYPES);

// AB partition types.
enum {
  HORZ_A = 0,
  HORZ_B,
  VERT_A,
  VERT_B,
  NUM_AB_PARTS
} UENUM1BYTE(AB_PART_TYPE);

// Rectangular partition types.
enum { HORZ = 0, VERT, NUM_RECT_PARTS } UENUM1BYTE(RECT_PART_TYPE);

// Structure to keep win flags for HORZ and VERT partition evaluations.
typedef struct {
  int rect_part_win[NUM_RECT_PARTS];
} RD_RECT_PART_WIN_INFO;

enum { PICK_MODE_RD = 0, PICK_MODE_NONRD };

enum {
  SB_SINGLE_PASS,  // Single pass encoding: all ctxs get updated normally
  SB_DRY_PASS,     // First pass of multi-pass: does not update the ctxs
  SB_WET_PASS      // Second pass of multi-pass: finalize and update the ctx
} UENUM1BYTE(SB_MULTI_PASS_MODE);

typedef struct {
  ENTROPY_CONTEXT a[MAX_MIB_SIZE * MAX_MB_PLANE];
  ENTROPY_CONTEXT l[MAX_MIB_SIZE * MAX_MB_PLANE];
  PARTITION_CONTEXT sa[MAX_MIB_SIZE];
  PARTITION_CONTEXT sl[MAX_MIB_SIZE];
  TXFM_CONTEXT *p_ta;
  TXFM_CONTEXT *p_tl;
  TXFM_CONTEXT ta[MAX_MIB_SIZE];
  TXFM_CONTEXT tl[MAX_MIB_SIZE];
} RD_SEARCH_MACROBLOCK_CONTEXT;

// This struct is used to store the statistics used by sb-level multi-pass
// encoding. Currently, this is only used to make a copy of the state before we
// perform the first pass
typedef struct SB_FIRST_PASS_STATS {
  RD_SEARCH_MACROBLOCK_CONTEXT x_ctx;
  RD_COUNTS rd_count;

  int split_count;
  FRAME_COUNTS fc;
  InterModeRdModel inter_mode_rd_models[BLOCK_SIZES_ALL];
  int thresh_freq_fact[BLOCK_SIZES_ALL][MAX_MODES];
  int current_qindex;

#if CONFIG_INTERNAL_STATS
  unsigned int mode_chosen_counts[MAX_MODES];
#endif  // CONFIG_INTERNAL_STATS
} SB_FIRST_PASS_STATS;

// This structure contains block size related
// variables for use in rd_pick_partition().
typedef struct {
  // Half of block width to determine block edge.
  int mi_step;

  // Block row and column indices.
  int mi_row;
  int mi_col;

  // Block edge row and column indices.
  int mi_row_edge;
  int mi_col_edge;

  // Block width of current partition block.
  int width;

  // Block width of minimum partition size allowed.
  int min_partition_size_1d;

  // Flag to indicate if partition is 8x8 or higher size.
  int bsize_at_least_8x8;

  // Indicates edge blocks in frame.
  int has_rows;
  int has_cols;

  // Block size of current partition.
  BLOCK_SIZE bsize;

  // Size of current sub-partition.
  BLOCK_SIZE subsize;

  // Size of split partition.
  BLOCK_SIZE split_bsize2;
} PartitionBlkParams;

#if CONFIG_COLLECT_PARTITION_STATS
typedef struct PartitionTimingStats {
  // Tracks the number of partition decision used in the current call to \ref
  // av1_rd_pick_partition
  int partition_decisions[EXT_PARTITION_TYPES];
  // Tracks the number of partition_block searched in the current call to \ref
  // av1_rd_pick_partition
  int partition_attempts[EXT_PARTITION_TYPES];
  // Tracks the time spent on each partition search in the current call to \ref
  // av1_rd_pick_partition
  int64_t partition_times[EXT_PARTITION_TYPES];
  // Tracks the rdcost spent on each partition search in the current call to
  // \ref av1_rd_pick_partition
  int64_t partition_rdcost[EXT_PARTITION_TYPES];
  // Timer used to time the partitions.
  struct aom_usec_timer timer;
  // Whether the timer is on
  int timer_is_on;
} PartitionTimingStats;
#endif  // CONFIG_COLLECT_PARTITION_STATS

// Structure holding state variables for partition search.
typedef struct {
  // Intra partitioning related info.
  PartitionSearchInfo *intra_part_info;

  // Parameters related to partition block size.
  PartitionBlkParams part_blk_params;

  // Win flags for HORZ and VERT partition evaluations.
  RD_RECT_PART_WIN_INFO split_part_rect_win[SUB_PARTITIONS_SPLIT];

  // RD cost for the current block of given partition type.
  RD_STATS this_rdc;

  // RD cost summed across all blocks of partition type.
  RD_STATS sum_rdc;

  // Array holding partition type cost.
  int tmp_partition_cost[PARTITION_TYPES];

  // Pointer to partition cost buffer
  int *partition_cost;

  // RD costs for different partition types.
  int64_t none_rd;
  int64_t split_rd[SUB_PARTITIONS_SPLIT];
  // RD costs for rectangular partitions.
  // rect_part_rd[0][i] is the RD cost of ith partition index of PARTITION_HORZ.
  // rect_part_rd[1][i] is the RD cost of ith partition index of PARTITION_VERT.
  int64_t rect_part_rd[NUM_RECT_PARTS][SUB_PARTITIONS_RECT];

  // Flags indicating if the corresponding partition was winner or not.
  // Used to bypass similar blocks during AB partition evaluation.
  int is_split_ctx_is_ready[2];
  int is_rect_ctx_is_ready[NUM_RECT_PARTS];

  // If true, skips the rest of partition evaluation at the current bsize level.
  int terminate_partition_search;

  // If false, skips rdopt on PARTITION_NONE.
  int partition_none_allowed;

  // If partition_rect_allowed[HORZ] is false, skips searching PARTITION_HORZ,
  // PARTITION_HORZ_A, PARTITIO_HORZ_B, PARTITION_HORZ_4. Same holds for VERT.
  int partition_rect_allowed[NUM_RECT_PARTS];

  // If false, skips searching rectangular partition unless some logic related
  // to edge detection holds.
  int do_rectangular_split;

  // If false, skips searching PARTITION_SPLIT.
  int do_square_split;

  // If true, prunes the corresponding PARTITION_HORZ/PARTITION_VERT. Note that
  // this does not directly affect the extended partitions, so this can be used
  // to prune out PARTITION_HORZ/PARTITION_VERT while still allowing rdopt of
  // PARTITION_HORZ_AB4, etc.
  int prune_rect_part[NUM_RECT_PARTS];

  // Chroma subsampling in x and y directions.
  int ss_x;
  int ss_y;

  // Partition plane context index.
  int pl_ctx_idx;

  // This flag will be set if best partition is found from the search.
  bool found_best_partition;

#if CONFIG_COLLECT_PARTITION_STATS
  PartitionTimingStats part_timing_stats;
#endif  // CONFIG_COLLECT_PARTITION_STATS
} PartitionSearchState;

static inline void av1_disable_square_split_partition(
    PartitionSearchState *part_state) {
  part_state->do_square_split = 0;
}

// Disables all possible rectangular splits. This includes PARTITION_AB4 as they
// depend on the corresponding partition_rect_allowed.
static inline void av1_disable_rect_partitions(
    PartitionSearchState *part_state) {
  part_state->do_rectangular_split = 0;
  part_state->partition_rect_allowed[HORZ] = 0;
  part_state->partition_rect_allowed[VERT] = 0;
}

// Disables all possible splits so that only PARTITION_NONE *might* be allowed.
static inline void av1_disable_all_splits(PartitionSearchState *part_state) {
  av1_disable_square_split_partition(part_state);
  av1_disable_rect_partitions(part_state);
}

static inline void av1_set_square_split_only(PartitionSearchState *part_state) {
  part_state->partition_none_allowed = 0;
  part_state->do_square_split = 1;
  av1_disable_rect_partitions(part_state);
}

static inline bool av1_blk_has_rows_and_cols(
    const PartitionBlkParams *blk_params) {
  return blk_params->has_rows && blk_params->has_cols;
}

static inline bool av1_is_whole_blk_in_frame(
    const PartitionBlkParams *blk_params,
    const CommonModeInfoParams *mi_params) {
  const int mi_row = blk_params->mi_row, mi_col = blk_params->mi_col;
  const BLOCK_SIZE bsize = blk_params->bsize;
  return mi_row + mi_size_high[bsize] <= mi_params->mi_rows &&
         mi_col + mi_size_wide[bsize] <= mi_params->mi_cols;
}

static inline void update_filter_type_cdf(const MACROBLOCKD *xd,
                                          const MB_MODE_INFO *mbmi,
                                          int dual_filter) {
  for (int dir = 0; dir < 2; ++dir) {
    if (dir && !dual_filter) break;
    const int ctx = av1_get_pred_context_switchable_interp(xd, dir);
    InterpFilter filter = av1_extract_interp_filter(mbmi->interp_filters, dir);
    update_cdf(xd->tile_ctx->switchable_interp_cdf[ctx], filter,
               SWITCHABLE_FILTERS);
  }
}

static inline int set_rdmult(const AV1_COMP *const cpi,
                             const MACROBLOCK *const x, int segment_id) {
  const AV1_COMMON *const cm = &cpi->common;
  const GF_GROUP *const gf_group = &cpi->ppi->gf_group;
  const CommonQuantParams *quant_params = &cm->quant_params;
  const aom_bit_depth_t bit_depth = cm->seq_params->bit_depth;
  const FRAME_UPDATE_TYPE update_type =
      cpi->ppi->gf_group.update_type[cpi->gf_frame_index];
  const FRAME_TYPE frame_type = cm->current_frame.frame_type;
  const int boost_index = AOMMIN(15, (cpi->ppi->p_rc.gfu_boost / 100));
  const int layer_depth = AOMMIN(gf_group->layer_depth[cpi->gf_frame_index], 6);

  int qindex;
  if (segment_id >= 0) {
    qindex = av1_get_qindex(&cm->seg, segment_id, cm->quant_params.base_qindex);
  } else {
    qindex = quant_params->base_qindex + x->rdmult_delta_qindex +
             quant_params->y_dc_delta_q;
  }

  return av1_compute_rd_mult(
      qindex, bit_depth, update_type, layer_depth, boost_index, frame_type,
      cpi->oxcf.q_cfg.use_fixed_qp_offsets, is_stat_consumption_stage(cpi),
      cpi->oxcf.tune_cfg.tuning);
}

static inline int do_split_check(BLOCK_SIZE bsize) {
  return (bsize == BLOCK_16X16 || bsize == BLOCK_32X32);
}

#if !CONFIG_REALTIME_ONLY
static inline const FIRSTPASS_STATS *read_one_frame_stats(const TWO_PASS *p,
                                                          int frm) {
  assert(frm >= 0);
  if (frm < 0 ||
      p->stats_buf_ctx->stats_in_start + frm > p->stats_buf_ctx->stats_in_end) {
    return NULL;
  }

  return &p->stats_buf_ctx->stats_in_start[frm];
}

int av1_get_rdmult_delta(AV1_COMP *cpi, BLOCK_SIZE bsize, int mi_row,
                         int mi_col, int orig_rdmult);

int av1_active_h_edge(const AV1_COMP *cpi, int mi_row, int mi_step);

int av1_active_v_edge(const AV1_COMP *cpi, int mi_col, int mi_step);

void av1_get_tpl_stats_sb(AV1_COMP *cpi, BLOCK_SIZE bsize, int mi_row,
                          int mi_col, SuperBlockEnc *sb_enc);

int av1_get_q_for_deltaq_objective(AV1_COMP *const cpi, ThreadData *td,
                                   int64_t *delta_dist, BLOCK_SIZE bsize,
                                   int mi_row, int mi_col);

int av1_get_q_for_hdr(AV1_COMP *const cpi, MACROBLOCK *const x,
                      BLOCK_SIZE bsize, int mi_row, int mi_col);

int av1_get_cb_rdmult(const AV1_COMP *const cpi, MACROBLOCK *const x,
                      const BLOCK_SIZE bsize, const int mi_row,
                      const int mi_col);
#endif  // !CONFIG_REALTIME_ONLY

void av1_set_ssim_rdmult(const AV1_COMP *const cpi, int *errorperbit,
                         const BLOCK_SIZE bsize, const int mi_row,
                         const int mi_col, int *const rdmult);

#if CONFIG_SALIENCY_MAP
void av1_set_saliency_map_vmaf_rdmult(const AV1_COMP *const cpi,
                                      int *errorperbit, const BLOCK_SIZE bsize,
                                      const int mi_row, const int mi_col,
                                      int *const rdmult);
#endif

void av1_update_state(const AV1_COMP *const cpi, ThreadData *td,
                      const PICK_MODE_CONTEXT *const ctx, int mi_row,
                      int mi_col, BLOCK_SIZE bsize, RUN_TYPE dry_run);

void av1_update_inter_mode_stats(FRAME_CONTEXT *fc, FRAME_COUNTS *counts,
                                 PREDICTION_MODE mode, int16_t mode_context);

void av1_sum_intra_stats(const AV1_COMMON *const cm, FRAME_COUNTS *counts,
                         MACROBLOCKD *xd, const MB_MODE_INFO *const mbmi,
                         const MB_MODE_INFO *above_mi,
                         const MB_MODE_INFO *left_mi, const int intraonly);

void av1_restore_context(MACROBLOCK *x, const RD_SEARCH_MACROBLOCK_CONTEXT *ctx,
                         int mi_row, int mi_col, BLOCK_SIZE bsize,
                         const int num_planes);

void av1_save_context(const MACROBLOCK *x, RD_SEARCH_MACROBLOCK_CONTEXT *ctx,
                      int mi_row, int mi_col, BLOCK_SIZE bsize,
                      const int num_planes);

void av1_set_fixed_partitioning(AV1_COMP *cpi, const TileInfo *const tile,
                                MB_MODE_INFO **mib, int mi_row, int mi_col,
                                BLOCK_SIZE bsize);

int av1_is_leaf_split_partition(AV1_COMMON *cm, int mi_row, int mi_col,
                                BLOCK_SIZE bsize);

void av1_reset_simple_motion_tree_partition(SIMPLE_MOTION_DATA_TREE *sms_tree,
                                            BLOCK_SIZE bsize);

void av1_update_picked_ref_frames_mask(MACROBLOCK *const x, int ref_type,
                                       BLOCK_SIZE bsize, int mib_size,
                                       int mi_row, int mi_col);

void av1_avg_cdf_symbols(FRAME_CONTEXT *ctx_left, FRAME_CONTEXT *ctx_tr,
                         int wt_left, int wt_tr);

void av1_source_content_sb(AV1_COMP *cpi, MACROBLOCK *x, TileDataEnc *tile_data,
                           int mi_row, int mi_col);

void av1_reset_mbmi(CommonModeInfoParams *const mi_params, BLOCK_SIZE sb_size,
                    int mi_row, int mi_col);

void av1_backup_sb_state(SB_FIRST_PASS_STATS *sb_fp_stats, const AV1_COMP *cpi,
                         ThreadData *td, const TileDataEnc *tile_data,
                         int mi_row, int mi_col);

void av1_restore_sb_state(const SB_FIRST_PASS_STATS *sb_fp_stats, AV1_COMP *cpi,
                          ThreadData *td, TileDataEnc *tile_data, int mi_row,
                          int mi_col);

void av1_set_cost_upd_freq(AV1_COMP *cpi, ThreadData *td,
                           const TileInfo *const tile_info, const int mi_row,
                           const int mi_col);

void av1_dealloc_src_diff_buf(struct macroblock *mb, int num_planes);

static inline void av1_dealloc_mb_data(struct macroblock *mb, int num_planes) {
  aom_free(mb->txfm_search_info.mb_rd_record);
  mb->txfm_search_info.mb_rd_record = NULL;

  aom_free(mb->inter_modes_info);
  mb->inter_modes_info = NULL;

  av1_dealloc_src_diff_buf(mb, num_planes);

  aom_free(mb->e_mbd.seg_mask);
  mb->e_mbd.seg_mask = NULL;

  aom_free(mb->winner_mode_stats);
  mb->winner_mode_stats = NULL;

  aom_free(mb->dqcoeff_buf);
  mb->dqcoeff_buf = NULL;
}

static inline void allocate_winner_mode_stats(const AV1_COMP *cpi,
                                              struct macroblock *mb) {
  const SPEED_FEATURES *sf = &cpi->sf;
  // The winner_mode_stats buffer is not required in these cases.
  if (is_stat_generation_stage(cpi) ||
      (sf->rt_sf.use_nonrd_pick_mode && !sf->rt_sf.hybrid_intra_pickmode) ||
      (sf->winner_mode_sf.multi_winner_mode_type == MULTI_WINNER_MODE_OFF))
    return;

  const AV1_COMMON *cm = &cpi->common;
  const int winner_mode_count =
      winner_mode_count_allowed[sf->winner_mode_sf.multi_winner_mode_type];
  CHECK_MEM_ERROR(cm, mb->winner_mode_stats,
                  (WinnerModeStats *)aom_malloc(
                      winner_mode_count * sizeof(mb->winner_mode_stats[0])));
}

void av1_alloc_src_diff_buf(const struct AV1Common *cm, struct macroblock *mb);

static inline void av1_alloc_mb_data(const AV1_COMP *cpi,
                                     struct macroblock *mb) {
  const AV1_COMMON *cm = &cpi->common;
  const SPEED_FEATURES *sf = &cpi->sf;
  if (!sf->rt_sf.use_nonrd_pick_mode) {
    // Memory for mb_rd_record is allocated only when use_mb_rd_hash sf is
    // enabled.
    if (sf->rd_sf.use_mb_rd_hash)
      CHECK_MEM_ERROR(cm, mb->txfm_search_info.mb_rd_record,
                      (MB_RD_RECORD *)aom_malloc(sizeof(MB_RD_RECORD)));
    if (!frame_is_intra_only(cm))
      CHECK_MEM_ERROR(
          cm, mb->inter_modes_info,
          (InterModesInfo *)aom_malloc(sizeof(*mb->inter_modes_info)));
  }

  av1_alloc_src_diff_buf(cm, mb);

  CHECK_MEM_ERROR(cm, mb->e_mbd.seg_mask,
                  (uint8_t *)aom_memalign(
                      16, 2 * MAX_SB_SQUARE * sizeof(mb->e_mbd.seg_mask[0])));

  allocate_winner_mode_stats(cpi, mb);

  const int max_sb_square_y = 1
                              << num_pels_log2_lookup[cm->seq_params->sb_size];
  CHECK_MEM_ERROR(
      cm, mb->dqcoeff_buf,
      (tran_low_t *)aom_memalign(32, max_sb_square_y * sizeof(tran_low_t)));
}

// This function will compute the number of reference frames to be disabled
// based on selective_ref_frame speed feature.
static inline unsigned int get_num_refs_to_disable(
    const AV1_COMP *cpi, const int *ref_frame_flags,
    const unsigned int *ref_display_order_hint,
    unsigned int cur_frame_display_index) {
  unsigned int num_refs_to_disable = 0;
  if (cpi->sf.inter_sf.selective_ref_frame >= 3) {
    num_refs_to_disable++;
    if (cpi->sf.inter_sf.selective_ref_frame >= 6) {
      // Disable LAST2_FRAME  and ALTREF2_FRAME
      num_refs_to_disable += 2;
    } else if (cpi->sf.inter_sf.selective_ref_frame == 5 &&
               *ref_frame_flags & av1_ref_frame_flag_list[LAST2_FRAME]) {
      const int last2_frame_dist = av1_encoder_get_relative_dist(
          ref_display_order_hint[LAST2_FRAME - LAST_FRAME],
          cur_frame_display_index);
      // Disable LAST2_FRAME if it is a temporally distant frame
      if (abs(last2_frame_dist) > 2) {
        num_refs_to_disable++;
      }
#if !CONFIG_REALTIME_ONLY
      else if (is_stat_consumption_stage_twopass(cpi)) {
        const FIRSTPASS_STATS *const this_frame_stats =
            read_one_frame_stats(&cpi->ppi->twopass, cur_frame_display_index);
        const double coded_error_per_mb = this_frame_stats->coded_error;
        // Disable LAST2_FRAME if the coded error of the current frame based on
        // first pass stats is very low.
        if (coded_error_per_mb < 100.0) num_refs_to_disable++;
      }
#endif  // CONFIG_REALTIME_ONLY
    }
  }
  return num_refs_to_disable;
}

static inline int get_max_allowed_ref_frames(
    const AV1_COMP *cpi, const int *ref_frame_flags,
    const unsigned int *ref_display_order_hint,
    unsigned int cur_frame_display_index) {
  const unsigned int max_reference_frames =
      cpi->oxcf.ref_frm_cfg.max_reference_frames;
  const unsigned int num_refs_to_disable = get_num_refs_to_disable(
      cpi, ref_frame_flags, ref_display_order_hint, cur_frame_display_index);
  const unsigned int max_allowed_refs_for_given_speed =
      INTER_REFS_PER_FRAME - num_refs_to_disable;
  return AOMMIN(max_allowed_refs_for_given_speed, max_reference_frames);
}

// Enforce the number of references for each arbitrary frame based on user
// options and speed.
static inline void enforce_max_ref_frames(
    AV1_COMP *cpi, int *ref_frame_flags,
    const unsigned int *ref_display_order_hint,
    unsigned int cur_frame_display_index) {
  MV_REFERENCE_FRAME ref_frame;
  int total_valid_refs = 0;

  for (ref_frame = LAST_FRAME; ref_frame <= ALTREF_FRAME; ++ref_frame) {
    if (*ref_frame_flags & av1_ref_frame_flag_list[ref_frame]) {
      total_valid_refs++;
    }
  }

  const int max_allowed_refs = get_max_allowed_ref_frames(
      cpi, ref_frame_flags, ref_display_order_hint, cur_frame_display_index);

  for (int i = 0; i < 4 && total_valid_refs > max_allowed_refs; ++i) {
    const MV_REFERENCE_FRAME ref_frame_to_disable = disable_order[i];

    if (!(*ref_frame_flags & av1_ref_frame_flag_list[ref_frame_to_disable])) {
      continue;
    }

    switch (ref_frame_to_disable) {
      case LAST3_FRAME: *ref_frame_flags &= ~AOM_LAST3_FLAG; break;
      case LAST2_FRAME: *ref_frame_flags &= ~AOM_LAST2_FLAG; break;
      case ALTREF2_FRAME: *ref_frame_flags &= ~AOM_ALT2_FLAG; break;
      case BWDREF_FRAME: *ref_frame_flags &= ~AOM_GOLD_FLAG; break;
      default: assert(0);
    }
    --total_valid_refs;
  }
  assert(total_valid_refs <= max_allowed_refs);
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ENCODEFRAME_UTILS_H_
