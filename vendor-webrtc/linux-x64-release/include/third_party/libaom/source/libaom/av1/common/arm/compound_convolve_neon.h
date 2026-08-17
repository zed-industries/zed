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
#ifndef AOM_AV1_COMMON_ARM_COMPOUND_CONVOLVE_NEON_H_
#define AOM_AV1_COMMON_ARM_COMPOUND_CONVOLVE_NEON_H_

#include <arm_neon.h>

#include "av1/common/convolve.h"
#include "av1/common/enums.h"
#include "av1/common/filter.h"

static inline void compute_dist_wtd_avg_4x1(uint16x4_t dd0, uint16x4_t d0,
                                            const uint16_t fwd_offset,
                                            const uint16_t bck_offset,
                                            const int16x4_t round_offset,
                                            uint8x8_t *d0_u8) {
  uint32x4_t blend0 = vmull_n_u16(dd0, fwd_offset);
  blend0 = vmlal_n_u16(blend0, d0, bck_offset);

  uint16x4_t avg0 = vshrn_n_u32(blend0, DIST_PRECISION_BITS);

  int16x4_t dst0 = vsub_s16(vreinterpret_s16_u16(avg0), round_offset);

  int16x8_t dst0q = vcombine_s16(dst0, vdup_n_s16(0));

  *d0_u8 = vqrshrun_n_s16(dst0q, FILTER_BITS - ROUND0_BITS);
}

static inline void compute_basic_avg_4x1(uint16x4_t dd0, uint16x4_t d0,
                                         const int16x4_t round_offset,
                                         uint8x8_t *d0_u8) {
  uint16x4_t avg0 = vhadd_u16(dd0, d0);

  int16x4_t dst0 = vsub_s16(vreinterpret_s16_u16(avg0), round_offset);

  int16x8_t dst0q = vcombine_s16(dst0, vdup_n_s16(0));

  *d0_u8 = vqrshrun_n_s16(dst0q, FILTER_BITS - ROUND0_BITS);
}

static inline void compute_dist_wtd_avg_8x1(uint16x8_t dd0, uint16x8_t d0,
                                            const uint16_t fwd_offset,
                                            const uint16_t bck_offset,
                                            const int16x8_t round_offset,
                                            uint8x8_t *d0_u8) {
  uint32x4_t blend0_lo = vmull_n_u16(vget_low_u16(dd0), fwd_offset);
  blend0_lo = vmlal_n_u16(blend0_lo, vget_low_u16(d0), bck_offset);
  uint32x4_t blend0_hi = vmull_n_u16(vget_high_u16(dd0), fwd_offset);
  blend0_hi = vmlal_n_u16(blend0_hi, vget_high_u16(d0), bck_offset);

  uint16x8_t avg0 = vcombine_u16(vshrn_n_u32(blend0_lo, DIST_PRECISION_BITS),
                                 vshrn_n_u32(blend0_hi, DIST_PRECISION_BITS));

  int16x8_t dst0 = vsubq_s16(vreinterpretq_s16_u16(avg0), round_offset);

  *d0_u8 = vqrshrun_n_s16(dst0, FILTER_BITS - ROUND0_BITS);
}

static inline void compute_basic_avg_8x1(uint16x8_t dd0, uint16x8_t d0,
                                         const int16x8_t round_offset,
                                         uint8x8_t *d0_u8) {
  uint16x8_t avg0 = vhaddq_u16(dd0, d0);

  int16x8_t dst0 = vsubq_s16(vreinterpretq_s16_u16(avg0), round_offset);

  *d0_u8 = vqrshrun_n_s16(dst0, FILTER_BITS - ROUND0_BITS);
}

static inline void compute_dist_wtd_avg_4x4(
    uint16x4_t dd0, uint16x4_t dd1, uint16x4_t dd2, uint16x4_t dd3,
    uint16x4_t d0, uint16x4_t d1, uint16x4_t d2, uint16x4_t d3,
    const uint16_t fwd_offset, const uint16_t bck_offset,
    const int16x8_t round_offset, uint8x8_t *d01_u8, uint8x8_t *d23_u8) {
  uint32x4_t blend0 = vmull_n_u16(dd0, fwd_offset);
  blend0 = vmlal_n_u16(blend0, d0, bck_offset);
  uint32x4_t blend1 = vmull_n_u16(dd1, fwd_offset);
  blend1 = vmlal_n_u16(blend1, d1, bck_offset);
  uint32x4_t blend2 = vmull_n_u16(dd2, fwd_offset);
  blend2 = vmlal_n_u16(blend2, d2, bck_offset);
  uint32x4_t blend3 = vmull_n_u16(dd3, fwd_offset);
  blend3 = vmlal_n_u16(blend3, d3, bck_offset);

  uint16x4_t avg0 = vshrn_n_u32(blend0, DIST_PRECISION_BITS);
  uint16x4_t avg1 = vshrn_n_u32(blend1, DIST_PRECISION_BITS);
  uint16x4_t avg2 = vshrn_n_u32(blend2, DIST_PRECISION_BITS);
  uint16x4_t avg3 = vshrn_n_u32(blend3, DIST_PRECISION_BITS);

  int16x8_t dst_01 = vreinterpretq_s16_u16(vcombine_u16(avg0, avg1));
  int16x8_t dst_23 = vreinterpretq_s16_u16(vcombine_u16(avg2, avg3));

  dst_01 = vsubq_s16(dst_01, round_offset);
  dst_23 = vsubq_s16(dst_23, round_offset);

  *d01_u8 = vqrshrun_n_s16(dst_01, FILTER_BITS - ROUND0_BITS);
  *d23_u8 = vqrshrun_n_s16(dst_23, FILTER_BITS - ROUND0_BITS);
}

