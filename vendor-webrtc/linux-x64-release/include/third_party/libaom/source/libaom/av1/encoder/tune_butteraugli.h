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

#ifndef AOM_AV1_ENCODER_TUNE_BUTTERAUGLI_H_
#define AOM_AV1_ENCODER_TUNE_BUTTERAUGLI_H_

#include "aom_scale/yv12config.h"
#include "av1/common/enums.h"
#include "av1/encoder/ratectrl.h"
#include "av1/encoder/block.h"

typedef struct {
  // Stores the scaling factors for rdmult when tuning for Butteraugli.
  // rdmult_scaling_factors[row * num_cols + col] stores the scaling factors for
  // 4x4 block at (row, col).
  double *rdmult_scaling_factors;
  YV12_BUFFER_CONFIG source, resized_source;
  bool recon_set;
} TuneButteraugliInfo;

struct AV1_COMP;
static const BLOCK_SIZE butteraugli_rdo_bsize = BLOCK_16X16;

void av1_set_butteraugli_rdmult(const struct AV1_COMP *cpi, MACROBLOCK *x,
                                BLOCK_SIZE bsize, int mi_row, int mi_col,
                                int *rdmult);

void av1_setup_butteraugli_source(struct AV1_COMP *cpi);

// 'K' is used to balance the rate-distortion distribution between PSNR
// and Butteraugli.
void av1_setup_butteraugli_rdmult_and_restore_source(struct AV1_COMP *cpi,
                                                     double K);

void av1_setup_butteraugli_rdmult(struct AV1_COMP *cpi);

#endif  // AOM_AV1_ENCODER_TUNE_BUTTERAUGLI_H_
