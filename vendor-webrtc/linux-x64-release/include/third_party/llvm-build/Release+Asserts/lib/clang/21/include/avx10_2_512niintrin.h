/*===---- avx10_2_512niintrin.h - AVX10.2-512 new instruction intrinsics ---===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2_512niintrin.h> directly; include <immintrin.h> instead."
#endif

#ifdef __SSE2__

#ifndef __AVX10_2_512NIINTRIN_H
#define __AVX10_2_512NIINTRIN_H

#define __DEFAULT_FN_ATTRS                                                     \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-512"),    \
                 __min_vector_width__(512)))

/* VNNI FP16 */
static __inline__ __m512 __DEFAULT_FN_ATTRS _mm512_dpph_ps(__m512 __W,
                                                           __m512h __A,
                                                           __m512h __B) {
  return (__m512)__builtin_ia32_vdpphps512((__v16sf)__W, (__v32hf)__A,
                                           (__v32hf)__B);
}

static __inline__ __m512 __DEFAULT_FN_ATTRS _mm512_mask_dpph_ps(__m512 __W,
                                                                __mmask16 __U,
                                                                __m512h __A,
                                                                __m512h __B) {
  return (__m512)__builtin_ia32_selectps_512(
      (__mmask16)__U, (__v16sf)_mm512_dpph_ps(__W, __A, __B), (__v16sf)__W);
}

static __inline__ __m512 __DEFAULT_FN_ATTRS _mm512_maskz_dpph_ps(__mmask16 __U,
                                                                 __m512 __W,
                                                                 __m512h __A,
                                                                 __m512h __B) {
  return (__m512)__builtin_ia32_selectps_512(
      (__mmask16)__U, (__v16sf)_mm512_dpph_ps(__W, __A, __B),
      (__v16sf)_mm512_setzero_ps());
}

/* VMPSADBW */
#define _mm512_mpsadbw_epu8(A, B, imm)                                         \
  ((__m512i)__builtin_ia32_mpsadbw512((__v64qi)(__m512i)(A),                   \
                                      (__v64qi)(__m512i)(B), (int)(imm)))

#define _mm512_mask_mpsadbw_epu8(W, U, A, B, imm)                              \
  ((__m512i)__builtin_ia32_selectw_512(                                        \
      (__mmask32)(U), (__v32hi)_mm512_mpsadbw_epu8((A), (B), (imm)),           \
      (__v32hi)(__m512i)(W)))

#define _mm512_maskz_mpsadbw_epu8(U, A, B, imm)                                \
  ((__m512i)__builtin_ia32_selectw_512(                                        \
      (__mmask32)(U), (__v32hi)_mm512_mpsadbw_epu8((A), (B), (imm)),           \
      (__v32hi)_mm512_setzero_si512()))

