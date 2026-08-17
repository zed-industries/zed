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

#ifndef AOM_AOM_DSP_SIMD_V128_INTRINSICS_X86_H_
#define AOM_AOM_DSP_SIMD_V128_INTRINSICS_X86_H_

#include <stdint.h>
#include "aom_dsp/simd/v64_intrinsics_x86.h"

typedef __m128i v128;

SIMD_INLINE uint32_t v128_low_u32(v128 a) {
  return (uint32_t)_mm_cvtsi128_si32(a);
}

SIMD_INLINE v64 v128_low_v64(v128 a) {
  return _mm_unpacklo_epi64(a, v64_zero());
}

SIMD_INLINE v64 v128_high_v64(v128 a) { return _mm_srli_si128(a, 8); }

SIMD_INLINE v128 v128_from_v64(v64 a, v64 b) {
  return _mm_unpacklo_epi64(b, a);
}

SIMD_INLINE v128 v128_from_64(uint64_t a, uint64_t b) {
  return v128_from_v64(v64_from_64(a), v64_from_64(b));
}

SIMD_INLINE v128 v128_from_32(uint32_t a, uint32_t b, uint32_t c, uint32_t d) {
  return _mm_set_epi32((int)a, (int)b, (int)c, (int)d);
}

SIMD_INLINE v128 v128_load_aligned(const void *p) {
  return _mm_load_si128((__m128i *)p);
}

SIMD_INLINE v128 v128_load_unaligned(const void *p) {
#if defined(__SSSE3__)
  return _mm_lddqu_si128((__m128i *)p);
#else
  return _mm_loadu_si128((__m128i *)p);
#endif
}

SIMD_INLINE void v128_store_aligned(void *p, v128 a) {
  _mm_store_si128((__m128i *)p, a);
}

SIMD_INLINE void v128_store_unaligned(void *p, v128 a) {
  _mm_storeu_si128((__m128i *)p, a);
}

// The following function requires an immediate.
// Some compilers will check this during optimisation, others wont.
#if defined(__OPTIMIZE__) && __OPTIMIZE__ && !defined(__clang__)
#if defined(__SSSE3__)
SIMD_INLINE v128 v128_align(v128 a, v128 b, const unsigned int c) {
  return c ? _mm_alignr_epi8(a, b, c) : b;
}
#else
#define v128_align(a, b, c) \
  ((c) ? _mm_or_si128(_mm_srli_si128(b, c), _mm_slli_si128(a, 16 - (c))) : (b))
#endif
#else
#if defined(__SSSE3__)
#define v128_align(a, b, c) ((c) ? _mm_alignr_epi8(a, b, (uint8_t)(c)) : (b))
#else
#define v128_align(a, b, c) \
  ((c) ? _mm_or_si128(_mm_srli_si128(b, c), _mm_slli_si128(a, 16 - (c))) : (b))
#endif
#endif

SIMD_INLINE v128 v128_zero(void) { return _mm_setzero_si128(); }

SIMD_INLINE v128 v128_dup_8(uint8_t x) { return _mm_set1_epi8((char)x); }

SIMD_INLINE v128 v128_dup_16(uint16_t x) { return _mm_set1_epi16((short)x); }

SIMD_INLINE v128 v128_dup_32(uint32_t x) { return _mm_set1_epi32((int)x); }

SIMD_INLINE v128 v128_dup_64(uint64_t x) {
  // _mm_set_pi64x and _mm_cvtsi64x_si64 missing in some compilers
  return _mm_set_epi32((int32_t)(x >> 32), (int32_t)x, (int32_t)(x >> 32),
                       (int32_t)x);
}

SIMD_INLINE v128 v128_add_8(v128 a, v128 b) { return _mm_add_epi8(a, b); }

SIMD_INLINE v128 v128_add_16(v128 a, v128 b) { return _mm_add_epi16(a, b); }

SIMD_INLINE v128 v128_sadd_u8(v128 a, v128 b) { return _mm_adds_epu8(a, b); }

SIMD_INLINE v128 v128_sadd_s8(v128 a, v128 b) { return _mm_adds_epi8(a, b); }

SIMD_INLINE v128 v128_sadd_s16(v128 a, v128 b) { return _mm_adds_epi16(a, b); }

SIMD_INLINE v128 v128_add_32(v128 a, v128 b) { return _mm_add_epi32(a, b); }

