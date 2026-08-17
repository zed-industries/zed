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

#include <emmintrin.h>

#include "aom/aom_integer.h"

static inline void load_b_values(const int16_t *zbin_ptr, __m128i *zbin,
                                 const int16_t *round_ptr, __m128i *round,
                                 const int16_t *quant_ptr, __m128i *quant,
                                 const int16_t *dequant_ptr, __m128i *dequant,
                                 const int16_t *shift_ptr, __m128i *shift) {
  *zbin = _mm_load_si128((const __m128i *)zbin_ptr);
  *round = _mm_load_si128((const __m128i *)round_ptr);
  *quant = _mm_load_si128((const __m128i *)quant_ptr);
  *zbin = _mm_sub_epi16(*zbin, _mm_set1_epi16(1));
  *dequant = _mm_load_si128((const __m128i *)dequant_ptr);
  *shift = _mm_load_si128((const __m128i *)shift_ptr);
}

// With ssse3 and later abs() and sign() are preferred.
static inline __m128i invert_sign_sse2(__m128i a, __m128i sign) {
  a = _mm_xor_si128(a, sign);
  return _mm_sub_epi16(a, sign);
}

static inline __m128i invert_sign_32_sse2(__m128i a, __m128i sign) {
  a = _mm_xor_si128(a, sign);
  return _mm_sub_epi32(a, sign);
}

static inline void calculate_qcoeff(__m128i *coeff, const __m128i round,
                                    const __m128i quant, const __m128i shift) {
  __m128i tmp, qcoeff;
  qcoeff = _mm_adds_epi16(*coeff, round);
  tmp = _mm_mulhi_epi16(qcoeff, quant);
  qcoeff = _mm_add_epi16(tmp, qcoeff);
  *coeff = _mm_mulhi_epi16(qcoeff, shift);
}

static inline void calculate_qcoeff_log_scale(__m128i *coeff,
                                              const __m128i round,
                                              const __m128i quant,
                                              const __m128i *shift,
                                              const int *log_scale) {
  __m128i tmp, tmp1, qcoeff;
  qcoeff = _mm_adds_epi16(*coeff, round);
  tmp = _mm_mulhi_epi16(qcoeff, quant);
  qcoeff = _mm_add_epi16(tmp, qcoeff);
  tmp = _mm_mullo_epi16(qcoeff, *shift);
  tmp = _mm_srli_epi16(tmp, (16 - *log_scale));
  tmp1 = _mm_mulhi_epi16(qcoeff, *shift);
  tmp1 = _mm_slli_epi16(tmp1, *log_scale);
  *coeff = _mm_or_si128(tmp, tmp1);
}

static inline __m128i calculate_dqcoeff(__m128i qcoeff, __m128i dequant) {
  return _mm_mullo_epi16(qcoeff, dequant);
}

static inline void calculate_dqcoeff_and_store_log_scale(__m128i qcoeff,
                                                         __m128i dequant,
                                                         const __m128i zero,
                                                         tran_low_t *dqcoeff,
                                                         const int *log_scale) {
  // calculate abs
  __m128i coeff_sign = _mm_srai_epi16(qcoeff, 15);
  __m128i coeff = invert_sign_sse2(qcoeff, coeff_sign);

  const __m128i sign_0 = _mm_unpacklo_epi16(coeff_sign, zero);
  const __m128i sign_1 = _mm_unpackhi_epi16(coeff_sign, zero);

  const __m128i low = _mm_mullo_epi16(coeff, dequant);
  const __m128i high = _mm_mulhi_epi16(coeff, dequant);
  __m128i dqcoeff32_0 = _mm_unpacklo_epi16(low, high);
  __m128i dqcoeff32_1 = _mm_unpackhi_epi16(low, high);

  dqcoeff32_0 = _mm_srli_epi32(dqcoeff32_0, *log_scale);
  dqcoeff32_1 = _mm_srli_epi32(dqcoeff32_1, *log_scale);

  dqcoeff32_0 = invert_sign_32_sse2(dqcoeff32_0, sign_0);
  dqcoeff32_1 = invert_sign_32_sse2(dqcoeff32_1, sign_1);

  _mm_store_si128((__m128i *)(dqcoeff), dqcoeff32_0);
  _mm_store_si128((__m128i *)(dqcoeff + 4), dqcoeff32_1);
}

// Scan 16 values for eob reference in scan_ptr. Use masks (-1) from comparing
// to zbin to add 1 to the index in 'scan'.
static inline __m128i scan_for_eob(__m128i *coeff0, __m128i *coeff1,
                                   const __m128i zbin_mask0,
                                   const __m128i zbin_mask1,
                                   const int16_t *scan_ptr, const int index,
                                   const __m128i zero) {
  const __m128i zero_coeff0 = _mm_cmpeq_epi16(*coeff0, zero);
  const __m128i zero_coeff1 = _mm_cmpeq_epi16(*coeff1, zero);
  __m128i scan0 = _mm_load_si128((const __m128i *)(scan_ptr + index));
  __m128i scan1 = _mm_load_si128((const __m128i *)(scan_ptr + index + 8));
  __m128i eob0, eob1;
  // Add one to convert from indices to counts
  scan0 = _mm_sub_epi16(scan0, zbin_mask0);
  scan1 = _mm_sub_epi16(scan1, zbin_mask1);
  eob0 = _mm_andnot_si128(zero_coeff0, scan0);
  eob1 = _mm_andnot_si128(zero_coeff1, scan1);
  return _mm_max_epi16(eob0, eob1);
}

