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

#ifndef AOM_AV1_COMMON_ARM_HIGHBD_CONVOLVE_NEON_H_
#define AOM_AV1_COMMON_ARM_HIGHBD_CONVOLVE_NEON_H_

#include <arm_neon.h>

#include "aom_dsp/arm/mem_neon.h"
#include "aom_dsp/arm/transpose_neon.h"
#include "av1/common/convolve.h"

static inline int32x4_t highbd_convolve8_4_s32(
    const int16x4_t s0, const int16x4_t s1, const int16x4_t s2,
    const int16x4_t s3, const int16x4_t s4, const int16x4_t s5,
    const int16x4_t s6, const int16x4_t s7, const int16x8_t y_filter,
    const int32x4_t offset) {
  const int16x4_t y_filter_lo = vget_low_s16(y_filter);
  const int16x4_t y_filter_hi = vget_high_s16(y_filter);

  int32x4_t sum = vmlal_lane_s16(offset, s0, y_filter_lo, 0);
  sum = vmlal_lane_s16(sum, s1, y_filter_lo, 1);
  sum = vmlal_lane_s16(sum, s2, y_filter_lo, 2);
  sum = vmlal_lane_s16(sum, s3, y_filter_lo, 3);
  sum = vmlal_lane_s16(sum, s4, y_filter_hi, 0);
  sum = vmlal_lane_s16(sum, s5, y_filter_hi, 1);
  sum = vmlal_lane_s16(sum, s6, y_filter_hi, 2);
  sum = vmlal_lane_s16(sum, s7, y_filter_hi, 3);

  return sum;
}

static inline uint16x4_t highbd_convolve8_4_sr_s32_s16(
    const int16x4_t s0, const int16x4_t s1, const int16x4_t s2,
    const int16x4_t s3, const int16x4_t s4, const int16x4_t s5,
    const int16x4_t s6, const int16x4_t s7, const int16x8_t y_filter,
    const int32x4_t shift_s32, const int32x4_t offset) {
  int32x4_t sum =
      highbd_convolve8_4_s32(s0, s1, s2, s3, s4, s5, s6, s7, y_filter, offset);

  sum = vqrshlq_s32(sum, shift_s32);
  return vqmovun_s32(sum);
}

// Like above but also perform round shifting and subtract correction term
static inline uint16x4_t highbd_convolve8_4_srsub_s32_s16(
    const int16x4_t s0, const int16x4_t s1, const int16x4_t s2,
    const int16x4_t s3, const int16x4_t s4, const int16x4_t s5,
    const int16x4_t s6, const int16x4_t s7, const int16x8_t y_filter,
    const int32x4_t round_shift, const int32x4_t offset,
    const int32x4_t correction) {
  int32x4_t sum =
      highbd_convolve8_4_s32(s0, s1, s2, s3, s4, s5, s6, s7, y_filter, offset);

  sum = vsubq_s32(vqrshlq_s32(sum, round_shift), correction);
  return vqmovun_s32(sum);
}

