/*===----- avx10_2_512satcvtdsintrin.h - AVX10_2_512SATCVTDS intrinsics ----===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2_512satcvtdsintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef __AVX10_2_512SATCVTDSINTRIN_H
#define __AVX10_2_512SATCVTDSINTRIN_H

/* Define the default attributes for the functions in this file. */
#define __DEFAULT_FN_ATTRS                                                     \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-512"),    \
                 __min_vector_width__(512)))

// 512 bit : Double -> Int
static __inline__ __m256i __DEFAULT_FN_ATTRS
_mm512_cvtts_pd_epi32(__m512d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2dqs512_round_mask(
      (__v8df)__A, (__v8si)_mm256_undefined_si256(), (__mmask8)-1,
      _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_pd_epi32(__m256i __W, __mmask8 __U, __m512d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2dqs512_round_mask(
      (__v8df)__A, (__v8si)__W, __U, _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_pd_epi32(__mmask8 __U, __m512d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2dqs512_round_mask(
      (__v8df)__A, (__v8si)_mm256_setzero_si256(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundpd_epi32(__A, __R)                                   \
  ((__m256i)__builtin_ia32_vcvttpd2dqs512_round_mask(                          \
      (__v8df)(__m512d)(__A), (__v8si)_mm256_undefined_si256(),                \
      (__mmask8) - 1, (const int)(__R)))

#define _mm512_mask_cvtts_roundpd_epi32(__W, __U, __A, __R)                    \
  ((__m256i)__builtin_ia32_vcvttpd2dqs512_round_mask(                          \
      (__v8df)(__m512d)(__A), (__v8si)(__m256i)(__W), (__mmask8)(__U),         \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundpd_epi32(__U, __A, __R)                        \
  ((__m256i)__builtin_ia32_vcvttpd2dqs512_round_mask(                          \
      (__v8df)(__m512d)(__A), (__v8si)_mm256_setzero_si256(), (__mmask8)(__U), \
      (const int)(__R)))

// 512 bit : Double -> uInt
static __inline__ __m256i __DEFAULT_FN_ATTRS
_mm512_cvtts_pd_epu32(__m512d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2udqs512_round_mask(
      (__v8df)__A, (__v8si)_mm256_undefined_si256(), (__mmask8)-1,
      _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_pd_epu32(__m256i __W, __mmask8 __U, __m512d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2udqs512_round_mask(
      (__v8df)__A, (__v8si)__W, __U, _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_pd_epu32(__mmask8 __U, __m512d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2udqs512_round_mask(
      (__v8df)__A, (__v8si)_mm256_setzero_si256(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundpd_epu32(__A, __R)                                   \
  ((__m256i)__builtin_ia32_vcvttpd2udqs512_round_mask(                         \
      (__v8df)(__m512d)(__A), (__v8si)_mm256_undefined_si256(),                \
      (__mmask8) - 1, (const int)(__R)))

#define _mm512_mask_cvtts_roundpd_epu32(__W, __U, __A, __R)                    \
  ((__m256i)__builtin_ia32_vcvttpd2udqs512_round_mask(                         \
      (__v8df)(__m512d)(__A), (__v8si)(__m256i)(__W), (__mmask8)(__U),         \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundpd_epu32(__U, __A, __R)                        \
  ((__m256i)__builtin_ia32_vcvttpd2udqs512_round_mask(                         \
      (__v8df)(__m512d)(__A), (__v8si)_mm256_setzero_si256(), (__mmask8)(__U), \
      (const int)(__R)))

//  512 bit : Double -> Long

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_cvtts_pd_epi64(__m512d __A) {
  return ((__m512i)__builtin_ia32_vcvttpd2qqs512_round_mask(
      (__v8df)__A, (__v8di)_mm512_undefined_epi32(), (__mmask8)-1,
      _MM_FROUND_CUR_DIRECTION));
}
static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_pd_epi64(__m512i __W, __mmask8 __U, __m512d __A) {
  return ((__m512i)__builtin_ia32_vcvttpd2qqs512_round_mask(
      (__v8df)__A, (__v8di)__W, __U, _MM_FROUND_CUR_DIRECTION));
}
static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_pd_epi64(__mmask8 __U, __m512d __A) {
  return ((__m512i)__builtin_ia32_vcvttpd2qqs512_round_mask(
      (__v8df)__A, (__v8di)_mm512_setzero_si512(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundpd_epi64(__A, __R)                                   \
  ((__m512i)__builtin_ia32_vcvttpd2qqs512_round_mask(                          \
      (__v8df)(__m512d)(__A), (__v8di)_mm512_undefined_epi32(),                \
      (__mmask8) - 1, (const int)(__R)))

#define _mm512_mask_cvtts_roundpd_epi64(__W, __U, __A, __R)                    \
  ((__m512i)__builtin_ia32_vcvttpd2qqs512_round_mask(                          \
      (__v8df)(__m512d)(__A), (__v8di)(__m512i)(__W), (__mmask8)(__U),         \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundpd_epi64(__U, __A, __R)                        \
  ((__m512i)__builtin_ia32_vcvttpd2qqs512_round_mask(                          \
      (__v8df)(__m512d)(__A), (__v8di)_mm512_setzero_si512(), (__mmask8)(__U), \
      (const int)(__R)))

// 512 bit : Double -> ULong

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_cvtts_pd_epu64(__m512d __A) {
  return ((__m512i)__builtin_ia32_vcvttpd2uqqs512_round_mask(
      (__v8df)__A, (__v8di)_mm512_undefined_epi32(), (__mmask8)-1,
      _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_pd_epu64(__m512i __W, __mmask8 __U, __m512d __A) {
  return ((__m512i)__builtin_ia32_vcvttpd2uqqs512_round_mask(
      (__v8df)__A, (__v8di)__W, __U, _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_pd_epu64(__mmask8 __U, __m512d __A) {
  return ((__m512i)__builtin_ia32_vcvttpd2uqqs512_round_mask(
      (__v8df)__A, (__v8di)_mm512_setzero_si512(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundpd_epu64(__A, __R)                                   \
  ((__m512i)__builtin_ia32_vcvttpd2uqqs512_round_mask(                         \
      (__v8df)(__m512d)(__A), (__v8di)_mm512_undefined_epi32(),                \
      (__mmask8) - 1, (const int)(__R)))

#define _mm512_mask_cvtts_roundpd_epu64(__W, __U, __A, __R)                    \
  ((__m512i)__builtin_ia32_vcvttpd2uqqs512_round_mask(                         \
      (__v8df)(__m512d)(__A), (__v8di)(__m512i)(__W), (__mmask8)(__U),         \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundpd_epu64(__U, __A, __R)                        \
  ((__m512i)__builtin_ia32_vcvttpd2uqqs512_round_mask(                         \
      (__v8df)(__m512d)(__A), (__v8di)_mm512_setzero_si512(), (__mmask8)(__U), \
      (const int)(__R)))

// 512 bit: Float -> int
static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_cvtts_ps_epi32(__m512 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2dqs512_round_mask(
      (__v16sf)(__A), (__v16si)_mm512_undefined_epi32(), (__mmask16)-1,
      _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_ps_epi32(__m512i __W, __mmask16 __U, __m512 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2dqs512_round_mask(
      (__v16sf)(__A), (__v16si)(__W), __U, _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_ps_epi32(__mmask16 __U, __m512 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2dqs512_round_mask(
      (__v16sf)(__A), (__v16si)_mm512_setzero_si512(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundps_epi32(__A, __R)                                   \
  ((__m512i)__builtin_ia32_vcvttps2dqs512_round_mask(                          \
      (__v16sf)(__m512)(__A), (__v16si)_mm512_undefined_epi32(),               \
      (__mmask16) - 1, (const int)(__R)))

#define _mm512_mask_cvtts_roundps_epi32(__W, __U, __A, __R)                    \
  ((__m512i)__builtin_ia32_vcvttps2dqs512_round_mask(                          \
      (__v16sf)(__m512)(__A), (__v16si)(__m512i)(__W), (__mmask16)(__U),       \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundps_epi32(__U, __A, __R)                        \
  ((__m512i)__builtin_ia32_vcvttps2dqs512_round_mask(                          \
      (__v16sf)(__m512)(__A), (__v16si)_mm512_setzero_si512(),                 \
      (__mmask16)(__U), (const int)(__R)))

// 512 bit: Float -> uint
static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_cvtts_ps_epu32(__m512 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2udqs512_round_mask(
      (__v16sf)(__A), (__v16si)_mm512_undefined_epi32(), (__mmask16)-1,
      _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_ps_epu32(__m512i __W, __mmask16 __U, __m512 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2udqs512_round_mask(
      (__v16sf)(__A), (__v16si)(__W), __U, _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_ps_epu32(__mmask16 __U, __m512 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2udqs512_round_mask(
      (__v16sf)(__A), (__v16si)_mm512_setzero_si512(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundps_epu32(__A, __R)                                   \
  ((__m512i)__builtin_ia32_vcvttps2udqs512_round_mask(                         \
      (__v16sf)(__m512)(__A), (__v16si)_mm512_undefined_epi32(),               \
      (__mmask16) - 1, (const int)(__R)))

#define _mm512_mask_cvtts_roundps_epu32(__W, __U, __A, __R)                    \
  ((__m512i)__builtin_ia32_vcvttps2udqs512_round_mask(                         \
      (__v16sf)(__m512)(__A), (__v16si)(__m512i)(__W), (__mmask16)(__U),       \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundps_epu32(__U, __A, __R)                        \
  ((__m512i)__builtin_ia32_vcvttps2udqs512_round_mask(                         \
      (__v16sf)(__m512)(__A), (__v16si)_mm512_setzero_si512(),                 \
      (__mmask16)(__U), (const int)(__R)))

// 512 bit : float -> long
static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_cvtts_ps_epi64(__m256 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2qqs512_round_mask(
      (__v8sf)__A, (__v8di)_mm512_undefined_epi32(), (__mmask8)-1,
      _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_ps_epi64(__m512i __W, __mmask8 __U, __m256 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2qqs512_round_mask(
      (__v8sf)__A, (__v8di)__W, __U, _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_ps_epi64(__mmask8 __U, __m256 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2qqs512_round_mask(
      (__v8sf)__A, (__v8di)_mm512_setzero_si512(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundps_epi64(__A, __R)                                   \
  ((__m512i)__builtin_ia32_vcvttps2qqs512_round_mask(                          \
      (__v8sf)(__m256)(__A), (__v8di)_mm512_undefined_epi32(), (__mmask8) - 1, \
      (const int)(__R)))

#define _mm512_mask_cvtts_roundps_epi64(__W, __U, __A, __R)                    \
  ((__m512i)__builtin_ia32_vcvttps2qqs512_round_mask(                          \
      (__v8sf)(__m256)(__A), (__v8di)(__m512i)(__W), (__mmask8)(__U),          \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundps_epi64(__U, __A, __R)                        \
  ((__m512i)__builtin_ia32_vcvttps2qqs512_round_mask(                          \
      (__v8sf)(__m256)(__A), (__v8di)_mm512_setzero_si512(), (__mmask8)(__U),  \
      (const int)(__R)))

// 512 bit : float -> ulong
static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_cvtts_ps_epu64(__m256 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2uqqs512_round_mask(
      (__v8sf)__A, (__v8di)_mm512_undefined_epi32(), (__mmask8)-1,
      _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_cvtts_ps_epu64(__m512i __W, __mmask8 __U, __m256 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2uqqs512_round_mask(
      (__v8sf)__A, (__v8di)__W, __U, _MM_FROUND_CUR_DIRECTION));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_maskz_cvtts_ps_epu64(__mmask8 __U, __m256 __A) {
  return ((__m512i)__builtin_ia32_vcvttps2uqqs512_round_mask(
      (__v8sf)__A, (__v8di)_mm512_setzero_si512(), __U,
      _MM_FROUND_CUR_DIRECTION));
}

#define _mm512_cvtts_roundps_epu64(__A, __R)                                   \
  ((__m512i)__builtin_ia32_vcvttps2uqqs512_round_mask(                         \
      (__v8sf)(__m256)(__A), (__v8di)_mm512_undefined_epi32(), (__mmask8) - 1, \
      (const int)(__R)))

#define _mm512_mask_cvtts_roundps_epu64(__W, __U, __A, __R)                    \
  ((__m512i)__builtin_ia32_vcvttps2uqqs512_round_mask(                         \
      (__v8sf)(__m256)(__A), (__v8di)(__m512i)(__W), (__mmask8)(__U),          \
      (const int)(__R)))

#define _mm512_maskz_cvtts_roundps_epu64(__U, __A, __R)                        \
  ((__m512i)__builtin_ia32_vcvttps2uqqs512_round_mask(                         \
      (__v8sf)(__m256)(__A), (__v8di)_mm512_setzero_si512(), (__mmask8)(__U),  \
      (const int)(__R)))

#undef __DEFAULT_FN_ATTRS
#endif // __AVX10_2_512SATCVTDSINTRIN_H
