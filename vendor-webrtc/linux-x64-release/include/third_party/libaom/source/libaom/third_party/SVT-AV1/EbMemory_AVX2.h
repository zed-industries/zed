/*
 * Copyright(c) 2019 Intel Corporation
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at https://www.aomedia.org/license/software-license. If the
 * Alliance for Open Media Patent License 1.0 was not distributed with this
 * source code in the PATENTS file, you can obtain it at
 * https://www.aomedia.org/license/patent-license.
 */

#ifndef AOM_THIRD_PARTY_SVT_AV1_EBMEMORY_AVX2_H_
#define AOM_THIRD_PARTY_SVT_AV1_EBMEMORY_AVX2_H_

#include <immintrin.h>

#include "config/aom_config.h"

#include "aom/aom_integer.h"

#ifndef _mm256_set_m128i
#define _mm256_set_m128i(/* __m128i */ hi, /* __m128i */ lo) \
  _mm256_insertf128_si256(_mm256_castsi128_si256(lo), (hi), 0x1)
#endif

#ifndef _mm256_setr_m128i
#define _mm256_setr_m128i(/* __m128i */ lo, /* __m128i */ hi) \
  _mm256_set_m128i((hi), (lo))
#endif

static inline __m256i load_u8_4x2_avx2(const uint8_t *const src,
                                       const ptrdiff_t stride) {
  __m128i src01;
  src01 = _mm_cvtsi32_si128(*(int32_t *)(src + 0 * stride));
  src01 = _mm_insert_epi32(src01, *(int32_t *)(src + 1 * stride), 1);
  return _mm256_setr_m128i(src01, _mm_setzero_si128());
}

static inline __m256i load_u8_4x4_avx2(const uint8_t *const src,
                                       const ptrdiff_t stride) {
  __m128i src01, src23;
  src01 = _mm_cvtsi32_si128(*(int32_t *)(src + 0 * stride));
  src01 = _mm_insert_epi32(src01, *(int32_t *)(src + 1 * stride), 1);
  src23 = _mm_cvtsi32_si128(*(int32_t *)(src + 2 * stride));
  src23 = _mm_insert_epi32(src23, *(int32_t *)(src + 3 * stride), 1);
  return _mm256_setr_m128i(src01, src23);
}

static inline __m256i load_u8_8x2_avx2(const uint8_t *const src,
                                       const ptrdiff_t stride) {
  const __m128i src0 = _mm_loadl_epi64((__m128i *)(src + 0 * stride));
  const __m128i src1 = _mm_loadl_epi64((__m128i *)(src + 1 * stride));
  return _mm256_setr_m128i(src0, src1);
}

static inline __m256i load_u8_8x4_avx2(const uint8_t *const src,
                                       const ptrdiff_t stride) {
  __m128i src01, src23;
  src01 = _mm_loadl_epi64((__m128i *)(src + 0 * stride));
  src01 = _mm_castpd_si128(_mm_loadh_pd(_mm_castsi128_pd(src01),
                                        (double *)(void *)(src + 1 * stride)));
  src23 = _mm_loadl_epi64((__m128i *)(src + 2 * stride));
  src23 = _mm_castpd_si128(_mm_loadh_pd(_mm_castsi128_pd(src23),
                                        (double *)(void *)(src + 3 * stride)));
  return _mm256_setr_m128i(src01, src23);
}

static inline __m256i loadu_8bit_16x2_avx2(const void *const src,
                                           const ptrdiff_t strideInByte) {
  const __m128i src0 = _mm_loadu_si128((__m128i *)src);
  const __m128i src1 =
      _mm_loadu_si128((__m128i *)((uint8_t *)src + strideInByte));
  return _mm256_setr_m128i(src0, src1);
}

static inline __m256i loadu_u8_16x2_avx2(const uint8_t *const src,
                                         const ptrdiff_t stride) {
  return loadu_8bit_16x2_avx2(src, sizeof(*src) * stride);
}

static inline __m256i loadu_u16_8x2_avx2(const uint16_t *const src,
                                         const ptrdiff_t stride) {
  return loadu_8bit_16x2_avx2(src, sizeof(*src) * stride);
}

static inline void storeu_8bit_16x2_avx2(const __m256i src, void *const dst,
                                         const ptrdiff_t strideInByte) {
  const __m128i d0 = _mm256_castsi256_si128(src);
  const __m128i d1 = _mm256_extracti128_si256(src, 1);
  _mm_storeu_si128((__m128i *)dst, d0);
  _mm_storeu_si128((__m128i *)((uint8_t *)dst + strideInByte), d1);
}

static inline void storeu_u8_16x2_avx2(const __m256i src, uint8_t *const dst,
                                       const ptrdiff_t stride) {
  storeu_8bit_16x2_avx2(src, dst, sizeof(*dst) * stride);
}

static inline void storeu_s16_8x2_avx2(const __m256i src, int16_t *const dst,
                                       const ptrdiff_t stride) {
  storeu_8bit_16x2_avx2(src, dst, sizeof(*dst) * stride);
}

static inline void storeu_u16_8x2_avx2(const __m256i src, uint16_t *const dst,
                                       const ptrdiff_t stride) {
  storeu_8bit_16x2_avx2(src, dst, sizeof(*dst) * stride);
}

#endif  // AOM_THIRD_PARTY_SVT_AV1_EBMEMORY_AVX2_H_
