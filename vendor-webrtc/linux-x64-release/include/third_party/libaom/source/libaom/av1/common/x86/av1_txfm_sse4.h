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

#ifndef AOM_AV1_COMMON_X86_AV1_TXFM_SSE4_H_
#define AOM_AV1_COMMON_X86_AV1_TXFM_SSE4_H_

#include <smmintrin.h>

#ifdef __cplusplus
extern "C" {
#endif

static inline __m128i av1_round_shift_32_sse4_1(__m128i vec, int bit) {
  __m128i tmp, round;
  round = _mm_set1_epi32(1 << (bit - 1));
  tmp = _mm_add_epi32(vec, round);
  return _mm_srai_epi32(tmp, bit);
}

static inline void av1_round_shift_array_32_sse4_1(const __m128i *input,
                                                   __m128i *output,
                                                   const int size,
                                                   const int bit) {
  if (bit > 0) {
    int i;
    for (i = 0; i < size; i++) {
      output[i] = av1_round_shift_32_sse4_1(input[i], bit);
    }
  } else {
    int i;
    for (i = 0; i < size; i++) {
      output[i] = _mm_slli_epi32(input[i], -bit);
    }
  }
}

static inline void av1_round_shift_rect_array_32_sse4_1(const __m128i *input,
                                                        __m128i *output,
                                                        const int size,
                                                        const int bit,
                                                        const int val) {
  const __m128i sqrt2 = _mm_set1_epi32(val);
  if (bit > 0) {
    int i;
    for (i = 0; i < size; i++) {
      const __m128i r0 = av1_round_shift_32_sse4_1(input[i], bit);
      const __m128i r1 = _mm_mullo_epi32(sqrt2, r0);
      output[i] = av1_round_shift_32_sse4_1(r1, NewSqrt2Bits);
    }
  } else {
    int i;
    for (i = 0; i < size; i++) {
      const __m128i r0 = _mm_slli_epi32(input[i], -bit);
      const __m128i r1 = _mm_mullo_epi32(sqrt2, r0);
      output[i] = av1_round_shift_32_sse4_1(r1, NewSqrt2Bits);
    }
  }
}

#ifdef __cplusplus
}
#endif

#endif  // AOM_AV1_COMMON_X86_AV1_TXFM_SSE4_H_
