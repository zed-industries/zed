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

#ifndef AOM_AOM_DSP_X86_CONVOLVE_COMMON_INTRIN_H_
#define AOM_AOM_DSP_X86_CONVOLVE_COMMON_INTRIN_H_

// Note:
//  This header file should be put below any x86 intrinsics head file

static inline void add_store(CONV_BUF_TYPE *const dst, const __m128i *const res,
                             const int do_average) {
  __m128i d;
  if (do_average) {
    d = _mm_load_si128((__m128i *)dst);
    d = _mm_add_epi32(d, *res);
    d = _mm_srai_epi32(d, 1);
  } else {
    d = *res;
  }
  _mm_store_si128((__m128i *)dst, d);
}

static inline void prepare_coeffs_12tap(const InterpFilterParams *filter_params,
                                        int subpel_q4,
                                        __m128i *coeffs /* [6] */) {
  const int16_t *const y_filter = av1_get_interp_filter_subpel_kernel(
      filter_params, subpel_q4 & SUBPEL_MASK);

  __m128i coeffs_y = _mm_loadu_si128((__m128i *)y_filter);

  coeffs[0] = _mm_shuffle_epi32(coeffs_y, 0);    // coeffs 0 1 0 1 0 1 0 1
  coeffs[1] = _mm_shuffle_epi32(coeffs_y, 85);   // coeffs 2 3 2 3 2 3 2 3
  coeffs[2] = _mm_shuffle_epi32(coeffs_y, 170);  // coeffs 4 5 4 5 4 5 4 5
  coeffs[3] = _mm_shuffle_epi32(coeffs_y, 255);  // coeffs 6 7 6 7 6 7 6 7

  coeffs_y = _mm_loadl_epi64((__m128i *)(y_filter + 8));

  coeffs[4] = _mm_shuffle_epi32(coeffs_y, 0);  // coeffs 8 9 8 9 8 9 8 9
  coeffs[5] =
      _mm_shuffle_epi32(coeffs_y, 85);  // coeffs 10 11 10 11 10 11 10 11
}

static inline __m128i convolve_12tap(const __m128i *s, const __m128i *coeffs) {
  const __m128i d0 = _mm_madd_epi16(s[0], coeffs[0]);
  const __m128i d1 = _mm_madd_epi16(s[1], coeffs[1]);
  const __m128i d2 = _mm_madd_epi16(s[2], coeffs[2]);
  const __m128i d3 = _mm_madd_epi16(s[3], coeffs[3]);
  const __m128i d4 = _mm_madd_epi16(s[4], coeffs[4]);
  const __m128i d5 = _mm_madd_epi16(s[5], coeffs[5]);
  const __m128i d_0123 =
      _mm_add_epi32(_mm_add_epi32(d0, d1), _mm_add_epi32(d2, d3));
  const __m128i d = _mm_add_epi32(_mm_add_epi32(d4, d5), d_0123);
  return d;
}

static inline __m128i convolve_lo_x_12tap(const __m128i *s,
                                          const __m128i *coeffs,
                                          const __m128i zero) {
  __m128i ss[6];
  ss[0] = _mm_unpacklo_epi8(s[0], zero);  //  0  1  1  2  2  3  3  4
  ss[1] = _mm_unpacklo_epi8(s[1], zero);  //  2  3  3  4  4  5  5  6
  ss[2] = _mm_unpacklo_epi8(s[2], zero);  //  4  5  5  6  6  7  7  8
  ss[3] = _mm_unpacklo_epi8(s[3], zero);  //  6  7  7  8  8  9  9 10
  ss[4] = _mm_unpackhi_epi8(s[2], zero);  //  8  9  9 10 10 11 11 12
  ss[5] = _mm_unpackhi_epi8(s[3], zero);  // 10 11 11 12 12 13 13 14
  return convolve_12tap(ss, coeffs);
}

static inline __m128i convolve_lo_y_12tap(const __m128i *s,
                                          const __m128i *coeffs) {
  __m128i ss[6];
  const __m128i zero = _mm_setzero_si128();
  ss[0] = _mm_unpacklo_epi8(s[0], zero);
  ss[1] = _mm_unpacklo_epi8(s[2], zero);
  ss[2] = _mm_unpacklo_epi8(s[4], zero);
  ss[3] = _mm_unpacklo_epi8(s[6], zero);
  ss[4] = _mm_unpacklo_epi8(s[8], zero);
  ss[5] = _mm_unpacklo_epi8(s[10], zero);
  return convolve_12tap(ss, coeffs);
}

static inline __m128i convolve_hi_y_12tap(const __m128i *s,
                                          const __m128i *coeffs) {
  __m128i ss[6];
  const __m128i zero = _mm_setzero_si128();
  ss[0] = _mm_unpackhi_epi8(s[0], zero);
  ss[1] = _mm_unpackhi_epi8(s[2], zero);
  ss[2] = _mm_unpackhi_epi8(s[4], zero);
  ss[3] = _mm_unpackhi_epi8(s[6], zero);
  ss[4] = _mm_unpackhi_epi8(s[8], zero);
  ss[5] = _mm_unpackhi_epi8(s[10], zero);
  return convolve_12tap(ss, coeffs);
}
#endif  // AOM_AOM_DSP_X86_CONVOLVE_COMMON_INTRIN_H_
