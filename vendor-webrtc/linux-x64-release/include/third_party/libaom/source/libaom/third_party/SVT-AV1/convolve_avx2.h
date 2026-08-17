/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef THIRD_PARTY_SVT_AV1_CONVOLVE_AVX2_H_
#define THIRD_PARTY_SVT_AV1_CONVOLVE_AVX2_H_

#include "EbMemory_AVX2.h"
#include "EbMemory_SSE4_1.h"
#include "synonyms.h"

#include "aom_dsp/aom_filter.h"
#include "aom_dsp/x86/convolve_avx2.h"
#include "aom_dsp/x86/mem_sse2.h"

static inline void populate_coeffs_4tap_avx2(const __m128i coeffs_128,
                                             __m256i coeffs[2]) {
  const __m256i coeffs_256 = _mm256_broadcastsi128_si256(coeffs_128);

  // coeffs 2 3 2 3 2 3 2 3
  coeffs[0] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0604u));
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[1] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0a08u));
}

static inline void populate_coeffs_6tap_avx2(const __m128i coeffs_128,
                                             __m256i coeffs[3]) {
  const __m256i coeffs_256 = _mm256_broadcastsi128_si256(coeffs_128);

  // coeffs 1 2 1 2 1 2 1 2
  coeffs[0] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0402u));
  // coeffs 3 4 3 4 3 4 3 4
  coeffs[1] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0806u));
  // coeffs 5 6 5 6 5 6 5 6
  coeffs[2] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0C0Au));
}

static inline void populate_coeffs_8tap_avx2(const __m128i coeffs_128,
                                             __m256i coeffs[4]) {
  const __m256i coeffs_256 = _mm256_broadcastsi128_si256(coeffs_128);

  // coeffs 0 1 0 1 0 1 0 1
  coeffs[0] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0200u));
  // coeffs 2 3 2 3 2 3 2 3
  coeffs[1] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0604u));
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[2] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0a08u));
  // coeffs 6 7 6 7 6 7 6 7
  coeffs[3] = _mm256_shuffle_epi8(coeffs_256, _mm256_set1_epi16(0x0e0cu));
}

static inline void prepare_half_coeffs_2tap_ssse3(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [1] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_cvtsi32_si128(loadu_int32(filter + 3));

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));

  const __m128i coeffs_1 = _mm_srai_epi16(coeffs_8, 1);

  // coeffs 3 4 3 4 3 4 3 4
  *coeffs = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0200u));
}

static inline void prepare_half_coeffs_4tap_ssse3(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [2] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_loadu_si128((__m128i *)filter);

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));

  const __m128i coeffs_1 = _mm_srai_epi16(coeffs_8, 1);

  // coeffs 2 3 2 3 2 3 2 3
  coeffs[0] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0604u));
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[1] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0a08u));
}

static inline void prepare_half_coeffs_6tap_ssse3(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [3] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_loadu_si128((__m128i *)filter);

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));

  const __m128i coeffs_1 = _mm_srai_epi16(coeffs_8, 1);

  // coeffs 1 2 1 2 1 2 1 2
  coeffs[0] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0402u));
  // coeffs 3 4 3 4 3 4 3 4
  coeffs[1] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0806u));
  // coeffs 5 6 5 6 5 6 5 6
  coeffs[2] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0C0Au));
}

static inline void prepare_half_coeffs_8tap_ssse3(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [4] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_loadu_si128((__m128i *)filter);

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));

  const __m128i coeffs_1 = _mm_srai_epi16(coeffs_8, 1);

  // coeffs 0 1 0 1 0 1 0 1
  coeffs[0] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0200u));
  // coeffs 2 3 2 3 2 3 2 3
  coeffs[1] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0604u));
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[2] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0a08u));
  // coeffs 6 7 6 7 6 7 6 7
  coeffs[3] = _mm_shuffle_epi8(coeffs_1, _mm_set1_epi16(0x0e0cu));
}

static inline void prepare_half_coeffs_2tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [1] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_cvtsi32_si128(loadu_int32(filter + 3));
  const __m256i filter_coeffs = _mm256_broadcastsi128_si256(coeffs_8);

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));

  const __m256i coeffs_1 = _mm256_srai_epi16(filter_coeffs, 1);

  // coeffs 3 4 3 4 3 4 3 4
  *coeffs = _mm256_shuffle_epi8(coeffs_1, _mm256_set1_epi16(0x0200u));
}

static inline void prepare_half_coeffs_4tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [2] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_loadu_si128((__m128i *)filter);

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));
  const __m128i coeffs_1 = _mm_srai_epi16(coeffs_8, 1);
  populate_coeffs_4tap_avx2(coeffs_1, coeffs);
}

static inline void prepare_half_coeffs_6tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [3] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_loadu_si128((__m128i *)filter);

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));
  const __m128i coeffs_1 = _mm_srai_epi16(coeffs_8, 1);
  populate_coeffs_6tap_avx2(coeffs_1, coeffs);
}

static inline void prepare_half_coeffs_8tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [4] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_loadu_si128((__m128i *)filter);

  // right shift all filter co-efficients by 1 to reduce the bits required.
  // This extra right shift will be taken care of at the end while rounding
  // the result.
  // Since all filter co-efficients are even, this change will not affect the
  // end result
  assert(_mm_test_all_zeros(_mm_and_si128(coeffs_8, _mm_set1_epi16(1)),
                            _mm_set1_epi16((short)0xffff)));
  const __m128i coeffs_1 = _mm_srai_epi16(coeffs_8, 1);
  populate_coeffs_8tap_avx2(coeffs_1, coeffs);
}

static inline void prepare_coeffs_2tap_sse2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [1] */) {
  const int16_t *filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);

  const __m128i coeff = _mm_cvtsi32_si128(loadu_int32(filter + 3));

  // coeffs 3 4 3 4 3 4 3 4
  coeffs[0] = _mm_shuffle_epi32(coeff, 0x00);
}

static inline void prepare_coeffs_4tap_sse2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [2] */) {
  const int16_t *filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);

  const __m128i coeff = _mm_loadu_si128((__m128i *)filter);

  // coeffs 2 3 2 3 2 3 2 3
  coeffs[0] = _mm_shuffle_epi32(coeff, 0x55);
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[1] = _mm_shuffle_epi32(coeff, 0xaa);
}

static inline void prepare_coeffs_6tap_ssse3(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [3] */) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeff = _mm_loadu_si128((__m128i *)filter);

  // coeffs 1 2 1 2 1 2 1 2
  coeffs[0] = _mm_shuffle_epi8(coeff, _mm_set1_epi32(0x05040302u));
  // coeffs 3 4 3 4 3 4 3 4
  coeffs[1] = _mm_shuffle_epi8(coeff, _mm_set1_epi32(0x09080706u));
  // coeffs 5 6 5 6 5 6 5 6
  coeffs[2] = _mm_shuffle_epi8(coeff, _mm_set1_epi32(0x0D0C0B0Au));
}

static inline void prepare_coeffs_8tap_sse2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m128i *const coeffs /* [4] */) {
  const int16_t *filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);

  const __m128i coeff = _mm_loadu_si128((__m128i *)filter);

  // coeffs 0 1 0 1 0 1 0 1
  coeffs[0] = _mm_shuffle_epi32(coeff, 0x00);
  // coeffs 2 3 2 3 2 3 2 3
  coeffs[1] = _mm_shuffle_epi32(coeff, 0x55);
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[2] = _mm_shuffle_epi32(coeff, 0xaa);
  // coeffs 6 7 6 7 6 7 6 7
  coeffs[3] = _mm_shuffle_epi32(coeff, 0xff);
}

static inline void prepare_coeffs_2tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [1] */) {
  const int16_t *filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);

  const __m128i coeff_8 = _mm_cvtsi32_si128(loadu_int32(filter + 3));
  const __m256i coeff = _mm256_broadcastsi128_si256(coeff_8);

  // coeffs 3 4 3 4 3 4 3 4
  coeffs[0] = _mm256_shuffle_epi32(coeff, 0x00);
}

static inline void prepare_coeffs_4tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [2] */) {
  const int16_t *filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);

  const __m128i coeff_8 = _mm_loadu_si128((__m128i *)filter);
  const __m256i coeff = _mm256_broadcastsi128_si256(coeff_8);

  // coeffs 2 3 2 3 2 3 2 3
  coeffs[0] = _mm256_shuffle_epi32(coeff, 0x55);
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[1] = _mm256_shuffle_epi32(coeff, 0xaa);
}

static inline void prepare_coeffs_6tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [3]*/) {
  const int16_t *const filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);
  const __m128i coeffs_8 = _mm_loadu_si128((__m128i *)filter);
  const __m256i coeff = _mm256_broadcastsi128_si256(coeffs_8);

  // coeffs 1 2 1 2 1 2 1 2
  coeffs[0] = _mm256_shuffle_epi8(coeff, _mm256_set1_epi32(0x05040302u));
  // coeffs 3 4 3 4 3 4 3 4
  coeffs[1] = _mm256_shuffle_epi8(coeff, _mm256_set1_epi32(0x09080706u));
  // coeffs 5 6 5 6 5 6 5 6
  coeffs[2] = _mm256_shuffle_epi8(coeff, _mm256_set1_epi32(0x0D0C0B0Au));
}

static inline void prepare_coeffs_8tap_avx2(
    const InterpFilterParams *const filter_params, const int32_t subpel_q4,
    __m256i *const coeffs /* [4] */) {
  const int16_t *filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);

  const __m128i coeff_8 = _mm_loadu_si128((__m128i *)filter);
  const __m256i coeff = _mm256_broadcastsi128_si256(coeff_8);

  // coeffs 0 1 0 1 0 1 0 1
  coeffs[0] = _mm256_shuffle_epi32(coeff, 0x00);
  // coeffs 2 3 2 3 2 3 2 3
  coeffs[1] = _mm256_shuffle_epi32(coeff, 0x55);
  // coeffs 4 5 4 5 4 5 4 5
  coeffs[2] = _mm256_shuffle_epi32(coeff, 0xaa);
  // coeffs 6 7 6 7 6 7 6 7
  coeffs[3] = _mm256_shuffle_epi32(coeff, 0xff);
}

static inline void load_16bit_5rows_avx2(const int16_t *const src,
                                         const ptrdiff_t stride,
                                         __m256i dst[5]) {
  dst[0] = _mm256_loadu_si256((__m256i *)(src + 0 * stride));
  dst[1] = _mm256_loadu_si256((__m256i *)(src + 1 * stride));
  dst[2] = _mm256_loadu_si256((__m256i *)(src + 2 * stride));
  dst[3] = _mm256_loadu_si256((__m256i *)(src + 3 * stride));
  dst[4] = _mm256_loadu_si256((__m256i *)(src + 4 * stride));
}

static inline void load_16bit_7rows_avx2(const int16_t *const src,
                                         const ptrdiff_t stride,
                                         __m256i dst[7]) {
  dst[0] = _mm256_loadu_si256((__m256i *)(src + 0 * stride));
  dst[1] = _mm256_loadu_si256((__m256i *)(src + 1 * stride));
  dst[2] = _mm256_loadu_si256((__m256i *)(src + 2 * stride));
  dst[3] = _mm256_loadu_si256((__m256i *)(src + 3 * stride));
  dst[4] = _mm256_loadu_si256((__m256i *)(src + 4 * stride));
  dst[5] = _mm256_loadu_si256((__m256i *)(src + 5 * stride));
  dst[6] = _mm256_loadu_si256((__m256i *)(src + 6 * stride));
}

static AOM_FORCE_INLINE void load_16bit_8rows_avx2(const int16_t *const src,
                                                   const ptrdiff_t stride,
                                                   __m256i dst[8]) {
  dst[0] = _mm256_loadu_si256((__m256i *)(src + 0 * stride));
  dst[1] = _mm256_loadu_si256((__m256i *)(src + 1 * stride));
  dst[2] = _mm256_loadu_si256((__m256i *)(src + 2 * stride));
  dst[3] = _mm256_loadu_si256((__m256i *)(src + 3 * stride));
  dst[4] = _mm256_loadu_si256((__m256i *)(src + 4 * stride));
  dst[5] = _mm256_loadu_si256((__m256i *)(src + 5 * stride));
  dst[6] = _mm256_loadu_si256((__m256i *)(src + 6 * stride));
  dst[7] = _mm256_loadu_si256((__m256i *)(src + 7 * stride));
}

static AOM_FORCE_INLINE void loadu_unpack_16bit_5rows_avx2(
    const int16_t *const src, const ptrdiff_t stride, __m256i s_256[5],
    __m256i ss_256[5], __m256i tt_256[5]) {
  s_256[0] = _mm256_loadu_si256((__m256i *)(src + 0 * stride));
  s_256[1] = _mm256_loadu_si256((__m256i *)(src + 1 * stride));
  s_256[2] = _mm256_loadu_si256((__m256i *)(src + 2 * stride));
  s_256[3] = _mm256_loadu_si256((__m256i *)(src + 3 * stride));
  s_256[4] = _mm256_loadu_si256((__m256i *)(src + 4 * stride));

  ss_256[0] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  ss_256[1] = _mm256_unpacklo_epi16(s_256[2], s_256[3]);
  ss_256[3] = _mm256_unpackhi_epi16(s_256[0], s_256[1]);
  ss_256[4] = _mm256_unpackhi_epi16(s_256[2], s_256[3]);

  tt_256[0] = _mm256_unpacklo_epi16(s_256[1], s_256[2]);
  tt_256[1] = _mm256_unpacklo_epi16(s_256[3], s_256[4]);
  tt_256[3] = _mm256_unpackhi_epi16(s_256[1], s_256[2]);
  tt_256[4] = _mm256_unpackhi_epi16(s_256[3], s_256[4]);
}

static AOM_FORCE_INLINE void loadu_unpack_16bit_3rows_avx2(
    const int16_t *const src, const ptrdiff_t stride, __m256i s_256[3],
    __m256i ss_256[3], __m256i tt_256[3]) {
  s_256[0] = _mm256_loadu_si256((__m256i *)(src + 0 * stride));
  s_256[1] = _mm256_loadu_si256((__m256i *)(src + 1 * stride));
  s_256[2] = _mm256_loadu_si256((__m256i *)(src + 2 * stride));

  ss_256[0] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  ss_256[2] = _mm256_unpackhi_epi16(s_256[0], s_256[1]);

  tt_256[0] = _mm256_unpacklo_epi16(s_256[1], s_256[2]);
  tt_256[2] = _mm256_unpackhi_epi16(s_256[1], s_256[2]);
}

static inline void convolve_8tap_unpack_avx2(const __m256i s[6],
                                             __m256i ss[7]) {
  ss[0] = _mm256_unpacklo_epi16(s[0], s[1]);
  ss[1] = _mm256_unpacklo_epi16(s[2], s[3]);
  ss[2] = _mm256_unpacklo_epi16(s[4], s[5]);
  ss[4] = _mm256_unpackhi_epi16(s[0], s[1]);
  ss[5] = _mm256_unpackhi_epi16(s[2], s[3]);
  ss[6] = _mm256_unpackhi_epi16(s[4], s[5]);
}

static inline __m128i convolve_2tap_ssse3(const __m128i ss[1],
                                          const __m128i coeffs[1]) {
  return _mm_maddubs_epi16(ss[0], coeffs[0]);
}

static inline __m128i convolve_4tap_ssse3(const __m128i ss[2],
                                          const __m128i coeffs[2]) {
  const __m128i res_23 = _mm_maddubs_epi16(ss[0], coeffs[0]);
  const __m128i res_45 = _mm_maddubs_epi16(ss[1], coeffs[1]);
  return _mm_add_epi16(res_23, res_45);
}

static inline __m128i convolve_6tap_ssse3(const __m128i ss[3],
                                          const __m128i coeffs[3]) {
  const __m128i res_12 = _mm_maddubs_epi16(ss[0], coeffs[0]);
  const __m128i res_34 = _mm_maddubs_epi16(ss[1], coeffs[1]);
  const __m128i res_56 = _mm_maddubs_epi16(ss[2], coeffs[2]);
  const __m128i res_1256 = _mm_add_epi16(res_12, res_56);
  return _mm_add_epi16(res_1256, res_34);
}

static inline __m128i convolve_8tap_ssse3(const __m128i ss[4],
                                          const __m128i coeffs[4]) {
  const __m128i res_01 = _mm_maddubs_epi16(ss[0], coeffs[0]);
  const __m128i res_23 = _mm_maddubs_epi16(ss[1], coeffs[1]);
  const __m128i res_45 = _mm_maddubs_epi16(ss[2], coeffs[2]);
  const __m128i res_67 = _mm_maddubs_epi16(ss[3], coeffs[3]);
  const __m128i res_0145 = _mm_add_epi16(res_01, res_45);
  const __m128i res_2367 = _mm_add_epi16(res_23, res_67);
  return _mm_add_epi16(res_0145, res_2367);
}

static inline __m256i convolve_2tap_avx2(const __m256i ss[1],
                                         const __m256i coeffs[1]) {
  return _mm256_maddubs_epi16(ss[0], coeffs[0]);
}

static inline __m256i convolve_4tap_avx2(const __m256i ss[2],
                                         const __m256i coeffs[2]) {
  const __m256i res_23 = _mm256_maddubs_epi16(ss[0], coeffs[0]);
  const __m256i res_45 = _mm256_maddubs_epi16(ss[1], coeffs[1]);
  return _mm256_add_epi16(res_23, res_45);
}

static inline __m256i convolve_6tap_avx2(const __m256i ss[3],
                                         const __m256i coeffs[3]) {
  const __m256i res_01 = _mm256_maddubs_epi16(ss[0], coeffs[0]);
  const __m256i res_23 = _mm256_maddubs_epi16(ss[1], coeffs[1]);
  const __m256i res_45 = _mm256_maddubs_epi16(ss[2], coeffs[2]);
  const __m256i res_0145 = _mm256_add_epi16(res_01, res_45);
  return _mm256_add_epi16(res_0145, res_23);
}

static inline __m256i convolve_8tap_avx2(const __m256i ss[4],
                                         const __m256i coeffs[4]) {
  const __m256i res_01 = _mm256_maddubs_epi16(ss[0], coeffs[0]);
  const __m256i res_23 = _mm256_maddubs_epi16(ss[1], coeffs[1]);
  const __m256i res_45 = _mm256_maddubs_epi16(ss[2], coeffs[2]);
  const __m256i res_67 = _mm256_maddubs_epi16(ss[3], coeffs[3]);
  const __m256i res_0145 = _mm256_add_epi16(res_01, res_45);
  const __m256i res_2367 = _mm256_add_epi16(res_23, res_67);
  return _mm256_add_epi16(res_0145, res_2367);
}

