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

#ifndef AOM_AV1_ENCODER_COMPOUND_TYPE_H_
#define AOM_AV1_ENCODER_COMPOUND_TYPE_H_

#include "av1/encoder/encoder.h"
#include "av1/encoder/interp_search.h"

#ifdef __cplusplus
extern "C" {
#endif

// Structure to store the compound type related stats for best compound type
typedef struct {
  INTERINTER_COMPOUND_DATA best_compound_data;
  int64_t comp_best_model_rd;
  int best_compmode_interinter_cost;
} BEST_COMP_TYPE_STATS;

#define IGNORE_MODE -1
// Searches for the best inter-intra mode. Returns IGNORE_MODE if no good mode
// is found, 0 otherwise.
int av1_handle_inter_intra_mode(const AV1_COMP *const cpi, MACROBLOCK *const x,
                                BLOCK_SIZE bsize, MB_MODE_INFO *mbmi,
                                HandleInterModeArgs *args, int64_t ref_best_rd,
                                int *rate_mv, int *tmp_rate2,
                                const BUFFER_SET *orig_dst);

int av1_compound_type_rd(const AV1_COMP *const cpi, MACROBLOCK *x,
                         HandleInterModeArgs *args, BLOCK_SIZE bsize,
                         int_mv *cur_mv, int mode_search_mask,
                         int masked_compound_used, const BUFFER_SET *orig_dst,
                         const BUFFER_SET *tmp_dst,
                         const CompoundTypeRdBuffers *buffers, int *rate_mv,
                         int64_t *rd, RD_STATS *rd_stats, int64_t ref_best_rd,
                         int64_t ref_skip_rd, int *is_luma_interp_done,
                         int64_t rd_thresh);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_COMPOUND_TYPE_H_
