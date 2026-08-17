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

#ifndef AOM_AOM_DSP_X86_FWD_TXFM_SSE2_H_
#define AOM_AOM_DSP_X86_FWD_TXFM_SSE2_H_

#ifdef __cplusplus
extern "C" {
#endif

static inline __m128i k_madd_epi32(__m128i a, __m128i b) {
  __m128i buf0, buf1;
  buf0 = _mm_mul_epu32(a, b);
  a = _mm_srli_epi64(a, 32);
  b = _mm_srli_epi64(b, 32);
  buf1 = _mm_mul_epu32(a, b);
  return _mm_add_epi64(buf0, buf1);
}

static inline __m128i k_packs_epi64(__m128i a, __m128i b) {
  __m128i buf0 = _mm_shuffle_epi32(a, _MM_SHUFFLE(0, 0, 2, 0));
  __m128i buf1 = _mm_shuffle_epi32(b, _MM_SHUFFLE(0, 0, 2, 0));
  return _mm_unpacklo_epi64(buf0, buf1);
}

static inline int check_epi16_overflow_x2(const __m128i *preg0,
                                          const __m128i *preg1) {
  const __m128i max_overflow = _mm_set1_epi16(0x7fff);
  const __m128i min_overflow = _mm_set1_epi16((short)0x8000);
  __m128i cmp0 = _mm_or_si128(_mm_cmpeq_epi16(*preg0, max_overflow),
                              _mm_cmpeq_epi16(*preg0, min_overflow));
  __m128i cmp1 = _mm_or_si128(_mm_cmpeq_epi16(*preg1, max_overflow),
                              _mm_cmpeq_epi16(*preg1, min_overflow));
  cmp0 = _mm_or_si128(cmp0, cmp1);
  return _mm_movemask_epi8(cmp0);
}

static inline int check_epi16_overflow_x4(const __m128i *preg0,
                                          const __m128i *preg1,
                                          const __m128i *preg2,
                                          const __m128i *preg3) {
  const __m128i max_overflow = _mm_set1_epi16(0x7fff);
  const __m128i min_overflow = _mm_set1_epi16((short)0x8000);
  __m128i cmp0 = _mm_or_si128(_mm_cmpeq_epi16(*preg0, max_overflow),
                              _mm_cmpeq_epi16(*preg0, min_overflow));
  __m128i cmp1 = _mm_or_si128(_mm_cmpeq_epi16(*preg1, max_overflow),
                              _mm_cmpeq_epi16(*preg1, min_overflow));
  __m128i cmp2 = _mm_or_si128(_mm_cmpeq_epi16(*preg2, max_overflow),
                              _mm_cmpeq_epi16(*preg2, min_overflow));
  __m128i cmp3 = _mm_or_si128(_mm_cmpeq_epi16(*preg3, max_overflow),
                              _mm_cmpeq_epi16(*preg3, min_overflow));
  cmp0 = _mm_or_si128(_mm_or_si128(cmp0, cmp1), _mm_or_si128(cmp2, cmp3));
  return _mm_movemask_epi8(cmp0);
}

static inline int check_epi16_overflow_x8(
    const __m128i *preg0, const __m128i *preg1, const __m128i *preg2,
    const __m128i *preg3, const __m128i *preg4, const __m128i *preg5,
    const __m128i *preg6, const __m128i *preg7) {
  int res0, res1;
  res0 = check_epi16_overflow_x4(preg0, preg1, preg2, preg3);
  res1 = check_epi16_overflow_x4(preg4, preg5, preg6, preg7);
  return res0 + res1;
}

static inline int check_epi16_overflow_x12(
    const __m128i *preg0, const __m128i *preg1, const __m128i *preg2,
    const __m128i *preg3, const __m128i *preg4, const __m128i *preg5,
    const __m128i *preg6, const __m128i *preg7, const __m128i *preg8,
    const __m128i *preg9, const __m128i *preg10, const __m128i *preg11) {
  int res0, res1;
  res0 = check_epi16_overflow_x4(preg0, preg1, preg2, preg3);
  res1 = check_epi16_overflow_x4(preg4, preg5, preg6, preg7);
  if (!res0) res0 = check_epi16_overflow_x4(preg8, preg9, preg10, preg11);
  return res0 + res1;
}

static inline int check_epi16_overflow_x16(
    const __m128i *preg0, const __m128i *preg1, const __m128i *preg2,
    const __m128i *preg3, const __m128i *preg4, const __m128i *preg5,
    const __m128i *preg6, const __m128i *preg7, const __m128i *preg8,
    const __m128i *preg9, const __m128i *preg10, const __m128i *preg11,
    const __m128i *preg12, const __m128i *preg13, const __m128i *preg14,
    const __m128i *preg15) {
  int res0, res1;
  res0 = check_epi16_overflow_x4(preg0, preg1, preg2, preg3);
  res1 = check_epi16_overflow_x4(preg4, preg5, preg6, preg7);
  if (!res0) {
    res0 = check_epi16_overflow_x4(preg8, preg9, preg10, preg11);
    if (!res1) res1 = check_epi16_overflow_x4(preg12, preg13, preg14, preg15);
  }
  return res0 + res1;
}

static inline int check_epi16_overflow_x32(
    const __m128i *preg0, const __m128i *preg1, const __m128i *preg2,
    const __m128i *preg3, const __m128i *preg4, const __m128i *preg5,
    const __m128i *preg6, const __m128i *preg7, const __m128i *preg8,
    const __m128i *preg9, const __m128i *preg10, const __m128i *preg11,
    const __m128i *preg12, const __m128i *preg13, const __m128i *preg14,
    const __m128i *preg15, const __m128i *preg16, const __m128i *preg17,
    const __m128i *preg18, const __m128i *preg19, const __m128i *preg20,
    const __m128i *preg21, const __m128i *preg22, const __m128i *preg23,
    const __m128i *preg24, const __m128i *preg25, const __m128i *preg26,
    const __m128i *preg27, const __m128i *preg28, const __m128i *preg29,
    const __m128i *preg30, const __m128i *preg31) {
  int res0, res1;
  res0 = check_epi16_overflow_x4(preg0, preg1, preg2, preg3);
  res1 = check_epi16_overflow_x4(preg4, preg5, preg6, preg7);
  if (!res0) {
    res0 = check_epi16_overflow_x4(preg8, preg9, preg10, preg11);
    if (!res1) {
      res1 = check_epi16_overflow_x4(preg12, preg13, preg14, preg15);
      if (!res0) {
        res0 = check_epi16_overflow_x4(preg16, preg17, preg18, preg19);
        if (!res1) {
          res1 = check_epi16_overflow_x4(preg20, preg21, preg22, preg23);
          if (!res0) {
            res0 = check_epi16_overflow_x4(preg24, preg25, preg26, preg27);
            if (!res1)
              res1 = check_epi16_overflow_x4(preg28, preg29, preg30, preg31);
          }
        }
      }
    }
  }
  return res0 + res1;
}

static inline void store_output(const __m128i *poutput, tran_low_t *dst_ptr) {
  const __m128i zero = _mm_setzero_si128();
  const __m128i sign_bits = _mm_cmplt_epi16(*poutput, zero);
  __m128i out0 = _mm_unpacklo_epi16(*poutput, sign_bits);
  __m128i out1 = _mm_unpackhi_epi16(*poutput, sign_bits);
  _mm_store_si128((__m128i *)(dst_ptr), out0);
  _mm_store_si128((__m128i *)(dst_ptr + 4), out1);
}

static inline void storeu_output(const __m128i *poutput, tran_low_t *dst_ptr) {
  const __m128i zero = _mm_setzero_si128();
  const __m128i sign_bits = _mm_cmplt_epi16(*poutput, zero);
  __m128i out0 = _mm_unpacklo_epi16(*poutput, sign_bits);
  __m128i out1 = _mm_unpackhi_epi16(*poutput, sign_bits);
  _mm_storeu_si128((__m128i *)(dst_ptr), out0);
  _mm_storeu_si128((__m128i *)(dst_ptr + 4), out1);
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_X86_FWD_TXFM_SSE2_H_
