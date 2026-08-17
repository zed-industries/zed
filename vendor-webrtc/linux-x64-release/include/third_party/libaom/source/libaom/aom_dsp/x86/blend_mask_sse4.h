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

#ifndef AOM_AOM_DSP_X86_BLEND_MASK_SSE4_H_
#define AOM_AOM_DSP_X86_BLEND_MASK_SSE4_H_
#include <smmintrin.h>  // SSE4.1

#include <assert.h>

#include "aom/aom_integer.h"
#include "aom_ports/mem.h"
#include "aom_dsp/aom_dsp_common.h"
#include "aom_dsp/blend.h"

#include "aom_dsp/x86/synonyms.h"

#include "config/aom_dsp_rtcd.h"

static inline void blend_a64_d16_mask_w4_sse41(
    uint8_t *dst, const CONV_BUF_TYPE *src0, const CONV_BUF_TYPE *src1,
    const __m128i *m, const __m128i *v_round_offset, const __m128i *v_maxval,
    int shift) {
  const __m128i max_minus_m = _mm_sub_epi16(*v_maxval, *m);
  const __m128i s0 = xx_loadl_64(src0);
  const __m128i s1 = xx_loadl_64(src1);
  const __m128i s0_s1 = _mm_unpacklo_epi16(s0, s1);
  const __m128i m_max_minus_m = _mm_unpacklo_epi16(*m, max_minus_m);
  const __m128i res_a = _mm_madd_epi16(s0_s1, m_max_minus_m);
  const __m128i res_c = _mm_sub_epi32(res_a, *v_round_offset);
  const __m128i res_d = _mm_srai_epi32(res_c, shift);
  const __m128i res_e = _mm_packs_epi32(res_d, res_d);
  const __m128i res = _mm_packus_epi16(res_e, res_e);

  xx_storel_32(dst, res);
}

static inline void blend_a64_d16_mask_w8_sse41(
    uint8_t *dst, const CONV_BUF_TYPE *src0, const CONV_BUF_TYPE *src1,
    const __m128i *m, const __m128i *v_round_offset, const __m128i *v_maxval,
    int shift) {
  const __m128i max_minus_m = _mm_sub_epi16(*v_maxval, *m);
  const __m128i s0 = xx_loadu_128(src0);
  const __m128i s1 = xx_loadu_128(src1);
  __m128i res_lo = _mm_madd_epi16(_mm_unpacklo_epi16(s0, s1),
                                  _mm_unpacklo_epi16(*m, max_minus_m));
  __m128i res_hi = _mm_madd_epi16(_mm_unpackhi_epi16(s0, s1),
                                  _mm_unpackhi_epi16(*m, max_minus_m));
  res_lo = _mm_srai_epi32(_mm_sub_epi32(res_lo, *v_round_offset), shift);
  res_hi = _mm_srai_epi32(_mm_sub_epi32(res_hi, *v_round_offset), shift);
  const __m128i res_e = _mm_packs_epi32(res_lo, res_hi);
  const __m128i res = _mm_packus_epi16(res_e, res_e);

  _mm_storel_epi64((__m128i *)(dst), res);
}

static inline void aom_lowbd_blend_a64_d16_mask_subw0_subh0_w4_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  for (int i = 0; i < h; ++i) {
    const __m128i m0 = xx_loadl_32(mask);
    const __m128i m = _mm_cvtepu8_epi16(m0);

    blend_a64_d16_mask_w4_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}

static inline void aom_lowbd_blend_a64_d16_mask_subw0_subh0_w8_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  for (int i = 0; i < h; ++i) {
    const __m128i m0 = xx_loadl_64(mask);
    const __m128i m = _mm_cvtepu8_epi16(m0);
    blend_a64_d16_mask_w8_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}

