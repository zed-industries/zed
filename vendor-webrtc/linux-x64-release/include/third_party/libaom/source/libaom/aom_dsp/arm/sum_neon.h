/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_ARM_SUM_NEON_H_
#define AOM_AOM_DSP_ARM_SUM_NEON_H_

#include "config/aom_dsp_rtcd.h"
#include "config/aom_config.h"

#include "aom/aom_integer.h"
#include "aom_ports/mem.h"

static inline int horizontal_add_u8x8(const uint8x8_t a) {
#if AOM_ARCH_AARCH64
  return vaddlv_u8(a);
#else
  uint16x4_t b = vpaddl_u8(a);
  uint32x2_t c = vpaddl_u16(b);
  return vget_lane_u32(c, 0) + vget_lane_u32(c, 1);
#endif
}

static inline int horizontal_add_s16x8(const int16x8_t a) {
#if AOM_ARCH_AARCH64
  return vaddlvq_s16(a);
#else
  const int32x4_t b = vpaddlq_s16(a);
  const int64x2_t c = vpaddlq_s32(b);
  const int32x2_t d = vadd_s32(vreinterpret_s32_s64(vget_low_s64(c)),
                               vreinterpret_s32_s64(vget_high_s64(c)));
  return vget_lane_s32(d, 0);
#endif
}

static inline int horizontal_add_s32x4(const int32x4_t a) {
#if AOM_ARCH_AARCH64
  return vaddvq_s32(a);
#else
  const int64x2_t b = vpaddlq_s32(a);
  const int32x2_t c = vadd_s32(vreinterpret_s32_s64(vget_low_s64(b)),
                               vreinterpret_s32_s64(vget_high_s64(b)));
  return vget_lane_s32(c, 0);
#endif
}

static inline int64_t horizontal_add_s64x2(const int64x2_t a) {
#if AOM_ARCH_AARCH64
  return vaddvq_s64(a);
#else
  return vgetq_lane_s64(a, 0) + vgetq_lane_s64(a, 1);
#endif
}

static inline uint64_t horizontal_add_u64x2(const uint64x2_t a) {
#if AOM_ARCH_AARCH64
  return vaddvq_u64(a);
#else
  return vgetq_lane_u64(a, 0) + vgetq_lane_u64(a, 1);
#endif
}

static inline uint64_t horizontal_long_add_u32x4(const uint32x4_t a) {
#if AOM_ARCH_AARCH64
  return vaddlvq_u32(a);
#else
  const uint64x2_t b = vpaddlq_u32(a);
  return vgetq_lane_u64(b, 0) + vgetq_lane_u64(b, 1);
#endif
}

static inline int64_t horizontal_long_add_s32x4(const int32x4_t a) {
#if AOM_ARCH_AARCH64
  return vaddlvq_s32(a);
#else
  const int64x2_t b = vpaddlq_s32(a);
  return vgetq_lane_s64(b, 0) + vgetq_lane_s64(b, 1);
#endif
}

static inline uint32_t horizontal_add_u32x4(const uint32x4_t a) {
#if AOM_ARCH_AARCH64
  return vaddvq_u32(a);
#else
  const uint64x2_t b = vpaddlq_u32(a);
  const uint32x2_t c = vadd_u32(vreinterpret_u32_u64(vget_low_u64(b)),
                                vreinterpret_u32_u64(vget_high_u64(b)));
  return vget_lane_u32(c, 0);
#endif
}

static inline uint32x4_t horizontal_add_4d_u32x4(const uint32x4_t sum[4]) {
#if AOM_ARCH_AARCH64
  uint32x4_t res01 = vpaddq_u32(sum[0], sum[1]);
  uint32x4_t res23 = vpaddq_u32(sum[2], sum[3]);
  return vpaddq_u32(res01, res23);
#else
  uint32x4_t res = vdupq_n_u32(0);
  res = vsetq_lane_u32(horizontal_add_u32x4(sum[0]), res, 0);
  res = vsetq_lane_u32(horizontal_add_u32x4(sum[1]), res, 1);
  res = vsetq_lane_u32(horizontal_add_u32x4(sum[2]), res, 2);
  res = vsetq_lane_u32(horizontal_add_u32x4(sum[3]), res, 3);
  return res;
#endif
}

static inline int32x4_t horizontal_add_4d_s32x4(const int32x4_t sum[4]) {
#if AOM_ARCH_AARCH64
  int32x4_t res01 = vpaddq_s32(sum[0], sum[1]);
  int32x4_t res23 = vpaddq_s32(sum[2], sum[3]);
  return vpaddq_s32(res01, res23);
#else
  int32x4_t res = vdupq_n_s32(0);
  res = vsetq_lane_s32(horizontal_add_s32x4(sum[0]), res, 0);
  res = vsetq_lane_s32(horizontal_add_s32x4(sum[1]), res, 1);
  res = vsetq_lane_s32(horizontal_add_s32x4(sum[2]), res, 2);
  res = vsetq_lane_s32(horizontal_add_s32x4(sum[3]), res, 3);
  return res;
#endif
}

