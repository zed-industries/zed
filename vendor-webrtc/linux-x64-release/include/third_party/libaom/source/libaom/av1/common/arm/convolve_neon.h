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

#ifndef AOM_AV1_COMMON_ARM_CONVOLVE_NEON_H_
#define AOM_AV1_COMMON_ARM_CONVOLVE_NEON_H_

#include <arm_neon.h>

#include "config/aom_config.h"

#include "aom_dsp/arm/mem_neon.h"
#include "av1/common/convolve.h"
#include "av1/common/filter.h"

static inline int32x4_t convolve12_4_2d_v(
    const int16x4_t s0, const int16x4_t s1, const int16x4_t s2,
    const int16x4_t s3, const int16x4_t s4, const int16x4_t s5,
    const int16x4_t s6, const int16x4_t s7, const int16x4_t s8,
    const int16x4_t s9, const int16x4_t s10, const int16x4_t s11,
    const int16x8_t y_filter_0_7, const int16x4_t y_filter_8_11) {
  const int16x4_t y_filter_0_3 = vget_low_s16(y_filter_0_7);
  const int16x4_t y_filter_4_7 = vget_high_s16(y_filter_0_7);

  int32x4_t sum = vmull_lane_s16(s0, y_filter_0_3, 0);
  sum = vmlal_lane_s16(sum, s1, y_filter_0_3, 1);
  sum = vmlal_lane_s16(sum, s2, y_filter_0_3, 2);
  sum = vmlal_lane_s16(sum, s3, y_filter_0_3, 3);
  sum = vmlal_lane_s16(sum, s4, y_filter_4_7, 0);
  sum = vmlal_lane_s16(sum, s5, y_filter_4_7, 1);
  sum = vmlal_lane_s16(sum, s6, y_filter_4_7, 2);
  sum = vmlal_lane_s16(sum, s7, y_filter_4_7, 3);
  sum = vmlal_lane_s16(sum, s8, y_filter_8_11, 0);
  sum = vmlal_lane_s16(sum, s9, y_filter_8_11, 1);
  sum = vmlal_lane_s16(sum, s10, y_filter_8_11, 2);
  sum = vmlal_lane_s16(sum, s11, y_filter_8_11, 3);

  return sum;
}

static inline uint8x8_t convolve12_8_2d_v(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x8_t s4, const int16x8_t s5,
    const int16x8_t s6, const int16x8_t s7, const int16x8_t s8,
    const int16x8_t s9, const int16x8_t s10, const int16x8_t s11,
    const int16x8_t y_filter_0_7, const int16x4_t y_filter_8_11,
    const int16x8_t sub_const) {
  const int16x4_t y_filter_0_3 = vget_low_s16(y_filter_0_7);
  const int16x4_t y_filter_4_7 = vget_high_s16(y_filter_0_7);

  int32x4_t sum0 = vmull_lane_s16(vget_low_s16(s0), y_filter_0_3, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s1), y_filter_0_3, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s2), y_filter_0_3, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s3), y_filter_0_3, 3);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s4), y_filter_4_7, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s5), y_filter_4_7, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s6), y_filter_4_7, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s7), y_filter_4_7, 3);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s8), y_filter_8_11, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s9), y_filter_8_11, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s10), y_filter_8_11, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s11), y_filter_8_11, 3);

  int32x4_t sum1 = vmull_lane_s16(vget_high_s16(s0), y_filter_0_3, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s1), y_filter_0_3, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s2), y_filter_0_3, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s3), y_filter_0_3, 3);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s4), y_filter_4_7, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s5), y_filter_4_7, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s6), y_filter_4_7, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s7), y_filter_4_7, 3);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s8), y_filter_8_11, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s9), y_filter_8_11, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s10), y_filter_8_11, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s11), y_filter_8_11, 3);

  int16x8_t res =
      vcombine_s16(vqrshrn_n_s32(sum0, 2 * FILTER_BITS - ROUND0_BITS),
                   vqrshrn_n_s32(sum1, 2 * FILTER_BITS - ROUND0_BITS));
  res = vsubq_s16(res, sub_const);

  return vqmovun_s16(res);
}

