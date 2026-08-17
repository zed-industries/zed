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

#ifndef AOM_AV1_ENCODER_AV1_FWD_TXFM1D_CFG_H_
#define AOM_AV1_ENCODER_AV1_FWD_TXFM1D_CFG_H_
#include "av1/common/enums.h"
#include "av1/encoder/av1_fwd_txfm1d.h"
extern const int8_t *av1_fwd_txfm_shift_ls[TX_SIZES_ALL];
extern const int8_t av1_fwd_cos_bit_col[5][5];
extern const int8_t av1_fwd_cos_bit_row[5][5];
#endif  // AOM_AV1_ENCODER_AV1_FWD_TXFM1D_CFG_H_