static inline void compute_basic_avg_4x4(uint16x4_t dd0, uint16x4_t dd1,
                                         uint16x4_t dd2, uint16x4_t dd3,
                                         uint16x4_t d0, uint16x4_t d1,
                                         uint16x4_t d2, uint16x4_t d3,
                                         const int16x8_t round_offset,
                                         uint8x8_t *d01_u8, uint8x8_t *d23_u8) {
  uint16x4_t avg0 = vhadd_u16(dd0, d0);
  uint16x4_t avg1 = vhadd_u16(dd1, d1);
  uint16x4_t avg2 = vhadd_u16(dd2, d2);
  uint16x4_t avg3 = vhadd_u16(dd3, d3);

  int16x8_t dst_01 = vreinterpretq_s16_u16(vcombine_u16(avg0, avg1));
  int16x8_t dst_23 = vreinterpretq_s16_u16(vcombine_u16(avg2, avg3));

  dst_01 = vsubq_s16(dst_01, round_offset);
  dst_23 = vsubq_s16(dst_23, round_offset);

  *d01_u8 = vqrshrun_n_s16(dst_01, FILTER_BITS - ROUND0_BITS);
  *d23_u8 = vqrshrun_n_s16(dst_23, FILTER_BITS - ROUND0_BITS);
}

static inline void compute_dist_wtd_avg_8x4(
    uint16x8_t dd0, uint16x8_t dd1, uint16x8_t dd2, uint16x8_t dd3,
    uint16x8_t d0, uint16x8_t d1, uint16x8_t d2, uint16x8_t d3,
    const uint16_t fwd_offset, const uint16_t bck_offset,
    const int16x8_t round_offset, uint8x8_t *d0_u8, uint8x8_t *d1_u8,
    uint8x8_t *d2_u8, uint8x8_t *d3_u8) {
  uint32x4_t blend0_lo = vmull_n_u16(vget_low_u16(dd0), fwd_offset);
  blend0_lo = vmlal_n_u16(blend0_lo, vget_low_u16(d0), bck_offset);
  uint32x4_t blend0_hi = vmull_n_u16(vget_high_u16(dd0), fwd_offset);
  blend0_hi = vmlal_n_u16(blend0_hi, vget_high_u16(d0), bck_offset);

  uint32x4_t blend1_lo = vmull_n_u16(vget_low_u16(dd1), fwd_offset);
  blend1_lo = vmlal_n_u16(blend1_lo, vget_low_u16(d1), bck_offset);
  uint32x4_t blend1_hi = vmull_n_u16(vget_high_u16(dd1), fwd_offset);
  blend1_hi = vmlal_n_u16(blend1_hi, vget_high_u16(d1), bck_offset);

  uint32x4_t blend2_lo = vmull_n_u16(vget_low_u16(dd2), fwd_offset);
  blend2_lo = vmlal_n_u16(blend2_lo, vget_low_u16(d2), bck_offset);
  uint32x4_t blend2_hi = vmull_n_u16(vget_high_u16(dd2), fwd_offset);
  blend2_hi = vmlal_n_u16(blend2_hi, vget_high_u16(d2), bck_offset);

  uint32x4_t blend3_lo = vmull_n_u16(vget_low_u16(dd3), fwd_offset);
  blend3_lo = vmlal_n_u16(blend3_lo, vget_low_u16(d3), bck_offset);
  uint32x4_t blend3_hi = vmull_n_u16(vget_high_u16(dd3), fwd_offset);
  blend3_hi = vmlal_n_u16(blend3_hi, vget_high_u16(d3), bck_offset);

  uint16x8_t avg0 = vcombine_u16(vshrn_n_u32(blend0_lo, DIST_PRECISION_BITS),
                                 vshrn_n_u32(blend0_hi, DIST_PRECISION_BITS));
  uint16x8_t avg1 = vcombine_u16(vshrn_n_u32(blend1_lo, DIST_PRECISION_BITS),
                                 vshrn_n_u32(blend1_hi, DIST_PRECISION_BITS));
  uint16x8_t avg2 = vcombine_u16(vshrn_n_u32(blend2_lo, DIST_PRECISION_BITS),
                                 vshrn_n_u32(blend2_hi, DIST_PRECISION_BITS));
  uint16x8_t avg3 = vcombine_u16(vshrn_n_u32(blend3_lo, DIST_PRECISION_BITS),
                                 vshrn_n_u32(blend3_hi, DIST_PRECISION_BITS));

  int16x8_t dst0 = vsubq_s16(vreinterpretq_s16_u16(avg0), round_offset);
  int16x8_t dst1 = vsubq_s16(vreinterpretq_s16_u16(avg1), round_offset);
  int16x8_t dst2 = vsubq_s16(vreinterpretq_s16_u16(avg2), round_offset);
  int16x8_t dst3 = vsubq_s16(vreinterpretq_s16_u16(avg3), round_offset);

  *d0_u8 = vqrshrun_n_s16(dst0, FILTER_BITS - ROUND0_BITS);
  *d1_u8 = vqrshrun_n_s16(dst1, FILTER_BITS - ROUND0_BITS);
  *d2_u8 = vqrshrun_n_s16(dst2, FILTER_BITS - ROUND0_BITS);
  *d3_u8 = vqrshrun_n_s16(dst3, FILTER_BITS - ROUND0_BITS);
}