static inline __m128i convolve16_2tap_sse2(const __m128i ss[1],
                                           const __m128i coeffs[1]) {
  return _mm_madd_epi16(ss[0], coeffs[0]);
}

static inline __m128i convolve16_4tap_sse2(const __m128i ss[2],
                                           const __m128i coeffs[2]) {
  const __m128i res_01 = _mm_madd_epi16(ss[0], coeffs[0]);
  const __m128i res_23 = _mm_madd_epi16(ss[1], coeffs[1]);
  return _mm_add_epi32(res_01, res_23);
}

static inline __m128i convolve16_6tap_sse2(const __m128i ss[3],
                                           const __m128i coeffs[3]) {
  const __m128i res_01 = _mm_madd_epi16(ss[0], coeffs[0]);
  const __m128i res_23 = _mm_madd_epi16(ss[1], coeffs[1]);
  const __m128i res_45 = _mm_madd_epi16(ss[2], coeffs[2]);
  const __m128i res_0123 = _mm_add_epi32(res_01, res_23);
  return _mm_add_epi32(res_0123, res_45);
}

static inline __m128i convolve16_8tap_sse2(const __m128i ss[4],
                                           const __m128i coeffs[4]) {
  const __m128i res_01 = _mm_madd_epi16(ss[0], coeffs[0]);
  const __m128i res_23 = _mm_madd_epi16(ss[1], coeffs[1]);
  const __m128i res_45 = _mm_madd_epi16(ss[2], coeffs[2]);
  const __m128i res_67 = _mm_madd_epi16(ss[3], coeffs[3]);
  const __m128i res_0123 = _mm_add_epi32(res_01, res_23);
  const __m128i res_4567 = _mm_add_epi32(res_45, res_67);
  return _mm_add_epi32(res_0123, res_4567);
}

static inline __m256i convolve16_2tap_avx2(const __m256i ss[1],
                                           const __m256i coeffs[1]) {
  return _mm256_madd_epi16(ss[0], coeffs[0]);
}

static inline __m256i convolve16_4tap_avx2(const __m256i ss[2],
                                           const __m256i coeffs[2]) {
  const __m256i res_1 = _mm256_madd_epi16(ss[0], coeffs[0]);
  const __m256i res_2 = _mm256_madd_epi16(ss[1], coeffs[1]);
  return _mm256_add_epi32(res_1, res_2);
}

static inline __m256i convolve16_6tap_avx2(const __m256i ss[3],
                                           const __m256i coeffs[3]) {
  const __m256i res_01 = _mm256_madd_epi16(ss[0], coeffs[0]);
  const __m256i res_23 = _mm256_madd_epi16(ss[1], coeffs[1]);
  const __m256i res_45 = _mm256_madd_epi16(ss[2], coeffs[2]);
  const __m256i res_0123 = _mm256_add_epi32(res_01, res_23);
  return _mm256_add_epi32(res_0123, res_45);
}

static inline __m256i convolve16_8tap_avx2(const __m256i ss[4],
                                           const __m256i coeffs[4]) {
  const __m256i res_01 = _mm256_madd_epi16(ss[0], coeffs[0]);
  const __m256i res_23 = _mm256_madd_epi16(ss[1], coeffs[1]);
  const __m256i res_45 = _mm256_madd_epi16(ss[2], coeffs[2]);
  const __m256i res_67 = _mm256_madd_epi16(ss[3], coeffs[3]);
  const __m256i res_0123 = _mm256_add_epi32(res_01, res_23);
  const __m256i res_4567 = _mm256_add_epi32(res_45, res_67);
  return _mm256_add_epi32(res_0123, res_4567);
}

static inline __m256i x_convolve_4tap_avx2(const __m256i data,
                                           const __m256i coeffs[2],
                                           const __m256i filt[2]) {
  __m256i ss[2];

  ss[0] = _mm256_shuffle_epi8(data, filt[0]);
  ss[1] = _mm256_shuffle_epi8(data, filt[1]);

  return convolve_4tap_avx2(ss, coeffs);
}

static inline __m256i x_convolve_6tap_avx2(const __m256i data,
                                           const __m256i coeffs[3],
                                           const __m256i filt[3]) {
  __m256i ss[3];

  ss[0] = _mm256_shuffle_epi8(data, filt[0]);
  ss[1] = _mm256_shuffle_epi8(data, filt[1]);
  ss[2] = _mm256_shuffle_epi8(data, filt[2]);

  return convolve_6tap_avx2(ss, coeffs);
}

static inline __m256i x_convolve_8tap_avx2(const __m256i data,
                                           const __m256i coeffs[4],
                                           const __m256i filt[4]) {
  __m256i ss[4];

  ss[0] = _mm256_shuffle_epi8(data, filt[0]);
  ss[1] = _mm256_shuffle_epi8(data, filt[1]);
  ss[2] = _mm256_shuffle_epi8(data, filt[2]);
  ss[3] = _mm256_shuffle_epi8(data, filt[3]);

  return convolve_8tap_avx2(ss, coeffs);
}

static inline __m256i sr_y_round_avx2(const __m256i src) {
  const __m256i round = _mm256_set1_epi16(32);
  const __m256i dst = _mm256_add_epi16(src, round);
  return _mm256_srai_epi16(dst, FILTER_BITS - 1);
}

static inline __m128i xy_x_round_sse2(const __m128i src) {
  const __m128i round = _mm_set1_epi16(2);
  const __m128i dst = _mm_add_epi16(src, round);
  return _mm_srai_epi16(dst, 2);
}

static inline __m256i xy_x_round_avx2(const __m256i src) {
  const __m256i round = _mm256_set1_epi16(2);
  const __m256i dst = _mm256_add_epi16(src, round);
  return _mm256_srai_epi16(dst, 2);
}

static inline void xy_x_round_store_2x2_sse2(const __m128i res,
                                             int16_t *const dst) {
  const __m128i d = xy_x_round_sse2(res);
  _mm_storel_epi64((__m128i *)dst, d);
}

static inline void xy_x_round_store_4x2_sse2(const __m128i res,
                                             int16_t *const dst) {
  const __m128i d = xy_x_round_sse2(res);
  _mm_storeu_si128((__m128i *)dst, d);
}

static inline void xy_x_round_store_8x2_sse2(const __m128i res[2],
                                             int16_t *const dst) {
  __m128i r[2];

  r[0] = xy_x_round_sse2(res[0]);
  r[1] = xy_x_round_sse2(res[1]);
  _mm_storeu_si128((__m128i *)dst, r[0]);
  _mm_storeu_si128((__m128i *)(dst + 8), r[1]);
}

static inline void xy_x_round_store_8x2_avx2(const __m256i res,
                                             int16_t *const dst) {
  const __m256i d = xy_x_round_avx2(res);
  _mm256_storeu_si256((__m256i *)dst, d);
}

static inline void xy_x_round_store_32_avx2(const __m256i res[2],
                                            int16_t *const dst) {
  __m256i r[2];

  r[0] = xy_x_round_avx2(res[0]);
  r[1] = xy_x_round_avx2(res[1]);
  const __m256i d0 =
      _mm256_inserti128_si256(r[0], _mm256_castsi256_si128(r[1]), 1);
  const __m256i d1 =
      _mm256_inserti128_si256(r[1], _mm256_extracti128_si256(r[0], 1), 0);
  _mm256_storeu_si256((__m256i *)dst, d0);
  _mm256_storeu_si256((__m256i *)(dst + 16), d1);
}

static inline __m128i xy_y_round_sse2(const __m128i src) {
  const __m128i round = _mm_set1_epi32(1024);
  const __m128i dst = _mm_add_epi32(src, round);
  return _mm_srai_epi32(dst, 11);
}

static inline __m128i xy_y_round_half_pel_sse2(const __m128i src) {
  const __m128i round = _mm_set1_epi16(16);
  const __m128i dst = _mm_add_epi16(src, round);
  return _mm_srai_epi16(dst, 5);
}

static inline __m256i xy_y_round_avx2(const __m256i src) {
  const __m256i round = _mm256_set1_epi32(1024);
  const __m256i dst = _mm256_add_epi32(src, round);
  return _mm256_srai_epi32(dst, 11);
}

static inline __m256i xy_y_round_16_avx2(const __m256i r[2]) {
  const __m256i r0 = xy_y_round_avx2(r[0]);
  const __m256i r1 = xy_y_round_avx2(r[1]);
  return _mm256_packs_epi32(r0, r1);
}

static inline __m256i xy_y_round_half_pel_avx2(const __m256i src) {
  const __m256i round = _mm256_set1_epi16(16);
  const __m256i dst = _mm256_add_epi16(src, round);
  return _mm256_srai_epi16(dst, 5);
}

static inline void pack_store_2x2_sse2(const __m128i res, uint8_t *const dst,
                                       const ptrdiff_t stride) {
  const __m128i d = _mm_packus_epi16(res, res);
  *(int16_t *)dst = (int16_t)_mm_cvtsi128_si32(d);
  *(int16_t *)(dst + stride) = (int16_t)_mm_extract_epi16(d, 1);
}

static inline void pack_store_4x2_sse2(const __m128i res, uint8_t *const dst,
                                       const ptrdiff_t stride) {
  const __m128i d = _mm_packus_epi16(res, res);
  store_u8_4x2_sse2(d, dst, stride);
}

static inline void pack_store_4x2_avx2(const __m256i res, uint8_t *const dst,
                                       const ptrdiff_t stride) {
  const __m256i d = _mm256_packus_epi16(res, res);
  const __m128i d0 = _mm256_castsi256_si128(d);
  const __m128i d1 = _mm256_extracti128_si256(d, 1);

  xx_storel_32(dst, d0);
  xx_storel_32(dst + stride, d1);
}

static inline void pack_store_8x2_avx2(const __m256i res, uint8_t *const dst,
                                       const ptrdiff_t stride) {
  const __m256i d = _mm256_packus_epi16(res, res);
  const __m128i d0 = _mm256_castsi256_si128(d);
  const __m128i d1 = _mm256_extracti128_si256(d, 1);
  _mm_storel_epi64((__m128i *)dst, d0);
  _mm_storel_epi64((__m128i *)(dst + stride), d1);
}

static inline void pack_store_16x2_avx2(const __m256i res0, const __m256i res1,
                                        uint8_t *const dst,
                                        const ptrdiff_t stride) {
  const __m256i d = _mm256_packus_epi16(res0, res1);
  storeu_u8_16x2_avx2(d, dst, stride);
}

static inline void xy_y_pack_store_16x2_avx2(const __m256i res0,
                                             const __m256i res1,
                                             uint8_t *const dst,
                                             const ptrdiff_t stride) {
  const __m256i t = _mm256_packus_epi16(res0, res1);
  const __m256i d = _mm256_permute4x64_epi64(t, 0xD8);
  storeu_u8_16x2_avx2(d, dst, stride);
}

static inline void pack_store_32_avx2(const __m256i res0, const __m256i res1,
                                      uint8_t *const dst) {
  const __m256i t = _mm256_packus_epi16(res0, res1);
  const __m256i d = _mm256_permute4x64_epi64(t, 0xD8);
  _mm256_storeu_si256((__m256i *)dst, d);
}

static inline void xy_y_round_store_2x2_sse2(const __m128i res,
                                             uint8_t *const dst,
                                             const ptrdiff_t stride) {
  const __m128i r = xy_y_round_sse2(res);
  const __m128i rr = _mm_packs_epi32(r, r);
  pack_store_2x2_sse2(rr, dst, stride);
}

static inline void xy_y_round_store_4x2_avx2(const __m256i res,
                                             uint8_t *const dst,
                                             const ptrdiff_t stride) {
  const __m256i r = xy_y_round_avx2(res);
  const __m256i rr = _mm256_packs_epi32(r, r);
  pack_store_4x2_avx2(rr, dst, stride);
}

static inline void xy_y_pack_store_32_avx2(const __m256i res0,
                                           const __m256i res1,
                                           uint8_t *const dst) {
  const __m256i d = _mm256_packus_epi16(res0, res1);
  // d = _mm256_permute4x64_epi64(d, 0xD8);
  _mm256_storeu_si256((__m256i *)dst, d);
}

static inline void xy_y_round_store_32_avx2(const __m256i r0[2],
                                            const __m256i r1[2],
                                            uint8_t *const dst) {
  const __m256i ra = xy_y_round_16_avx2(r0);
  const __m256i rb = xy_y_round_16_avx2(r1);
  xy_y_pack_store_32_avx2(ra, rb, dst);
}

static inline void convolve_store_32_avx2(const __m256i res0,
                                          const __m256i res1,
                                          uint8_t *const dst) {
  const __m256i d = _mm256_packus_epi16(res0, res1);
  _mm256_storeu_si256((__m256i *)dst, d);
}

static inline __m128i sr_x_round_sse2(const __m128i src) {
  const __m128i round = _mm_set1_epi16(34);
  const __m128i dst = _mm_add_epi16(src, round);
  return _mm_srai_epi16(dst, 6);
}

static inline __m256i sr_x_round_avx2(const __m256i src) {
  const __m256i round = _mm256_set1_epi16(34);
  const __m256i dst = _mm256_add_epi16(src, round);
  return _mm256_srai_epi16(dst, 6);
}

static inline __m128i sr_y_round_sse2(const __m128i src) {
  const __m128i round = _mm_set1_epi16(32);
  const __m128i dst = _mm_add_epi16(src, round);
  return _mm_srai_epi16(dst, FILTER_BITS - 1);
}

static inline void sr_x_round_store_8x2_avx2(const __m256i res,
                                             uint8_t *const dst,
                                             const ptrdiff_t dst_stride) {
  const __m256i r = sr_x_round_avx2(res);
  pack_store_8x2_avx2(r, dst, dst_stride);
}

static inline void sr_x_round_store_16x2_avx2(const __m256i res[2],
                                              uint8_t *const dst,
                                              const ptrdiff_t dst_stride) {
  __m256i r[2];

  r[0] = sr_x_round_avx2(res[0]);
  r[1] = sr_x_round_avx2(res[1]);
  pack_store_16x2_avx2(r[0], r[1], dst, dst_stride);
}

static inline void sr_x_round_store_32_avx2(const __m256i res[2],
                                            uint8_t *const dst) {
  __m256i r[2];

  r[0] = sr_x_round_avx2(res[0]);
  r[1] = sr_x_round_avx2(res[1]);
  convolve_store_32_avx2(r[0], r[1], dst);
}

static inline void sr_y_round_store_8x2_avx2(const __m256i res,
                                             uint8_t *const dst,
                                             const ptrdiff_t dst_stride) {
  const __m256i r = sr_y_round_avx2(res);
  pack_store_8x2_avx2(r, dst, dst_stride);
}

static inline void sr_y_round_store_16x2_avx2(const __m256i res[2],
                                              uint8_t *const dst,
                                              const ptrdiff_t dst_stride) {
  __m256i r[2];

  r[0] = sr_y_round_avx2(res[0]);
  r[1] = sr_y_round_avx2(res[1]);
  pack_store_16x2_avx2(r[0], r[1], dst, dst_stride);
}

static inline void sr_y_2tap_32_avg_avx2(const uint8_t *const src,
                                         const __m256i s0, __m256i *const s1,
                                         uint8_t *const dst) {
  *s1 = _mm256_loadu_si256((__m256i *)src);
  const __m256i d = _mm256_avg_epu8(s0, *s1);
  _mm256_storeu_si256((__m256i *)dst, d);
}

static inline void sr_x_2tap_32_avg_avx2(const uint8_t *const src,
                                         uint8_t *const dst) {
  const __m256i s0 = _mm256_loadu_si256((__m256i *)src);
  const __m256i s1 = _mm256_loadu_si256((__m256i *)(src + 1));
  const __m256i d = _mm256_avg_epu8(s0, s1);
  _mm256_storeu_si256((__m256i *)dst, d);
}

static inline __m128i x_convolve_2tap_2x2_sse4_1(const uint8_t *const src,
                                                 const ptrdiff_t stride,
                                                 const __m128i coeffs[1]) {
  const __m128i sfl =
      _mm_setr_epi8(0, 1, 1, 2, 4, 5, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0);
  const __m128i s_128 = load_u8_4x2_sse4_1(src, stride);
  const __m128i ss = _mm_shuffle_epi8(s_128, sfl);
  return convolve_2tap_ssse3(&ss, coeffs);
}

static inline __m128i x_convolve_2tap_4x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[1]) {
  const __m128i sfl =
      _mm_setr_epi8(0, 1, 1, 2, 2, 3, 3, 4, 8, 9, 9, 10, 10, 11, 11, 12);
  const __m128i s_128 = load_u8_8x2_sse2(src, stride);
  const __m128i ss = _mm_shuffle_epi8(s_128, sfl);
  return convolve_2tap_ssse3(&ss, coeffs);
}

static inline void x_convolve_2tap_8x2_ssse3(const uint8_t *const src,
                                             const ptrdiff_t stride,
                                             const __m128i coeffs[1],
                                             __m128i r[2]) {
  __m128i ss[2];
  const __m128i s00 = _mm_loadu_si128((__m128i *)src);
  const __m128i s10 = _mm_loadu_si128((__m128i *)(src + stride));
  const __m128i s01 = _mm_srli_si128(s00, 1);
  const __m128i s11 = _mm_srli_si128(s10, 1);
  ss[0] = _mm_unpacklo_epi8(s00, s01);
  ss[1] = _mm_unpacklo_epi8(s10, s11);

  r[0] = convolve_2tap_ssse3(&ss[0], coeffs);
  r[1] = convolve_2tap_ssse3(&ss[1], coeffs);
}

static inline __m256i x_convolve_2tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[1]) {
  __m128i s_128[2][2];
  __m256i s_256[2];

  s_128[0][0] = _mm_loadu_si128((__m128i *)src);
  s_128[1][0] = _mm_loadu_si128((__m128i *)(src + stride));
  s_128[0][1] = _mm_srli_si128(s_128[0][0], 1);
  s_128[1][1] = _mm_srli_si128(s_128[1][0], 1);
  s_256[0] = _mm256_setr_m128i(s_128[0][0], s_128[1][0]);
  s_256[1] = _mm256_setr_m128i(s_128[0][1], s_128[1][1]);
  const __m256i ss = _mm256_unpacklo_epi8(s_256[0], s_256[1]);
  return convolve_2tap_avx2(&ss, coeffs);
}