static inline void convolve_2d_sr_vert_12tap_neon(
    int16_t *src_ptr, int src_stride, uint8_t *dst_ptr, int dst_stride, int w,
    int h, const int16x8_t y_filter_0_7, const int16x4_t y_filter_8_11) {
  const int bd = 8;
  const int16x8_t sub_const = vdupq_n_s16(1 << (bd - 1));

  if (w <= 4) {
    int16x4_t s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10;
    load_s16_4x11(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6, &s7,
                  &s8, &s9, &s10);
    src_ptr += 11 * src_stride;

    do {
      int16x4_t s11, s12, s13, s14;
      load_s16_4x4(src_ptr, src_stride, &s11, &s12, &s13, &s14);

      int32x4_t d0 = convolve12_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, s8, s9,
                                       s10, s11, y_filter_0_7, y_filter_8_11);
      int32x4_t d1 = convolve12_4_2d_v(s1, s2, s3, s4, s5, s6, s7, s8, s9, s10,
                                       s11, s12, y_filter_0_7, y_filter_8_11);
      int32x4_t d2 = convolve12_4_2d_v(s2, s3, s4, s5, s6, s7, s8, s9, s10, s11,
                                       s12, s13, y_filter_0_7, y_filter_8_11);
      int32x4_t d3 =
          convolve12_4_2d_v(s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14,
                            y_filter_0_7, y_filter_8_11);

      int16x8_t dd01 =
          vcombine_s16(vqrshrn_n_s32(d0, 2 * FILTER_BITS - ROUND0_BITS),
                       vqrshrn_n_s32(d1, 2 * FILTER_BITS - ROUND0_BITS));
      int16x8_t dd23 =
          vcombine_s16(vqrshrn_n_s32(d2, 2 * FILTER_BITS - ROUND0_BITS),
                       vqrshrn_n_s32(d3, 2 * FILTER_BITS - ROUND0_BITS));

      dd01 = vsubq_s16(dd01, sub_const);
      dd23 = vsubq_s16(dd23, sub_const);

      uint8x8_t d01 = vqmovun_s16(dd01);
      uint8x8_t d23 = vqmovun_s16(dd23);

      store_u8x4_strided_x2(dst_ptr + 0 * dst_stride, dst_stride, d01);
      store_u8x4_strided_x2(dst_ptr + 2 * dst_stride, dst_stride, d23);

      s0 = s4;
      s1 = s5;
      s2 = s6;
      s3 = s7;
      s4 = s8;
      s5 = s9;
      s6 = s10;
      s7 = s11;
      s8 = s12;
      s9 = s13;
      s10 = s14;
      src_ptr += 4 * src_stride;
      dst_ptr += 4 * dst_stride;
      h -= 4;
    } while (h != 0);

  } else {
    do {
      int height = h;
      int16_t *s = src_ptr;
      uint8_t *d = dst_ptr;

      int16x8_t s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10;
      load_s16_8x11(s, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6, &s7, &s8,
                    &s9, &s10);
      s += 11 * src_stride;

      do {
        int16x8_t s11, s12, s13, s14;
        load_s16_8x4(s, src_stride, &s11, &s12, &s13, &s14);

        uint8x8_t d0 =
            convolve12_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11,
                              y_filter_0_7, y_filter_8_11, sub_const);
        uint8x8_t d1 =
            convolve12_8_2d_v(s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12,
                              y_filter_0_7, y_filter_8_11, sub_const);
        uint8x8_t d2 =
            convolve12_8_2d_v(s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12,
                              s13, y_filter_0_7, y_filter_8_11, sub_const);
        uint8x8_t d3 =
            convolve12_8_2d_v(s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13,
                              s14, y_filter_0_7, y_filter_8_11, sub_const);

        store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

        s0 = s4;
        s1 = s5;
        s2 = s6;
        s3 = s7;
        s4 = s8;
        s5 = s9;
        s6 = s10;
        s7 = s11;
        s8 = s12;
        s9 = s13;
        s10 = s14;
        s += 4 * src_stride;
        d += 4 * dst_stride;
        height -= 4;
      } while (height != 0);
      src_ptr += 8;
      dst_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline int16x4_t convolve8_4_2d_v(const int16x4_t s0, const int16x4_t s1,
                                         const int16x4_t s2, const int16x4_t s3,
                                         const int16x4_t s4, const int16x4_t s5,
                                         const int16x4_t s6, const int16x4_t s7,
                                         const int16x8_t y_filter) {
  const int16x4_t y_filter_lo = vget_low_s16(y_filter);
  const int16x4_t y_filter_hi = vget_high_s16(y_filter);

  int32x4_t sum = vmull_lane_s16(s0, y_filter_lo, 0);
  sum = vmlal_lane_s16(sum, s1, y_filter_lo, 1);
  sum = vmlal_lane_s16(sum, s2, y_filter_lo, 2);
  sum = vmlal_lane_s16(sum, s3, y_filter_lo, 3);
  sum = vmlal_lane_s16(sum, s4, y_filter_hi, 0);
  sum = vmlal_lane_s16(sum, s5, y_filter_hi, 1);
  sum = vmlal_lane_s16(sum, s6, y_filter_hi, 2);
  sum = vmlal_lane_s16(sum, s7, y_filter_hi, 3);

  return vqrshrn_n_s32(sum, 2 * FILTER_BITS - ROUND0_BITS);
}

static inline uint8x8_t convolve8_8_2d_v(const int16x8_t s0, const int16x8_t s1,
                                         const int16x8_t s2, const int16x8_t s3,
                                         const int16x8_t s4, const int16x8_t s5,
                                         const int16x8_t s6, const int16x8_t s7,
                                         const int16x8_t y_filter,
                                         const int16x8_t sub_const) {
  const int16x4_t y_filter_lo = vget_low_s16(y_filter);
  const int16x4_t y_filter_hi = vget_high_s16(y_filter);

  int32x4_t sum0 = vmull_lane_s16(vget_low_s16(s0), y_filter_lo, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s1), y_filter_lo, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s2), y_filter_lo, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s3), y_filter_lo, 3);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s4), y_filter_hi, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s5), y_filter_hi, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s6), y_filter_hi, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s7), y_filter_hi, 3);

  int32x4_t sum1 = vmull_lane_s16(vget_high_s16(s0), y_filter_lo, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s1), y_filter_lo, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s2), y_filter_lo, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s3), y_filter_lo, 3);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s4), y_filter_hi, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s5), y_filter_hi, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s6), y_filter_hi, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s7), y_filter_hi, 3);

  int16x8_t res =
      vcombine_s16(vqrshrn_n_s32(sum0, 2 * FILTER_BITS - ROUND0_BITS),
                   vqrshrn_n_s32(sum1, 2 * FILTER_BITS - ROUND0_BITS));
  res = vsubq_s16(res, sub_const);

  return vqmovun_s16(res);
}

