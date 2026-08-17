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

#ifndef AOM_TEST_AV1_TXFM_TEST_H_
#define AOM_TEST_AV1_TXFM_TEST_H_

#include <stdio.h>
#include <stdlib.h>
#ifdef _MSC_VER
#define _USE_MATH_DEFINES
#endif
#include <math.h>

#include "config/av1_rtcd.h"

#include "gtest/gtest.h"

#include "test/acm_random.h"
#include "av1/common/av1_txfm.h"
#include "av1/common/blockd.h"
#include "av1/common/enums.h"

namespace libaom_test {

extern const char *tx_type_name[];

enum {
  TYPE_DCT = 0,
  TYPE_ADST,
  TYPE_IDTX,
  TYPE_IDCT,
  TYPE_IADST,
  TYPE_LAST
} UENUM1BYTE(TYPE_TXFM);

int get_txfm1d_size(TX_SIZE tx_size);

void get_txfm1d_type(TX_TYPE txfm2d_type, TYPE_TXFM *type0, TYPE_TXFM *type1);

void reference_dct_1d(const double *in, double *out, int size);
void reference_idct_1d(const double *in, double *out, int size);

void reference_adst_1d(const double *in, double *out, int size);

void reference_hybrid_1d(double *in, double *out, int size, int type);

double get_amplification_factor(TX_TYPE tx_type, TX_SIZE tx_size);

void reference_hybrid_2d(double *in, double *out, TX_TYPE tx_type,
                         TX_SIZE tx_size);
template <typename Type1, typename Type2>
static double compute_avg_abs_error(const Type1 *a, const Type2 *b,
                                    const int size) {
  double error = 0;
  for (int i = 0; i < size; i++) {
    error += fabs(static_cast<double>(a[i]) - static_cast<double>(b[i]));
  }
  error = error / size;
  return error;
}

template <typename Type>
void fliplr(Type *dest, int width, int height, int stride);

template <typename Type>
void flipud(Type *dest, int width, int height, int stride);

template <typename Type>
void fliplrud(Type *dest, int width, int height, int stride);

typedef void (*TxfmFunc)(const int32_t *in, int32_t *out, const int8_t cos_bit,
                         const int8_t *range_bit);

typedef void (*InvTxfm2dFunc)(const int32_t *, uint16_t *, int, TX_TYPE, int);
typedef void (*LbdInvTxfm2dFunc)(const int32_t *, uint8_t *, int, TX_TYPE,
                                 TX_SIZE, int);

static const int bd = 10;
static const int input_base = (1 << bd);

static inline bool IsTxSizeTypeValid(TX_SIZE tx_size, TX_TYPE tx_type) {
  const TX_SIZE tx_size_sqr_up = txsize_sqr_up_map[tx_size];
  TxSetType tx_set_type;
  if (tx_size_sqr_up > TX_32X32) {
    tx_set_type = EXT_TX_SET_DCTONLY;
  } else if (tx_size_sqr_up == TX_32X32) {
    tx_set_type = EXT_TX_SET_DCT_IDTX;
  } else {
    tx_set_type = EXT_TX_SET_ALL16;
  }
  return av1_ext_tx_used[tx_set_type][tx_type] != 0;
}

#if CONFIG_AV1_ENCODER
#if !CONFIG_REALTIME_ONLY
static const FwdTxfm2dFunc fwd_txfm_func_ls[TX_SIZES_ALL] = {
  av1_fwd_txfm2d_4x4_c,   av1_fwd_txfm2d_8x8_c,   av1_fwd_txfm2d_16x16_c,
  av1_fwd_txfm2d_32x32_c, av1_fwd_txfm2d_64x64_c, av1_fwd_txfm2d_4x8_c,
  av1_fwd_txfm2d_8x4_c,   av1_fwd_txfm2d_8x16_c,  av1_fwd_txfm2d_16x8_c,
  av1_fwd_txfm2d_16x32_c, av1_fwd_txfm2d_32x16_c, av1_fwd_txfm2d_32x64_c,
  av1_fwd_txfm2d_64x32_c, av1_fwd_txfm2d_4x16_c,  av1_fwd_txfm2d_16x4_c,
  av1_fwd_txfm2d_8x32_c,  av1_fwd_txfm2d_32x8_c,  av1_fwd_txfm2d_16x64_c,
  av1_fwd_txfm2d_64x16_c,
};
#else
static const FwdTxfm2dFunc fwd_txfm_func_ls[TX_SIZES_ALL] = {
  av1_fwd_txfm2d_4x4_c,
  av1_fwd_txfm2d_8x8_c,
  av1_fwd_txfm2d_16x16_c,
  av1_fwd_txfm2d_32x32_c,
  av1_fwd_txfm2d_64x64_c,
  av1_fwd_txfm2d_4x8_c,
  av1_fwd_txfm2d_8x4_c,
  av1_fwd_txfm2d_8x16_c,
  av1_fwd_txfm2d_16x8_c,
  av1_fwd_txfm2d_16x32_c,
  av1_fwd_txfm2d_32x16_c,
  av1_fwd_txfm2d_32x64_c,
  av1_fwd_txfm2d_64x32_c,
  nullptr,
  av1_fwd_txfm2d_16x4_c,
  nullptr,
  nullptr,
  nullptr,
  nullptr,
};
#endif
#endif

static const InvTxfm2dFunc inv_txfm_func_ls[TX_SIZES_ALL] = {
  av1_inv_txfm2d_add_4x4_c,   av1_inv_txfm2d_add_8x8_c,
  av1_inv_txfm2d_add_16x16_c, av1_inv_txfm2d_add_32x32_c,
  av1_inv_txfm2d_add_64x64_c, av1_inv_txfm2d_add_4x8_c,
  av1_inv_txfm2d_add_8x4_c,   av1_inv_txfm2d_add_8x16_c,
  av1_inv_txfm2d_add_16x8_c,  av1_inv_txfm2d_add_16x32_c,
  av1_inv_txfm2d_add_32x16_c, av1_inv_txfm2d_add_32x64_c,
  av1_inv_txfm2d_add_64x32_c, av1_inv_txfm2d_add_4x16_c,
  av1_inv_txfm2d_add_16x4_c,  av1_inv_txfm2d_add_8x32_c,
  av1_inv_txfm2d_add_32x8_c,  av1_inv_txfm2d_add_16x64_c,
  av1_inv_txfm2d_add_64x16_c,
};

#define BD_NUM 3

extern int bd_arr[];
extern int8_t low_range_arr[];
extern int8_t high_range_arr[];

void txfm_stage_range_check(const int8_t *stage_range, int stage_num,
                            const int8_t cos_bit, int low_range,
                            int high_range);
}  // namespace libaom_test
#endif  // AOM_TEST_AV1_TXFM_TEST_H_
