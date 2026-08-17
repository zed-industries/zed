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

#ifndef AOM_AOM_DSP_SIMD_V64_INTRINSICS_X86_H_
#define AOM_AOM_DSP_SIMD_V64_INTRINSICS_X86_H_

#include <emmintrin.h>
#if defined(__SSSE3__)
#include <tmmintrin.h>
#endif
#if defined(__SSE4_1__)
#include <smmintrin.h>
#endif

typedef __m128i v64;

SIMD_INLINE uint32_t v64_low_u32(v64 a) {
  return (uint32_t)_mm_cvtsi128_si32(a);
}

SIMD_INLINE uint32_t v64_high_u32(v64 a) {
  return (uint32_t)_mm_cvtsi128_si32(_mm_srli_si128(a, 4));
}

SIMD_INLINE int32_t v64_low_s32(v64 a) { return (int32_t)_mm_cvtsi128_si32(a); }

SIMD_INLINE int32_t v64_high_s32(v64 a) {
  return (int32_t)_mm_cvtsi128_si32(_mm_srli_si128(a, 4));
}

SIMD_INLINE v64 v64_from_16(uint16_t a, uint16_t b, uint16_t c, uint16_t d) {
  return _mm_packs_epi32(
      _mm_set_epi32((int16_t)a, (int16_t)b, (int16_t)c, (int16_t)d),
      _mm_setzero_si128());
}

SIMD_INLINE v64 v64_from_32(uint32_t x, uint32_t y) {
  return _mm_set_epi32(0, 0, (int32_t)x, (int32_t)y);
}

SIMD_INLINE v64 v64_from_64(uint64_t x) {
#ifdef __x86_64__
  return _mm_cvtsi64_si128((int64_t)x);
#else
  return _mm_set_epi32(0, 0, (int32_t)(x >> 32), (int32_t)x);
#endif
}

SIMD_INLINE uint64_t v64_u64(v64 x) {
  return (uint64_t)v64_low_u32(x) | ((uint64_t)v64_high_u32(x) << 32);
}

SIMD_INLINE uint32_t u32_load_aligned(const void *p) {
  return *((uint32_t *)p);
}

SIMD_INLINE uint32_t u32_load_unaligned(const void *p) {
  return *((uint32_t *)p);
}

SIMD_INLINE void u32_store_aligned(void *p, uint32_t a) {
  *((uint32_t *)p) = a;
}

SIMD_INLINE void u32_store_unaligned(void *p, uint32_t a) {
  *((uint32_t *)p) = a;
}

SIMD_INLINE v64 v64_load_aligned(const void *p) {
  return _mm_loadl_epi64((__m128i *)p);
}

SIMD_INLINE v64 v64_load_unaligned(const void *p) {
  return _mm_loadl_epi64((__m128i *)p);
}

SIMD_INLINE void v64_store_aligned(void *p, v64 a) {
  _mm_storel_epi64((__m128i *)p, a);
}

SIMD_INLINE void v64_store_unaligned(void *p, v64 a) {
  _mm_storel_epi64((__m128i *)p, a);
}

#if defined(__OPTIMIZE__) && __OPTIMIZE__ && !defined(__clang__)
#define v64_align(a, b, c) \
  ((c) ? _mm_srli_si128(_mm_unpacklo_epi64(b, a), (c)) : b)
#else
#define v64_align(a, b, c)                                                  \
  ((c) ? v64_from_64((v64_u64(b) >> (c)*8) | (v64_u64(a) << (8 - (c)) * 8)) \
       : (b))
#endif

SIMD_INLINE v64 v64_zero(void) { return _mm_setzero_si128(); }

SIMD_INLINE v64 v64_dup_8(uint8_t x) { return _mm_set1_epi8((char)x); }

SIMD_INLINE v64 v64_dup_16(uint16_t x) { return _mm_set1_epi16((short)x); }

SIMD_INLINE v64 v64_dup_32(uint32_t x) { return _mm_set1_epi32((int)x); }

SIMD_INLINE v64 v64_add_8(v64 a, v64 b) { return _mm_add_epi8(a, b); }

SIMD_INLINE v64 v64_add_16(v64 a, v64 b) { return _mm_add_epi16(a, b); }

SIMD_INLINE v64 v64_sadd_u8(v64 a, v64 b) { return _mm_adds_epu8(a, b); }

SIMD_INLINE v64 v64_sadd_s8(v64 a, v64 b) { return _mm_adds_epi8(a, b); }

SIMD_INLINE v64 v64_sadd_s16(v64 a, v64 b) { return _mm_adds_epi16(a, b); }

