/*===---- avx10_2niintrin.h - AVX10.2 new instruction intrinsics -----------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error "Never use <avx10_2niintrin.h> directly; include <immintrin.h> instead."
#endif

#ifdef __SSE2__

#ifndef __AVX10_2NIINTRIN_H
#define __AVX10_2NIINTRIN_H

#define __DEFAULT_FN_ATTRS128                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-256"),    \
                 __min_vector_width__(128)))
#define __DEFAULT_FN_ATTRS256                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-256"),    \
                 __min_vector_width__(256)))

/* VNNI FP16 */
static __inline__ __m128 __DEFAULT_FN_ATTRS128 _mm_dpph_ps(__m128 __W,
                                                           __m128h __A,
                                                           __m128h __B) {
  return (__m128)__builtin_ia32_vdpphps128((__v4sf)__W, (__v8hf)__A,
                                           (__v8hf)__B);
}

static __inline__ __m128 __DEFAULT_FN_ATTRS128 _mm_mask_dpph_ps(__m128 __W,
                                                                __mmask8 __U,
                                                                __m128h __A,
                                                                __m128h __B) {
  return (__m128)__builtin_ia32_selectps_128(
      (__mmask8)__U, (__v4sf)_mm_dpph_ps(__W, __A, __B), (__v4sf)__W);
}

static __inline__ __m128 __DEFAULT_FN_ATTRS128 _mm_maskz_dpph_ps(__mmask8 __U,
                                                                 __m128 __W,
                                                                 __m128h __A,
                                                                 __m128h __B) {
  return (__m128)__builtin_ia32_selectps_128((__mmask8)__U,
                                             (__v4sf)_mm_dpph_ps(__W, __A, __B),
                                             (__v4sf)_mm_setzero_ps());
}

static __inline__ __m256 __DEFAULT_FN_ATTRS256 _mm256_dpph_ps(__m256 __W,
                                                              __m256h __A,
                                                              __m256h __B) {
  return (__m256)__builtin_ia32_vdpphps256((__v8sf)__W, (__v16hf)__A,
                                           (__v16hf)__B);
}

static __inline__ __m256 __DEFAULT_FN_ATTRS256
_mm256_mask_dpph_ps(__m256 __W, __mmask8 __U, __m256h __A, __m256h __B) {
  return (__m256)__builtin_ia32_selectps_256(
      (__mmask8)__U, (__v8sf)_mm256_dpph_ps(__W, __A, __B), (__v8sf)__W);
}

static __inline__ __m256 __DEFAULT_FN_ATTRS256
_mm256_maskz_dpph_ps(__mmask8 __U, __m256 __W, __m256h __A, __m256h __B) {
  return (__m256)__builtin_ia32_selectps_256(
      (__mmask8)__U, (__v8sf)_mm256_dpph_ps(__W, __A, __B),
      (__v8sf)_mm256_setzero_ps());
}

/* VMPSADBW */
#define _mm_mask_mpsadbw_epu8(W, U, A, B, imm)                                 \
  ((__m128i)__builtin_ia32_selectw_128(                                        \
      (__mmask8)(U), (__v8hi)_mm_mpsadbw_epu8((A), (B), (imm)),                \
      (__v8hi)(__m128i)(W)))

#define _mm_maskz_mpsadbw_epu8(U, A, B, imm)                                   \
  ((__m128i)__builtin_ia32_selectw_128(                                        \
      (__mmask8)(U), (__v8hi)_mm_mpsadbw_epu8((A), (B), (imm)),                \
      (__v8hi)_mm_setzero_si128()))

#define _mm256_mask_mpsadbw_epu8(W, U, A, B, imm)                              \
  ((__m256i)__builtin_ia32_selectw_256(                                        \
      (__mmask16)(U), (__v16hi)_mm256_mpsadbw_epu8((A), (B), (imm)),           \
      (__v16hi)(__m256i)(W)))

#define _mm256_maskz_mpsadbw_epu8(U, A, B, imm)                                \
  ((__m256i)__builtin_ia32_selectw_256(                                        \
      (__mmask16)(U), (__v16hi)_mm256_mpsadbw_epu8((A), (B), (imm)),           \
      (__v16hi)_mm256_setzero_si256()))

