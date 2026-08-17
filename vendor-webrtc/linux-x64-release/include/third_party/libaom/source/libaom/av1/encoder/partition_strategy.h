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

#ifndef AOM_AV1_ENCODER_PARTITION_STRATEGY_H_
#define AOM_AV1_ENCODER_PARTITION_STRATEGY_H_

#include "config/aom_config.h"

#include "av1/encoder/encodeframe.h"
#include "av1/encoder/encodeframe_utils.h"
#include "av1/encoder/encodemb.h"
#include "av1/encoder/encoder.h"

#if !CONFIG_REALTIME_ONLY
// Early terminates PARTITION_NONE using simple_motion_search features and the
// rate, distortion, and rdcost of PARTITION_NONE. This is only called when:
//  - The frame is a show frame
//  - The frame is not intra only
//  - The current bsize is > BLOCK_8X8
//  - blk_row + blk_height/2 < total_rows and blk_col + blk_width/2 < total_cols
void av1_simple_motion_search_early_term_none(AV1_COMP *const cpi,
                                              MACROBLOCK *x,
                                              SIMPLE_MOTION_DATA_TREE *sms_tree,
                                              const RD_STATS *none_rdc,
                                              PartitionSearchState *part_state);

// Get the features for selecting the max and min partition size. Currently this
// performs simple_motion_search on 16X16 subblocks of the current superblock,
// and then extract the statistics of sse and motion vectors as features.
void av1_get_max_min_partition_features(AV1_COMP *const cpi, MACROBLOCK *x,
                                        int mi_row, int mi_col,
                                        float *features);

// Predict the maximum BLOCK_SIZE to be used to encoder the current superblock.
BLOCK_SIZE av1_predict_max_partition(const AV1_COMP *const cpi,
                                     const MACROBLOCK *const x,
                                     const float *features);

// Attempts an early termination after PARTITION_SPLIT.
void av1_ml_early_term_after_split(AV1_COMP *const cpi, MACROBLOCK *const x,
                                   SIMPLE_MOTION_DATA_TREE *const sms_tree,
                                   int64_t best_rd, int64_t part_none_rd,
                                   int64_t part_split_rd,
                                   int64_t *split_block_rd,
                                   PartitionSearchState *part_state);

// Use the rdcost ratio and source var ratio to prune PARTITION_HORZ and
// PARTITION_VERT.
// TODO(chiyotsai@google.com): Currently this model does not use q value and has
// no information about rectangular partitions. Preliminary experiments suggest
// that we can get better performance by adding in q_index and rectangular
// sse/var from SMS. We should retrain and tune this model later.
void av1_ml_prune_rect_partition(AV1_COMP *const cpi, const MACROBLOCK *const x,
                                 int64_t best_rd, int64_t none_rd,
                                 const int64_t *split_rd,
                                 PartitionSearchState *part_state);

// Use a ML model to predict if horz4 and vert4 should be considered.
void av1_ml_prune_4_partition(AV1_COMP *const cpi, MACROBLOCK *const x,
                              int part_ctx, int64_t best_rd,
                              PartitionSearchState *part_state,
                              int *part4_allowed,
                              unsigned int pb_source_variance);

// ML-based partition search breakout after PARTITION_NONE.
void av1_ml_predict_breakout(AV1_COMP *const cpi, const MACROBLOCK *const x,
                             const RD_STATS *const rd_stats,
                             unsigned int pb_source_variance, int bit_depth,
                             PartitionSearchState *part_state);

// The first round of partition pruning determined before any partition
// has been tested. The decisions will be updated and passed back
// to the partition search function.
void av1_prune_partitions_before_search(AV1_COMP *const cpi,
                                        MACROBLOCK *const x,
                                        SIMPLE_MOTION_DATA_TREE *const sms_tree,
                                        PartitionSearchState *part_state);