static inline void compute_basic_avg_8x4(uint16x8_t dd0, uint16x8_t dd1,
                                         uint16x8_t dd2, uint16x8_t dd3,
                                         uint16x8_t d0, uint16x8_t d1,
                                         uint16x8_t d2, uint16x8_t d3,
                                         const int16x8_t round_offset,
                                         uint8x8_t *d0_u8, uint8x8_t *d1_u8,
                                         uint8x8_t *d2_u8, uint8x8_t *d3_u8) {
  uint16x8_t avg0 = vhaddq_u16(dd0, d0);
  uint16x8_t avg1 = vhaddq_u16(dd1, d1);
  uint16x8_t avg2 = vhaddq_u16(dd2, d2);
  uint16x8_t avg3 = vhaddq_u16(dd3, d3);

  int16x8_t dst0 = vsubq_s16(vreinterpretq_s16_u16(avg0), round_offset);
  int16x8_t dst1 = vsubq_s16(vreinterpretq_s16_u16(avg1), round_offset);
  int16x8_t dst2 = vsubq_s16(vreinterpretq_s16_u16(avg2), round_offset);
  int16x8_t dst3 = vsubq_s16(vreinterpretq_s16_u16(avg3), round_offset);

  *d0_u8 = vqrshrun_n_s16(dst0, FILTER_BITS - ROUND0_BITS);
  *d1_u8 = vqrshrun_n_s16(dst1, FILTER_BITS - ROUND0_BITS);
  *d2_u8 = vqrshrun_n_s16(dst2, FILTER_BITS - ROUND0_BITS);
  *d3_u8 = vqrshrun_n_s16(dst3, FILTER_BITS - ROUND0_BITS);
}

static inline uint16x4_t convolve6_4_2d_v(
    const int16x4_t s0, const int16x4_t s1, const int16x4_t s2,
    const int16x4_t s3, const int16x4_t s4, const int16x4_t s5,
    const int16x8_t y_filter, const int32x4_t offset_const) {
  const int16x4_t y_filter_0_3 = vget_low_s16(y_filter);
  const int16x4_t y_filter_4_7 = vget_high_s16(y_filter);

  int32x4_t sum = offset_const;
  // Filter values at indices 0 and 7 are 0.
  sum = vmlal_lane_s16(sum, s0, y_filter_0_3, 1);
  sum = vmlal_lane_s16(sum, s1, y_filter_0_3, 2);
  sum = vmlal_lane_s16(sum, s2, y_filter_0_3, 3);
  sum = vmlal_lane_s16(sum, s3, y_filter_4_7, 0);
  sum = vmlal_lane_s16(sum, s4, y_filter_4_7, 1);
  sum = vmlal_lane_s16(sum, s5, y_filter_4_7, 2);

  return vqrshrun_n_s32(sum, COMPOUND_ROUND1_BITS);
}

static inline uint16x8_t convolve6_8_2d_v(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x8_t s4, const int16x8_t s5,
    const int16x8_t y_filter, const int32x4_t offset_const) {
  const int16x4_t y_filter_0_3 = vget_low_s16(y_filter);
  const int16x4_t y_filter_4_7 = vget_high_s16(y_filter);

  int32x4_t sum0 = offset_const;
  // Filter values at indices 0 and 7 are 0.
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s0), y_filter_0_3, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s1), y_filter_0_3, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s2), y_filter_0_3, 3);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s3), y_filter_4_7, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s4), y_filter_4_7, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s5), y_filter_4_7, 2);

  int32x4_t sum1 = offset_const;
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s0), y_filter_0_3, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s1), y_filter_0_3, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s2), y_filter_0_3, 3);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s3), y_filter_4_7, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s4), y_filter_4_7, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s5), y_filter_4_7, 2);

  return vcombine_u16(vqrshrun_n_s32(sum0, COMPOUND_ROUND1_BITS),
                      vqrshrun_n_s32(sum1, COMPOUND_ROUND1_BITS));
}

