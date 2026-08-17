/*===-------- avx10_2minmaxintrin.h - AVX10_2MINMAX intrinsics -------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2minmaxintrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifndef __AVX10_2MINMAXINTRIN_H
#define __AVX10_2MINMAXINTRIN_H

#define _mm_minmax_pbh(A, B, C)                                                \
  ((__m128bh)__builtin_ia32_vminmaxbf16128((__m128bh)(__v8bf)(A),              \
                                           (__m128bh)(__v8bf)(B), (int)(C)))

#define _mm_mask_minmax_pbh(W, U, A, B, C)                                     \
  ((__m128bh)__builtin_ia32_selectpbf_128(                                     \
      (__mmask8)(U),                                                           \
      (__v8bf)_mm_minmax_pbh((__m128bh)(__v8bf)(A), (__m128bh)(__v8bf)(B),     \
                             (int)(C)),                                        \
      (__v8bf)(W)))

#define _mm_maskz_minmax_pbh(U, A, B, C)                                       \
  ((__m128bh)__builtin_ia32_selectpbf_128(                                     \
      (__mmask8)(U),                                                           \
      (__v8bf)_mm_minmax_pbh((__m128bh)(__v8bf)(A), (__m128bh)(__v8bf)(B),     \
                             (int)(C)),                                        \
      (__v8bf) __builtin_bit_cast(__m128bh, _mm_setzero_ps())))

#define _mm256_minmax_pbh(A, B, C)                                             \
  ((__m256bh)__builtin_ia32_vminmaxbf16256((__m256bh)(__v16bf)(A),             \
                                           (__m256bh)(__v16bf)(B), (int)(C)))

#define _mm256_mask_minmax_pbh(W, U, A, B, C)                                  \
  ((__m256bh)__builtin_ia32_selectpbf_256(                                     \
      (__mmask16)(U),                                                          \
      (__v16bf)_mm256_minmax_pbh((__m256bh)(__v16bf)(A),                       \
                                 (__m256bh)(__v16bf)(B), (int)(C)),            \
      (__v16bf)(W)))

#define _mm256_maskz_minmax_pbh(U, A, B, C)                                    \
  ((__m256bh)__builtin_ia32_selectpbf_256(                                     \
      (__mmask16)(U),                                                          \
      (__v16bf)_mm256_minmax_pbh((__m256bh)(__v16bf)(A),                       \
                                 (__m256bh)(__v16bf)(B), (int)(C)),            \
      (__v16bf) __builtin_bit_cast(__m256bh, _mm256_setzero_ps())))

#define _mm_minmax_pd(A, B, C)                                                 \
  ((__m128d)__builtin_ia32_vminmaxpd128_mask(                                  \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)_mm_setzero_pd(), (__mmask8)-1))

#define _mm_mask_minmax_pd(W, U, A, B, C)                                      \
  ((__m128d)__builtin_ia32_vminmaxpd128_mask(                                  \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)(__m128d)(W), (__mmask8)(U)))

#define _mm_maskz_minmax_pd(U, A, B, C)                                        \
  ((__m128d)__builtin_ia32_vminmaxpd128_mask(                                  \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)_mm_setzero_pd(), (__mmask8)(U)))

#define _mm256_minmax_pd(A, B, C)                                              \
  ((__m256d)__builtin_ia32_vminmaxpd256_mask(                                  \
      (__v4df)(__m256d)(A), (__v4df)(__m256d)(B), (int)(C),                    \
      (__v4df)_mm256_setzero_pd(), (__mmask8)-1))

#define _mm256_mask_minmax_pd(W, U, A, B, C)                                   \
  ((__m256d)__builtin_ia32_vminmaxpd256_mask(                                  \
      (__v4df)(__m256d)(A), (__v4df)(__m256d)(B), (int)(C),                    \
      (__v4df)(__m256d)(W), (__mmask8)(U)))

#define _mm256_maskz_minmax_pd(U, A, B, C)                                     \
  ((__m256d)__builtin_ia32_vminmaxpd256_mask(                                  \
      (__v4df)(__m256d)(A), (__v4df)(__m256d)(B), (int)(C),                    \
      (__v4df)_mm256_setzero_pd(), (__mmask8)(U)))

#define _mm_minmax_ph(A, B, C)                                                 \
  ((__m128h)__builtin_ia32_vminmaxph128_mask(                                  \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)_mm_setzero_ph(), (__mmask8)-1))

#define _mm_mask_minmax_ph(W, U, A, B, C)                                      \
  ((__m128h)__builtin_ia32_vminmaxph128_mask(                                  \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)(__m128h)(W), (__mmask16)-1))

#define _mm_maskz_minmax_ph(U, A, B, C)                                        \
  ((__m128h)__builtin_ia32_vminmaxph128_mask(                                  \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)_mm_setzero_ph(), (__mmask8)(U)))

#define _mm256_minmax_ph(A, B, C)                                              \
  ((__m256h)__builtin_ia32_vminmaxph256_mask(                                  \
      (__v16hf)(__m256h)(A), (__v16hf)(__m256h)(B), (int)(C),                  \
      (__v16hf)_mm256_setzero_ph(), (__mmask16)-1))

#define _mm256_mask_minmax_ph(W, U, A, B, C)                                   \
  ((__m256h)__builtin_ia32_vminmaxph256_mask(                                  \
      (__v16hf)(__m256h)(A), (__v16hf)(__m256h)(B), (int)(C),                  \
      (__v16hf)(__m256h)(W), (__mmask16)(U)))

#define _mm256_maskz_minmax_ph(U, A, B, C)                                     \
  ((__m256h)__builtin_ia32_vminmaxph256_mask(                                  \
      (__v16hf)(__m256h)(A), (__v16hf)(__m256h)(B), (int)(C),                  \
      (__v16hf)_mm256_setzero_ph(), (__mmask16)(U)))

#define _mm_minmax_ps(A, B, C)                                                 \
  ((__m128)__builtin_ia32_vminmaxps128_mask(                                   \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C),                      \
      (__v4sf)_mm_setzero_ps(), (__mmask8)-1))

#define _mm_mask_minmax_ps(W, U, A, B, C)                                      \
  ((__m128)__builtin_ia32_vminmaxps128_mask(                                   \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C), (__v4sf)(__m128)(W), \
      (__mmask8)(U)))

#define _mm_maskz_minmax_ps(U, A, B, C)                                        \
  ((__m128)__builtin_ia32_vminmaxps128_mask(                                   \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C),                      \
      (__v4sf)_mm_setzero_ps(), (__mmask8)(U)))

#define _mm256_minmax_ps(A, B, C)                                              \
  ((__m256)__builtin_ia32_vminmaxps256_mask(                                   \
      (__v8sf)(__m256)(A), (__v8sf)(__m256)(B), (int)(C),                      \
      (__v8sf)_mm256_setzero_ps(), (__mmask8)-1))

#define _mm256_mask_minmax_ps(W, U, A, B, C)                                   \
  ((__m256)__builtin_ia32_vminmaxps256_mask(                                   \
      (__v8sf)(__m256)(A), (__v8sf)(__m256)(B), (int)(C), (__v8sf)(__m256)(W), \
      (__mmask8)(U)))

#define _mm256_maskz_minmax_ps(U, A, B, C)                                     \
  ((__m256)__builtin_ia32_vminmaxps256_mask(                                   \
      (__v8sf)(__m256)(A), (__v8sf)(__m256)(B), (int)(C),                      \
      (__v8sf)_mm256_setzero_ps(), (__mmask8)(U)))

#define _mm_minmax_sd(A, B, C)                                                 \
  ((__m128d)__builtin_ia32_vminmaxsd_round_mask(                               \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)_mm_undefined_pd(), (__mmask8)-1, _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_minmax_sd(W, U, A, B, C)                                      \
  ((__m128d)__builtin_ia32_vminmaxsd_round_mask(                               \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)(__m128d)(W), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_minmax_sd(U, A, B, C)                                        \
  ((__m128d)__builtin_ia32_vminmaxsd_round_mask(                               \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)_mm_setzero_pd(), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_minmax_round_sd(A, B, C, R)                                        \
  ((__m128d)__builtin_ia32_vminmaxsd_round_mask(                               \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)_mm_undefined_pd(), (__mmask8)-1, (int)(R)))

#define _mm_mask_minmax_round_sd(W, U, A, B, C, R)                             \
  ((__m128d)__builtin_ia32_vminmaxsd_round_mask(                               \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)(__m128d)(W), (__mmask8)(U), (int)(R)))

#define _mm_maskz_minmax_round_sd(U, A, B, C, R)                               \
  ((__m128d)__builtin_ia32_vminmaxsd_round_mask(                               \
      (__v2df)(__m128d)(A), (__v2df)(__m128d)(B), (int)(C),                    \
      (__v2df)_mm_setzero_pd(), (__mmask8)(U), (int)(R)))

#define _mm_minmax_sh(A, B, C)                                                 \
  ((__m128h)__builtin_ia32_vminmaxsh_round_mask(                               \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)_mm_undefined_ph(), (__mmask8)-1, _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_minmax_sh(W, U, A, B, C)                                      \
  ((__m128h)__builtin_ia32_vminmaxsh_round_mask(                               \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)(__m128h)(W), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_minmax_sh(U, A, B, C)                                        \
  ((__m128h)__builtin_ia32_vminmaxsh_round_mask(                               \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)_mm_setzero_ph(), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_minmax_round_sh(A, B, C, R)                                        \
  ((__m128h)__builtin_ia32_vminmaxsh_round_mask(                               \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)_mm_undefined_ph(), (__mmask8)-1, (int)(R)))

#define _mm_mask_minmax_round_sh(W, U, A, B, C, R)                             \
  ((__m128h)__builtin_ia32_vminmaxsh_round_mask(                               \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)(__m128h)(W), (__mmask8)(U), (int)(R)))

#define _mm_maskz_minmax_round_sh(U, A, B, C, R)                               \
  ((__m128h)__builtin_ia32_vminmaxsh_round_mask(                               \
      (__v8hf)(__m128h)(A), (__v8hf)(__m128h)(B), (int)(C),                    \
      (__v8hf)_mm_setzero_ph(), (__mmask8)(U), (int)(R)))

#define _mm_minmax_ss(A, B, C)                                                 \
  ((__m128)__builtin_ia32_vminmaxss_round_mask(                                \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C),                      \
      (__v4sf)_mm_undefined_ps(), (__mmask8)-1, _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_minmax_ss(W, U, A, B, C)                                      \
  ((__m128)__builtin_ia32_vminmaxss_round_mask(                                \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C), (__v4sf)(W),         \
      (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_minmax_ss(U, A, B, C)                                        \
  ((__m128)__builtin_ia32_vminmaxss_round_mask(                                \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C),                      \
      (__v4sf)_mm_setzero_ps(), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_minmax_round_ss(A, B, C, R)                                        \
  ((__m128)__builtin_ia32_vminmaxss_round_mask(                                \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C),                      \
      (__v4sf)_mm_undefined_ps(), (__mmask8)-1, (int)(R)))

#define _mm_mask_minmax_round_ss(W, U, A, B, C, R)                             \
  ((__m128)__builtin_ia32_vminmaxss_round_mask(                                \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C), (__v4sf)(W),         \
      (__mmask8)(U), (int)(R)))

#define _mm_maskz_minmax_round_ss(U, A, B, C, R)                               \
  ((__m128)__builtin_ia32_vminmaxss_round_mask(                                \
      (__v4sf)(__m128)(A), (__v4sf)(__m128)(B), (int)(C),                      \
      (__v4sf)_mm_setzero_ps(), (__mmask8)(U), (int)(R)))
#endif // __AVX10_2MINMAXINTRIN_H