SIMD_INLINE v128 v128_add_64(v128 a, v128 b) { return _mm_add_epi64(a, b); }

SIMD_INLINE v128 v128_padd_s16(v128 a) {
  return _mm_madd_epi16(a, _mm_set1_epi16(1));
}

SIMD_INLINE v128 v128_sub_8(v128 a, v128 b) { return _mm_sub_epi8(a, b); }

SIMD_INLINE v128 v128_ssub_u8(v128 a, v128 b) { return _mm_subs_epu8(a, b); }

SIMD_INLINE v128 v128_ssub_s8(v128 a, v128 b) { return _mm_subs_epi8(a, b); }

SIMD_INLINE v128 v128_sub_16(v128 a, v128 b) { return _mm_sub_epi16(a, b); }

SIMD_INLINE v128 v128_ssub_s16(v128 a, v128 b) { return _mm_subs_epi16(a, b); }

SIMD_INLINE v128 v128_ssub_u16(v128 a, v128 b) { return _mm_subs_epu16(a, b); }

SIMD_INLINE v128 v128_sub_32(v128 a, v128 b) { return _mm_sub_epi32(a, b); }

SIMD_INLINE v128 v128_sub_64(v128 a, v128 b) { return _mm_sub_epi64(a, b); }

SIMD_INLINE v128 v128_abs_s16(v128 a) {
#if defined(__SSSE3__)
  return _mm_abs_epi16(a);
#else
  return _mm_max_epi16(a, _mm_sub_epi16(_mm_setzero_si128(), a));
#endif
}

SIMD_INLINE v128 v128_abs_s8(v128 a) {
#if defined(__SSSE3__)
  return _mm_abs_epi8(a);
#else
  v128 sign = _mm_cmplt_epi8(a, _mm_setzero_si128());
  return _mm_xor_si128(sign, _mm_add_epi8(a, sign));
#endif
}

SIMD_INLINE v128 v128_ziplo_8(v128 a, v128 b) {
  return _mm_unpacklo_epi8(b, a);
}

SIMD_INLINE v128 v128_ziphi_8(v128 a, v128 b) {
  return _mm_unpackhi_epi8(b, a);
}

SIMD_INLINE v128 v128_ziplo_16(v128 a, v128 b) {
  return _mm_unpacklo_epi16(b, a);
}

SIMD_INLINE v128 v128_ziphi_16(v128 a, v128 b) {
  return _mm_unpackhi_epi16(b, a);
}

SIMD_INLINE v128 v128_ziplo_32(v128 a, v128 b) {
  return _mm_unpacklo_epi32(b, a);
}

SIMD_INLINE v128 v128_ziphi_32(v128 a, v128 b) {
  return _mm_unpackhi_epi32(b, a);
}

SIMD_INLINE v128 v128_ziplo_64(v128 a, v128 b) {
  return _mm_unpacklo_epi64(b, a);
}

SIMD_INLINE v128 v128_ziphi_64(v128 a, v128 b) {
  return _mm_unpackhi_epi64(b, a);
}

SIMD_INLINE v128 v128_zip_8(v64 a, v64 b) { return _mm_unpacklo_epi8(b, a); }

SIMD_INLINE v128 v128_zip_16(v64 a, v64 b) { return _mm_unpacklo_epi16(b, a); }

SIMD_INLINE v128 v128_zip_32(v64 a, v64 b) { return _mm_unpacklo_epi32(b, a); }

SIMD_INLINE v128 v128_unziphi_8(v128 a, v128 b) {
  return _mm_packs_epi16(_mm_srai_epi16(b, 8), _mm_srai_epi16(a, 8));
}

SIMD_INLINE v128 v128_unziplo_8(v128 a, v128 b) {
#if defined(__SSSE3__)
#ifdef __x86_64__
  v128 order = _mm_cvtsi64_si128(0x0e0c0a0806040200LL);
#else
  v128 order = _mm_set_epi32(0, 0, 0x0e0c0a08, 0x06040200);
#endif
  return _mm_unpacklo_epi64(_mm_shuffle_epi8(b, order),
                            _mm_shuffle_epi8(a, order));
#else
  return v128_unziphi_8(_mm_slli_si128(a, 1), _mm_slli_si128(b, 1));
#endif
}

