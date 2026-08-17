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

#include <immintrin.h>

#include "config/aom_config.h"
#include "aom/aom_integer.h"
#include "aom_dsp/aom_dsp_common.h"

static inline __m256i load_tran_low(const tran_low_t *a) {
  const __m256i a_low = _mm256_loadu_si256((const __m256i *)a);
  const __m256i a_high = _mm256_loadu_si256((const __m256i *)(a + 8));
  return _mm256_packs_epi32(a_low, a_high);
}

static inline void store_tran_low(__m256i a, tran_low_t *b) {
  const __m256i one = _mm256_set1_epi16(1);
  const __m256i a_hi = _mm256_mulhi_epi16(a, one);
  const __m256i a_lo = _mm256_mullo_epi16(a, one);
  const __m256i a_1 = _mm256_unpacklo_epi16(a_lo, a_hi);
  const __m256i a_2 = _mm256_unpackhi_epi16(a_lo, a_hi);
  _mm256_storeu_si256((__m256i *)b, a_1);
  _mm256_storeu_si256((__m256i *)(b + 8), a_2);
}