static inline void dist_wtd_convolve_2d_vert_6tap_dist_wtd_avg_neon(
    int16_t *src_ptr, const int src_stride, uint8_t *dst8_ptr, int dst8_stride,
    ConvolveParams *conv_params, const int16x8_t y_filter, int h, int w) {
  const int bd = 8;
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int32x4_t offset_const = vdupq_n_s32(1 << offset_bits);
  const int16_t round_offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                               (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));
  const int16x8_t round_offset_vec = vdupq_n_s16(round_offset);

  const uint16_t fwd_offset = conv_params->fwd_offset;
  const uint16_t bck_offset = conv_params->bck_offset;

  CONV_BUF_TYPE *dst_ptr = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;

  if (w == 4) {
    int16x4_t s0, s1, s2, s3, s4;
    load_s16_4x5(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4);
    src_ptr += 5 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s5, s6, s7, s8;
      load_s16_4x4(src_ptr, src_stride, &s5, &s6, &s7, &s8);

      uint16x4_t d0 =
          convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);
      uint16x4_t d1 =
          convolve6_4_2d_v(s1, s2, s3, s4, s5, s6, y_filter, offset_const);
      uint16x4_t d2 =
          convolve6_4_2d_v(s2, s3, s4, s5, s6, s7, y_filter, offset_const);
      uint16x4_t d3 =
          convolve6_4_2d_v(s3, s4, s5, s6, s7, s8, y_filter, offset_const);

      uint16x4_t dd0, dd1, dd2, dd3;
      load_u16_4x4(dst_ptr, dst_stride, &dd0, &dd1, &dd2, &dd3);

      uint8x8_t d01_u8, d23_u8;
      compute_dist_wtd_avg_4x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3, fwd_offset,
                               bck_offset, round_offset_vec, &d01_u8, &d23_u8);

      store_u8x4_strided_x2(dst8_ptr + 0 * dst8_stride, dst8_stride, d01_u8);
      store_u8x4_strided_x2(dst8_ptr + 2 * dst8_stride, dst8_stride, d23_u8);
      dst8_ptr += 4 * dst8_stride;

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

      uint16x4_t d0 =
          convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);

      uint16x4_t dd0 = vld1_u16(dst_ptr);

      uint8x8_t d01_u8;
      compute_dist_wtd_avg_4x1(dd0, d0, fwd_offset, bck_offset,
                               vget_low_s16(round_offset_vec), &d01_u8);

      store_u8_4x1(dst8_ptr, d01_u8);
      dst8_ptr += dst8_stride;

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
    do {
      int16_t *s = src_ptr;
      CONV_BUF_TYPE *d = dst_ptr;
      uint8_t *d_u8 = dst8_ptr;
      int height = h;

      int16x8_t s0, s1, s2, s3, s4;
      load_s16_8x5(s, src_stride, &s0, &s1, &s2, &s3, &s4);
      s += 5 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s5, s6, s7, s8;
        load_s16_8x4(s, src_stride, &s5, &s6, &s7, &s8);

        uint16x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);
        uint16x8_t d1 =
            convolve6_8_2d_v(s1, s2, s3, s4, s5, s6, y_filter, offset_const);
        uint16x8_t d2 =
            convolve6_8_2d_v(s2, s3, s4, s5, s6, s7, y_filter, offset_const);
        uint16x8_t d3 =
            convolve6_8_2d_v(s3, s4, s5, s6, s7, s8, y_filter, offset_const);

        uint16x8_t dd0, dd1, dd2, dd3;
        load_u16_8x4(d, dst_stride, &dd0, &dd1, &dd2, &dd3);

        uint8x8_t d0_u8, d1_u8, d2_u8, d3_u8;
        compute_dist_wtd_avg_8x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3, fwd_offset,
                                 bck_offset, round_offset_vec, &d0_u8, &d1_u8,
                                 &d2_u8, &d3_u8);

        store_u8_8x4(d_u8, dst8_stride, d0_u8, d1_u8, d2_u8, d3_u8);
        d_u8 += 4 * dst8_stride;

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

        uint16x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);

        uint16x8_t dd0 = vld1q_u16(d);

        uint8x8_t d0_u8;
        compute_dist_wtd_avg_8x1(dd0, d0, fwd_offset, bck_offset,
                                 round_offset_vec, &d0_u8);

        vst1_u8(d_u8, d0_u8);
        d_u8 += dst8_stride;

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
      dst8_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline void dist_wtd_convolve_2d_vert_6tap_avg_neon(
    int16_t *src_ptr, const int src_stride, uint8_t *dst8_ptr, int dst8_stride,
    ConvolveParams *conv_params, const int16x8_t y_filter, int h, int w) {
  const int bd = 8;
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int32x4_t offset_const = vdupq_n_s32(1 << offset_bits);
  const int16_t round_offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                               (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));
  const int16x8_t round_offset_vec = vdupq_n_s16(round_offset);

  CONV_BUF_TYPE *dst_ptr = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;

  if (w == 4) {
    int16x4_t s0, s1, s2, s3, s4;
    load_s16_4x5(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4);
    src_ptr += 5 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s5, s6, s7, s8;
      load_s16_4x4(src_ptr, src_stride, &s5, &s6, &s7, &s8);

      uint16x4_t d0 =
          convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);
      uint16x4_t d1 =
          convolve6_4_2d_v(s1, s2, s3, s4, s5, s6, y_filter, offset_const);
      uint16x4_t d2 =
          convolve6_4_2d_v(s2, s3, s4, s5, s6, s7, y_filter, offset_const);
      uint16x4_t d3 =
          convolve6_4_2d_v(s3, s4, s5, s6, s7, s8, y_filter, offset_const);

      uint16x4_t dd0, dd1, dd2, dd3;
      load_u16_4x4(dst_ptr, dst_stride, &dd0, &dd1, &dd2, &dd3);

      uint8x8_t d01_u8, d23_u8;
      compute_basic_avg_4x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3,
                            round_offset_vec, &d01_u8, &d23_u8);

      store_u8x4_strided_x2(dst8_ptr + 0 * dst8_stride, dst8_stride, d01_u8);
      store_u8x4_strided_x2(dst8_ptr + 2 * dst8_stride, dst8_stride, d23_u8);
      dst8_ptr += 4 * dst8_stride;

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

      uint16x4_t d0 =
          convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);

      uint16x4_t dd0 = vld1_u16(dst_ptr);

      uint8x8_t d01_u8;
      compute_basic_avg_4x1(dd0, d0, vget_low_s16(round_offset_vec), &d01_u8);

      store_u8_4x1(dst8_ptr, d01_u8);
      dst8_ptr += dst8_stride;

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
    do {
      int16_t *s = src_ptr;
      CONV_BUF_TYPE *d = dst_ptr;
      uint8_t *d_u8 = dst8_ptr;
      int height = h;

      int16x8_t s0, s1, s2, s3, s4;
      load_s16_8x5(s, src_stride, &s0, &s1, &s2, &s3, &s4);
      s += 5 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s5, s6, s7, s8;
        load_s16_8x4(s, src_stride, &s5, &s6, &s7, &s8);

        uint16x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);
        uint16x8_t d1 =
            convolve6_8_2d_v(s1, s2, s3, s4, s5, s6, y_filter, offset_const);
        uint16x8_t d2 =
            convolve6_8_2d_v(s2, s3, s4, s5, s6, s7, y_filter, offset_const);
        uint16x8_t d3 =
            convolve6_8_2d_v(s3, s4, s5, s6, s7, s8, y_filter, offset_const);

        uint16x8_t dd0, dd1, dd2, dd3;
        load_u16_8x4(d, dst_stride, &dd0, &dd1, &dd2, &dd3);

        uint8x8_t d0_u8, d1_u8, d2_u8, d3_u8;
        compute_basic_avg_8x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3,
                              round_offset_vec, &d0_u8, &d1_u8, &d2_u8, &d3_u8);

        store_u8_8x4(d_u8, dst8_stride, d0_u8, d1_u8, d2_u8, d3_u8);
        d_u8 += 4 * dst8_stride;

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

        uint16x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);

        uint16x8_t dd0 = vld1q_u16(d);

        uint8x8_t d0_u8;
        compute_basic_avg_8x1(dd0, d0, round_offset_vec, &d0_u8);

        vst1_u8(d_u8, d0_u8);
        d_u8 += dst8_stride;

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
      dst8_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline void dist_wtd_convolve_2d_vert_6tap_neon(
    int16_t *src_ptr, const int src_stride, ConvolveParams *conv_params,
    const int16x8_t y_filter, int h, int w) {
  const int bd = 8;
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int32x4_t offset_const = vdupq_n_s32(1 << offset_bits);

  CONV_BUF_TYPE *dst_ptr = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;

  if (w == 4) {
    int16x4_t s0, s1, s2, s3, s4;
    load_s16_4x5(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4);
    src_ptr += 5 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s5, s6, s7, s8;
      load_s16_4x4(src_ptr, src_stride, &s5, &s6, &s7, &s8);

      uint16x4_t d0 =
          convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);
      uint16x4_t d1 =
          convolve6_4_2d_v(s1, s2, s3, s4, s5, s6, y_filter, offset_const);
      uint16x4_t d2 =
          convolve6_4_2d_v(s2, s3, s4, s5, s6, s7, y_filter, offset_const);
      uint16x4_t d3 =
          convolve6_4_2d_v(s3, s4, s5, s6, s7, s8, y_filter, offset_const);

      store_u16_4x4(dst_ptr, dst_stride, d0, d1, d2, d3);

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

      uint16x4_t d0 =
          convolve6_4_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);

      vst1_u16(dst_ptr, d0);

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
    do {
      int16_t *s = src_ptr;
      CONV_BUF_TYPE *d = dst_ptr;
      int height = h;

      int16x8_t s0, s1, s2, s3, s4;
      load_s16_8x5(s, src_stride, &s0, &s1, &s2, &s3, &s4);
      s += 5 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s5, s6, s7, s8;
        load_s16_8x4(s, src_stride, &s5, &s6, &s7, &s8);

        uint16x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);
        uint16x8_t d1 =
            convolve6_8_2d_v(s1, s2, s3, s4, s5, s6, y_filter, offset_const);
        uint16x8_t d2 =
            convolve6_8_2d_v(s2, s3, s4, s5, s6, s7, y_filter, offset_const);
        uint16x8_t d3 =
            convolve6_8_2d_v(s3, s4, s5, s6, s7, s8, y_filter, offset_const);

        store_u16_8x4(d, dst_stride, d0, d1, d2, d3);

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

        uint16x8_t d0 =
            convolve6_8_2d_v(s0, s1, s2, s3, s4, s5, y_filter, offset_const);

        vst1q_u16(d, d0);

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

