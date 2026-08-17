/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_ALLINTRA_VIS_H_
#define AOM_AV1_ENCODER_ALLINTRA_VIS_H_

#include "config/aom_dsp_rtcd.h"

#include "av1/common/enums.h"
#include "av1/common/reconintra.h"

#include "av1/encoder/block.h"
#include "av1/encoder/encoder.h"

#define MB_WIENER_MT_UNIT_SIZE BLOCK_64X64

void av1_init_mb_wiener_var_buffer(AV1_COMP *cpi);

void av1_calc_mb_wiener_var_row(AV1_COMP *const cpi, MACROBLOCK *x,
                                MACROBLOCKD *xd, const int mi_row,
                                int16_t *src_diff, tran_low_t *coeff,
                                tran_low_t *qcoeff, tran_low_t *dqcoeff,
                                double *sum_rec_distortion,
                                double *sum_est_rate, uint8_t *pred_buffer);

void av1_set_mb_wiener_variance(AV1_COMP *cpi);

int av1_get_sbq_perceptual_ai(const AV1_COMP *const cpi, BLOCK_SIZE bsize,
                              int mi_row, int mi_col);

// User rating based mode
void av1_init_mb_ur_var_buffer(AV1_COMP *cpi);

void av1_set_mb_ur_variance(AV1_COMP *cpi);

int av1_get_sbq_user_rating_based(const AV1_COMP *const cpi, int mi_row,
                                  int mi_col);

#if !CONFIG_REALTIME_ONLY
int av1_get_sbq_variance_boost(const AV1_COMP *const cpi, const MACROBLOCK *x);
#endif

#endif  // AOM_AV1_ENCODER_ALLINTRA_VIS_H_
