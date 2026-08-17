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

#include <xmmintrin.h>

#include "config/aom_config.h"
#include "aom/aom_integer.h"
#include "aom_dsp/aom_dsp_common.h"

// Load 8 16 bit values. If the source is 32 bits then pack down with
// saturation.
static inline __m128i load_tran_low(const tran_low_t *a) {
  const __m128i a_low = _mm_load_si128((const __m128i *)a);
  return _mm_packs_epi32(a_low, *(const __m128i *)(a + 4));
}

static inline void unpack_trans(__m128i a, __m128i *a_1, __m128i *a_2) {
  const __m128i one = _mm_set1_epi16(1);
  const __m128i a_hi = _mm_mulhi_epi16(a, one);
  const __m128i a_lo = _mm_mullo_epi16(a, one);
  *a_1 = _mm_unpacklo_epi16(a_lo, a_hi);
  *a_2 = _mm_unpackhi_epi16(a_lo, a_hi);
}

// Store 8 16 bit values. If the destination is 32 bits then sign extend the
// values by multiplying by 1.
static inline void store_tran_low(__m128i a, tran_low_t *b) {
  __m128i a_1, a_2;
  unpack_trans(a, &a_1, &a_2);
  _mm_store_si128((__m128i *)(b), a_1);
  _mm_store_si128((__m128i *)(b + 4), a_2);
}
// Stores the second result at an offset of 8 (instead of 4) to match the output
// with that of AVX2 implementation and the function is similar to
// store_tran_low().
static inline void store_tran_low_offset_4(__m128i a, tran_low_t *b) {
  __m128i a_1, a_2;
  unpack_trans(a, &a_1, &a_2);
  _mm_store_si128((__m128i *)(b), a_1);
  _mm_store_si128((__m128i *)(b + 8), a_2);
}