static inline void convolve_2d_sr_vert_8tap_neon(int16_t *src_ptr,
                                                 int src_stride,
                                                 uint8_t *dst_ptr,
                                                 int dst_stride, int w, int h,
                                                 const int16x8_t y_filter) {
  const int bd = 8;
  const int16x8_t sub_const = vdupq_n_s16(1 << (bd - 1));

  if (w <= 4) {
    int16x4_t s0, s1, s2, s3, s4, s5, s6;
    load_s16_4x7(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
    src_ptr += 7 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s7, s8, s9, s10;
      load_s16_4x4(src_ptr, src_stride, &s7, &s8, &s9, &s10);

      int16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter);
      int16x4_t d1 = convolve8_4_2d_v(s1, s2, s3, s4, s5, s6, s7, s8, y_filter);
      int16x4_t d2 = convolve8_4_2d_v(s2, s3, s4, s5, s6, s7, s8, s9, y_filter);
      int16x4_t d3 =
          convolve8_4_2d_v(s3, s4, s5, s6, s7, s8, s9, s10, y_filter);

      uint8x8_t d01 = vqmovun_s16(vsubq_s16(vcombine_s16(d0, d1), sub_const));
      uint8x8_t d23 = vqmovun_s16(vsubq_s16(vcombine_s16(d2, d3), sub_const));

      store_u8x4_strided_x2(dst_ptr + 0 * dst_stride, dst_stride, d01);
      store_u8x4_strided_x2(dst_ptr + 2 * dst_stride, dst_stride, d23);

      s0 = s4;
      s1 = s5;
      s2 = s6;
      s3 = s7;
      s4 = s8;
      s5 = s9;
      s6 = s10;
      src_ptr += 4 * src_stride;
      dst_ptr += 4 * dst_stride;
      h -= 4;
#else   // !AOM_ARCH_AARCH64
      int16x4_t s7 = vld1_s16(src_ptr);
      int16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter);
      uint8x8_t d01 =
          vqmovun_s16(vsubq_s16(vcombine_s16(d0, vdup_n_s16(0)), sub_const));

      store_u8_4x1(dst_ptr, d01);

      s0 = s1;
      s1 = s2;
      s2 = s3;
      s3 = s4;
      s4 = s5;
      s5 = s6;
      s6 = s7;
      src_ptr += src_stride;
      dst_ptr += dst_stride;
      h--;
#endif  // AOM_ARCH_AARCH64
    } while (h != 0);
  } else {
    // Width is a multiple of 8 and height is a multiple of 4.
    do {
      int height = h;
      int16_t *s = src_ptr;
      uint8_t *d = dst_ptr;

      int16x8_t s0, s1, s2, s3, s4, s5, s6;
      load_s16_8x7(s, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
      s += 7 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s7, s8, s9, s10;
        load_s16_8x4(s, src_stride, &s7, &s8, &s9, &s10);

        uint8x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                        y_filter, sub_const);
        uint8x8_t d1 = convolve8_8_2d_v(s1, s2, s3, s4, s5, s6, s7, s8,
                                        y_filter, sub_const);
        uint8x8_t d2 = convolve8_8_2d_v(s2, s3, s4, s5, s6, s7, s8, s9,
                                        y_filter, sub_const);
        uint8x8_t d3 = convolve8_8_2d_v(s3, s4, s5, s6, s7, s8, s9, s10,
                                        y_filter, sub_const);

        store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

        s0 = s4;
        s1 = s5;
        s2 = s6;
        s3 = s7;
        s4 = s8;
        s5 = s9;
        s6 = s10;
        s += 4 * src_stride;
        d += 4 * dst_stride;
        height -= 4;
#else   // !AOM_ARCH_AARCH64
        int16x8_t s7 = vld1q_s16(s);
        uint8x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                        y_filter, sub_const);
        vst1_u8(d, d0);

        s0 = s1;
        s1 = s2;
        s2 = s3;
        s3 = s4;
        s4 = s5;
        s5 = s6;
        s6 = s7;
        s += src_stride;
        d += dst_stride;
        height--;
#endif  // AOM_ARCH_AARCH64
      } while (height != 0);
      src_ptr += 8;
      dst_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline int16x4_t convolve6_4_2d_v(const int16x4_t s0, const int16x4_t s1,
                                         const int16x4_t s2, const int16x4_t s3,
                                         const int16x4_t s4, const int16x4_t s5,
                                         const int16x8_t y_filter) {
  const int16x4_t y_filter_lo = vget_low_s16(y_filter);
  const int16x4_t y_filter_hi = vget_high_s16(y_filter);

  int32x4_t sum = vmull_lane_s16(s0, y_filter_lo, 1);
  sum = vmlal_lane_s16(sum, s1, y_filter_lo, 2);
  sum = vmlal_lane_s16(sum, s2, y_filter_lo, 3);
  sum = vmlal_lane_s16(sum, s3, y_filter_hi, 0);
  sum = vmlal_lane_s16(sum, s4, y_filter_hi, 1);
  sum = vmlal_lane_s16(sum, s5, y_filter_hi, 2);

  return vqrshrn_n_s32(sum, 2 * FILTER_BITS - ROUND0_BITS);
}

