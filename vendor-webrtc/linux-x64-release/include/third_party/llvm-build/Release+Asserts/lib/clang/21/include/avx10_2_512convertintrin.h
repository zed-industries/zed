/*===--------- avx10_2_512convertintrin.h - AVX10_2_512CONVERT -------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2_512convertintrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifdef __SSE2__

#ifndef __AVX10_2_512CONVERTINTRIN_H
#define __AVX10_2_512CONVERTINTRIN_H

/* Define the default attributes for the functions in this file. */
#define __DEFAULT_FN_ATTRS512                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-512"),    \
                 __min_vector_width__(512)))

static __inline__ __m512h __DEFAULT_FN_ATTRS512 _mm512_cvtx2ps_ph(__m512 __A,
                                                                  __m512 __B) {
  return (__m512h)__builtin_ia32_vcvt2ps2phx512_mask(
      (__v16sf)__A, (__v16sf)__B, (__v32hf)_mm512_setzero_ph(), (__mmask32)(-1),
      _MM_FROUND_CUR_DIRECTION);
}

static __inline__ __m512h __DEFAULT_FN_ATTRS512
_mm512_mask_cvtx2ps_ph(__m512h __W, __mmask32 __U, __m512 __A, __m512 __B) {
  return (__m512h)__builtin_ia32_vcvt2ps2phx512_mask(
      (__v16sf)__A, (__v16sf)__B, (__v32hf)__W, (__mmask32)__U,
      _MM_FROUND_CUR_DIRECTION);
}

static __inline__ __m512h __DEFAULT_FN_ATTRS512
_mm512_maskz_cvtx2ps_ph(__mmask32 __U, __m512 __A, __m512 __B) {
  return (__m512h)__builtin_ia32_vcvt2ps2phx512_mask(
      (__v16sf)__A, (__v16sf)__B, (__v32hf)_mm512_setzero_ph(), (__mmask32)__U,
      _MM_FROUND_CUR_DIRECTION);
}

#define _mm512_cvtx_round2ps_ph(A, B, R)                                       \
  ((__m512h)__builtin_ia32_vcvt2ps2phx512_mask(                                \
      (__v16sf)(A), (__v16sf)(B), (__v32hf)_mm512_undefined_ph(),              \
      (__mmask32)(-1), (const int)(R)))

#define _mm512_mask_cvtx_round2ps_ph(W, U, A, B, R)                            \
  ((__m512h)__builtin_ia32_vcvt2ps2phx512_mask((__v16sf)(A), (__v16sf)(B),     \
                                               (__v32hf)(W), (__mmask32)(U),   \
                                               (const int)(R)))

