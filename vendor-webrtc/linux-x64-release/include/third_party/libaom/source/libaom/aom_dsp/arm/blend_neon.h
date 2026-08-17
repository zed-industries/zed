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

#ifndef AOM_AOM_DSP_ARM_BLEND_NEON_H_
#define AOM_AOM_DSP_ARM_BLEND_NEON_H_

#include <arm_neon.h>

#include "aom_dsp/blend.h"

static inline uint8x16_t alpha_blend_a64_u8x16(uint8x16_t m, uint8x16_t a,
                                               uint8x16_t b) {
  const uint8x16_t m_inv = vsubq_u8(vdupq_n_u8(AOM_BLEND_A64_MAX_ALPHA), m);

  uint16x8_t blend_u16_lo = vmull_u8(vget_low_u8(m), vget_low_u8(a));
  uint16x8_t blend_u16_hi = vmull_u8(vget_high_u8(m), vget_high_u8(a));

  blend_u16_lo = vmlal_u8(blend_u16_lo, vget_low_u8(m_inv), vget_low_u8(b));
  blend_u16_hi = vmlal_u8(blend_u16_hi, vget_high_u8(m_inv), vget_high_u8(b));

  uint8x8_t blend_u8_lo = vrshrn_n_u16(blend_u16_lo, AOM_BLEND_A64_ROUND_BITS);
  uint8x8_t blend_u8_hi = vrshrn_n_u16(blend_u16_hi, AOM_BLEND_A64_ROUND_BITS);

  return vcombine_u8(blend_u8_lo, blend_u8_hi);
}

static inline uint8x8_t alpha_blend_a64_u8x8(uint8x8_t m, uint8x8_t a,
                                             uint8x8_t b) {
  const uint8x8_t m_inv = vsub_u8(vdup_n_u8(AOM_BLEND_A64_MAX_ALPHA), m);

  uint16x8_t blend_u16 = vmull_u8(m, a);

  blend_u16 = vmlal_u8(blend_u16, m_inv, b);

  return vrshrn_n_u16(blend_u16, AOM_BLEND_A64_ROUND_BITS);
}

#if CONFIG_AV1_HIGHBITDEPTH
static inline uint16x8_t alpha_blend_a64_u16x8(uint16x8_t m, uint16x8_t a,
                                               uint16x8_t b) {
  uint16x8_t m_inv = vsubq_u16(vdupq_n_u16(AOM_BLEND_A64_MAX_ALPHA), m);

  uint32x4_t blend_u32_lo = vmull_u16(vget_low_u16(a), vget_low_u16(m));
  uint32x4_t blend_u32_hi = vmull_u16(vget_high_u16(a), vget_high_u16(m));

  blend_u32_lo = vmlal_u16(blend_u32_lo, vget_low_u16(b), vget_low_u16(m_inv));
  blend_u32_hi =
      vmlal_u16(blend_u32_hi, vget_high_u16(b), vget_high_u16(m_inv));

  uint16x4_t blend_u16_lo =
      vrshrn_n_u32(blend_u32_lo, AOM_BLEND_A64_ROUND_BITS);
  uint16x4_t blend_u16_hi =
      vrshrn_n_u32(blend_u32_hi, AOM_BLEND_A64_ROUND_BITS);

  return vcombine_u16(blend_u16_lo, blend_u16_hi);
}

static inline uint16x4_t alpha_blend_a64_u16x4(uint16x4_t m, uint16x4_t a,
                                               uint16x4_t b) {
  const uint16x4_t m_inv = vsub_u16(vdup_n_u16(AOM_BLEND_A64_MAX_ALPHA), m);

  uint32x4_t blend_u16 = vmull_u16(m, a);

  blend_u16 = vmlal_u16(blend_u16, m_inv, b);

  return vrshrn_n_u32(blend_u16, AOM_BLEND_A64_ROUND_BITS);
}
#endif  // CONFIG_AV1_HIGHBITDEPTH

static inline uint8x8_t avg_blend_u8x8(uint8x8_t a, uint8x8_t b) {
  return vrhadd_u8(a, b);
}

static inline uint8x16_t avg_blend_u8x16(uint8x16_t a, uint8x16_t b) {
  return vrhaddq_u8(a, b);
}

static inline uint8x8_t avg_blend_pairwise_u8x8(uint8x8_t a, uint8x8_t b) {
  return vrshr_n_u8(vpadd_u8(a, b), 1);
}

static inline uint8x16_t avg_blend_pairwise_u8x16(uint8x16_t a, uint8x16_t b) {
#if AOM_ARCH_AARCH64
  return vrshrq_n_u8(vpaddq_u8(a, b), 1);
#else
  uint8x8_t sum_pairwise_a = vpadd_u8(vget_low_u8(a), vget_high_u8(a));
  uint8x8_t sum_pairwise_b = vpadd_u8(vget_low_u8(b), vget_high_u8(b));
  return vrshrq_n_u8(vcombine_u8(sum_pairwise_a, sum_pairwise_b), 1);
#endif  // AOM_ARCH_AARCH64
}

static inline uint8x8_t avg_blend_pairwise_u8x8_4(uint8x8_t a, uint8x8_t b,
                                                  uint8x8_t c, uint8x8_t d) {
  uint8x8_t a_c = vpadd_u8(a, c);
  uint8x8_t b_d = vpadd_u8(b, d);
  return vrshr_n_u8(vqadd_u8(a_c, b_d), 2);
}

static inline uint8x16_t avg_blend_pairwise_u8x16_4(uint8x16_t a, uint8x16_t b,
                                                    uint8x16_t c,
                                                    uint8x16_t d) {
#if AOM_ARCH_AARCH64
  uint8x16_t a_c = vpaddq_u8(a, c);
  uint8x16_t b_d = vpaddq_u8(b, d);
  return vrshrq_n_u8(vqaddq_u8(a_c, b_d), 2);
#else
  uint8x8_t sum_pairwise_a = vpadd_u8(vget_low_u8(a), vget_high_u8(a));
  uint8x8_t sum_pairwise_b = vpadd_u8(vget_low_u8(b), vget_high_u8(b));
  uint8x8_t sum_pairwise_c = vpadd_u8(vget_low_u8(c), vget_high_u8(c));
  uint8x8_t sum_pairwise_d = vpadd_u8(vget_low_u8(d), vget_high_u8(d));
  uint8x16_t a_c = vcombine_u8(sum_pairwise_a, sum_pairwise_c);
  uint8x16_t b_d = vcombine_u8(sum_pairwise_b, sum_pairwise_d);
  return vrshrq_n_u8(vqaddq_u8(a_c, b_d), 2);
#endif  // AOM_ARCH_AARCH64
}

#endif  // AOM_AOM_DSP_ARM_BLEND_NEON_H_