static inline uint32_t horizontal_long_add_u16x8(const uint16x8_t vec_lo,
                                                 const uint16x8_t vec_hi) {
#if AOM_ARCH_AARCH64
  return vaddlvq_u16(vec_lo) + vaddlvq_u16(vec_hi);
#else
  const uint32x4_t vec_l_lo =
      vaddl_u16(vget_low_u16(vec_lo), vget_high_u16(vec_lo));
  const uint32x4_t vec_l_hi =
      vaddl_u16(vget_low_u16(vec_hi), vget_high_u16(vec_hi));
  const uint32x4_t a = vaddq_u32(vec_l_lo, vec_l_hi);
  const uint64x2_t b = vpaddlq_u32(a);
  const uint32x2_t c = vadd_u32(vreinterpret_u32_u64(vget_low_u64(b)),
                                vreinterpret_u32_u64(vget_high_u64(b)));
  return vget_lane_u32(c, 0);
#endif
}

static inline uint32x4_t horizontal_long_add_4d_u16x8(
    const uint16x8_t sum_lo[4], const uint16x8_t sum_hi[4]) {
  const uint32x4_t a0 = vpaddlq_u16(sum_lo[0]);
  const uint32x4_t a1 = vpaddlq_u16(sum_lo[1]);
  const uint32x4_t a2 = vpaddlq_u16(sum_lo[2]);
  const uint32x4_t a3 = vpaddlq_u16(sum_lo[3]);
  const uint32x4_t b0 = vpadalq_u16(a0, sum_hi[0]);
  const uint32x4_t b1 = vpadalq_u16(a1, sum_hi[1]);
  const uint32x4_t b2 = vpadalq_u16(a2, sum_hi[2]);
  const uint32x4_t b3 = vpadalq_u16(a3, sum_hi[3]);
#if AOM_ARCH_AARCH64
  const uint32x4_t c0 = vpaddq_u32(b0, b1);
  const uint32x4_t c1 = vpaddq_u32(b2, b3);
  return vpaddq_u32(c0, c1);
#else
  const uint32x2_t c0 = vadd_u32(vget_low_u32(b0), vget_high_u32(b0));
  const uint32x2_t c1 = vadd_u32(vget_low_u32(b1), vget_high_u32(b1));
  const uint32x2_t c2 = vadd_u32(vget_low_u32(b2), vget_high_u32(b2));
  const uint32x2_t c3 = vadd_u32(vget_low_u32(b3), vget_high_u32(b3));
  const uint32x2_t d0 = vpadd_u32(c0, c1);
  const uint32x2_t d1 = vpadd_u32(c2, c3);
  return vcombine_u32(d0, d1);
#endif
}

static inline uint32_t horizontal_add_u16x8(const uint16x8_t a) {
#if AOM_ARCH_AARCH64
  return vaddlvq_u16(a);
#else
  const uint32x4_t b = vpaddlq_u16(a);
  const uint64x2_t c = vpaddlq_u32(b);
  const uint32x2_t d = vadd_u32(vreinterpret_u32_u64(vget_low_u64(c)),
                                vreinterpret_u32_u64(vget_high_u64(c)));
  return vget_lane_u32(d, 0);
#endif
}

static inline uint32x4_t horizontal_add_4d_u16x8(const uint16x8_t sum[4]) {
#if AOM_ARCH_AARCH64
  const uint16x8_t a0 = vpaddq_u16(sum[0], sum[1]);
  const uint16x8_t a1 = vpaddq_u16(sum[2], sum[3]);
  const uint16x8_t b0 = vpaddq_u16(a0, a1);
  return vpaddlq_u16(b0);
#else
  const uint16x4_t a0 = vadd_u16(vget_low_u16(sum[0]), vget_high_u16(sum[0]));
  const uint16x4_t a1 = vadd_u16(vget_low_u16(sum[1]), vget_high_u16(sum[1]));
  const uint16x4_t a2 = vadd_u16(vget_low_u16(sum[2]), vget_high_u16(sum[2]));
  const uint16x4_t a3 = vadd_u16(vget_low_u16(sum[3]), vget_high_u16(sum[3]));
  const uint16x4_t b0 = vpadd_u16(a0, a1);
  const uint16x4_t b1 = vpadd_u16(a2, a3);
  return vpaddlq_u16(vcombine_u16(b0, b1));
#endif
}