// Prune out partitions that lead to coding block sizes outside the min and max
// bsizes set by the encoder. Max and min square partition levels are defined as
// the partition nodes that the recursive function rd_pick_partition() can
// reach. To implement this: only PARTITION_NONE is allowed if the current node
// equals max_partition_size, only PARTITION_SPLIT is allowed if the current
// node exceeds max_partition_size.
void av1_prune_partitions_by_max_min_bsize(SuperBlockEnc *sb_enc,
                                           PartitionSearchState *part_state);

// Prune out AB partitions based on rd decisions made from testing the
// basic partitions.
void av1_prune_ab_partitions(AV1_COMP *cpi, const MACROBLOCK *x,
                             const PC_TREE *pc_tree, int pb_source_variance,
                             int64_t best_rdcost,
                             const RD_RECT_PART_WIN_INFO *rect_part_win_info,
                             bool ext_partition_allowed,
                             PartitionSearchState *part_state,
                             int *ab_partitions_allowed);

void av1_collect_motion_search_features_sb(AV1_COMP *const cpi, ThreadData *td,
                                           TileDataEnc *tile_data,
                                           const int mi_row, const int mi_col,
                                           const BLOCK_SIZE bsize,
                                           aom_partition_features_t *features);
#if CONFIG_PARTITION_SEARCH_ORDER
void av1_prepare_motion_search_features_block(
    AV1_COMP *const cpi, ThreadData *td, TileDataEnc *tile_data,
    const int mi_row, const int mi_col, const BLOCK_SIZE bsize,
    const int valid_partition_types, unsigned int *block_sse,
    unsigned int *block_var, unsigned int sub_block_sse[4],
    unsigned int sub_block_var[4], unsigned int horz_block_sse[2],
    unsigned int horz_block_var[2], unsigned int vert_block_sse[2],
    unsigned int vert_block_var[2]);
#endif  // CONFIG_PARTITION_SEARCH_ORDER
#endif  // !CONFIG_REALTIME_ONLY

// A simplified version of set_offsets meant to be used for
// simple_motion_search.
static inline void set_offsets_for_motion_search(const AV1_COMP *const cpi,
                                                 MACROBLOCK *const x,
                                                 int mi_row, int mi_col,
                                                 BLOCK_SIZE bsize) {
  const AV1_COMMON *const cm = &cpi->common;
  const CommonModeInfoParams *const mi_params = &cm->mi_params;
  const int num_planes = av1_num_planes(cm);
  MACROBLOCKD *const xd = &x->e_mbd;
  const int mi_width = mi_size_wide[bsize];
  const int mi_height = mi_size_high[bsize];

  set_mode_info_offsets(&cpi->common.mi_params, &cpi->mbmi_ext_info, x, xd,
                        mi_row, mi_col);

  // Set up destination pointers.
  av1_setup_dst_planes(xd->plane, bsize, &cm->cur_frame->buf, mi_row, mi_col, 0,
                       num_planes);

  // Set up limit values for MV components.
  // Mv beyond the range do not produce new/different prediction block.
  av1_set_mv_limits(mi_params, &x->mv_limits, mi_row, mi_col, mi_height,
                    mi_width, cpi->oxcf.border_in_pixels);

  set_plane_n4(xd, mi_width, mi_height, num_planes);

  xd->mi_row = mi_row;
  xd->mi_col = mi_col;

  // Set up distance of MB to edge of frame in 1/8th pel units.
  assert(!(mi_col & (mi_width - 1)) && !(mi_row & (mi_height - 1)));
  xd->mb_to_top_edge = -GET_MV_SUBPEL(mi_row * MI_SIZE);
  xd->mb_to_bottom_edge =
      GET_MV_SUBPEL((mi_params->mi_rows - mi_height - mi_row) * MI_SIZE);
  xd->mb_to_left_edge = -GET_MV_SUBPEL(mi_col * MI_SIZE);
  xd->mb_to_right_edge =
      GET_MV_SUBPEL((mi_params->mi_cols - mi_width - mi_col) * MI_SIZE);

  // Set up source buffers.
  av1_setup_src_planes(x, cpi->source, mi_row, mi_col, num_planes, bsize);
}