static inline uint16x4_t convolve8_4_2d_v(
    const int16x4_t s0, const int16x4_t s1, const int16x4_t s2,
    const int16x4_t s3, const int16x4_t s4, const int16x4_t s5,
    const int16x4_t s6, const int16x4_t s7, const int16x8_t y_filter,
    const int32x4_t offset_const) {
  const int16x4_t y_filter_0_3 = vget_low_s16(y_filter);
  const int16x4_t y_filter_4_7 = vget_high_s16(y_filter);

  int32x4_t sum = offset_const;
  sum = vmlal_lane_s16(sum, s0, y_filter_0_3, 0);
  sum = vmlal_lane_s16(sum, s1, y_filter_0_3, 1);
  sum = vmlal_lane_s16(sum, s2, y_filter_0_3, 2);
  sum = vmlal_lane_s16(sum, s3, y_filter_0_3, 3);
  sum = vmlal_lane_s16(sum, s4, y_filter_4_7, 0);
  sum = vmlal_lane_s16(sum, s5, y_filter_4_7, 1);
  sum = vmlal_lane_s16(sum, s6, y_filter_4_7, 2);
  sum = vmlal_lane_s16(sum, s7, y_filter_4_7, 3);

  return vqrshrun_n_s32(sum, COMPOUND_ROUND1_BITS);
}

