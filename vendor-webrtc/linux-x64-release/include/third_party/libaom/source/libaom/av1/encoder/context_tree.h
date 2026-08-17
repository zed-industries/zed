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

#ifndef AOM_AV1_ENCODER_CONTEXT_TREE_H_
#define AOM_AV1_ENCODER_CONTEXT_TREE_H_

#include "config/aom_config.h"

#include "av1/common/blockd.h"
#include "av1/encoder/block.h"
#include "av1/encoder/speed_features.h"

#ifdef __cplusplus
extern "C" {
#endif

struct AV1_PRIMARY;
struct AV1_COMP;
struct AV1Common;
struct ThreadData;

typedef struct {
  tran_low_t *coeff_buf[MAX_MB_PLANE];
  tran_low_t *qcoeff_buf[MAX_MB_PLANE];
  tran_low_t *dqcoeff_buf[MAX_MB_PLANE];
} PC_TREE_SHARED_BUFFERS;

// Structure to hold snapshot of coding context during the mode picking process
typedef struct PICK_MODE_CONTEXT {
  MB_MODE_INFO mic;
  MB_MODE_INFO_EXT_FRAME mbmi_ext_best;
  uint8_t *color_index_map[2];
  uint8_t *blk_skip;

  tran_low_t *coeff[MAX_MB_PLANE];
  tran_low_t *qcoeff[MAX_MB_PLANE];
  tran_low_t *dqcoeff[MAX_MB_PLANE];
  uint16_t *eobs[MAX_MB_PLANE];
  uint8_t *txb_entropy_ctx[MAX_MB_PLANE];
  uint8_t *tx_type_map;

  int num_4x4_blk;
  // For current partition, only if all Y, U, and V transform blocks'
  // coefficients are quantized to 0, skippable is set to 1.
  int skippable;
#if CONFIG_INTERNAL_STATS
  THR_MODES best_mode_index;
#endif  // CONFIG_INTERNAL_STATS
  RD_STATS rd_stats;

  int rd_mode_is_ready;  // Flag to indicate whether rd pick mode decision has
                         // been made.
#if CONFIG_AV1_TEMPORAL_DENOISING
  int64_t newmv_sse;
  int64_t zeromv_sse;
  int64_t zeromv_lastref_sse;
  PREDICTION_MODE best_sse_inter_mode;
  int_mv best_sse_mv;
  MV_REFERENCE_FRAME best_reference_frame;
  MV_REFERENCE_FRAME best_zeromv_reference_frame;
  int sb_skip_denoising;
#endif
} PICK_MODE_CONTEXT;

typedef struct PC_TREE {
  PARTITION_TYPE partitioning;
  BLOCK_SIZE block_size;
  PICK_MODE_CONTEXT *none;
  PICK_MODE_CONTEXT *horizontal[2];
  PICK_MODE_CONTEXT *vertical[2];
#if !CONFIG_REALTIME_ONLY
  PICK_MODE_CONTEXT *horizontala[3];
  PICK_MODE_CONTEXT *horizontalb[3];
  PICK_MODE_CONTEXT *verticala[3];
  PICK_MODE_CONTEXT *verticalb[3];
  PICK_MODE_CONTEXT *horizontal4[4];
  PICK_MODE_CONTEXT *vertical4[4];
#endif
  struct PC_TREE *split[4];
  int index;
} PC_TREE;

typedef struct SIMPLE_MOTION_DATA_TREE {
  BLOCK_SIZE block_size;
  PARTITION_TYPE partitioning;
  struct SIMPLE_MOTION_DATA_TREE *split[4];

  // Simple motion search_features
  FULLPEL_MV start_mvs[REF_FRAMES];
  unsigned int sms_none_feat[2];
  unsigned int sms_rect_feat[8];
  int sms_none_valid;
  int sms_rect_valid;
} SIMPLE_MOTION_DATA_TREE;

void av1_setup_shared_coeff_buffer(const SequenceHeader *const seq_params,
                                   PC_TREE_SHARED_BUFFERS *shared_bufs,
                                   struct aom_internal_error_info *error);
void av1_free_shared_coeff_buffer(PC_TREE_SHARED_BUFFERS *shared_bufs);

PC_TREE *av1_alloc_pc_tree_node(BLOCK_SIZE bsize);
void av1_free_pc_tree_recursive(PC_TREE *tree, int num_planes, int keep_best,
                                int keep_none,
                                PARTITION_SEARCH_TYPE partition_search_type);

PICK_MODE_CONTEXT *av1_alloc_pmc(const struct AV1_COMP *const cpi,
                                 BLOCK_SIZE bsize,
                                 PC_TREE_SHARED_BUFFERS *shared_bufs);
void av1_reset_pmc(PICK_MODE_CONTEXT *ctx);
void av1_free_pmc(PICK_MODE_CONTEXT *ctx, int num_planes);
void av1_copy_tree_context(PICK_MODE_CONTEXT *dst_ctx,
                           PICK_MODE_CONTEXT *src_ctx);

static const BLOCK_SIZE square[MAX_SB_SIZE_LOG2 - 1] = {
  BLOCK_4X4, BLOCK_8X8, BLOCK_16X16, BLOCK_32X32, BLOCK_64X64, BLOCK_128X128,
};

static inline int av1_get_pc_tree_nodes(const int is_sb_size_128,
                                        int stat_generation_stage) {
  const int tree_nodes_inc = is_sb_size_128 ? 1024 : 0;
  const int tree_nodes =
      stat_generation_stage ? 1 : (tree_nodes_inc + 256 + 64 + 16 + 4 + 1);
  return tree_nodes;
}

// Returns 0 on success, -1 on memory allocation failure.
int av1_setup_sms_tree(struct AV1_COMP *const cpi, struct ThreadData *td);
void av1_free_sms_tree(struct ThreadData *td);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_CONTEXT_TREE_H_
