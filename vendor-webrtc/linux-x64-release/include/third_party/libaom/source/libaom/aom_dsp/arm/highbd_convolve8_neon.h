/*
 * Copyright (c) 2024, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_ARM_HIGHBD_CONVOLVE8_NEON_H_
#define AOM_AOM_DSP_ARM_HIGHBD_CONVOLVE8_NEON_H_

#include <arm_neon.h>

#include "config/aom_config.h"
#include "aom_dsp/arm/mem_neon.h"

static inline void highbd_convolve8_horiz_2tap_neon(
    const uint16_t *src_ptr, ptrdiff_t src_stride, uint16_t *dst_ptr,
    ptrdiff_t dst_stride, const int16_t *x_filter_ptr, int w, int h, int bd) {
  // Bilinear filter values are all positive and multiples of 8. Divide by 8 to
  // reduce intermediate precision requirements and allow the use of non
  // widening multiply.
  const uint16x8_t f0 = vdupq_n_u16((uint16_t)x_filter_ptr[3] / 8);
  const uint16x8_t f1 = vdupq_n_u16((uint16_t)x_filter_ptr[4] / 8);

  const uint16x8_t max = vdupq_n_u16((1 << bd) - 1);

  if (w == 4) {
    do {
      uint16x8_t s0 =
          load_unaligned_u16_4x2(src_ptr + 0 * src_stride + 0, (int)src_stride);
      uint16x8_t s1 =
          load_unaligned_u16_4x2(src_ptr + 0 * src_stride + 1, (int)src_stride);
      uint16x8_t s2 =
          load_unaligned_u16_4x2(src_ptr + 2 * src_stride + 0, (int)src_stride);
      uint16x8_t s3 =
          load_unaligned_u16_4x2(src_ptr + 2 * src_stride + 1, (int)src_stride);

      uint16x8_t sum01 = vmulq_u16(s0, f0);
      sum01 = vmlaq_u16(sum01, s1, f1);
      uint16x8_t sum23 = vmulq_u16(s2, f0);
      sum23 = vmlaq_u16(sum23, s3, f1);

      // We divided filter taps by 8 so subtract 3 from right shift.
      sum01 = vrshrq_n_u16(sum01, FILTER_BITS - 3);
      sum23 = vrshrq_n_u16(sum23, FILTER_BITS - 3);

      sum01 = vminq_u16(sum01, max);
      sum23 = vminq_u16(sum23, max);

      store_u16x4_strided_x2(dst_ptr + 0 * dst_stride, (int)dst_stride, sum01);
      store_u16x4_strided_x2(dst_ptr + 2 * dst_stride, (int)dst_stride, sum23);

      src_ptr += 4 * src_stride;
      dst_ptr += 4 * dst_stride;
      h -= 4;
    } while (h > 0);
  } else {
    do {
      int width = w;
      const uint16_t *s = src_ptr;
      uint16_t *d = dst_ptr;

      do {
        uint16x8_t s0 = vld1q_u16(s + 0 * src_stride + 0);
        uint16x8_t s1 = vld1q_u16(s + 0 * src_stride + 1);
        uint16x8_t s2 = vld1q_u16(s + 1 * src_stride + 0);
        uint16x8_t s3 = vld1q_u16(s + 1 * src_stride + 1);

        uint16x8_t sum01 = vmulq_u16(s0, f0);
        sum01 = vmlaq_u16(sum01, s1, f1);
        uint16x8_t sum23 = vmulq_u16(s2, f0);
        sum23 = vmlaq_u16(sum23, s3, f1);

        // We divided filter taps by 8 so subtract 3 from right shift.
        sum01 = vrshrq_n_u16(sum01, FILTER_BITS - 3);
        sum23 = vrshrq_n_u16(sum23, FILTER_BITS - 3);

        sum01 = vminq_u16(sum01, max);
        sum23 = vminq_u16(sum23, max);

        vst1q_u16(d + 0 * dst_stride, sum01);
        vst1q_u16(d + 1 * dst_stride, sum23);

        s += 8;
        d += 8;
        width -= 8;
      } while (width != 0);
      src_ptr += 2 * src_stride;
      dst_ptr += 2 * dst_stride;
      h -= 2;
    } while (h > 0);
  }
}

static inline uint16x4_t highbd_convolve4_4(
    const int16x4_t s0, const int16x4_t s1, const int16x4_t s2,
    const int16x4_t s3, const int16x4_t filter, const uint16x4_t max) {
  int32x4_t sum = vmull_lane_s16(s0, filter, 0);
  sum = vmlal_lane_s16(sum, s1, filter, 1);
  sum = vmlal_lane_s16(sum, s2, filter, 2);
  sum = vmlal_lane_s16(sum, s3, filter, 3);

  uint16x4_t res = vqrshrun_n_s32(sum, FILTER_BITS);

  return vmin_u16(res, max);
}

static inline uint16x8_t highbd_convolve4_8(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x4_t filter, const uint16x8_t max) {
  int32x4_t sum0 = vmull_lane_s16(vget_low_s16(s0), filter, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s1), filter, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s2), filter, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s3), filter, 3);

  int32x4_t sum1 = vmull_lane_s16(vget_high_s16(s0), filter, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s1), filter, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s2), filter, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s3), filter, 3);

  uint16x8_t res = vcombine_u16(vqrshrun_n_s32(sum0, FILTER_BITS),
                                vqrshrun_n_s32(sum1, FILTER_BITS));

  return vminq_u16(res, max);
}

static inline void highbd_convolve8_vert_4tap_neon(
    const uint16_t *src_ptr, ptrdiff_t src_stride, uint16_t *dst_ptr,
    ptrdiff_t dst_stride, const int16_t *y_filter_ptr, int w, int h, int bd) {
  assert(w >= 4 && h >= 4);
  const int16x4_t y_filter = vld1_s16(y_filter_ptr + 2);

  if (w == 4) {
    const uint16x4_t max = vdup_n_u16((1 << bd) - 1);
    const int16_t *s = (const int16_t *)src_ptr;
    uint16_t *d = dst_ptr;

    int16x4_t s0, s1, s2;
    load_s16_4x3(s, src_stride, &s0, &s1, &s2);
    s += 3 * src_stride;

    do {
      int16x4_t s3, s4, s5, s6;
      load_s16_4x4(s, src_stride, &s3, &s4, &s5, &s6);

      uint16x4_t d0 = highbd_convolve4_4(s0, s1, s2, s3, y_filter, max);
      uint16x4_t d1 = highbd_convolve4_4(s1, s2, s3, s4, y_filter, max);
      uint16x4_t d2 = highbd_convolve4_4(s2, s3, s4, s5, y_filter, max);
      uint16x4_t d3 = highbd_convolve4_4(s3, s4, s5, s6, y_filter, max);

      store_u16_4x4(d, dst_stride, d0, d1, d2, d3);

      s0 = s4;
      s1 = s5;
      s2 = s6;

      s += 4 * src_stride;
      d += 4 * dst_stride;
      h -= 4;
    } while (h > 0);
  } else {
    const uint16x8_t max = vdupq_n_u16((1 << bd) - 1);

    do {
      int height = h;
      const int16_t *s = (const int16_t *)src_ptr;
      uint16_t *d = dst_ptr;

      int16x8_t s0, s1, s2;
      load_s16_8x3(s, src_stride, &s0, &s1, &s2);
      s += 3 * src_stride;

      do {
        int16x8_t s3, s4, s5, s6;
        load_s16_8x4(s, src_stride, &s3, &s4, &s5, &s6);

        uint16x8_t d0 = highbd_convolve4_8(s0, s1, s2, s3, y_filter, max);
        uint16x8_t d1 = highbd_convolve4_8(s1, s2, s3, s4, y_filter, max);
        uint16x8_t d2 = highbd_convolve4_8(s2, s3, s4, s5, y_filter, max);
        uint16x8_t d3 = highbd_convolve4_8(s3, s4, s5, s6, y_filter, max);

        store_u16_8x4(d, dst_stride, d0, d1, d2, d3);

        s0 = s4;
        s1 = s5;
        s2 = s6;

        s += 4 * src_stride;
        d += 4 * dst_stride;
        height -= 4;
      } while (height > 0);
      src_ptr += 8;
      dst_ptr += 8;
      w -= 8;
    } while (w > 0);
  }
}

static inline void highbd_convolve8_vert_2tap_neon(
    const uint16_t *src_ptr, ptrdiff_t src_stride, uint16_t *dst_ptr,
    ptrdiff_t dst_stride, const int16_t *x_filter_ptr, int w, int h, int bd) {
  // Bilinear filter values are all positive and multiples of 8. Divide by 8 to
  // reduce intermediate precision requirements and allow the use of non
  // widening multiply.
  const uint16x8_t f0 = vdupq_n_u16((uint16_t)x_filter_ptr[3] / 8);
  const uint16x8_t f1 = vdupq_n_u16((uint16_t)x_filter_ptr[4] / 8);

  const uint16x8_t max = vdupq_n_u16((1 << bd) - 1);

  if (w == 4) {
    do {
      uint16x8_t s0 =
          load_unaligned_u16_4x2(src_ptr + 0 * src_stride, (int)src_stride);
      uint16x8_t s1 =
          load_unaligned_u16_4x2(src_ptr + 1 * src_stride, (int)src_stride);
      uint16x8_t s2 =
          load_unaligned_u16_4x2(src_ptr + 2 * src_stride, (int)src_stride);
      uint16x8_t s3 =
          load_unaligned_u16_4x2(src_ptr + 3 * src_stride, (int)src_stride);

      uint16x8_t sum01 = vmulq_u16(s0, f0);
      sum01 = vmlaq_u16(sum01, s1, f1);
      uint16x8_t sum23 = vmulq_u16(s2, f0);
      sum23 = vmlaq_u16(sum23, s3, f1);

      // We divided filter taps by 8 so subtract 3 from right shift.
      sum01 = vrshrq_n_u16(sum01, FILTER_BITS - 3);
      sum23 = vrshrq_n_u16(sum23, FILTER_BITS - 3);

      sum01 = vminq_u16(sum01, max);
      sum23 = vminq_u16(sum23, max);

      store_u16x4_strided_x2(dst_ptr + 0 * dst_stride, (int)dst_stride, sum01);
      store_u16x4_strided_x2(dst_ptr + 2 * dst_stride, (int)dst_stride, sum23);

      src_ptr += 4 * src_stride;
      dst_ptr += 4 * dst_stride;
      h -= 4;
    } while (h > 0);
  } else {
    do {
      int width = w;
      const uint16_t *s = src_ptr;
      uint16_t *d = dst_ptr;

      do {
        uint16x8_t s0, s1, s2;
        load_u16_8x3(s, src_stride, &s0, &s1, &s2);

        uint16x8_t sum01 = vmulq_u16(s0, f0);
        sum01 = vmlaq_u16(sum01, s1, f1);
        uint16x8_t sum23 = vmulq_u16(s1, f0);
        sum23 = vmlaq_u16(sum23, s2, f1);

        // We divided filter taps by 8 so subtract 3 from right shift.
        sum01 = vrshrq_n_u16(sum01, FILTER_BITS - 3);
        sum23 = vrshrq_n_u16(sum23, FILTER_BITS - 3);

        sum01 = vminq_u16(sum01, max);
        sum23 = vminq_u16(sum23, max);

        vst1q_u16(d + 0 * dst_stride, sum01);
        vst1q_u16(d + 1 * dst_stride, sum23);

        s += 8;
        d += 8;
        width -= 8;
      } while (width != 0);
      src_ptr += 2 * src_stride;
      dst_ptr += 2 * dst_stride;
      h -= 2;
    } while (h > 0);
  }
}

#endif  // AOM_AOM_DSP_ARM_HIGHBD_CONVOLVE8_NEON_H_