/* VNNI INT8 */
static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpbssd_epi32(__m128i __W, __mmask8 __U, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbssd_epi32(__W, __A, __B), (__v4si)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpbssd_epi32(__mmask8 __U, __m128i __W, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbssd_epi32(__W, __A, __B),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpbssd_epi32(__m256i __W, __mmask8 __U, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbssd_epi32(__W, __A, __B), (__v8si)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_dpbssd_epi32(__mmask8 __U, __m256i __W, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbssd_epi32(__W, __A, __B),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpbssds_epi32(__m128i __W, __mmask8 __U, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbssds_epi32(__W, __A, __B), (__v4si)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpbssds_epi32(__mmask8 __U, __m128i __W, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbssds_epi32(__W, __A, __B),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpbssds_epi32(__m256i __W, __mmask8 __U, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbssds_epi32(__W, __A, __B), (__v8si)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256 _mm256_maskz_dpbssds_epi32(
    __mmask8 __U, __m256i __W, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbssds_epi32(__W, __A, __B),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpbsud_epi32(__m128i __W, __mmask8 __U, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbsud_epi32(__W, __A, __B), (__v4si)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpbsud_epi32(__mmask8 __U, __m128i __W, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbsud_epi32(__W, __A, __B),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpbsud_epi32(__m256i __W, __mmask8 __U, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbsud_epi32(__W, __A, __B), (__v8si)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_dpbsud_epi32(__mmask8 __U, __m256i __W, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbsud_epi32(__W, __A, __B),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpbsuds_epi32(__m128i __W, __mmask8 __U, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbsuds_epi32(__W, __A, __B), (__v4si)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpbsuds_epi32(__mmask8 __U, __m128i __W, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbsuds_epi32(__W, __A, __B),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpbsuds_epi32(__m256i __W, __mmask8 __U, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbsuds_epi32(__W, __A, __B), (__v8si)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256 _mm256_maskz_dpbsuds_epi32(
    __mmask8 __U, __m256i __W, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbsuds_epi32(__W, __A, __B),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpbuud_epi32(__m128i __W, __mmask8 __U, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbuud_epi32(__W, __A, __B), (__v4si)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpbuud_epi32(__mmask8 __U, __m128i __W, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbuud_epi32(__W, __A, __B),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpbuud_epi32(__m256i __W, __mmask8 __U, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbuud_epi32(__W, __A, __B), (__v8si)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_dpbuud_epi32(__mmask8 __U, __m256i __W, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbuud_epi32(__W, __A, __B),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpbuuds_epi32(__m128i __W, __mmask8 __U, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbuuds_epi32(__W, __A, __B), (__v4si)__W);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpbuuds_epi32(__mmask8 __U, __m128i __W, __m128i __A, __m128i __B) {
  return (__m128i)__builtin_ia32_selectd_128(
      __U, (__v4si)_mm_dpbuuds_epi32(__W, __A, __B),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpbuuds_epi32(__m256i __W, __mmask8 __U, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbuuds_epi32(__W, __A, __B), (__v8si)__W);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256 _mm256_maskz_dpbuuds_epi32(
    __mmask8 __U, __m256i __W, __m256i __A, __m256i __B) {
  return (__m256i)__builtin_ia32_selectd_256(
      __U, (__v8si)_mm256_dpbuuds_epi32(__W, __A, __B),
      (__v8si)_mm256_setzero_si256());
}

/* VNNI INT16 */
static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpwsud_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwsud_epi32(__A, __B, __C), (__v4si)__A);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpwsud_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwsud_epi32(__A, __B, __C),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpwsud_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwsud_epi32(__A, __B, __C), (__v8si)__A);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_dpwsud_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwsud_epi32(__A, __B, __C),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpwsuds_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwsuds_epi32(__A, __B, __C), (__v4si)__A);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpwsuds_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwsuds_epi32(__A, __B, __C),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpwsuds_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwsuds_epi32(__A, __B, __C), (__v8si)__A);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256 _mm256_maskz_dpwsuds_epi32(
    __m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwsuds_epi32(__A, __B, __C),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpwusd_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwusd_epi32(__A, __B, __C), (__v4si)__A);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpwusd_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwusd_epi32(__A, __B, __C),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpwusd_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwusd_epi32(__A, __B, __C), (__v8si)__A);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_dpwusd_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwusd_epi32(__A, __B, __C),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpwusds_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwusds_epi32(__A, __B, __C), (__v4si)__A);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpwusds_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwusds_epi32(__A, __B, __C),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpwusds_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwusds_epi32(__A, __B, __C), (__v8si)__A);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256 _mm256_maskz_dpwusds_epi32(
    __m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwusds_epi32(__A, __B, __C),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpwuud_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwuud_epi32(__A, __B, __C), (__v4si)__A);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpwuud_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwuud_epi32(__A, __B, __C),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpwuud_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwuud_epi32(__A, __B, __C), (__v8si)__A);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_maskz_dpwuud_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwuud_epi32(__A, __B, __C),
      (__v8si)_mm256_setzero_si256());
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_mask_dpwuuds_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwuuds_epi32(__A, __B, __C), (__v4si)__A);
}

static __inline__ __m128i __DEFAULT_FN_ATTRS128
_mm_maskz_dpwuuds_epi32(__m128i __A, __mmask8 __U, __m128i __B, __m128i __C) {
  return (__m128i)__builtin_ia32_selectd_128(
      (__mmask8)__U, (__v4si)_mm_dpwuuds_epi32(__A, __B, __C),
      (__v4si)_mm_setzero_si128());
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256
_mm256_mask_dpwuuds_epi32(__m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwuuds_epi32(__A, __B, __C), (__v8si)__A);
}

static __inline__ __m256i __DEFAULT_FN_ATTRS256 _mm256_maskz_dpwuuds_epi32(
    __m256i __A, __mmask8 __U, __m256i __B, __m256i __C) {
  return (__m256i)__builtin_ia32_selectd_256(
      (__mmask8)__U, (__v8si)_mm256_dpwuuds_epi32(__A, __B, __C),
      (__v8si)_mm256_setzero_si256());
}

#undef __DEFAULT_FN_ATTRS256
#undef __DEFAULT_FN_ATTRS128

#endif /* __AVX10_2NIINTRIN_H */
#endif /* __SSE2__ */
