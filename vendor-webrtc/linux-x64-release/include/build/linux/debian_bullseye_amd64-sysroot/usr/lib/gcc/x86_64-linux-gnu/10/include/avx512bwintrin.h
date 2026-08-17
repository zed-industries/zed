/* Copyright (C) 2014-2020 Free Software Foundation, Inc.

   This file is part of GCC.

   GCC is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   GCC is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   Under Section 7 of GPL version 3, you are granted additional
   permissions described in the GCC Runtime Library Exception, version
   3.1, as published by the Free Software Foundation.

   You should have received a copy of the GNU General Public License and
   a copy of the GCC Runtime Library Exception along with this program;
   see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
   <http://www.gnu.org/licenses/>.  */

#ifndef _IMMINTRIN_H_INCLUDED
#error "Never use <avx512bwintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512BWINTRIN_H_INCLUDED
#define _AVX512BWINTRIN_H_INCLUDED

#ifndef __AVX512BW__
#pragma GCC push_options
#pragma GCC target("avx512bw")
#define __DISABLE_AVX512BW__
#endif /* __AVX512BW__ */

/* Internal data types for implementing the intrinsics.  */
typedef short __v32hi __attribute__ ((__vector_size__ (64)));
typedef char __v64qi __attribute__ ((__vector_size__ (64)));