#define _mm512_maskz_cvtx_round2ps_ph(U, A, B, R)                              \
  ((__m512h)__builtin_ia32_vcvt2ps2phx512_mask(                                \
      (__v16sf)(A), (__v16sf)(B), (__v32hf)_mm512_setzero_ph(),                \
      (__mmask32)(U), (const int)(R)))

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_cvtbiasph_bf8(__m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2bf8_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)_mm256_undefined_si256(),
      (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512 _mm512_mask_cvtbiasph_bf8(
    __m256i __W, __mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2bf8_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvtbiasph_bf8(__mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2bf8_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)_mm256_setzero_si256(),
      (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_cvts_biasph_bf8(__m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2bf8s_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)_mm256_undefined_si256(),
      (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512 _mm512_mask_cvts_biasph_bf8(
    __m256i __W, __mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2bf8s_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvts_biasph_bf8(__mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2bf8s_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)_mm256_setzero_si256(),
      (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_cvtbiasph_hf8(__m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2hf8_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)_mm256_undefined_si256(),
      (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512 _mm512_mask_cvtbiasph_hf8(
    __m256i __W, __mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2hf8_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvtbiasph_hf8(__mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2hf8_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)_mm256_setzero_si256(),
      (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_cvts_biasph_hf8(__m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2hf8s_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)_mm256_undefined_si256(),
      (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512 _mm512_mask_cvts_biasph_hf8(
    __m256i __W, __mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2hf8s_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvts_biasph_hf8(__mmask32 __U, __m512i __A, __m512h __B) {
  return (__m256i)__builtin_ia32_vcvtbiasph2hf8s_512_mask(
      (__v64qi)__A, (__v32hf)__B, (__v32qi)(__m256i)_mm256_setzero_si256(),
      (__mmask32)__U);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512 _mm512_cvt2ph_bf8(__m512h __A,
                                                                  __m512h __B) {
  return (__m512i)__builtin_ia32_vcvt2ph2bf8_512((__v32hf)(__A),
                                                 (__v32hf)(__B));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_mask_cvt2ph_bf8(__m512i __W, __mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvt2ph_bf8(__A, __B), (__v64qi)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvt2ph_bf8(__mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvt2ph_bf8(__A, __B),
      (__v64qi)(__m512i)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_cvts_2ph_bf8(__m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_vcvt2ph2bf8s_512((__v32hf)(__A),
                                                  (__v32hf)(__B));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_mask_cvts_2ph_bf8(__m512i __W, __mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvts_2ph_bf8(__A, __B), (__v64qi)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvts_2ph_bf8(__mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvts_2ph_bf8(__A, __B),
      (__v64qi)(__m512i)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512 _mm512_cvt2ph_hf8(__m512h __A,
                                                                  __m512h __B) {
  return (__m512i)__builtin_ia32_vcvt2ph2hf8_512((__v32hf)(__A),
                                                 (__v32hf)(__B));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_mask_cvt2ph_hf8(__m512i __W, __mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvt2ph_hf8(__A, __B), (__v64qi)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvt2ph_hf8(__mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvt2ph_hf8(__A, __B),
      (__v64qi)(__m512i)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_cvts_2ph_hf8(__m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_vcvt2ph2hf8s_512((__v32hf)(__A),
                                                  (__v32hf)(__B));
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_mask_cvts_2ph_hf8(__m512i __W, __mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvts_2ph_hf8(__A, __B), (__v64qi)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvts_2ph_hf8(__mmask64 __U, __m512h __A, __m512h __B) {
  return (__m512i)__builtin_ia32_selectb_512(
      (__mmask64)__U, (__v64qi)_mm512_cvts_2ph_hf8(__A, __B),
      (__v64qi)(__m512i)_mm512_setzero_si512());
}

static __inline__ __m512h __DEFAULT_FN_ATTRS512 _mm512_cvthf8_ph(__m256i __A) {
  return (__m512h)__builtin_ia32_vcvthf8_2ph512_mask(
      (__v32qi)__A, (__v32hf)(__m512h)_mm512_undefined_ph(), (__mmask32)-1);
}

static __inline__ __m512h __DEFAULT_FN_ATTRS512
_mm512_mask_cvthf8_ph(__m512h __W, __mmask32 __U, __m256i __A) {
  return (__m512h)__builtin_ia32_vcvthf8_2ph512_mask(
      (__v32qi)__A, (__v32hf)(__m512h)__W, (__mmask32)__U);
}

static __inline__ __m512h __DEFAULT_FN_ATTRS512
_mm512_maskz_cvthf8_ph(__mmask32 __U, __m256i __A) {
  return (__m512h)__builtin_ia32_vcvthf8_2ph512_mask(
      (__v32qi)__A, (__v32hf)(__m512h)_mm512_setzero_ph(), (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512 _mm512_cvtph_bf8(__m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2bf8_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_undefined_si256(), (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_mask_cvtph_bf8(__m256i __W, __mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2bf8_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvtph_bf8(__mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2bf8_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_setzero_si256(), (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_cvts_ph_bf8(__m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2bf8s_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_undefined_si256(), (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_mask_cvts_ph_bf8(__m256i __W, __mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2bf8s_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvts_ph_bf8(__mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2bf8s_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_setzero_si256(), (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512 _mm512_cvtph_hf8(__m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2hf8_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_undefined_si256(), (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_mask_cvtph_hf8(__m256i __W, __mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2hf8_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvtph_hf8(__mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2hf8_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_setzero_si256(), (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_cvts_ph_hf8(__m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2hf8s_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_undefined_si256(), (__mmask32)-1);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_mask_cvts_ph_hf8(__m256i __W, __mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2hf8s_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)__W, (__mmask32)__U);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS512
_mm512_maskz_cvts_ph_hf8(__mmask32 __U, __m512h __A) {
  return (__m256i)__builtin_ia32_vcvtph2hf8s_512_mask(
      (__v32hf)__A, (__v32qi)(__m256i)_mm256_setzero_si256(), (__mmask32)__U);
}

static __inline __m512h __DEFAULT_FN_ATTRS512 _mm512_cvtbf8_ph(__m256i __A) {
  return _mm512_castsi512_ph(_mm512_slli_epi16(_mm512_cvtepi8_epi16(__A), 8));
}

static __inline __m512h __DEFAULT_FN_ATTRS512
_mm512_mask_cvtbf8_ph(__m512h __S, __mmask32 __U, __m256i __A) {
  return _mm512_castsi512_ph(
      _mm512_mask_slli_epi16((__m512i)__S, __U, _mm512_cvtepi8_epi16(__A), 8));
}

static __inline __m512h __DEFAULT_FN_ATTRS512
_mm512_maskz_cvtbf8_ph(__mmask32 __U, __m256i __A) {
  return _mm512_castsi512_ph(
      _mm512_slli_epi16(_mm512_maskz_cvtepi8_epi16(__U, __A), 8));
}

#undef __DEFAULT_FN_ATTRS512

#endif // __AVX10_2_512CONVERTINTRIN_H
#endif // __SSE2__
