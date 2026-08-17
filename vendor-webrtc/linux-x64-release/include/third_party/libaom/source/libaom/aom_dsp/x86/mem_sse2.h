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

#ifndef AOM_AOM_DSP_X86_MEM_SSE2_H_
#define AOM_AOM_DSP_X86_MEM_SSE2_H_

#include <emmintrin.h>  // SSE2
#include <string.h>

#include "config/aom_config.h"

#include "aom/aom_integer.h"

static inline int16_t loadu_int16(const void *src) {
  int16_t v;
  memcpy(&v, src, sizeof(v));
  return v;
}

static inline int32_t loadu_int32(const void *src) {
  int32_t v;
  memcpy(&v, src, sizeof(v));
  return v;
}

static inline int64_t loadu_int64(const void *src) {
  int64_t v;
  memcpy(&v, src, sizeof(v));
  return v;
}

static inline void _mm_storeh_epi64(__m128i *const d, const __m128i s) {
  _mm_storeh_pi((__m64 *)d, _mm_castsi128_ps(s));
}

static inline __m128i loadh_epi64(const void *const src, const __m128i s) {
  return _mm_castps_si128(
      _mm_loadh_pi(_mm_castsi128_ps(s), (const __m64 *)src));
}

static inline __m128i load_8bit_4x4_to_1_reg_sse2(const void *const src,
                                                  const int byte_stride) {
  return _mm_setr_epi32(loadu_int32((int8_t *)src + 0 * byte_stride),
                        loadu_int32((int8_t *)src + 1 * byte_stride),
                        loadu_int32((int8_t *)src + 2 * byte_stride),
                        loadu_int32((int8_t *)src + 3 * byte_stride));
}

static inline __m128i load_8bit_8x2_to_1_reg_sse2(const void *const src,
                                                  const int byte_stride) {
  __m128i dst;
  dst = _mm_loadl_epi64((__m128i *)((int8_t *)src + 0 * byte_stride));
  dst = loadh_epi64((int8_t *)src + 1 * byte_stride, dst);
  return dst;
}

static inline void store_8bit_8x4_from_16x2(const __m128i *const s,
                                            uint8_t *const d,
                                            const ptrdiff_t stride) {
  _mm_storel_epi64((__m128i *)(d + 0 * stride), s[0]);
  _mm_storeh_epi64((__m128i *)(d + 1 * stride), s[0]);
  _mm_storel_epi64((__m128i *)(d + 2 * stride), s[1]);
  _mm_storeh_epi64((__m128i *)(d + 3 * stride), s[1]);
}

static inline void store_8bit_4x4(const __m128i *const s, uint8_t *const d,
                                  const ptrdiff_t stride) {
  *(int *)(d + 0 * stride) = _mm_cvtsi128_si32(s[0]);
  *(int *)(d + 1 * stride) = _mm_cvtsi128_si32(s[1]);
  *(int *)(d + 2 * stride) = _mm_cvtsi128_si32(s[2]);
  *(int *)(d + 3 * stride) = _mm_cvtsi128_si32(s[3]);
}

static inline void store_8bit_4x4_sse2(const __m128i s, uint8_t *const d,
                                       const ptrdiff_t stride) {
  __m128i ss[4];

  ss[0] = s;
  ss[1] = _mm_srli_si128(s, 4);
  ss[2] = _mm_srli_si128(s, 8);
  ss[3] = _mm_srli_si128(s, 12);
  store_8bit_4x4(ss, d, stride);
}

static inline void load_8bit_4x4(const uint8_t *const s, const ptrdiff_t stride,
                                 __m128i *const d) {
  d[0] = _mm_cvtsi32_si128(*(const int *)(s + 0 * stride));
  d[1] = _mm_cvtsi32_si128(*(const int *)(s + 1 * stride));
  d[2] = _mm_cvtsi32_si128(*(const int *)(s + 2 * stride));
  d[3] = _mm_cvtsi32_si128(*(const int *)(s + 3 * stride));
}