SIMD_INLINE v128 v128_unziphi_16(v128 a, v128 b) {
  return _mm_packs_epi32(_mm_srai_epi32(b, 16), _mm_srai_epi32(a, 16));
}

SIMD_INLINE v128 v128_unziplo_16(v128 a, v128 b) {
#if defined(__SSSE3__)
#ifdef __x86_64__
  v128 order = _mm_cvtsi64_si128(0x0d0c090805040100LL);
#else
  v128 order = _mm_set_epi32(0, 0, 0x0d0c0908, 0x05040100);
#endif
  return _mm_unpacklo_epi64(_mm_shuffle_epi8(b, order),
                            _mm_shuffle_epi8(a, order));
#else
  return v128_unziphi_16(_mm_slli_si128(a, 2), _mm_slli_si128(b, 2));
#endif
}

SIMD_INLINE v128 v128_unziphi_32(v128 a, v128 b) {
  return _mm_castps_si128(_mm_shuffle_ps(
      _mm_castsi128_ps(b), _mm_castsi128_ps(a), _MM_SHUFFLE(3, 1, 3, 1)));
}

SIMD_INLINE v128 v128_unziplo_32(v128 a, v128 b) {
  return _mm_castps_si128(_mm_shuffle_ps(
      _mm_castsi128_ps(b), _mm_castsi128_ps(a), _MM_SHUFFLE(2, 0, 2, 0)));
}

SIMD_INLINE v128 v128_unpack_u8_s16(v64 a) {
  return _mm_unpacklo_epi8(a, _mm_setzero_si128());
}

SIMD_INLINE v128 v128_unpacklo_u8_s16(v128 a) {
  return _mm_unpacklo_epi8(a, _mm_setzero_si128());
}

SIMD_INLINE v128 v128_unpackhi_u8_s16(v128 a) {
  return _mm_unpackhi_epi8(a, _mm_setzero_si128());
}

SIMD_INLINE v128 v128_unpack_s8_s16(v64 a) {
  return _mm_srai_epi16(_mm_unpacklo_epi8(a, a), 8);
}

SIMD_INLINE v128 v128_unpacklo_s8_s16(v128 a) {
  return _mm_srai_epi16(_mm_unpacklo_epi8(a, a), 8);
}

SIMD_INLINE v128 v128_unpackhi_s8_s16(v128 a) {
  return _mm_srai_epi16(_mm_unpackhi_epi8(a, a), 8);
}

SIMD_INLINE v128 v128_pack_s32_s16(v128 a, v128 b) {
  return _mm_packs_epi32(b, a);
}

SIMD_INLINE v128 v128_pack_s32_u16(v128 a, v128 b) {
#if defined(__SSE4_1__)
  return _mm_packus_epi32(b, a);
#else
  return v128_from_v64(v64_pack_s32_u16(v128_high_v64(a), v128_low_v64(a)),
                       v64_pack_s32_u16(v128_high_v64(b), v128_low_v64(b)));
#endif
}

SIMD_INLINE v128 v128_pack_s16_u8(v128 a, v128 b) {
  return _mm_packus_epi16(b, a);
}

SIMD_INLINE v128 v128_pack_s16_s8(v128 a, v128 b) {
  return _mm_packs_epi16(b, a);
}

SIMD_INLINE v128 v128_unpack_u16_s32(v64 a) {
  return _mm_unpacklo_epi16(a, _mm_setzero_si128());
}

SIMD_INLINE v128 v128_unpack_s16_s32(v64 a) {
  return _mm_srai_epi32(_mm_unpacklo_epi16(a, a), 16);
}

SIMD_INLINE v128 v128_unpacklo_u16_s32(v128 a) {
  return _mm_unpacklo_epi16(a, _mm_setzero_si128());
}

SIMD_INLINE v128 v128_unpacklo_s16_s32(v128 a) {
  return _mm_srai_epi32(_mm_unpacklo_epi16(a, a), 16);
}

SIMD_INLINE v128 v128_unpackhi_u16_s32(v128 a) {
  return _mm_unpackhi_epi16(a, _mm_setzero_si128());
}

SIMD_INLINE v128 v128_unpackhi_s16_s32(v128 a) {
  return _mm_srai_epi32(_mm_unpackhi_epi16(a, a), 16);
}