static inline void aom_lowbd_blend_a64_d16_mask_subw1_subh1_w4_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i one_b = _mm_set1_epi8(1);
  const __m128i two_w = _mm_set1_epi16(2);
  for (int i = 0; i < h; ++i) {
    const __m128i m_i0 = xx_loadl_64(mask);
    const __m128i m_i1 = xx_loadl_64(mask + mask_stride);
    const __m128i m_ac = _mm_adds_epu8(m_i0, m_i1);
    const __m128i m_acbd = _mm_maddubs_epi16(m_ac, one_b);
    const __m128i m_acbd_2 = _mm_add_epi16(m_acbd, two_w);
    const __m128i m = _mm_srli_epi16(m_acbd_2, 2);

    blend_a64_d16_mask_w4_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride << 1;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}

static inline void aom_lowbd_blend_a64_d16_mask_subw1_subh1_w8_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i one_b = _mm_set1_epi8(1);
  const __m128i two_w = _mm_set1_epi16(2);
  for (int i = 0; i < h; ++i) {
    const __m128i m_i0 = xx_loadu_128(mask);
    const __m128i m_i1 = xx_loadu_128(mask + mask_stride);
    const __m128i m_ac = _mm_adds_epu8(m_i0, m_i1);
    const __m128i m_acbd = _mm_maddubs_epi16(m_ac, one_b);
    const __m128i m_acbd_2 = _mm_add_epi16(m_acbd, two_w);
    const __m128i m = _mm_srli_epi16(m_acbd_2, 2);

    blend_a64_d16_mask_w8_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride << 1;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}

static inline void aom_lowbd_blend_a64_d16_mask_subw1_subh0_w4_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i one_b = _mm_set1_epi8(1);
  const __m128i zeros = _mm_setzero_si128();
  for (int i = 0; i < h; ++i) {
    const __m128i m_i0 = xx_loadl_64(mask);
    const __m128i m_ac = _mm_maddubs_epi16(m_i0, one_b);
    const __m128i m = _mm_avg_epu16(m_ac, zeros);

    blend_a64_d16_mask_w4_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}

static inline void aom_lowbd_blend_a64_d16_mask_subw1_subh0_w8_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i one_b = _mm_set1_epi8(1);
  const __m128i zeros = _mm_setzero_si128();
  for (int i = 0; i < h; ++i) {
    const __m128i m_i0 = xx_loadu_128(mask);
    const __m128i m_ac = _mm_maddubs_epi16(m_i0, one_b);
    const __m128i m = _mm_avg_epu16(m_ac, zeros);

    blend_a64_d16_mask_w8_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}
static inline void aom_lowbd_blend_a64_d16_mask_subw0_subh1_w4_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i zeros = _mm_setzero_si128();
  for (int i = 0; i < h; ++i) {
    const __m128i m_i0 = xx_loadl_64(mask);
    const __m128i m_i1 = xx_loadl_64(mask + mask_stride);
    const __m128i m_ac = _mm_adds_epu8(m_i0, m_i1);
    const __m128i m = _mm_cvtepu8_epi16(_mm_avg_epu8(m_ac, zeros));

    blend_a64_d16_mask_w4_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride << 1;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}

static inline void aom_lowbd_blend_a64_d16_mask_subw0_subh1_w8_sse4_1(
    uint8_t *dst, uint32_t dst_stride, const CONV_BUF_TYPE *src0,
    uint32_t src0_stride, const CONV_BUF_TYPE *src1, uint32_t src1_stride,
    const uint8_t *mask, uint32_t mask_stride, int h,
    const __m128i *round_offset, int shift) {
  const __m128i v_maxval = _mm_set1_epi16(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i zeros = _mm_setzero_si128();
  for (int i = 0; i < h; ++i) {
    const __m128i m_i0 = xx_loadl_64(mask);
    const __m128i m_i1 = xx_loadl_64(mask + mask_stride);
    const __m128i m_ac = _mm_adds_epu8(m_i0, m_i1);
    const __m128i m = _mm_cvtepu8_epi16(_mm_avg_epu8(m_ac, zeros));

    blend_a64_d16_mask_w8_sse41(dst, src0, src1, &m, round_offset, &v_maxval,
                                shift);
    mask += mask_stride << 1;
    dst += dst_stride;
    src0 += src0_stride;
    src1 += src1_stride;
  }
}
#endif  // AOM_AOM_DSP_X86_BLEND_MASK_SSE4_H_
