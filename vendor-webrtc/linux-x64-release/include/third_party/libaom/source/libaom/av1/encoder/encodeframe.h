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

#ifndef AOM_AV1_ENCODER_ENCODEFRAME_H_
#define AOM_AV1_ENCODER_ENCODEFRAME_H_

#include "aom/aom_integer.h"
#include "av1/common/blockd.h"
#include "av1/common/enums.h"

#include "av1/encoder/global_motion.h"

#ifdef __cplusplus
extern "C" {
#endif

#define DELTA_Q_PERCEPTUAL_MODULATION \
  1  // 0: variance based
     // 1: wavelet AC energy based

struct macroblock;
struct yv12_buffer_config;
struct AV1_COMP;
struct ThreadData;

void av1_init_rtc_counters(struct macroblock *const x);

void av1_accumulate_rtc_counters(struct AV1_COMP *cpi,
                                 const struct macroblock *const x);

void av1_setup_src_planes(struct macroblock *x,
                          const struct yv12_buffer_config *src, int mi_row,
                          int mi_col, const int num_planes, BLOCK_SIZE bsize);

void av1_encode_frame(struct AV1_COMP *cpi);

void av1_alloc_tile_data(struct AV1_COMP *cpi);
void av1_init_tile_data(struct AV1_COMP *cpi);
void av1_encode_tile(struct AV1_COMP *cpi, struct ThreadData *td, int tile_row,
                     int tile_col);
void av1_encode_sb_row(struct AV1_COMP *cpi, struct ThreadData *td,
                       int tile_row, int tile_col, int mi_row);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ENCODEFRAME_H_
