/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_X86_OBMC_INTRINSIC_SSSE3_H_
#define AOM_AOM_DSP_X86_OBMC_INTRINSIC_SSSE3_H_

#include <immintrin.h>

#include "config/aom_config.h"

static inline int32_t xx_hsum_epi32_si32(__m128i v_d) {
  v_d = _mm_hadd_epi32(v_d, v_d);
  v_d = _mm_hadd_epi32(v_d, v_d);
  return _mm_cvtsi128_si32(v_d);
}

static inline int64_t xx_hsum_epi64_si64(__m128i v_q) {
  v_q = _mm_add_epi64(v_q, _mm_srli_si128(v_q, 8));
#if AOM_ARCH_X86_64
  return _mm_cvtsi128_si64(v_q);
#else
  {
    int64_t tmp;
    _mm_storel_epi64((__m128i *)&tmp, v_q);
    return tmp;
  }
#endif
}

static inline int64_t xx_hsum_epi32_si64(__m128i v_d) {
  const __m128i v_sign_d = _mm_cmplt_epi32(v_d, _mm_setzero_si128());
  const __m128i v_0_q = _mm_unpacklo_epi32(v_d, v_sign_d);
  const __m128i v_1_q = _mm_unpackhi_epi32(v_d, v_sign_d);
  return xx_hsum_epi64_si64(_mm_add_epi64(v_0_q, v_1_q));
}

// This is equivalent to ROUND_POWER_OF_TWO_SIGNED(v_val_d, bits)
static inline __m128i xx_roundn_epi32(__m128i v_val_d, int bits) {
  const __m128i v_bias_d = _mm_set1_epi32((1 << bits) >> 1);
  const __m128i v_sign_d = _mm_srai_epi32(v_val_d, 31);
  const __m128i v_tmp_d =
      _mm_add_epi32(_mm_add_epi32(v_val_d, v_bias_d), v_sign_d);
  return _mm_srai_epi32(v_tmp_d, bits);
}

#endif  // AOM_AOM_DSP_X86_OBMC_INTRINSIC_SSSE3_H_