SIMD_INLINE v64 v64_add_32(v64 a, v64 b) { return _mm_add_epi32(a, b); }

SIMD_INLINE v64 v64_sub_8(v64 a, v64 b) { return _mm_sub_epi8(a, b); }

SIMD_INLINE v64 v64_ssub_u8(v64 a, v64 b) { return _mm_subs_epu8(a, b); }

SIMD_INLINE v64 v64_ssub_s8(v64 a, v64 b) { return _mm_subs_epi8(a, b); }

SIMD_INLINE v64 v64_sub_16(v64 a, v64 b) { return _mm_sub_epi16(a, b); }

SIMD_INLINE v64 v64_ssub_s16(v64 a, v64 b) { return _mm_subs_epi16(a, b); }

SIMD_INLINE v64 v64_ssub_u16(v64 a, v64 b) { return _mm_subs_epu16(a, b); }

SIMD_INLINE v64 v64_sub_32(v64 a, v64 b) { return _mm_sub_epi32(a, b); }

SIMD_INLINE v64 v64_abs_s16(v64 a) {
#if defined(__SSSE3__)
  return _mm_abs_epi16(a);
#else
  return _mm_max_epi16(a, _mm_sub_epi16(_mm_setzero_si128(), a));
#endif
}

SIMD_INLINE v64 v64_abs_s8(v64 a) {
#if defined(__SSSE3__)
  return _mm_abs_epi8(a);
#else
  v64 sign = _mm_cmplt_epi8(a, _mm_setzero_si128());
  return _mm_xor_si128(sign, _mm_add_epi8(a, sign));
#endif
}

SIMD_INLINE v64 v64_ziplo_8(v64 a, v64 b) { return _mm_unpacklo_epi8(b, a); }

SIMD_INLINE v64 v64_ziphi_8(v64 a, v64 b) {
  return _mm_srli_si128(_mm_unpacklo_epi8(b, a), 8);
}

SIMD_INLINE v64 v64_ziplo_16(v64 a, v64 b) { return _mm_unpacklo_epi16(b, a); }

SIMD_INLINE v64 v64_ziphi_16(v64 a, v64 b) {
  return _mm_srli_si128(_mm_unpacklo_epi16(b, a), 8);
}

SIMD_INLINE v64 v64_ziplo_32(v64 a, v64 b) { return _mm_unpacklo_epi32(b, a); }

SIMD_INLINE v64 v64_ziphi_32(v64 a, v64 b) {
  return _mm_srli_si128(_mm_unpacklo_epi32(b, a), 8);
}

SIMD_INLINE v64 v64_pack_s32_s16(v64 a, v64 b) {
  __m128i t = _mm_unpacklo_epi64(b, a);
  return _mm_packs_epi32(t, t);
}

SIMD_INLINE v64 v64_pack_s32_u16(v64 a, v64 b) {
#if defined(__SSE4_1__)
  __m128i t = _mm_unpacklo_epi64(b, a);
  return _mm_packus_epi32(t, t);
#else
  const int32_t ah = SIMD_CLAMP(v64_high_s32(a), 0, 65535);
  const int32_t al = SIMD_CLAMP(v64_low_s32(a), 0, 65535);
  const int32_t bh = SIMD_CLAMP(v64_high_s32(b), 0, 65535);
  const int32_t bl = SIMD_CLAMP(v64_low_s32(b), 0, 65535);
  return v64_from_16(ah, al, bh, bl);
#endif
}

SIMD_INLINE v64 v64_pack_s16_u8(v64 a, v64 b) {
  __m128i t = _mm_unpacklo_epi64(b, a);
  return _mm_packus_epi16(t, t);
}

SIMD_INLINE v64 v64_pack_s16_s8(v64 a, v64 b) {
  __m128i t = _mm_unpacklo_epi64(b, a);
  return _mm_packs_epi16(t, t);
}

SIMD_INLINE v64 v64_unziphi_8(v64 a, v64 b) {
#if defined(__SSSE3__)
  return _mm_shuffle_epi8(_mm_unpacklo_epi64(b, a),
                          v64_from_64(0x0f0d0b0907050301LL));
#else
  return _mm_packus_epi16(
      _mm_unpacklo_epi64(_mm_srli_epi16(b, 8), _mm_srli_epi16(a, 8)),
      _mm_setzero_si128());
#endif
}