SIMD_INLINE v128 v128_shuffle_8(v128 x, v128 pattern) {
#if defined(__SSSE3__)
  return _mm_shuffle_epi8(x, pattern);
#else
  v128 output;
  unsigned char *input = (unsigned char *)&x;
  unsigned char *index = (unsigned char *)&pattern;
  unsigned char *selected = (unsigned char *)&output;
  int counter;

  for (counter = 0; counter < 16; counter++) {
    selected[counter] = input[index[counter] & 15];
  }

  return output;
#endif
}

SIMD_INLINE int64_t v128_dotp_su8(v128 a, v128 b) {
  v128 t1 = _mm_madd_epi16(v128_unpackhi_s8_s16(a), v128_unpackhi_u8_s16(b));
  v128 t2 = _mm_madd_epi16(v128_unpacklo_s8_s16(a), v128_unpacklo_u8_s16(b));
  v128 t = v128_add_32(t1, t2);
  t = v128_add_32(t, _mm_srli_si128(t, 8));
  t = v128_add_32(t, _mm_srli_si128(t, 4));
  return (int32_t)v128_low_u32(t);
}

SIMD_INLINE int64_t v128_dotp_s16(v128 a, v128 b) {
  v128 r = _mm_madd_epi16(a, b);
#if defined(__SSE4_1__) && defined(__x86_64__)
  v128 c = _mm_add_epi64(_mm_cvtepi32_epi64(r),
                         _mm_cvtepi32_epi64(_mm_srli_si128(r, 8)));
  return _mm_cvtsi128_si64(_mm_add_epi64(c, _mm_srli_si128(c, 8)));
#else
  return (int64_t)_mm_cvtsi128_si32(r) +
         (int64_t)_mm_cvtsi128_si32(_mm_srli_si128(r, 4)) +
         (int64_t)_mm_cvtsi128_si32(_mm_srli_si128(r, 8)) +
         (int64_t)_mm_cvtsi128_si32(_mm_srli_si128(r, 12));
#endif
}

SIMD_INLINE uint64_t v128_hadd_u8(v128 a) {
  v128 t = _mm_sad_epu8(a, _mm_setzero_si128());
  return v64_low_u32(v128_low_v64(t)) + v64_low_u32(v128_high_v64(t));
}

typedef v128 sad128_internal;

SIMD_INLINE sad128_internal v128_sad_u8_init(void) {
  return _mm_setzero_si128();
}

/* Implementation dependent return value.  Result must be finalised with
   v128_sad_sum().
   The result for more than 32 v128_sad_u8() calls is undefined. */
SIMD_INLINE sad128_internal v128_sad_u8(sad128_internal s, v128 a, v128 b) {
  return _mm_add_epi64(s, _mm_sad_epu8(a, b));
}

SIMD_INLINE uint32_t v128_sad_u8_sum(sad128_internal s) {
  return v128_low_u32(_mm_add_epi32(s, _mm_unpackhi_epi64(s, s)));
}

typedef int32_t ssd128_internal;

SIMD_INLINE ssd128_internal v128_ssd_u8_init(void) { return 0; }

/* Implementation dependent return value.  Result must be finalised with
 * v128_ssd_sum(). */
SIMD_INLINE ssd128_internal v128_ssd_u8(ssd128_internal s, v128 a, v128 b) {
  v128 z = _mm_setzero_si128();
  v128 l = _mm_sub_epi16(_mm_unpacklo_epi8(a, z), _mm_unpacklo_epi8(b, z));
  v128 h = _mm_sub_epi16(_mm_unpackhi_epi8(a, z), _mm_unpackhi_epi8(b, z));
  v128 rl = _mm_madd_epi16(l, l);
  v128 rh = _mm_madd_epi16(h, h);
  v128 r = _mm_add_epi32(rl, rh);
  r = _mm_add_epi32(r, _mm_srli_si128(r, 8));
  r = _mm_add_epi32(r, _mm_srli_si128(r, 4));
  return s + _mm_cvtsi128_si32(r);
}

SIMD_INLINE int32_t v128_ssd_u8_sum(ssd128_internal s) { return s; }

SIMD_INLINE v128 v128_or(v128 a, v128 b) { return _mm_or_si128(a, b); }

SIMD_INLINE v128 v128_xor(v128 a, v128 b) { return _mm_xor_si128(a, b); }

SIMD_INLINE v128 v128_and(v128 a, v128 b) { return _mm_and_si128(a, b); }

