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

#ifndef AOM_THIRD_PARTY_SVT_AV1_EBMEMORY_SSE4_1_H_
#define AOM_THIRD_PARTY_SVT_AV1_EBMEMORY_SSE4_1_H_

#include <smmintrin.h>

#include "config/aom_config.h"

#include "aom/aom_integer.h"
#include "aom_dsp/x86/mem_sse2.h"

static inline __m128i load8bit_4x2_sse4_1(const void *const src,
                                          const ptrdiff_t strideInByte) {
  const __m128i s = _mm_cvtsi32_si128(loadu_int32(src));
  return _mm_insert_epi32(s, loadu_int32((uint8_t *)src + strideInByte), 1);
}

static inline __m128i load_u8_4x2_sse4_1(const uint8_t *const src,
                                         const ptrdiff_t stride) {
  return load8bit_4x2_sse4_1(src, sizeof(*src) * stride);
}

static inline __m128i load_u16_2x2_sse4_1(const uint16_t *const src,
                                          const ptrdiff_t stride) {
  return load8bit_4x2_sse4_1(src, sizeof(*src) * stride);
}

#endif  // AOM_THIRD_PARTY_SVT_AV1_EBMEMORY_SSE4_1_H_