static inline uint16x8_t convolve8_8_2d_v(
    const int16x8_t s0, const int16x8_t s1, const int16x8_t s2,
    const int16x8_t s3, const int16x8_t s4, const int16x8_t s5,
    const int16x8_t s6, const int16x8_t s7, const int16x8_t y_filter,
    const int32x4_t offset_const) {
  const int16x4_t y_filter_0_3 = vget_low_s16(y_filter);
  const int16x4_t y_filter_4_7 = vget_high_s16(y_filter);

  int32x4_t sum0 = offset_const;
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s0), y_filter_0_3, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s1), y_filter_0_3, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s2), y_filter_0_3, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s3), y_filter_0_3, 3);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s4), y_filter_4_7, 0);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s5), y_filter_4_7, 1);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s6), y_filter_4_7, 2);
  sum0 = vmlal_lane_s16(sum0, vget_low_s16(s7), y_filter_4_7, 3);

  int32x4_t sum1 = offset_const;
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s0), y_filter_0_3, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s1), y_filter_0_3, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s2), y_filter_0_3, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s3), y_filter_0_3, 3);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s4), y_filter_4_7, 0);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s5), y_filter_4_7, 1);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s6), y_filter_4_7, 2);
  sum1 = vmlal_lane_s16(sum1, vget_high_s16(s7), y_filter_4_7, 3);

  return vcombine_u16(vqrshrun_n_s32(sum0, COMPOUND_ROUND1_BITS),
                      vqrshrun_n_s32(sum1, COMPOUND_ROUND1_BITS));
}

