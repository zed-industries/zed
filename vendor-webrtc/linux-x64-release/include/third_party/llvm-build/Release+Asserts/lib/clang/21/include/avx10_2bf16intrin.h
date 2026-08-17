/*===-------------- avx10_2bf16intrin.h - AVX10-BF16 intrinsics ------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2bf16intrin.h> directly; include <immintrin.h> instead."
#endif

#ifdef __SSE2__

#ifndef __AVX10_2BF16INTRIN_H
#define __AVX10_2BF16INTRIN_H

typedef __bf16 __m128bh_u __attribute__((__vector_size__(16), __aligned__(1)));
typedef __bf16 __m256bh_u __attribute__((__vector_size__(32), __aligned__(1)));

/* Define the default attributes for the functions in this file. */
#define __DEFAULT_FN_ATTRS256                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-256"),    \
                 __min_vector_width__(256)))
#define __DEFAULT_FN_ATTRS128                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-256"),    \
                 __min_vector_width__(128)))

static __inline __m256bh __DEFAULT_FN_ATTRS256 _mm256_setzero_pbh(void) {
  return __builtin_bit_cast(__m256bh, _mm256_setzero_ps());
}

static __inline __m128bh __DEFAULT_FN_ATTRS128 _mm_setzero_pbh(void) {
  return __builtin_bit_cast(__m128bh, _mm_setzero_ps());
}

static __inline__ __m128 __DEFAULT_FN_ATTRS128 _mm_castbf16_ps(__m128bh __a) {
  return (__m128)__a;
}

static __inline__ __m256 __DEFAULT_FN_ATTRS256
_mm256_castbf16_ps(__m256bh __a) {
  return (__m256)__a;
}

static __inline__ __m256d __DEFAULT_FN_ATTRS256
_mm256_castbf16_pd(__m256bh __a) {
  return (__m256d)__a;
}