static inline void load_8bit_4x8(const uint8_t *const s, const ptrdiff_t stride,
                                 __m128i *const d) {
  load_8bit_4x4(s + 0 * stride, stride, &d[0]);
  load_8bit_4x4(s + 4 * stride, stride, &d[4]);
}

static inline void load_8bit_8x4(const uint8_t *const s, const ptrdiff_t stride,
                                 __m128i *const d) {
  d[0] = _mm_loadl_epi64((const __m128i *)(s + 0 * stride));
  d[1] = _mm_loadl_epi64((const __m128i *)(s + 1 * stride));
  d[2] = _mm_loadl_epi64((const __m128i *)(s + 2 * stride));
  d[3] = _mm_loadl_epi64((const __m128i *)(s + 3 * stride));
}

static inline void loadu_8bit_16x4(const uint8_t *const s,
                                   const ptrdiff_t stride, __m128i *const d) {
  d[0] = _mm_loadu_si128((const __m128i *)(s + 0 * stride));
  d[1] = _mm_loadu_si128((const __m128i *)(s + 1 * stride));
  d[2] = _mm_loadu_si128((const __m128i *)(s + 2 * stride));
  d[3] = _mm_loadu_si128((const __m128i *)(s + 3 * stride));
}

static inline void load_8bit_8x8(const uint8_t *const s, const ptrdiff_t stride,
                                 __m128i *const d) {
  load_8bit_8x4(s + 0 * stride, stride, &d[0]);
  load_8bit_8x4(s + 4 * stride, stride, &d[4]);
}

static inline void load_8bit_16x8(const uint8_t *const s,
                                  const ptrdiff_t stride, __m128i *const d) {
  d[0] = _mm_load_si128((const __m128i *)(s + 0 * stride));
  d[1] = _mm_load_si128((const __m128i *)(s + 1 * stride));
  d[2] = _mm_load_si128((const __m128i *)(s + 2 * stride));
  d[3] = _mm_load_si128((const __m128i *)(s + 3 * stride));
  d[4] = _mm_load_si128((const __m128i *)(s + 4 * stride));
  d[5] = _mm_load_si128((const __m128i *)(s + 5 * stride));
  d[6] = _mm_load_si128((const __m128i *)(s + 6 * stride));
  d[7] = _mm_load_si128((const __m128i *)(s + 7 * stride));
}

static inline void loadu_8bit_16x8(const uint8_t *const s,
                                   const ptrdiff_t stride, __m128i *const d) {
  loadu_8bit_16x4(s + 0 * stride, stride, &d[0]);
  loadu_8bit_16x4(s + 4 * stride, stride, &d[4]);
}

static inline void store_8bit_8x8(const __m128i *const s, uint8_t *const d,
                                  const ptrdiff_t stride) {
  _mm_storel_epi64((__m128i *)(d + 0 * stride), s[0]);
  _mm_storel_epi64((__m128i *)(d + 1 * stride), s[1]);
  _mm_storel_epi64((__m128i *)(d + 2 * stride), s[2]);
  _mm_storel_epi64((__m128i *)(d + 3 * stride), s[3]);
  _mm_storel_epi64((__m128i *)(d + 4 * stride), s[4]);
  _mm_storel_epi64((__m128i *)(d + 5 * stride), s[5]);
  _mm_storel_epi64((__m128i *)(d + 6 * stride), s[6]);
  _mm_storel_epi64((__m128i *)(d + 7 * stride), s[7]);
}

static inline void storeu_8bit_16x4(const __m128i *const s, uint8_t *const d,
                                    const ptrdiff_t stride) {
  _mm_storeu_si128((__m128i *)(d + 0 * stride), s[0]);
  _mm_storeu_si128((__m128i *)(d + 1 * stride), s[1]);
  _mm_storeu_si128((__m128i *)(d + 2 * stride), s[2]);
  _mm_storeu_si128((__m128i *)(d + 3 * stride), s[3]);
}

#endif  // AOM_AOM_DSP_X86_MEM_SSE2_H_
