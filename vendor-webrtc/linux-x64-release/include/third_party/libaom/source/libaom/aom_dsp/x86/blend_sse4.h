/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_X86_BLEND_SSE4_H_
#define AOM_AOM_DSP_X86_BLEND_SSE4_H_

#include "aom_dsp/blend.h"
#include "aom_dsp/x86/synonyms.h"
static const uint8_t g_blend_a64_mask_shuffle[32] = {
  0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15,
  0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15,
};

//////////////////////////////////////////////////////////////////////////////
// Common kernels
//////////////////////////////////////////////////////////////////////////////

static inline __m128i blend_4(const uint8_t *src0, const uint8_t *src1,
                              const __m128i *v_m0_w, const __m128i *v_m1_w) {
  const __m128i v_s0_b = xx_loadl_32(src0);
  const __m128i v_s1_b = xx_loadl_32(src1);
  const __m128i v_s0_w = _mm_cvtepu8_epi16(v_s0_b);
  const __m128i v_s1_w = _mm_cvtepu8_epi16(v_s1_b);

  const __m128i v_p0_w = _mm_mullo_epi16(v_s0_w, *v_m0_w);
  const __m128i v_p1_w = _mm_mullo_epi16(v_s1_w, *v_m1_w);
  const __m128i v_sum_w = _mm_add_epi16(v_p0_w, v_p1_w);
  const __m128i v_res_w = xx_roundn_epu16(v_sum_w, AOM_BLEND_A64_ROUND_BITS);

  return v_res_w;
}

static inline __m128i blend_8(const uint8_t *src0, const uint8_t *src1,
                              const __m128i *v_m0_w, const __m128i *v_m1_w) {
  const __m128i v_s0_b = xx_loadl_64(src0);
  const __m128i v_s1_b = xx_loadl_64(src1);
  const __m128i v_s0_w = _mm_cvtepu8_epi16(v_s0_b);
  const __m128i v_s1_w = _mm_cvtepu8_epi16(v_s1_b);

  const __m128i v_p0_w = _mm_mullo_epi16(v_s0_w, *v_m0_w);
  const __m128i v_p1_w = _mm_mullo_epi16(v_s1_w, *v_m1_w);

  const __m128i v_sum_w = _mm_add_epi16(v_p0_w, v_p1_w);

  const __m128i v_res_w = xx_roundn_epu16(v_sum_w, AOM_BLEND_A64_ROUND_BITS);

  return v_res_w;
}

static inline __m128i blend_4_u8(const uint8_t *src0, const uint8_t *src1,
                                 const __m128i *v_m0_b, const __m128i *v_m1_b,
                                 const __m128i *rounding) {
  const __m128i v_s0_b = xx_loadl_32(src0);
  const __m128i v_s1_b = xx_loadl_32(src1);

  const __m128i v_p0_w = _mm_maddubs_epi16(_mm_unpacklo_epi8(v_s0_b, v_s1_b),
                                           _mm_unpacklo_epi8(*v_m0_b, *v_m1_b));

  const __m128i v_res_w = _mm_mulhrs_epi16(v_p0_w, *rounding);
  const __m128i v_res = _mm_packus_epi16(v_res_w, v_res_w);
  return v_res;
}

static inline __m128i blend_8_u8(const uint8_t *src0, const uint8_t *src1,
                                 const __m128i *v_m0_b, const __m128i *v_m1_b,
                                 const __m128i *rounding) {
  const __m128i v_s0_b = xx_loadl_64(src0);
  const __m128i v_s1_b = xx_loadl_64(src1);

  const __m128i v_p0_w = _mm_maddubs_epi16(_mm_unpacklo_epi8(v_s0_b, v_s1_b),
                                           _mm_unpacklo_epi8(*v_m0_b, *v_m1_b));

  const __m128i v_res_w = _mm_mulhrs_epi16(v_p0_w, *rounding);
  const __m128i v_res = _mm_packus_epi16(v_res_w, v_res_w);
  return v_res;
}

static inline __m128i blend_16_u8(const uint8_t *src0, const uint8_t *src1,
                                  const __m128i *v_m0_b, const __m128i *v_m1_b,
                                  const __m128i *rounding) {
  const __m128i v_s0_b = xx_loadu_128(src0);
  const __m128i v_s1_b = xx_loadu_128(src1);

  const __m128i v_p0_w = _mm_maddubs_epi16(_mm_unpacklo_epi8(v_s0_b, v_s1_b),
                                           _mm_unpacklo_epi8(*v_m0_b, *v_m1_b));
  const __m128i v_p1_w = _mm_maddubs_epi16(_mm_unpackhi_epi8(v_s0_b, v_s1_b),
                                           _mm_unpackhi_epi8(*v_m0_b, *v_m1_b));

  const __m128i v_res0_w = _mm_mulhrs_epi16(v_p0_w, *rounding);
  const __m128i v_res1_w = _mm_mulhrs_epi16(v_p1_w, *rounding);
  const __m128i v_res = _mm_packus_epi16(v_res0_w, v_res1_w);
  return v_res;
}

