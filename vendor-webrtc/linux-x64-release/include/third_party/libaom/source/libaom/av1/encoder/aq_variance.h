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

#ifndef AOM_AV1_ENCODER_AQ_VARIANCE_H_
#define AOM_AV1_ENCODER_AQ_VARIANCE_H_

#include "av1/encoder/encoder.h"
#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

#if !CONFIG_REALTIME_ONLY
void av1_vaq_frame_setup(AV1_COMP *cpi);

int av1_log_block_avg(const AV1_COMP *cpi, const MACROBLOCK *x, BLOCK_SIZE bs,
                      int mi_row, int mi_col);
int av1_compute_q_from_energy_level_deltaq_mode(const AV1_COMP *const cpi,
                                                int block_var_level);
int av1_block_wavelet_energy_level(const AV1_COMP *cpi, const MACROBLOCK *x,
                                   BLOCK_SIZE bs);

unsigned int av1_get_variance_boost_block_variance(const AV1_COMP *cpi,
                                                   const MACROBLOCK *x);
#endif  // !CONFIG_REALTIME_ONLY

int av1_log_block_var(const AV1_COMP *cpi, const MACROBLOCK *x, BLOCK_SIZE bs);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_AQ_VARIANCE_H_