static inline int16_t accumulate_eob(__m128i eob) {
  __m128i eob_shuffled;
  eob_shuffled = _mm_shuffle_epi32(eob, 0xe);
  eob = _mm_max_epi16(eob, eob_shuffled);
  eob_shuffled = _mm_shufflelo_epi16(eob, 0xe);
  eob = _mm_max_epi16(eob, eob_shuffled);
  eob_shuffled = _mm_shufflelo_epi16(eob, 0x1);
  eob = _mm_max_epi16(eob, eob_shuffled);
  return _mm_extract_epi16(eob, 1);
}

static inline __m128i load_coefficients(const tran_low_t *coeff_ptr) {
  assert(sizeof(tran_low_t) == 4);
  const __m128i coeff1 = _mm_load_si128((__m128i *)(coeff_ptr));
  const __m128i coeff2 = _mm_load_si128((__m128i *)(coeff_ptr + 4));
  return _mm_packs_epi32(coeff1, coeff2);
}

static inline void store_coefficients(__m128i coeff_vals,
                                      tran_low_t *coeff_ptr) {
  assert(sizeof(tran_low_t) == 4);

  __m128i one = _mm_set1_epi16(1);
  __m128i coeff_vals_hi = _mm_mulhi_epi16(coeff_vals, one);
  __m128i coeff_vals_lo = _mm_mullo_epi16(coeff_vals, one);
  __m128i coeff_vals_1 = _mm_unpacklo_epi16(coeff_vals_lo, coeff_vals_hi);
  __m128i coeff_vals_2 = _mm_unpackhi_epi16(coeff_vals_lo, coeff_vals_hi);
  _mm_store_si128((__m128i *)(coeff_ptr), coeff_vals_1);
  _mm_store_si128((__m128i *)(coeff_ptr + 4), coeff_vals_2);
}

static inline void update_mask1(__m128i *cmp_mask0, __m128i *cmp_mask1,
                                const int16_t *iscan_ptr, int *is_found,
                                __m128i *mask) {
  __m128i all_zero;
  __m128i temp_mask = _mm_setzero_si128();
  all_zero = _mm_or_si128(*cmp_mask0, *cmp_mask1);
  if (_mm_movemask_epi8(all_zero)) {
    __m128i iscan0 = _mm_load_si128((const __m128i *)(iscan_ptr));
    __m128i mask0 = _mm_and_si128(*cmp_mask0, iscan0);
    __m128i iscan1 = _mm_load_si128((const __m128i *)(iscan_ptr + 8));
    __m128i mask1 = _mm_and_si128(*cmp_mask1, iscan1);
    temp_mask = _mm_max_epi16(mask0, mask1);
    *is_found = 1;
  }
  *mask = _mm_max_epi16(temp_mask, *mask);
}

static inline void update_mask0(__m128i *qcoeff0, __m128i *qcoeff1,
                                __m128i *threshold, const int16_t *iscan_ptr,
                                int *is_found, __m128i *mask) {
  __m128i zero = _mm_setzero_si128();
  __m128i coeff[4], cmp_mask0, cmp_mask1, cmp_mask2, cmp_mask3;

  coeff[0] = _mm_unpacklo_epi16(*qcoeff0, zero);
  coeff[1] = _mm_unpackhi_epi16(*qcoeff0, zero);
  coeff[2] = _mm_unpacklo_epi16(*qcoeff1, zero);
  coeff[3] = _mm_unpackhi_epi16(*qcoeff1, zero);

  coeff[0] = _mm_slli_epi32(coeff[0], AOM_QM_BITS);
  cmp_mask0 = _mm_cmpgt_epi32(coeff[0], threshold[0]);
  coeff[1] = _mm_slli_epi32(coeff[1], AOM_QM_BITS);
  cmp_mask1 = _mm_cmpgt_epi32(coeff[1], threshold[1]);
  coeff[2] = _mm_slli_epi32(coeff[2], AOM_QM_BITS);
  cmp_mask2 = _mm_cmpgt_epi32(coeff[2], threshold[1]);
  coeff[3] = _mm_slli_epi32(coeff[3], AOM_QM_BITS);
  cmp_mask3 = _mm_cmpgt_epi32(coeff[3], threshold[1]);

  cmp_mask0 = _mm_packs_epi32(cmp_mask0, cmp_mask1);
  cmp_mask1 = _mm_packs_epi32(cmp_mask2, cmp_mask3);

  update_mask1(&cmp_mask0, &cmp_mask1, iscan_ptr, is_found, mask);
}

static inline int calculate_non_zero_count(__m128i mask) {
  __m128i mask0, mask1;
  int non_zero_count = 0;
  mask0 = _mm_unpackhi_epi64(mask, mask);
  mask1 = _mm_max_epi16(mask0, mask);
  mask0 = _mm_shuffle_epi32(mask1, 1);
  mask0 = _mm_max_epi16(mask0, mask1);
  mask1 = _mm_srli_epi32(mask0, 16);
  mask0 = _mm_max_epi16(mask0, mask1);
  non_zero_count = _mm_extract_epi16(mask0, 0) + 1;

  return non_zero_count;
}
