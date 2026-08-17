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

#ifndef AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_AVX2_H_
#define AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_AVX2_H_
#include <immintrin.h>

// out0 = in0*w0 + in1*w1
// out1 = -in1*w0 + in0*w1
static inline void btf_32_avx2_type0(const int32_t w0, const int32_t w1,
                                     __m256i *in0, __m256i *in1,
                                     const __m256i _r, const int32_t cos_bit) {
  __m256i _in0 = *in0;
  __m256i _in1 = *in1;
  const __m256i ww0 = _mm256_set1_epi32(w0);
  const __m256i ww1 = _mm256_set1_epi32(w1);
  const __m256i in0_w0 = _mm256_mullo_epi32(_in0, ww0);
  const __m256i in1_w1 = _mm256_mullo_epi32(_in1, ww1);
  __m256i temp0 = _mm256_add_epi32(in0_w0, in1_w1);
  temp0 = _mm256_add_epi32(temp0, _r);
  *in0 = _mm256_srai_epi32(temp0, cos_bit);
  const __m256i in0_w1 = _mm256_mullo_epi32(_in0, ww1);
  const __m256i in1_w0 = _mm256_mullo_epi32(_in1, ww0);
  __m256i temp1 = _mm256_sub_epi32(in0_w1, in1_w0);
  temp1 = _mm256_add_epi32(temp1, _r);
  *in1 = _mm256_srai_epi32(temp1, cos_bit);
}

static inline void btf_32_avx2_type1(const int32_t w0, const int32_t w1,
                                     __m256i *in0, __m256i *in1,
                                     const __m256i _r, const int32_t cos_bit) {
  __m256i _in0 = *in0;
  __m256i _in1 = *in1;
  const __m256i ww0 = _mm256_set1_epi32(w0);
  const __m256i ww1 = _mm256_set1_epi32(w1);
  const __m256i in0_w0 = _mm256_mullo_epi32(_in0, ww0);
  const __m256i in1_w1 = _mm256_mullo_epi32(_in1, ww1);
  __m256i temp0 = _mm256_add_epi32(in0_w0, in1_w1);
  temp0 = _mm256_add_epi32(temp0, _r);
  *in0 = _mm256_srai_epi32(temp0, cos_bit);
  const __m256i in0_w1 = _mm256_mullo_epi32(_in0, ww1);
  const __m256i in1_w0 = _mm256_mullo_epi32(_in1, ww0);
  __m256i temp1 = _mm256_sub_epi32(in1_w0, in0_w1);
  temp1 = _mm256_add_epi32(temp1, _r);
  *in1 = _mm256_srai_epi32(temp1, cos_bit);
}

// out0 = in0*w0 + in1*w1
// out1 = -in1*w0 + in0*w1
static inline void btf_32_avx2_type0_new(const __m256i ww0, const __m256i ww1,
                                         __m256i *in0, __m256i *in1,
                                         const __m256i _r,
                                         const int32_t cos_bit) {
  __m256i _in0 = *in0;
  __m256i _in1 = *in1;
  const __m256i in0_w0 = _mm256_mullo_epi32(_in0, ww0);
  const __m256i in1_w1 = _mm256_mullo_epi32(_in1, ww1);
  __m256i temp0 = _mm256_add_epi32(in0_w0, in1_w1);
  temp0 = _mm256_add_epi32(temp0, _r);
  *in0 = _mm256_srai_epi32(temp0, cos_bit);
  const __m256i in0_w1 = _mm256_mullo_epi32(_in0, ww1);
  const __m256i in1_w0 = _mm256_mullo_epi32(_in1, ww0);
  __m256i temp1 = _mm256_sub_epi32(in0_w1, in1_w0);
  temp1 = _mm256_add_epi32(temp1, _r);
  *in1 = _mm256_srai_epi32(temp1, cos_bit);
}

// out0 = in0*w0 + in1*w1
// out1 = in1*w0 - in0*w1
static inline void btf_32_avx2_type1_new(const __m256i ww0, const __m256i ww1,
                                         __m256i *in0, __m256i *in1,
                                         const __m256i _r,
                                         const int32_t cos_bit) {
  __m256i _in0 = *in0;
  __m256i _in1 = *in1;
  const __m256i in0_w0 = _mm256_mullo_epi32(_in0, ww0);
  const __m256i in1_w1 = _mm256_mullo_epi32(_in1, ww1);
  __m256i temp0 = _mm256_add_epi32(in0_w0, in1_w1);
  temp0 = _mm256_add_epi32(temp0, _r);
  *in0 = _mm256_srai_epi32(temp0, cos_bit);
  const __m256i in0_w1 = _mm256_mullo_epi32(_in0, ww1);
  const __m256i in1_w0 = _mm256_mullo_epi32(_in1, ww0);
  __m256i temp1 = _mm256_sub_epi32(in1_w0, in0_w1);
  temp1 = _mm256_add_epi32(temp1, _r);
  *in1 = _mm256_srai_epi32(temp1, cos_bit);
}

#endif  // AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_AVX2_H_