SIMD_INLINE v128 v128_andn(v128 a, v128 b) { return _mm_andnot_si128(b, a); }

SIMD_INLINE v128 v128_mul_s16(v64 a, v64 b) {
  v64 lo_bits = v64_mullo_s16(a, b);
  v64 hi_bits = v64_mulhi_s16(a, b);
  return v128_from_v64(v64_ziphi_16(hi_bits, lo_bits),
                       v64_ziplo_16(hi_bits, lo_bits));
}

SIMD_INLINE v128 v128_mullo_s16(v128 a, v128 b) {
  return _mm_mullo_epi16(a, b);
}

SIMD_INLINE v128 v128_mulhi_s16(v128 a, v128 b) {
  return _mm_mulhi_epi16(a, b);
}

SIMD_INLINE v128 v128_mullo_s32(v128 a, v128 b) {
#if defined(__SSE4_1__)
  return _mm_mullo_epi32(a, b);
#else
  return _mm_unpacklo_epi32(
      _mm_shuffle_epi32(_mm_mul_epu32(a, b), 8),
      _mm_shuffle_epi32(
          _mm_mul_epu32(_mm_srli_si128(a, 4), _mm_srli_si128(b, 4)), 8));
#endif
}

SIMD_INLINE int64_t v128_dotp_s32(v128 a, v128 b) {
  v128 r = v128_mullo_s32(a, b);
  return (int64_t)_mm_cvtsi128_si32(r) +
         (int64_t)_mm_cvtsi128_si32(_mm_srli_si128(r, 4)) +
         (int64_t)_mm_cvtsi128_si32(_mm_srli_si128(r, 8)) +
         (int64_t)_mm_cvtsi128_si32(_mm_srli_si128(r, 12));
}

SIMD_INLINE v128 v128_madd_s16(v128 a, v128 b) { return _mm_madd_epi16(a, b); }

SIMD_INLINE v128 v128_madd_us8(v128 a, v128 b) {
#if defined(__SSSE3__)
  return _mm_maddubs_epi16(a, b);
#else
  return _mm_packs_epi32(
      _mm_madd_epi16(_mm_unpacklo_epi8(a, _mm_setzero_si128()),
                     _mm_srai_epi16(_mm_unpacklo_epi8(b, b), 8)),
      _mm_madd_epi16(_mm_unpackhi_epi8(a, _mm_setzero_si128()),
                     _mm_srai_epi16(_mm_unpackhi_epi8(b, b), 8)));
#endif
}

SIMD_INLINE v128 v128_padd_u8(v128 a) {
  return v128_madd_us8(a, _mm_set1_epi8(1));
}

SIMD_INLINE v128 v128_avg_u8(v128 a, v128 b) { return _mm_avg_epu8(a, b); }

SIMD_INLINE v128 v128_rdavg_u8(v128 a, v128 b) {
  return _mm_sub_epi8(_mm_avg_epu8(a, b),
                      _mm_and_si128(_mm_xor_si128(a, b), v128_dup_8(1)));
}

SIMD_INLINE v128 v128_rdavg_u16(v128 a, v128 b) {
  return _mm_sub_epi16(_mm_avg_epu16(a, b),
                       _mm_and_si128(_mm_xor_si128(a, b), v128_dup_16(1)));
}

SIMD_INLINE v128 v128_avg_u16(v128 a, v128 b) { return _mm_avg_epu16(a, b); }

SIMD_INLINE v128 v128_min_u8(v128 a, v128 b) { return _mm_min_epu8(a, b); }

SIMD_INLINE v128 v128_max_u8(v128 a, v128 b) { return _mm_max_epu8(a, b); }

SIMD_INLINE v128 v128_min_s8(v128 a, v128 b) {
#if defined(__SSE4_1__)
  return _mm_min_epi8(a, b);
#else
  v128 mask = _mm_cmplt_epi8(a, b);
  return _mm_or_si128(_mm_andnot_si128(mask, b), _mm_and_si128(mask, a));
#endif
}

SIMD_INLINE uint32_t v128_movemask_8(v128 a) { return _mm_movemask_epi8(a); }

SIMD_INLINE v128 v128_blend_8(v128 a, v128 b, v128 c) {
#if defined(__SSE4_1__)
  return _mm_blendv_epi8(a, b, c);
#else
  c = _mm_cmplt_epi8(c, v128_zero());
  return v128_or(v128_and(b, c), v128_andn(a, c));
#endif
}