SIMD_INLINE v64 v64_unziplo_8(v64 a, v64 b) {
#if defined(__SSSE3__)
  return _mm_shuffle_epi8(_mm_unpacklo_epi64(b, a),
                          v64_from_64(0x0e0c0a0806040200LL));
#else
  return v64_unziphi_8(_mm_slli_si128(a, 1), _mm_slli_si128(b, 1));
#endif
}

SIMD_INLINE v64 v64_unziphi_16(v64 a, v64 b) {
#if defined(__SSSE3__)
  return _mm_shuffle_epi8(_mm_unpacklo_epi64(b, a),
                          v64_from_64(0x0f0e0b0a07060302LL));
#else
  return _mm_packs_epi32(
      _mm_unpacklo_epi64(_mm_srai_epi32(b, 16), _mm_srai_epi32(a, 16)),
      _mm_setzero_si128());
#endif
}

SIMD_INLINE v64 v64_unziplo_16(v64 a, v64 b) {
#if defined(__SSSE3__)
  return _mm_shuffle_epi8(_mm_unpacklo_epi64(b, a),
                          v64_from_64(0x0d0c090805040100LL));
#else
  return v64_unziphi_16(_mm_slli_si128(a, 2), _mm_slli_si128(b, 2));
#endif
}

SIMD_INLINE v64 v64_unpacklo_u8_s16(v64 a) {
  return _mm_unpacklo_epi8(a, _mm_setzero_si128());
}

SIMD_INLINE v64 v64_unpackhi_u8_s16(v64 a) {
  return _mm_srli_si128(_mm_unpacklo_epi8(a, _mm_setzero_si128()), 8);
}

SIMD_INLINE v64 v64_unpacklo_s8_s16(v64 a) {
  return _mm_srai_epi16(_mm_unpacklo_epi8(a, a), 8);
}

SIMD_INLINE v64 v64_unpackhi_s8_s16(v64 a) {
  return _mm_srli_si128(_mm_srai_epi16(_mm_unpacklo_epi8(a, a), 8), 8);
}

SIMD_INLINE v64 v64_unpacklo_u16_s32(v64 a) {
  return _mm_unpacklo_epi16(a, _mm_setzero_si128());
}

SIMD_INLINE v64 v64_unpacklo_s16_s32(v64 a) {
  return _mm_srai_epi32(_mm_unpacklo_epi16(_mm_setzero_si128(), a), 16);
}

SIMD_INLINE v64 v64_unpackhi_u16_s32(v64 a) {
  return _mm_srli_si128(_mm_unpacklo_epi16(a, _mm_setzero_si128()), 8);
}

SIMD_INLINE v64 v64_unpackhi_s16_s32(v64 a) {
  return _mm_srli_si128(
      _mm_srai_epi32(_mm_unpacklo_epi16(_mm_setzero_si128(), a), 16), 8);
}

SIMD_INLINE v64 v64_shuffle_8(v64 x, v64 pattern) {
#if defined(__SSSE3__)
  return _mm_shuffle_epi8(x, pattern);
#else
  v64 output;
  unsigned char *input = (unsigned char *)&x;
  unsigned char *index = (unsigned char *)&pattern;
  unsigned char *selected = (unsigned char *)&output;
  int counter;

  for (counter = 0; counter < 8; counter++) {
    selected[counter] = input[index[counter]];
  }

  return output;
#endif
}

SIMD_INLINE int64_t v64_dotp_su8(v64 a, v64 b) {
  __m128i t = _mm_madd_epi16(_mm_srai_epi16(_mm_unpacklo_epi8(a, a), 8),
                             _mm_unpacklo_epi8(b, _mm_setzero_si128()));
  t = _mm_add_epi32(t, _mm_srli_si128(t, 8));
  t = _mm_add_epi32(t, _mm_srli_si128(t, 4));
  return (int32_t)v64_low_u32(t);
}

SIMD_INLINE int64_t v64_dotp_s16(v64 a, v64 b) {
  __m128i r = _mm_madd_epi16(a, b);
#if defined(__SSE4_1__) && defined(__x86_64__)
  __m128i x = _mm_cvtepi32_epi64(r);
  return _mm_cvtsi128_si64(_mm_add_epi64(x, _mm_srli_si128(x, 8)));
#else
  return (int64_t)_mm_cvtsi128_si32(_mm_srli_si128(r, 4)) +
         (int64_t)_mm_cvtsi128_si32(r);
#endif
}

SIMD_INLINE uint64_t v64_hadd_u8(v64 a) {
  return v64_low_u32(_mm_sad_epu8(a, _mm_setzero_si128()));
}

SIMD_INLINE int64_t v64_hadd_s16(v64 a) {
  return v64_dotp_s16(a, v64_dup_16(1));
}