static inline uint8x8_t convolve6_8_2d_v(const int16x8_t s0, const int16x8_t s1,
                                         const int16x8_t s2, const int16x8_t s3,
                                         const int16x8_t s4, const int16x8_t s5,
                                         const int16x8_t y_filter,
                                         const int16x8_t sub_const) {
  const int16x4_t y_filter_lo = vget_low_s16(y_filter);
  const int16x4_t y_filter_hi = vget_high_s16(y_filter);

  int32x4_t sum0 = vmull_lane_s16(vget_low_s16(s0), y_filter_lo, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s1), y_filter_lo, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s2), y_filter_lo, 3);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s3), y_filter_hi, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s4), y_filter_hi, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s5), y_filter_hi, 2);

  int32x4_t sum1 = vmull_lane_s16(vget_high_s16(s0), y_filter_lo, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s1), y_filter_lo, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s2), y_filter_lo, 3);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s3), y_filter_hi, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s4), y_filter_hi, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s5), y_filter_hi, 2);

  int16x8_t res =
      vcombine_s16(vqrshrn_n_s32(sum0, 2 * FILTER_BITS - ROUND0_BITS),
                   vqrshrn_n_s32(sum1, 2 * FILTER_BITS - ROUND0_BITS));
  res = vsubq_s16(res, sub_const);

  return vqmovun_s16(res);
}