SIMD_INLINE v128 v128_max_s8(v128 a, v128 b) {
#if defined(__SSE4_1__)
  return _mm_max_epi8(a, b);
#else
  v128 mask = _mm_cmplt_epi8(b, a);
  return _mm_or_si128(_mm_andnot_si128(mask, b), _mm_and_si128(mask, a));
#endif
}

SIMD_INLINE v128 v128_min_s16(v128 a, v128 b) { return _mm_min_epi16(a, b); }

SIMD_INLINE v128 v128_max_s16(v128 a, v128 b) { return _mm_max_epi16(a, b); }

SIMD_INLINE v128 v128_min_s32(v128 a, v128 b) {
#if defined(__SSE4_1__)
  return _mm_min_epi32(a, b);
#else
  v128 mask = _mm_cmplt_epi32(a, b);
  return _mm_or_si128(_mm_andnot_si128(mask, b), _mm_and_si128(mask, a));
#endif
}

SIMD_INLINE v128 v128_max_s32(v128 a, v128 b) {
#if defined(__SSE4_1__)
  return _mm_max_epi32(a, b);
#else
  v128 mask = _mm_cmplt_epi32(b, a);
  return _mm_or_si128(_mm_andnot_si128(mask, b), _mm_and_si128(mask, a));
#endif
}

SIMD_INLINE v128 v128_cmpgt_s8(v128 a, v128 b) { return _mm_cmpgt_epi8(a, b); }

SIMD_INLINE v128 v128_cmplt_s8(v128 a, v128 b) { return _mm_cmplt_epi8(a, b); }

SIMD_INLINE v128 v128_cmpeq_8(v128 a, v128 b) { return _mm_cmpeq_epi8(a, b); }

SIMD_INLINE v128 v128_cmpgt_s16(v128 a, v128 b) {
  return _mm_cmpgt_epi16(a, b);
}

SIMD_INLINE v128 v128_cmplt_s16(v128 a, v128 b) {
  return _mm_cmplt_epi16(a, b);
}

SIMD_INLINE v128 v128_cmpeq_32(v128 a, v128 b) { return _mm_cmpeq_epi32(a, b); }

SIMD_INLINE v128 v128_cmpgt_s32(v128 a, v128 b) {
  return _mm_cmpgt_epi32(a, b);
}

SIMD_INLINE v128 v128_cmplt_s32(v128 a, v128 b) {
  return _mm_cmplt_epi32(a, b);
}

SIMD_INLINE v128 v128_cmpeq_16(v128 a, v128 b) { return _mm_cmpeq_epi16(a, b); }

SIMD_INLINE v128 v128_shl_8(v128 a, unsigned int c) {
  return _mm_and_si128(_mm_set1_epi8((char)(0xff << c)),
                       _mm_sll_epi16(a, _mm_cvtsi32_si128((int)c)));
}

SIMD_INLINE v128 v128_shr_u8(v128 a, unsigned int c) {
  return _mm_and_si128(_mm_set1_epi8((char)(0xff >> c)),
                       _mm_srl_epi16(a, _mm_cvtsi32_si128((int)c)));
}

SIMD_INLINE v128 v128_shr_s8(v128 a, unsigned int c) {
  __m128i x = _mm_cvtsi32_si128((int)(c + 8));
  return _mm_packs_epi16(_mm_sra_epi16(_mm_unpacklo_epi8(a, a), x),
                         _mm_sra_epi16(_mm_unpackhi_epi8(a, a), x));
}