static inline int32x4_t horizontal_add_4d_s16x8(const int16x8_t sum[4]) {
#if AOM_ARCH_AARCH64
  const int16x8_t a0 = vpaddq_s16(sum[0], sum[1]);
  const int16x8_t a1 = vpaddq_s16(sum[2], sum[3]);
  const int16x8_t b0 = vpaddq_s16(a0, a1);
  return vpaddlq_s16(b0);
#else
  const int16x4_t a0 = vadd_s16(vget_low_s16(sum[0]), vget_high_s16(sum[0]));
  const int16x4_t a1 = vadd_s16(vget_low_s16(sum[1]), vget_high_s16(sum[1]));
  const int16x4_t a2 = vadd_s16(vget_low_s16(sum[2]), vget_high_s16(sum[2]));
  const int16x4_t a3 = vadd_s16(vget_low_s16(sum[3]), vget_high_s16(sum[3]));
  const int16x4_t b0 = vpadd_s16(a0, a1);
  const int16x4_t b1 = vpadd_s16(a2, a3);
  return vpaddlq_s16(vcombine_s16(b0, b1));
#endif
}

static inline uint32_t horizontal_add_u32x2(const uint32x2_t a) {
#if AOM_ARCH_AARCH64
  return vaddv_u32(a);
#else
  const uint64x1_t b = vpaddl_u32(a);
  return vget_lane_u32(vreinterpret_u32_u64(b), 0);
#endif
}

static inline uint64_t horizontal_long_add_u32x2(const uint32x2_t a) {
#if AOM_ARCH_AARCH64
  return vaddlv_u32(a);
#else
  const uint64x1_t b = vpaddl_u32(a);
  return vget_lane_u64(b, 0);
#endif
}

static inline uint32_t horizontal_add_u16x4(const uint16x4_t a) {
#if AOM_ARCH_AARCH64
  return vaddlv_u16(a);
#else
  const uint32x2_t b = vpaddl_u16(a);
  const uint64x1_t c = vpaddl_u32(b);
  return vget_lane_u32(vreinterpret_u32_u64(c), 0);
#endif
}

static inline int32x4_t horizontal_add_2d_s32(int32x4_t a, int32x4_t b) {
#if AOM_ARCH_AARCH64
  return vpaddq_s32(a, b);
#else
  const int32x2_t a0 = vpadd_s32(vget_low_s32(a), vget_high_s32(a));
  const int32x2_t b0 = vpadd_s32(vget_low_s32(b), vget_high_s32(b));
  return vcombine_s32(a0, b0);
#endif
}

static inline int32x2_t add_pairwise_s32x4(int32x4_t a) {
#if AOM_ARCH_AARCH64
  return vget_low_s32(vpaddq_s32(a, a));
#else
  return vpadd_s32(vget_low_s32(a), vget_high_s32(a));
#endif
}

static inline uint64_t horizontal_long_add_u32x4_x2(const uint32x4_t a[2]) {
  return horizontal_long_add_u32x4(a[0]) + horizontal_long_add_u32x4(a[1]);
}

static inline uint64_t horizontal_long_add_u32x4_x4(const uint32x4_t a[4]) {
  uint64x2_t sum = vpaddlq_u32(a[0]);
  sum = vpadalq_u32(sum, a[1]);
  sum = vpadalq_u32(sum, a[2]);
  sum = vpadalq_u32(sum, a[3]);

  return horizontal_add_u64x2(sum);
}

static inline uint64_t horizontal_long_add_u32x4_x8(const uint32x4_t a[8]) {
  uint64x2_t sum[2];
  sum[0] = vpaddlq_u32(a[0]);
  sum[1] = vpaddlq_u32(a[1]);
  sum[0] = vpadalq_u32(sum[0], a[2]);
  sum[1] = vpadalq_u32(sum[1], a[3]);
  sum[0] = vpadalq_u32(sum[0], a[4]);
  sum[1] = vpadalq_u32(sum[1], a[5]);
  sum[0] = vpadalq_u32(sum[0], a[6]);
  sum[1] = vpadalq_u32(sum[1], a[7]);

  return horizontal_add_u64x2(vaddq_u64(sum[0], sum[1]));
}

static inline uint64_t horizontal_long_add_u32x4_x16(const uint32x4_t a[16]) {
  uint64x2_t sum[2];
  sum[0] = vpaddlq_u32(a[0]);
  sum[1] = vpaddlq_u32(a[1]);
  sum[0] = vpadalq_u32(sum[0], a[2]);
  sum[1] = vpadalq_u32(sum[1], a[3]);
  sum[0] = vpadalq_u32(sum[0], a[4]);
  sum[1] = vpadalq_u32(sum[1], a[5]);
  sum[0] = vpadalq_u32(sum[0], a[6]);
  sum[1] = vpadalq_u32(sum[1], a[7]);
  sum[0] = vpadalq_u32(sum[0], a[8]);
  sum[1] = vpadalq_u32(sum[1], a[9]);
  sum[0] = vpadalq_u32(sum[0], a[10]);
  sum[1] = vpadalq_u32(sum[1], a[11]);
  sum[0] = vpadalq_u32(sum[0], a[12]);
  sum[1] = vpadalq_u32(sum[1], a[13]);
  sum[0] = vpadalq_u32(sum[0], a[14]);
  sum[1] = vpadalq_u32(sum[1], a[15]);

  return horizontal_add_u64x2(vaddq_u64(sum[0], sum[1]));
}

#endif  // AOM_AOM_DSP_ARM_SUM_NEON_H_
