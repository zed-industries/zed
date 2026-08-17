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

#ifndef AOM_AOM_DSP_ARM_AOM_CONVOLVE8_NEON_H_
#define AOM_AOM_DSP_ARM_AOM_CONVOLVE8_NEON_H_

#include <arm_neon.h>

#include "aom_dsp/aom_filter.h"
#include "aom_dsp/arm/mem_neon.h"
#include "config/aom_config.h"

static inline int16x4_t convolve8_4(const int16x4_t s0, const int16x4_t s1,
                                    const int16x4_t s2, const int16x4_t s3,
                                    const int16x4_t s4, const int16x4_t s5,
                                    const int16x4_t s6, const int16x4_t s7,
                                    const int16x8_t filter) {
  const int16x4_t filter_lo = vget_low_s16(filter);
  const int16x4_t filter_hi = vget_high_s16(filter);

  int16x4_t sum = vmul_lane_s16(s0, filter_lo, 0);
  sum = vmla_lane_s16(sum, s1, filter_lo, 1);
  sum = vmla_lane_s16(sum, s2, filter_lo, 2);
  sum = vmla_lane_s16(sum, s3, filter_lo, 3);
  sum = vmla_lane_s16(sum, s4, filter_hi, 0);
  sum = vmla_lane_s16(sum, s5, filter_hi, 1);
  sum = vmla_lane_s16(sum, s6, filter_hi, 2);
  sum = vmla_lane_s16(sum, s7, filter_hi, 3);

  return sum;
}

static inline uint8x8_t convolve8_8(const int16x8_t s0, const int16x8_t s1,
                                    const int16x8_t s2, const int16x8_t s3,
                                    const int16x8_t s4, const int16x8_t s5,
                                    const int16x8_t s6, const int16x8_t s7,
                                    const int16x8_t filter) {
  const int16x4_t filter_lo = vget_low_s16(filter);
  const int16x4_t filter_hi = vget_high_s16(filter);

  int16x8_t sum = vmulq_lane_s16(s0, filter_lo, 0);
  sum = vmlaq_lane_s16(sum, s1, filter_lo, 1);
  sum = vmlaq_lane_s16(sum, s2, filter_lo, 2);
  sum = vmlaq_lane_s16(sum, s3, filter_lo, 3);
  sum = vmlaq_lane_s16(sum, s4, filter_hi, 0);
  sum = vmlaq_lane_s16(sum, s5, filter_hi, 1);
  sum = vmlaq_lane_s16(sum, s6, filter_hi, 2);
  sum = vmlaq_lane_s16(sum, s7, filter_hi, 3);

  // We halved the filter values so -1 from right shift.
  return vqrshrun_n_s16(sum, FILTER_BITS - 1);
}

static inline void convolve8_horiz_2tap_neon(const uint8_t *src,
                                             ptrdiff_t src_stride, uint8_t *dst,
                                             ptrdiff_t dst_stride,
                                             const int16_t *filter_x, int w,
                                             int h) {
  // Bilinear filter values are all positive.
  const uint8x8_t f0 = vdup_n_u8((uint8_t)filter_x[3]);
  const uint8x8_t f1 = vdup_n_u8((uint8_t)filter_x[4]);

  if (w == 4) {
    do {
      uint8x8_t s0 =
          load_unaligned_u8(src + 0 * src_stride + 0, (int)src_stride);
      uint8x8_t s1 =
          load_unaligned_u8(src + 0 * src_stride + 1, (int)src_stride);
      uint8x8_t s2 =
          load_unaligned_u8(src + 2 * src_stride + 0, (int)src_stride);
      uint8x8_t s3 =
          load_unaligned_u8(src + 2 * src_stride + 1, (int)src_stride);

      uint16x8_t sum0 = vmull_u8(s0, f0);
      sum0 = vmlal_u8(sum0, s1, f1);
      uint16x8_t sum1 = vmull_u8(s2, f0);
      sum1 = vmlal_u8(sum1, s3, f1);

      uint8x8_t d0 = vqrshrn_n_u16(sum0, FILTER_BITS);
      uint8x8_t d1 = vqrshrn_n_u16(sum1, FILTER_BITS);

      store_u8x4_strided_x2(dst + 0 * dst_stride, dst_stride, d0);
      store_u8x4_strided_x2(dst + 2 * dst_stride, dst_stride, d1);

      src += 4 * src_stride;
      dst += 4 * dst_stride;
      h -= 4;
    } while (h > 0);
  } else if (w == 8) {
    do {
      uint8x8_t s0 = vld1_u8(src + 0 * src_stride + 0);
      uint8x8_t s1 = vld1_u8(src + 0 * src_stride + 1);
      uint8x8_t s2 = vld1_u8(src + 1 * src_stride + 0);
      uint8x8_t s3 = vld1_u8(src + 1 * src_stride + 1);

      uint16x8_t sum0 = vmull_u8(s0, f0);
      sum0 = vmlal_u8(sum0, s1, f1);
      uint16x8_t sum1 = vmull_u8(s2, f0);
      sum1 = vmlal_u8(sum1, s3, f1);

      uint8x8_t d0 = vqrshrn_n_u16(sum0, FILTER_BITS);
      uint8x8_t d1 = vqrshrn_n_u16(sum1, FILTER_BITS);

      vst1_u8(dst + 0 * dst_stride, d0);
      vst1_u8(dst + 1 * dst_stride, d1);

      src += 2 * src_stride;
      dst += 2 * dst_stride;
      h -= 2;
    } while (h > 0);
  } else {
    do {
      int width = w;
      const uint8_t *s = src;
      uint8_t *d = dst;

      do {
        uint8x16_t s0 = vld1q_u8(s + 0);
        uint8x16_t s1 = vld1q_u8(s + 1);

        uint16x8_t sum0 = vmull_u8(vget_low_u8(s0), f0);
        sum0 = vmlal_u8(sum0, vget_low_u8(s1), f1);
        uint16x8_t sum1 = vmull_u8(vget_high_u8(s0), f0);
        sum1 = vmlal_u8(sum1, vget_high_u8(s1), f1);

        uint8x8_t d0 = vqrshrn_n_u16(sum0, FILTER_BITS);
        uint8x8_t d1 = vqrshrn_n_u16(sum1, FILTER_BITS);

        vst1q_u8(d, vcombine_u8(d0, d1));

        s += 16;
        d += 16;
        width -= 16;
      } while (width != 0);
      src += src_stride;
      dst += dst_stride;
    } while (--h > 0);
  }
}