typedef unsigned long long __mmask64;

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktest_mask32_u8  (__mmask32 __A,  __mmask32 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_ktestcsi (__A, __B);
  return (unsigned char) __builtin_ia32_ktestzsi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktest_mask64_u8  (__mmask64 __A,  __mmask64 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_ktestcdi (__A, __B);
  return (unsigned char) __builtin_ia32_ktestzdi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestz_mask32_u8 (__mmask32 __A, __mmask32 __B)
{
  return (unsigned char) __builtin_ia32_ktestzsi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestz_mask64_u8 (__mmask64 __A, __mmask64 __B)
{
  return (unsigned char) __builtin_ia32_ktestzdi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestc_mask32_u8 (__mmask32 __A, __mmask32 __B)
{
  return (unsigned char) __builtin_ia32_ktestcsi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestc_mask64_u8 (__mmask64 __A, __mmask64 __B)
{
  return (unsigned char) __builtin_ia32_ktestcdi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortest_mask32_u8  (__mmask32 __A,  __mmask32 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_kortestcsi (__A, __B);
  return (unsigned char) __builtin_ia32_kortestzsi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortest_mask64_u8  (__mmask64 __A,  __mmask64 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_kortestcdi (__A, __B);
  return (unsigned char) __builtin_ia32_kortestzdi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestz_mask32_u8 (__mmask32 __A, __mmask32 __B)
{
  return (unsigned char) __builtin_ia32_kortestzsi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestz_mask64_u8 (__mmask64 __A, __mmask64 __B)
{
  return (unsigned char) __builtin_ia32_kortestzdi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestc_mask32_u8 (__mmask32 __A, __mmask32 __B)
{
  return (unsigned char) __builtin_ia32_kortestcsi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestc_mask64_u8 (__mmask64 __A, __mmask64 __B)
{
  return (unsigned char) __builtin_ia32_kortestcdi (__A, __B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kadd_mask32 (__mmask32 __A, __mmask32 __B)
{
  return (__mmask32) __builtin_ia32_kaddsi ((__mmask32) __A, (__mmask32) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kadd_mask64 (__mmask64 __A, __mmask64 __B)
{
  return (__mmask64) __builtin_ia32_kadddi ((__mmask64) __A, (__mmask64) __B);
}

extern __inline unsigned int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtmask32_u32 (__mmask32 __A)
{
  return (unsigned int) __builtin_ia32_kmovd ((__mmask32) __A);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtmask64_u64 (__mmask64 __A)
{
  return (unsigned long long) __builtin_ia32_kmovq ((__mmask64) __A);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtu32_mask32 (unsigned int __A)
{
  return (__mmask32) __builtin_ia32_kmovd ((__mmask32) __A);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtu64_mask64 (unsigned long long __A)
{
  return (__mmask64) __builtin_ia32_kmovq ((__mmask64) __A);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_load_mask32 (__mmask32 *__A)
{
  return (__mmask32) __builtin_ia32_kmovd (*__A);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_load_mask64 (__mmask64 *__A)
{
  return (__mmask64) __builtin_ia32_kmovq (*(__mmask64 *) __A);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_store_mask32 (__mmask32 *__A, __mmask32 __B)
{
  *(__mmask32 *) __A = __builtin_ia32_kmovd (__B);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_store_mask64 (__mmask64 *__A, __mmask64 __B)
{
  *(__mmask64 *) __A = __builtin_ia32_kmovq (__B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_knot_mask32 (__mmask32 __A)
{
  return (__mmask32) __builtin_ia32_knotsi ((__mmask32) __A);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_knot_mask64 (__mmask64 __A)
{
  return (__mmask64) __builtin_ia32_knotdi ((__mmask64) __A);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kor_mask32 (__mmask32 __A, __mmask32 __B)
{
  return (__mmask32) __builtin_ia32_korsi ((__mmask32) __A, (__mmask32) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kor_mask64 (__mmask64 __A, __mmask64 __B)
{
  return (__mmask64) __builtin_ia32_kordi ((__mmask64) __A, (__mmask64) __B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kxnor_mask32 (__mmask32 __A, __mmask32 __B)
{
  return (__mmask32) __builtin_ia32_kxnorsi ((__mmask32) __A, (__mmask32) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kxnor_mask64 (__mmask64 __A, __mmask64 __B)
{
  return (__mmask64) __builtin_ia32_kxnordi ((__mmask64) __A, (__mmask64) __B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kxor_mask32 (__mmask32 __A, __mmask32 __B)
{
  return (__mmask32) __builtin_ia32_kxorsi ((__mmask32) __A, (__mmask32) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kxor_mask64 (__mmask64 __A, __mmask64 __B)
{
  return (__mmask64) __builtin_ia32_kxordi ((__mmask64) __A, (__mmask64) __B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kand_mask32 (__mmask32 __A, __mmask32 __B)
{
  return (__mmask32) __builtin_ia32_kandsi ((__mmask32) __A, (__mmask32) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kand_mask64 (__mmask64 __A, __mmask64 __B)
{
  return (__mmask64) __builtin_ia32_kanddi ((__mmask64) __A, (__mmask64) __B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kandn_mask32 (__mmask32 __A, __mmask32 __B)
{
  return (__mmask32) __builtin_ia32_kandnsi ((__mmask32) __A, (__mmask32) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kandn_mask64 (__mmask64 __A, __mmask64 __B)
{
  return (__mmask64) __builtin_ia32_kandndi ((__mmask64) __A, (__mmask64) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mov_epi16 (__m512i __W, __mmask32 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdquhi512_mask ((__v32hi) __A,
						    (__v32hi) __W,
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mov_epi16 (__mmask32 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdquhi512_mask ((__v32hi) __A,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_loadu_epi16 (__m512i __W, __mmask32 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddquhi512_mask ((const short *) __P,
						     (__v32hi) __W,
						     (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_loadu_epi16 (__mmask32 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddquhi512_mask ((const short *) __P,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_storeu_epi16 (void *__P, __mmask32 __U, __m512i __A)
{
  __builtin_ia32_storedquhi512_mask ((short *) __P,
				     (__v32hi) __A,
				     (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mov_epi8 (__m512i __W, __mmask64 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdquqi512_mask ((__v64qi) __A,
						    (__v64qi) __W,
						    (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mov_epi8 (__mmask64 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdquqi512_mask ((__v64qi) __A,
						    (__v64qi)
						    _mm512_setzero_si512 (),
						    (__mmask64) __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kunpackw (__mmask32 __A, __mmask32 __B)
{
  return (__mmask32) __builtin_ia32_kunpcksi ((__mmask32) __A,
					      (__mmask32) __B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kunpackw_mask32 (__mmask16 __A, __mmask16 __B)
{
  return (__mmask32) __builtin_ia32_kunpcksi ((__mmask32) __A,
					      (__mmask32) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kunpackd (__mmask64 __A, __mmask64 __B)
{
  return (__mmask64) __builtin_ia32_kunpckdi ((__mmask64) __A,
					      (__mmask64) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kunpackd_mask64 (__mmask32 __A, __mmask32 __B)
{
  return (__mmask64) __builtin_ia32_kunpckdi ((__mmask64) __A,
					      (__mmask64) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_loadu_epi8 (__m512i __W, __mmask64 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddquqi512_mask ((const char *) __P,
						     (__v64qi) __W,
						     (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_loadu_epi8 (__mmask64 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddquqi512_mask ((const char *) __P,
						     (__v64qi)
						     _mm512_setzero_si512 (),
						     (__mmask64) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_storeu_epi8 (void *__P, __mmask64 __U, __m512i __A)
{
  __builtin_ia32_storedquqi512_mask ((char *) __P,
				     (__v64qi) __A,
				     (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sad_epu8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psadbw512 ((__v64qi) __A,
					     (__v64qi) __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi16_epi8 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovwb512_mask ((__v32hi) __A,
						  (__v32qi) _mm256_undefined_si256(),
						  (__mmask32) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi16_storeu_epi8 (void * __P, __mmask32 __M, __m512i __A)
{
  __builtin_ia32_pmovwb512mem_mask ((__v32qi *) __P, (__v32hi) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi16_epi8 (__m256i __O, __mmask32 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovwb512_mask ((__v32hi) __A,
						  (__v32qi) __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi16_epi8 (__mmask32 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovwb512_mask ((__v32hi) __A,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtsepi16_epi8 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovswb512_mask ((__v32hi) __A,
						   (__v32qi)_mm256_undefined_si256(),
						   (__mmask32) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi16_storeu_epi8 (void * __P, __mmask32 __M, __m512i __A)
{
  __builtin_ia32_pmovswb512mem_mask ((__v32qi *) __P, (__v32hi) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi16_epi8 (__m256i __O, __mmask32 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovswb512_mask ((__v32hi) __A,
						   (__v32qi)__O,
						   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtsepi16_epi8 (__mmask32 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovswb512_mask ((__v32hi) __A,
						   (__v32qi)
						   _mm256_setzero_si256 (),
						   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtusepi16_epi8 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovuswb512_mask ((__v32hi) __A,
						    (__v32qi)_mm256_undefined_si256(),
						    (__mmask32) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi16_epi8 (__m256i __O, __mmask32 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovuswb512_mask ((__v32hi) __A,
						    (__v32qi) __O,
						    __M);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi16_storeu_epi8 (void * __P, __mmask32 __M, __m512i __A)
{
  __builtin_ia32_pmovuswb512mem_mask ((__v32qi *) __P, (__v32hi) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtusepi16_epi8 (__mmask32 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovuswb512_mask ((__v32hi) __A,
						    (__v32qi)
						    _mm256_setzero_si256 (),
						    __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastb_epi8 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastb512_mask ((__v16qi) __A,
						       (__v64qi)_mm512_undefined_epi32(),
						       (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcastb_epi8 (__m512i __O, __mmask64 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastb512_mask ((__v16qi) __A,
						       (__v64qi) __O,
						       __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcastb_epi8 (__mmask64 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastb512_mask ((__v16qi) __A,
						       (__v64qi)
						       _mm512_setzero_si512 (),
						       __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_set1_epi8 (__m512i __O, __mmask64 __M, char __A)
{
  return (__m512i) __builtin_ia32_pbroadcastb512_gpr_mask (__A,
							   (__v64qi) __O,
							   __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_set1_epi8 (__mmask64 __M, char __A)
{
  return (__m512i)
	 __builtin_ia32_pbroadcastb512_gpr_mask (__A,
						 (__v64qi)
						 _mm512_setzero_si512 (),
						 __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastw_epi16 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastw512_mask ((__v8hi) __A,
						       (__v32hi)_mm512_undefined_epi32(),
						       (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcastw_epi16 (__m512i __O, __mmask32 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastw512_mask ((__v8hi) __A,
						       (__v32hi) __O,
						       __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcastw_epi16 (__mmask32 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastw512_mask ((__v8hi) __A,
						       (__v32hi)
						       _mm512_setzero_si512 (),
						       __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_set1_epi16 (__m512i __O, __mmask32 __M, short __A)
{
  return (__m512i) __builtin_ia32_pbroadcastw512_gpr_mask (__A,
							   (__v32hi) __O,
							   __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_set1_epi16 (__mmask32 __M, short __A)
{
  return (__m512i)
	 __builtin_ia32_pbroadcastw512_gpr_mask (__A,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mulhrs_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhrsw512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mulhrs_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			  __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhrsw512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v32hi) __W,
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mulhrs_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhrsw512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mulhi_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mulhi_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			 __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mulhi_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mulhi_epu16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhuw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mulhi_epu16 (__m512i __W, __mmask32 __U, __m512i __A,
			 __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhuw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi) __W,
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mulhi_epu16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulhuw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mullo_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v32hu) __A * (__v32hu) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mullo_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			 __m512i __B)
{
  return (__m512i) __builtin_ia32_pmullw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mullo_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmullw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi8_epi16 (__m256i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbw512_mask ((__v32qi) __A,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi8_epi16 (__m512i __W, __mmask32 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbw512_mask ((__v32qi) __A,
						    (__v32hi) __W,
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi8_epi16 (__mmask32 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbw512_mask ((__v32qi) __A,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu8_epi16 (__m256i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbw512_mask ((__v32qi) __A,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu8_epi16 (__m512i __W, __mmask32 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbw512_mask ((__v32qi) __A,
						    (__v32hi) __W,
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu8_epi16 (__mmask32 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbw512_mask ((__v32qi) __A,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutexvar_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_permvarhi512_mask ((__v32hi) __B,
						     (__v32hi) __A,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutexvar_epi16 (__mmask32 __M, __m512i __A,
				__m512i __B)
{
  return (__m512i) __builtin_ia32_permvarhi512_mask ((__v32hi) __B,
						     (__v32hi) __A,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutexvar_epi16 (__m512i __W, __mmask32 __M, __m512i __A,
			       __m512i __B)
{
  return (__m512i) __builtin_ia32_permvarhi512_mask ((__v32hi) __B,
						     (__v32hi) __A,
						     (__v32hi) __W,
						     (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex2var_epi16 (__m512i __A, __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varhi512_mask ((__v32hi) __I
							/* idx */ ,
							(__v32hi) __A,
							(__v32hi) __B,
							(__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex2var_epi16 (__m512i __A, __mmask32 __U,
				__m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varhi512_mask ((__v32hi) __I
							/* idx */ ,
							(__v32hi) __A,
							(__v32hi) __B,
							(__mmask32)
							__U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask2_permutex2var_epi16 (__m512i __A, __m512i __I,
				 __mmask32 __U, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermi2varhi512_mask ((__v32hi) __A,
							(__v32hi) __I
							/* idx */ ,
							(__v32hi) __B,
							(__mmask32)
							__U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex2var_epi16 (__mmask32 __U, __m512i __A,
				 __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varhi512_maskz ((__v32hi) __I
							 /* idx */ ,
							 (__v32hi) __A,
							 (__v32hi) __B,
							 (__mmask32)
							 __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_avg_epu8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pavgb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__v64qi)
						 _mm512_setzero_si512 (),
						 (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_avg_epu8 (__m512i __W, __mmask64 __U, __m512i __A,
		      __m512i __B)
{
  return (__m512i) __builtin_ia32_pavgb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__v64qi) __W,
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_avg_epu8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pavgb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__v64qi)
						 _mm512_setzero_si512 (),
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v64qu) __A + (__v64qu) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
		      __m512i __B)
{
  return (__m512i) __builtin_ia32_paddb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__v64qi) __W,
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_epi8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__v64qi)
						 _mm512_setzero_si512 (),
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v64qu) __A - (__v64qu) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
		      __m512i __B)
{
  return (__m512i) __builtin_ia32_psubb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__v64qi) __W,
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_epi8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__v64qi)
						 _mm512_setzero_si512 (),
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_avg_epu16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pavgw512_mask ((__v32hi) __A,
						 (__v32hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_avg_epu16 (__m512i __W, __mmask32 __U, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_pavgw512_mask ((__v32hi) __A,
						 (__v32hi) __B,
						 (__v32hi) __W,
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_avg_epu16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pavgw512_mask ((__v32hi) __A,
						 (__v32hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_subs_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_subs_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_psubsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi) __W,
						  (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_subs_epi8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_subs_epu8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubusb512_mask ((__v64qi) __A,
						   (__v64qi) __B,
						   (__v64qi)
						   _mm512_setzero_si512 (),
						   (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_subs_epu8 (__m512i __W, __mmask64 __U, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_psubusb512_mask ((__v64qi) __A,
						   (__v64qi) __B,
						   (__v64qi) __W,
						   (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_subs_epu8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubusb512_mask ((__v64qi) __A,
						   (__v64qi) __B,
						   (__v64qi)
						   _mm512_setzero_si512 (),
						   (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_adds_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_adds_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_paddsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi) __W,
						  (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_adds_epi8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_adds_epu8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddusb512_mask ((__v64qi) __A,
						   (__v64qi) __B,
						   (__v64qi)
						   _mm512_setzero_si512 (),
						   (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_adds_epu8 (__m512i __W, __mmask64 __U, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_paddusb512_mask ((__v64qi) __A,
						   (__v64qi) __B,
						   (__v64qi) __W,
						   (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_adds_epu8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddusb512_mask ((__v64qi) __A,
						   (__v64qi) __B,
						   (__v64qi)
						   _mm512_setzero_si512 (),
						   (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v32hu) __A - (__v32hu) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_psubw512_mask ((__v32hi) __A,
						 (__v32hi) __B,
						 (__v32hi) __W,
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubw512_mask ((__v32hi) __A,
						 (__v32hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_subs_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_subs_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_psubsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_subs_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_subs_epu16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubusw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_subs_epu16 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_psubusw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi) __W,
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_subs_epu16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubusw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v32hu) __A + (__v32hu) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_paddw512_mask ((__v32hi) __A,
						 (__v32hi) __B,
						 (__v32hi) __W,
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddw512_mask ((__v32hi) __A,
						 (__v32hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_adds_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_adds_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_paddsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_adds_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_adds_epu16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddusw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_adds_epu16 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_paddusw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi) __W,
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_adds_epu16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddusw512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srl_epi16 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrlw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srl_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
		       __m128i __B)
{
  return (__m512i) __builtin_ia32_psrlw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi) __W,
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srl_epi16 (__mmask32 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrlw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_packs_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packsswb512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v64qi)
						    _mm512_setzero_si512 (),
						    (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sll_epi16 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psllw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sll_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
		       __m128i __B)
{
  return (__m512i) __builtin_ia32_psllw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi) __W,
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sll_epi16 (__mmask32 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psllw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maddubs_epi16 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmaddubsw512_mask ((__v64qi) __X,
						     (__v64qi) __Y,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_maddubs_epi16 (__m512i __W, __mmask32 __U, __m512i __X,
			   __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmaddubsw512_mask ((__v64qi) __X,
						     (__v64qi) __Y,
						     (__v32hi) __W,
						     (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_maddubs_epi16 (__mmask32 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmaddubsw512_mask ((__v64qi) __X,
						     (__v64qi) __Y,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_madd_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaddwd512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v16si)
						   _mm512_setzero_si512 (),
						   (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_madd_epi16 (__m512i __W, __mmask16 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_pmaddwd512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v16si) __W,
						   (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_madd_epi16 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaddwd512_mask ((__v32hi) __A,
						   (__v32hi) __B,
						   (__v16si)
						   _mm512_setzero_si512 (),
						   (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpackhi_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhbw512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__v64qi)
						     _mm512_setzero_si512 (),
						     (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpackhi_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
			   __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhbw512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__v64qi) __W,
						     (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpackhi_epi8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhbw512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__v64qi)
						     _mm512_setzero_si512 (),
						     (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpackhi_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhwd512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpackhi_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			    __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhwd512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__v32hi) __W,
						     (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpackhi_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhwd512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpacklo_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklbw512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__v64qi)
						     _mm512_setzero_si512 (),
						     (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpacklo_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
			   __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklbw512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__v64qi) __W,
						     (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpacklo_epi8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklbw512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__v64qi)
						     _mm512_setzero_si512 (),
						     (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpacklo_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklwd512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpacklo_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			    __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklwd512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__v32hi) __W,
						     (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpacklo_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklwd512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__v32hi)
						     _mm512_setzero_si512 (),
						     (__mmask32) __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epu8_mask (__m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __A,
						    (__v64qi) __B, 0,
						    (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epi8_mask (__m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_pcmpeqb512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epu8_mask (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __A,
						    (__v64qi) __B, 0,
						    __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epi8_mask (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_pcmpeqb512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epu16_mask (__m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __A,
						    (__v32hi) __B, 0,
						    (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epi16_mask (__m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_pcmpeqw512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epu16_mask (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __A,
						    (__v32hi) __B, 0,
						    __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epi16_mask (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_pcmpeqw512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epu8_mask (__m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __A,
						    (__v64qi) __B, 6,
						    (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epi8_mask (__m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_pcmpgtb512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epu8_mask (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __A,
						    (__v64qi) __B, 6,
						    __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epi8_mask (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_pcmpgtb512_mask ((__v64qi) __A,
						     (__v64qi) __B,
						     __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epu16_mask (__m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __A,
						    (__v32hi) __B, 6,
						    (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epi16_mask (__m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_pcmpgtw512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epu16_mask (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __A,
						    (__v32hi) __B, 6,
						    __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epi16_mask (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_pcmpgtw512_mask ((__v32hi) __A,
						     (__v32hi) __B,
						     __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movepi8_mask (__m512i __A)
{
  return (__mmask64) __builtin_ia32_cvtb2mask512 ((__v64qi) __A);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movepi16_mask (__m512i __A)
{
  return (__mmask32) __builtin_ia32_cvtw2mask512 ((__v32hi) __A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movm_epi8 (__mmask64 __A)
{
  return (__m512i) __builtin_ia32_cvtmask2b512 (__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movm_epi16 (__mmask32 __A)
{
  return (__m512i) __builtin_ia32_cvtmask2w512 (__A);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_test_epi8_mask (__m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ptestmb512 ((__v64qi) __A,
						(__v64qi) __B,
						(__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_test_epi8_mask (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ptestmb512 ((__v64qi) __A,
						(__v64qi) __B, __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_test_epi16_mask (__m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ptestmw512 ((__v32hi) __A,
						(__v32hi) __B,
						(__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_test_epi16_mask (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ptestmw512 ((__v32hi) __A,
						(__v32hi) __B, __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_testn_epi8_mask (__m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ptestnmb512 ((__v64qi) __A,
						 (__v64qi) __B,
						 (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_testn_epi8_mask (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_ptestnmb512 ((__v64qi) __A,
						 (__v64qi) __B, __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_testn_epi16_mask (__m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ptestnmw512 ((__v32hi) __A,
						 (__v32hi) __B,
						 (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_testn_epi16_mask (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__mmask32) __builtin_ia32_ptestnmw512 ((__v32hi) __A,
						 (__v32hi) __B, __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pshufb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
			  __m512i __B)
{
  return (__m512i) __builtin_ia32_pshufb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi) __W,
						  (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_epi8 (__mmask64 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pshufb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epu16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminuw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epu16 (__mmask32 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminuw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epu16 (__m512i __W, __mmask32 __M, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_pminuw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epi16 (__mmask32 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epi16 (__m512i __W, __mmask32 __M, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epu8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxub512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epu8 (__mmask64 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxub512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epu8 (__m512i __W, __mmask64 __M, __m512i __A,
		      __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxub512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi) __W,
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epi8 (__mmask64 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epi8 (__m512i __W, __mmask64 __M, __m512i __A,
		      __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi) __W,
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epu8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminub512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epu8 (__mmask64 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminub512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epu8 (__m512i __W, __mmask64 __M, __m512i __A,
		      __m512i __B)
{
  return (__m512i) __builtin_ia32_pminub512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi) __W,
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epi8 (__mmask64 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi)
						  _mm512_setzero_si512 (),
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epi8 (__m512i __W, __mmask64 __M, __m512i __A,
		      __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsb512_mask ((__v64qi) __A,
						  (__v64qi) __B,
						  (__v64qi) __W,
						  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epi16 (__mmask32 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epi16 (__m512i __W, __mmask32 __M, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epu16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxuw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epu16 (__mmask32 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxuw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epu16 (__m512i __W, __mmask32 __M, __m512i __A,
		       __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxuw512_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sra_epi16 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psraw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sra_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
		       __m128i __B)
{
  return (__m512i) __builtin_ia32_psraw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi) __W,
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sra_epi16 (__mmask32 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psraw512_mask ((__v32hi) __A,
						 (__v8hi) __B,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srav_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psrav32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srav_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_psrav32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srav_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psrav32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srlv_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psrlv32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srlv_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_psrlv32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srlv_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psrlv32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sllv_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psllv32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sllv_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B)
{
  return (__m512i) __builtin_ia32_psllv32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sllv_epi16 (__mmask32 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psllv32hi_mask ((__v32hi) __A,
						  (__v32hi) __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_packs_epi16 (__m512i __W, __mmask64 __M, __m512i __A,
			 __m512i __B)
{
  return (__m512i) __builtin_ia32_packsswb512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v64qi) __W,
						    (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_packs_epi16 (__mmask64 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packsswb512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v64qi)
						    _mm512_setzero_si512 (),
						    __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_packus_epi16 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packuswb512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v64qi)
						    _mm512_setzero_si512 (),
						    (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_packus_epi16 (__m512i __W, __mmask64 __M, __m512i __A,
			  __m512i __B)
{
  return (__m512i) __builtin_ia32_packuswb512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v64qi) __W,
						    (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_packus_epi16 (__mmask64 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packuswb512_mask ((__v32hi) __A,
						    (__v32hi) __B,
						    (__v64qi)
						    _mm512_setzero_si512 (),
						    (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_abs_epi8 (__m512i __A)
{
  return (__m512i) __builtin_ia32_pabsb512_mask ((__v64qi) __A,
						 (__v64qi)
						 _mm512_setzero_si512 (),
						 (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_abs_epi8 (__m512i __W, __mmask64 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsb512_mask ((__v64qi) __A,
						 (__v64qi) __W,
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_abs_epi8 (__mmask64 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsb512_mask ((__v64qi) __A,
						 (__v64qi)
						 _mm512_setzero_si512 (),
						 (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_abs_epi16 (__m512i __A)
{
  return (__m512i) __builtin_ia32_pabsw512_mask ((__v32hi) __A,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_abs_epi16 (__m512i __W, __mmask32 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsw512_mask ((__v32hi) __A,
						 (__v32hi) __W,
						 (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_abs_epi16 (__mmask32 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsw512_mask ((__v32hi) __A,
						 (__v32hi)
						 _mm512_setzero_si512 (),
						 (__mmask32) __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epu8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 4,
						   (__mmask64) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epu8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 1,
						   (__mmask64) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epu8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 5,
						   (__mmask64) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epu8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 2,
						   (__mmask64) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epu16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 4,
						   (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epu16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 1,
						   (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epu16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 5,
						   (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epu16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 2,
						   (__mmask32) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epi8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 4,
						  (__mmask64) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epi8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 1,
						  (__mmask64) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epi8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 5,
						  (__mmask64) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epi8_mask (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 2,
						  (__mmask64) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epi16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 4,
						  (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epi16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 1,
						  (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epi16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 5,
						  (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epi16_mask (__mmask32 __M, __m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 2,
						  (__mmask32) __M);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epu8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 4,
						   (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epu8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 1,
						   (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epu8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 5,
						   (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epu8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, 2,
						   (__mmask64) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epu16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 4,
						   (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epu16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 1,
						   (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epu16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 5,
						   (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epu16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, 2,
						   (__mmask32) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epi8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 4,
						  (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epi8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 1,
						  (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epi8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 5,
						  (__mmask64) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epi8_mask (__m512i __X, __m512i __Y)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, 2,
						  (__mmask64) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epi16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 4,
						  (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epi16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 1,
						  (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epi16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 5,
						  (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epi16_mask (__m512i __X, __m512i __Y)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, 2,
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_packs_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packssdw512_mask ((__v16si) __A,
						    (__v16si) __B,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_packs_epi32 (__mmask32 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packssdw512_mask ((__v16si) __A,
						    (__v16si) __B,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_packs_epi32 (__m512i __W, __mmask32 __M, __m512i __A,
			 __m512i __B)
{
  return (__m512i) __builtin_ia32_packssdw512_mask ((__v16si) __A,
						    (__v16si) __B,
						    (__v32hi) __W,
						    __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_packus_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packusdw512_mask ((__v16si) __A,
						    (__v16si) __B,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_packus_epi32 (__mmask32 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_packusdw512_mask ((__v16si) __A,
						    (__v16si) __B,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_packus_epi32 (__m512i __W, __mmask32 __M, __m512i __A,
			  __m512i __B)
{
  return (__m512i) __builtin_ia32_packusdw512_mask ((__v16si) __A,
						    (__v16si) __B,
						    (__v32hi) __W,
						    __M);
}

#ifdef __OPTIMIZE__
extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftli_mask32 (__mmask32 __A, unsigned int __B)
{
  return (__mmask32) __builtin_ia32_kshiftlisi ((__mmask32) __A,
						(__mmask8) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftli_mask64 (__mmask64 __A, unsigned int __B)
{
  return (__mmask64) __builtin_ia32_kshiftlidi ((__mmask64) __A,
						(__mmask8) __B);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftri_mask32 (__mmask32 __A, unsigned int __B)
{
  return (__mmask32) __builtin_ia32_kshiftrisi ((__mmask32) __A,
						(__mmask8) __B);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftri_mask64 (__mmask64 __A, unsigned int __B)
{
  return (__mmask64) __builtin_ia32_kshiftridi ((__mmask64) __A,
						(__mmask8) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_alignr_epi8 (__m512i __A, __m512i __B, const int __N)
{
  return (__m512i) __builtin_ia32_palignr512 ((__v8di) __A,
					      (__v8di) __B, __N * 8);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_alignr_epi8 (__m512i __W, __mmask64 __U, __m512i __A,
			 __m512i __B, const int __N)
{
  return (__m512i) __builtin_ia32_palignr512_mask ((__v8di) __A,
						   (__v8di) __B,
						   __N * 8,
						   (__v8di) __W,
						   (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_alignr_epi8 (__mmask64 __U, __m512i __A, __m512i __B,
			  const int __N)
{
  return (__m512i) __builtin_ia32_palignr512_mask ((__v8di) __A,
						   (__v8di) __B,
						   __N * 8,
						   (__v8di)
						   _mm512_setzero_si512 (),
						   (__mmask64) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_dbsad_epu8 (__m512i __A, __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_dbpsadbw512_mask ((__v64qi) __A,
						    (__v64qi) __B,
						    __imm,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_dbsad_epu8 (__m512i __W, __mmask32 __U, __m512i __A,
			__m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_dbpsadbw512_mask ((__v64qi) __A,
						    (__v64qi) __B,
						    __imm,
						    (__v32hi) __W,
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_dbsad_epu8 (__mmask32 __U, __m512i __A, __m512i __B,
			 const int __imm)
{
  return (__m512i) __builtin_ia32_dbpsadbw512_mask ((__v64qi) __A,
						    (__v64qi) __B,
						    __imm,
						    (__v32hi)
						    _mm512_setzero_si512 (),
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srli_epi16 (__m512i __A, const int __imm)
{
  return (__m512i) __builtin_ia32_psrlwi512_mask ((__v32hi) __A, __imm,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srli_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			const int __imm)
{
  return (__m512i) __builtin_ia32_psrlwi512_mask ((__v32hi) __A, __imm,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srli_epi16 (__mmask32 __U, __m512i __A, const int __imm)
{
  return (__m512i) __builtin_ia32_psrlwi512_mask ((__v32hi) __A, __imm,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_slli_epi16 (__m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_psllwi512_mask ((__v32hi) __A, __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_slli_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			const int __B)
{
  return (__m512i) __builtin_ia32_psllwi512_mask ((__v32hi) __A, __B,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_slli_epi16 (__mmask32 __U, __m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_psllwi512_mask ((__v32hi) __A, __B,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shufflehi_epi16 (__m512i __A, const int __imm)
{
  return (__m512i) __builtin_ia32_pshufhw512_mask ((__v32hi) __A,
						   __imm,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shufflehi_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			     const int __imm)
{
  return (__m512i) __builtin_ia32_pshufhw512_mask ((__v32hi) __A,
						   __imm,
						   (__v32hi) __W,
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shufflehi_epi16 (__mmask32 __U, __m512i __A,
			      const int __imm)
{
  return (__m512i) __builtin_ia32_pshufhw512_mask ((__v32hi) __A,
						   __imm,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shufflelo_epi16 (__m512i __A, const int __imm)
{
  return (__m512i) __builtin_ia32_pshuflw512_mask ((__v32hi) __A,
						   __imm,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shufflelo_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			     const int __imm)
{
  return (__m512i) __builtin_ia32_pshuflw512_mask ((__v32hi) __A,
						   __imm,
						   (__v32hi) __W,
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shufflelo_epi16 (__mmask32 __U, __m512i __A,
			      const int __imm)
{
  return (__m512i) __builtin_ia32_pshuflw512_mask ((__v32hi) __A,
						   __imm,
						   (__v32hi)
						   _mm512_setzero_si512 (),
						   (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srai_epi16 (__m512i __A, const int __imm)
{
  return (__m512i) __builtin_ia32_psrawi512_mask ((__v32hi) __A, __imm,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srai_epi16 (__m512i __W, __mmask32 __U, __m512i __A,
			const int __imm)
{
  return (__m512i) __builtin_ia32_psrawi512_mask ((__v32hi) __A, __imm,
						  (__v32hi) __W,
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srai_epi16 (__mmask32 __U, __m512i __A, const int __imm)
{
  return (__m512i) __builtin_ia32_psrawi512_mask ((__v32hi) __A, __imm,
						  (__v32hi)
						  _mm512_setzero_si512 (),
						  (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_blend_epi16 (__mmask32 __U, __m512i __A, __m512i __W)
{
  return (__m512i) __builtin_ia32_blendmw_512_mask ((__v32hi) __A,
						    (__v32hi) __W,
						    (__mmask32) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_blend_epi8 (__mmask64 __U, __m512i __A, __m512i __W)
{
  return (__m512i) __builtin_ia32_blendmb_512_mask ((__v64qi) __A,
						    (__v64qi) __W,
						    (__mmask64) __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epi16_mask (__mmask32 __U, __m512i __X, __m512i __Y,
			    const int __P)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, __P,
						  (__mmask32) __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epi16_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi) __X,
						  (__v32hi) __Y, __P,
						  (__mmask32) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epi8_mask (__mmask64 __U, __m512i __X, __m512i __Y,
			   const int __P)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, __P,
						  (__mmask64) __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epi8_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi) __X,
						  (__v64qi) __Y, __P,
						  (__mmask64) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epu16_mask (__mmask32 __U, __m512i __X, __m512i __Y,
			    const int __P)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, __P,
						   (__mmask32) __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epu16_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi) __X,
						   (__v32hi) __Y, __P,
						   (__mmask32) -1);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epu8_mask (__mmask64 __U, __m512i __X, __m512i __Y,
			   const int __P)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, __P,
						   (__mmask64) __U);
}

extern __inline __mmask64
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epu8_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi) __X,
						   (__v64qi) __Y, __P,
						   (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_bslli_epi128 (__m512i __A, const int __N)
{
  return (__m512i) __builtin_ia32_pslldq512 (__A, __N * 8);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_bsrli_epi128 (__m512i __A, const int __N)
{
  return (__m512i) __builtin_ia32_psrldq512 (__A, __N * 8);
}

#else
#define _kshiftli_mask32(X, Y)							\
  ((__mmask32) __builtin_ia32_kshiftlisi ((__mmask32)(X), (__mmask8)(Y)))

#define _kshiftli_mask64(X, Y)							\
  ((__mmask64) __builtin_ia32_kshiftlidi ((__mmask64)(X), (__mmask8)(Y)))

#define _kshiftri_mask32(X, Y)							\
  ((__mmask32) __builtin_ia32_kshiftrisi ((__mmask32)(X), (__mmask8)(Y)))

#define _kshiftri_mask64(X, Y)							\
  ((__mmask64) __builtin_ia32_kshiftridi ((__mmask64)(X), (__mmask8)(Y)))

#define _mm512_alignr_epi8(X, Y, N)						    \
  ((__m512i) __builtin_ia32_palignr512 ((__v8di)(__m512i)(X),			    \
					(__v8di)(__m512i)(Y),			    \
					(int)((N) * 8)))

#define _mm512_mask_alignr_epi8(W, U, X, Y, N)					    \
  ((__m512i) __builtin_ia32_palignr512_mask ((__v8di)(__m512i)(X),		    \
					    (__v8di)(__m512i)(Y), (int)((N) * 8),   \
					    (__v8di)(__m512i)(W), (__mmask64)(U)))

#define _mm512_maskz_alignr_epi8(U, X, Y, N)					    \
  ((__m512i) __builtin_ia32_palignr512_mask ((__v8di)(__m512i)(X),		    \
					     (__v8di)(__m512i)(Y), (int)((N) * 8),  \
					     (__v8di)(__m512i)			    \
					     _mm512_setzero_si512 (),		    \
					     (__mmask64)(U)))

#define _mm512_dbsad_epu8(X, Y, C)                                                  \
  ((__m512i) __builtin_ia32_dbpsadbw512_mask ((__v64qi)(__m512i) (X),               \
                                              (__v64qi)(__m512i) (Y), (int) (C),    \
                                              (__v32hi)(__m512i)		    \
					      _mm512_setzero_si512 (),		    \
                                              (__mmask32)-1))

#define _mm512_mask_dbsad_epu8(W, U, X, Y, C)                                       \
  ((__m512i) __builtin_ia32_dbpsadbw512_mask ((__v64qi)(__m512i) (X),               \
                                              (__v64qi)(__m512i) (Y), (int) (C),    \
                                              (__v32hi)(__m512i)(W),                \
                                              (__mmask32)(U)))

#define _mm512_maskz_dbsad_epu8(U, X, Y, C)                                         \
  ((__m512i) __builtin_ia32_dbpsadbw512_mask ((__v64qi)(__m512i) (X),               \
                                              (__v64qi)(__m512i) (Y), (int) (C),    \
                                              (__v32hi)(__m512i)		    \
					      _mm512_setzero_si512 (),		    \
                                              (__mmask32)(U)))

#define _mm512_srli_epi16(A, B)                                         \
  ((__m512i) __builtin_ia32_psrlwi512_mask ((__v32hi)(__m512i)(A),      \
    (int)(B), (__v32hi)_mm512_setzero_si512 (), (__mmask32)-1))

#define _mm512_mask_srli_epi16(W, U, A, B)                              \
  ((__m512i) __builtin_ia32_psrlwi512_mask ((__v32hi)(__m512i)(A),      \
    (int)(B), (__v32hi)(__m512i)(W), (__mmask32)(U)))

#define _mm512_maskz_srli_epi16(U, A, B)                                \
  ((__m512i) __builtin_ia32_psrlwi512_mask ((__v32hi)(__m512i)(A),      \
    (int)(B), (__v32hi)_mm512_setzero_si512 (), (__mmask32)(U)))

#define _mm512_slli_epi16(X, C)						   \
  ((__m512i)__builtin_ia32_psllwi512_mask ((__v32hi)(__m512i)(X), (int)(C),\
    (__v32hi)(__m512i)_mm512_setzero_si512 (),				   \
    (__mmask32)-1))

#define _mm512_mask_slli_epi16(W, U, X, C)                                 \
  ((__m512i)__builtin_ia32_psllwi512_mask ((__v32hi)(__m512i)(X), (int)(C),\
    (__v32hi)(__m512i)(W),\
    (__mmask32)(U)))

#define _mm512_maskz_slli_epi16(U, X, C)                                   \
  ((__m512i)__builtin_ia32_psllwi512_mask ((__v32hi)(__m512i)(X), (int)(C),\
    (__v32hi)(__m512i)_mm512_setzero_si512 (),				   \
    (__mmask32)(U)))

#define _mm512_shufflehi_epi16(A, B)                                                \
  ((__m512i) __builtin_ia32_pshufhw512_mask ((__v32hi)(__m512i)(A), (int)(B),       \
                                             (__v32hi)(__m512i)			    \
					     _mm512_setzero_si512 (),		    \
                                             (__mmask32)-1))

#define _mm512_mask_shufflehi_epi16(W, U, A, B)                                     \
  ((__m512i) __builtin_ia32_pshufhw512_mask ((__v32hi)(__m512i)(A), (int)(B),       \
                                             (__v32hi)(__m512i)(W),                 \
                                             (__mmask32)(U)))

#define _mm512_maskz_shufflehi_epi16(U, A, B)                                       \
  ((__m512i) __builtin_ia32_pshufhw512_mask ((__v32hi)(__m512i)(A), (int)(B),       \
                                             (__v32hi)(__m512i)			    \
					     _mm512_setzero_si512 (),		    \
                                             (__mmask32)(U)))

#define _mm512_shufflelo_epi16(A, B)                                                \
  ((__m512i) __builtin_ia32_pshuflw512_mask ((__v32hi)(__m512i)(A), (int)(B),       \
                                             (__v32hi)(__m512i)			    \
					     _mm512_setzero_si512 (),		    \
                                             (__mmask32)-1))

#define _mm512_mask_shufflelo_epi16(W, U, A, B)                                     \
  ((__m512i) __builtin_ia32_pshuflw512_mask ((__v32hi)(__m512i)(A), (int)(B),       \
                                             (__v32hi)(__m512i)(W),                 \
                                             (__mmask32)(U)))

#define _mm512_maskz_shufflelo_epi16(U, A, B)                                       \
  ((__m512i) __builtin_ia32_pshuflw512_mask ((__v32hi)(__m512i)(A), (int)(B),       \
                                             (__v32hi)(__m512i)			    \
					     _mm512_setzero_si512 (),		    \
                                             (__mmask32)(U)))

#define _mm512_srai_epi16(A, B)                                         \
  ((__m512i) __builtin_ia32_psrawi512_mask ((__v32hi)(__m512i)(A),      \
    (int)(B), (__v32hi)_mm512_setzero_si512 (), (__mmask32)-1))

#define _mm512_mask_srai_epi16(W, U, A, B)                              \
  ((__m512i) __builtin_ia32_psrawi512_mask ((__v32hi)(__m512i)(A),      \
    (int)(B), (__v32hi)(__m512i)(W), (__mmask32)(U)))

#define _mm512_maskz_srai_epi16(U, A, B)                                \
  ((__m512i) __builtin_ia32_psrawi512_mask ((__v32hi)(__m512i)(A),      \
    (int)(B), (__v32hi)_mm512_setzero_si512 (), (__mmask32)(U)))

#define _mm512_mask_blend_epi16(__U, __A, __W)			      \
  ((__m512i) __builtin_ia32_blendmw_512_mask ((__v32hi) (__A),	      \
						    (__v32hi) (__W),  \
						    (__mmask32) (__U)))

#define _mm512_mask_blend_epi8(__U, __A, __W)			      \
  ((__m512i) __builtin_ia32_blendmb_512_mask ((__v64qi) (__A),	      \
						    (__v64qi) (__W),  \
						    (__mmask64) (__U)))

#define _mm512_cmp_epi16_mask(X, Y, P)				\
  ((__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi)(__m512i)(X),	\
					    (__v32hi)(__m512i)(Y), (int)(P),\
					    (__mmask32)(-1)))

#define _mm512_cmp_epi8_mask(X, Y, P)				\
  ((__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi)(__m512i)(X),	\
					    (__v64qi)(__m512i)(Y), (int)(P),\
					    (__mmask64)(-1)))

#define _mm512_cmp_epu16_mask(X, Y, P)				\
  ((__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi)(__m512i)(X),	\
					    (__v32hi)(__m512i)(Y), (int)(P),\
					    (__mmask32)(-1)))

#define _mm512_cmp_epu8_mask(X, Y, P)				\
  ((__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi)(__m512i)(X),	\
					    (__v64qi)(__m512i)(Y), (int)(P),\
					    (__mmask64)(-1)))

#define _mm512_mask_cmp_epi16_mask(M, X, Y, P)				\
  ((__mmask32) __builtin_ia32_cmpw512_mask ((__v32hi)(__m512i)(X),	\
					    (__v32hi)(__m512i)(Y), (int)(P),\
					    (__mmask32)(M)))

#define _mm512_mask_cmp_epi8_mask(M, X, Y, P)				\
  ((__mmask64) __builtin_ia32_cmpb512_mask ((__v64qi)(__m512i)(X),	\
					    (__v64qi)(__m512i)(Y), (int)(P),\
					    (__mmask64)(M)))

#define _mm512_mask_cmp_epu16_mask(M, X, Y, P)				\
  ((__mmask32) __builtin_ia32_ucmpw512_mask ((__v32hi)(__m512i)(X),	\
					    (__v32hi)(__m512i)(Y), (int)(P),\
					    (__mmask32)(M)))

#define _mm512_mask_cmp_epu8_mask(M, X, Y, P)				\
  ((__mmask64) __builtin_ia32_ucmpb512_mask ((__v64qi)(__m512i)(X),	\
					    (__v64qi)(__m512i)(Y), (int)(P),\
					    (__mmask64)(M)))

#define _mm512_bslli_epi128(A, N)                                         \
  ((__m512i)__builtin_ia32_pslldq512 ((__m512i)(A), (int)(N) * 8))

#define _mm512_bsrli_epi128(A, N)                                         \
  ((__m512i)__builtin_ia32_psrldq512 ((__m512i)(A), (int)(N) * 8))

#endif

#ifdef __DISABLE_AVX512BW__
#undef __DISABLE_AVX512BW__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512BW__ */

#endif /* _AVX512BWINTRIN_H_INCLUDED */
