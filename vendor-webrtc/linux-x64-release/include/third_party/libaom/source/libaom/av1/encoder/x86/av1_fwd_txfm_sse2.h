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
#ifndef AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_SSE2_H_
#define AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_SSE2_H_

#include <immintrin.h>

#include "config/aom_config.h"
#include "config/av1_rtcd.h"

#include "aom/aom_integer.h"
#include "aom_dsp/x86/transpose_sse2.h"
#include "aom_dsp/x86/txfm_common_sse2.h"

#ifdef __cplusplus
extern "C" {
#endif

void av1_fdct8x32_new_sse2(const __m128i *input, __m128i *output,
                           int8_t cos_bit);
void av1_fdct8x64_new_sse2(const __m128i *input, __m128i *output,
                           int8_t cos_bit);

static inline void fidentity4x4_new_sse2(const __m128i *const input,
                                         __m128i *const output,
                                         const int8_t cos_bit) {
  (void)cos_bit;
  const __m128i one = _mm_set1_epi16(1);

  for (int i = 0; i < 4; ++i) {
    const __m128i a = _mm_unpacklo_epi16(input[i], one);
    const __m128i b = scale_round_sse2(a, NewSqrt2);
    output[i] = _mm_packs_epi32(b, b);
  }
}

static inline void fidentity8x4_new_sse2(const __m128i *const input,
                                         __m128i *const output,
                                         const int8_t cos_bit) {
  (void)cos_bit;
  const __m128i one = _mm_set1_epi16(1);

  for (int i = 0; i < 4; ++i) {
    const __m128i a_lo = _mm_unpacklo_epi16(input[i], one);
    const __m128i a_hi = _mm_unpackhi_epi16(input[i], one);
    const __m128i b_lo = scale_round_sse2(a_lo, NewSqrt2);
    const __m128i b_hi = scale_round_sse2(a_hi, NewSqrt2);
    output[i] = _mm_packs_epi32(b_lo, b_hi);
  }
}

static inline void fidentity8x8_new_sse2(const __m128i *input, __m128i *output,
                                         int8_t cos_bit) {
  (void)cos_bit;

  output[0] = _mm_adds_epi16(input[0], input[0]);
  output[1] = _mm_adds_epi16(input[1], input[1]);
  output[2] = _mm_adds_epi16(input[2], input[2]);
  output[3] = _mm_adds_epi16(input[3], input[3]);
  output[4] = _mm_adds_epi16(input[4], input[4]);
  output[5] = _mm_adds_epi16(input[5], input[5]);
  output[6] = _mm_adds_epi16(input[6], input[6]);
  output[7] = _mm_adds_epi16(input[7], input[7]);
}

static inline void fdct8x8_new_sse2(const __m128i *input, __m128i *output,
                                    int8_t cos_bit) {
  const int32_t *cospi = cospi_arr(cos_bit);
  const __m128i __rounding = _mm_set1_epi32(1 << (cos_bit - 1));

  const __m128i cospi_m32_p32 = pair_set_epi16(-cospi[32], cospi[32]);
  const __m128i cospi_p32_p32 = pair_set_epi16(cospi[32], cospi[32]);
  const __m128i cospi_p32_m32 = pair_set_epi16(cospi[32], -cospi[32]);
  const __m128i cospi_p48_p16 = pair_set_epi16(cospi[48], cospi[16]);
  const __m128i cospi_m16_p48 = pair_set_epi16(-cospi[16], cospi[48]);
  const __m128i cospi_p56_p08 = pair_set_epi16(cospi[56], cospi[8]);
  const __m128i cospi_m08_p56 = pair_set_epi16(-cospi[8], cospi[56]);
  const __m128i cospi_p24_p40 = pair_set_epi16(cospi[24], cospi[40]);
  const __m128i cospi_m40_p24 = pair_set_epi16(-cospi[40], cospi[24]);

  // stage 1
  __m128i x1[8];
  x1[0] = _mm_adds_epi16(input[0], input[7]);
  x1[7] = _mm_subs_epi16(input[0], input[7]);
  x1[1] = _mm_adds_epi16(input[1], input[6]);
  x1[6] = _mm_subs_epi16(input[1], input[6]);
  x1[2] = _mm_adds_epi16(input[2], input[5]);
  x1[5] = _mm_subs_epi16(input[2], input[5]);
  x1[3] = _mm_adds_epi16(input[3], input[4]);
  x1[4] = _mm_subs_epi16(input[3], input[4]);

  // stage 2
  __m128i x2[8];
  x2[0] = _mm_adds_epi16(x1[0], x1[3]);
  x2[3] = _mm_subs_epi16(x1[0], x1[3]);
  x2[1] = _mm_adds_epi16(x1[1], x1[2]);
  x2[2] = _mm_subs_epi16(x1[1], x1[2]);
  x2[4] = x1[4];
  btf_16_sse2(cospi_m32_p32, cospi_p32_p32, x1[5], x1[6], x2[5], x2[6]);
  x2[7] = x1[7];

  // stage 3
  __m128i x3[8];
  btf_16_sse2(cospi_p32_p32, cospi_p32_m32, x2[0], x2[1], x3[0], x3[1]);
  btf_16_sse2(cospi_p48_p16, cospi_m16_p48, x2[2], x2[3], x3[2], x3[3]);
  x3[4] = _mm_adds_epi16(x2[4], x2[5]);
  x3[5] = _mm_subs_epi16(x2[4], x2[5]);
  x3[6] = _mm_subs_epi16(x2[7], x2[6]);
  x3[7] = _mm_adds_epi16(x2[7], x2[6]);

  // stage 4 and 5
  output[0] = x3[0];
  output[4] = x3[1];
  output[2] = x3[2];
  output[6] = x3[3];
  btf_16_sse2(cospi_p56_p08, cospi_m08_p56, x3[4], x3[7], output[1], output[7]);
  btf_16_sse2(cospi_p24_p40, cospi_m40_p24, x3[5], x3[6], output[5], output[3]);
}

static inline void fadst8x8_new_sse2(const __m128i *input, __m128i *output,
                                     int8_t cos_bit) {
  const int32_t *cospi = cospi_arr(cos_bit);
  const __m128i __zero = _mm_setzero_si128();
  const __m128i __rounding = _mm_set1_epi32(1 << (cos_bit - 1));

  const __m128i cospi_p32_p32 = pair_set_epi16(cospi[32], cospi[32]);
  const __m128i cospi_p32_m32 = pair_set_epi16(cospi[32], -cospi[32]);
  const __m128i cospi_p16_p48 = pair_set_epi16(cospi[16], cospi[48]);
  const __m128i cospi_p48_m16 = pair_set_epi16(cospi[48], -cospi[16]);
  const __m128i cospi_m48_p16 = pair_set_epi16(-cospi[48], cospi[16]);
  const __m128i cospi_p04_p60 = pair_set_epi16(cospi[4], cospi[60]);
  const __m128i cospi_p60_m04 = pair_set_epi16(cospi[60], -cospi[4]);
  const __m128i cospi_p20_p44 = pair_set_epi16(cospi[20], cospi[44]);
  const __m128i cospi_p44_m20 = pair_set_epi16(cospi[44], -cospi[20]);
  const __m128i cospi_p36_p28 = pair_set_epi16(cospi[36], cospi[28]);
  const __m128i cospi_p28_m36 = pair_set_epi16(cospi[28], -cospi[36]);
  const __m128i cospi_p52_p12 = pair_set_epi16(cospi[52], cospi[12]);
  const __m128i cospi_p12_m52 = pair_set_epi16(cospi[12], -cospi[52]);

  // stage 1
  __m128i x1[8];
  x1[0] = input[0];
  x1[1] = _mm_subs_epi16(__zero, input[7]);
  x1[2] = _mm_subs_epi16(__zero, input[3]);
  x1[3] = input[4];
  x1[4] = _mm_subs_epi16(__zero, input[1]);
  x1[5] = input[6];
  x1[6] = input[2];
  x1[7] = _mm_subs_epi16(__zero, input[5]);

  // stage 2
  __m128i x2[8];
  x2[0] = x1[0];
  x2[1] = x1[1];
  btf_16_sse2(cospi_p32_p32, cospi_p32_m32, x1[2], x1[3], x2[2], x2[3]);
  x2[4] = x1[4];
  x2[5] = x1[5];
  btf_16_sse2(cospi_p32_p32, cospi_p32_m32, x1[6], x1[7], x2[6], x2[7]);

  // stage 3
  __m128i x3[8];
  x3[0] = _mm_adds_epi16(x2[0], x2[2]);
  x3[2] = _mm_subs_epi16(x2[0], x2[2]);
  x3[1] = _mm_adds_epi16(x2[1], x2[3]);
  x3[3] = _mm_subs_epi16(x2[1], x2[3]);
  x3[4] = _mm_adds_epi16(x2[4], x2[6]);
  x3[6] = _mm_subs_epi16(x2[4], x2[6]);
  x3[5] = _mm_adds_epi16(x2[5], x2[7]);
  x3[7] = _mm_subs_epi16(x2[5], x2[7]);

  // stage 4
  __m128i x4[8];
  x4[0] = x3[0];
  x4[1] = x3[1];
  x4[2] = x3[2];
  x4[3] = x3[3];
  btf_16_sse2(cospi_p16_p48, cospi_p48_m16, x3[4], x3[5], x4[4], x4[5]);
  btf_16_sse2(cospi_m48_p16, cospi_p16_p48, x3[6], x3[7], x4[6], x4[7]);

  // stage 5, 6 and 7
  output[7] = _mm_adds_epi16(x4[0], x4[4]);
  output[3] = _mm_subs_epi16(x4[0], x4[4]);
  output[0] = _mm_adds_epi16(x4[1], x4[5]);
  output[4] = _mm_subs_epi16(x4[1], x4[5]);
  output[5] = _mm_adds_epi16(x4[2], x4[6]);
  output[1] = _mm_subs_epi16(x4[2], x4[6]);
  output[2] = _mm_adds_epi16(x4[3], x4[7]);
  output[6] = _mm_subs_epi16(x4[3], x4[7]);

  btf_16_sse2(cospi_p04_p60, cospi_p60_m04, output[7], output[0], output[7],
              output[0]);
  btf_16_sse2(cospi_p20_p44, cospi_p44_m20, output[5], output[2], output[5],
              output[2]);
  btf_16_sse2(cospi_p36_p28, cospi_p28_m36, output[3], output[4], output[3],
              output[4]);
  btf_16_sse2(cospi_p52_p12, cospi_p12_m52, output[1], output[6], output[1],
              output[6]);
}

static inline void fidentity8x16_new_sse2(const __m128i *input, __m128i *output,
                                          int8_t cos_bit) {
  (void)cos_bit;
  const __m128i one = _mm_set1_epi16(1);

  for (int i = 0; i < 16; ++i) {
    const __m128i a_lo = _mm_unpacklo_epi16(input[i], one);
    const __m128i a_hi = _mm_unpackhi_epi16(input[i], one);
    const __m128i b_lo = scale_round_sse2(a_lo, 2 * NewSqrt2);
    const __m128i b_hi = scale_round_sse2(a_hi, 2 * NewSqrt2);
    output[i] = _mm_packs_epi32(b_lo, b_hi);
  }
}

static inline void fidentity8x32_new_sse2(const __m128i *input, __m128i *output,
                                          int8_t cos_bit) {
  (void)cos_bit;
  for (int i = 0; i < 32; ++i) {
    output[i] = _mm_slli_epi16(input[i], 2);
  }
}

static const transform_1d_sse2 col_txfm8x32_arr[TX_TYPES] = {
  av1_fdct8x32_new_sse2,   // DCT_DCT
  NULL,                    // ADST_DCT
  NULL,                    // DCT_ADST
  NULL,                    // ADST_ADST
  NULL,                    // FLIPADST_DCT
  NULL,                    // DCT_FLIPADST
  NULL,                    // FLIPADST_FLIPADST
  NULL,                    // ADST_FLIPADST
  NULL,                    // FLIPADST_ADST
  fidentity8x32_new_sse2,  // IDTX
  av1_fdct8x32_new_sse2,   // V_DCT
  fidentity8x32_new_sse2,  // H_DCT
  NULL,                    // V_ADST
  NULL,                    // H_ADST
  NULL,                    // V_FLIPADST
  NULL                     // H_FLIPADST
};

#ifdef __cplusplus
}
#endif

#endif  // AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_SSE2_H_
