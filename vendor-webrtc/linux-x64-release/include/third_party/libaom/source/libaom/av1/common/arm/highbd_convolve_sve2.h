/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_ARM_HIGHBD_CONVOLVE_SVE2_H_
#define AOM_AV1_COMMON_ARM_HIGHBD_CONVOLVE_SVE2_H_

#include <arm_neon.h>

#include "aom_dsp/arm/aom_neon_sve2_bridge.h"

// clang-format off
DECLARE_ALIGNED(16, static const uint16_t, kDotProdMergeBlockTbl[24]) = {
  // Shift left and insert new last column in transposed 4x4 block.
  1, 2, 3, 0, 5, 6, 7, 4,
  // Shift left and insert two new columns in transposed 4x4 block.
  2, 3, 0, 1, 6, 7, 4, 5,
  // Shift left and insert three new columns in transposed 4x4 block.
  3, 0, 1, 2, 7, 4, 5, 6,
};
// clang-format on

static inline void transpose_concat_4x4(int16x4_t s0, int16x4_t s1,
                                        int16x4_t s2, int16x4_t s3,
                                        int16x8_t res[2]) {
  // Transpose 16-bit elements and concatenate result rows as follows:
  // s0: 00, 01, 02, 03
  // s1: 10, 11, 12, 13
  // s2: 20, 21, 22, 23
  // s3: 30, 31, 32, 33
  //
  // res[0]: 00 10 20 30 01 11 21 31
  // res[1]: 02 12 22 32 03 13 23 33

  int16x8_t s0q = vcombine_s16(s0, vdup_n_s16(0));
  int16x8_t s1q = vcombine_s16(s1, vdup_n_s16(0));
  int16x8_t s2q = vcombine_s16(s2, vdup_n_s16(0));
  int16x8_t s3q = vcombine_s16(s3, vdup_n_s16(0));

  int32x4_t s01 = vreinterpretq_s32_s16(vzip1q_s16(s0q, s1q));
  int32x4_t s23 = vreinterpretq_s32_s16(vzip1q_s16(s2q, s3q));

  int32x4x2_t s0123 = vzipq_s32(s01, s23);

  res[0] = vreinterpretq_s16_s32(s0123.val[0]);
  res[1] = vreinterpretq_s16_s32(s0123.val[1]);
}

static inline void transpose_concat_8x4(int16x8_t s0, int16x8_t s1,
                                        int16x8_t s2, int16x8_t s3,
                                        int16x8_t res[4]) {
  // Transpose 16-bit elements and concatenate result rows as follows:
  // s0: 00, 01, 02, 03, 04, 05, 06, 07
  // s1: 10, 11, 12, 13, 14, 15, 16, 17
  // s2: 20, 21, 22, 23, 24, 25, 26, 27
  // s3: 30, 31, 32, 33, 34, 35, 36, 37
  //
  // res[0]: 00 10 20 30 01 11 21 31
  // res[1]: 02 12 22 32 03 13 23 33
  // res[2]: 04 14 24 34 05 15 25 35
  // res[3]: 06 16 26 36 07 17 27 37

  int16x8x2_t tr01_16 = vzipq_s16(s0, s1);
  int16x8x2_t tr23_16 = vzipq_s16(s2, s3);
  int32x4x2_t tr01_32 = vzipq_s32(vreinterpretq_s32_s16(tr01_16.val[0]),
                                  vreinterpretq_s32_s16(tr23_16.val[0]));
  int32x4x2_t tr23_32 = vzipq_s32(vreinterpretq_s32_s16(tr01_16.val[1]),
                                  vreinterpretq_s32_s16(tr23_16.val[1]));

  res[0] = vreinterpretq_s16_s32(tr01_32.val[0]);
  res[1] = vreinterpretq_s16_s32(tr01_32.val[1]);
  res[2] = vreinterpretq_s16_s32(tr23_32.val[0]);
  res[3] = vreinterpretq_s16_s32(tr23_32.val[1]);
}

static inline void aom_tbl2x4_s16(int16x8_t t0[4], int16x8_t t1[4],
                                  uint16x8_t tbl, int16x8_t res[4]) {
  res[0] = aom_tbl2_s16(t0[0], t1[0], tbl);
  res[1] = aom_tbl2_s16(t0[1], t1[1], tbl);
  res[2] = aom_tbl2_s16(t0[2], t1[2], tbl);
  res[3] = aom_tbl2_s16(t0[3], t1[3], tbl);
}

static inline void aom_tbl2x2_s16(int16x8_t t0[2], int16x8_t t1[2],
                                  uint16x8_t tbl, int16x8_t res[2]) {
  res[0] = aom_tbl2_s16(t0[0], t1[0], tbl);
  res[1] = aom_tbl2_s16(t0[1], t1[1], tbl);
}

#endif  // AOM_AV1_COMMON_ARM_HIGHBD_CONVOLVE_SVE2_H_