static __inline__ __m128d __DEFAULT_FN_ATTRS128 _mm_castbf16_pd(__m128bh __a) {
  return (__m128d)__a;
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_castbf16_si128(__m128bh __a) {
  return (__m128i)__a;
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_castbf16_si256(__m256bh __a) {
  return (__m256i)__a;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_castps_pbh(__m128 __a) {
  return (__m128bh)__a;
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_castps_pbh(__m256 __a) {
  return (__m256bh)__a;
}

static __inline__ __bf16 __DEFAULT_FN_ATTRS128 _mm_cvtsbh_bf16(__m128bh __a) {
  return __a[0];
}

static __inline__ __bf16 __DEFAULT_FN_ATTRS256
_mm256_cvtsbh_bf16(__m256bh __a) {
  return __a[0];
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_castpd_pbh(__m128d __a) {
  return (__m128bh)__a;
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_castpd_pbh(__m256d __a) {
  return (__m256bh)__a;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_castsi128_pbh(__m128i __a) {
  return (__m128bh)__a;
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_castsi256_pbh(__m256i __a) {
  return (__m256bh)__a;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS256
_mm256_castbf16256_pbh128(__m256bh __a) {
  return __builtin_shufflevector(__a, __a, 0, 1, 2, 3, 4, 5, 6, 7);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_castbf16128_pbh256(__m128bh __a) {
  return __builtin_shufflevector(__a, __a, 0, 1, 2, 3, 4, 5, 6, 7, -1, -1, -1,
                                 -1, -1, -1, -1, -1);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_zextbf16128_pbh256(__m128bh __a) {
  return __builtin_shufflevector(__a, (__v8bf)_mm_setzero_pbh(), 0, 1, 2, 3, 4,
                                 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_undefined_pbh(void) {
  return (__m256bh)__builtin_ia32_undef256();
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_load_sbh(void const *__dp) {
  __m128bh src = (__v8bf)_mm_setzero_pbh();
  return (__m128bh)__builtin_ia32_loadsbf16128_mask((const __v8bf *)__dp, src,
                                                    1);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_load_sbh(__m128bh __W, __mmask8 __U, const void *__A) {
  __m128bh src = (__v8bf)__builtin_shufflevector(
      (__v8bf)__W, (__v8bf)_mm_setzero_pbh(), 0, 8, 8, 8, 8, 8, 8, 8);

  return (__m128bh)__builtin_ia32_loadsbf16128_mask((const __v8bf *)__A, src,
                                                    __U & 1);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_load_sbh(__mmask8 __U, const void *__A) {
  return (__m128bh)__builtin_ia32_loadsbf16128_mask(
      (const __v8bf *)__A, (__v8bf)_mm_setzero_pbh(), __U & 1);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_load_pbh(void const *__p) {
  return *(const __m256bh *)__p;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_load_pbh(void const *__p) {
  return *(const __m128bh *)__p;
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_loadu_pbh(void const *__p) {
  struct __loadu_pbh {
    __m256bh_u __v;
  } __attribute__((__packed__, __may_alias__));
  return ((const struct __loadu_pbh *)__p)->__v;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_loadu_pbh(void const *__p) {
  struct __loadu_pbh {
    __m128bh_u __v;
  } __attribute__((__packed__, __may_alias__));
  return ((const struct __loadu_pbh *)__p)->__v;
}

static __inline__ void __DEFAULT_FN_ATTRS128 _mm_store_sbh(void *__dp,
                                                           __m128bh __a) {
  struct __mm_store_sbh_struct {
    __bf16 __u;
  } __attribute__((__packed__, __may_alias__));
  ((struct __mm_store_sbh_struct *)__dp)->__u = __a[0];
}

static __inline__ void __DEFAULT_FN_ATTRS128 _mm_mask_store_sbh(void *__W,
                                                                __mmask8 __U,
                                                                __m128bh __A) {
  __builtin_ia32_storesbf16128_mask((__v8bf *)__W, __A, __U & 1);
}

static __inline__ void __DEFAULT_FN_ATTRS256 _mm256_store_pbh(void *__P,
                                                              __m256bh __A) {
  *(__m256bh *)__P = __A;
}

static __inline__ void __DEFAULT_FN_ATTRS128 _mm_store_pbh(void *__P,
                                                           __m128bh __A) {
  *(__m128bh *)__P = __A;
}

static __inline__ void __DEFAULT_FN_ATTRS256 _mm256_storeu_pbh(void *__P,
                                                               __m256bh __A) {
  struct __storeu_pbh {
    __m256bh_u __v;
  } __attribute__((__packed__, __may_alias__));
  ((struct __storeu_pbh *)__P)->__v = __A;
}

static __inline__ void __DEFAULT_FN_ATTRS128 _mm_storeu_pbh(void *__P,
                                                            __m128bh __A) {
  struct __storeu_pbh {
    __m128bh_u __v;
  } __attribute__((__packed__, __may_alias__));
  ((struct __storeu_pbh *)__P)->__v = __A;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_move_sbh(__m128bh __a,
                                                              __m128bh __b) {
  __a[0] = __b[0];
  return __a;
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_move_sbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return __builtin_ia32_selectsbf_128(__U, _mm_move_sbh(__A, __B), __W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_move_sbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return __builtin_ia32_selectsbf_128(__U, _mm_move_sbh(__A, __B),
                                      _mm_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_undefined_pbh(void) {
  return (__m128bh)__builtin_ia32_undef128();
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_set_sbh(__bf16 bf) {
  return (__v8bf)__builtin_shufflevector(
      (__v8bf){bf, bf, bf, bf, bf, bf, bf, bf}, (__v8bf)_mm_setzero_pbh(), 0, 8,
      8, 8, 8, 8, 8, 8);
}

static __inline __m128bh __DEFAULT_FN_ATTRS128 _mm_set1_pbh(__bf16 bf) {
  return (__m128bh)(__v8bf){bf, bf, bf, bf, bf, bf, bf, bf};
}

static __inline __m256bh __DEFAULT_FN_ATTRS256 _mm256_set1_pbh(__bf16 bf) {
  return (__m256bh)(__v16bf){bf, bf, bf, bf, bf, bf, bf, bf,
                             bf, bf, bf, bf, bf, bf, bf, bf};
}

static __inline __m128bh __DEFAULT_FN_ATTRS128
_mm_set_pbh(__bf16 bf1, __bf16 bf2, __bf16 bf3, __bf16 bf4, __bf16 bf5,
            __bf16 bf6, __bf16 bf7, __bf16 bf8) {
  return (__m128bh)(__v8bf){bf1, bf2, bf3, bf4, bf5, bf6, bf7, bf8};
}

static __inline __m256bh __DEFAULT_FN_ATTRS256 _mm256_set_pbh(
    __bf16 bf1, __bf16 bf2, __bf16 bf3, __bf16 bf4, __bf16 bf5, __bf16 bf6,
    __bf16 bf7, __bf16 bf8, __bf16 bf9, __bf16 bf10, __bf16 bf11, __bf16 bf12,
    __bf16 bf13, __bf16 bf14, __bf16 bf15, __bf16 bf16) {
  return (__m256bh)(__v16bf){bf1, bf2,  bf3,  bf4,  bf5,  bf6,  bf7,  bf8,
                             bf9, bf10, bf11, bf12, bf13, bf14, bf15, bf16};
}

#define _mm_setr_pbh(bf1, bf2, bf3, bf4, bf5, bf6, bf7, bf8)                   \
  _mm_set_pbh((bf8), (bf7), (bf6), (bf5), (bf4), (bf3), (bf2), (bf1))

#define _mm256_setr_pbh(bf1, bf2, bf3, bf4, bf5, bf6, bf7, bf8, bf9, bf10,     \
                        bf11, bf12, bf13, bf14, bf15, bf16)                    \
  _mm256_set_pbh((bf16), (bf15), (bf14), (bf13), (bf12), (bf11), (bf10),       \
                 (bf9), (bf8), (bf7), (bf6), (bf5), (bf4), (bf3), (bf2),       \
                 (bf1))

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_abs_pbh(__m256bh __A) {
  return (__m256bh)_mm256_and_epi32(_mm256_set1_epi32(0x7FFF7FFF),
                                    (__m256i)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_abs_pbh(__m128bh __A) {
  return (__m128bh)_mm_and_epi32(_mm_set1_epi32(0x7FFF7FFF), (__m128i)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_blend_pbh(__mmask8 __U, __m128bh __A, __m128bh __W) {
  return (__m128bh)__builtin_ia32_selectpbf_128((__mmask8)__U, (__v8bf)__W,
                                                (__v8bf)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_blend_pbh(__mmask16 __U, __m256bh __A, __m256bh __W) {
  return (__m256bh)__builtin_ia32_selectpbf_256((__mmask16)__U, (__v16bf)__W,
                                                (__v16bf)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_permutex2var_pbh(__m128bh __A, __m128i __I, __m128bh __B) {
  return (__m128bh)__builtin_ia32_vpermi2varhi128((__v8hi)__A, (__v8hi)__I,
                                                  (__v8hi)__B);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_permutex2var_pbh(__m256bh __A, __m256i __I, __m256bh __B) {
  return (__m256bh)__builtin_ia32_vpermi2varhi256((__v16hi)__A, (__v16hi)__I,
                                                  (__v16hi)__B);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_permutexvar_pbh(__m128i __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_permvarhi128((__v8hi)__B, (__v8hi)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_permutexvar_pbh(__m256i __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_permvarhi256((__v16hi)__B, (__v16hi)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_add_pbh(__m256bh __A,
                                                                __m256bh __B) {
  return (__m256bh)((__v16bf)__A + (__v16bf)__B);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_add_pbh(__m256bh __W, __mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_add_pbh(__A, __B), (__v16bf)__W);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_add_pbh(__mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_add_pbh(__A, __B),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_add_pbh(__m128bh __A,
                                                             __m128bh __B) {
  return (__m128bh)((__v8bf)__A + (__v8bf)__B);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_add_pbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_add_pbh(__A, __B), (__v8bf)__W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_add_pbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_add_pbh(__A, __B), (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_sub_pbh(__m256bh __A,
                                                                __m256bh __B) {
  return (__m256bh)((__v16bf)__A - (__v16bf)__B);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_sub_pbh(__m256bh __W, __mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_sub_pbh(__A, __B), (__v16bf)__W);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_sub_pbh(__mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_sub_pbh(__A, __B),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_sub_pbh(__m128bh __A,
                                                             __m128bh __B) {
  return (__m128bh)((__v8bf)__A - (__v8bf)__B);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_sub_pbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_sub_pbh(__A, __B), (__v8bf)__W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_sub_pbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_sub_pbh(__A, __B), (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mul_pbh(__m256bh __A,
                                                                __m256bh __B) {
  return (__m256bh)((__v16bf)__A * (__v16bf)__B);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_mul_pbh(__m256bh __W, __mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_mul_pbh(__A, __B), (__v16bf)__W);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_mul_pbh(__mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_mul_pbh(__A, __B),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_mul_pbh(__m128bh __A,
                                                             __m128bh __B) {
  return (__m128bh)((__v8bf)__A * (__v8bf)__B);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_mul_pbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_mul_pbh(__A, __B), (__v8bf)__W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_mul_pbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_mul_pbh(__A, __B), (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_div_pbh(__m256bh __A,
                                                                __m256bh __B) {
  return (__m256bh)((__v16bf)__A / (__v16bf)__B);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_div_pbh(__m256bh __W, __mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_div_pbh(__A, __B), (__v16bf)__W);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_div_pbh(__mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_div_pbh(__A, __B),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_div_pbh(__m128bh __A,
                                                             __m128bh __B) {
  return (__m128bh)((__v8bf)__A / (__v8bf)__B);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_div_pbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_div_pbh(__A, __B), (__v8bf)__W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_div_pbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_div_pbh(__A, __B), (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_max_pbh(__m256bh __A,
                                                                __m256bh __B) {
  return (__m256bh)__builtin_ia32_vmaxbf16256((__v16bf)__A, (__v16bf)__B);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_max_pbh(__m256bh __W, __mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_max_pbh(__A, __B), (__v16bf)__W);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_max_pbh(__mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_max_pbh(__A, __B),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_max_pbh(__m128bh __A,
                                                             __m128bh __B) {
  return (__m128bh)__builtin_ia32_vmaxbf16128((__v8bf)__A, (__v8bf)__B);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_max_pbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_max_pbh(__A, __B), (__v8bf)__W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_max_pbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_max_pbh(__A, __B), (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_min_pbh(__m256bh __A,
                                                                __m256bh __B) {
  return (__m256bh)__builtin_ia32_vminbf16256((__v16bf)__A, (__v16bf)__B);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_min_pbh(__m256bh __W, __mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_min_pbh(__A, __B), (__v16bf)__W);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_min_pbh(__mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_min_pbh(__A, __B),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_min_pbh(__m128bh __A,
                                                             __m128bh __B) {
  return (__m128bh)__builtin_ia32_vminbf16128((__v8bf)__A, (__v8bf)__B);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_min_pbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_min_pbh(__A, __B), (__v8bf)__W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_min_pbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_min_pbh(__A, __B), (__v8bf)_mm_setzero_pbh());
}

static __inline__ int __DEFAULT_FN_ATTRS128 _mm_comieq_sbh(__m128bh A,
                                                           __m128bh B) {
  return __builtin_ia32_vcomisbf16eq((__v8bf)A, (__v8bf)B);
}

static __inline__ int __DEFAULT_FN_ATTRS128 _mm_comilt_sbh(__m128bh A,
                                                           __m128bh B) {
  return __builtin_ia32_vcomisbf16lt((__v8bf)A, (__v8bf)B);
}

static __inline__ int __DEFAULT_FN_ATTRS128 _mm_comile_sbh(__m128bh A,
                                                           __m128bh B) {
  return __builtin_ia32_vcomisbf16le((__v8bf)A, (__v8bf)B);
}

static __inline__ int __DEFAULT_FN_ATTRS128 _mm_comigt_sbh(__m128bh A,
                                                           __m128bh B) {
  return __builtin_ia32_vcomisbf16gt((__v8bf)A, (__v8bf)B);
}

static __inline__ int __DEFAULT_FN_ATTRS128 _mm_comige_sbh(__m128bh A,
                                                           __m128bh B) {
  return __builtin_ia32_vcomisbf16ge((__v8bf)A, (__v8bf)B);
}

static __inline__ int __DEFAULT_FN_ATTRS128 _mm_comineq_sbh(__m128bh A,
                                                            __m128bh B) {
  return __builtin_ia32_vcomisbf16neq((__v8bf)A, (__v8bf)B);
}

#define _mm256_cmp_pbh_mask(__A, __B, __P)                                     \
  ((__mmask16)__builtin_ia32_vcmpbf16256_mask((__v16bf)(__m256bh)(__A),        \
                                              (__v16bf)(__m256bh)(__B),        \
                                              (int)(__P), (__mmask16) - 1))

#define _mm256_mask_cmp_pbh_mask(__U, __A, __B, __P)                           \
  ((__mmask16)__builtin_ia32_vcmpbf16256_mask((__v16bf)(__m256bh)(__A),        \
                                              (__v16bf)(__m256bh)(__B),        \
                                              (int)(__P), (__mmask16)(__U)))

#define _mm_cmp_pbh_mask(__A, __B, __P)                                        \
  ((__mmask8)__builtin_ia32_vcmpbf16128_mask((__v8bf)(__m128bh)(__A),          \
                                             (__v8bf)(__m128bh)(__B),          \
                                             (int)(__P), (__mmask8) - 1))

#define _mm_mask_cmp_pbh_mask(__U, __A, __B, __P)                              \
  ((__mmask8)__builtin_ia32_vcmpbf16128_mask((__v8bf)(__m128bh)(__A),          \
                                             (__v8bf)(__m128bh)(__B),          \
                                             (int)(__P), (__mmask8)(__U)))

#define _mm256_mask_fpclass_pbh_mask(__U, __A, imm)                            \
  ((__mmask16)__builtin_ia32_vfpclassbf16256_mask(                             \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__mmask16)(__U)))

#define _mm256_fpclass_pbh_mask(__A, imm)                                      \
  ((__mmask16)__builtin_ia32_vfpclassbf16256_mask(                             \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__mmask16) - 1))

#define _mm_mask_fpclass_pbh_mask(__U, __A, imm)                               \
  ((__mmask8)__builtin_ia32_vfpclassbf16128_mask((__v8bf)(__m128bh)(__A),      \
                                                 (int)(imm), (__mmask8)(__U)))

#define _mm_fpclass_pbh_mask(__A, imm)                                         \
  ((__mmask8)__builtin_ia32_vfpclassbf16128_mask((__v8bf)(__m128bh)(__A),      \
                                                 (int)(imm), (__mmask8) - 1))

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_scalef_pbh(__m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_vscalefbf16256_mask(
      (__v16bf)__A, (__v16bf)__B, (__v16bf)_mm256_undefined_pbh(),
      (__mmask16)-1);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mask_scalef_pbh(
    __m256bh __W, __mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_vscalefbf16256_mask(
      (__v16bf)__A, (__v16bf)__B, (__v16bf)__W, (__mmask16)__U);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_scalef_pbh(__mmask16 __U, __m256bh __A, __m256bh __B) {
  return (__m256bh)__builtin_ia32_vscalefbf16256_mask(
      (__v16bf)__A, (__v16bf)__B, (__v16bf)_mm256_setzero_pbh(),
      (__mmask16)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_scalef_pbh(__m128bh __A,
                                                                __m128bh __B) {
  return (__m128bh)__builtin_ia32_vscalefbf16128_mask(
      (__v8bf)__A, (__v8bf)__B, (__v8bf)_mm_undefined_pbh(), (__mmask8)-1);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_scalef_pbh(__m128bh __W, __mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_vscalefbf16128_mask(
      (__v8bf)__A, (__v8bf)__B, (__v8bf)__W, (__mmask8)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_scalef_pbh(__mmask8 __U, __m128bh __A, __m128bh __B) {
  return (__m128bh)__builtin_ia32_vscalefbf16128_mask(
      (__v8bf)__A, (__v8bf)__B, (__v8bf)_mm_setzero_pbh(), (__mmask8)__U);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_rcp_pbh(__m256bh __A) {
  return (__m256bh)__builtin_ia32_vrcpbf16256_mask(
      (__v16bf)__A, (__v16bf)_mm256_undefined_pbh(), (__mmask16)-1);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_rcp_pbh(__m256bh __W, __mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_vrcpbf16256_mask((__v16bf)__A, (__v16bf)__W,
                                                   (__mmask16)__U);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_rcp_pbh(__mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_vrcpbf16256_mask(
      (__v16bf)__A, (__v16bf)_mm256_setzero_pbh(), (__mmask16)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_rcp_pbh(__m128bh __A) {
  return (__m128bh)__builtin_ia32_vrcpbf16128_mask(
      (__v8bf)__A, (__v8bf)_mm_undefined_pbh(), (__mmask8)-1);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_rcp_pbh(__m128bh __W, __mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_vrcpbf16128_mask((__v8bf)__A, (__v8bf)__W,
                                                   (__mmask8)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_rcp_pbh(__mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_vrcpbf16128_mask(
      (__v8bf)__A, (__v8bf)_mm_setzero_pbh(), (__mmask8)__U);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_getexp_pbh(__m256bh __A) {
  return (__m256bh)__builtin_ia32_vgetexpbf16256_mask(
      (__v16bf)__A, (__v16bf)_mm256_undefined_pbh(), (__mmask16)-1);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_getexp_pbh(__m256bh __W, __mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_vgetexpbf16256_mask(
      (__v16bf)__A, (__v16bf)__W, (__mmask16)__U);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_getexp_pbh(__mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_vgetexpbf16256_mask(
      (__v16bf)__A, (__v16bf)_mm256_setzero_pbh(), (__mmask16)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_getexp_pbh(__m128bh __A) {
  return (__m128bh)__builtin_ia32_vgetexpbf16128_mask(
      (__v8bf)__A, (__v8bf)_mm_undefined_pbh(), (__mmask8)-1);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_getexp_pbh(__m128bh __W, __mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_vgetexpbf16128_mask((__v8bf)__A, (__v8bf)__W,
                                                      (__mmask8)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_getexp_pbh(__mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_vgetexpbf16128_mask(
      (__v8bf)__A, (__v8bf)_mm_setzero_pbh(), (__mmask8)__U);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_rsqrt_pbh(__m256bh __A) {
  return (__m256bh)__builtin_ia32_vrsqrtbf16256_mask(
      (__v16bf)__A, (__v16bf)_mm256_undefined_pbh(), (__mmask16)-1);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_rsqrt_pbh(__m256bh __W, __mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_vrsqrtbf16256_mask((__v16bf)__A, (__v16bf)__W,
                                                     (__mmask16)__U);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_rsqrt_pbh(__mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_vrsqrtbf16256_mask(
      (__v16bf)__A, (__v16bf)_mm256_setzero_pbh(), (__mmask16)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_rsqrt_pbh(__m128bh __A) {
  return (__m128bh)__builtin_ia32_vrsqrtbf16128_mask(
      (__v8bf)__A, (__v8bf)_mm_undefined_pbh(), (__mmask8)-1);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_rsqrt_pbh(__m128bh __W, __mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_vrsqrtbf16128_mask((__v8bf)__A, (__v8bf)__W,
                                                     (__mmask8)__U);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_rsqrt_pbh(__mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_vrsqrtbf16128_mask(
      (__v8bf)__A, (__v8bf)_mm_setzero_pbh(), (__mmask8)__U);
}

#define _mm256_reduce_pbh(__A, imm)                                            \
  ((__m256bh)__builtin_ia32_vreducebf16256_mask(                               \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__v16bf)_mm256_undefined_pbh(),   \
      (__mmask16) - 1))

#define _mm256_mask_reduce_pbh(__W, __U, __A, imm)                             \
  ((__m256bh)__builtin_ia32_vreducebf16256_mask(                               \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__v16bf)(__m256bh)(__W),          \
      (__mmask16)(__U)))

#define _mm256_maskz_reduce_pbh(__U, __A, imm)                                 \
  ((__m256bh)__builtin_ia32_vreducebf16256_mask(                               \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__v16bf)_mm256_setzero_pbh(),     \
      (__mmask16)(__U)))

#define _mm_reduce_pbh(__A, imm)                                               \
  ((__m128bh)__builtin_ia32_vreducebf16128_mask(                               \
      (__v8bf)(__m128bh)(__A), (int)(imm), (__v8bf)_mm_undefined_pbh(),        \
      (__mmask8) - 1))

#define _mm_mask_reduce_pbh(__W, __U, __A, imm)                                \
  ((__m128bh)__builtin_ia32_vreducebf16128_mask(                               \
      (__v8bf)(__m128bh)(__A), (int)(imm), (__v8bf)(__m128bh)(__W),            \
      (__mmask8)(__U)))

#define _mm_maskz_reduce_pbh(__U, __A, imm)                                    \
  ((__m128bh)__builtin_ia32_vreducebf16128_mask(                               \
      (__v8bf)(__m128bh)(__A), (int)(imm), (__v8bf)_mm_setzero_pbh(),          \
      (__mmask8)(__U)))

#define _mm256_roundscale_pbh(__A, imm)                                        \
  ((__m256bh)__builtin_ia32_vrndscalebf16_256_mask(                            \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__v16bf)_mm256_setzero_pbh(),     \
      (__mmask16) - 1))

#define _mm256_mask_roundscale_pbh(__W, __U, __A, imm)                         \
  ((__m256bh)__builtin_ia32_vrndscalebf16_256_mask(                            \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__v16bf)(__m256bh)(__W),          \
      (__mmask16)(__U)))

#define _mm256_maskz_roundscale_pbh(__U, __A, imm)                             \
  ((__m256bh)__builtin_ia32_vrndscalebf16_256_mask(                            \
      (__v16bf)(__m256bh)(__A), (int)(imm), (__v16bf)_mm256_setzero_pbh(),     \
      (__mmask16)(__U)))

#define _mm_roundscale_pbh(__A, imm)                                           \
  ((__m128bh)__builtin_ia32_vrndscalebf16_128_mask(                            \
      (__v8bf)(__m128bh)(__A), (int)(imm), (__v8bf)_mm_setzero_pbh(),          \
      (__mmask8) - 1))

#define _mm_mask_roundscale_pbh(__W, __U, __A, imm)                            \
  ((__m128bh)__builtin_ia32_vrndscalebf16_128_mask(                            \
      (__v8bf)(__m128bh)(__A), (int)(imm), (__v8bf)(__m128bh)(__W),            \
      (__mmask8)(__U)))

#define _mm_maskz_roundscale_pbh(__U, __A, imm)                                \
  ((__m128bh)__builtin_ia32_vrndscalebf16_128_mask(                            \
      (__v8bf)(__m128bh)(__A), (int)(imm), (__v8bf)_mm_setzero_pbh(),          \
      (__mmask8)(__U)))

#define _mm256_getmant_pbh(__A, __B, __C)                                      \
  ((__m256bh)__builtin_ia32_vgetmantbf16256_mask(                              \
      (__v16bf)(__m256bh)(__A), (int)(((__C) << 2) | (__B)),                   \
      (__v16bf)_mm256_undefined_pbh(), (__mmask16) - 1))

#define _mm256_mask_getmant_pbh(__W, __U, __A, __B, __C)                       \
  ((__m256bh)__builtin_ia32_vgetmantbf16256_mask(                              \
      (__v16bf)(__m256bh)(__A), (int)(((__C) << 2) | (__B)),                   \
      (__v16bf)(__m256bh)(__W), (__mmask16)(__U)))

#define _mm256_maskz_getmant_pbh(__U, __A, __B, __C)                           \
  ((__m256bh)__builtin_ia32_vgetmantbf16256_mask(                              \
      (__v16bf)(__m256bh)(__A), (int)(((__C) << 2) | (__B)),                   \
      (__v16bf)_mm256_setzero_pbh(), (__mmask16)(__U)))

#define _mm_getmant_pbh(__A, __B, __C)                                         \
  ((__m128bh)__builtin_ia32_vgetmantbf16128_mask(                              \
      (__v8bf)(__m128bh)(__A), (int)(((__C) << 2) | (__B)),                    \
      (__v8bf)_mm_undefined_pbh(), (__mmask8) - 1))

#define _mm_mask_getmant_pbh(__W, __U, __A, __B, __C)                          \
  ((__m128bh)__builtin_ia32_vgetmantbf16128_mask(                              \
      (__v8bf)(__m128bh)(__A), (int)(((__C) << 2) | (__B)),                    \
      (__v8bf)(__m128bh)(__W), (__mmask8)(__U)))

#define _mm_maskz_getmant_pbh(__U, __A, __B, __C)                              \
  ((__m128bh)__builtin_ia32_vgetmantbf16128_mask(                              \
      (__v8bf)(__m128bh)(__A), (int)(((__C) << 2) | (__B)),                    \
      (__v8bf)_mm_setzero_pbh(), (__mmask8)(__U)))

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_sqrt_pbh(__m256bh __A) {
  return (__m256bh)__builtin_ia32_vsqrtbf16256((__v16bf)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_sqrt_pbh(__m256bh __W, __mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U, (__v16bf)_mm256_sqrt_pbh(__A), (__v16bf)__W);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_maskz_sqrt_pbh(__mmask16 __U, __m256bh __A) {
  return (__m256bh)__builtin_ia32_selectpbf_256((__mmask16)__U,
                                                (__v16bf)_mm256_sqrt_pbh(__A),
                                                (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_sqrt_pbh(__m128bh __A) {
  return (__m128bh)__builtin_ia32_vsqrtbf16((__v8bf)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_sqrt_pbh(__m128bh __W, __mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_sqrt_pbh(__A), (__v8bf)__W);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_sqrt_pbh(__mmask8 __U, __m128bh __A) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, (__v8bf)_mm_sqrt_pbh(__A), (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_fmadd_pbh(__m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_vfmaddbf16256((__v16bf)__A, (__v16bf)__B,
                                                (__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_fmadd_pbh(__m256bh __A, __mmask16 __U, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fmadd_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C), (__v16bf)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mask3_fmadd_pbh(
    __m256bh __A, __m256bh __B, __m256bh __C, __mmask16 __U) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fmadd_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C), (__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_maskz_fmadd_pbh(
    __mmask16 __U, __m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fmadd_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_fmsub_pbh(__m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_vfmaddbf16256((__v16bf)__A, (__v16bf)__B,
                                                -(__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_mask_fmsub_pbh(__m256bh __A, __mmask16 __U, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fmsub_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C), (__v16bf)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mask3_fmsub_pbh(
    __m256bh __A, __m256bh __B, __m256bh __C, __mmask16 __U) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fmsub_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C), (__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_maskz_fmsub_pbh(
    __mmask16 __U, __m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fmsub_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_fnmadd_pbh(__m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_vfmaddbf16256((__v16bf)__A, -(__v16bf)__B,
                                                (__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mask_fnmadd_pbh(
    __m256bh __A, __mmask16 __U, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fnmadd_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mask3_fnmadd_pbh(
    __m256bh __A, __m256bh __B, __m256bh __C, __mmask16 __U) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fnmadd_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_maskz_fnmadd_pbh(
    __mmask16 __U, __m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fnmadd_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256
_mm256_fnmsub_pbh(__m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_vfmaddbf16256((__v16bf)__A, -(__v16bf)__B,
                                                -(__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mask_fnmsub_pbh(
    __m256bh __A, __mmask16 __U, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fnmsub_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)__A);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_mask3_fnmsub_pbh(
    __m256bh __A, __m256bh __B, __m256bh __C, __mmask16 __U) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fnmsub_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)__C);
}

static __inline__ __m256bh __DEFAULT_FN_ATTRS256 _mm256_maskz_fnmsub_pbh(
    __mmask16 __U, __m256bh __A, __m256bh __B, __m256bh __C) {
  return (__m256bh)__builtin_ia32_selectpbf_256(
      (__mmask16)__U,
      _mm256_fnmsub_pbh((__v16bf)__A, (__v16bf)__B, (__v16bf)__C),
      (__v16bf)_mm256_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_fmadd_pbh(__m128bh __A,
                                                               __m128bh __B,
                                                               __m128bh __C) {
  return (__m128bh)__builtin_ia32_vfmaddbf16128((__v8bf)__A, (__v8bf)__B,
                                                (__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_fmadd_pbh(__m128bh __A, __mmask8 __U, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fmadd_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask3_fmadd_pbh(__m128bh __A, __m128bh __B, __m128bh __C, __mmask8 __U) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fmadd_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_fmadd_pbh(__mmask8 __U, __m128bh __A, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fmadd_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_fmsub_pbh(__m128bh __A,
                                                               __m128bh __B,
                                                               __m128bh __C) {
  return (__m128bh)__builtin_ia32_vfmaddbf16128((__v8bf)__A, (__v8bf)__B,
                                                -(__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_fmsub_pbh(__m128bh __A, __mmask8 __U, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fmsub_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask3_fmsub_pbh(__m128bh __A, __m128bh __B, __m128bh __C, __mmask8 __U) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fmsub_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_fmsub_pbh(__mmask8 __U, __m128bh __A, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fmsub_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_fnmadd_pbh(__m128bh __A,
                                                                __m128bh __B,
                                                                __m128bh __C) {
  return (__m128bh)__builtin_ia32_vfmaddbf16128((__v8bf)__A, -(__v8bf)__B,
                                                (__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_fnmadd_pbh(__m128bh __A, __mmask8 __U, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fnmadd_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask3_fnmadd_pbh(__m128bh __A, __m128bh __B, __m128bh __C, __mmask8 __U) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fnmadd_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_fnmadd_pbh(__mmask8 __U, __m128bh __A, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fnmadd_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)_mm_setzero_pbh());
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128 _mm_fnmsub_pbh(__m128bh __A,
                                                                __m128bh __B,
                                                                __m128bh __C) {
  return (__m128bh)__builtin_ia32_vfmaddbf16128((__v8bf)__A, -(__v8bf)__B,
                                                -(__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask_fnmsub_pbh(__m128bh __A, __mmask8 __U, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fnmsub_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__A);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_mask3_fnmsub_pbh(__m128bh __A, __m128bh __B, __m128bh __C, __mmask8 __U) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fnmsub_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)__C);
}

static __inline__ __m128bh __DEFAULT_FN_ATTRS128
_mm_maskz_fnmsub_pbh(__mmask8 __U, __m128bh __A, __m128bh __B, __m128bh __C) {
  return (__m128bh)__builtin_ia32_selectpbf_128(
      (__mmask8)__U, _mm_fnmsub_pbh((__v8bf)__A, (__v8bf)__B, (__v8bf)__C),
      (__v8bf)_mm_setzero_pbh());
}

#undef __DEFAULT_FN_ATTRS128
#undef __DEFAULT_FN_ATTRS256

#endif
#endif