void av1_init_simple_motion_search_mvs_for_sb(const AV1_COMP *cpi,
                                              const TileInfo *tile_info,
                                              MACROBLOCK *x,
                                              SIMPLE_MOTION_DATA_TREE *sms_root,
                                              int mi_row, int mi_col);

static inline int is_full_sb(const CommonModeInfoParams *const mi_params,
                             int mi_row, int mi_col, BLOCK_SIZE sb_size) {
  const int sb_mi_wide = mi_size_wide[sb_size];
  const int sb_mi_high = mi_size_high[sb_size];

  return (mi_row + sb_mi_high) <= mi_params->mi_rows &&
         (mi_col + sb_mi_wide) <= mi_params->mi_cols;
}

#if !CONFIG_REALTIME_ONLY
// Do not use this criteria for screen content videos.
// Since screen content videos could often find good predictors and the largest
// block size is likely to be used.
static inline int use_auto_max_partition(const AV1_COMP *const cpi,
                                         BLOCK_SIZE sb_size, int mi_row,
                                         int mi_col) {
  assert(IMPLIES(cpi->ppi->gf_group.size > 0,
                 cpi->gf_frame_index < cpi->ppi->gf_group.size));
  const AV1_COMMON *const cm = &cpi->common;
  return !frame_is_intra_only(cm) && !cpi->use_screen_content_tools &&
         cpi->sf.part_sf.auto_max_partition_based_on_simple_motion !=
             NOT_IN_USE &&
         sb_size == BLOCK_128X128 &&
         is_full_sb(&cm->mi_params, mi_row, mi_col, sb_size) &&
         cpi->ppi->gf_group.update_type[cpi->gf_frame_index] !=
             OVERLAY_UPDATE &&
         cpi->ppi->gf_group.update_type[cpi->gf_frame_index] !=
             INTNL_OVERLAY_UPDATE;
}

static BLOCK_SIZE dim_to_size(int dim) {
  switch (dim) {
    case 4: return BLOCK_4X4;
    case 8: return BLOCK_8X8;
    case 16: return BLOCK_16X16;
    case 32: return BLOCK_32X32;
    case 64: return BLOCK_64X64;
    case 128: return BLOCK_128X128;
    default: assert(0); return 0;
  }
}

static inline void set_max_min_partition_size(SuperBlockEnc *sb_enc,
                                              AV1_COMP *cpi, MACROBLOCK *x,
                                              const SPEED_FEATURES *sf,
                                              BLOCK_SIZE sb_size, int mi_row,
                                              int mi_col) {
  const AV1_COMMON *cm = &cpi->common;

  sb_enc->max_partition_size =
      AOMMIN(sf->part_sf.default_max_partition_size,
             dim_to_size(cpi->oxcf.part_cfg.max_partition_size));
  sb_enc->min_partition_size =
      AOMMAX(sf->part_sf.default_min_partition_size,
             dim_to_size(cpi->oxcf.part_cfg.min_partition_size));
  sb_enc->max_partition_size =
      AOMMIN(sb_enc->max_partition_size, cm->seq_params->sb_size);
  sb_enc->min_partition_size =
      AOMMIN(sb_enc->min_partition_size, cm->seq_params->sb_size);

  if (use_auto_max_partition(cpi, sb_size, mi_row, mi_col)) {
    float features[FEATURE_SIZE_MAX_MIN_PART_PRED] = { 0.0f };

    av1_get_max_min_partition_features(cpi, x, mi_row, mi_col, features);
    sb_enc->max_partition_size =
        AOMMAX(AOMMIN(av1_predict_max_partition(cpi, x, features),
                      sb_enc->max_partition_size),
               sb_enc->min_partition_size);
  }
}
#endif  // !CONFIG_REALTIME_ONLY
#endif  // AOM_AV1_ENCODER_PARTITION_STRATEGY_H_
