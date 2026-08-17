/*===----------- avx10_2satcvtdsintrin.h - AVX512SATCVTDS intrinsics --------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */

#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2satcvtdsintrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifndef __AVX10_2SATCVTDSINTRIN_H
#define __AVX10_2SATCVTDSINTRIN_H

/* Define the default attributes for the functions in this file. */
#define __DEFAULT_FN_ATTRS256                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-256"),    \
                 __min_vector_width__(256)))

#define __DEFAULT_FN_ATTRS128                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-256"),    \
                 __min_vector_width__(128)))

#define _mm_cvtts_roundsd_i32(__A, __R)                                        \
  ((int)__builtin_ia32_vcvttsd2sis32((__v2df)(__m128)(__A), (const int)(__R)))

#define _mm_cvtts_roundsd_si32(__A, __R)                                       \
  ((int)__builtin_ia32_vcvttsd2sis32((__v2df)(__m128d)(__A), (const int)(__R)))

#define _mm_cvtts_roundsd_u32(__A, __R)                                        \
  ((unsigned int)__builtin_ia32_vcvttsd2usis32((__v2df)(__m128d)(__A),         \
                                               (const int)(__R)))

#define _mm_cvtts_roundss_i32(__A, __R)                                        \
  ((int)__builtin_ia32_vcvttss2sis32((__v4sf)(__m128)(__A), (const int)(__R)))

#define _mm_cvtts_roundss_si32(__A, __R)                                       \
  ((int)__builtin_ia32_vcvttss2sis32((__v4sf)(__m128)(__A), (const int)(__R)))

#define _mm_cvtts_roundss_u32(__A, __R)                                        \
  ((unsigned int)__builtin_ia32_vcvttss2usis32((__v4sf)(__m128)(__A),          \
                                               (const int)(__R)))

#ifdef __x86_64__
#define _mm_cvtts_roundss_u64(__A, __R)                                        \
  ((unsigned long long)__builtin_ia32_vcvttss2usis64((__v4sf)(__m128)(__A),    \
                                                     (const int)(__R)))

#define _mm_cvtts_roundsd_u64(__A, __R)                                        \
  ((unsigned long long)__builtin_ia32_vcvttsd2usis64((__v2df)(__m128d)(__A),   \
                                                     (const int)(__R)))

#define _mm_cvtts_roundss_i64(__A, __R)                                        \
  ((long long)__builtin_ia32_vcvttss2sis64((__v4sf)(__m128)(__A),              \
                                           (const int)(__R)))

#define _mm_cvtts_roundss_si64(__A, __R)                                       \
  ((long long)__builtin_ia32_vcvttss2sis64((__v4sf)(__m128)(__A),              \
                                           (const int)(__R)))

#define _mm_cvtts_roundsd_si64(__A, __R)                                       \
  ((long long)__builtin_ia32_vcvttsd2sis64((__v2df)(__m128d)(__A),             \
                                           (const int)(__R)))

#define _mm_cvtts_roundsd_i64(__A, __R)                                        \
  ((long long)__builtin_ia32_vcvttsd2sis64((__v2df)(__m128d)(__A),             \
                                           (const int)(__R)))
#endif /* __x86_64__ */