static inline void dist_wtd_convolve_2d_vert_8tap_dist_wtd_avg_neon(
    int16_t *src_ptr, const int src_stride, uint8_t *dst8_ptr, int dst8_stride,
    ConvolveParams *conv_params, const int16x8_t y_filter, int h, int w) {
  const int bd = 8;
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int32x4_t offset_const = vdupq_n_s32(1 << offset_bits);
  const int16_t round_offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                               (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));
  const int16x8_t round_offset_vec = vdupq_n_s16(round_offset);

  const uint16_t fwd_offset = conv_params->fwd_offset;
  const uint16_t bck_offset = conv_params->bck_offset;

  CONV_BUF_TYPE *dst_ptr = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;

  if (w == 4) {
    int16x4_t s0, s1, s2, s3, s4, s5, s6;
    load_s16_4x7(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
    src_ptr += 7 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s7, s8, s9, s10;
      load_s16_4x4(src_ptr, src_stride, &s7, &s8, &s9, &s10);

      uint16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter,
                                       offset_const);
      uint16x4_t d1 = convolve8_4_2d_v(s1, s2, s3, s4, s5, s6, s7, s8, y_filter,
                                       offset_const);
      uint16x4_t d2 = convolve8_4_2d_v(s2, s3, s4, s5, s6, s7, s8, s9, y_filter,
                                       offset_const);
      uint16x4_t d3 = convolve8_4_2d_v(s3, s4, s5, s6, s7, s8, s9, s10,
                                       y_filter, offset_const);

      uint16x4_t dd0, dd1, dd2, dd3;
      load_u16_4x4(dst_ptr, dst_stride, &dd0, &dd1, &dd2, &dd3);

      uint8x8_t d01_u8, d23_u8;
      compute_dist_wtd_avg_4x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3, fwd_offset,
                               bck_offset, round_offset_vec, &d01_u8, &d23_u8);

      store_u8x4_strided_x2(dst8_ptr + 0 * dst8_stride, dst8_stride, d01_u8);
      store_u8x4_strided_x2(dst8_ptr + 2 * dst8_stride, dst8_stride, d23_u8);
      dst8_ptr += 4 * dst8_stride;

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

      uint16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter,
                                       offset_const);

      uint16x4_t dd0 = vld1_u16(dst_ptr);

      uint8x8_t d01_u8;
      compute_dist_wtd_avg_4x1(dd0, d0, fwd_offset, bck_offset,
                               vget_low_s16(round_offset_vec), &d01_u8);

      store_u8_4x1(dst8_ptr, d01_u8);
      dst8_ptr += dst8_stride;

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
    do {
      int16_t *s = src_ptr;
      CONV_BUF_TYPE *d = dst_ptr;
      uint8_t *d_u8 = dst8_ptr;
      int height = h;

      int16x8_t s0, s1, s2, s3, s4, s5, s6;
      load_s16_8x7(s, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
      s += 7 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s7, s8, s9, s10;
        load_s16_8x4(s, src_stride, &s7, &s8, &s9, &s10);

        uint16x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                         y_filter, offset_const);
        uint16x8_t d1 = convolve8_8_2d_v(s1, s2, s3, s4, s5, s6, s7, s8,
                                         y_filter, offset_const);
        uint16x8_t d2 = convolve8_8_2d_v(s2, s3, s4, s5, s6, s7, s8, s9,
                                         y_filter, offset_const);
        uint16x8_t d3 = convolve8_8_2d_v(s3, s4, s5, s6, s7, s8, s9, s10,
                                         y_filter, offset_const);

        uint16x8_t dd0, dd1, dd2, dd3;
        load_u16_8x4(d, dst_stride, &dd0, &dd1, &dd2, &dd3);

        uint8x8_t d0_u8, d1_u8, d2_u8, d3_u8;
        compute_dist_wtd_avg_8x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3, fwd_offset,
                                 bck_offset, round_offset_vec, &d0_u8, &d1_u8,
                                 &d2_u8, &d3_u8);

        store_u8_8x4(d_u8, dst8_stride, d0_u8, d1_u8, d2_u8, d3_u8);
        d_u8 += 4 * dst8_stride;

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

        uint16x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                         y_filter, offset_const);

        uint16x8_t dd0 = vld1q_u16(d);

        uint8x8_t d0_u8;
        compute_dist_wtd_avg_8x1(dd0, d0, fwd_offset, bck_offset,
                                 round_offset_vec, &d0_u8);

        vst1_u8(d_u8, d0_u8);
        d_u8 += dst8_stride;

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
      dst8_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline void dist_wtd_convolve_2d_vert_8tap_avg_neon(
    int16_t *src_ptr, const int src_stride, uint8_t *dst8_ptr, int dst8_stride,
    ConvolveParams *conv_params, const int16x8_t y_filter, int h, int w) {
  const int bd = 8;
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int32x4_t offset_const = vdupq_n_s32(1 << offset_bits);
  const int16_t round_offset = (1 << (offset_bits - COMPOUND_ROUND1_BITS)) +
                               (1 << (offset_bits - COMPOUND_ROUND1_BITS - 1));
  const int16x8_t round_offset_vec = vdupq_n_s16(round_offset);

  CONV_BUF_TYPE *dst_ptr = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;

  if (w == 4) {
    int16x4_t s0, s1, s2, s3, s4, s5, s6;
    load_s16_4x7(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
    src_ptr += 7 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s7, s8, s9, s10;
      load_s16_4x4(src_ptr, src_stride, &s7, &s8, &s9, &s10);

      uint16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter,
                                       offset_const);
      uint16x4_t d1 = convolve8_4_2d_v(s1, s2, s3, s4, s5, s6, s7, s8, y_filter,
                                       offset_const);
      uint16x4_t d2 = convolve8_4_2d_v(s2, s3, s4, s5, s6, s7, s8, s9, y_filter,
                                       offset_const);
      uint16x4_t d3 = convolve8_4_2d_v(s3, s4, s5, s6, s7, s8, s9, s10,
                                       y_filter, offset_const);

      uint16x4_t dd0, dd1, dd2, dd3;
      load_u16_4x4(dst_ptr, dst_stride, &dd0, &dd1, &dd2, &dd3);

      uint8x8_t d01_u8, d23_u8;
      compute_basic_avg_4x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3,
                            round_offset_vec, &d01_u8, &d23_u8);

      store_u8x4_strided_x2(dst8_ptr + 0 * dst8_stride, dst8_stride, d01_u8);
      store_u8x4_strided_x2(dst8_ptr + 2 * dst8_stride, dst8_stride, d23_u8);
      dst8_ptr += 4 * dst8_stride;

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

      uint16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter,
                                       offset_const);

      uint16x4_t dd0 = vld1_u16(dst_ptr);

      uint8x8_t d01_u8;
      compute_basic_avg_4x1(dd0, d0, vget_low_s16(round_offset_vec), &d01_u8);

      store_u8_4x1(dst8_ptr, d01_u8);
      dst8_ptr += dst8_stride;

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
    do {
      int16_t *s = src_ptr;
      CONV_BUF_TYPE *d = dst_ptr;
      uint8_t *d_u8 = dst8_ptr;
      int height = h;

      int16x8_t s0, s1, s2, s3, s4, s5, s6;
      load_s16_8x7(s, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
      s += 7 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s7, s8, s9, s10;
        load_s16_8x4(s, src_stride, &s7, &s8, &s9, &s10);

        uint16x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                         y_filter, offset_const);
        uint16x8_t d1 = convolve8_8_2d_v(s1, s2, s3, s4, s5, s6, s7, s8,
                                         y_filter, offset_const);
        uint16x8_t d2 = convolve8_8_2d_v(s2, s3, s4, s5, s6, s7, s8, s9,
                                         y_filter, offset_const);
        uint16x8_t d3 = convolve8_8_2d_v(s3, s4, s5, s6, s7, s8, s9, s10,
                                         y_filter, offset_const);

        uint16x8_t dd0, dd1, dd2, dd3;
        load_u16_8x4(d, dst_stride, &dd0, &dd1, &dd2, &dd3);

        uint8x8_t d0_u8, d1_u8, d2_u8, d3_u8;
        compute_basic_avg_8x4(dd0, dd1, dd2, dd3, d0, d1, d2, d3,
                              round_offset_vec, &d0_u8, &d1_u8, &d2_u8, &d3_u8);

        store_u8_8x4(d_u8, dst8_stride, d0_u8, d1_u8, d2_u8, d3_u8);
        d_u8 += 4 * dst8_stride;

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

        uint16x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                         y_filter, offset_const);

        uint16x8_t dd0 = vld1q_u16(d);

        uint8x8_t d0_u8;
        compute_basic_avg_8x1(dd0, d0, round_offset_vec, &d0_u8);

        vst1_u8(d_u8, d0_u8);
        d_u8 += dst8_stride;

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
      dst8_ptr += 8;
      w -= 8;
    } while (w != 0);
  }
}

