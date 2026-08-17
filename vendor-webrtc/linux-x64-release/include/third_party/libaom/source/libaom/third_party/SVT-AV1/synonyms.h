/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_THIRD_PARTY_SVT_AV1_SYNONYMS_H_
#define AOM_THIRD_PARTY_SVT_AV1_SYNONYMS_H_

#include "aom_dsp/x86/mem_sse2.h"
#include "aom_dsp/x86/synonyms.h"

static inline __m128i load_u8_8x2_sse2(const uint8_t *const src,
                                       const ptrdiff_t stride) {
  return load_8bit_8x2_to_1_reg_sse2(src, (int)(sizeof(*src) * stride));
}

static AOM_FORCE_INLINE void store_u8_4x2_sse2(const __m128i src,
                                               uint8_t *const dst,
                                               const ptrdiff_t stride) {
  xx_storel_32(dst, src);
  *(uint32_t *)(dst + stride) =
      ((uint32_t)_mm_extract_epi16(src, 3) << 16) | _mm_extract_epi16(src, 2);
}

#endif  // AOM_THIRD_PARTY_SVT_AV1_SYNONYMS_H_