static inline void convolve_2d_sr_vert_6tap_neon(int16_t *src_ptr,
                                                 int src_stride,
                                                 uint8_t *dst_ptr,
                                                 int dst_stride, int w, int h,
                                                 const int16x8_t y_filter) {
  const int bd = 8;
  const int16x8_t sub_const = vdupq_n_s16(1 << (bd - 1));

  if (w <= 4) {
    int16x4_t s0, s1, s2, s3, s4;
    load_s16_4x5(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4);
    src_ptr += 5 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s5, s6, s7, s8;
      load_s16_4x4(src_ptr, src_stride, &s5, &s6, &s7, &s8);

      int16x4_t d0 = convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter);
      int16x4_t d1 = convolve6_4_2d_v(s1, s2, s3, s4, s5, s6, y_filter);
      int16x4_t d2 = convolve6_4_2d_v(s2, s3, s4, s5, s6, s7, y_filter);
      int16x4_t d3 = convolve6_4_2d_v(s3, s4, s5, s6, s7, s8, y_filter);

      uint8x8_t d01 = vqmovun_s16(vsubq_s16(vcombine_s16(d0, d1), sub_const));
      uint8x8_t d23 = vqmovun_s16(vsubq_s16(vcombine_s16(d2, d3), sub_const));

      store_u8x4_strided_x2(dst_ptr + 0 * dst_stride, dst_stride, d01);
      store_u8x4_strided_x2(dst_ptr + 2 * dst_stride, dst_stride, d23);

      s0 = s4;
      s1 = s5;
      s2 = s6;
      s3 = s7;
      s4 = s8;
      src_ptr += 4 * src_stride;
      dst_ptr += 4 * dst_stride;
      h -= 4;
#else   // !AOM_ARCH_AARCH64
      int16x4_t s5 = vld1_s16(src_ptr);
      int16x4_t d0 = convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter);
      uint8x8_t d01 =
          vqmovun_s16(vsubq_s16(vcombine_s16(d0, vdup_n_s16(0)), sub_const));

      store_u8_4x1(dst_ptr, d01);

      s0 = s1;
      s1 = s2;
      s2 = s3;
      s3 = s4;
      s4 = s5;
      src_ptr += src_stride;
      dst_ptr += dst_stride;
      h--;
#endif  // AOM_ARCH_AARCH64
    } while (h != 0);
  } else {
    // Width is a multiple of 8 and height is a multiple of 4.
    do {
      int height = h;
      int16_t *s = src_ptr;
      uint8_t *d = dst_ptr;

      int16x8_t s0, s1, s2, s3, s4;
      load_s16_8x5(s, src_stride, &s0, &s1, &s2, &s3, &s4);
      s += 5 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s5, s6, s7, s8;
        load_s16_8x4(s, src_stride, &s5, &s6, &s7, &s8);

        uint8x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, sub_const);
        uint8x8_t d1 =
            convolve6_8_2d_v(s1, s2, s3, s4, s5, s6, y_filter, sub_const);
        uint8x8_t d2 =
            convolve6_8_2d_v(s2, s3, s4, s5, s6, s7, y_filter, sub_const);
        uint8x8_t d3 =
            convolve6_8_2d_v(s3, s4, s5, s6, s7, s8, y_filter, sub_const);

        store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

        s0 = s4;
        s1 = s5;
        s2 = s6;
        s3 = s7;
        s4 = s8;
        s += 4 * src_stride;
        d += 4 * dst_stride;
        height -= 4;
#else   // !AOM_ARCH_AARCH64
        int16x8_t s5 = vld1q_s16(s);
        uint8x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, sub_const);
        vst1_u8(d, d0);

        s0 = s1;
        s1 = s2;
        s2 = s3;
        s3 = s4;
        s4 = s5;
        s += src_stride;
        d += dst_stride;
        height--;
#endif  // AOM_ARCH_AARCH64
      } while (height != 0);
      src_ptr += 8;
      dst_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline int16x4_t convolve4_4_2d_v(const int16x4_t s0, const int16x4_t s1,
                                         const int16x4_t s2, const int16x4_t s3,
                                         const int16x4_t y_filter) {
  int32x4_t sum = vmull_lane_s16(s0, y_filter, 0);
  sum = vmlal_lane_s16(sum, s1, y_filter, 1);
  sum = vmlal_lane_s16(sum, s2, y_filter, 2);
  sum = vmlal_lane_s16(sum, s3, y_filter, 3);

  return vqrshrn_n_s32(sum, 2 * FILTER_BITS - ROUND0_BITS);
}