// 128 Bit : Double -> int
static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_cvtts_pd_epi32(__m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2dqs128_mask(
      (__v2df)__A, (__v4si)(__m128i)_mm_undefined_si128(), (__mmask8)(-1)));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_pd_epi32(__m128i __W, __mmask8 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2dqs128_mask((__v2df)__A, (__v4si)__W,
                                                      __U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_pd_epi32(__mmask16 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2dqs128_mask(
      (__v2df)__A, (__v4si)(__m128i)_mm_setzero_si128(), __U));
}

// 256 Bit : Double -> int
static __inline__ __m128i __DEFAULT_FN_ATTRS256
_mm256_cvtts_pd_epi32(__m256d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2dqs256_mask(
      (__v4df)__A, (__v4si)_mm_undefined_si128(), (__mmask8)-1));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_pd_epi32(__m128i __W, __mmask8 __U, __m256d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2dqs256_mask((__v4df)__A, (__v4si)__W,
                                                      __U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_pd_epi32(__mmask8 __U, __m256d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2dqs256_mask(
      (__v4df)__A, (__v4si)_mm_setzero_si128(), __U));
}

// 128 Bit : Double -> uint
static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_cvtts_pd_epu32(__m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2udqs128_mask(
      (__v2df)__A, (__v4si)(__m128i)_mm_undefined_si128(), (__mmask8)(-1)));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_pd_epu32(__m128i __W, __mmask8 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2udqs128_mask(
      (__v2df)__A, (__v4si)(__m128i)__W, (__mmask8)__U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_pd_epu32(__mmask8 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2udqs128_mask(
      (__v2df)__A, (__v4si)(__m128i)_mm_setzero_si128(), __U));
}

// 256 Bit : Double -> uint
static __inline__ __m128i __DEFAULT_FN_ATTRS256
_mm256_cvtts_pd_epu32(__m256d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2udqs256_mask(
      (__v4df)__A, (__v4si)_mm_undefined_si128(), (__mmask8)-1));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_pd_epu32(__m128i __W, __mmask8 __U, __m256d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2udqs256_mask((__v4df)__A, (__v4si)__W,
                                                       __U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_pd_epu32(__mmask8 __U, __m256d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2udqs256_mask(
      (__v4df)__A, (__v4si)_mm_setzero_si128(), __U));
}

// 128 Bit : Double -> long
static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_cvtts_pd_epi64(__m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2qqs128_mask(
      (__v2df)__A, (__v2di)_mm_undefined_si128(), (__mmask8)-1));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_pd_epi64(__m128i __W, __mmask8 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2qqs128_mask((__v2df)__A, (__v2di)__W,
                                                      (__mmask8)__U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_pd_epi64(__mmask8 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2qqs128_mask(
      (__v2df)__A, (__v2di)_mm_setzero_si128(), (__mmask8)__U));
}

// 256 Bit : Double -> long
static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_cvtts_pd_epi64(__m256d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2qqs256_mask(
      (__v4df)__A, (__v4di)_mm256_undefined_si256(), (__mmask8)-1));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_pd_epi64(__m256i __W, __mmask8 __U, __m256d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2qqs256_mask((__v4df)__A, (__v4di)__W,
                                                      __U));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_pd_epi64(__mmask8 __U, __m256d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2qqs256_mask(
      (__v4df)__A, (__v4di)_mm256_setzero_si256(), __U));
}

// 128 Bit : Double -> ulong
static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_cvtts_pd_epu64(__m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2uqqs128_mask(
      (__v2df)__A, (__v2di)_mm_undefined_si128(), (__mmask8)-1));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_pd_epu64(__m128i __W, __mmask8 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2uqqs128_mask((__v2df)__A, (__v2di)__W,
                                                       (__mmask8)__U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_pd_epu64(__mmask8 __U, __m128d __A) {
  return ((__m128i)__builtin_ia32_vcvttpd2uqqs128_mask(
      (__v2df)__A, (__v2di)_mm_setzero_si128(), (__mmask8)__U));
}

// 256 Bit : Double -> ulong

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_cvtts_pd_epu64(__m256d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2uqqs256_mask(
      (__v4df)__A, (__v4di)_mm256_undefined_si256(), (__mmask8)-1));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_pd_epu64(__m256i __W, __mmask8 __U, __m256d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2uqqs256_mask((__v4df)__A, (__v4di)__W,
                                                       __U));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_pd_epu64(__mmask8 __U, __m256d __A) {
  return ((__m256i)__builtin_ia32_vcvttpd2uqqs256_mask(
      (__v4df)__A, (__v4di)_mm256_setzero_si256(), __U));
}

// 128 Bit : float -> int
static __inline__ __m128i __DEFAULT_FN_ATTRS128 _mm_cvtts_ps_epi32(__m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2dqs128_mask(
      (__v4sf)__A, (__v4si)(__m128i)_mm_undefined_si128(), (__mmask8)(-1)));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_ps_epi32(__m128i __W, __mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2dqs128_mask((__v4sf)__A, (__v4si)__W,
                                                      (__mmask8)__U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_ps_epi32(__mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2dqs128_mask(
      (__v4sf)__A, (__v4si)(__m128i)_mm_setzero_si128(), (__mmask8)__U));
}

// 256 Bit : float -> int
static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_cvtts_ps_epi32(__m256 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2dqs256_mask(
      (__v8sf)__A, (__v8si)_mm256_undefined_si256(), (__mmask8)-1));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_ps_epi32(__m256i __W, __mmask8 __U, __m256 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2dqs256_mask((__v8sf)__A, (__v8si)__W,
                                                      __U));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_ps_epi32(__mmask8 __U, __m256 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2dqs256_mask(
      (__v8sf)__A, (__v8si)_mm256_setzero_si256(), __U));
}

// 128 Bit : float -> uint
static __inline__ __m128i __DEFAULT_FN_ATTRS128 _mm_cvtts_ps_epu32(__m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2udqs128_mask(
      (__v4sf)__A, (__v4si)(__m128i)_mm_undefined_si128(), (__mmask8)(-1)));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_ps_epu32(__m128i __W, __mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2udqs128_mask((__v4sf)__A, (__v4si)__W,
                                                       (__mmask8)__U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_ps_epu32(__mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2udqs128_mask(
      (__v4sf)__A, (__v4si)_mm_setzero_si128(), (__mmask8)__U));
}

// 256 Bit : float -> uint

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_cvtts_ps_epu32(__m256 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2udqs256_mask(
      (__v8sf)__A, (__v8si)_mm256_undefined_si256(), (__mmask8)-1));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_ps_epu32(__m256i __W, __mmask8 __U, __m256 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2udqs256_mask((__v8sf)__A, (__v8si)__W,
                                                       __U));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_ps_epu32(__mmask8 __U, __m256 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2udqs256_mask(
      (__v8sf)__A, (__v8si)_mm256_setzero_si256(), __U));
}

// 128 bit : float -> long
static __inline__ __m128i __DEFAULT_FN_ATTRS128 _mm_cvtts_ps_epi64(__m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2qqs128_mask(
      (__v4sf)__A, (__v2di)_mm_undefined_si128(), (__mmask8)-1));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_ps_epi64(__m128i __W, __mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2qqs128_mask(
      (__v4sf)__A, (__v2di)(__m128i)__W, (__mmask8)__U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_ps_epi64(__mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2qqs128_mask(
      (__v4sf)__A, (__v2di)_mm_setzero_si128(), (__mmask8)__U));
}
// 256 bit : float -> long

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_cvtts_ps_epi64(__m128 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2qqs256_mask(
      (__v4sf)__A, (__v4di)_mm256_undefined_si256(), (__mmask8)-1));
}
static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_ps_epi64(__m256i __W, __mmask8 __U, __m128 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2qqs256_mask((__v4sf)__A, (__v4di)__W,
                                                      __U));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_ps_epi64(__mmask8 __U, __m128 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2qqs256_mask(
      (__v4sf)__A, (__v4di)_mm256_setzero_si256(), __U));
}

// 128 bit : float -> ulong
static __inline__ __m128i __DEFAULT_FN_ATTRS128 _mm_cvtts_ps_epu64(__m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2uqqs128_mask(
      (__v4sf)__A, (__v2di)_mm_undefined_si128(), (__mmask8)-1));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_cvtts_ps_epu64(__m128i __W, __mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2uqqs128_mask(
      (__v4sf)__A, (__v2di)(__m128i)__W, (__mmask8)__U));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_cvtts_ps_epu64(__mmask8 __U, __m128 __A) {
  return ((__m128i)__builtin_ia32_vcvttps2uqqs128_mask(
      (__v4sf)__A, (__v2di)_mm_setzero_si128(), (__mmask8)__U));
}
// 256 bit : float -> ulong

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_cvtts_ps_epu64(__m128 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2uqqs256_mask(
      (__v4sf)__A, (__v4di)_mm256_undefined_si256(), (__mmask8)-1));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_cvtts_ps_epu64(__m256i __W, __mmask8 __U, __m128 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2uqqs256_mask((__v4sf)__A, (__v4di)__W,
                                                       __U));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_cvtts_ps_epu64(__mmask8 __U, __m128 __A) {
  return ((__m256i)__builtin_ia32_vcvttps2uqqs256_mask(
      (__v4sf)__A, (__v4di)_mm256_setzero_si256(), __U));
}

#undef __DEFAULT_FN_ATTRS128
#undef __DEFAULT_FN_ATTRS256
#endif // __AVX10_2SATCVTDSINTRIN_H