static inline void highbd_convolve8_8_s32(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x8_t s4, const int16x8_t s5,
    const int16x8_t s6, const int16x8_t s7, const int16x8_t y_filter,
    const int32x4_t offset, int32x4_t *sum0, int32x4_t *sum1) {
  const int16x4_t y_filter_lo = vget_low_s16(y_filter);
  const int16x4_t y_filter_hi = vget_high_s16(y_filter);

  *sum0 = vmlal_lane_s16(offset, vget_low_s16(s0), y_filter_lo, 0);
  *sum0 = vmlal_lane_s16(*sum0, vget_low_s16(s1), y_filter_lo, 1);
  *sum0 = vmlal_lane_s16(*sum0, vget_low_s16(s2), y_filter_lo, 2);
  *sum0 = vmlal_lane_s16(*sum0, vget_low_s16(s3), y_filter_lo, 3);
  *sum0 = vmlal_lane_s16(*sum0, vget_low_s16(s4), y_filter_hi, 0);
  *sum0 = vmlal_lane_s16(*sum0, vget_low_s16(s5), y_filter_hi, 1);
  *sum0 = vmlal_lane_s16(*sum0, vget_low_s16(s6), y_filter_hi, 2);
  *sum0 = vmlal_lane_s16(*sum0, vget_low_s16(s7), y_filter_hi, 3);

  *sum1 = vmlal_lane_s16(offset, vget_high_s16(s0), y_filter_lo, 0);
  *sum1 = vmlal_lane_s16(*sum1, vget_high_s16(s1), y_filter_lo, 1);
  *sum1 = vmlal_lane_s16(*sum1, vget_high_s16(s2), y_filter_lo, 2);
  *sum1 = vmlal_lane_s16(*sum1, vget_high_s16(s3), y_filter_lo, 3);
  *sum1 = vmlal_lane_s16(*sum1, vget_high_s16(s4), y_filter_hi, 0);
  *sum1 = vmlal_lane_s16(*sum1, vget_high_s16(s5), y_filter_hi, 1);
  *sum1 = vmlal_lane_s16(*sum1, vget_high_s16(s6), y_filter_hi, 2);
  *sum1 = vmlal_lane_s16(*sum1, vget_high_s16(s7), y_filter_hi, 3);
}

// Like above but also perform round shifting and subtract correction term
static inline uint16x8_t highbd_convolve8_8_srsub_s32_s16(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x8_t s4, const int16x8_t s5,
    const int16x8_t s6, const int16x8_t s7, const int16x8_t y_filter,
    const int32x4_t round_shift, const int32x4_t offset,
    const int32x4_t correction) {
  int32x4_t sum0;
  int32x4_t sum1;
  highbd_convolve8_8_s32(s0, s1, s2, s3, s4, s5, s6, s7, y_filter, offset,
                         &sum0, &sum1);

  sum0 = vsubq_s32(vqrshlq_s32(sum0, round_shift), correction);
  sum1 = vsubq_s32(vqrshlq_s32(sum1, round_shift), correction);

  return vcombine_u16(vqmovun_s32(sum0), vqmovun_s32(sum1));
}

static inline int32x4_t highbd_convolve8_2d_scale_horiz4x8_s32(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x4_t *filters_lo,
    const int16x4_t *filters_hi, const int32x4_t offset) {
  int16x4_t s_lo[] = { vget_low_s16(s0), vget_low_s16(s1), vget_low_s16(s2),
                       vget_low_s16(s3) };
  int16x4_t s_hi[] = { vget_high_s16(s0), vget_high_s16(s1), vget_high_s16(s2),
                       vget_high_s16(s3) };

  transpose_array_inplace_u16_4x4((uint16x4_t *)s_lo);
  transpose_array_inplace_u16_4x4((uint16x4_t *)s_hi);

  int32x4_t sum = vmlal_s16(offset, s_lo[0], filters_lo[0]);
  sum = vmlal_s16(sum, s_lo[1], filters_lo[1]);
  sum = vmlal_s16(sum, s_lo[2], filters_lo[2]);
  sum = vmlal_s16(sum, s_lo[3], filters_lo[3]);
  sum = vmlal_s16(sum, s_hi[0], filters_hi[0]);
  sum = vmlal_s16(sum, s_hi[1], filters_hi[1]);
  sum = vmlal_s16(sum, s_hi[2], filters_hi[2]);
  sum = vmlal_s16(sum, s_hi[3], filters_hi[3]);

  return sum;
}

static inline uint16x4_t highbd_convolve8_2d_scale_horiz4x8_s32_s16(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x4_t *filters_lo,
    const int16x4_t *filters_hi, const int32x4_t shift_s32,
    const int32x4_t offset) {
  int32x4_t sum = highbd_convolve8_2d_scale_horiz4x8_s32(
      s0, s1, s2, s3, filters_lo, filters_hi, offset);

  sum = vqrshlq_s32(sum, shift_s32);
  return vqmovun_s32(sum);
}

#endif  // AOM_AV1_COMMON_ARM_HIGHBD_CONVOLVE_NEON_H_
