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

#ifndef AOM_AOM_DSP_X86_CONVOLVE_SSE4_1_H_
#define AOM_AOM_DSP_X86_CONVOLVE_SSE4_1_H_

// Note:
//  This header file should be put below any x86 intrinsics head file

static inline void mult_add_store(CONV_BUF_TYPE *const dst,
                                  const __m128i *const res,
                                  const __m128i *const wt0,
                                  const __m128i *const wt1,
                                  const int do_average) {
  __m128i d;
  if (do_average) {
    d = _mm_load_si128((__m128i *)dst);
    d = _mm_add_epi32(_mm_mullo_epi32(d, *wt0), _mm_mullo_epi32(*res, *wt1));
    d = _mm_srai_epi32(d, DIST_PRECISION_BITS);
  } else {
    d = *res;
  }
  _mm_store_si128((__m128i *)dst, d);
}

static inline __m128i highbd_comp_avg_sse4_1(const __m128i *const data_ref_0,
                                             const __m128i *const res_unsigned,
                                             const __m128i *const wt0,
                                             const __m128i *const wt1,
                                             const int use_dist_wtd_avg) {
  __m128i res;
  if (use_dist_wtd_avg) {
    const __m128i wt0_res = _mm_mullo_epi32(*data_ref_0, *wt0);
    const __m128i wt1_res = _mm_mullo_epi32(*res_unsigned, *wt1);

    const __m128i wt_res = _mm_add_epi32(wt0_res, wt1_res);
    res = _mm_srai_epi32(wt_res, DIST_PRECISION_BITS);
  } else {
    const __m128i wt_res = _mm_add_epi32(*data_ref_0, *res_unsigned);
    res = _mm_srai_epi32(wt_res, 1);
  }
  return res;
}

#endif  // AOM_AOM_DSP_X86_CONVOLVE_SSE4_1_H_
