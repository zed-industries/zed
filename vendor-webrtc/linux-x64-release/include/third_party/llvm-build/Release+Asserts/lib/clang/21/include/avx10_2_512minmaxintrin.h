/*===---- avx10_2_512minmaxintrin.h - AVX10_2_512MINMAX intrinsics ---------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2_512minmaxintrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifndef __AVX10_2_512MINMAXINTRIN_H
#define __AVX10_2_512MINMAXINTRIN_H

#define _mm512_minmax_pbh(A, B, C)                                             \
  ((__m512bh)__builtin_ia32_vminmaxbf16512((__v32bf)(__m512bh)(A),             \
                                           (__v32bf)(__m512bh)(A), (int)(C)))

#define _mm512_mask_minmax_pbh(W, U, A, B, C)                                  \
  ((__m512bh)__builtin_ia32_selectpbf_512(                                     \
      (__mmask32)(U),                                                          \
      (__v32bf)_mm512_minmax_pbh((__v32bf)(__m512bh)(A),                       \
                                 (__v32bf)(__m512bh)(B), (int)(C)),            \
      (__v32bf)(__m512bh)(W)))

#define _mm512_maskz_minmax_pbh(U, A, B, C)                                    \
  ((__m512bh)__builtin_ia32_selectpbf_512(                                     \
      (__mmask32)(U),                                                          \
      (__v32bf)_mm512_minmax_pbh((__v32bf)(__m512bh)(A),                       \
                                 (__v32bf)(__m512bh)(B), (int)(C)),            \
      (__v32bf) __builtin_bit_cast(__m512bh, _mm512_setzero_ps())))

#define _mm512_minmax_pd(A, B, C)                                              \
  ((__m512d)__builtin_ia32_vminmaxpd512_round_mask(                            \
      (__v8df)(__m512d)(A), (__v8df)(__m512d)(B), (int)(C),                    \
      (__v8df)_mm512_undefined_pd(), (__mmask8)-1,                             \
      _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_minmax_pd(W, U, A, B, C)                                   \
  ((__m512d)__builtin_ia32_vminmaxpd512_round_mask(                            \
      (__v8df)(__m512d)(A), (__v8df)(__m512d)(B), (int)(C),                    \
      (__v8df)(__m512d)(W), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_minmax_pd(U, A, B, C)                                     \
  ((__m512d)__builtin_ia32_vminmaxpd512_round_mask(                            \
      (__v8df)(__m512d)(A), (__v8df)(__m512d)(B), (int)(C),                    \
      (__v8df)_mm512_setzero_pd(), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_minmax_round_pd(A, B, C, R)                                     \
  ((__m512d)__builtin_ia32_vminmaxpd512_round_mask(                            \
      (__v8df)(__m512d)(A), (__v8df)(__m512d)(B), (int)(C),                    \
      (__v8df)_mm512_undefined_pd(), (__mmask8)-1, (int)(R)))

#define _mm512_mask_minmax_round_pd(W, U, A, B, C, R)                          \
  ((__m512d)__builtin_ia32_vminmaxpd512_round_mask(                            \
      (__v8df)(__m512d)(A), (__v8df)(__m512d)(B), (int)(C),                    \
      (__v8df)(__m512d)(W), (__mmask8)(U), (int)(R)))

#define _mm512_maskz_minmax_round_pd(U, A, B, C, R)                            \
  ((__m512d)__builtin_ia32_vminmaxpd512_round_mask(                            \
      (__v8df)(__m512d)(A), (__v8df)(__m512d)(B), (int)(C),                    \
      (__v8df)_mm512_setzero_pd(), (__mmask8)(U), (int)(R)))

#define _mm512_minmax_ph(A, B, C)                                              \
  ((__m512h)__builtin_ia32_vminmaxph512_round_mask(                            \
      (__v32hf)(__m512h)(A), (__v32hf)(__m512h)(B), (int)(C),                  \
      (__v32hf)_mm512_undefined_ph(), (__mmask32)-1,                           \
      _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_minmax_ph(W, U, A, B, C)                                   \
  ((__m512h)__builtin_ia32_vminmaxph512_round_mask(                            \
      (__v32hf)(__m512h)(A), (__v32hf)(__m512h)(B), (int)(C),                  \
      (__v32hf)(__m512h)(W), (__mmask32)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_minmax_ph(U, A, B, C)                                     \
  ((__m512h)__builtin_ia32_vminmaxph512_round_mask(                            \
      (__v32hf)(__m512h)(A), (__v32hf)(__m512h)(B), (int)(C),                  \
      (__v32hf)_mm512_setzero_ph(), (__mmask32)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_minmax_round_ph(A, B, C, R)                                     \
  ((__m512h)__builtin_ia32_vminmaxph512_round_mask(                            \
      (__v32hf)(__m512h)(A), (__v32hf)(__m512h)(B), (int)(C),                  \
      (__v32hf)_mm512_undefined_ph(), (__mmask32)-1, (int)(R)))

#define _mm512_mask_minmax_round_ph(W, U, A, B, C, R)                          \
  ((__m512h)__builtin_ia32_vminmaxph512_round_mask(                            \
      (__v32hf)(__m512h)(A), (__v32hf)(__m512h)(B), (int)(C),                  \
      (__v32hf)(__m512h)(W), (__mmask32)(U), (int)(R)))

#define _mm512_maskz_minmax_round_ph(U, A, B, C, R)                            \
  ((__m512h)__builtin_ia32_vminmaxph512_round_mask(                            \
      (__v32hf)(__m512h)(A), (__v32hf)(__m512h)(B), (int)(C),                  \
      (__v32hf)_mm512_setzero_ph(), (__mmask32)(U), (int)(R)))

#define _mm512_minmax_ps(A, B, C)                                              \
  ((__m512)__builtin_ia32_vminmaxps512_round_mask(                             \
      (__v16sf)(__m512)(A), (__v16sf)(__m512)(B), (int)(C),                    \
      (__v16sf)_mm512_undefined_ps(), (__mmask16)-1,                           \
      _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_minmax_ps(W, U, A, B, C)                                   \
  ((__m512)__builtin_ia32_vminmaxps512_round_mask(                             \
      (__v16sf)(__m512)(A), (__v16sf)(__m512)(B), (int)(C), (__v16sf)(W),      \
      (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_minmax_ps(U, A, B, C)                                     \
  ((__m512)__builtin_ia32_vminmaxps512_round_mask(                             \
      (__v16sf)(__m512)(A), (__v16sf)(__m512)(B), (int)(C),                    \
      (__v16sf)_mm512_setzero_ps(), (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_minmax_round_ps(A, B, C, R)                                     \
  ((__m512)__builtin_ia32_vminmaxps512_round_mask(                             \
      (__v16sf)(__m512)(A), (__v16sf)(__m512)(B), (int)(C),                    \
      (__v16sf)_mm512_undefined_ps(), (__mmask16)-1, (int)(R)))

#define _mm512_mask_minmax_round_ps(W, U, A, B, C, R)                          \
  ((__m512)__builtin_ia32_vminmaxps512_round_mask(                             \
      (__v16sf)(__m512)(A), (__v16sf)(__m512)(B), (int)(C), (__v16sf)(W),      \
      (__mmask16)(U), (int)(R)))

#define _mm512_maskz_minmax_round_ps(U, A, B, C, R)                            \
  ((__m512)__builtin_ia32_vminmaxps512_round_mask(                             \
      (__v16sf)(__m512)(A), (__v16sf)(__m512)(B), (int)(C),                    \
      (__v16sf)_mm512_setzero_ps(), (__mmask16)(U), (int)(R)))
#endif // __AVX10_2_512MINMAXINTRIN_H