typedef v64 sad64_internal;

SIMD_INLINE sad64_internal v64_sad_u8_init(void) { return _mm_setzero_si128(); }

/* Implementation dependent return value.  Result must be finalised with
   v64_sad_u8_sum().
   The result for more than 32 v64_sad_u8() calls is undefined. */
SIMD_INLINE sad64_internal v64_sad_u8(sad64_internal s, v64 a, v64 b) {
  return _mm_add_epi64(s, _mm_sad_epu8(a, b));
}

SIMD_INLINE uint32_t v64_sad_u8_sum(sad64_internal s) { return v64_low_u32(s); }

typedef v64 ssd64_internal;

SIMD_INLINE ssd64_internal v64_ssd_u8_init(void) { return _mm_setzero_si128(); }

/* Implementation dependent return value.  Result must be finalised with
 * v64_ssd_u8_sum(). */
SIMD_INLINE ssd64_internal v64_ssd_u8(ssd64_internal s, v64 a, v64 b) {
  v64 l = v64_sub_16(v64_ziplo_8(v64_zero(), a), v64_ziplo_8(v64_zero(), b));
  v64 h = v64_sub_16(v64_ziphi_8(v64_zero(), a), v64_ziphi_8(v64_zero(), b));
  v64 r = v64_add_32(_mm_madd_epi16(l, l), _mm_madd_epi16(h, h));
  return _mm_add_epi64(
      s, v64_ziplo_32(v64_zero(), _mm_add_epi32(r, _mm_srli_si128(r, 4))));
}

SIMD_INLINE uint32_t v64_ssd_u8_sum(sad64_internal s) { return v64_low_u32(s); }

SIMD_INLINE v64 v64_or(v64 a, v64 b) { return _mm_or_si128(a, b); }

SIMD_INLINE v64 v64_xor(v64 a, v64 b) { return _mm_xor_si128(a, b); }

SIMD_INLINE v64 v64_and(v64 a, v64 b) { return _mm_and_si128(a, b); }

SIMD_INLINE v64 v64_andn(v64 a, v64 b) { return _mm_andnot_si128(b, a); }

SIMD_INLINE v64 v64_mullo_s16(v64 a, v64 b) { return _mm_mullo_epi16(a, b); }

SIMD_INLINE v64 v64_mulhi_s16(v64 a, v64 b) { return _mm_mulhi_epi16(a, b); }

SIMD_INLINE v64 v64_mullo_s32(v64 a, v64 b) {
#if defined(__SSE4_1__)
  return _mm_mullo_epi32(a, b);
#else
  return _mm_unpacklo_epi32(
      _mm_mul_epu32(a, b),
      _mm_mul_epu32(_mm_srli_si128(a, 4), _mm_srli_si128(b, 4)));
#endif
}

SIMD_INLINE v64 v64_madd_s16(v64 a, v64 b) { return _mm_madd_epi16(a, b); }

SIMD_INLINE v64 v64_madd_us8(v64 a, v64 b) {
#if defined(__SSSE3__)
  return _mm_maddubs_epi16(a, b);
#else
  __m128i t = _mm_madd_epi16(_mm_unpacklo_epi8(a, _mm_setzero_si128()),
                             _mm_srai_epi16(_mm_unpacklo_epi8(b, b), 8));
  return _mm_packs_epi32(t, t);
#endif
}

SIMD_INLINE v64 v64_avg_u8(v64 a, v64 b) { return _mm_avg_epu8(a, b); }

SIMD_INLINE v64 v64_rdavg_u8(v64 a, v64 b) {
  return _mm_sub_epi8(_mm_avg_epu8(a, b),
                      _mm_and_si128(_mm_xor_si128(a, b), v64_dup_8(1)));
}

SIMD_INLINE v64 v64_rdavg_u16(v64 a, v64 b) {
  return _mm_sub_epi16(_mm_avg_epu16(a, b),
                       _mm_and_si128(_mm_xor_si128(a, b), v64_dup_16(1)));
}

SIMD_INLINE v64 v64_avg_u16(v64 a, v64 b) { return _mm_avg_epu16(a, b); }

SIMD_INLINE v64 v64_min_u8(v64 a, v64 b) { return _mm_min_epu8(a, b); }

SIMD_INLINE v64 v64_max_u8(v64 a, v64 b) { return _mm_max_epu8(a, b); }