static inline void x_convolve_2tap_16x2_avx2(const uint8_t *const src,
                                             const ptrdiff_t stride,
                                             const __m256i coeffs[1],
                                             __m256i r[2]) {
  const __m256i s0_256 = loadu_8bit_16x2_avx2(src, stride);
  const __m256i s1_256 = loadu_8bit_16x2_avx2(src + 1, stride);
  const __m256i s0 = _mm256_unpacklo_epi8(s0_256, s1_256);
  const __m256i s1 = _mm256_unpackhi_epi8(s0_256, s1_256);
  r[0] = convolve_2tap_avx2(&s0, coeffs);
  r[1] = convolve_2tap_avx2(&s1, coeffs);
}

static inline void x_convolve_2tap_32_avx2(const uint8_t *const src,
                                           const __m256i coeffs[1],
                                           __m256i r[2]) {
  const __m256i s0 = _mm256_loadu_si256((__m256i *)src);
  const __m256i s1 = _mm256_loadu_si256((__m256i *)(src + 1));
  const __m256i ss0 = _mm256_unpacklo_epi8(s0, s1);
  const __m256i ss1 = _mm256_unpackhi_epi8(s0, s1);

  r[0] = convolve_2tap_avx2(&ss0, coeffs);
  r[1] = convolve_2tap_avx2(&ss1, coeffs);
}

static inline __m128i x_convolve_4tap_2x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[2]) {
  const __m128i sfl0 =
      _mm_setr_epi8(0, 1, 1, 2, 8, 9, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0);
  const __m128i sfl1 =
      _mm_setr_epi8(2, 3, 3, 4, 10, 11, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0);
  const __m128i s = load_u8_8x2_sse2(src, stride);
  __m128i ss[2];

  ss[0] = _mm_shuffle_epi8(s, sfl0);
  ss[1] = _mm_shuffle_epi8(s, sfl1);
  return convolve_4tap_ssse3(ss, coeffs);
}

static inline __m128i x_convolve_4tap_4x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[2]) {
  const __m128i s = load_u8_8x2_sse2(src, stride);
  const __m128i sfl0 =
      _mm_setr_epi8(0, 1, 1, 2, 2, 3, 3, 4, 8, 9, 9, 10, 10, 11, 11, 12);
  const __m128i sfl1 =
      _mm_setr_epi8(2, 3, 3, 4, 4, 5, 5, 6, 10, 11, 11, 12, 12, 13, 13, 14);
  __m128i ss[2];

  ss[0] = _mm_shuffle_epi8(s, sfl0);
  ss[1] = _mm_shuffle_epi8(s, sfl1);
  return convolve_4tap_ssse3(ss, coeffs);
}

static inline __m256i x_convolve_4tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[2],
                                               const __m256i filt[2]) {
  const __m256i s_256 = loadu_8bit_16x2_avx2(src, stride);
  return x_convolve_4tap_avx2(s_256, coeffs, filt);
}

static inline void x_convolve_4tap_16x2_avx2(const uint8_t *const src,
                                             const int32_t src_stride,
                                             const __m256i coeffs[2],
                                             const __m256i filt[2],
                                             __m256i r[2]) {
  r[0] = x_convolve_4tap_8x2_avx2(src + 0, src_stride, coeffs, filt);
  r[1] = x_convolve_4tap_8x2_avx2(src + 8, src_stride, coeffs, filt);
}

static inline void x_convolve_4tap_32_avx2(const uint8_t *const src,
                                           const __m256i coeffs[2],
                                           const __m256i filt[2],
                                           __m256i r[2]) {
  const __m256i s0_256 = _mm256_loadu_si256((__m256i *)src);
  const __m256i s1_256 = _mm256_loadu_si256((__m256i *)(src + 8));

  r[0] = x_convolve_4tap_avx2(s0_256, coeffs, filt);
  r[1] = x_convolve_4tap_avx2(s1_256, coeffs, filt);
}

static inline __m128i x_convolve_6tap_2x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[3]) {
  const __m128i sfl0 =
      _mm_setr_epi8(0, 1, 1, 2, 8, 9, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0);
  const __m128i sfl1 =
      _mm_setr_epi8(2, 3, 3, 4, 10, 11, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0);
  const __m128i sfl2 =
      _mm_setr_epi8(4, 5, 5, 6, 12, 13, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0);

  const __m128i s = load_u8_8x2_sse2(src, stride);
  __m128i ss[3];

  ss[0] = _mm_shuffle_epi8(s, sfl0);
  ss[1] = _mm_shuffle_epi8(s, sfl1);
  ss[2] = _mm_shuffle_epi8(s, sfl2);
  return convolve_6tap_ssse3(ss, coeffs);
}

static inline __m128i x_convolve_6tap_4x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[3]) {
  const __m128i s = load_u8_8x2_sse2(src, stride);
  const __m128i sfl0 =
      _mm_setr_epi8(0, 1, 1, 2, 8, 9, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0);
  const __m128i sfl1 =
      _mm_setr_epi8(2, 3, 3, 4, 10, 11, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0);
  const __m128i sfl2 =
      _mm_setr_epi8(4, 5, 5, 6, 12, 13, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0);
  __m128i ss[3];

  ss[0] = _mm_shuffle_epi8(s, sfl0);
  ss[1] = _mm_shuffle_epi8(s, sfl1);
  ss[2] = _mm_shuffle_epi8(s, sfl2);
  return convolve_6tap_ssse3(ss, coeffs);
}

static inline __m256i x_convolve_6tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[3],
                                               const __m256i filt[3]) {
  const __m256i s_256 = loadu_8bit_16x2_avx2(src, stride);
  return x_convolve_6tap_avx2(s_256, coeffs, filt);
}

static inline void x_convolve_6tap_16x2_avx2(const uint8_t *const src,
                                             const int32_t src_stride,
                                             const __m256i coeffs[3],
                                             const __m256i filt[3],
                                             __m256i r[2]) {
  r[0] = x_convolve_6tap_8x2_avx2(src + 0, src_stride, coeffs, filt);
  r[1] = x_convolve_6tap_8x2_avx2(src + 8, src_stride, coeffs, filt);
}

static inline void x_convolve_6tap_32_avx2(const uint8_t *const src,
                                           const __m256i coeffs[3],
                                           const __m256i filt[3],
                                           __m256i r[2]) {
  const __m256i s0_256 = _mm256_loadu_si256((__m256i *)src);
  const __m256i s1_256 = _mm256_loadu_si256((__m256i *)(src + 8));

  r[0] = x_convolve_6tap_avx2(s0_256, coeffs, filt);
  r[1] = x_convolve_6tap_avx2(s1_256, coeffs, filt);
}

static inline __m256i x_convolve_8tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[4],
                                               const __m256i filt[4]) {
  const __m256i s_256 = loadu_8bit_16x2_avx2(src, stride);
  return x_convolve_8tap_avx2(s_256, coeffs, filt);
}

static AOM_FORCE_INLINE void x_convolve_8tap_16x2_avx2(const uint8_t *const src,
                                                       const int32_t src_stride,
                                                       const __m256i coeffs[4],
                                                       const __m256i filt[4],
                                                       __m256i r[2]) {
  r[0] = x_convolve_8tap_8x2_avx2(src + 0, src_stride, coeffs, filt);
  r[1] = x_convolve_8tap_8x2_avx2(src + 8, src_stride, coeffs, filt);
}

static AOM_FORCE_INLINE void x_convolve_8tap_32_avx2(const uint8_t *const src,
                                                     const __m256i coeffs[4],
                                                     const __m256i filt[4],
                                                     __m256i r[2]) {
  const __m256i s0_256 = _mm256_loadu_si256((__m256i *)src);
  const __m256i s1_256 = _mm256_loadu_si256((__m256i *)(src + 8));

  r[0] = x_convolve_8tap_avx2(s0_256, coeffs, filt);
  r[1] = x_convolve_8tap_avx2(s1_256, coeffs, filt);
}

static inline __m128i y_convolve_2tap_2x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[1],
                                                __m128i s_16[2]) {
  __m128i s_128[2];

  s_16[1] = _mm_cvtsi32_si128(*(int16_t *)(src + stride));
  s_128[0] = _mm_unpacklo_epi16(s_16[0], s_16[1]);
  s_16[0] = _mm_cvtsi32_si128(*(int16_t *)(src + 2 * stride));
  s_128[1] = _mm_unpacklo_epi16(s_16[1], s_16[0]);
  const __m128i ss = _mm_unpacklo_epi8(s_128[0], s_128[1]);
  return convolve_2tap_ssse3(&ss, coeffs);
}

static inline __m128i y_convolve_2tap_4x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[1],
                                                __m128i s_32[2]) {
  __m128i s_128[2];

  s_32[1] = _mm_cvtsi32_si128(loadu_int32(src + stride));
  s_128[0] = _mm_unpacklo_epi32(s_32[0], s_32[1]);
  s_32[0] = _mm_cvtsi32_si128(loadu_int32(src + 2 * stride));
  s_128[1] = _mm_unpacklo_epi32(s_32[1], s_32[0]);
  const __m128i ss = _mm_unpacklo_epi8(s_128[0], s_128[1]);
  return convolve_2tap_ssse3(&ss, coeffs);
}

static inline __m256i y_convolve_2tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[1],
                                               __m128i s_64[2]) {
  __m256i s_256[2];

  s_64[1] = _mm_loadl_epi64((__m128i *)(src + stride));
  s_256[0] = _mm256_setr_m128i(s_64[0], s_64[1]);
  s_64[0] = _mm_loadl_epi64((__m128i *)(src + 2 * stride));
  s_256[1] = _mm256_setr_m128i(s_64[1], s_64[0]);
  const __m256i ss = _mm256_unpacklo_epi8(s_256[0], s_256[1]);
  return convolve_2tap_avx2(&ss, coeffs);
}

static inline void y_convolve_2tap_16x2_avx2(const uint8_t *const src,
                                             const ptrdiff_t stride,
                                             const __m256i coeffs[1],
                                             __m128i s_128[2], __m256i r[2]) {
  __m256i s_256[2];

  s_128[1] = _mm_loadu_si128((__m128i *)(src + stride));
  s_256[0] = _mm256_setr_m128i(s_128[0], s_128[1]);
  s_128[0] = _mm_loadu_si128((__m128i *)(src + 2 * stride));
  s_256[1] = _mm256_setr_m128i(s_128[1], s_128[0]);
  const __m256i ss0 = _mm256_unpacklo_epi8(s_256[0], s_256[1]);
  const __m256i ss1 = _mm256_unpackhi_epi8(s_256[0], s_256[1]);
  r[0] = convolve_2tap_avx2(&ss0, coeffs);
  r[1] = convolve_2tap_avx2(&ss1, coeffs);
}

static inline void y_convolve_2tap_32_avx2(const uint8_t *const src,
                                           const __m256i coeffs[1],
                                           const __m256i s0, __m256i *const s1,
                                           __m256i r[2]) {
  *s1 = _mm256_loadu_si256((__m256i *)src);
  const __m256i ss0 = _mm256_unpacklo_epi8(s0, *s1);
  const __m256i ss1 = _mm256_unpackhi_epi8(s0, *s1);
  r[0] = convolve_2tap_avx2(&ss0, coeffs);
  r[1] = convolve_2tap_avx2(&ss1, coeffs);
}

static inline __m128i y_convolve_4tap_2x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[2],
                                                __m128i s_16[4],
                                                __m128i ss_128[2]) {
  s_16[3] = _mm_cvtsi32_si128(loadu_int16(src + stride));
  const __m128i src23 = _mm_unpacklo_epi16(s_16[2], s_16[3]);
  s_16[2] = _mm_cvtsi32_si128(loadu_int16(src + 2 * stride));
  const __m128i src34 = _mm_unpacklo_epi16(s_16[3], s_16[2]);
  ss_128[1] = _mm_unpacklo_epi8(src23, src34);
  return convolve_4tap_ssse3(ss_128, coeffs);
}

static inline __m128i y_convolve_4tap_4x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[2],
                                                __m128i s_32[4],
                                                __m128i ss_128[2]) {
  s_32[3] = _mm_cvtsi32_si128(loadu_int32(src + stride));
  const __m128i src23 = _mm_unpacklo_epi32(s_32[2], s_32[3]);
  s_32[2] = _mm_cvtsi32_si128(loadu_int32(src + 2 * stride));
  const __m128i src34 = _mm_unpacklo_epi32(s_32[3], s_32[2]);
  ss_128[1] = _mm_unpacklo_epi8(src23, src34);
  return convolve_4tap_ssse3(ss_128, coeffs);
}

static inline __m256i y_convolve_4tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[2],
                                               __m128i s_64[4],
                                               __m256i ss_256[2]) {
  s_64[3] = _mm_loadl_epi64((__m128i *)(src + stride));
  const __m256i src23 = _mm256_setr_m128i(s_64[2], s_64[3]);
  s_64[2] = _mm_loadl_epi64((__m128i *)(src + 2 * stride));
  const __m256i src34 = _mm256_setr_m128i(s_64[3], s_64[2]);
  ss_256[1] = _mm256_unpacklo_epi8(src23, src34);
  return convolve_4tap_avx2(ss_256, coeffs);
}

static inline void y_convolve_4tap_16x2_avx2(const uint8_t *const src,
                                             const ptrdiff_t stride,
                                             const __m256i coeffs[2],
                                             __m128i s_128[4],
                                             __m256i ss_256[4], __m256i r[2]) {
  s_128[3] = _mm_loadu_si128((__m128i *)(src + stride));
  const __m256i src23 = _mm256_setr_m128i(s_128[2], s_128[3]);
  s_128[2] = _mm_loadu_si128((__m128i *)(src + 2 * stride));
  const __m256i src34 = _mm256_setr_m128i(s_128[3], s_128[2]);
  ss_256[1] = _mm256_unpacklo_epi8(src23, src34);
  ss_256[3] = _mm256_unpackhi_epi8(src23, src34);
  r[0] = convolve_4tap_avx2(ss_256, coeffs);
  r[1] = convolve_4tap_avx2(ss_256 + 2, coeffs);
}

static inline __m128i y_convolve_6tap_2x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[3],
                                                __m128i s_16[6],
                                                __m128i ss_128[3]) {
  s_16[5] = _mm_cvtsi32_si128(loadu_int16(src + 3 * stride));
  const __m128i src45 = _mm_unpacklo_epi16(s_16[4], s_16[5]);
  s_16[4] = _mm_cvtsi32_si128(loadu_int16(src + 4 * stride));
  const __m128i src56 = _mm_unpacklo_epi16(s_16[5], s_16[4]);
  ss_128[2] = _mm_unpacklo_epi8(src45, src56);
  return convolve_6tap_ssse3(ss_128, coeffs);
}

static inline void y_convolve_4tap_32x2_avx2(
    const uint8_t *const src, const ptrdiff_t stride, const __m256i coeffs[2],
    __m256i s_256[4], __m256i ss_256[4], __m256i tt_256[4], __m256i r[4]) {
  s_256[3] = _mm256_loadu_si256((__m256i *)(src + 1 * stride));
  ss_256[1] = _mm256_unpacklo_epi8(s_256[2], s_256[3]);
  ss_256[3] = _mm256_unpackhi_epi8(s_256[2], s_256[3]);
  s_256[2] = _mm256_loadu_si256((__m256i *)(src + 2 * stride));
  tt_256[1] = _mm256_unpacklo_epi8(s_256[3], s_256[2]);
  tt_256[3] = _mm256_unpackhi_epi8(s_256[3], s_256[2]);
  r[0] = convolve_4tap_avx2(ss_256 + 0, coeffs);
  r[1] = convolve_4tap_avx2(ss_256 + 2, coeffs);
  r[2] = convolve_4tap_avx2(tt_256 + 0, coeffs);
  r[3] = convolve_4tap_avx2(tt_256 + 2, coeffs);
}

static inline __m128i y_convolve_6tap_4x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[3],
                                                __m128i s_32[6],
                                                __m128i ss_128[3]) {
  s_32[5] = _mm_cvtsi32_si128(loadu_int32(src + 3 * stride));
  const __m128i src45 = _mm_unpacklo_epi32(s_32[4], s_32[5]);
  s_32[4] = _mm_cvtsi32_si128(loadu_int32(src + 4 * stride));
  const __m128i src56 = _mm_unpacklo_epi32(s_32[5], s_32[4]);
  ss_128[2] = _mm_unpacklo_epi8(src45, src56);
  return convolve_6tap_ssse3(ss_128, coeffs);
}

static inline __m256i y_convolve_6tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[3],
                                               __m128i s_64[6],
                                               __m256i ss_256[3]) {
  s_64[5] = _mm_loadl_epi64((__m128i *)(src + 3 * stride));
  const __m256i src45 = _mm256_setr_m128i(s_64[4], s_64[5]);
  s_64[4] = _mm_loadl_epi64((__m128i *)(src + 4 * stride));
  const __m256i src56 = _mm256_setr_m128i(s_64[5], s_64[4]);
  ss_256[2] = _mm256_unpacklo_epi8(src45, src56);
  return convolve_6tap_avx2(ss_256, coeffs);
}

static inline void y_convolve_6tap_16x2_avx2(const uint8_t *const src,
                                             const ptrdiff_t stride,
                                             const __m256i coeffs[3],
                                             __m128i s_128[6],
                                             __m256i ss_256[6], __m256i r[2]) {
  s_128[5] = _mm_loadu_si128((__m128i *)(src + 3 * stride));
  const __m256i src45 = _mm256_setr_m128i(s_128[4], s_128[5]);
  s_128[4] = _mm_loadu_si128((__m128i *)(src + 4 * stride));
  const __m256i src56 = _mm256_setr_m128i(s_128[5], s_128[4]);
  ss_256[2] = _mm256_unpacklo_epi8(src45, src56);
  ss_256[5] = _mm256_unpackhi_epi8(src45, src56);
  r[0] = convolve_6tap_avx2(ss_256, coeffs);
  r[1] = convolve_6tap_avx2(ss_256 + 3, coeffs);
}

static inline void y_convolve_6tap_32x2_avx2(
    const uint8_t *const src, const ptrdiff_t stride, const __m256i coeffs[3],
    __m256i s_256[6], __m256i ss_256[6], __m256i tt_256[6], __m256i r[4]) {
  s_256[5] = _mm256_loadu_si256((__m256i *)(src + 3 * stride));
  ss_256[2] = _mm256_unpacklo_epi8(s_256[4], s_256[5]);
  ss_256[5] = _mm256_unpackhi_epi8(s_256[4], s_256[5]);
  s_256[4] = _mm256_loadu_si256((__m256i *)(src + 4 * stride));
  tt_256[2] = _mm256_unpacklo_epi8(s_256[5], s_256[4]);
  tt_256[5] = _mm256_unpackhi_epi8(s_256[5], s_256[4]);
  r[0] = convolve_6tap_avx2(ss_256 + 0, coeffs);
  r[1] = convolve_6tap_avx2(ss_256 + 3, coeffs);
  r[2] = convolve_6tap_avx2(tt_256 + 0, coeffs);
  r[3] = convolve_6tap_avx2(tt_256 + 3, coeffs);
}

