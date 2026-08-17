/*===----------- avx10_2_512bf16intrin.h - AVX10-BF16 intrinsics ---------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2_512bf16intrin.h> directly; include <immintrin.h> instead."
#endif

#ifdef __SSE2__

#ifndef __AVX10_2_512BF16INTRIN_H
#define __AVX10_2_512BF16INTRIN_H

/* Define the default attributes for the functions in this file. */
typedef __bf16 __m512bh_u __attribute__((__vector_size__(64), __aligned__(1)));

/* Define the default attributes for the functions in this file. */
#define __DEFAULT_FN_ATTRS512                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-512"),    \
                 __min_vector_width__(512)))

static __inline __m512bh __DEFAULT_FN_ATTRS512 _mm512_setzero_pbh(void) {
  return __builtin_bit_cast(__m512bh, _mm512_setzero_ps());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_undefined_pbh(void) {
  return (__m512bh)__builtin_ia32_undef512();
}

static __inline __m512bh __DEFAULT_FN_ATTRS512 _mm512_set1_pbh(__bf16 bf) {
  return (__m512bh)(__v32bf){bf, bf, bf, bf, bf, bf, bf, bf, bf, bf, bf,
                             bf, bf, bf, bf, bf, bf, bf, bf, bf, bf, bf,
                             bf, bf, bf, bf, bf, bf, bf, bf, bf, bf};
}

static __inline __m512bh __DEFAULT_FN_ATTRS512 _mm512_set_pbh(
    __bf16 bf1, __bf16 bf2, __bf16 bf3, __bf16 bf4, __bf16 bf5, __bf16 bf6,
    __bf16 bf7, __bf16 bf8, __bf16 bf9, __bf16 bf10, __bf16 bf11, __bf16 bf12,
    __bf16 bf13, __bf16 bf14, __bf16 bf15, __bf16 bf16, __bf16 bf17,
    __bf16 bf18, __bf16 bf19, __bf16 bf20, __bf16 bf21, __bf16 bf22,
    __bf16 bf23, __bf16 bf24, __bf16 bf25, __bf16 bf26, __bf16 bf27,
    __bf16 bf28, __bf16 bf29, __bf16 bf30, __bf16 bf31, __bf16 bf32) {
  return (__m512bh)(__v32bf){bf32, bf31, bf30, bf29, bf28, bf27, bf26, bf25,
                             bf24, bf23, bf22, bf21, bf20, bf19, bf18, bf17,
                             bf16, bf15, bf14, bf13, bf12, bf11, bf10, bf9,
                             bf8,  bf7,  bf6,  bf5,  bf4,  bf3,  bf2,  bf1};
}

#define _mm512_setr_pbh(bf1, bf2, bf3, bf4, bf5, bf6, bf7, bf8, bf9, bf10,     \
                        bf11, bf12, bf13, bf14, bf15, bf16, bf17, bf18, bf19,  \
                        bf20, bf21, bf22, bf23, bf24, bf25, bf26, bf27, bf28,  \
                        bf29, bf30, bf31, bf32)                                \
  _mm512_set_pbh((bf32), (bf31), (bf30), (bf29), (bf28), (bf27), (bf26),       \
                 (bf25), (bf24), (bf23), (bf22), (bf21), (bf20), (bf19),       \
                 (bf18), (bf17), (bf16), (bf15), (bf14), (bf13), (bf12),       \
                 (bf11), (bf10), (bf9), (bf8), (bf7), (bf6), (bf5), (bf4),     \
                 (bf3), (bf2), (bf1))

static __inline__ __m512 __DEFAULT_FN_ATTRS512
_mm512_castbf16_ps(__m512bh __a) {
  return (__m512)__a;
}

static __inline__ __m512d __DEFAULT_FN_ATTRS512
_mm512_castbf16_pd(__m512bh __a) {
  return (__m512d)__a;
}