SIMD_INLINE v64 v64_min_s8(v64 a, v64 b) {
#if defined(__SSE4_1__)
  return _mm_min_epi8(a, b);
#else
  v64 mask = _mm_cmplt_epi8(a, b);
  return _mm_or_si128(_mm_andnot_si128(mask, b), _mm_and_si128(mask, a));
#endif
}

SIMD_INLINE v64 v64_max_s8(v64 a, v64 b) {
#if defined(__SSE4_1__)
  return _mm_max_epi8(a, b);
#else
  v64 mask = _mm_cmplt_epi8(b, a);
  return _mm_or_si128(_mm_andnot_si128(mask, b), _mm_and_si128(mask, a));
#endif
}

SIMD_INLINE v64 v64_min_s16(v64 a, v64 b) { return _mm_min_epi16(a, b); }

SIMD_INLINE v64 v64_max_s16(v64 a, v64 b) { return _mm_max_epi16(a, b); }

SIMD_INLINE v64 v64_cmpgt_s8(v64 a, v64 b) { return _mm_cmpgt_epi8(a, b); }

SIMD_INLINE v64 v64_cmplt_s8(v64 a, v64 b) { return _mm_cmplt_epi8(a, b); }

SIMD_INLINE v64 v64_cmpeq_8(v64 a, v64 b) { return _mm_cmpeq_epi8(a, b); }

SIMD_INLINE v64 v64_cmpgt_s16(v64 a, v64 b) { return _mm_cmpgt_epi16(a, b); }

SIMD_INLINE v64 v64_cmplt_s16(v64 a, v64 b) { return _mm_cmplt_epi16(a, b); }

SIMD_INLINE v64 v64_cmpeq_16(v64 a, v64 b) { return _mm_cmpeq_epi16(a, b); }

SIMD_INLINE v64 v64_shl_8(v64 a, unsigned int c) {
  return _mm_and_si128(_mm_set1_epi8((char)(0xff << c)),
                       _mm_sll_epi16(a, _mm_cvtsi32_si128((int)c)));
}

SIMD_INLINE v64 v64_shr_u8(v64 a, unsigned int c) {
  return _mm_and_si128(_mm_set1_epi8((char)(0xff >> c)),
                       _mm_srl_epi16(a, _mm_cvtsi32_si128((int)c)));
}

SIMD_INLINE v64 v64_shr_s8(v64 a, unsigned int c) {
  return _mm_packs_epi16(
      _mm_sra_epi16(_mm_unpacklo_epi8(a, a), _mm_cvtsi32_si128((int)(c + 8))),
      a);
}

SIMD_INLINE v64 v64_shl_16(v64 a, unsigned int c) {
  return _mm_sll_epi16(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v64 v64_shr_u16(v64 a, unsigned int c) {
  return _mm_srl_epi16(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v64 v64_shr_s16(v64 a, unsigned int c) {
  return _mm_sra_epi16(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v64 v64_shl_32(v64 a, unsigned int c) {
  return _mm_sll_epi32(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v64 v64_shr_u32(v64 a, unsigned int c) {
  return _mm_srl_epi32(a, _mm_cvtsi32_si128((int)c));
}

SIMD_INLINE v64 v64_shr_s32(v64 a, unsigned int c) {
  return _mm_sra_epi32(a, _mm_cvtsi32_si128((int)c));
}

/* These intrinsics require immediate values, so we must use #defines
   to enforce that. */
#define v64_shl_n_byte(a, c) _mm_slli_si128(a, c)
#define v64_shr_n_byte(a, c) _mm_srli_si128(_mm_unpacklo_epi64(a, a), c + 8)
#define v64_shl_n_8(a, c) \
  _mm_and_si128(_mm_set1_epi8((char)(0xff << (c))), _mm_slli_epi16(a, c))
#define v64_shr_n_u8(a, c) \
  _mm_and_si128(_mm_set1_epi8((char)(0xff >> (c))), _mm_srli_epi16(a, c))
#define v64_shr_n_s8(a, c) \
  _mm_packs_epi16(_mm_srai_epi16(_mm_unpacklo_epi8(a, a), (c) + 8), a)
#define v64_shl_n_16(a, c) _mm_slli_epi16(a, c)
#define v64_shr_n_u16(a, c) _mm_srli_epi16(a, c)
#define v64_shr_n_s16(a, c) _mm_srai_epi16(a, c)
#define v64_shl_n_32(a, c) _mm_slli_epi32(a, c)
#define v64_shr_n_u32(a, c) _mm_srli_epi32(a, c)
#define v64_shr_n_s32(a, c) _mm_srai_epi32(a, c)

#endif  // AOM_AOM_DSP_SIMD_V64_INTRINSICS_X86_H_