static inline __m128i y_convolve_8tap_2x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[4],
                                                __m128i s_16[8],
                                                __m128i ss_128[4]) {
  s_16[7] = _mm_cvtsi32_si128(loadu_int16(src + 7 * stride));
  const __m128i src67 = _mm_unpacklo_epi16(s_16[6], s_16[7]);
  s_16[6] = _mm_cvtsi32_si128(loadu_int16(src + 8 * stride));
  const __m128i src78 = _mm_unpacklo_epi16(s_16[7], s_16[6]);
  ss_128[3] = _mm_unpacklo_epi8(src67, src78);
  return convolve_8tap_ssse3(ss_128, coeffs);
}

static inline __m128i y_convolve_8tap_4x2_ssse3(const uint8_t *const src,
                                                const ptrdiff_t stride,
                                                const __m128i coeffs[4],
                                                __m128i s_32[8],
                                                __m128i ss_128[4]) {
  s_32[7] = _mm_cvtsi32_si128(loadu_int32(src + 7 * stride));
  const __m128i src67 = _mm_unpacklo_epi32(s_32[6], s_32[7]);
  s_32[6] = _mm_cvtsi32_si128(loadu_int32(src + 8 * stride));
  const __m128i src78 = _mm_unpacklo_epi32(s_32[7], s_32[6]);
  ss_128[3] = _mm_unpacklo_epi8(src67, src78);
  return convolve_8tap_ssse3(ss_128, coeffs);
}

static inline __m256i y_convolve_8tap_8x2_avx2(const uint8_t *const src,
                                               const ptrdiff_t stride,
                                               const __m256i coeffs[4],
                                               __m128i s_64[8],
                                               __m256i ss_256[4]) {
  s_64[7] = _mm_loadl_epi64((__m128i *)(src + 7 * stride));
  const __m256i src67 = _mm256_setr_m128i(s_64[6], s_64[7]);
  s_64[6] = _mm_loadl_epi64((__m128i *)(src + 8 * stride));
  const __m256i src78 = _mm256_setr_m128i(s_64[7], s_64[6]);
  ss_256[3] = _mm256_unpacklo_epi8(src67, src78);
  return convolve_8tap_avx2(ss_256, coeffs);
}

static inline void y_convolve_8tap_16x2_avx2(const uint8_t *const src,
                                             const ptrdiff_t stride,
                                             const __m256i coeffs[4],
                                             __m128i s_128[8],
                                             __m256i ss_256[8], __m256i r[2]) {
  s_128[7] = _mm_loadu_si128((__m128i *)(src + 7 * stride));
  const __m256i src67 = _mm256_setr_m128i(s_128[6], s_128[7]);
  s_128[6] = _mm_loadu_si128((__m128i *)(src + 8 * stride));
  const __m256i src78 = _mm256_setr_m128i(s_128[7], s_128[6]);
  ss_256[3] = _mm256_unpacklo_epi8(src67, src78);
  ss_256[7] = _mm256_unpackhi_epi8(src67, src78);
  r[0] = convolve_8tap_avx2(ss_256, coeffs);
  r[1] = convolve_8tap_avx2(ss_256 + 4, coeffs);
}

static inline void y_convolve_8tap_32x2_avx2(
    const uint8_t *const src, const ptrdiff_t stride, const __m256i coeffs[4],
    __m256i s_256[8], __m256i ss_256[8], __m256i tt_256[8], __m256i r[4]) {
  s_256[7] = _mm256_loadu_si256((__m256i *)(src + 7 * stride));
  ss_256[3] = _mm256_unpacklo_epi8(s_256[6], s_256[7]);
  ss_256[7] = _mm256_unpackhi_epi8(s_256[6], s_256[7]);
  s_256[6] = _mm256_loadu_si256((__m256i *)(src + 8 * stride));
  tt_256[3] = _mm256_unpacklo_epi8(s_256[7], s_256[6]);
  tt_256[7] = _mm256_unpackhi_epi8(s_256[7], s_256[6]);
  r[0] = convolve_8tap_avx2(ss_256 + 0, coeffs);
  r[1] = convolve_8tap_avx2(ss_256 + 4, coeffs);
  r[2] = convolve_8tap_avx2(tt_256 + 0, coeffs);
  r[3] = convolve_8tap_avx2(tt_256 + 4, coeffs);
}

static inline void xy_x_convolve_2tap_32_avx2(const uint8_t *const src,
                                              const __m256i coeffs[1],
                                              __m256i r[2]) {
  const __m256i s0 = _mm256_loadu_si256((__m256i *)src);
  const __m256i s1 = _mm256_loadu_si256((__m256i *)(src + 1));
  const __m256i ss0 = _mm256_unpacklo_epi8(s0, s1);
  const __m256i ss1 = _mm256_unpackhi_epi8(s0, s1);

  r[0] = convolve_2tap_avx2(&ss0, coeffs);
  r[1] = convolve_2tap_avx2(&ss1, coeffs);
}

static inline void xy_x_2tap_32_avx2(const uint8_t *const src,
                                     const __m256i coeffs[1],
                                     int16_t *const dst) {
  __m256i r[2];

  xy_x_convolve_2tap_32_avx2(src, coeffs, r);
  const __m256i d0 = xy_x_round_avx2(r[0]);
  const __m256i d1 = xy_x_round_avx2(r[1]);
  _mm256_storeu_si256((__m256i *)dst, d0);
  _mm256_storeu_si256((__m256i *)(dst + 16), d1);
}

static inline void xy_x_4tap_32_avx2(const uint8_t *const src,
                                     const __m256i coeffs[2],
                                     const __m256i filt[2],
                                     int16_t *const dst) {
  __m256i r[2];

  x_convolve_4tap_32_avx2(src, coeffs, filt, r);
  const __m256i d0 = xy_x_round_avx2(r[0]);
  const __m256i d1 = xy_x_round_avx2(r[1]);
  _mm256_storeu_si256((__m256i *)dst, d0);
  _mm256_storeu_si256((__m256i *)(dst + 16), d1);
}

static inline void xy_x_6tap_32_avx2(const uint8_t *const src,
                                     const __m256i coeffs[3],
                                     const __m256i filt[3],
                                     int16_t *const dst) {
  __m256i r[2];

  x_convolve_6tap_32_avx2(src, coeffs, filt, r);
  const __m256i d0 = xy_x_round_avx2(r[0]);
  const __m256i d1 = xy_x_round_avx2(r[1]);
  _mm256_storeu_si256((__m256i *)dst, d0);
  _mm256_storeu_si256((__m256i *)(dst + 16), d1);
}

static inline void xy_x_8tap_32_avx2(const uint8_t *const src,
                                     const __m256i coeffs[4],
                                     const __m256i filt[4],
                                     int16_t *const dst) {
  __m256i r[2];

  x_convolve_8tap_32_avx2(src, coeffs, filt, r);
  const __m256i d0 = xy_x_round_avx2(r[0]);
  const __m256i d1 = xy_x_round_avx2(r[1]);
  _mm256_storeu_si256((__m256i *)dst, d0);
  _mm256_storeu_si256((__m256i *)(dst + 16), d1);
}

static inline __m128i xy_y_convolve_2tap_2x2_sse2(const int16_t *const src,
                                                  __m128i s_32[2],
                                                  const __m128i coeffs[1]) {
  __m128i s_128[2];

  s_32[1] = _mm_cvtsi32_si128(loadu_int32(src + 2));
  s_128[0] = _mm_unpacklo_epi32(s_32[0], s_32[1]);
  s_32[0] = _mm_cvtsi32_si128(loadu_int32(src + 2 * 2));
  s_128[1] = _mm_unpacklo_epi32(s_32[1], s_32[0]);
  const __m128i ss = _mm_unpacklo_epi16(s_128[0], s_128[1]);
  return convolve16_2tap_sse2(&ss, coeffs);
}

static inline __m128i xy_y_convolve_2tap_2x2_half_pel_sse2(
    const int16_t *const src, __m128i s_32[2]) {
  __m128i s_128[2];

  s_32[1] = _mm_cvtsi32_si128(loadu_int32(src + 2));
  s_128[0] = _mm_unpacklo_epi32(s_32[0], s_32[1]);
  s_32[0] = _mm_cvtsi32_si128(loadu_int32(src + 2 * 2));
  s_128[1] = _mm_unpacklo_epi32(s_32[1], s_32[0]);
  return _mm_add_epi16(s_128[0], s_128[1]);
}

static inline void xy_y_convolve_2tap_4x2_sse2(const int16_t *const src,
                                               __m128i s_64[2],
                                               const __m128i coeffs[1],
                                               __m128i r[2]) {
  __m128i s_128[2];

  s_64[1] = _mm_loadl_epi64((__m128i *)(src + 4));
  s_128[0] = _mm_unpacklo_epi64(s_64[0], s_64[1]);
  s_64[0] = _mm_loadl_epi64((__m128i *)(src + 2 * 4));
  s_128[1] = _mm_unpacklo_epi64(s_64[1], s_64[0]);
  const __m128i ss0 = _mm_unpacklo_epi16(s_128[0], s_128[1]);
  const __m128i ss1 = _mm_unpackhi_epi16(s_128[0], s_128[1]);
  r[0] = convolve16_2tap_sse2(&ss0, coeffs);
  r[1] = convolve16_2tap_sse2(&ss1, coeffs);
}

static inline __m128i xy_y_convolve_2tap_4x2_half_pel_sse2(
    const int16_t *const src, __m128i s_64[2]) {
  __m128i s_128[2];

  s_64[1] = _mm_loadl_epi64((__m128i *)(src + 4));
  s_128[0] = _mm_unpacklo_epi64(s_64[0], s_64[1]);
  s_64[0] = _mm_loadl_epi64((__m128i *)(src + 2 * 4));
  s_128[1] = _mm_unpacklo_epi64(s_64[1], s_64[0]);
  return _mm_add_epi16(s_128[0], s_128[1]);
}

static inline void xy_y_convolve_2tap_16_avx2(const __m256i s0,
                                              const __m256i s1,
                                              const __m256i coeffs[1],
                                              __m256i r[2]) {
  const __m256i ss0 = _mm256_unpacklo_epi16(s0, s1);
  const __m256i ss1 = _mm256_unpackhi_epi16(s0, s1);
  r[0] = convolve16_2tap_avx2(&ss0, coeffs);
  r[1] = convolve16_2tap_avx2(&ss1, coeffs);
}

static inline void xy_y_convolve_2tap_8x2_avx2(const int16_t *const src,
                                               __m128i s_128[2],
                                               const __m256i coeffs[1],
                                               __m256i r[2]) {
  __m256i s_256[2];
  s_128[1] = _mm_loadu_si128((__m128i *)(src + 8));
  s_256[0] = _mm256_setr_m128i(s_128[0], s_128[1]);
  s_128[0] = _mm_loadu_si128((__m128i *)(src + 2 * 8));
  s_256[1] = _mm256_setr_m128i(s_128[1], s_128[0]);
  xy_y_convolve_2tap_16_avx2(s_256[0], s_256[1], coeffs, r);
}

static inline __m256i xy_y_convolve_2tap_8x2_half_pel_avx2(
    const int16_t *const src, __m128i s_128[2]) {
  __m256i s_256[2];
  s_128[1] = _mm_loadu_si128((__m128i *)(src + 8));
  s_256[0] = _mm256_setr_m128i(s_128[0], s_128[1]);
  s_128[0] = _mm_loadu_si128((__m128i *)(src + 2 * 8));
  s_256[1] = _mm256_setr_m128i(s_128[1], s_128[0]);
  return _mm256_add_epi16(s_256[0], s_256[1]);
}

static inline void xy_y_convolve_2tap_16x2_half_pel_avx2(
    const int16_t *const src, __m256i s_256[2], __m256i r[2]) {
  s_256[1] = _mm256_loadu_si256((__m256i *)(src + 16));
  r[0] = _mm256_add_epi16(s_256[0], s_256[1]);
  s_256[0] = _mm256_loadu_si256((__m256i *)(src + 2 * 16));
  r[1] = _mm256_add_epi16(s_256[1], s_256[0]);
}

static inline void xy_y_store_16x2_avx2(const __m256i r[2], uint8_t *const dst,
                                        const ptrdiff_t stride) {
  const __m256i t = _mm256_packus_epi16(r[0], r[1]);
  const __m256i d = _mm256_permute4x64_epi64(t, 0xD8);
  storeu_u8_16x2_avx2(d, dst, stride);
}

static inline void xy_y_convolve_2tap_16x2_avx2(const int16_t *const src,
                                                __m256i s[2],
                                                const __m256i coeffs[1],
                                                __m256i r[4]) {
  s[1] = _mm256_loadu_si256((__m256i *)(src + 16));
  xy_y_convolve_2tap_16_avx2(s[0], s[1], coeffs, r + 0);
  s[0] = _mm256_loadu_si256((__m256i *)(src + 2 * 16));
  xy_y_convolve_2tap_16_avx2(s[1], s[0], coeffs, r + 2);
}

static inline void xy_y_convolve_2tap_32_avx2(const int16_t *const src,
                                              const __m256i s0[2],
                                              __m256i s1[2],
                                              const __m256i coeffs[1],
                                              __m256i r[4]) {
  s1[0] = _mm256_loadu_si256((__m256i *)src);
  s1[1] = _mm256_loadu_si256((__m256i *)(src + 16));
  xy_y_convolve_2tap_16_avx2(s0[0], s1[0], coeffs, r + 0);
  xy_y_convolve_2tap_16_avx2(s0[1], s1[1], coeffs, r + 2);
}

static inline void xy_y_convolve_2tap_32_all_avx2(const int16_t *const src,
                                                  const __m256i s0[2],
                                                  __m256i s1[2],
                                                  const __m256i coeffs[1],
                                                  uint8_t *const dst) {
  __m256i r[4];

  xy_y_convolve_2tap_32_avx2(src, s0, s1, coeffs, r);
  xy_y_round_store_32_avx2(r + 0, r + 2, dst);
}

static inline void xy_y_convolve_2tap_half_pel_32_avx2(const int16_t *const src,
                                                       const __m256i s0[2],
                                                       __m256i s1[2],
                                                       __m256i r[2]) {
  s1[0] = _mm256_loadu_si256((__m256i *)src);
  s1[1] = _mm256_loadu_si256((__m256i *)(src + 16));
  r[0] = _mm256_add_epi16(s0[0], s1[0]);
  r[1] = _mm256_add_epi16(s0[1], s1[1]);
}

static inline void xy_y_convolve_2tap_half_pel_32_all_avx2(
    const int16_t *const src, const __m256i s0[2], __m256i s1[2],
    uint8_t *const dst) {
  __m256i r[2];

  xy_y_convolve_2tap_half_pel_32_avx2(src, s0, s1, r);
  r[0] = xy_y_round_half_pel_avx2(r[0]);
  r[1] = xy_y_round_half_pel_avx2(r[1]);
  xy_y_pack_store_32_avx2(r[0], r[1], dst);
}

static inline __m128i xy_y_convolve_4tap_2x2_sse2(const int16_t *const src,
                                                  __m128i s_32[4],
                                                  __m128i ss_128[2],
                                                  const __m128i coeffs[2]) {
  s_32[3] = _mm_cvtsi32_si128(loadu_int32(src + 3 * 2));
  const __m128i src23 = _mm_unpacklo_epi32(s_32[2], s_32[3]);
  s_32[2] = _mm_cvtsi32_si128(loadu_int32(src + 4 * 2));
  const __m128i src34 = _mm_unpacklo_epi32(s_32[3], s_32[2]);
  ss_128[1] = _mm_unpacklo_epi16(src23, src34);
  const __m128i r = convolve16_4tap_sse2(ss_128, coeffs);
  ss_128[0] = ss_128[1];
  return r;
}

static inline __m256i xy_y_convolve_4tap_4x2_avx2(const int16_t *const src,
                                                  __m128i s_64[4],
                                                  __m256i ss_256[2],
                                                  const __m256i coeffs[2]) {
  __m256i s_256[2];
  s_64[3] = _mm_loadl_epi64((__m128i *)(src + 3 * 4));
  s_256[0] = _mm256_setr_m128i(s_64[2], s_64[3]);
  s_64[2] = _mm_loadl_epi64((__m128i *)(src + 4 * 4));
  s_256[1] = _mm256_setr_m128i(s_64[3], s_64[2]);
  ss_256[1] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  const __m256i r = convolve16_4tap_avx2(ss_256, coeffs);
  ss_256[0] = ss_256[1];
  return r;
}

static inline void xy_y_convolve_4tap_16_avx2(const __m256i *const ss,
                                              const __m256i coeffs[2],
                                              __m256i r[2]) {
  r[0] = convolve16_4tap_avx2(ss, coeffs);
  r[1] = convolve16_4tap_avx2(ss + 2, coeffs);
}

static inline void xy_y_convolve_4tap_8x2_avx2(const int16_t *const src,
                                               __m256i ss_256[4],
                                               const __m256i coeffs[2],
                                               __m256i r[2]) {
  __m256i s_256[2];
  s_256[0] = _mm256_loadu_si256((__m256i *)(src + 2 * 8));
  s_256[1] = _mm256_loadu_si256((__m256i *)(src + 3 * 8));
  ss_256[1] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  ss_256[3] = _mm256_unpackhi_epi16(s_256[0], s_256[1]);
  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r);
  ss_256[0] = ss_256[1];
  ss_256[2] = ss_256[3];
}

static inline void xy_y_convolve_4tap_8x2_half_pel_avx2(
    const int16_t *const src, const __m256i coeffs[1], __m256i s_256[4],
    __m256i r[2]) {
  __m256i a_256[2];
  s_256[2] = _mm256_loadu_si256((__m256i *)(src + 2 * 8));
  s_256[3] = _mm256_loadu_si256((__m256i *)(src + 3 * 8));
  a_256[0] = _mm256_add_epi16(s_256[0], s_256[3]);
  a_256[1] = _mm256_add_epi16(s_256[1], s_256[2]);
  xy_y_convolve_2tap_16_avx2(a_256[0], a_256[1], coeffs, r);
  s_256[0] = s_256[2];
  s_256[1] = s_256[3];
}