static inline uint8x8_t convolve4_8_2d_v(const int16x8_t s0, const int16x8_t s1,
                                         const int16x8_t s2, const int16x8_t s3,
                                         const int16x4_t y_filter,
                                         const int16x8_t sub_const) {
  int32x4_t sum0 = vmull_lane_s16(vget_low_s16(s0), y_filter, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s1), y_filter, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s2), y_filter, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s3), y_filter, 3);

  int32x4_t sum1 = vmull_lane_s16(vget_high_s16(s0), y_filter, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s1), y_filter, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s2), y_filter, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s3), y_filter, 3);

  int16x8_t res =
      vcombine_s16(vqrshrn_n_s32(sum0, 2 * FILTER_BITS - ROUND0_BITS),
                   vqrshrn_n_s32(sum1, 2 * FILTER_BITS - ROUND0_BITS));
  res = vsubq_s16(res, sub_const);

  return vqmovun_s16(res);
}

static inline void convolve_2d_sr_vert_4tap_neon(int16_t *src_ptr,
                                                 int src_stride,
                                                 uint8_t *dst_ptr,
                                                 int dst_stride, int w, int h,
                                                 const int16_t *y_filter) {
  const int bd = 8;
  const int16x8_t sub_const = vdupq_n_s16(1 << (bd - 1));

  const int16x4_t filter = vld1_s16(y_filter + 2);

  if (w == 4) {
    int16x4_t s0, s1, s2;
    load_s16_4x3(src_ptr, src_stride, &s0, &s1, &s2);
    src_ptr += 3 * src_stride;

    do {
      int16x4_t s3, s4, s5, s6;
      load_s16_4x4(src_ptr, src_stride, &s3, &s4, &s5, &s6);

      int16x4_t d0 = convolve4_4_2d_v(s0, s1, s2, s3, filter);
      int16x4_t d1 = convolve4_4_2d_v(s1, s2, s3, s4, filter);
      int16x4_t d2 = convolve4_4_2d_v(s2, s3, s4, s5, filter);
      int16x4_t d3 = convolve4_4_2d_v(s3, s4, s5, s6, filter);

      uint8x8_t d01 = vqmovun_s16(vsubq_s16(vcombine_s16(d0, d1), sub_const));
      uint8x8_t d23 = vqmovun_s16(vsubq_s16(vcombine_s16(d2, d3), sub_const));

      store_u8x4_strided_x2(dst_ptr + 0 * dst_stride, dst_stride, d01);
      store_u8x4_strided_x2(dst_ptr + 2 * dst_stride, dst_stride, d23);

      s0 = s4;
      s1 = s5;
      s2 = s6;

      src_ptr += 4 * src_stride;
      dst_ptr += 4 * dst_stride;
      h -= 4;
    } while (h != 0);
  } else {
    // Width is a multiple of 8 and height is a multiple of 4.
    do {
      int height = h;
      int16_t *s = src_ptr;
      uint8_t *d = dst_ptr;

      int16x8_t s0, s1, s2;
      load_s16_8x3(s, src_stride, &s0, &s1, &s2);
      s += 3 * src_stride;

      do {
        int16x8_t s3, s4, s5, s6;
        load_s16_8x4(s, src_stride, &s3, &s4, &s5, &s6);

        uint8x8_t d0 = convolve4_8_2d_v(s0, s1, s2, s3, filter, sub_const);
        uint8x8_t d1 = convolve4_8_2d_v(s1, s2, s3, s4, filter, sub_const);
        uint8x8_t d2 = convolve4_8_2d_v(s2, s3, s4, s5, filter, sub_const);
        uint8x8_t d3 = convolve4_8_2d_v(s3, s4, s5, s6, filter, sub_const);

        store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

        s0 = s4;
        s1 = s5;
        s2 = s6;

        s += 4 * src_stride;
        d += 4 * dst_stride;
        height -= 4;
      } while (height != 0);
      src_ptr += 8;
      dst_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

#endif  // AOM_AV1_COMMON_ARM_CONVOLVE_NEON_H_
