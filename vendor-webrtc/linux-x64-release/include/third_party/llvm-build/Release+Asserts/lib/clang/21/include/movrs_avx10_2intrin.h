/*===--------- movrs_avx10_2intrin.h - AVX10.2-MOVRS intrinsics ------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <movrs_avx10_2intrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef __MOVRS_AVX10_2INTRIN_H
#define __MOVRS_AVX10_2INTRIN_H
#ifdef __x86_64__

/* Define the default attributes for the functions in this file. */
#define __DEFAULT_FN_ATTRS128                                                  \
  __attribute__((__always_inline__, __nodebug__,                               \
                 __target__("movrs,avx10.2-256"), __min_vector_width__(128)))
#define __DEFAULT_FN_ATTRS256                                                  \
  __attribute__((__always_inline__, __nodebug__,                               \
                 __target__("movrs,avx10.2-256"), __min_vector_width__(256)))

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_loadrs_epi8(void const *__A) {
  return (__m128i)__builtin_ia32_vmovrsb128((const __v16qi *)(__A));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_loadrs_epi8(__m128i __W, __mmask16 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectb_128(
      (__mmask16)__U, (__v16qi)_mm_loadrs_epi8(__A), (__v16qi)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_loadrs_epi8(__mmask16 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectb_128((__mmask16)__U,
                                             (__v16qi)_mm_loadrs_epi8(__A),
                                             (__v16qi)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_loadrs_epi8(void const *__A) {
  return (__m256i)__builtin_ia32_vmovrsb256((const __v32qi *)(__A));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_loadrs_epi8(__m256i __W, __mmask32 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectb_256(
      (__mmask32)__U, (__v32qi)_mm256_loadrs_epi8(__A), (__v32qi)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_loadrs_epi8(__mmask32 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectb_256((__mmask32)__U,
                                             (__v32qi)_mm256_loadrs_epi8(__A),
                                             (__v32qi)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_loadrs_epi32(void const *__A) {
  return (__m128i)__builtin_ia32_vmovrsd128((const __v4si *)(__A));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_loadrs_epi32(__m128i __W, __mmask8 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_loadrs_epi32(__A), (__v4si)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_loadrs_epi32(__mmask8 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectd_128((__mmask8)__U,
                                             (__v4si)_mm_loadrs_epi32(__A),
                                             (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_loadrs_epi32(void const *__A) {
  return (__m256i)__builtin_ia32_vmovrsd256((const __v8si *)(__A));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_loadrs_epi32(__m256i __W, __mmask8 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_loadrs_epi32(__A), (__v8si)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_loadrs_epi32(__mmask8 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectd_256((__mmask8)__U,
                                             (__v8si)_mm256_loadrs_epi32(__A),
                                             (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_loadrs_epi64(void const *__A) {
  return (__m128i)__builtin_ia32_vmovrsq128((const __v2di *)(__A));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_loadrs_epi64(__m128i __W, __mmask8 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectq_128(
      (__mmask8)__U, (__v2di)_mm_loadrs_epi64(__A), (__v2di)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_loadrs_epi64(__mmask8 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectq_128((__mmask8)__U,
                                             (__v2di)_mm_loadrs_epi64(__A),
                                             (__v2di)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_loadrs_epi64(void const *__A) {
  return (__m256i)__builtin_ia32_vmovrsq256((const __v4di *)(__A));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_loadrs_epi64(__m256i __W, __mmask8 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectq_256(
      (__mmask8)__U, (__v4di)_mm256_loadrs_epi64(__A), (__v4di)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_loadrs_epi64(__mmask8 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectq_256((__mmask8)__U,
                                             (__v4di)_mm256_loadrs_epi64(__A),
                                             (__v4di)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_loadrs_epi16(void const *__A) {
  return (__m128i)__builtin_ia32_vmovrsw128((const __v8hi *)(__A));
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_loadrs_epi16(__m128i __W, __mmask8 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectw_128(
      (__mmask8)__U, (__v8hi)_mm_loadrs_epi16(__A), (__v8hi)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_loadrs_epi16(__mmask8 __U, void const *__A) {
  return (__m128i)__builtin_ia32_selectw_128((__mmask8)__U,
                                             (__v8hi)_mm_loadrs_epi16(__A),
                                             (__v8hi)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_loadrs_epi16(void const *__A) {
  return (__m256i)__builtin_ia32_vmovrsw256((const __v16hi *)(__A));
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_loadrs_epi16(__m256i __W, __mmask16 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectw_256(
      (__mmask16)__U, (__v16hi)_mm256_loadrs_epi16(__A), (__v16hi)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_loadrs_epi16(__mmask16 __U, void const *__A) {
  return (__m256i)__builtin_ia32_selectw_256((__mmask16)__U,
                                             (__v16hi)_mm256_loadrs_epi16(__A),
                                             (__v16hi)_mm256_setzero_si256());
}

#undef __DEFAULT_FN_ATTRS128
#undef __DEFAULT_FN_ATTRS256

#endif /* __x86_64__ */
#endif /* __MOVRS_AVX10_2INTRIN_H */
