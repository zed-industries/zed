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

#ifndef AOM_AV1_COMMON_AV1_INV_TXFM1D_CFG_H_
#define AOM_AV1_COMMON_AV1_INV_TXFM1D_CFG_H_
#include "av1/common/av1_inv_txfm1d.h"

// sum of fwd_shift_##
static const int8_t inv_start_range[TX_SIZES_ALL] = {
  5,  // 4x4 transform
  6,  // 8x8 transform
  7,  // 16x16 transform
  7,  // 32x32 transform
  7,  // 64x64 transform
  5,  // 4x8 transform
  5,  // 8x4 transform
  6,  // 8x16 transform
  6,  // 16x8 transform
  6,  // 16x32 transform
  6,  // 32x16 transform
  6,  // 32x64 transform
  6,  // 64x32 transform
  6,  // 4x16 transform
  6,  // 16x4 transform
  7,  // 8x32 transform
  7,  // 32x8 transform
  7,  // 16x64 transform
  7,  // 64x16 transform
};

extern const int8_t *av1_inv_txfm_shift_ls[TX_SIZES_ALL];

// Values in both av1_inv_cos_bit_col and av1_inv_cos_bit_row are always 12
// for each valid row and col combination
#define INV_COS_BIT 12

#endif  // AOM_AV1_COMMON_AV1_INV_TXFM1D_CFG_H_