static inline void xy_y_convolve_4tap_16x2_avx2(
    const int16_t *const src, __m256i s_256[4], __m256i ss_256[4],
    __m256i tt_256[4], const __m256i coeffs[2], __m256i r[4]) {
  s_256[3] = _mm256_loadu_si256((__m256i *)(src + 3 * 16));
  ss_256[1] = _mm256_unpacklo_epi16(s_256[2], s_256[3]);
  ss_256[3] = _mm256_unpackhi_epi16(s_256[2], s_256[3]);
  s_256[2] = _mm256_loadu_si256((__m256i *)(src + 4 * 16));
  tt_256[1] = _mm256_unpacklo_epi16(s_256[3], s_256[2]);
  tt_256[3] = _mm256_unpackhi_epi16(s_256[3], s_256[2]);
  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r + 0);
  xy_y_convolve_4tap_16_avx2(tt_256, coeffs, r + 2);
  ss_256[0] = ss_256[1];
  ss_256[2] = ss_256[3];
  tt_256[0] = tt_256[1];
  tt_256[2] = tt_256[3];
}

static inline void xy_y_convolve_4tap_32x2_avx2(
    const int16_t *const src, const ptrdiff_t stride, __m256i s_256[4],
    __m256i ss_256[4], __m256i tt_256[4], const __m256i coeffs[2],
    __m256i r[4]) {
  s_256[3] = _mm256_loadu_si256((__m256i *)(src + 3 * stride));
  ss_256[1] = _mm256_unpacklo_epi16(s_256[2], s_256[3]);
  ss_256[3] = _mm256_unpackhi_epi16(s_256[2], s_256[3]);
  s_256[2] = _mm256_loadu_si256((__m256i *)(src + 4 * stride));
  tt_256[1] = _mm256_unpacklo_epi16(s_256[3], s_256[2]);
  tt_256[3] = _mm256_unpackhi_epi16(s_256[3], s_256[2]);
  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r + 0);
  xy_y_convolve_4tap_16_avx2(tt_256, coeffs, r + 2);
  ss_256[0] = ss_256[1];
  ss_256[2] = ss_256[3];
  tt_256[0] = tt_256[1];
  tt_256[2] = tt_256[3];
}

static inline void xy_y_convolve_4tap_16x2_half_pelavx2(
    const int16_t *const src, __m256i s_256[5], const __m256i coeffs[1],
    __m256i r[4]) {
  __m256i a_256[2];

  s_256[3] = _mm256_loadu_si256((__m256i *)(src + 3 * 16));
  s_256[4] = _mm256_loadu_si256((__m256i *)(src + 4 * 16));

  a_256[0] = _mm256_add_epi16(s_256[0], s_256[3]);
  a_256[1] = _mm256_add_epi16(s_256[1], s_256[2]);
  xy_y_convolve_2tap_16_avx2(a_256[0], a_256[1], coeffs, r + 0);

  a_256[0] = _mm256_add_epi16(s_256[1], s_256[4]);
  a_256[1] = _mm256_add_epi16(s_256[2], s_256[3]);
  xy_y_convolve_2tap_16_avx2(a_256[0], a_256[1], coeffs, r + 2);

  s_256[0] = s_256[2];
  s_256[1] = s_256[3];
  s_256[2] = s_256[4];
}

static inline __m128i xy_y_convolve_6tap_2x2_sse2(const int16_t *const src,
                                                  __m128i s_32[6],
                                                  __m128i ss_128[3],
                                                  const __m128i coeffs[3]) {
  s_32[5] = _mm_cvtsi32_si128(loadu_int32(src + 5 * 2));
  const __m128i src45 = _mm_unpacklo_epi32(s_32[4], s_32[5]);
  s_32[4] = _mm_cvtsi32_si128(loadu_int32(src + 6 * 2));
  const __m128i src56 = _mm_unpacklo_epi32(s_32[5], s_32[4]);
  ss_128[2] = _mm_unpacklo_epi16(src45, src56);
  const __m128i r = convolve16_6tap_sse2(ss_128, coeffs);
  ss_128[0] = ss_128[1];
  ss_128[1] = ss_128[2];
  return r;
}

static inline __m256i xy_y_convolve_6tap_4x2_avx2(const int16_t *const src,
                                                  __m128i s_64[6],
                                                  __m256i ss_256[3],
                                                  const __m256i coeffs[3]) {
  __m256i s_256[2];
  s_64[5] = _mm_loadl_epi64((__m128i *)(src + 5 * 4));
  s_256[0] = _mm256_setr_m128i(s_64[4], s_64[5]);
  s_64[4] = _mm_loadl_epi64((__m128i *)(src + 6 * 4));
  s_256[1] = _mm256_setr_m128i(s_64[5], s_64[4]);
  ss_256[2] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  const __m256i r = convolve16_6tap_avx2(ss_256, coeffs);
  ss_256[0] = ss_256[1];
  ss_256[1] = ss_256[2];
  return r;
}

static inline void xy_y_convolve_6tap_16_avx2(const __m256i ss[6],
                                              const __m256i coeffs[3],
                                              __m256i r[2]) {
  r[0] = convolve16_6tap_avx2(ss, coeffs);
  r[1] = convolve16_6tap_avx2(ss + 3, coeffs);
}

static inline void xy_y_convolve_6tap_8x2_avx2(const int16_t *const src,
                                               __m256i ss_256[6],
                                               const __m256i coeffs[3],
                                               __m256i r[2]) {
  __m256i s_256[2];
  s_256[0] = _mm256_loadu_si256((__m256i *)(src + 4 * 8));
  s_256[1] = _mm256_loadu_si256((__m256i *)(src + 5 * 8));
  ss_256[2] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  ss_256[5] = _mm256_unpackhi_epi16(s_256[0], s_256[1]);
  xy_y_convolve_6tap_16_avx2(ss_256, coeffs, r);
  ss_256[0] = ss_256[1];
  ss_256[1] = ss_256[2];
  ss_256[3] = ss_256[4];
  ss_256[4] = ss_256[5];
}

static inline void xy_y_convolve_6tap_8x2_half_pel_avx2(
    const int16_t *const src, const __m256i coeffs[2], __m256i s_256[6],
    __m256i r[2]) {
  __m256i a_256[2], ss_256[4];
  s_256[4] = _mm256_loadu_si256((__m256i *)(src + 4 * 8));
  s_256[5] = _mm256_loadu_si256((__m256i *)(src + 5 * 8));
  a_256[0] = _mm256_add_epi16(s_256[0], s_256[5]);
  a_256[1] = _mm256_add_epi16(s_256[1], s_256[4]);
  ss_256[0] = _mm256_unpacklo_epi16(a_256[0], a_256[1]);
  ss_256[1] = _mm256_unpacklo_epi16(s_256[2], s_256[3]);
  ss_256[2] = _mm256_unpackhi_epi16(a_256[0], a_256[1]);
  ss_256[3] = _mm256_unpackhi_epi16(s_256[2], s_256[3]);
  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r);
  s_256[0] = s_256[2];
  s_256[1] = s_256[3];
  s_256[2] = s_256[4];
  s_256[3] = s_256[5];
}

static inline void xy_y_convolve_6tap_16x2_avx2(
    const int16_t *const src, const ptrdiff_t stride, __m256i s_256[6],
    __m256i ss_256[6], __m256i tt_256[6], const __m256i coeffs[3],
    __m256i r[4]) {
  s_256[5] = _mm256_loadu_si256((__m256i *)(src + 5 * stride));
  ss_256[2] = _mm256_unpacklo_epi16(s_256[4], s_256[5]);
  ss_256[5] = _mm256_unpackhi_epi16(s_256[4], s_256[5]);
  s_256[4] = _mm256_loadu_si256((__m256i *)(src + 6 * stride));
  tt_256[2] = _mm256_unpacklo_epi16(s_256[5], s_256[4]);
  tt_256[5] = _mm256_unpackhi_epi16(s_256[5], s_256[4]);

  xy_y_convolve_6tap_16_avx2(ss_256, coeffs, r + 0);
  xy_y_convolve_6tap_16_avx2(tt_256, coeffs, r + 2);

  ss_256[0] = ss_256[1];
  ss_256[1] = ss_256[2];
  ss_256[3] = ss_256[4];
  ss_256[4] = ss_256[5];

  tt_256[0] = tt_256[1];
  tt_256[1] = tt_256[2];
  tt_256[3] = tt_256[4];
  tt_256[4] = tt_256[5];
}

static inline void xy_y_convolve_6tap_16x2_half_pel_avx2(
    const int16_t *const src, const ptrdiff_t stride, __m256i s_256[6],
    __m256i ss_256[4], const __m256i coeffs[2], __m256i r[4]) {
  __m256i a_256[2];

  s_256[5] = _mm256_loadu_si256((__m256i *)(src + 5 * stride));
  a_256[0] = _mm256_add_epi16(s_256[0], s_256[5]);
  a_256[1] = _mm256_add_epi16(s_256[1], s_256[4]);
  ss_256[0] = _mm256_unpacklo_epi16(a_256[0], a_256[1]);
  ss_256[1] = _mm256_unpacklo_epi16(s_256[2], s_256[3]);
  ss_256[2] = _mm256_unpackhi_epi16(a_256[0], a_256[1]);
  ss_256[3] = _mm256_unpackhi_epi16(s_256[2], s_256[3]);
  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r + 0);

  a_256[1] = _mm256_add_epi16(s_256[2], s_256[5]);
  s_256[0] = s_256[2];
  s_256[2] = s_256[4];
  s_256[4] = _mm256_loadu_si256((__m256i *)(src + 6 * stride));
  a_256[0] = _mm256_add_epi16(s_256[1], s_256[4]);
  s_256[1] = s_256[3];
  s_256[3] = s_256[5];
  ss_256[0] = _mm256_unpacklo_epi16(a_256[0], a_256[1]);
  ss_256[1] = _mm256_unpacklo_epi16(s_256[1], s_256[2]);
  ss_256[2] = _mm256_unpackhi_epi16(a_256[0], a_256[1]);
  ss_256[3] = _mm256_unpackhi_epi16(s_256[1], s_256[2]);
  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r + 2);
}

static inline __m128i xy_y_convolve_8tap_2x2_sse2(const int16_t *const src,
                                                  __m128i s_32[8],
                                                  __m128i ss_128[4],
                                                  const __m128i coeffs[4]) {
  s_32[7] = _mm_cvtsi32_si128(loadu_int32(src + 7 * 2));
  const __m128i src67 = _mm_unpacklo_epi32(s_32[6], s_32[7]);
  s_32[6] = _mm_cvtsi32_si128(loadu_int32(src + 8 * 2));
  const __m128i src78 = _mm_unpacklo_epi32(s_32[7], s_32[6]);
  ss_128[3] = _mm_unpacklo_epi16(src67, src78);
  const __m128i r = convolve16_8tap_sse2(ss_128, coeffs);
  ss_128[0] = ss_128[1];
  ss_128[1] = ss_128[2];
  ss_128[2] = ss_128[3];
  return r;
}

static inline __m256i xy_y_convolve_8tap_4x2_avx2(const int16_t *const src,
                                                  __m128i s_64[8],
                                                  __m256i ss_256[4],
                                                  const __m256i coeffs[4]) {
  __m256i s_256[2];
  s_64[7] = _mm_loadl_epi64((__m128i *)(src + 7 * 4));
  s_256[0] = _mm256_setr_m128i(s_64[6], s_64[7]);
  s_64[6] = _mm_loadl_epi64((__m128i *)(src + 8 * 4));
  s_256[1] = _mm256_setr_m128i(s_64[7], s_64[6]);
  ss_256[3] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  const __m256i r = convolve16_8tap_avx2(ss_256, coeffs);
  ss_256[0] = ss_256[1];
  ss_256[1] = ss_256[2];
  ss_256[2] = ss_256[3];
  return r;
}

static inline void xy_y_convolve_8tap_16_avx2(const __m256i *const ss,
                                              const __m256i coeffs[4],
                                              __m256i r[2]) {
  r[0] = convolve16_8tap_avx2(ss, coeffs);
  r[1] = convolve16_8tap_avx2(ss + 4, coeffs);
}

static inline void xy_y_convolve_8tap_8x2_avx2(const int16_t *const src,
                                               __m256i ss_256[8],
                                               const __m256i coeffs[4],
                                               __m256i r[2]) {
  __m256i s_256[2];
  s_256[0] = _mm256_loadu_si256((__m256i *)(src + 6 * 8));
  s_256[1] = _mm256_loadu_si256((__m256i *)(src + 7 * 8));
  ss_256[3] = _mm256_unpacklo_epi16(s_256[0], s_256[1]);
  ss_256[7] = _mm256_unpackhi_epi16(s_256[0], s_256[1]);
  xy_y_convolve_8tap_16_avx2(ss_256, coeffs, r);
  ss_256[0] = ss_256[1];
  ss_256[1] = ss_256[2];
  ss_256[2] = ss_256[3];
  ss_256[4] = ss_256[5];
  ss_256[5] = ss_256[6];
  ss_256[6] = ss_256[7];
}

static inline void xy_y_convolve_8tap_8x2_half_pel_avx2(
    const int16_t *const src, const __m256i coeffs[2], __m256i s_256[8],
    __m256i r[2]) {
  __m256i a_256[4], ss_256[4];

  s_256[6] = _mm256_loadu_si256((__m256i *)(src + 6 * 8));
  s_256[7] = _mm256_loadu_si256((__m256i *)(src + 7 * 8));
  a_256[0] = _mm256_add_epi16(s_256[0], s_256[7]);
  a_256[1] = _mm256_add_epi16(s_256[1], s_256[6]);
  a_256[2] = _mm256_add_epi16(s_256[2], s_256[5]);
  a_256[3] = _mm256_add_epi16(s_256[3], s_256[4]);
  ss_256[0] = _mm256_unpacklo_epi16(a_256[0], a_256[1]);
  ss_256[1] = _mm256_unpacklo_epi16(a_256[2], a_256[3]);
  ss_256[2] = _mm256_unpackhi_epi16(a_256[0], a_256[1]);
  ss_256[3] = _mm256_unpackhi_epi16(a_256[2], a_256[3]);
  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r);
  s_256[0] = s_256[2];
  s_256[1] = s_256[3];
  s_256[2] = s_256[4];
  s_256[3] = s_256[5];
  s_256[4] = s_256[6];
  s_256[5] = s_256[7];
}

static AOM_FORCE_INLINE void xy_y_convolve_8tap_16x2_avx2(
    const int16_t *const src, const ptrdiff_t stride, const __m256i coeffs[4],
    __m256i s_256[8], __m256i ss_256[8], __m256i tt_256[8], __m256i r[4]) {
  s_256[7] = _mm256_loadu_si256((__m256i *)(src + 7 * stride));
  ss_256[3] = _mm256_unpacklo_epi16(s_256[6], s_256[7]);
  ss_256[7] = _mm256_unpackhi_epi16(s_256[6], s_256[7]);
  s_256[6] = _mm256_loadu_si256((__m256i *)(src + 8 * stride));
  tt_256[3] = _mm256_unpacklo_epi16(s_256[7], s_256[6]);
  tt_256[7] = _mm256_unpackhi_epi16(s_256[7], s_256[6]);

  xy_y_convolve_8tap_16_avx2(ss_256, coeffs, r + 0);
  xy_y_convolve_8tap_16_avx2(tt_256, coeffs, r + 2);

  ss_256[0] = ss_256[1];
  ss_256[1] = ss_256[2];
  ss_256[2] = ss_256[3];
  ss_256[4] = ss_256[5];
  ss_256[5] = ss_256[6];
  ss_256[6] = ss_256[7];

  tt_256[0] = tt_256[1];
  tt_256[1] = tt_256[2];
  tt_256[2] = tt_256[3];
  tt_256[4] = tt_256[5];
  tt_256[5] = tt_256[6];
  tt_256[6] = tt_256[7];
}

static inline void xy_y_convolve_8tap_16x2_half_pel_avx2(
    const int16_t *const src, const ptrdiff_t stride, const __m256i coeffs[4],
    __m256i s_256[8], __m256i r[4]) {
  __m256i a_256[4], ss_256[4];
  s_256[7] = _mm256_loadu_si256((__m256i *)(src + 7 * stride));

  a_256[0] = _mm256_add_epi16(s_256[0], s_256[7]);
  a_256[1] = _mm256_add_epi16(s_256[1], s_256[6]);
  a_256[2] = _mm256_add_epi16(s_256[2], s_256[5]);
  a_256[3] = _mm256_add_epi16(s_256[3], s_256[4]);
  ss_256[0] = _mm256_unpacklo_epi16(a_256[0], a_256[1]);
  ss_256[1] = _mm256_unpacklo_epi16(a_256[2], a_256[3]);
  ss_256[2] = _mm256_unpackhi_epi16(a_256[0], a_256[1]);
  ss_256[3] = _mm256_unpackhi_epi16(a_256[2], a_256[3]);

  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r + 0);

  a_256[1] = _mm256_add_epi16(s_256[2], s_256[7]);
  a_256[2] = _mm256_add_epi16(s_256[3], s_256[6]);
  a_256[3] = _mm256_add_epi16(s_256[4], s_256[5]);
  s_256[0] = s_256[2];
  s_256[2] = s_256[4];
  s_256[4] = s_256[6];
  s_256[6] = _mm256_loadu_si256((__m256i *)(src + 8 * stride));

  a_256[0] = _mm256_add_epi16(s_256[1], s_256[6]);
  s_256[1] = s_256[3];
  s_256[3] = s_256[5];
  s_256[5] = s_256[7];
  ss_256[0] = _mm256_unpacklo_epi16(a_256[0], a_256[1]);
  ss_256[1] = _mm256_unpacklo_epi16(a_256[2], a_256[3]);
  ss_256[2] = _mm256_unpackhi_epi16(a_256[0], a_256[1]);
  ss_256[3] = _mm256_unpackhi_epi16(a_256[2], a_256[3]);

  xy_y_convolve_4tap_16_avx2(ss_256, coeffs, r + 2);
}

static inline void xy_y_round_store_8x2_avx2(const __m256i res[2],
                                             uint8_t *const dst,
                                             const ptrdiff_t stride) {
  const __m256i r = xy_y_round_16_avx2(res);
  pack_store_8x2_avx2(r, dst, stride);
}