static __inline__ __m512i __DEFAULT_FN_ATTRS512
_mm512_castbf16_si512(__m512bh __a) {
  return (__m512i)__a;
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_castps_pbh(__m512 __a) {
  return (__m512bh)__a;
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_castpd_pbh(__m512d __a) {
  return (__m512bh)__a;
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_castsi512_pbh(__m512i __a) {
  return (__m512bh)__a;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS512
_mm512_castbf16512_pbh128(__m512bh __a) {
  return __builtin_shufflevector(__a, __a, 0, 1, 2, 3, 4, 5, 6, 7);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS512
_mm512_castbf16512_pbh256(__m512bh __a) {
  return __builtin_shufflevector(__a, __a, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
                                 12, 13, 14, 15);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_castbf16128_pbh512(__m128bh __a) {
  return __builtin_shufflevector(__a, __a, 0, 1, 2, 3, 4, 5, 6, 7, -1, -1, -1,
                                 -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                                 -1, -1, -1, -1, -1, -1, -1, -1, -1);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_castbf16256_pbh512(__m256bh __a) {
  return __builtin_shufflevector(__a, __a, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
                                 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1,
                                 -1, -1, -1, -1, -1, -1, -1, -1);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_zextbf16128_pbh512(__m128bh __a) {
  return __builtin_shufflevector(
      __a, (__v8bf)_mm_setzero_pbh(), 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
      13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_zextbf16256_pbh512(__m256bh __a) {
  return __builtin_shufflevector(__a, (__v16bf)_mm256_setzero_pbh(), 0, 1, 2, 3,
                                 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
                                 29, 30, 31);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_abs_pbh(__m512bh __A) {
  return (__m512bh)_mm512_and_epi32(_mm512_set1_epi32(0x7FFF7FFF),
                                    (__m512i)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_load_pbh(void const *__p) {
  return *(const __m512bh *)__p;
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_loadu_pbh(void const *__p) {
  struct __loadu_pbh {
    __m512bh_u __v;
  } __attribute__((__packed__, __may_alias__));
  return ((const struct __loadu_pbh *)__p)->__v;
}

static __inline__ void __DEFAULT_FN_ATTRS512 _mm512_store_pbh(void *__P,
                                                              __m512bh __A) {
  *(__m512bh *)__P = __A;
}

static __inline__ void __DEFAULT_FN_ATTRS512 _mm512_storeu_pbh(void *__P,
                                                               __m512bh __A) {
  struct __storeu_pbh {
    __m512bh_u __v;
  } __attribute__((__packed__, __may_alias__));
  ((struct __storeu_pbh *)__P)->__v = __A;
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_blend_pbh(__mmask32 __U, __m512bh __A, __m512bh __W) {
  return (__m512bh)__builtin_ia32_selectpbf_512((__mmask32)__U, (__v32bf)__W,
                                                (__v32bf)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_permutex2var_pbh(__m512bh __A, __m512i __I, __m512bh __B) {
  return (__m512bh)__builtin_ia32_vpermi2varhi512((__v32hi)__A, (__v32hi)__I,
                                                  (__v32hi)__B);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_permutexvar_pbh(__m512i __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_permvarhi512((__v32hi)__B, (__v32hi)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_add_pbh(__m512bh __A,
                                                                __m512bh __B) {
  return (__m512bh)((__v32bf)__A + (__v32bf)__B);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_add_pbh(__m512bh __W, __mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_add_pbh(__A, __B), (__v32bf)__W);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_add_pbh(__mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_add_pbh(__A, __B),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_sub_pbh(__m512bh __A,
                                                                __m512bh __B) {
  return (__m512bh)((__v32bf)__A - (__v32bf)__B);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_sub_pbh(__m512bh __W, __mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_sub_pbh(__A, __B), (__v32bf)__W);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_sub_pbh(__mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_sub_pbh(__A, __B),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mul_pbh(__m512bh __A,
                                                                __m512bh __B) {
  return (__m512bh)((__v32bf)__A * (__v32bf)__B);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_mul_pbh(__m512bh __W, __mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_mul_pbh(__A, __B), (__v32bf)__W);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_mul_pbh(__mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_mul_pbh(__A, __B),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_div_pbh(__m512bh __A,
                                                                __m512bh __B) {
  return (__m512bh)((__v32bf)__A / (__v32bf)__B);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_div_pbh(__m512bh __W, __mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_div_pbh(__A, __B), (__v32bf)__W);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_div_pbh(__mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_div_pbh(__A, __B),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_max_pbh(__m512bh __A,
                                                                __m512bh __B) {
  return (__m512bh)__builtin_ia32_vmaxbf16512((__v32bf)__A, (__v32bf)__B);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_max_pbh(__m512bh __W, __mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_max_pbh(__A, __B), (__v32bf)__W);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_max_pbh(__mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_max_pbh(__A, __B),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_min_pbh(__m512bh __A,
                                                                __m512bh __B) {
  return (__m512bh)__builtin_ia32_vminbf16512((__v32bf)__A, (__v32bf)__B);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_min_pbh(__m512bh __W, __mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_min_pbh(__A, __B), (__v32bf)__W);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_min_pbh(__mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_min_pbh(__A, __B),
      (__v32bf)_mm512_setzero_pbh());
}

#define _mm512_cmp_pbh_mask(__A, __B, __P)                                     \
  ((__mmask32)__builtin_ia32_vcmpbf16512_mask((__v32bf)(__m512bh)(__A),        \
                                              (__v32bf)(__m512bh)(__B),        \
                                              (int)(__P), (__mmask32) - 1))

#define _mm512_mask_cmp_pbh_mask(__U, __A, __B, __P)                           \
  ((__mmask32)__builtin_ia32_vcmpbf16512_mask((__v32bf)(__m512bh)(__A),        \
                                              (__v32bf)(__m512bh)(__B),        \
                                              (int)(__P), (__mmask32)(__U)))

#define _mm512_mask_fpclass_pbh_mask(__U, __A, imm)                            \
  ((__mmask32)__builtin_ia32_vfpclassbf16512_mask(                             \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__mmask32)(__U)))

#define _mm512_fpclass_pbh_mask(__A, imm)                                      \
  ((__mmask32)__builtin_ia32_vfpclassbf16512_mask(                             \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__mmask32) - 1))

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_scalef_pbh(__m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_vscalefbf16512_mask(
      (__v32bf)__A, (__v32bf)__B, (__v32bf)_mm512_undefined_pbh(),
      (__mmask32)-1);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mask_scalef_pbh(
    __m512bh __W, __mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_vscalefbf16512_mask(
      (__v32bf)__A, (__v32bf)__B, (__v32bf)__W, (__mmask32)__U);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_scalef_pbh(__mmask32 __U, __m512bh __A, __m512bh __B) {
  return (__m512bh)__builtin_ia32_vscalefbf16512_mask(
      (__v32bf)__A, (__v32bf)__B, (__v32bf)_mm512_setzero_pbh(),
      (__mmask32)__U);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_rcp_pbh(__m512bh __A) {
  return (__m512bh)__builtin_ia32_vrcpbf16512_mask(
      (__v32bf)__A, (__v32bf)_mm512_undefined_pbh(), (__mmask32)-1);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_rcp_pbh(__m512bh __W, __mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_vrcpbf16512_mask((__v32bf)__A, (__v32bf)__W,
                                                   (__mmask32)__U);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_rcp_pbh(__mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_vrcpbf16512_mask(
      (__v32bf)__A, (__v32bf)_mm512_setzero_pbh(), (__mmask32)__U);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_getexp_pbh(__m512bh __A) {
  return (__m512bh)__builtin_ia32_vgetexpbf16512_mask(
      (__v32bf)__A, (__v32bf)_mm512_undefined_pbh(), (__mmask32)-1);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_getexp_pbh(__m512bh __W, __mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_vgetexpbf16512_mask(
      (__v32bf)__A, (__v32bf)__W, (__mmask32)__U);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_getexp_pbh(__mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_vgetexpbf16512_mask(
      (__v32bf)__A, (__v32bf)_mm512_setzero_pbh(), (__mmask32)__U);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_rsqrt_pbh(__m512bh __A) {
  return (__m512bh)__builtin_ia32_vrsqrtbf16512_mask(
      (__v32bf)__A, (__v32bf)_mm512_undefined_pbh(), (__mmask32)-1);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_rsqrt_pbh(__m512bh __W, __mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_vrsqrtbf16512_mask((__v32bf)__A, (__v32bf)__W,
                                                     (__mmask32)__U);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_rsqrt_pbh(__mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_vrsqrtbf16512_mask(
      (__v32bf)__A, (__v32bf)_mm512_setzero_pbh(), (__mmask32)__U);
}

#define _mm512_reduce_pbh(__A, imm)                                            \
  ((__m512bh)__builtin_ia32_vreducebf16512_mask(                               \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__v32bf)_mm512_undefined_pbh(),   \
      (__mmask32) - 1))

#define _mm512_mask_reduce_pbh(__W, __U, __A, imm)                             \
  ((__m512bh)__builtin_ia32_vreducebf16512_mask(                               \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__v32bf)(__m512bh)(__W),          \
      (__mmask32)(__U)))

#define _mm512_maskz_reduce_pbh(__U, __A, imm)                                 \
  ((__m512bh)__builtin_ia32_vreducebf16512_mask(                               \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__v32bf)_mm512_setzero_pbh(),     \
      (__mmask32)(__U)))

#define _mm512_roundscale_pbh(__A, imm)                                        \
  ((__m512bh)__builtin_ia32_vrndscalebf16_mask(                                \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__v32bf)_mm512_setzero_pbh(),     \
      (__mmask32) - 1))

#define _mm512_mask_roundscale_pbh(__W, __U, __A, imm)                         \
  ((__m512bh)__builtin_ia32_vrndscalebf16_mask(                                \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__v32bf)(__m512bh)(__W),          \
      (__mmask32)(__U)))

#define _mm512_maskz_roundscale_pbh(__U, __A, imm)                             \
  ((__m512bh)__builtin_ia32_vrndscalebf16_mask(                                \
      (__v32bf)(__m512bh)(__A), (int)(imm), (__v32bf)_mm512_setzero_pbh(),     \
      (__mmask32)(__U)))

#define _mm512_getmant_pbh(__A, __B, __C)                                      \
  ((__m512bh)__builtin_ia32_vgetmantbf16512_mask(                              \
      (__v32bf)(__m512bh)(__A), (int)(((__C) << 2) | (__B)),                   \
      (__v32bf)_mm512_undefined_pbh(), (__mmask32) - 1))

#define _mm512_mask_getmant_pbh(__W, __U, __A, __B, __C)                       \
  ((__m512bh)__builtin_ia32_vgetmantbf16512_mask(                              \
      (__v32bf)(__m512bh)(__A), (int)(((__C) << 2) | (__B)),                   \
      (__v32bf)(__m512bh)(__W), (__mmask32)(__U)))

#define _mm512_maskz_getmant_pbh(__U, __A, __B, __C)                           \
  ((__m512bh)__builtin_ia32_vgetmantbf16512_mask(                              \
      (__v32bf)(__m512bh)(__A), (int)(((__C) << 2) | (__B)),                   \
      (__v32bf)_mm512_setzero_pbh(), (__mmask32)(__U)))

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_sqrt_pbh(__m512bh __A) {
  return (__m512bh)__builtin_ia32_vsqrtbf16512((__v32bf)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_sqrt_pbh(__m512bh __W, __mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U, (__v32bf)_mm512_sqrt_pbh(__A), (__v32bf)__W);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_maskz_sqrt_pbh(__mmask32 __U, __m512bh __A) {
  return (__m512bh)__builtin_ia32_selectpbf_512((__mmask32)__U,
                                                (__v32bf)_mm512_sqrt_pbh(__A),
                                                (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_fmadd_pbh(__m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_vfmaddbf16512((__v32bf)__A, (__v32bf)__B,
                                                (__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_fmadd_pbh(__m512bh __A, __mmask32 __U, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fmadd_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C), (__v32bf)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mask3_fmadd_pbh(
    __m512bh __A, __m512bh __B, __m512bh __C, __mmask32 __U) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fmadd_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C), (__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_maskz_fmadd_pbh(
    __mmask32 __U, __m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fmadd_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_fmsub_pbh(__m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_vfmaddbf16512((__v32bf)__A, (__v32bf)__B,
                                                -(__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_mask_fmsub_pbh(__m512bh __A, __mmask32 __U, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fmsub_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C), (__v32bf)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mask3_fmsub_pbh(
    __m512bh __A, __m512bh __B, __m512bh __C, __mmask32 __U) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fmsub_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C), (__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_maskz_fmsub_pbh(
    __mmask32 __U, __m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fmsub_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_fnmadd_pbh(__m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_vfmaddbf16512((__v32bf)__A, -(__v32bf)__B,
                                                (__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mask_fnmadd_pbh(
    __m512bh __A, __mmask32 __U, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fnmadd_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mask3_fnmadd_pbh(
    __m512bh __A, __m512bh __B, __m512bh __C, __mmask32 __U) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fnmadd_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_maskz_fnmadd_pbh(
    __mmask32 __U, __m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fnmadd_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)_mm512_setzero_pbh());
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512
_mm512_fnmsub_pbh(__m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_vfmaddbf16512((__v32bf)__A, -(__v32bf)__B,
                                                -(__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mask_fnmsub_pbh(
    __m512bh __A, __mmask32 __U, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fnmsub_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)__A);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_mask3_fnmsub_pbh(
    __m512bh __A, __m512bh __B, __m512bh __C, __mmask32 __U) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fnmsub_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)__C);
}

static __inline__ __m512bh __DEFAULT_FN_ATTRS512 _mm512_maskz_fnmsub_pbh(
    __mmask32 __U, __m512bh __A, __m512bh __B, __m512bh __C) {
  return (__m512bh)__builtin_ia32_selectpbf_512(
      (__mmask32)__U,
      _mm512_fnmsub_pbh((__v32bf)__A, (__v32bf)__B, (__v32bf)__C),
      (__v32bf)_mm512_setzero_pbh());
}

#undef __DEFAULT_FN_ATTRS512

#endif
#endif
