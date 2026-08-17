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

#ifndef AOM_AOM_DSP_X86_SYNONYMS_H_
#define AOM_AOM_DSP_X86_SYNONYMS_H_

#include <emmintrin.h>
#include <string.h>

#include "config/aom_config.h"

#include "aom/aom_integer.h"

/**
 * Various reusable shorthands for x86 SIMD intrinsics.
 *
 * Intrinsics prefixed with xx_ operate on or return 128bit XMM registers.
 * Intrinsics prefixed with yy_ operate on or return 256bit YMM registers.
 */

// Loads and stores to do away with the tedium of casting the address
// to the right type.
static inline __m128i xx_loadl_32(const void *a) {
  int val;
  memcpy(&val, a, sizeof(val));
  return _mm_cvtsi32_si128(val);
}

static inline __m128i xx_loadl_64(const void *a) {
  return _mm_loadl_epi64((const __m128i *)a);
}

static inline __m128i xx_load_128(const void *a) {
  return _mm_load_si128((const __m128i *)a);
}

static inline __m128i xx_loadu_128(const void *a) {
  return _mm_loadu_si128((const __m128i *)a);
}

// Load 64 bits from each of hi and low, and pack into an SSE register
// Since directly loading as `int64_t`s and using _mm_set_epi64 may violate
// the strict aliasing rule, this takes a different approach
static inline __m128i xx_loadu_2x64(const void *hi, const void *lo) {
  return _mm_unpacklo_epi64(_mm_loadl_epi64((const __m128i *)lo),
                            _mm_loadl_epi64((const __m128i *)hi));
}

static inline void xx_storel_32(void *const a, const __m128i v) {
  const int val = _mm_cvtsi128_si32(v);
  memcpy(a, &val, sizeof(val));
}

static inline void xx_storel_64(void *const a, const __m128i v) {
  _mm_storel_epi64((__m128i *)a, v);
}

static inline void xx_store_128(void *const a, const __m128i v) {
  _mm_store_si128((__m128i *)a, v);
}

static inline void xx_storeu_128(void *const a, const __m128i v) {
  _mm_storeu_si128((__m128i *)a, v);
}

// Fill an SSE register using an interleaved pair of values, ie. set the
// 8 channels to {a, b, a, b, a, b, a, b}, using the same channel ordering
// as when a register is stored to / loaded from memory.
//
// This is useful for rearranging filter kernels for use with the _mm_madd_epi16
// instruction
static inline __m128i xx_set2_epi16(int16_t a, int16_t b) {
  return _mm_setr_epi16(a, b, a, b, a, b, a, b);
}

static inline __m128i xx_round_epu16(__m128i v_val_w) {
  return _mm_avg_epu16(v_val_w, _mm_setzero_si128());
}

static inline __m128i xx_roundn_epu16(__m128i v_val_w, int bits) {
  const __m128i v_s_w = _mm_srli_epi16(v_val_w, bits - 1);
  return _mm_avg_epu16(v_s_w, _mm_setzero_si128());
}

static inline __m128i xx_roundn_epu32(__m128i v_val_d, int bits) {
  const __m128i v_bias_d = _mm_set1_epi32((1 << bits) >> 1);
  const __m128i v_tmp_d = _mm_add_epi32(v_val_d, v_bias_d);
  return _mm_srli_epi32(v_tmp_d, bits);
}

static inline __m128i xx_roundn_epi16_unsigned(__m128i v_val_d, int bits) {
  const __m128i v_bias_d = _mm_set1_epi16((1 << bits) >> 1);
  const __m128i v_tmp_d = _mm_add_epi16(v_val_d, v_bias_d);
  return _mm_srai_epi16(v_tmp_d, bits);
}

// This is equivalent to ROUND_POWER_OF_TWO(v_val_d, bits)
static inline __m128i xx_roundn_epi32_unsigned(__m128i v_val_d, int bits) {
  const __m128i v_bias_d = _mm_set1_epi32((1 << bits) >> 1);
  const __m128i v_tmp_d = _mm_add_epi32(v_val_d, v_bias_d);
  return _mm_srai_epi32(v_tmp_d, bits);
}

static inline __m128i xx_roundn_epi16(__m128i v_val_d, int bits) {
  const __m128i v_bias_d = _mm_set1_epi16((1 << bits) >> 1);
  const __m128i v_sign_d = _mm_srai_epi16(v_val_d, 15);
  const __m128i v_tmp_d =
      _mm_add_epi16(_mm_add_epi16(v_val_d, v_bias_d), v_sign_d);
  return _mm_srai_epi16(v_tmp_d, bits);
}

#endif  // AOM_AOM_DSP_X86_SYNONYMS_H_
