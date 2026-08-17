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

#ifndef AOM_AOM_DSP_X86_MASKED_VARIANCE_INTRIN_SSSE3_H_
#define AOM_AOM_DSP_X86_MASKED_VARIANCE_INTRIN_SSSE3_H_

#include <stdlib.h>
#include <string.h>
#include <tmmintrin.h>

#include "config/aom_config.h"
#include "config/aom_dsp_rtcd.h"

#include "aom_dsp/blend.h"

static inline void comp_mask_pred_16_ssse3(const uint8_t *src0,
                                           const uint8_t *src1,
                                           const uint8_t *mask, uint8_t *dst) {
  const __m128i alpha_max = _mm_set1_epi8(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i round_offset =
      _mm_set1_epi16(1 << (15 - AOM_BLEND_A64_ROUND_BITS));

  const __m128i sA0 = _mm_lddqu_si128((const __m128i *)(src0));
  const __m128i sA1 = _mm_lddqu_si128((const __m128i *)(src1));
  const __m128i aA = _mm_load_si128((const __m128i *)(mask));

  const __m128i maA = _mm_sub_epi8(alpha_max, aA);

  const __m128i ssAL = _mm_unpacklo_epi8(sA0, sA1);
  const __m128i aaAL = _mm_unpacklo_epi8(aA, maA);
  const __m128i ssAH = _mm_unpackhi_epi8(sA0, sA1);
  const __m128i aaAH = _mm_unpackhi_epi8(aA, maA);

  const __m128i blendAL = _mm_maddubs_epi16(ssAL, aaAL);
  const __m128i blendAH = _mm_maddubs_epi16(ssAH, aaAH);

  const __m128i roundAL = _mm_mulhrs_epi16(blendAL, round_offset);
  const __m128i roundAH = _mm_mulhrs_epi16(blendAH, round_offset);
  _mm_store_si128((__m128i *)dst, _mm_packus_epi16(roundAL, roundAH));
}

static inline void comp_mask_pred_8_ssse3(uint8_t *comp_pred, int height,
                                          const uint8_t *src0, int stride0,
                                          const uint8_t *src1, int stride1,
                                          const uint8_t *mask,
                                          int mask_stride) {
  int i = 0;
  const __m128i alpha_max = _mm_set1_epi8(AOM_BLEND_A64_MAX_ALPHA);
  const __m128i round_offset =
      _mm_set1_epi16(1 << (15 - AOM_BLEND_A64_ROUND_BITS));
  do {
    // odd line A
    const __m128i sA0 = _mm_loadl_epi64((const __m128i *)(src0));
    const __m128i sA1 = _mm_loadl_epi64((const __m128i *)(src1));
    const __m128i aA = _mm_loadl_epi64((const __m128i *)(mask));
    // even line B
    const __m128i sB0 = _mm_loadl_epi64((const __m128i *)(src0 + stride0));
    const __m128i sB1 = _mm_loadl_epi64((const __m128i *)(src1 + stride1));
    const __m128i a = _mm_castps_si128(_mm_loadh_pi(
        _mm_castsi128_ps(aA), (const __m64 *)(mask + mask_stride)));

    const __m128i ssA = _mm_unpacklo_epi8(sA0, sA1);
    const __m128i ssB = _mm_unpacklo_epi8(sB0, sB1);

    const __m128i ma = _mm_sub_epi8(alpha_max, a);
    const __m128i aaA = _mm_unpacklo_epi8(a, ma);
    const __m128i aaB = _mm_unpackhi_epi8(a, ma);

    const __m128i blendA = _mm_maddubs_epi16(ssA, aaA);
    const __m128i blendB = _mm_maddubs_epi16(ssB, aaB);
    const __m128i roundA = _mm_mulhrs_epi16(blendA, round_offset);
    const __m128i roundB = _mm_mulhrs_epi16(blendB, round_offset);
    const __m128i round = _mm_packus_epi16(roundA, roundB);
    // comp_pred's stride == width == 8
    _mm_store_si128((__m128i *)(comp_pred), round);
    comp_pred += (8 << 1);
    src0 += (stride0 << 1);
    src1 += (stride1 << 1);
    mask += (mask_stride << 1);
    i += 2;
  } while (i < height);
}

#endif  // AOM_AOM_DSP_X86_MASKED_VARIANCE_INTRIN_SSSE3_H_
