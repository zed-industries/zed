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

#ifndef AOM_AV1_ENCODER_PARTITION_SEARCH_H_
#define AOM_AV1_ENCODER_PARTITION_SEARCH_H_

#include "config/aom_config.h"

#include "av1/encoder/block.h"
#include "av1/encoder/encoder.h"
#include "av1/encoder/encodeframe.h"
#include "av1/encoder/tokenize.h"

void av1_set_offsets_without_segment_id(const AV1_COMP *const cpi,
                                        const TileInfo *const tile,
                                        MACROBLOCK *const x, int mi_row,
                                        int mi_col, BLOCK_SIZE bsize);
void av1_set_offsets(const AV1_COMP *const cpi, const TileInfo *const tile,
                     MACROBLOCK *const x, int mi_row, int mi_col,
                     BLOCK_SIZE bsize);
void av1_rd_use_partition(AV1_COMP *cpi, ThreadData *td, TileDataEnc *tile_data,
                          MB_MODE_INFO **mib, TokenExtra **tp, int mi_row,
                          int mi_col, BLOCK_SIZE bsize, int *rate,
                          int64_t *dist, int do_recon, PC_TREE *pc_tree);
void av1_nonrd_use_partition(AV1_COMP *cpi, ThreadData *td,
                             TileDataEnc *tile_data, MB_MODE_INFO **mib,
                             TokenExtra **tp, int mi_row, int mi_col,
                             BLOCK_SIZE bsize, PC_TREE *pc_tree);
#if CONFIG_RT_ML_PARTITIONING
void av1_nonrd_pick_partition(AV1_COMP *cpi, ThreadData *td,
                              TileDataEnc *tile_data, TokenExtra **tp,
                              int mi_row, int mi_col, BLOCK_SIZE bsize,
                              RD_STATS *rd_cost, int do_recon, int64_t best_rd,
                              PC_TREE *pc_tree);
#endif

#if CONFIG_PARTITION_SEARCH_ORDER
void av1_reset_part_sf(PARTITION_SPEED_FEATURES *part_sf);
void av1_reset_sf_for_ext_part(AV1_COMP *const cpi);
bool av1_rd_partition_search(AV1_COMP *const cpi, ThreadData *td,
                             TileDataEnc *tile_data, TokenExtra **tp,
                             SIMPLE_MOTION_DATA_TREE *sms_root, int mi_row,
                             int mi_col, BLOCK_SIZE bsize,
                             RD_STATS *best_rd_cost);
#endif

bool av1_rd_pick_partition(AV1_COMP *const cpi, ThreadData *td,
                           TileDataEnc *tile_data, TokenExtra **tp, int mi_row,
                           int mi_col, BLOCK_SIZE bsize, RD_STATS *rd_cost,
                           RD_STATS best_rdc, PC_TREE *pc_tree,
                           SIMPLE_MOTION_DATA_TREE *sms_tree, int64_t *none_rd,
                           SB_MULTI_PASS_MODE multi_pass_mode,
                           RD_RECT_PART_WIN_INFO *rect_part_win_info);

static inline void set_cb_offsets(uint16_t *cb_offset,
                                  const uint16_t cb_offset_y,
                                  const uint16_t cb_offset_uv) {
  cb_offset[PLANE_TYPE_Y] = cb_offset_y;
  cb_offset[PLANE_TYPE_UV] = cb_offset_uv;
}

static inline void update_cb_offsets(MACROBLOCK *x, const BLOCK_SIZE bsize,
                                     const int subsampling_x,
                                     const int subsampling_y) {
  x->cb_offset[PLANE_TYPE_Y] += block_size_wide[bsize] * block_size_high[bsize];
  if (x->e_mbd.is_chroma_ref) {
    const BLOCK_SIZE plane_bsize =
        get_plane_block_size(bsize, subsampling_x, subsampling_y);
    assert(plane_bsize != BLOCK_INVALID);
    x->cb_offset[PLANE_TYPE_UV] +=
        block_size_wide[plane_bsize] * block_size_high[plane_bsize];
  }
}

#endif  // AOM_AV1_ENCODER_PARTITION_SEARCH_H_