static inline uint8x8_t convolve4_8(const int16x8_t s0, const int16x8_t s1,
                                    const int16x8_t s2, const int16x8_t s3,
                                    const int16x4_t filter) {
  int16x8_t sum = vmulq_lane_s16(s0, filter, 0);
  sum = vmlaq_lane_s16(sum, s1, filter, 1);
  sum = vmlaq_lane_s16(sum, s2, filter, 2);
  sum = vmlaq_lane_s16(sum, s3, filter, 3);

  // We halved the filter values so -1 from right shift.
  return vqrshrun_n_s16(sum, FILTER_BITS - 1);
}

static inline void convolve8_vert_4tap_neon(const uint8_t *src,
                                            ptrdiff_t src_stride, uint8_t *dst,
                                            ptrdiff_t dst_stride,
                                            const int16_t *filter_y, int w,
                                            int h) {
  // All filter values are even, halve to reduce intermediate precision
  // requirements.
  const int16x4_t filter = vshr_n_s16(vld1_s16(filter_y + 2), 1);

  if (w == 4) {
    uint8x8_t t01 = load_unaligned_u8(src + 0 * src_stride, (int)src_stride);
    uint8x8_t t12 = load_unaligned_u8(src + 1 * src_stride, (int)src_stride);

    int16x8_t s01 = vreinterpretq_s16_u16(vmovl_u8(t01));
    int16x8_t s12 = vreinterpretq_s16_u16(vmovl_u8(t12));

    src += 2 * src_stride;

    do {
      uint8x8_t t23 = load_unaligned_u8(src + 0 * src_stride, (int)src_stride);
      uint8x8_t t34 = load_unaligned_u8(src + 1 * src_stride, (int)src_stride);
      uint8x8_t t45 = load_unaligned_u8(src + 2 * src_stride, (int)src_stride);
      uint8x8_t t56 = load_unaligned_u8(src + 3 * src_stride, (int)src_stride);

      int16x8_t s23 = vreinterpretq_s16_u16(vmovl_u8(t23));
      int16x8_t s34 = vreinterpretq_s16_u16(vmovl_u8(t34));
      int16x8_t s45 = vreinterpretq_s16_u16(vmovl_u8(t45));
      int16x8_t s56 = vreinterpretq_s16_u16(vmovl_u8(t56));

      uint8x8_t d01 = convolve4_8(s01, s12, s23, s34, filter);
      uint8x8_t d23 = convolve4_8(s23, s34, s45, s56, filter);

      store_u8x4_strided_x2(dst + 0 * dst_stride, dst_stride, d01);
      store_u8x4_strided_x2(dst + 2 * dst_stride, dst_stride, d23);

      s01 = s45;
      s12 = s56;

      src += 4 * src_stride;
      dst += 4 * dst_stride;
      h -= 4;
    } while (h != 0);
  } else {
    do {
      uint8x8_t t0, t1, t2;
      load_u8_8x3(src, src_stride, &t0, &t1, &t2);

      int16x8_t s0 = vreinterpretq_s16_u16(vmovl_u8(t0));
      int16x8_t s1 = vreinterpretq_s16_u16(vmovl_u8(t1));
      int16x8_t s2 = vreinterpretq_s16_u16(vmovl_u8(t2));

      int height = h;
      const uint8_t *s = src + 3 * src_stride;
      uint8_t *d = dst;

      do {
        uint8x8_t t3;
        load_u8_8x4(s, src_stride, &t0, &t1, &t2, &t3);

        int16x8_t s3 = vreinterpretq_s16_u16(vmovl_u8(t0));
        int16x8_t s4 = vreinterpretq_s16_u16(vmovl_u8(t1));
        int16x8_t s5 = vreinterpretq_s16_u16(vmovl_u8(t2));
        int16x8_t s6 = vreinterpretq_s16_u16(vmovl_u8(t3));

        uint8x8_t d0 = convolve4_8(s0, s1, s2, s3, filter);
        uint8x8_t d1 = convolve4_8(s1, s2, s3, s4, filter);
        uint8x8_t d2 = convolve4_8(s2, s3, s4, s5, filter);
        uint8x8_t d3 = convolve4_8(s3, s4, s5, s6, filter);

        store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

        s0 = s4;
        s1 = s5;
        s2 = s6;

        s += 4 * src_stride;
        d += 4 * dst_stride;
        height -= 4;
      } while (height != 0);
      src += 8;
      dst += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline void convolve8_vert_2tap_neon(const uint8_t *src,
                                            ptrdiff_t src_stride, uint8_t *dst,
                                            ptrdiff_t dst_stride,
                                            const int16_t *filter_y, int w,
                                            int h) {
  // Bilinear filter values are all positive.
  uint8x8_t f0 = vdup_n_u8((uint8_t)filter_y[3]);
  uint8x8_t f1 = vdup_n_u8((uint8_t)filter_y[4]);

  if (w == 4) {
    do {
      uint8x8_t s0 = load_unaligned_u8(src + 0 * src_stride, (int)src_stride);
      uint8x8_t s1 = load_unaligned_u8(src + 1 * src_stride, (int)src_stride);
      uint8x8_t s2 = load_unaligned_u8(src + 2 * src_stride, (int)src_stride);
      uint8x8_t s3 = load_unaligned_u8(src + 3 * src_stride, (int)src_stride);

      uint16x8_t sum0 = vmull_u8(s0, f0);
      sum0 = vmlal_u8(sum0, s1, f1);
      uint16x8_t sum1 = vmull_u8(s2, f0);
      sum1 = vmlal_u8(sum1, s3, f1);

      uint8x8_t d0 = vqrshrn_n_u16(sum0, FILTER_BITS);
      uint8x8_t d1 = vqrshrn_n_u16(sum1, FILTER_BITS);

      store_u8x4_strided_x2(dst + 0 * dst_stride, dst_stride, d0);
      store_u8x4_strided_x2(dst + 2 * dst_stride, dst_stride, d1);

      src += 4 * src_stride;
      dst += 4 * dst_stride;
      h -= 4;
    } while (h > 0);
  } else if (w == 8) {
    do {
      uint8x8_t s0, s1, s2;
      load_u8_8x3(src, src_stride, &s0, &s1, &s2);

      uint16x8_t sum0 = vmull_u8(s0, f0);
      sum0 = vmlal_u8(sum0, s1, f1);
      uint16x8_t sum1 = vmull_u8(s1, f0);
      sum1 = vmlal_u8(sum1, s2, f1);

      uint8x8_t d0 = vqrshrn_n_u16(sum0, FILTER_BITS);
      uint8x8_t d1 = vqrshrn_n_u16(sum1, FILTER_BITS);

      vst1_u8(dst + 0 * dst_stride, d0);
      vst1_u8(dst + 1 * dst_stride, d1);

      src += 2 * src_stride;
      dst += 2 * dst_stride;
      h -= 2;
    } while (h > 0);
  } else {
    do {
      int width = w;
      const uint8_t *s = src;
      uint8_t *d = dst;

      do {
        uint8x16_t s0 = vld1q_u8(s + 0 * src_stride);
        uint8x16_t s1 = vld1q_u8(s + 1 * src_stride);

        uint16x8_t sum0 = vmull_u8(vget_low_u8(s0), f0);
        sum0 = vmlal_u8(sum0, vget_low_u8(s1), f1);
        uint16x8_t sum1 = vmull_u8(vget_high_u8(s0), f0);
        sum1 = vmlal_u8(sum1, vget_high_u8(s1), f1);

        uint8x8_t d0 = vqrshrn_n_u16(sum0, FILTER_BITS);
        uint8x8_t d1 = vqrshrn_n_u16(sum1, FILTER_BITS);

        vst1q_u8(d, vcombine_u8(d0, d1));

        s += 16;
        d += 16;
        width -= 16;
      } while (width != 0);
      src += src_stride;
      dst += dst_stride;
    } while (--h > 0);
  }
}

#endif  // AOM_AOM_DSP_ARM_AOM_CONVOLVE8_NEON_H_