SIMD_INLINE v128 v128_shl_16(v128 a, unsigned int c) {
  return _mm_sll_epi16(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shr_u16(v128 a, unsigned int c) {
  return _mm_srl_epi16(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shr_s16(v128 a, unsigned int c) {
  return _mm_sra_epi16(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shl_32(v128 a, unsigned int c) {
  return _mm_sll_epi32(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shr_u32(v128 a, unsigned int c) {
  return _mm_srl_epi32(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shr_s32(v128 a, unsigned int c) {
  return _mm_sra_epi32(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shl_64(v128 a, unsigned int c) {
  return _mm_sll_epi64(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shr_u64(v128 a, unsigned int c) {
  return _mm_srl_epi64(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v128 v128_shr_s64(v128 a, unsigned int c) {
  // _mm_sra_epi64 is missing in gcc?
  return v128_from_64((uint64_t)((int64_t)v64_u64(v128_high_v64(a)) >> c),
                      (uint64_t)((int64_t)v64_u64(v128_low_v64(a)) >> c));
  // return _mm_sra_epi64(a, _mm_cvtsi32_si128((int)c));
}

/* These intrinsics require immediate values, so we must use #defines
   to enforce that. */
#define v128_shl_n_byte(a, c) _mm_slli_si128(a, (c)&127)
#define v128_shr_n_byte(a, c) _mm_srli_si128(a, (c)&127)
#define v128_shl_n_8(a, c) \
  _mm_and_si128(_mm_set1_epi8((char)(0xff << (c))), _mm_slli_epi16(a, c))
#define v128_shr_n_u8(a, c) \
  _mm_and_si128(_mm_set1_epi8((char)(0xff >> (c))), _mm_srli_epi16(a, c))
#define v128_shr_n_s8(a, c)                                         \
  _mm_packs_epi16(_mm_srai_epi16(_mm_unpacklo_epi8(a, a), (c) + 8), \
                  _mm_srai_epi16(_mm_unpackhi_epi8(a, a), (c) + 8))
#define v128_shl_n_16(a, c) _mm_slli_epi16(a, c)
#define v128_shr_n_u16(a, c) _mm_srli_epi16(a, c)
#define v128_shr_n_s16(a, c) _mm_srai_epi16(a, c)
#define v128_shl_n_32(a, c) _mm_slli_epi32(a, c)
#define v128_shr_n_u32(a, c) _mm_srli_epi32(a, c)
#define v128_shr_n_s32(a, c) _mm_srai_epi32(a, c)
#define v128_shl_n_64(a, c) _mm_slli_epi64(a, c)
#define v128_shr_n_u64(a, c) _mm_srli_epi64(a, c)
#define v128_shr_n_s64(a, c) \
  v128_shr_s64(a, c)  // _mm_srai_epi64 missing in gcc?

typedef v128 sad128_internal_u16;

SIMD_INLINE sad128_internal_u16 v128_sad_u16_init(void) { return v128_zero(); }

/* Implementation dependent return value.  Result must be finalised with
 * v128_sad_u16_sum(). */
SIMD_INLINE sad128_internal_u16 v128_sad_u16(sad128_internal_u16 s, v128 a,
                                             v128 b) {
#if defined(__SSE4_1__)
  v128 t = v128_sub_16(_mm_max_epu16(a, b), _mm_min_epu16(a, b));
#else
  v128 t = v128_cmplt_s16(v128_xor(a, v128_dup_16(32768)),
                          v128_xor(b, v128_dup_16(32768)));
  t = v128_sub_16(v128_or(v128_and(b, t), v128_andn(a, t)),
                  v128_or(v128_and(a, t), v128_andn(b, t)));
#endif
  return v128_add_32(
      s, v128_add_32(v128_unpackhi_u16_s32(t), v128_unpacklo_u16_s32(t)));
}

SIMD_INLINE uint32_t v128_sad_u16_sum(sad128_internal_u16 s) {
  return v128_low_u32(s) + v128_low_u32(v128_shr_n_byte(s, 4)) +
         v128_low_u32(v128_shr_n_byte(s, 8)) +
         v128_low_u32(v128_shr_n_byte(s, 12));
}

typedef v128 ssd128_internal_s16;

SIMD_INLINE ssd128_internal_s16 v128_ssd_s16_init(void) { return v128_zero(); }

/* Implementation dependent return value.  Result must be finalised with
 * v128_ssd_s16_sum(). */
SIMD_INLINE ssd128_internal_s16 v128_ssd_s16(ssd128_internal_s16 s, v128 a,
                                             v128 b) {
  v128 d = v128_sub_16(a, b);
  d = v128_madd_s16(d, d);
  return v128_add_64(s, v128_add_64(_mm_unpackhi_epi32(d, v128_zero()),
                                    _mm_unpacklo_epi32(d, v128_zero())));
}

SIMD_INLINE uint64_t v128_ssd_s16_sum(ssd128_internal_s16 s) {
  return v64_u64(v128_low_v64(s)) + v64_u64(v128_high_v64(s));
}

#endif  // AOM_AOM_DSP_SIMD_V128_INTRINSICS_X86_H_
