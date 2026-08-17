/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_DWT_H_
#define AOM_AV1_ENCODER_DWT_H_

#include "av1/common/common.h"
#include "av1/common/enums.h"

#define DWT_MAX_LENGTH 64

int64_t av1_haar_ac_sad_mxn_uint8_input(const uint8_t *input, int stride,
                                        int hbd, int num_8x8_rows,
                                        int num_8x8_cols);

#endif  // AOM_AV1_ENCODER_DWT_H_