/* VNNI INT8 */
static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpbssd_epi32(__m512i __W,
                                                                 __m512i __A,
                                                                 __m512i __B) {
  return (__m512i)__builtin_ia32_vpdpbssd512((__v16si)__W, (__v16si)__A,
                                             (__v16si)__B);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_dpbssd_epi32(__m512i __W, __mmask16 __U, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbssd_epi32(__W, __A, __B), (__v16si)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpbssd_epi32(
    __mmask16 __U, __m512i __W, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbssd_epi32(__W, __A, __B),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpbssds_epi32(__m512i __W,
                                                                  __m512i __A,
                                                                  __m512i __B) {
  return (__m512i)__builtin_ia32_vpdpbssds512((__v16si)__W, (__v16si)__A,
                                              (__v16si)__B);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_mask_dpbssds_epi32(
    __m512i __W, __mmask16 __U, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbssds_epi32(__W, __A, __B), (__v16si)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpbssds_epi32(
    __mmask16 __U, __m512i __W, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbssds_epi32(__W, __A, __B),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpbsud_epi32(__m512i __W,
                                                                 __m512i __A,
                                                                 __m512i __B) {
  return (__m512i)__builtin_ia32_vpdpbsud512((__v16si)__W, (__v16si)__A,
                                             (__v16si)__B);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_dpbsud_epi32(__m512i __W, __mmask16 __U, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbsud_epi32(__W, __A, __B), (__v16si)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpbsud_epi32(
    __mmask16 __U, __m512i __W, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbsud_epi32(__W, __A, __B),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpbsuds_epi32(__m512i __W,
                                                                  __m512i __A,
                                                                  __m512i __B) {
  return (__m512i)__builtin_ia32_vpdpbsuds512((__v16si)__W, (__v16si)__A,
                                              (__v16si)__B);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_mask_dpbsuds_epi32(
    __m512i __W, __mmask16 __U, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbsuds_epi32(__W, __A, __B), (__v16si)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpbsuds_epi32(
    __mmask16 __U, __m512i __W, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbsuds_epi32(__W, __A, __B),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpbuud_epi32(__m512i __W,
                                                                 __m512i __A,
                                                                 __m512i __B) {
  return (__m512i)__builtin_ia32_vpdpbuud512((__v16si)__W, (__v16si)__A,
                                             (__v16si)__B);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_dpbuud_epi32(__m512i __W, __mmask16 __U, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbuud_epi32(__W, __A, __B), (__v16si)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpbuud_epi32(
    __mmask16 __U, __m512i __W, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbuud_epi32(__W, __A, __B),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpbuuds_epi32(__m512i __W,
                                                                  __m512i __A,
                                                                  __m512i __B) {
  return (__m512i)__builtin_ia32_vpdpbuuds512((__v16si)__W, (__v16si)__A,
                                              (__v16si)__B);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_mask_dpbuuds_epi32(
    __m512i __W, __mmask16 __U, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbuuds_epi32(__W, __A, __B), (__v16si)__W);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpbuuds_epi32(
    __mmask16 __U, __m512i __W, __m512i __A, __m512i __B) {
  return (__m512i)__builtin_ia32_selectd_512(
      __U, (__v16si)_mm512_dpbuuds_epi32(__W, __A, __B),
      (__v16si)_mm512_setzero_si512());
}

/* VNNI INT16 */
static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpwsud_epi32(__m512i __A,
                                                                 __m512i __B,
                                                                 __m512i __C) {
  return (__m512i)__builtin_ia32_vpdpwsud512((__v16si)__A, (__v16si)__B,
                                             (__v16si)__C);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_dpwsud_epi32(__m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwsud_epi32(__A, __B, __C),
      (__v16si)__A);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpwsud_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwsud_epi32(__A, __B, __C),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpwsuds_epi32(__m512i __A,
                                                                  __m512i __B,
                                                                  __m512i __C) {
  return (__m512i)__builtin_ia32_vpdpwsuds512((__v16si)__A, (__v16si)__B,
                                              (__v16si)__C);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_mask_dpwsuds_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwsuds_epi32(__A, __B, __C),
      (__v16si)__A);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpwsuds_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwsuds_epi32(__A, __B, __C),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpwusd_epi32(__m512i __A,
                                                                 __m512i __B,
                                                                 __m512i __C) {
  return (__m512i)__builtin_ia32_vpdpwusd512((__v16si)__A, (__v16si)__B,
                                             (__v16si)__C);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_dpwusd_epi32(__m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwusd_epi32(__A, __B, __C),
      (__v16si)__A);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpwusd_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwusd_epi32(__A, __B, __C),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpwusds_epi32(__m512i __A,
                                                                  __m512i __B,
                                                                  __m512i __C) {
  return (__m512i)__builtin_ia32_vpdpwusds512((__v16si)__A, (__v16si)__B,
                                              (__v16si)__C);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_mask_dpwusds_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwusds_epi32(__A, __B, __C),
      (__v16si)__A);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpwusds_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwusds_epi32(__A, __B, __C),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpwuud_epi32(__m512i __A,
                                                                 __m512i __B,
                                                                 __m512i __C) {
  return (__m512i)__builtin_ia32_vpdpwuud512((__v16si)__A, (__v16si)__B,
                                             (__v16si)__C);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS
_mm512_mask_dpwuud_epi32(__m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwuud_epi32(__A, __B, __C),
      (__v16si)__A);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpwuud_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwuud_epi32(__A, __B, __C),
      (__v16si)_mm512_setzero_si512());
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_dpwuuds_epi32(__m512i __A,
                                                                  __m512i __B,
                                                                  __m512i __C) {
  return (__m512i)__builtin_ia32_vpdpwuuds512((__v16si)__A, (__v16si)__B,
                                              (__v16si)__C);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_mask_dpwuuds_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwuuds_epi32(__A, __B, __C),
      (__v16si)__A);
}

static __inline__ __m512i __DEFAULT_FN_ATTRS _mm512_maskz_dpwuuds_epi32(
    __m512i __A, __mmask16 __U, __m512i __B, __m512i __C) {
  return (__m512i)__builtin_ia32_selectd_512(
      (__mmask16)__U, (__v16si)_mm512_dpwuuds_epi32(__A, __B, __C),
      (__v16si)_mm512_setzero_si512());
}

#undef __DEFAULT_FN_ATTRS

#endif /* __SSE2__ */
#endif /* __AVX10_2_512NIINTRIN_H */