static inline void dist_wtd_convolve_2d_vert_8tap_neon(
    int16_t *src_ptr, const int src_stride, ConvolveParams *conv_params,
    const int16x8_t y_filter, int h, int w) {
  const int bd = 8;
  const int offset_bits = bd + 2 * FILTER_BITS - ROUND0_BITS;
  const int32x4_t offset_const = vdupq_n_s32(1 << offset_bits);

  CONV_BUF_TYPE *dst_ptr = conv_params->dst;
  const int dst_stride = conv_params->dst_stride;

  if (w == 4) {
    int16x4_t s0, s1, s2, s3, s4, s5, s6;
    load_s16_4x7(src_ptr, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
    src_ptr += 7 * src_stride;

    do {
#if AOM_ARCH_AARCH64
      int16x4_t s7, s8, s9, s10;
      load_s16_4x4(src_ptr, src_stride, &s7, &s8, &s9, &s10);

      uint16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter,
                                       offset_const);
      uint16x4_t d1 = convolve8_4_2d_v(s1, s2, s3, s4, s5, s6, s7, s8, y_filter,
                                       offset_const);
      uint16x4_t d2 = convolve8_4_2d_v(s2, s3, s4, s5, s6, s7, s8, s9, y_filter,
                                       offset_const);
      uint16x4_t d3 = convolve8_4_2d_v(s3, s4, s5, s6, s7, s8, s9, s10,
                                       y_filter, offset_const);

      store_u16_4x4(dst_ptr, dst_stride, d0, d1, d2, d3);

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

      uint16x4_t d0 = convolve8_4_2d_v(s0, s1, s2, s3, s4, s5, s6, s7, y_filter,
                                       offset_const);

      vst1_u16(dst_ptr, d0);

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
    do {
      int16_t *s = src_ptr;
      CONV_BUF_TYPE *d = dst_ptr;
      int height = h;

      int16x8_t s0, s1, s2, s3, s4, s5, s6;
      load_s16_8x7(s, src_stride, &s0, &s1, &s2, &s3, &s4, &s5, &s6);
      s += 7 * src_stride;

      do {
#if AOM_ARCH_AARCH64
        int16x8_t s7, s8, s9, s10;
        load_s16_8x4(s, src_stride, &s7, &s8, &s9, &s10);

        uint16x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                         y_filter, offset_const);
        uint16x8_t d1 = convolve8_8_2d_v(s1, s2, s3, s4, s5, s6, s7, s8,
                                         y_filter, offset_const);
        uint16x8_t d2 = convolve8_8_2d_v(s2, s3, s4, s5, s6, s7, s8, s9,
                                         y_filter, offset_const);
        uint16x8_t d3 = convolve8_8_2d_v(s3, s4, s5, s6, s7, s8, s9, s10,
                                         y_filter, offset_const);

        store_u16_8x4(d, dst_stride, d0, d1, d2, d3);

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

        uint16x8_t d0 = convolve8_8_2d_v(s0, s1, s2, s3, s4, s5, s6, s7,
                                         y_filter, offset_const);

        vst1q_u16(d, d0);

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

#endif  // AOM_AV1_COMMON_ARM_COMPOUND_CONVOLVE_NEON_H_