static inline void xy_y_round_store_16x2_avx2(const __m256i res[4],
                                              uint8_t *const dst,
                                              const ptrdiff_t stride) {
  const __m256i r0 = xy_y_round_16_avx2(res + 0);
  const __m256i r1 = xy_y_round_16_avx2(res + 2);
  xy_y_pack_store_16x2_avx2(r0, r1, dst, stride);
}

static inline void sr_y_round_store_32_avx2(const __m256i res[2],
                                            uint8_t *const dst) {
  __m256i r[2];

  r[0] = sr_y_round_avx2(res[0]);
  r[1] = sr_y_round_avx2(res[1]);
  convolve_store_32_avx2(r[0], r[1], dst);
}

static inline void sr_y_round_store_32x2_avx2(const __m256i res[4],
                                              uint8_t *const dst,
                                              const int32_t dst_stride) {
  sr_y_round_store_32_avx2(res, dst);
  sr_y_round_store_32_avx2(res + 2, dst + dst_stride);
}

static inline void sr_y_2tap_32_avx2(const uint8_t *const src,
                                     const __m256i coeffs[1], const __m256i s0,
                                     __m256i *const s1, uint8_t *const dst) {
  __m256i r[2];
  y_convolve_2tap_32_avx2(src, coeffs, s0, s1, r);
  sr_y_round_store_32_avx2(r, dst);
}

