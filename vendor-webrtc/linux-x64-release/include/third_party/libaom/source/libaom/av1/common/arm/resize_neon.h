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

#ifndef AOM_AV1_COMMON_ARM_RESIZE_NEON_H_
#define AOM_AV1_COMMON_ARM_RESIZE_NEON_H_

#include <arm_neon.h>

#include "aom_dsp/aom_filter.h"
#include "aom_dsp/arm/mem_neon.h"
#include "aom_dsp/arm/transpose_neon.h"

static inline uint8x8_t scale_filter6_8(const int16x8_t s0, const int16x8_t s1,
                                        const int16x8_t s2, const int16x8_t s3,
                                        const int16x8_t s4, const int16x8_t s5,
                                        int16x8_t filter) {
  const int16x4_t filter_lo = vget_low_s16(filter);
  const int16x4_t filter_hi = vget_high_s16(filter);

  // Filter values at indices 0 and 7 are 0.
  int16x8_t sum = vmulq_lane_s16(s0, filter_lo, 1);
  sum = vmlaq_lane_s16(sum, s1, filter_lo, 2);
  sum = vmlaq_lane_s16(sum, s2, filter_lo, 3);
  sum = vmlaq_lane_s16(sum, s3, filter_hi, 0);
  sum = vmlaq_lane_s16(sum, s4, filter_hi, 1);
  sum = vmlaq_lane_s16(sum, s5, filter_hi, 2);

  // We halved the convolution filter values so -1 from the right shift.
  return vqrshrun_n_s16(sum, FILTER_BITS - 1);
}

static inline void scale_2_to_1_vert_6tap(const uint8_t *src,
                                          const int src_stride, int w, int h,
                                          uint8_t *dst, const int dst_stride,
                                          const int16x8_t filters) {
  do {
    uint8x8_t t0, t1, t2, t3;
    load_u8_8x4(src, src_stride, &t0, &t1, &t2, &t3);

    int16x8_t s0 = vreinterpretq_s16_u16(vmovl_u8(t0));
    int16x8_t s1 = vreinterpretq_s16_u16(vmovl_u8(t1));
    int16x8_t s2 = vreinterpretq_s16_u16(vmovl_u8(t2));
    int16x8_t s3 = vreinterpretq_s16_u16(vmovl_u8(t3));

    const uint8_t *s = src + 4 * src_stride;
    uint8_t *d = dst;
    int height = h;

    do {
      uint8x8_t t4, t5, t6, t7, t8, t9, t10, t11;
      load_u8_8x8(s, src_stride, &t4, &t5, &t6, &t7, &t8, &t9, &t10, &t11);

      int16x8_t s4 = vreinterpretq_s16_u16(vmovl_u8(t4));
      int16x8_t s5 = vreinterpretq_s16_u16(vmovl_u8(t5));
      int16x8_t s6 = vreinterpretq_s16_u16(vmovl_u8(t6));
      int16x8_t s7 = vreinterpretq_s16_u16(vmovl_u8(t7));
      int16x8_t s8 = vreinterpretq_s16_u16(vmovl_u8(t8));
      int16x8_t s9 = vreinterpretq_s16_u16(vmovl_u8(t9));
      int16x8_t s10 = vreinterpretq_s16_u16(vmovl_u8(t10));
      int16x8_t s11 = vreinterpretq_s16_u16(vmovl_u8(t11));

      uint8x8_t d0 = scale_filter6_8(s0, s1, s2, s3, s4, s5, filters);
      uint8x8_t d1 = scale_filter6_8(s2, s3, s4, s5, s6, s7, filters);
      uint8x8_t d2 = scale_filter6_8(s4, s5, s6, s7, s8, s9, filters);
      uint8x8_t d3 = scale_filter6_8(s6, s7, s8, s9, s10, s11, filters);

      store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

      s0 = s8;
      s1 = s9;
      s2 = s10;
      s3 = s11;

      d += 4 * dst_stride;
      s += 8 * src_stride;
      height -= 4;
    } while (height > 0);

    dst += 8;
    src += 8;
    w -= 8;
  } while (w > 0);
}

static inline void scale_4_to_1_vert_6tap(const uint8_t *src,
                                          const int src_stride, int w, int h,
                                          uint8_t *dst, const int dst_stride,
                                          const int16x8_t filters) {
  do {
    uint8x8_t t0 = vld1_u8(src + 0 * src_stride);
    uint8x8_t t1 = vld1_u8(src + 1 * src_stride);

    int16x8_t s0 = vreinterpretq_s16_u16(vmovl_u8(t0));
    int16x8_t s1 = vreinterpretq_s16_u16(vmovl_u8(t1));

    const uint8_t *s = src + 2 * src_stride;
    uint8_t *d = dst;
    int height = h;

    do {
      uint8x8_t t2, t3, t4, t5, t6, t7, t8, t9;
      load_u8_8x8(s, src_stride, &t2, &t3, &t4, &t5, &t6, &t7, &t8, &t9);

      int16x8_t s2 = vreinterpretq_s16_u16(vmovl_u8(t2));
      int16x8_t s3 = vreinterpretq_s16_u16(vmovl_u8(t3));
      int16x8_t s4 = vreinterpretq_s16_u16(vmovl_u8(t4));
      int16x8_t s5 = vreinterpretq_s16_u16(vmovl_u8(t5));
      int16x8_t s6 = vreinterpretq_s16_u16(vmovl_u8(t6));
      int16x8_t s7 = vreinterpretq_s16_u16(vmovl_u8(t7));
      int16x8_t s8 = vreinterpretq_s16_u16(vmovl_u8(t8));
      int16x8_t s9 = vreinterpretq_s16_u16(vmovl_u8(t9));

      uint8x8_t d0 = scale_filter6_8(s0, s1, s2, s3, s4, s5, filters);
      uint8x8_t d1 = scale_filter6_8(s4, s5, s6, s7, s8, s9, filters);

      store_u8_8x2(d, dst_stride, d0, d1);

      s0 = s8;
      s1 = s9;

      s += 8 * src_stride;
      d += 2 * dst_stride;
      height -= 2;
    } while (height > 0);

    src += 8;
    dst += 8;
    w -= 8;
  } while (w > 0);
}

#endif  // AOM_AV1_COMMON_ARM_RESIZE_NEON_H_