typedef __m128i (*blend_unit_fn)(const uint16_t *src0, const uint16_t *src1,
                                 const __m128i v_m0_w, const __m128i v_m1_w);

static inline __m128i blend_4_b10(const uint16_t *src0, const uint16_t *src1,
                                  const __m128i v_m0_w, const __m128i v_m1_w) {
  const __m128i v_s0_w = xx_loadl_64(src0);
  const __m128i v_s1_w = xx_loadl_64(src1);

  const __m128i v_p0_w = _mm_mullo_epi16(v_s0_w, v_m0_w);
  const __m128i v_p1_w = _mm_mullo_epi16(v_s1_w, v_m1_w);

  const __m128i v_sum_w = _mm_add_epi16(v_p0_w, v_p1_w);

  const __m128i v_res_w = xx_roundn_epu16(v_sum_w, AOM_BLEND_A64_ROUND_BITS);

  return v_res_w;
}

static inline __m128i blend_8_b10(const uint16_t *src0, const uint16_t *src1,
                                  const __m128i v_m0_w, const __m128i v_m1_w) {
  const __m128i v_s0_w = xx_loadu_128(src0);
  const __m128i v_s1_w = xx_loadu_128(src1);

  const __m128i v_p0_w = _mm_mullo_epi16(v_s0_w, v_m0_w);
  const __m128i v_p1_w = _mm_mullo_epi16(v_s1_w, v_m1_w);

  const __m128i v_sum_w = _mm_add_epi16(v_p0_w, v_p1_w);

  const __m128i v_res_w = xx_roundn_epu16(v_sum_w, AOM_BLEND_A64_ROUND_BITS);

  return v_res_w;
}

static inline __m128i blend_4_b12(const uint16_t *src0, const uint16_t *src1,
                                  const __m128i v_m0_w, const __m128i v_m1_w) {
  const __m128i v_s0_w = xx_loadl_64(src0);
  const __m128i v_s1_w = xx_loadl_64(src1);

  // Interleave
  const __m128i v_m01_w = _mm_unpacklo_epi16(v_m0_w, v_m1_w);
  const __m128i v_s01_w = _mm_unpacklo_epi16(v_s0_w, v_s1_w);

  // Multiply-Add
  const __m128i v_sum_d = _mm_madd_epi16(v_s01_w, v_m01_w);

  // Scale
  const __m128i v_ssum_d =
      _mm_srli_epi32(v_sum_d, AOM_BLEND_A64_ROUND_BITS - 1);

  // Pack
  const __m128i v_pssum_d = _mm_packs_epi32(v_ssum_d, v_ssum_d);

  // Round
  const __m128i v_res_w = xx_round_epu16(v_pssum_d);

  return v_res_w;
}

static inline __m128i blend_8_b12(const uint16_t *src0, const uint16_t *src1,
                                  const __m128i v_m0_w, const __m128i v_m1_w) {
  const __m128i v_s0_w = xx_loadu_128(src0);
  const __m128i v_s1_w = xx_loadu_128(src1);

  // Interleave
  const __m128i v_m01l_w = _mm_unpacklo_epi16(v_m0_w, v_m1_w);
  const __m128i v_m01h_w = _mm_unpackhi_epi16(v_m0_w, v_m1_w);
  const __m128i v_s01l_w = _mm_unpacklo_epi16(v_s0_w, v_s1_w);
  const __m128i v_s01h_w = _mm_unpackhi_epi16(v_s0_w, v_s1_w);

  // Multiply-Add
  const __m128i v_suml_d = _mm_madd_epi16(v_s01l_w, v_m01l_w);
  const __m128i v_sumh_d = _mm_madd_epi16(v_s01h_w, v_m01h_w);

  // Scale
  const __m128i v_ssuml_d =
      _mm_srli_epi32(v_suml_d, AOM_BLEND_A64_ROUND_BITS - 1);
  const __m128i v_ssumh_d =
      _mm_srli_epi32(v_sumh_d, AOM_BLEND_A64_ROUND_BITS - 1);

  // Pack
  const __m128i v_pssum_d = _mm_packs_epi32(v_ssuml_d, v_ssumh_d);

  // Round
  const __m128i v_res_w = xx_round_epu16(v_pssum_d);

  return v_res_w;
}

#endif  // AOM_AOM_DSP_X86_BLEND_SSE4_H_