static AOM_FORCE_INLINE void av1_convolve_y_sr_specialized_avx2(
    const uint8_t *src, int32_t src_stride, uint8_t *dst, int32_t dst_stride,
    int32_t w, int32_t h, const InterpFilterParams *filter_params_y,
    const int32_t subpel_y_q4) {
  int32_t x, y;
  __m128i coeffs_128[4];
  __m256i coeffs_256[4];

  int vert_tap = get_filter_tap(filter_params_y, subpel_y_q4);

  if (vert_tap == 2) {
    // vert_filt as 2 tap
    const uint8_t *src_ptr = src;

    y = h;

    if (subpel_y_q4 != 8) {
      if (w <= 8) {
        prepare_half_coeffs_2tap_ssse3(filter_params_y, subpel_y_q4,
                                       coeffs_128);

        if (w == 2) {
          __m128i s_16[2];

          s_16[0] = _mm_cvtsi32_si128(*(int16_t *)src_ptr);

          do {
            const __m128i res = y_convolve_2tap_2x2_ssse3(src_ptr, src_stride,
                                                          coeffs_128, s_16);
            const __m128i r = sr_y_round_sse2(res);
            pack_store_2x2_sse2(r, dst, dst_stride);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else if (w == 4) {
          __m128i s_32[2];

          s_32[0] = _mm_cvtsi32_si128(loadu_int32(src_ptr));

          do {
            const __m128i res = y_convolve_2tap_4x2_ssse3(src_ptr, src_stride,
                                                          coeffs_128, s_32);
            const __m128i r = sr_y_round_sse2(res);
            pack_store_4x2_sse2(r, dst, dst_stride);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else {
          __m128i s_64[2], s_128[2];

          assert(w == 8);

          s_64[0] = _mm_loadl_epi64((__m128i *)src_ptr);

          do {
            // Note: Faster than binding to AVX2 registers.
            s_64[1] = _mm_loadl_epi64((__m128i *)(src_ptr + src_stride));
            s_128[0] = _mm_unpacklo_epi64(s_64[0], s_64[1]);
            s_64[0] = _mm_loadl_epi64((__m128i *)(src_ptr + 2 * src_stride));
            s_128[1] = _mm_unpacklo_epi64(s_64[1], s_64[0]);
            const __m128i ss0 = _mm_unpacklo_epi8(s_128[0], s_128[1]);
            const __m128i ss1 = _mm_unpackhi_epi8(s_128[0], s_128[1]);
            const __m128i res0 = convolve_2tap_ssse3(&ss0, coeffs_128);
            const __m128i res1 = convolve_2tap_ssse3(&ss1, coeffs_128);
            const __m128i r0 = sr_y_round_sse2(res0);
            const __m128i r1 = sr_y_round_sse2(res1);
            const __m128i d = _mm_packus_epi16(r0, r1);
            _mm_storel_epi64((__m128i *)dst, d);
            _mm_storeh_epi64((__m128i *)(dst + dst_stride), d);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        }
      } else {
        prepare_half_coeffs_2tap_avx2(filter_params_y, subpel_y_q4, coeffs_256);

        if (w == 16) {
          __m128i s_128[2];

          s_128[0] = _mm_loadu_si128((__m128i *)src_ptr);

          do {
            __m256i r[2];

            y_convolve_2tap_16x2_avx2(src_ptr, src_stride, coeffs_256, s_128,
                                      r);
            sr_y_round_store_16x2_avx2(r, dst, dst_stride);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else if (w == 32) {
          __m256i s_256[2];

          s_256[0] = _mm256_loadu_si256((__m256i *)src_ptr);

          do {
            sr_y_2tap_32_avx2(src_ptr + src_stride, coeffs_256, s_256[0],
                              &s_256[1], dst);
            sr_y_2tap_32_avx2(src_ptr + 2 * src_stride, coeffs_256, s_256[1],
                              &s_256[0], dst + dst_stride);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else if (w == 64) {
          __m256i s_256[2][2];

          s_256[0][0] = _mm256_loadu_si256((__m256i *)(src_ptr + 0 * 32));
          s_256[0][1] = _mm256_loadu_si256((__m256i *)(src_ptr + 1 * 32));

          do {
            sr_y_2tap_32_avx2(src_ptr + src_stride, coeffs_256, s_256[0][0],
                              &s_256[1][0], dst);
            sr_y_2tap_32_avx2(src_ptr + src_stride + 32, coeffs_256,
                              s_256[0][1], &s_256[1][1], dst + 32);
            sr_y_2tap_32_avx2(src_ptr + 2 * src_stride, coeffs_256, s_256[1][0],
                              &s_256[0][0], dst + dst_stride);
            sr_y_2tap_32_avx2(src_ptr + 2 * src_stride + 32, coeffs_256,
                              s_256[1][1], &s_256[0][1], dst + dst_stride + 32);

            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else {
          __m256i s_256[2][4];

          assert(w == 128);

          s_256[0][0] = _mm256_loadu_si256((__m256i *)(src_ptr + 0 * 32));
          s_256[0][1] = _mm256_loadu_si256((__m256i *)(src_ptr + 1 * 32));
          s_256[0][2] = _mm256_loadu_si256((__m256i *)(src_ptr + 2 * 32));
          s_256[0][3] = _mm256_loadu_si256((__m256i *)(src_ptr + 3 * 32));

          do {
            sr_y_2tap_32_avx2(src_ptr + src_stride, coeffs_256, s_256[0][0],
                              &s_256[1][0], dst);
            sr_y_2tap_32_avx2(src_ptr + src_stride + 1 * 32, coeffs_256,
                              s_256[0][1], &s_256[1][1], dst + 1 * 32);
            sr_y_2tap_32_avx2(src_ptr + src_stride + 2 * 32, coeffs_256,
                              s_256[0][2], &s_256[1][2], dst + 2 * 32);
            sr_y_2tap_32_avx2(src_ptr + src_stride + 3 * 32, coeffs_256,
                              s_256[0][3], &s_256[1][3], dst + 3 * 32);

            sr_y_2tap_32_avx2(src_ptr + 2 * src_stride, coeffs_256, s_256[1][0],
                              &s_256[0][0], dst + dst_stride);
            sr_y_2tap_32_avx2(src_ptr + 2 * src_stride + 1 * 32, coeffs_256,
                              s_256[1][1], &s_256[0][1],
                              dst + dst_stride + 1 * 32);
            sr_y_2tap_32_avx2(src_ptr + 2 * src_stride + 2 * 32, coeffs_256,
                              s_256[1][2], &s_256[0][2],
                              dst + dst_stride + 2 * 32);
            sr_y_2tap_32_avx2(src_ptr + 2 * src_stride + 3 * 32, coeffs_256,
                              s_256[1][3], &s_256[0][3],
                              dst + dst_stride + 3 * 32);

            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        }
      }
    } else {
      // average to get half pel
      if (w <= 8) {
        if (w == 2) {
          __m128i s_16[2];

          s_16[0] = _mm_cvtsi32_si128(*(int16_t *)src_ptr);

          do {
            s_16[1] = _mm_cvtsi32_si128(*(int16_t *)(src_ptr + src_stride));
            const __m128i d0 = _mm_avg_epu8(s_16[0], s_16[1]);
            *(int16_t *)dst = (int16_t)_mm_cvtsi128_si32(d0);
            s_16[0] = _mm_cvtsi32_si128(*(int16_t *)(src_ptr + 2 * src_stride));
            const __m128i d1 = _mm_avg_epu8(s_16[1], s_16[0]);
            *(int16_t *)(dst + dst_stride) = (int16_t)_mm_cvtsi128_si32(d1);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else if (w == 4) {
          __m128i s_32[2];

          s_32[0] = _mm_cvtsi32_si128(loadu_int32(src_ptr));

          do {
            s_32[1] = _mm_cvtsi32_si128(loadu_int32(src_ptr + src_stride));
            const __m128i d0 = _mm_avg_epu8(s_32[0], s_32[1]);
            xx_storel_32(dst, d0);
            s_32[0] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 2 * src_stride));
            const __m128i d1 = _mm_avg_epu8(s_32[1], s_32[0]);
            xx_storel_32(dst + dst_stride, d1);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else {
          __m128i s_64[2];

          assert(w == 8);

          s_64[0] = _mm_loadl_epi64((__m128i *)src_ptr);

          do {
            // Note: Faster than binding to AVX2 registers.
            s_64[1] = _mm_loadl_epi64((__m128i *)(src_ptr + src_stride));
            const __m128i d0 = _mm_avg_epu8(s_64[0], s_64[1]);
            _mm_storel_epi64((__m128i *)dst, d0);
            s_64[0] = _mm_loadl_epi64((__m128i *)(src_ptr + 2 * src_stride));
            const __m128i d1 = _mm_avg_epu8(s_64[1], s_64[0]);
            _mm_storel_epi64((__m128i *)(dst + dst_stride), d1);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        }
      } else if (w == 16) {
        __m128i s_128[2];

        s_128[0] = _mm_loadu_si128((__m128i *)src_ptr);

        do {
          s_128[1] = _mm_loadu_si128((__m128i *)(src_ptr + src_stride));
          const __m128i d0 = _mm_avg_epu8(s_128[0], s_128[1]);
          _mm_storeu_si128((__m128i *)dst, d0);
          s_128[0] = _mm_loadu_si128((__m128i *)(src_ptr + 2 * src_stride));
          const __m128i d1 = _mm_avg_epu8(s_128[1], s_128[0]);
          _mm_storeu_si128((__m128i *)(dst + dst_stride), d1);
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 32) {
        __m256i s_256[2];

        s_256[0] = _mm256_loadu_si256((__m256i *)src_ptr);

        do {
          sr_y_2tap_32_avg_avx2(src_ptr + src_stride, s_256[0], &s_256[1], dst);
          sr_y_2tap_32_avg_avx2(src_ptr + 2 * src_stride, s_256[1], &s_256[0],
                                dst + dst_stride);
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 64) {
        __m256i s_256[2][2];

        s_256[0][0] = _mm256_loadu_si256((__m256i *)(src_ptr + 0 * 32));
        s_256[0][1] = _mm256_loadu_si256((__m256i *)(src_ptr + 1 * 32));

        do {
          sr_y_2tap_32_avg_avx2(src_ptr + src_stride, s_256[0][0], &s_256[1][0],
                                dst);
          sr_y_2tap_32_avg_avx2(src_ptr + src_stride + 32, s_256[0][1],
                                &s_256[1][1], dst + 32);

          sr_y_2tap_32_avg_avx2(src_ptr + 2 * src_stride, s_256[1][0],
                                &s_256[0][0], dst + dst_stride);
          sr_y_2tap_32_avg_avx2(src_ptr + 2 * src_stride + 32, s_256[1][1],
                                &s_256[0][1], dst + dst_stride + 32);

          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else {
        __m256i s_256[2][4];

        assert(w == 128);

        s_256[0][0] = _mm256_loadu_si256((__m256i *)(src_ptr + 0 * 32));
        s_256[0][1] = _mm256_loadu_si256((__m256i *)(src_ptr + 1 * 32));
        s_256[0][2] = _mm256_loadu_si256((__m256i *)(src_ptr + 2 * 32));
        s_256[0][3] = _mm256_loadu_si256((__m256i *)(src_ptr + 3 * 32));

        do {
          sr_y_2tap_32_avg_avx2(src_ptr + src_stride, s_256[0][0], &s_256[1][0],
                                dst);
          sr_y_2tap_32_avg_avx2(src_ptr + src_stride + 1 * 32, s_256[0][1],
                                &s_256[1][1], dst + 1 * 32);
          sr_y_2tap_32_avg_avx2(src_ptr + src_stride + 2 * 32, s_256[0][2],
                                &s_256[1][2], dst + 2 * 32);
          sr_y_2tap_32_avg_avx2(src_ptr + src_stride + 3 * 32, s_256[0][3],
                                &s_256[1][3], dst + 3 * 32);

          sr_y_2tap_32_avg_avx2(src_ptr + 2 * src_stride, s_256[1][0],
                                &s_256[0][0], dst + dst_stride);
          sr_y_2tap_32_avg_avx2(src_ptr + 2 * src_stride + 1 * 32, s_256[1][1],
                                &s_256[0][1], dst + dst_stride + 1 * 32);
          sr_y_2tap_32_avg_avx2(src_ptr + 2 * src_stride + 2 * 32, s_256[1][2],
                                &s_256[0][2], dst + dst_stride + 2 * 32);
          sr_y_2tap_32_avg_avx2(src_ptr + 2 * src_stride + 3 * 32, s_256[1][3],
                                &s_256[0][3], dst + dst_stride + 3 * 32);

          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      }
    }
  } else if (vert_tap == 4) {
    // vert_filt as 4 tap
    const uint8_t *src_ptr = src - src_stride;

    y = h;

    if (w <= 4) {
      prepare_half_coeffs_4tap_ssse3(filter_params_y, subpel_y_q4, coeffs_128);

      if (w == 2) {
        __m128i s_16[4], ss_128[2];

        s_16[0] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 0 * src_stride));
        s_16[1] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 1 * src_stride));
        s_16[2] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 2 * src_stride));

        const __m128i src01 = _mm_unpacklo_epi16(s_16[0], s_16[1]);
        const __m128i src12 = _mm_unpacklo_epi16(s_16[1], s_16[2]);

        ss_128[0] = _mm_unpacklo_epi8(src01, src12);

        do {
          src_ptr += 2 * src_stride;
          const __m128i res = y_convolve_4tap_2x2_ssse3(
              src_ptr, src_stride, coeffs_128, s_16, ss_128);
          const __m128i r = sr_y_round_sse2(res);
          pack_store_2x2_sse2(r, dst, dst_stride);

          ss_128[0] = ss_128[1];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else {
        __m128i s_32[4], ss_128[2];

        assert(w == 4);

        s_32[0] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 0 * src_stride));
        s_32[1] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 1 * src_stride));
        s_32[2] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 2 * src_stride));

        const __m128i src01 = _mm_unpacklo_epi32(s_32[0], s_32[1]);
        const __m128i src12 = _mm_unpacklo_epi32(s_32[1], s_32[2]);

        ss_128[0] = _mm_unpacklo_epi8(src01, src12);

        do {
          src_ptr += 2 * src_stride;
          const __m128i res = y_convolve_4tap_4x2_ssse3(
              src_ptr, src_stride, coeffs_128, s_32, ss_128);
          const __m128i r = sr_y_round_sse2(res);
          pack_store_4x2_sse2(r, dst, dst_stride);

          ss_128[0] = ss_128[1];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      }
    } else {
      prepare_half_coeffs_4tap_avx2(filter_params_y, subpel_y_q4, coeffs_256);

      if (w == 8) {
        __m128i s_64[4];
        __m256i ss_256[2];

        s_64[0] = _mm_loadl_epi64((__m128i *)(src_ptr + 0 * src_stride));
        s_64[1] = _mm_loadl_epi64((__m128i *)(src_ptr + 1 * src_stride));
        s_64[2] = _mm_loadl_epi64((__m128i *)(src_ptr + 2 * src_stride));

        // Load lines a and b. Line a to lower 128, line b to upper 128
        const __m256i src01 = _mm256_setr_m128i(s_64[0], s_64[1]);
        const __m256i src12 = _mm256_setr_m128i(s_64[1], s_64[2]);

        ss_256[0] = _mm256_unpacklo_epi8(src01, src12);

        do {
          src_ptr += 2 * src_stride;
          const __m256i res = y_convolve_4tap_8x2_avx2(
              src_ptr, src_stride, coeffs_256, s_64, ss_256);
          sr_y_round_store_8x2_avx2(res, dst, dst_stride);

          ss_256[0] = ss_256[1];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 16) {
        __m128i s_128[4];
        __m256i ss_256[4], r[2];

        s_128[0] = _mm_loadu_si128((__m128i *)(src_ptr + 0 * src_stride));
        s_128[1] = _mm_loadu_si128((__m128i *)(src_ptr + 1 * src_stride));
        s_128[2] = _mm_loadu_si128((__m128i *)(src_ptr + 2 * src_stride));

        // Load lines a and b. Line a to lower 128, line b to upper 128
        const __m256i src01 = _mm256_setr_m128i(s_128[0], s_128[1]);
        const __m256i src12 = _mm256_setr_m128i(s_128[1], s_128[2]);

        ss_256[0] = _mm256_unpacklo_epi8(src01, src12);
        ss_256[2] = _mm256_unpackhi_epi8(src01, src12);

        do {
          src_ptr += 2 * src_stride;
          y_convolve_4tap_16x2_avx2(src_ptr, src_stride, coeffs_256, s_128,
                                    ss_256, r);
          sr_y_round_store_16x2_avx2(r, dst, dst_stride);

          ss_256[0] = ss_256[1];
          ss_256[2] = ss_256[3];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 32) {
        // AV1 standard won't have 32x4 case.
        // This only favors some optimization feature which
        // subsamples 32x8 to 32x4 and triggers 4-tap filter.

        __m256i s_256[4], ss_256[4], tt_256[4], r[4];

        s_256[0] = _mm256_loadu_si256((__m256i *)(src_ptr + 0 * src_stride));
        s_256[1] = _mm256_loadu_si256((__m256i *)(src_ptr + 1 * src_stride));
        s_256[2] = _mm256_loadu_si256((__m256i *)(src_ptr + 2 * src_stride));

        ss_256[0] = _mm256_unpacklo_epi8(s_256[0], s_256[1]);
        ss_256[2] = _mm256_unpackhi_epi8(s_256[0], s_256[1]);

        tt_256[0] = _mm256_unpacklo_epi8(s_256[1], s_256[2]);
        tt_256[2] = _mm256_unpackhi_epi8(s_256[1], s_256[2]);

        do {
          src_ptr += 2 * src_stride;
          y_convolve_4tap_32x2_avx2(src_ptr, src_stride, coeffs_256, s_256,
                                    ss_256, tt_256, r);
          sr_y_round_store_32x2_avx2(r, dst, dst_stride);

          ss_256[0] = ss_256[1];
          ss_256[2] = ss_256[3];

          tt_256[0] = tt_256[1];
          tt_256[2] = tt_256[3];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else {
        assert(!(w % 32));

        __m256i s_256[4], ss_256[4], tt_256[4], r[4];
        x = 0;
        do {
          const uint8_t *s = src_ptr + x;
          uint8_t *d = dst + x;
          s_256[0] = _mm256_loadu_si256((__m256i *)(s + 0 * src_stride));
          s_256[1] = _mm256_loadu_si256((__m256i *)(s + 1 * src_stride));
          s_256[2] = _mm256_loadu_si256((__m256i *)(s + 2 * src_stride));

          ss_256[0] = _mm256_unpacklo_epi8(s_256[0], s_256[1]);
          ss_256[2] = _mm256_unpackhi_epi8(s_256[0], s_256[1]);

          tt_256[0] = _mm256_unpacklo_epi8(s_256[1], s_256[2]);
          tt_256[2] = _mm256_unpackhi_epi8(s_256[1], s_256[2]);

          y = h;
          do {
            s += 2 * src_stride;
            y_convolve_4tap_32x2_avx2(s, src_stride, coeffs_256, s_256, ss_256,
                                      tt_256, r);
            sr_y_round_store_32x2_avx2(r, d, dst_stride);

            ss_256[0] = ss_256[1];
            ss_256[2] = ss_256[3];

            tt_256[0] = tt_256[1];
            tt_256[2] = tt_256[3];
            d += 2 * dst_stride;
            y -= 2;
          } while (y);
          x += 32;
        } while (x < w);
      }
    }
  } else if (vert_tap == 6) {
    // vert_filt as 6 tap
    const uint8_t *src_ptr = src - 2 * src_stride;

    if (w <= 4) {
      prepare_half_coeffs_6tap_ssse3(filter_params_y, subpel_y_q4, coeffs_128);

      y = h;

      if (w == 2) {
        __m128i s_16[6], ss_128[3];

        s_16[0] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 0 * src_stride));
        s_16[1] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 1 * src_stride));
        s_16[2] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 2 * src_stride));
        s_16[3] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 3 * src_stride));
        s_16[4] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 4 * src_stride));

        const __m128i src01 = _mm_unpacklo_epi16(s_16[0], s_16[1]);
        const __m128i src12 = _mm_unpacklo_epi16(s_16[1], s_16[2]);
        const __m128i src23 = _mm_unpacklo_epi16(s_16[2], s_16[3]);
        const __m128i src34 = _mm_unpacklo_epi16(s_16[3], s_16[4]);

        ss_128[0] = _mm_unpacklo_epi8(src01, src12);
        ss_128[1] = _mm_unpacklo_epi8(src23, src34);

        do {
          src_ptr += 2 * src_stride;
          const __m128i res = y_convolve_6tap_2x2_ssse3(
              src_ptr, src_stride, coeffs_128, s_16, ss_128);
          const __m128i r = sr_y_round_sse2(res);
          pack_store_2x2_sse2(r, dst, dst_stride);

          ss_128[0] = ss_128[1];
          ss_128[1] = ss_128[2];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else {
        __m128i s_32[6], ss_128[3];

        assert(w == 4);

        s_32[0] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 0 * src_stride));
        s_32[1] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 1 * src_stride));
        s_32[2] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 2 * src_stride));
        s_32[3] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 3 * src_stride));
        s_32[4] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 4 * src_stride));

        const __m128i src01 = _mm_unpacklo_epi32(s_32[0], s_32[1]);
        const __m128i src12 = _mm_unpacklo_epi32(s_32[1], s_32[2]);
        const __m128i src23 = _mm_unpacklo_epi32(s_32[2], s_32[3]);
        const __m128i src34 = _mm_unpacklo_epi32(s_32[3], s_32[4]);

        ss_128[0] = _mm_unpacklo_epi8(src01, src12);
        ss_128[1] = _mm_unpacklo_epi8(src23, src34);

        do {
          src_ptr += 2 * src_stride;
          const __m128i res = y_convolve_6tap_4x2_ssse3(
              src_ptr, src_stride, coeffs_128, s_32, ss_128);
          const __m128i r = sr_y_round_sse2(res);
          pack_store_4x2_sse2(r, dst, dst_stride);

          ss_128[0] = ss_128[1];
          ss_128[1] = ss_128[2];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      }
    } else {
      prepare_half_coeffs_6tap_avx2(filter_params_y, subpel_y_q4, coeffs_256);

      if (w == 8) {
        __m128i s_64[6];
        __m256i ss_256[3];

        s_64[0] = _mm_loadl_epi64((__m128i *)(src_ptr + 0 * src_stride));
        s_64[1] = _mm_loadl_epi64((__m128i *)(src_ptr + 1 * src_stride));
        s_64[2] = _mm_loadl_epi64((__m128i *)(src_ptr + 2 * src_stride));
        s_64[3] = _mm_loadl_epi64((__m128i *)(src_ptr + 3 * src_stride));
        s_64[4] = _mm_loadl_epi64((__m128i *)(src_ptr + 4 * src_stride));

        // Load lines a and b. Line a to lower 128, line b to upper 128
        const __m256i src01 = _mm256_setr_m128i(s_64[0], s_64[1]);
        const __m256i src12 = _mm256_setr_m128i(s_64[1], s_64[2]);
        const __m256i src23 = _mm256_setr_m128i(s_64[2], s_64[3]);
        const __m256i src34 = _mm256_setr_m128i(s_64[3], s_64[4]);

        ss_256[0] = _mm256_unpacklo_epi8(src01, src12);
        ss_256[1] = _mm256_unpacklo_epi8(src23, src34);

        y = h;
        do {
          src_ptr += 2 * src_stride;
          const __m256i res = y_convolve_6tap_8x2_avx2(
              src_ptr, src_stride, coeffs_256, s_64, ss_256);
          sr_y_round_store_8x2_avx2(res, dst, dst_stride);

          ss_256[0] = ss_256[1];
          ss_256[1] = ss_256[2];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 16) {
        __m128i s_128[6];
        __m256i ss_256[6], r[2];

        s_128[0] = _mm_loadu_si128((__m128i *)(src_ptr + 0 * src_stride));
        s_128[1] = _mm_loadu_si128((__m128i *)(src_ptr + 1 * src_stride));
        s_128[2] = _mm_loadu_si128((__m128i *)(src_ptr + 2 * src_stride));
        s_128[3] = _mm_loadu_si128((__m128i *)(src_ptr + 3 * src_stride));
        s_128[4] = _mm_loadu_si128((__m128i *)(src_ptr + 4 * src_stride));

        // Load lines a and b. Line a to lower 128, line b to upper 128
        const __m256i src01 = _mm256_setr_m128i(s_128[0], s_128[1]);
        const __m256i src12 = _mm256_setr_m128i(s_128[1], s_128[2]);
        const __m256i src23 = _mm256_setr_m128i(s_128[2], s_128[3]);
        const __m256i src34 = _mm256_setr_m128i(s_128[3], s_128[4]);

        ss_256[0] = _mm256_unpacklo_epi8(src01, src12);
        ss_256[1] = _mm256_unpacklo_epi8(src23, src34);

        ss_256[3] = _mm256_unpackhi_epi8(src01, src12);
        ss_256[4] = _mm256_unpackhi_epi8(src23, src34);

        y = h;
        do {
          src_ptr += 2 * src_stride;
          y_convolve_6tap_16x2_avx2(src_ptr, src_stride, coeffs_256, s_128,
                                    ss_256, r);
          sr_y_round_store_16x2_avx2(r, dst, dst_stride);

          ss_256[0] = ss_256[1];
          ss_256[1] = ss_256[2];

          ss_256[3] = ss_256[4];
          ss_256[4] = ss_256[5];
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else {
        __m256i s_256[6], ss_256[6], tt_256[6], r[4];

        assert(!(w % 32));

        x = 0;
        do {
          const uint8_t *s = src_ptr + x;
          uint8_t *d = dst + x;

          s_256[0] = _mm256_loadu_si256((__m256i *)(s + 0 * src_stride));
          s_256[1] = _mm256_loadu_si256((__m256i *)(s + 1 * src_stride));
          s_256[2] = _mm256_loadu_si256((__m256i *)(s + 2 * src_stride));
          s_256[3] = _mm256_loadu_si256((__m256i *)(s + 3 * src_stride));
          s_256[4] = _mm256_loadu_si256((__m256i *)(s + 4 * src_stride));

          ss_256[0] = _mm256_unpacklo_epi8(s_256[0], s_256[1]);
          ss_256[1] = _mm256_unpacklo_epi8(s_256[2], s_256[3]);
          ss_256[3] = _mm256_unpackhi_epi8(s_256[0], s_256[1]);
          ss_256[4] = _mm256_unpackhi_epi8(s_256[2], s_256[3]);

          tt_256[0] = _mm256_unpacklo_epi8(s_256[1], s_256[2]);
          tt_256[1] = _mm256_unpacklo_epi8(s_256[3], s_256[4]);
          tt_256[3] = _mm256_unpackhi_epi8(s_256[1], s_256[2]);
          tt_256[4] = _mm256_unpackhi_epi8(s_256[3], s_256[4]);

          y = h;
          do {
            s += 2 * src_stride;
            y_convolve_6tap_32x2_avx2(s, src_stride, coeffs_256, s_256, ss_256,
                                      tt_256, r);
            sr_y_round_store_32x2_avx2(r, d, dst_stride);

            ss_256[0] = ss_256[1];
            ss_256[1] = ss_256[2];
            ss_256[3] = ss_256[4];
            ss_256[4] = ss_256[5];

            tt_256[0] = tt_256[1];
            tt_256[1] = tt_256[2];
            tt_256[3] = tt_256[4];
            tt_256[4] = tt_256[5];
            d += 2 * dst_stride;
            y -= 2;
          } while (y);

          x += 32;
        } while (x < w);
      }
    }
  } else if (vert_tap == 8) {
    // vert_filt as 8 tap
    const uint8_t *src_ptr = src - 3 * src_stride;

    if (w <= 4) {
      prepare_half_coeffs_8tap_ssse3(filter_params_y, subpel_y_q4, coeffs_128);

      y = h;

      if (w == 2) {
        __m128i s_16[8], ss_128[4];

        s_16[0] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 0 * src_stride));
        s_16[1] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 1 * src_stride));
        s_16[2] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 2 * src_stride));
        s_16[3] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 3 * src_stride));
        s_16[4] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 4 * src_stride));
        s_16[5] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 5 * src_stride));
        s_16[6] = _mm_cvtsi32_si128(loadu_int16(src_ptr + 6 * src_stride));

        const __m128i src01 = _mm_unpacklo_epi16(s_16[0], s_16[1]);
        const __m128i src12 = _mm_unpacklo_epi16(s_16[1], s_16[2]);
        const __m128i src23 = _mm_unpacklo_epi16(s_16[2], s_16[3]);
        const __m128i src34 = _mm_unpacklo_epi16(s_16[3], s_16[4]);
        const __m128i src45 = _mm_unpacklo_epi16(s_16[4], s_16[5]);
        const __m128i src56 = _mm_unpacklo_epi16(s_16[5], s_16[6]);

        ss_128[0] = _mm_unpacklo_epi8(src01, src12);
        ss_128[1] = _mm_unpacklo_epi8(src23, src34);
        ss_128[2] = _mm_unpacklo_epi8(src45, src56);

        do {
          const __m128i res = y_convolve_8tap_2x2_ssse3(
              src_ptr, src_stride, coeffs_128, s_16, ss_128);
          const __m128i r = sr_y_round_sse2(res);
          pack_store_2x2_sse2(r, dst, dst_stride);
          ss_128[0] = ss_128[1];
          ss_128[1] = ss_128[2];
          ss_128[2] = ss_128[3];
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else {
        __m128i s_32[8], ss_128[4];

        assert(w == 4);

        s_32[0] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 0 * src_stride));
        s_32[1] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 1 * src_stride));
        s_32[2] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 2 * src_stride));
        s_32[3] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 3 * src_stride));
        s_32[4] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 4 * src_stride));
        s_32[5] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 5 * src_stride));
        s_32[6] = _mm_cvtsi32_si128(loadu_int32(src_ptr + 6 * src_stride));

        const __m128i src01 = _mm_unpacklo_epi32(s_32[0], s_32[1]);
        const __m128i src12 = _mm_unpacklo_epi32(s_32[1], s_32[2]);
        const __m128i src23 = _mm_unpacklo_epi32(s_32[2], s_32[3]);
        const __m128i src34 = _mm_unpacklo_epi32(s_32[3], s_32[4]);
        const __m128i src45 = _mm_unpacklo_epi32(s_32[4], s_32[5]);
        const __m128i src56 = _mm_unpacklo_epi32(s_32[5], s_32[6]);

        ss_128[0] = _mm_unpacklo_epi8(src01, src12);
        ss_128[1] = _mm_unpacklo_epi8(src23, src34);
        ss_128[2] = _mm_unpacklo_epi8(src45, src56);

        do {
          const __m128i res = y_convolve_8tap_4x2_ssse3(
              src_ptr, src_stride, coeffs_128, s_32, ss_128);
          const __m128i r = sr_y_round_sse2(res);
          pack_store_4x2_sse2(r, dst, dst_stride);
          ss_128[0] = ss_128[1];
          ss_128[1] = ss_128[2];
          ss_128[2] = ss_128[3];
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      }
    } else {
      prepare_half_coeffs_8tap_avx2(filter_params_y, subpel_y_q4, coeffs_256);

      if (w == 8) {
        __m128i s_64[8];
        __m256i ss_256[4];

        s_64[0] = _mm_loadl_epi64((__m128i *)(src_ptr + 0 * src_stride));
        s_64[1] = _mm_loadl_epi64((__m128i *)(src_ptr + 1 * src_stride));
        s_64[2] = _mm_loadl_epi64((__m128i *)(src_ptr + 2 * src_stride));
        s_64[3] = _mm_loadl_epi64((__m128i *)(src_ptr + 3 * src_stride));
        s_64[4] = _mm_loadl_epi64((__m128i *)(src_ptr + 4 * src_stride));
        s_64[5] = _mm_loadl_epi64((__m128i *)(src_ptr + 5 * src_stride));
        s_64[6] = _mm_loadl_epi64((__m128i *)(src_ptr + 6 * src_stride));

        // Load lines a and b. Line a to lower 128, line b to upper 128
        const __m256i src01 = _mm256_setr_m128i(s_64[0], s_64[1]);
        const __m256i src12 = _mm256_setr_m128i(s_64[1], s_64[2]);
        const __m256i src23 = _mm256_setr_m128i(s_64[2], s_64[3]);
        const __m256i src34 = _mm256_setr_m128i(s_64[3], s_64[4]);
        const __m256i src45 = _mm256_setr_m128i(s_64[4], s_64[5]);
        const __m256i src56 = _mm256_setr_m128i(s_64[5], s_64[6]);

        ss_256[0] = _mm256_unpacklo_epi8(src01, src12);
        ss_256[1] = _mm256_unpacklo_epi8(src23, src34);
        ss_256[2] = _mm256_unpacklo_epi8(src45, src56);

        y = h;
        do {
          const __m256i res = y_convolve_8tap_8x2_avx2(
              src_ptr, src_stride, coeffs_256, s_64, ss_256);
          sr_y_round_store_8x2_avx2(res, dst, dst_stride);
          ss_256[0] = ss_256[1];
          ss_256[1] = ss_256[2];
          ss_256[2] = ss_256[3];
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 16) {
        __m128i s_128[8];
        __m256i ss_256[8], r[2];

        s_128[0] = _mm_loadu_si128((__m128i *)(src_ptr + 0 * src_stride));
        s_128[1] = _mm_loadu_si128((__m128i *)(src_ptr + 1 * src_stride));
        s_128[2] = _mm_loadu_si128((__m128i *)(src_ptr + 2 * src_stride));
        s_128[3] = _mm_loadu_si128((__m128i *)(src_ptr + 3 * src_stride));
        s_128[4] = _mm_loadu_si128((__m128i *)(src_ptr + 4 * src_stride));
        s_128[5] = _mm_loadu_si128((__m128i *)(src_ptr + 5 * src_stride));
        s_128[6] = _mm_loadu_si128((__m128i *)(src_ptr + 6 * src_stride));

        // Load lines a and b. Line a to lower 128, line b to upper 128
        const __m256i src01 = _mm256_setr_m128i(s_128[0], s_128[1]);
        const __m256i src12 = _mm256_setr_m128i(s_128[1], s_128[2]);
        const __m256i src23 = _mm256_setr_m128i(s_128[2], s_128[3]);
        const __m256i src34 = _mm256_setr_m128i(s_128[3], s_128[4]);
        const __m256i src45 = _mm256_setr_m128i(s_128[4], s_128[5]);
        const __m256i src56 = _mm256_setr_m128i(s_128[5], s_128[6]);

        ss_256[0] = _mm256_unpacklo_epi8(src01, src12);
        ss_256[1] = _mm256_unpacklo_epi8(src23, src34);
        ss_256[2] = _mm256_unpacklo_epi8(src45, src56);

        ss_256[4] = _mm256_unpackhi_epi8(src01, src12);
        ss_256[5] = _mm256_unpackhi_epi8(src23, src34);
        ss_256[6] = _mm256_unpackhi_epi8(src45, src56);

        y = h;
        do {
          y_convolve_8tap_16x2_avx2(src_ptr, src_stride, coeffs_256, s_128,
                                    ss_256, r);
          sr_y_round_store_16x2_avx2(r, dst, dst_stride);

          ss_256[0] = ss_256[1];
          ss_256[1] = ss_256[2];
          ss_256[2] = ss_256[3];

          ss_256[4] = ss_256[5];
          ss_256[5] = ss_256[6];
          ss_256[6] = ss_256[7];
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else {
        __m256i s_256[8], ss_256[8], tt_256[8], r[4];

        assert(!(w % 32));

        x = 0;
        do {
          const uint8_t *s = src_ptr + x;
          uint8_t *d = dst + x;

          s_256[0] = _mm256_loadu_si256((__m256i *)(s + 0 * src_stride));
          s_256[1] = _mm256_loadu_si256((__m256i *)(s + 1 * src_stride));
          s_256[2] = _mm256_loadu_si256((__m256i *)(s + 2 * src_stride));
          s_256[3] = _mm256_loadu_si256((__m256i *)(s + 3 * src_stride));
          s_256[4] = _mm256_loadu_si256((__m256i *)(s + 4 * src_stride));
          s_256[5] = _mm256_loadu_si256((__m256i *)(s + 5 * src_stride));
          s_256[6] = _mm256_loadu_si256((__m256i *)(s + 6 * src_stride));

          ss_256[0] = _mm256_unpacklo_epi8(s_256[0], s_256[1]);
          ss_256[1] = _mm256_unpacklo_epi8(s_256[2], s_256[3]);
          ss_256[2] = _mm256_unpacklo_epi8(s_256[4], s_256[5]);
          ss_256[4] = _mm256_unpackhi_epi8(s_256[0], s_256[1]);
          ss_256[5] = _mm256_unpackhi_epi8(s_256[2], s_256[3]);
          ss_256[6] = _mm256_unpackhi_epi8(s_256[4], s_256[5]);

          tt_256[0] = _mm256_unpacklo_epi8(s_256[1], s_256[2]);
          tt_256[1] = _mm256_unpacklo_epi8(s_256[3], s_256[4]);
          tt_256[2] = _mm256_unpacklo_epi8(s_256[5], s_256[6]);
          tt_256[4] = _mm256_unpackhi_epi8(s_256[1], s_256[2]);
          tt_256[5] = _mm256_unpackhi_epi8(s_256[3], s_256[4]);
          tt_256[6] = _mm256_unpackhi_epi8(s_256[5], s_256[6]);

          y = h;
          do {
            y_convolve_8tap_32x2_avx2(s, src_stride, coeffs_256, s_256, ss_256,
                                      tt_256, r);
            sr_y_round_store_32x2_avx2(r, d, dst_stride);

            ss_256[0] = ss_256[1];
            ss_256[1] = ss_256[2];
            ss_256[2] = ss_256[3];
            ss_256[4] = ss_256[5];
            ss_256[5] = ss_256[6];
            ss_256[6] = ss_256[7];

            tt_256[0] = tt_256[1];
            tt_256[1] = tt_256[2];
            tt_256[2] = tt_256[3];
            tt_256[4] = tt_256[5];
            tt_256[5] = tt_256[6];
            tt_256[6] = tt_256[7];
            s += 2 * src_stride;
            d += 2 * dst_stride;
            y -= 2;
          } while (y);

          x += 32;
        } while (x < w);
      }
    }
  }
}

static inline void sr_x_2tap_32_avx2(const uint8_t *const src,
                                     const __m256i coeffs[1],
                                     uint8_t *const dst) {
  __m256i r[2];

  x_convolve_2tap_32_avx2(src, coeffs, r);
  sr_x_round_store_32_avx2(r, dst);
}

static inline void sr_x_6tap_32_avx2(const uint8_t *const src,
                                     const __m256i coeffs[3],
                                     const __m256i filt[3],
                                     uint8_t *const dst) {
  __m256i r[2];

  x_convolve_6tap_32_avx2(src, coeffs, filt, r);
  sr_x_round_store_32_avx2(r, dst);
}

static AOM_FORCE_INLINE void sr_x_8tap_32_avx2(const uint8_t *const src,
                                               const __m256i coeffs[4],
                                               const __m256i filt[4],
                                               uint8_t *const dst) {
  __m256i r[2];

  x_convolve_8tap_32_avx2(src, coeffs, filt, r);
  sr_x_round_store_32_avx2(r, dst);
}

static AOM_FORCE_INLINE void av1_convolve_x_sr_specialized_avx2(
    const uint8_t *src, int32_t src_stride, uint8_t *dst, int32_t dst_stride,
    int32_t w, int32_t h, const InterpFilterParams *filter_params_x,
    const int32_t subpel_x_q4, ConvolveParams *conv_params) {
  int32_t y = h;
  __m128i coeffs_128[4];
  __m256i coeffs_256[4];

  assert(conv_params->round_0 == 3);
  assert((FILTER_BITS - conv_params->round_1) >= 0 ||
         ((conv_params->round_0 + conv_params->round_1) == 2 * FILTER_BITS));
  (void)conv_params;

  const int horz_tap = get_filter_tap(filter_params_x, subpel_x_q4);

  if (horz_tap == 2) {
    // horz_filt as 2 tap
    const uint8_t *src_ptr = src;

    if (subpel_x_q4 != 8) {
      if (w <= 8) {
        prepare_half_coeffs_2tap_ssse3(filter_params_x, subpel_x_q4,
                                       coeffs_128);

        if (w == 2) {
          do {
            const __m128i res =
                x_convolve_2tap_2x2_sse4_1(src_ptr, src_stride, coeffs_128);
            const __m128i r = sr_x_round_sse2(res);
            pack_store_2x2_sse2(r, dst, dst_stride);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else if (w == 4) {
          do {
            const __m128i res =
                x_convolve_2tap_4x2_ssse3(src_ptr, src_stride, coeffs_128);
            const __m128i r = sr_x_round_sse2(res);
            pack_store_4x2_sse2(r, dst, dst_stride);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else {
          assert(w == 8);

          do {
            __m128i res[2];

            x_convolve_2tap_8x2_ssse3(src_ptr, src_stride, coeffs_128, res);
            res[0] = sr_x_round_sse2(res[0]);
            res[1] = sr_x_round_sse2(res[1]);
            const __m128i d = _mm_packus_epi16(res[0], res[1]);
            _mm_storel_epi64((__m128i *)dst, d);
            _mm_storeh_epi64((__m128i *)(dst + dst_stride), d);

            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        }
      } else {
        prepare_half_coeffs_2tap_avx2(filter_params_x, subpel_x_q4, coeffs_256);

        if (w == 16) {
          do {
            __m256i r[2];

            x_convolve_2tap_16x2_avx2(src_ptr, src_stride, coeffs_256, r);
            sr_x_round_store_16x2_avx2(r, dst, dst_stride);
            src_ptr += 2 * src_stride;
            dst += 2 * dst_stride;
            y -= 2;
          } while (y);
        } else if (w == 32) {
          do {
            sr_x_2tap_32_avx2(src_ptr, coeffs_256, dst);
            src_ptr += src_stride;
            dst += dst_stride;
          } while (--y);
        } else if (w == 64) {
          do {
            sr_x_2tap_32_avx2(src_ptr + 0 * 32, coeffs_256, dst + 0 * 32);
            sr_x_2tap_32_avx2(src_ptr + 1 * 32, coeffs_256, dst + 1 * 32);
            src_ptr += src_stride;
            dst += dst_stride;
          } while (--y);
        } else {
          assert(w == 128);

          do {
            sr_x_2tap_32_avx2(src_ptr + 0 * 32, coeffs_256, dst + 0 * 32);
            sr_x_2tap_32_avx2(src_ptr + 1 * 32, coeffs_256, dst + 1 * 32);
            sr_x_2tap_32_avx2(src_ptr + 2 * 32, coeffs_256, dst + 2 * 32);
            sr_x_2tap_32_avx2(src_ptr + 3 * 32, coeffs_256, dst + 3 * 32);
            src_ptr += src_stride;
            dst += dst_stride;
          } while (--y);
        }
      }
    } else {
      // average to get half pel
      if (w == 2) {
        do {
          __m128i s_128;

          s_128 = load_u8_4x2_sse4_1(src_ptr, src_stride);
          const __m128i s1 = _mm_srli_si128(s_128, 1);
          const __m128i d = _mm_avg_epu8(s_128, s1);
          *(uint16_t *)dst = (uint16_t)_mm_cvtsi128_si32(d);
          *(uint16_t *)(dst + dst_stride) = _mm_extract_epi16(d, 2);

          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 4) {
        do {
          __m128i s_128;

          s_128 = load_u8_8x2_sse2(src_ptr, src_stride);
          const __m128i s1 = _mm_srli_si128(s_128, 1);
          const __m128i d = _mm_avg_epu8(s_128, s1);
          xx_storel_32(dst, d);
          *(int32_t *)(dst + dst_stride) = _mm_extract_epi32(d, 2);

          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 8) {
        do {
          const __m128i s00 = _mm_loadu_si128((__m128i *)src_ptr);
          const __m128i s10 =
              _mm_loadu_si128((__m128i *)(src_ptr + src_stride));
          const __m128i s01 = _mm_srli_si128(s00, 1);
          const __m128i s11 = _mm_srli_si128(s10, 1);
          const __m128i d0 = _mm_avg_epu8(s00, s01);
          const __m128i d1 = _mm_avg_epu8(s10, s11);
          _mm_storel_epi64((__m128i *)dst, d0);
          _mm_storel_epi64((__m128i *)(dst + dst_stride), d1);

          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 16) {
        do {
          const __m128i s00 = _mm_loadu_si128((__m128i *)src_ptr);
          const __m128i s01 = _mm_loadu_si128((__m128i *)(src_ptr + 1));
          const __m128i s10 =
              _mm_loadu_si128((__m128i *)(src_ptr + src_stride));
          const __m128i s11 =
              _mm_loadu_si128((__m128i *)(src_ptr + src_stride + 1));
          const __m128i d0 = _mm_avg_epu8(s00, s01);
          const __m128i d1 = _mm_avg_epu8(s10, s11);
          _mm_storeu_si128((__m128i *)dst, d0);
          _mm_storeu_si128((__m128i *)(dst + dst_stride), d1);

          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 32) {
        do {
          sr_x_2tap_32_avg_avx2(src_ptr, dst);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      } else if (w == 64) {
        do {
          sr_x_2tap_32_avg_avx2(src_ptr + 0 * 32, dst + 0 * 32);
          sr_x_2tap_32_avg_avx2(src_ptr + 1 * 32, dst + 1 * 32);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      } else {
        assert(w == 128);

        do {
          sr_x_2tap_32_avg_avx2(src_ptr + 0 * 32, dst + 0 * 32);
          sr_x_2tap_32_avg_avx2(src_ptr + 1 * 32, dst + 1 * 32);
          sr_x_2tap_32_avg_avx2(src_ptr + 2 * 32, dst + 2 * 32);
          sr_x_2tap_32_avg_avx2(src_ptr + 3 * 32, dst + 3 * 32);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      }
    }
  } else if (horz_tap == 4) {
    // horz_filt as 4 tap
    const uint8_t *src_ptr = src - 1;

    prepare_half_coeffs_4tap_ssse3(filter_params_x, subpel_x_q4, coeffs_128);

    if (w == 2) {
      do {
        const __m128i res =
            x_convolve_4tap_2x2_ssse3(src_ptr, src_stride, coeffs_128);
        const __m128i r = sr_x_round_sse2(res);
        pack_store_2x2_sse2(r, dst, dst_stride);
        src_ptr += 2 * src_stride;
        dst += 2 * dst_stride;
        y -= 2;
      } while (y);
    } else if (w == 4) {
      do {
        const __m128i res =
            x_convolve_4tap_4x2_ssse3(src_ptr, src_stride, coeffs_128);
        const __m128i r = sr_x_round_sse2(res);
        pack_store_4x2_sse2(r, dst, dst_stride);
        src_ptr += 2 * src_stride;
        dst += 2 * dst_stride;
        y -= 2;
      } while (y);
    } else if (w == 8) {
      // TODO(chiyotsai@google.com): Reuse the old SIMD code here. Need to
      // rewrite this for better performance later.
      __m256i filt_256[2];
      prepare_coeffs_lowbd(filter_params_x, subpel_x_q4, coeffs_256);

      filt_256[0] = _mm256_loadu_si256((__m256i const *)filt1_global_avx2);
      filt_256[1] = _mm256_loadu_si256((__m256i const *)filt2_global_avx2);
      for (int i = 0; i < h; i += 2) {
        const __m256i data = _mm256_permute2x128_si256(
            _mm256_castsi128_si256(
                _mm_loadu_si128((__m128i *)(&src_ptr[i * src_stride]))),
            _mm256_castsi128_si256(_mm_loadu_si128(
                (__m128i *)(&src_ptr[i * src_stride + src_stride]))),
            0x20);

        __m256i res_16b = convolve_lowbd_x_4tap(data, coeffs_256 + 1, filt_256);
        res_16b = sr_x_round_avx2(res_16b);

        __m256i res_8b = _mm256_packus_epi16(res_16b, res_16b);

        const __m128i res_0 = _mm256_castsi256_si128(res_8b);
        const __m128i res_1 = _mm256_extracti128_si256(res_8b, 1);

        _mm_storel_epi64((__m128i *)&dst[i * dst_stride], res_0);
        _mm_storel_epi64((__m128i *)&dst[i * dst_stride + dst_stride], res_1);
      }
    } else {
      assert(!(w % 16));
      // TODO(chiyotsai@google.com): Reuse the old SIMD code here. Need to
      // rewrite this for better performance later.
      __m256i filt_256[2];
      prepare_coeffs_lowbd(filter_params_x, subpel_x_q4, coeffs_256);
      filt_256[0] = _mm256_loadu_si256((__m256i const *)filt1_global_avx2);
      filt_256[1] = _mm256_loadu_si256((__m256i const *)filt2_global_avx2);

      for (int i = 0; i < h; ++i) {
        for (int j = 0; j < w; j += 16) {
          // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 8 9 10 11 12 13 14 15 16 17
          // 18 19 20 21 22 23
          const __m256i data = _mm256_inserti128_si256(
              _mm256_loadu_si256((__m256i *)&src_ptr[(i * src_stride) + j]),
              _mm_loadu_si128((__m128i *)&src_ptr[(i * src_stride) + (j + 8)]),
              1);

          __m256i res_16b =
              convolve_lowbd_x_4tap(data, coeffs_256 + 1, filt_256);
          res_16b = sr_x_round_avx2(res_16b);

          /* rounding code */
          // 8 bit conversion and saturation to uint8
          __m256i res_8b = _mm256_packus_epi16(res_16b, res_16b);

          // Store values into the destination buffer
          // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
          res_8b = _mm256_permute4x64_epi64(res_8b, 216);
          __m128i res = _mm256_castsi256_si128(res_8b);
          _mm_storeu_si128((__m128i *)&dst[i * dst_stride + j], res);
        }
      }
    }
  } else {
    __m256i filt_256[4];

    filt_256[0] = _mm256_loadu_si256((__m256i const *)filt1_global_avx2);
    filt_256[1] = _mm256_loadu_si256((__m256i const *)filt2_global_avx2);
    filt_256[2] = _mm256_loadu_si256((__m256i const *)filt3_global_avx2);

    if (horz_tap == 6) {
      // horz_filt as 6 tap
      const uint8_t *src_ptr = src - 2;

      prepare_half_coeffs_6tap_avx2(filter_params_x, subpel_x_q4, coeffs_256);

      if (w == 8) {
        do {
          const __m256i res = x_convolve_6tap_8x2_avx2(src_ptr, src_stride,
                                                       coeffs_256, filt_256);
          sr_x_round_store_8x2_avx2(res, dst, dst_stride);
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 16) {
        do {
          __m256i r[2];

          x_convolve_6tap_16x2_avx2(src_ptr, src_stride, coeffs_256, filt_256,
                                    r);
          sr_x_round_store_16x2_avx2(r, dst, dst_stride);
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 32) {
        do {
          sr_x_6tap_32_avx2(src_ptr, coeffs_256, filt_256, dst);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      } else if (w == 64) {
        do {
          sr_x_6tap_32_avx2(src_ptr, coeffs_256, filt_256, dst);
          sr_x_6tap_32_avx2(src_ptr + 32, coeffs_256, filt_256, dst + 32);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      } else {
        assert(w == 128);

        do {
          sr_x_6tap_32_avx2(src_ptr, coeffs_256, filt_256, dst);
          sr_x_6tap_32_avx2(src_ptr + 1 * 32, coeffs_256, filt_256,
                            dst + 1 * 32);
          sr_x_6tap_32_avx2(src_ptr + 2 * 32, coeffs_256, filt_256,
                            dst + 2 * 32);
          sr_x_6tap_32_avx2(src_ptr + 3 * 32, coeffs_256, filt_256,
                            dst + 3 * 32);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      }
    } else if (horz_tap == 8) {
      // horz_filt as 8 tap
      const uint8_t *src_ptr = src - 3;

      filt_256[3] = _mm256_loadu_si256((__m256i const *)filt4_global_avx2);

      prepare_half_coeffs_8tap_avx2(filter_params_x, subpel_x_q4, coeffs_256);

      if (w == 8) {
        do {
          const __m256i res = x_convolve_8tap_8x2_avx2(src_ptr, src_stride,
                                                       coeffs_256, filt_256);
          sr_x_round_store_8x2_avx2(res, dst, dst_stride);
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 16) {
        do {
          __m256i r[2];

          x_convolve_8tap_16x2_avx2(src_ptr, src_stride, coeffs_256, filt_256,
                                    r);
          sr_x_round_store_16x2_avx2(r, dst, dst_stride);
          src_ptr += 2 * src_stride;
          dst += 2 * dst_stride;
          y -= 2;
        } while (y);
      } else if (w == 32) {
        do {
          sr_x_8tap_32_avx2(src_ptr, coeffs_256, filt_256, dst);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      } else if (w == 64) {
        do {
          sr_x_8tap_32_avx2(src_ptr, coeffs_256, filt_256, dst);
          sr_x_8tap_32_avx2(src_ptr + 32, coeffs_256, filt_256, dst + 32);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      } else {
        assert(w == 128);

        do {
          sr_x_8tap_32_avx2(src_ptr, coeffs_256, filt_256, dst);
          sr_x_8tap_32_avx2(src_ptr + 1 * 32, coeffs_256, filt_256,
                            dst + 1 * 32);
          sr_x_8tap_32_avx2(src_ptr + 2 * 32, coeffs_256, filt_256,
                            dst + 2 * 32);
          sr_x_8tap_32_avx2(src_ptr + 3 * 32, coeffs_256, filt_256,
                            dst + 3 * 32);
          src_ptr += src_stride;
          dst += dst_stride;
        } while (--y);
      }
    }
  }
}

#endif  // THIRD_PARTY_SVT_AV1_CONVOLVE_AVX2_H_
