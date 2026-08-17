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

#ifndef AOM_AOM_DSP_X86_OBMC_INTRINSIC_SSE4_H_
#define AOM_AOM_DSP_X86_OBMC_INTRINSIC_SSE4_H_

#include <smmintrin.h>

#include "aom_dsp/x86/obmc_intrinsic_ssse3.h"
#include "aom_dsp/x86/synonyms.h"

static inline void obmc_variance_w4(const uint8_t *pre, const int pre_stride,
                                    const int32_t *wsrc, const int32_t *mask,
                                    unsigned int *const sse, int *const sum,
                                    const int h) {
  const int pre_step = pre_stride - 4;
  int n = 0;
  __m128i v_sum_d = _mm_setzero_si128();
  __m128i v_sse_d = _mm_setzero_si128();

  assert(IS_POWER_OF_TWO(h));

  do {
    const __m128i v_p_b = xx_loadl_32(pre + n);
    const __m128i v_m_d = _mm_load_si128((const __m128i *)(mask + n));
    const __m128i v_w_d = _mm_load_si128((const __m128i *)(wsrc + n));

    const __m128i v_p_d = _mm_cvtepu8_epi32(v_p_b);

    // Values in both pre and mask fit in 15 bits, and are packed at 32 bit
    // boundaries. We use pmaddwd, as it has lower latency on Haswell
    // than pmulld but produces the same result with these inputs.
    const __m128i v_pm_d = _mm_madd_epi16(v_p_d, v_m_d);

    const __m128i v_diff_d = _mm_sub_epi32(v_w_d, v_pm_d);
    const __m128i v_rdiff_d = xx_roundn_epi32(v_diff_d, 12);
    const __m128i v_sqrdiff_d = _mm_mullo_epi32(v_rdiff_d, v_rdiff_d);

    v_sum_d = _mm_add_epi32(v_sum_d, v_rdiff_d);
    v_sse_d = _mm_add_epi32(v_sse_d, v_sqrdiff_d);

    n += 4;

    if (n % 4 == 0) pre += pre_step;
  } while (n < 4 * h);

  *sum = xx_hsum_epi32_si32(v_sum_d);
  *sse = xx_hsum_epi32_si32(v_sse_d);
}

#endif  // AOM_AOM_DSP_X86_OBMC_INTRINSIC_SSE4_H_
