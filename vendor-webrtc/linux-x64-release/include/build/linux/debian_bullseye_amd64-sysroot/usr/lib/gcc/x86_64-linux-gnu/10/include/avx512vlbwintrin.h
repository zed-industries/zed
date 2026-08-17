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
#error "Never use <avx512vlbwintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512VLBWINTRIN_H_INCLUDED
#define _AVX512VLBWINTRIN_H_INCLUDED

#if !defined(__AVX512VL__) || !defined(__AVX512BW__)
#pragma GCC push_options
#pragma GCC target("avx512vl,avx512bw")
#define __DISABLE_AVX512VLBW__
#endif /* __AVX512VLBW__ */


extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mov_epi8 (__m256i __W, __mmask32 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdquqi256_mask ((__v32qi) __A,
						    (__v32qi) __W,
						    (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mov_epi8 (__mmask32 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdquqi256_mask ((__v32qi) __A,
						    (__v32qi)
						    _mm256_setzero_si256 (),
						    (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mov_epi8 (__m128i __W, __mmask16 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdquqi128_mask ((__v16qi) __A,
						    (__v16qi) __W,
						    (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mov_epi8 (__mmask16 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdquqi128_mask ((__v16qi) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    (__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_storeu_epi8 (void *__P, __mmask32 __U, __m256i __A)
{
  __builtin_ia32_storedquqi256_mask ((char *) __P,
				     (__v32qi) __A,
				     (__mmask32) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_storeu_epi8 (void *__P, __mmask16 __U, __m128i __A)
{
  __builtin_ia32_storedquqi128_mask ((char *) __P,
				     (__v16qi) __A,
				     (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_loadu_epi16 (__m256i __W, __mmask16 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddquhi256_mask ((const short *) __P,
						     (__v16hi) __W,
						     (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_loadu_epi16 (__mmask16 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddquhi256_mask ((const short *) __P,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_loadu_epi16 (__m128i __W, __mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddquhi128_mask ((const short *) __P,
						     (__v8hi) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_loadu_epi16 (__mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddquhi128_mask ((const short *) __P,
						     (__v8hi)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}


extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mov_epi16 (__m256i __W, __mmask16 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdquhi256_mask ((__v16hi) __A,
						    (__v16hi) __W,
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mov_epi16 (__mmask16 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdquhi256_mask ((__v16hi) __A,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mov_epi16 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdquhi128_mask ((__v8hi) __A,
						    (__v8hi) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mov_epi16 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdquhi128_mask ((__v8hi) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_loadu_epi8 (__m256i __W, __mmask32 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddquqi256_mask ((const char *) __P,
						     (__v32qi) __W,
						     (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_loadu_epi8 (__mmask32 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddquqi256_mask ((const char *) __P,
						     (__v32qi)
						     _mm256_setzero_si256 (),
						     (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_loadu_epi8 (__m128i __W, __mmask16 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddquqi128_mask ((const char *) __P,
						     (__v16qi) __W,
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_loadu_epi8 (__mmask16 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddquqi128_mask ((const char *) __P,
						     (__v16qi)
						     _mm_setzero_si128 (),
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi16_epi8 (__m256i __A)
{

  return (__m128i) __builtin_ia32_pmovwb256_mask ((__v16hi) __A,
						  (__v16qi)_mm_undefined_si128(),
						  (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi16_storeu_epi8 (void * __P, __mmask16 __M,__m256i __A)
{
  __builtin_ia32_pmovwb256mem_mask ((__v16qi *) __P , (__v16hi) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi16_epi8 (__m128i __O, __mmask16 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovwb256_mask ((__v16hi) __A,
						  (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi16_epi8 (__mmask16 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovwb256_mask ((__v16hi) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsepi16_epi8 (__m128i __A)
{

  return (__m128i) __builtin_ia32_pmovswb128_mask ((__v8hi) __A,
						   (__v16qi)_mm_undefined_si128(),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi16_storeu_epi8 (void * __P, __mmask8 __M,__m128i __A)
{
  __builtin_ia32_pmovswb128mem_mask ((__v8qi *) __P , (__v8hi) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi16_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovswb128_mask ((__v8hi) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtsepi16_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovswb128_mask ((__v8hi) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtsepi16_epi8 (__m256i __A)
{

  return (__m128i) __builtin_ia32_pmovswb256_mask ((__v16hi) __A,
						   (__v16qi)_mm_undefined_si128(),
						   (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi16_storeu_epi8 (void * __P, __mmask16 __M,__m256i __A)
{
  __builtin_ia32_pmovswb256mem_mask ((__v16qi *) __P , (__v16hi) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi16_epi8 (__m128i __O, __mmask16 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovswb256_mask ((__v16hi) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtsepi16_epi8 (__mmask16 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovswb256_mask ((__v16hi) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtusepi16_epi8 (__m128i __A)
{

  return (__m128i) __builtin_ia32_pmovuswb128_mask ((__v8hi) __A,
						    (__v16qi)_mm_undefined_si128(),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi16_storeu_epi8 (void * __P, __mmask8 __M,__m128i __A)
{
  __builtin_ia32_pmovuswb128mem_mask ((__v8qi *) __P , (__v8hi) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi16_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovuswb128_mask ((__v8hi) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtusepi16_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovuswb128_mask ((__v8hi) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtusepi16_epi8 (__m256i __A)
{

  return (__m128i) __builtin_ia32_pmovuswb256_mask ((__v16hi) __A,
						    (__v16qi)_mm_undefined_si128(),
						    (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi16_storeu_epi8 (void * __P, __mmask16 __M,__m256i __A)
{
  __builtin_ia32_pmovuswb256mem_mask ((__v16qi *) __P , (__v16hi) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi16_epi8 (__m128i __O, __mmask16 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovuswb256_mask ((__v16hi) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtusepi16_epi8 (__mmask16 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovuswb256_mask ((__v16hi) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcastb_epi8 (__m256i __O, __mmask32 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastb256_mask ((__v16qi) __A,
						       (__v32qi) __O,
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcastb_epi8 (__mmask32 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastb256_mask ((__v16qi) __A,
						       (__v32qi)
						       _mm256_setzero_si256 (),
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_set1_epi8 (__m256i __O, __mmask32 __M, char __A)
{
  return (__m256i) __builtin_ia32_pbroadcastb256_gpr_mask (__A,
							   (__v32qi) __O,
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_set1_epi8 (__mmask32 __M, char __A)
{
  return (__m256i) __builtin_ia32_pbroadcastb256_gpr_mask (__A,
							   (__v32qi)
							   _mm256_setzero_si256 (),
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_broadcastb_epi8 (__m128i __O, __mmask16 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastb128_mask ((__v16qi) __A,
						       (__v16qi) __O,
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_broadcastb_epi8 (__mmask16 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastb128_mask ((__v16qi) __A,
						       (__v16qi)
						       _mm_setzero_si128 (),
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_set1_epi8 (__m128i __O, __mmask16 __M, char __A)
{
  return (__m128i) __builtin_ia32_pbroadcastb128_gpr_mask (__A,
							   (__v16qi) __O,
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_set1_epi8 (__mmask16 __M, char __A)
{
  return (__m128i) __builtin_ia32_pbroadcastb128_gpr_mask (__A,
							   (__v16qi)
							   _mm_setzero_si128 (),
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcastw_epi16 (__m256i __O, __mmask16 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastw256_mask ((__v8hi) __A,
						       (__v16hi) __O,
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcastw_epi16 (__mmask16 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastw256_mask ((__v8hi) __A,
						       (__v16hi)
						       _mm256_setzero_si256 (),
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_set1_epi16 (__m256i __O, __mmask16 __M, short __A)
{
  return (__m256i) __builtin_ia32_pbroadcastw256_gpr_mask (__A,
							   (__v16hi) __O,
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_set1_epi16 (__mmask16 __M, short __A)
{
  return (__m256i) __builtin_ia32_pbroadcastw256_gpr_mask (__A,
							   (__v16hi)
							   _mm256_setzero_si256 (),
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_broadcastw_epi16 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastw128_mask ((__v8hi) __A,
						       (__v8hi) __O,
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_broadcastw_epi16 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastw128_mask ((__v8hi) __A,
						       (__v8hi)
						       _mm_setzero_si128 (),
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_set1_epi16 (__m128i __O, __mmask8 __M, short __A)
{
  return (__m128i) __builtin_ia32_pbroadcastw128_gpr_mask (__A,
							   (__v8hi) __O,
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_set1_epi16 (__mmask8 __M, short __A)
{
  return (__m128i) __builtin_ia32_pbroadcastw128_gpr_mask (__A,
							   (__v8hi)
							   _mm_setzero_si128 (),
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutexvar_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_permvarhi256_mask ((__v16hi) __B,
						     (__v16hi) __A,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutexvar_epi16 (__mmask16 __M, __m256i __A,
				__m256i __B)
{
  return (__m256i) __builtin_ia32_permvarhi256_mask ((__v16hi) __B,
						     (__v16hi) __A,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutexvar_epi16 (__m256i __W, __mmask16 __M, __m256i __A,
			       __m256i __B)
{
  return (__m256i) __builtin_ia32_permvarhi256_mask ((__v16hi) __B,
						     (__v16hi) __A,
						     (__v16hi) __W,
						     (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutexvar_epi16 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_permvarhi128_mask ((__v8hi) __B,
						     (__v8hi) __A,
						     (__v8hi)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutexvar_epi16 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_permvarhi128_mask ((__v8hi) __B,
						     (__v8hi) __A,
						     (__v8hi)
						     _mm_setzero_si128 (),
						     (__mmask8) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutexvar_epi16 (__m128i __W, __mmask8 __M, __m128i __A,
			    __m128i __B)
{
  return (__m128i) __builtin_ia32_permvarhi128_mask ((__v8hi) __B,
						     (__v8hi) __A,
						     (__v8hi) __W,
						     (__mmask8) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutex2var_epi16 (__m256i __A, __m256i __I, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2varhi256_mask ((__v16hi) __I
							/* idx */ ,
							(__v16hi) __A,
							(__v16hi) __B,
							(__mmask16) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutex2var_epi16 (__m256i __A, __mmask16 __U,
				__m256i __I, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2varhi256_mask ((__v16hi) __I
							/* idx */ ,
							(__v16hi) __A,
							(__v16hi) __B,
							(__mmask16)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask2_permutex2var_epi16 (__m256i __A, __m256i __I,
				 __mmask16 __U, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermi2varhi256_mask ((__v16hi) __A,
							(__v16hi) __I
							/* idx */ ,
							(__v16hi) __B,
							(__mmask16)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutex2var_epi16 (__mmask16 __U, __m256i __A,
				 __m256i __I, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2varhi256_maskz ((__v16hi) __I
							 /* idx */ ,
							 (__v16hi) __A,
							 (__v16hi) __B,
							 (__mmask16)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutex2var_epi16 (__m128i __A, __m128i __I, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2varhi128_mask ((__v8hi) __I
							/* idx */ ,
							(__v8hi) __A,
							(__v8hi) __B,
							(__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutex2var_epi16 (__m128i __A, __mmask8 __U, __m128i __I,
			     __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2varhi128_mask ((__v8hi) __I
							/* idx */ ,
							(__v8hi) __A,
							(__v8hi) __B,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask2_permutex2var_epi16 (__m128i __A, __m128i __I, __mmask8 __U,
			      __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermi2varhi128_mask ((__v8hi) __A,
							(__v8hi) __I
							/* idx */ ,
							(__v8hi) __B,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutex2var_epi16 (__mmask8 __U, __m128i __A, __m128i __I,
			      __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2varhi128_maskz ((__v8hi) __I
							 /* idx */ ,
							 (__v8hi) __A,
							 (__v8hi) __B,
							 (__mmask8)
							 __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_maddubs_epi16 (__m256i __W, __mmask16 __U, __m256i __X,
			   __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmaddubsw256_mask ((__v32qi) __X,
						     (__v32qi) __Y,
						     (__v16hi) __W,
						     (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_maddubs_epi16 (__mmask16 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmaddubsw256_mask ((__v32qi) __X,
						     (__v32qi) __Y,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_maddubs_epi16 (__m128i __W, __mmask8 __U, __m128i __X,
			__m128i __Y)
{
  return (__m128i) __builtin_ia32_pmaddubsw128_mask ((__v16qi) __X,
						     (__v16qi) __Y,
						     (__v8hi) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_maddubs_epi16 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmaddubsw128_mask ((__v16qi) __X,
						     (__v16qi) __Y,
						     (__v8hi)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_madd_epi16 (__m256i __W, __mmask8 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_pmaddwd256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v8si) __W,
						   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_madd_epi16 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaddwd256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v8si)
						   _mm256_setzero_si256 (),
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_madd_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaddwd128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v4si) __W,
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_madd_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaddwd128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v4si)
						   _mm_setzero_si128 (),
						   (__mmask8) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movepi8_mask (__m128i __A)
{
  return (__mmask16) __builtin_ia32_cvtb2mask128 ((__v16qi) __A);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movepi8_mask (__m256i __A)
{
  return (__mmask32) __builtin_ia32_cvtb2mask256 ((__v32qi) __A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movepi16_mask (__m128i __A)
{
  return (__mmask8) __builtin_ia32_cvtw2mask128 ((__v8hi) __A);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movepi16_mask (__m256i __A)
{
  return (__mmask16) __builtin_ia32_cvtw2mask256 ((__v16hi) __A);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movm_epi8 (__mmask16 __A)
{
  return (__m128i) __builtin_ia32_cvtmask2b128 (__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movm_epi8 (__mmask32 __A)
{
  return (__m256i) __builtin_ia32_cvtmask2b256 (__A);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movm_epi16 (__mmask8 __A)
{
  return (__m128i) __builtin_ia32_cvtmask2w128 (__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movm_epi16 (__mmask16 __A)
{
  return (__m256i) __builtin_ia32_cvtmask2w256 (__A);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_test_epi8_mask (__m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ptestmb128 ((__v16qi) __A,
						(__v16qi) __B,
						(__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_test_epi8_mask (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ptestmb128 ((__v16qi) __A,
						(__v16qi) __B, __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_test_epi8_mask (__m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ptestmb256 ((__v32qi) __A,
						(__v32qi) __B,
						(__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_test_epi8_mask (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ptestmb256 ((__v32qi) __A,
						(__v32qi) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_test_epi16_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestmw128 ((__v8hi) __A,
					       (__v8hi) __B,
					       (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_test_epi16_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestmw128 ((__v8hi) __A,
					       (__v8hi) __B, __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_test_epi16_mask (__m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ptestmw256 ((__v16hi) __A,
						(__v16hi) __B,
						(__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_test_epi16_mask (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ptestmw256 ((__v16hi) __A,
						(__v16hi) __B, __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epu16 (__mmask16 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminuw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epu16 (__m256i __W, __mmask16 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pminuw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epu16 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminuw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epu16 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pminuw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epi16 (__mmask16 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epi16 (__m256i __W, __mmask16 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epu8 (__mmask32 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxub256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  (__mmask32) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epu8 (__m256i __W, __mmask32 __M, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxub256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi) __W,
						  (__mmask32) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epu8 (__mmask16 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxub128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epu8 (__m128i __W, __mmask16 __M, __m128i __A,
		   __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxub128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi) __W,
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epi8 (__mmask32 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  (__mmask32) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epi8 (__m256i __W, __mmask32 __M, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi) __W,
						  (__mmask32) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epi8 (__mmask16 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epi8 (__m128i __W, __mmask16 __M, __m128i __A,
		   __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi) __W,
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epu8 (__mmask32 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminub256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  (__mmask32) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epu8 (__m256i __W, __mmask32 __M, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_pminub256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi) __W,
						  (__mmask32) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epu8 (__mmask16 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminub128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epu8 (__m128i __W, __mmask16 __M, __m128i __A,
		   __m128i __B)
{
  return (__m128i) __builtin_ia32_pminub128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi) __W,
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epi8 (__mmask32 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  (__mmask32) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epi8 (__m256i __W, __mmask32 __M, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi) __W,
						  (__mmask32) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epi8 (__mmask16 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epi8 (__m128i __W, __mmask16 __M, __m128i __A,
		   __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi) __W,
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epi16 (__mmask16 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epi16 (__m256i __W, __mmask16 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epi16 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epi16 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epu16 (__mmask16 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxuw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epu16 (__m256i __W, __mmask16 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxuw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epu16 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxuw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epu16 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxuw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epi16 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epi16 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __M);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_alignr_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
			 __m256i __B, const int __N)
{
  return (__m256i) __builtin_ia32_palignr256_mask ((__v4di) __A,
						   (__v4di) __B,
						   __N * 8,
						   (__v4di) __W,
						   (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_alignr_epi8 (__mmask32 __U, __m256i __A, __m256i __B,
			  const int __N)
{
  return (__m256i) __builtin_ia32_palignr256_mask ((__v4di) __A,
						   (__v4di) __B,
						   __N * 8,
						   (__v4di)
						   _mm256_setzero_si256 (),
						   (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_alignr_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
		      __m128i __B, const int __N)
{
  return (__m128i) __builtin_ia32_palignr128_mask ((__v2di) __A,
						   (__v2di) __B,
						   __N * 8,
						   (__v2di) __W,
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_alignr_epi8 (__mmask16 __U, __m128i __A, __m128i __B,
		       const int __N)
{
  return (__m128i) __builtin_ia32_palignr128_mask ((__v2di) __A,
						   (__v2di) __B,
						   __N * 8,
						   (__v2di)
						   _mm_setzero_si128 (),
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_dbsad_epu8 (__m256i __A, __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_dbpsadbw256_mask ((__v32qi) __A,
						    (__v32qi) __B,
						    __imm,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    (__mmask16) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_dbsad_epu8 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_dbpsadbw256_mask ((__v32qi) __A,
						    (__v32qi) __B,
						    __imm,
						    (__v16hi) __W,
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_dbsad_epu8 (__mmask16 __U, __m256i __A, __m256i __B,
			 const int __imm)
{
  return (__m256i) __builtin_ia32_dbpsadbw256_mask ((__v32qi) __A,
						    (__v32qi) __B,
						    __imm,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_dbsad_epu8 (__m128i __A, __m128i __B, const int __imm)
{
  return (__m128i) __builtin_ia32_dbpsadbw128_mask ((__v16qi) __A,
						    (__v16qi) __B,
						    __imm,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_dbsad_epu8 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B, const int __imm)
{
  return (__m128i) __builtin_ia32_dbpsadbw128_mask ((__v16qi) __A,
						    (__v16qi) __B,
						    __imm,
						    (__v8hi) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_dbsad_epu8 (__mmask8 __U, __m128i __A, __m128i __B,
		      const int __imm)
{
  return (__m128i) __builtin_ia32_dbpsadbw128_mask ((__v16qi) __A,
						    (__v16qi) __B,
						    __imm,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_blend_epi16 (__mmask8 __U, __m128i __A, __m128i __W)
{
  return (__m128i) __builtin_ia32_blendmw_128_mask ((__v8hi) __A,
						    (__v8hi) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_blend_epi8 (__mmask16 __U, __m128i __A, __m128i __W)
{
  return (__m128i) __builtin_ia32_blendmb_128_mask ((__v16qi) __A,
						    (__v16qi) __W,
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_blend_epi16 (__mmask16 __U, __m256i __A, __m256i __W)
{
  return (__m256i) __builtin_ia32_blendmw_256_mask ((__v16hi) __A,
						    (__v16hi) __W,
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_blend_epi8 (__mmask32 __U, __m256i __A, __m256i __W)
{
  return (__m256i) __builtin_ia32_blendmb_256_mask ((__v32qi) __A,
						    (__v32qi) __W,
						    (__mmask32) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epi16_mask (__mmask8 __U, __m128i __X, __m128i __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, __P,
						 (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epi16_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, __P,
						 (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epi16_mask (__mmask16 __U, __m256i __X, __m256i __Y,
			    const int __P)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, __P,
						  (__mmask16) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epi16_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, __P,
						  (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epi8_mask (__mmask16 __U, __m128i __X, __m128i __Y,
			const int __P)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, __P,
						  (__mmask16) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epi8_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, __P,
						  (__mmask16) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epi8_mask (__mmask32 __U, __m256i __X, __m256i __Y,
			   const int __P)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, __P,
						  (__mmask32) __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epi8_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, __P,
						  (__mmask32) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epu16_mask (__mmask8 __U, __m128i __X, __m128i __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epu16_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epu16_mask (__mmask16 __U, __m256i __X, __m256i __Y,
			    const int __P)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, __P,
						   (__mmask16) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epu16_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, __P,
						   (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epu8_mask (__mmask16 __U, __m128i __X, __m128i __Y,
			const int __P)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, __P,
						   (__mmask16) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epu8_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, __P,
						   (__mmask16) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epu8_mask (__mmask32 __U, __m256i __X, __m256i __Y,
			   const int __P)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, __P,
						   (__mmask32) __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epu8_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, __P,
						   (__mmask32) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srli_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			const int __imm)
{
  return (__m256i) __builtin_ia32_psrlwi256_mask ((__v16hi) __A, __imm,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srli_epi16 (__mmask16 __U, __m256i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_psrlwi256_mask ((__v16hi) __A, __imm,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srli_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     const int __imm)
{
  return (__m128i) __builtin_ia32_psrlwi128_mask ((__v8hi) __A, __imm,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srli_epi16 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_psrlwi128_mask ((__v8hi) __A, __imm,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shufflehi_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			     const int __imm)
{
  return (__m256i) __builtin_ia32_pshufhw256_mask ((__v16hi) __A,
						   __imm,
						   (__v16hi) __W,
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shufflehi_epi16 (__mmask16 __U, __m256i __A,
			      const int __imm)
{
  return (__m256i) __builtin_ia32_pshufhw256_mask ((__v16hi) __A,
						   __imm,
						   (__v16hi)
						   _mm256_setzero_si256 (),
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shufflehi_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
			  const int __imm)
{
  return (__m128i) __builtin_ia32_pshufhw128_mask ((__v8hi) __A, __imm,
						   (__v8hi) __W,
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shufflehi_epi16 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_pshufhw128_mask ((__v8hi) __A, __imm,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shufflelo_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			     const int __imm)
{
  return (__m256i) __builtin_ia32_pshuflw256_mask ((__v16hi) __A,
						   __imm,
						   (__v16hi) __W,
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shufflelo_epi16 (__mmask16 __U, __m256i __A,
			      const int __imm)
{
  return (__m256i) __builtin_ia32_pshuflw256_mask ((__v16hi) __A,
						   __imm,
						   (__v16hi)
						   _mm256_setzero_si256 (),
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shufflelo_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
			  const int __imm)
{
  return (__m128i) __builtin_ia32_pshuflw128_mask ((__v8hi) __A, __imm,
						   (__v8hi) __W,
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shufflelo_epi16 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_pshuflw128_mask ((__v8hi) __A, __imm,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srai_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			const int __imm)
{
  return (__m256i) __builtin_ia32_psrawi256_mask ((__v16hi) __A, __imm,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srai_epi16 (__mmask16 __U, __m256i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_psrawi256_mask ((__v16hi) __A, __imm,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srai_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     const int __imm)
{
  return (__m128i) __builtin_ia32_psrawi128_mask ((__v8hi) __A, __imm,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srai_epi16 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_psrawi128_mask ((__v8hi) __A, __imm,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_slli_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			int __B)
{
  return (__m256i) __builtin_ia32_psllwi256_mask ((__v16hi) __A, __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_slli_epi16 (__mmask16 __U, __m256i __A, int __B)
{
  return (__m256i) __builtin_ia32_psllwi256_mask ((__v16hi) __A, __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_slli_epi16 (__m128i __W, __mmask8 __U, __m128i __A, int __B)
{
  return (__m128i) __builtin_ia32_psllwi128_mask ((__v8hi) __A, __B,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_slli_epi16 (__mmask8 __U, __m128i __A, int __B)
{
  return (__m128i) __builtin_ia32_psllwi128_mask ((__v8hi) __A, __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

#else
#define _mm256_mask_alignr_epi8(W, U, X, Y, N)					    \
  ((__m256i) __builtin_ia32_palignr256_mask ((__v4di)(__m256i)(X),		    \
					    (__v4di)(__m256i)(Y), (int)((N) * 8),   \
					    (__v4di)(__m256i)(X), (__mmask32)(U)))

#define _mm256_mask_srli_epi16(W, U, A, B)                              \
  ((__m256i) __builtin_ia32_psrlwi256_mask ((__v16hi)(__m256i)(A),      \
    (int)(B), (__v16hi)(__m256i)(W), (__mmask16)(U)))

#define _mm256_maskz_srli_epi16(U, A, B)                                \
  ((__m256i) __builtin_ia32_psrlwi256_mask ((__v16hi)(__m256i)(A),      \
    (int)(B), (__v16hi)_mm256_setzero_si256 (), (__mmask16)(U)))

#define _mm_mask_srli_epi16(W, U, A, B)                                 \
  ((__m128i) __builtin_ia32_psrlwi128_mask ((__v8hi)(__m128i)(A),       \
    (int)(B), (__v8hi)(__m128i)(W), (__mmask8)(U)))

#define _mm_maskz_srli_epi16(U, A, B)                                   \
  ((__m128i) __builtin_ia32_psrlwi128_mask ((__v8hi)(__m128i)(A),       \
    (int)(B), (__v8hi)_mm_setzero_si128(), (__mmask8)(U)))

#define _mm256_mask_srai_epi16(W, U, A, B)                              \
  ((__m256i) __builtin_ia32_psrawi256_mask ((__v16hi)(__m256i)(A),      \
    (int)(B), (__v16hi)(__m256i)(W), (__mmask16)(U)))

#define _mm256_maskz_srai_epi16(U, A, B)                                \
  ((__m256i) __builtin_ia32_psrawi256_mask ((__v16hi)(__m256i)(A),      \
    (int)(B), (__v16hi)_mm256_setzero_si256 (), (__mmask16)(U)))

#define _mm_mask_srai_epi16(W, U, A, B)                                 \
  ((__m128i) __builtin_ia32_psrawi128_mask ((__v8hi)(__m128i)(A),       \
    (int)(B), (__v8hi)(__m128i)(W), (__mmask8)(U)))

#define _mm_maskz_srai_epi16(U, A, B)                                   \
  ((__m128i) __builtin_ia32_psrawi128_mask ((__v8hi)(__m128i)(A),       \
    (int)(B), (__v8hi)_mm_setzero_si128(), (__mmask8)(U)))

#define _mm256_mask_shufflehi_epi16(W, U, A, B)                                     \
  ((__m256i) __builtin_ia32_pshufhw256_mask ((__v16hi)(__m256i)(A), (int)(B),       \
                                             (__v16hi)(__m256i)(W),                 \
                                             (__mmask16)(U)))

#define _mm256_maskz_shufflehi_epi16(U, A, B)                                       \
  ((__m256i) __builtin_ia32_pshufhw256_mask ((__v16hi)(__m256i)(A), (int)(B),       \
                                             (__v16hi)(__m256i)_mm256_setzero_si256 (), \
                                             (__mmask16)(U)))

#define _mm_mask_shufflehi_epi16(W, U, A, B)                                        \
  ((__m128i) __builtin_ia32_pshufhw128_mask ((__v8hi)(__m128i)(A), (int)(B),        \
                                             (__v8hi)(__m128i)(W),                  \
                                             (__mmask8)(U)))

#define _mm_maskz_shufflehi_epi16(U, A, B)                                          \
  ((__m128i) __builtin_ia32_pshufhw128_mask ((__v8hi)(__m128i)(A), (int)(B),        \
					     (__v8hi)(__m128i)_mm_setzero_si128 (), \
                                             (__mmask8)(U)))

#define _mm256_mask_shufflelo_epi16(W, U, A, B)                                     \
  ((__m256i) __builtin_ia32_pshuflw256_mask ((__v16hi)(__m256i)(A), (int)(B),       \
                                             (__v16hi)(__m256i)(W),                 \
                                             (__mmask16)(U)))

#define _mm256_maskz_shufflelo_epi16(U, A, B)                                       \
  ((__m256i) __builtin_ia32_pshuflw256_mask ((__v16hi)(__m256i)(A), (int)(B),       \
                                             (__v16hi)(__m256i)_mm256_setzero_si256 (), \
                                             (__mmask16)(U)))

#define _mm_mask_shufflelo_epi16(W, U, A, B)                                        \
  ((__m128i) __builtin_ia32_pshuflw128_mask ((__v8hi)(__m128i)(A), (int)(B),        \
                                             (__v8hi)(__m128i)(W),                  \
                                             (__mmask8)(U)))

#define _mm_maskz_shufflelo_epi16(U, A, B)                                          \
  ((__m128i) __builtin_ia32_pshuflw128_mask ((__v8hi)(__m128i)(A), (int)(B),        \
					     (__v8hi)(__m128i)_mm_setzero_si128 (), \
                                             (__mmask8)(U)))

#define _mm256_maskz_alignr_epi8(U, X, Y, N)					    \
  ((__m256i) __builtin_ia32_palignr256_mask ((__v4di)(__m256i)(X),		    \
					    (__v4di)(__m256i)(Y), (int)((N) * 8),   \
					    (__v4di)(__m256i)_mm256_setzero_si256 (),   \
					    (__mmask32)(U)))

#define _mm_mask_alignr_epi8(W, U, X, Y, N)					    \
  ((__m128i) __builtin_ia32_palignr128_mask ((__v2di)(__m128i)(X),		    \
					    (__v2di)(__m128i)(Y), (int)((N) * 8),   \
					    (__v2di)(__m128i)(X), (__mmask16)(U)))

#define _mm_maskz_alignr_epi8(U, X, Y, N)					    \
  ((__m128i) __builtin_ia32_palignr128_mask ((__v2di)(__m128i)(X),		    \
					    (__v2di)(__m128i)(Y), (int)((N) * 8),   \
					    (__v2di)(__m128i)_mm_setzero_si128 (),  \
					    (__mmask16)(U)))

#define _mm_mask_slli_epi16(W, U, X, C)					  \
  ((__m128i)__builtin_ia32_psllwi128_mask ((__v8hi)(__m128i)(X), (int)(C),\
    (__v8hi)(__m128i)(W),\
    (__mmask8)(U)))

#define _mm_maskz_slli_epi16(U, X, C)					  \
  ((__m128i)__builtin_ia32_psllwi128_mask ((__v8hi)(__m128i)(X), (int)(C),\
    (__v8hi)(__m128i)_mm_setzero_si128 (),\
    (__mmask8)(U)))

#define _mm256_dbsad_epu8(X, Y, C)                                                  \
  ((__m256i) __builtin_ia32_dbpsadbw256_mask ((__v32qi)(__m256i) (X),               \
                                              (__v32qi)(__m256i) (Y), (int) (C),    \
                                              (__v16hi)(__m256i)_mm256_setzero_si256(),\
                                              (__mmask16)-1))

#define _mm256_mask_slli_epi16(W, U, X, C)                                 \
  ((__m256i)__builtin_ia32_psllwi256_mask ((__v16hi)(__m256i)(X), (int)(C),\
    (__v16hi)(__m256i)(W),\
    (__mmask16)(U)))

#define _mm256_maskz_slli_epi16(U, X, C)                                   \
  ((__m256i)__builtin_ia32_psllwi256_mask ((__v16hi)(__m256i)(X), (int)(C),\
    (__v16hi)(__m256i)_mm256_setzero_si256 (),\
    (__mmask16)(U)))

#define _mm256_mask_dbsad_epu8(W, U, X, Y, C)                                       \
  ((__m256i) __builtin_ia32_dbpsadbw256_mask ((__v32qi)(__m256i) (X),               \
                                              (__v32qi)(__m256i) (Y), (int) (C),    \
                                              (__v16hi)(__m256i)(W),                \
                                              (__mmask16)(U)))

#define _mm256_maskz_dbsad_epu8(U, X, Y, C)                                         \
  ((__m256i) __builtin_ia32_dbpsadbw256_mask ((__v32qi)(__m256i) (X),               \
                                              (__v32qi)(__m256i) (Y), (int) (C),    \
                                              (__v16hi)(__m256i)_mm256_setzero_si256(),\
                                              (__mmask16)(U)))

#define _mm_dbsad_epu8(X, Y, C)                                                     \
  ((__m128i) __builtin_ia32_dbpsadbw128_mask ((__v16qi)(__m128i) (X),               \
                                              (__v16qi)(__m128i) (Y), (int) (C),    \
                                              (__v8hi)(__m128i)_mm_setzero_si128(), \
                                              (__mmask8)-1))

#define _mm_mask_dbsad_epu8(W, U, X, Y, C)                                          \
  ((__m128i) __builtin_ia32_dbpsadbw128_mask ((__v16qi)(__m128i) (X),               \
                                              (__v16qi)(__m128i) (Y), (int) (C),    \
                                              (__v8hi)(__m128i)(W),                 \
                                              (__mmask8)(U)))

#define _mm_maskz_dbsad_epu8(U, X, Y, C)                                            \
  ((__m128i) __builtin_ia32_dbpsadbw128_mask ((__v16qi)(__m128i) (X),               \
                                              (__v16qi)(__m128i) (Y), (int) (C),    \
                                              (__v8hi)(__m128i)_mm_setzero_si128(), \
                                              (__mmask8)(U)))

#define _mm_mask_blend_epi16(__U, __A, __W)			      \
  ((__m128i) __builtin_ia32_blendmw_128_mask ((__v8hi) (__A),	      \
						    (__v8hi) (__W),   \
						    (__mmask8) (__U)))

#define _mm_mask_blend_epi8(__U, __A, __W)			      \
  ((__m128i) __builtin_ia32_blendmb_128_mask ((__v16qi) (__A),	      \
						    (__v16qi) (__W),  \
						    (__mmask16) (__U)))

#define _mm256_mask_blend_epi16(__U, __A, __W)			      \
  ((__m256i) __builtin_ia32_blendmw_256_mask ((__v16hi) (__A),	      \
						    (__v16hi) (__W),  \
						    (__mmask16) (__U)))

#define _mm256_mask_blend_epi8(__U, __A, __W)			      \
  ((__m256i) __builtin_ia32_blendmb_256_mask ((__v32qi) (__A),	      \
						    (__v32qi) (__W),  \
						    (__mmask32) (__U)))

#define _mm_cmp_epi16_mask(X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi)(__m128i)(X),	\
					    (__v8hi)(__m128i)(Y), (int)(P),\
					    (__mmask8)(-1)))

#define _mm_cmp_epi8_mask(X, Y, P)				\
  ((__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi)(__m128i)(X),	\
					    (__v16qi)(__m128i)(Y), (int)(P),\
					    (__mmask16)(-1)))

#define _mm256_cmp_epi16_mask(X, Y, P)				\
  ((__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi)(__m256i)(X),	\
					    (__v16hi)(__m256i)(Y), (int)(P),\
					    (__mmask16)(-1)))

#define _mm256_cmp_epi8_mask(X, Y, P)				\
  ((__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi)(__m256i)(X),	\
					    (__v32qi)(__m256i)(Y), (int)(P),\
					    (__mmask32)(-1)))

#define _mm_cmp_epu16_mask(X, Y, P)				\
  ((__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi)(__m128i)(X),	\
					    (__v8hi)(__m128i)(Y), (int)(P),\
					    (__mmask8)(-1)))

#define _mm_cmp_epu8_mask(X, Y, P)				\
  ((__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi)(__m128i)(X),	\
					    (__v16qi)(__m128i)(Y), (int)(P),\
					    (__mmask16)(-1)))

#define _mm256_cmp_epu16_mask(X, Y, P)				\
  ((__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi)(__m256i)(X),	\
					    (__v16hi)(__m256i)(Y), (int)(P),\
					    (__mmask16)(-1)))

#define _mm256_cmp_epu8_mask(X, Y, P)				\
  ((__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi)(__m256i)(X),	\
					    (__v32qi)(__m256i)(Y), (int)(P),\
					    (__mmask32)-1))

#define _mm_mask_cmp_epi16_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi)(__m128i)(X),	\
					    (__v8hi)(__m128i)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm_mask_cmp_epi8_mask(M, X, Y, P)				\
  ((__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi)(__m128i)(X),	\
					    (__v16qi)(__m128i)(Y), (int)(P),\
					    (__mmask16)(M)))

#define _mm256_mask_cmp_epi16_mask(M, X, Y, P)				\
  ((__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi)(__m256i)(X),	\
					    (__v16hi)(__m256i)(Y), (int)(P),\
					    (__mmask16)(M)))

#define _mm256_mask_cmp_epi8_mask(M, X, Y, P)				\
  ((__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi)(__m256i)(X),	\
					    (__v32qi)(__m256i)(Y), (int)(P),\
					    (__mmask32)(M)))

#define _mm_mask_cmp_epu16_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi)(__m128i)(X),	\
					    (__v8hi)(__m128i)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm_mask_cmp_epu8_mask(M, X, Y, P)				\
  ((__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi)(__m128i)(X),	\
					    (__v16qi)(__m128i)(Y), (int)(P),\
					    (__mmask16)(M)))

#define _mm256_mask_cmp_epu16_mask(M, X, Y, P)				\
  ((__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi)(__m256i)(X),	\
					    (__v16hi)(__m256i)(Y), (int)(P),\
					    (__mmask16)(M)))

#define _mm256_mask_cmp_epu8_mask(M, X, Y, P)				\
  ((__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi)(__m256i)(X),	\
					    (__v32qi)(__m256i)(Y), (int)(P),\
					    (__mmask32)(M)))
#endif

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epi8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 4,
						  (__mmask32) -1);
}

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epi8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 1,
						  (__mmask32) -1);
}

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epi8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 5,
						  (__mmask32) -1);
}

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epi8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 2,
						  (__mmask32) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epi16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 4,
						  (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epi16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 1,
						  (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epi16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 5,
						  (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epi16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 2,
						  (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epu8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 4,
						   (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epu8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 1,
						   (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epu8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 5,
						   (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epu8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 2,
						   (__mmask16) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epu16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 4,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epu16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 1,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epu16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 5,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epu16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 2,
						  (__mmask8) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epi8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 4,
						  (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epi8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 1,
						  (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epi8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 5,
						  (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epi8_mask (__m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 2,
						  (__mmask16) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epi16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 4,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epi16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 1,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epi16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 5,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epi16_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 2,
						 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mulhrs_epi16 (__m256i __W, __mmask16 __U, __m256i __X,
			  __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmulhrsw256_mask ((__v16hi) __X,
						    (__v16hi) __Y,
						    (__v16hi) __W,
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mulhrs_epi16 (__mmask16 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmulhrsw256_mask ((__v16hi) __X,
						    (__v16hi) __Y,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mulhi_epu16 (__m256i __W, __mmask16 __U, __m256i __A,
			 __m256i __B)
{
  return (__m256i) __builtin_ia32_pmulhuw256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v16hi) __W,
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mulhi_epu16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmulhuw256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v16hi)
						   _mm256_setzero_si256 (),
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mulhi_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			 __m256i __B)
{
  return (__m256i) __builtin_ia32_pmulhw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mulhi_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmulhw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mulhi_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		      __m128i __B)
{
  return (__m128i) __builtin_ia32_pmulhw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mulhi_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmulhw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mulhi_epu16 (__m128i __W, __mmask8 __U, __m128i __A,
		      __m128i __B)
{
  return (__m128i) __builtin_ia32_pmulhuw128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v8hi) __W,
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mulhi_epu16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmulhuw128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mulhrs_epi16 (__m128i __W, __mmask8 __U, __m128i __X,
		       __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmulhrsw128_mask ((__v8hi) __X,
						    (__v8hi) __Y,
						    (__v8hi) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mulhrs_epi16 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmulhrsw128_mask ((__v8hi) __X,
						    (__v8hi) __Y,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mullo_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			 __m256i __B)
{
  return (__m256i) __builtin_ia32_pmullw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mullo_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmullw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mullo_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		      __m128i __B)
{
  return (__m128i) __builtin_ia32_pmullw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mullo_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmullw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi8_epi16 (__m256i __W, __mmask16 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxbw256_mask ((__v16qi) __A,
						    (__v16hi) __W,
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi8_epi16 (__mmask16 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxbw256_mask ((__v16qi) __A,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi8_epi16 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxbw128_mask ((__v16qi) __A,
						    (__v8hi) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi8_epi16 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxbw128_mask ((__v16qi) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu8_epi16 (__m256i __W, __mmask16 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxbw256_mask ((__v16qi) __A,
						    (__v16hi) __W,
						    (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu8_epi16 (__mmask16 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxbw256_mask ((__v16qi) __A,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu8_epi16 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxbw128_mask ((__v16qi) __A,
						    (__v8hi) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu8_epi16 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxbw128_mask ((__v16qi) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_avg_epu8 (__m256i __W, __mmask32 __U, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_pavgb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__v32qi) __W,
						 (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_avg_epu8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pavgb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__v32qi)
						 _mm256_setzero_si256 (),
						 (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_avg_epu8 (__m128i __W, __mmask16 __U, __m128i __A,
		   __m128i __B)
{
  return (__m128i) __builtin_ia32_pavgb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__v16qi) __W,
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_avg_epu8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pavgb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__v16qi)
						 _mm_setzero_si128 (),
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_avg_epu16 (__m256i __W, __mmask16 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pavgw256_mask ((__v16hi) __A,
						 (__v16hi) __B,
						 (__v16hi) __W,
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_avg_epu16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pavgw256_mask ((__v16hi) __A,
						 (__v16hi) __B,
						 (__v16hi)
						 _mm256_setzero_si256 (),
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_avg_epu16 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pavgw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_avg_epu16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pavgw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_add_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_paddb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__v32qi) __W,
						 (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_add_epi8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__v32qi)
						 _mm256_setzero_si256 (),
						 (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_add_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_paddw256_mask ((__v16hi) __A,
						 (__v16hi) __B,
						 (__v16hi) __W,
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_add_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddw256_mask ((__v16hi) __A,
						 (__v16hi) __B,
						 (__v16hi)
						 _mm256_setzero_si256 (),
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_adds_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_paddsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi) __W,
						  (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_adds_epi8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_adds_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_paddsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_adds_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_adds_epu8 (__m256i __W, __mmask32 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_paddusb256_mask ((__v32qi) __A,
						   (__v32qi) __B,
						   (__v32qi) __W,
						   (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_adds_epu8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddusb256_mask ((__v32qi) __A,
						   (__v32qi) __B,
						   (__v32qi)
						   _mm256_setzero_si256 (),
						   (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_adds_epu16 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_paddusw256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v16hi) __W,
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_adds_epu16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddusw256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v16hi)
						   _mm256_setzero_si256 (),
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sub_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_psubb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__v32qi) __W,
						 (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sub_epi8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__v32qi)
						 _mm256_setzero_si256 (),
						 (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sub_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_psubw256_mask ((__v16hi) __A,
						 (__v16hi) __B,
						 (__v16hi) __W,
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sub_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubw256_mask ((__v16hi) __A,
						 (__v16hi) __B,
						 (__v16hi)
						 _mm256_setzero_si256 (),
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_subs_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_psubsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi) __W,
						  (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_subs_epi8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubsb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_subs_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_psubsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_subs_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubsw256_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_subs_epu8 (__m256i __W, __mmask32 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_psubusb256_mask ((__v32qi) __A,
						   (__v32qi) __B,
						   (__v32qi) __W,
						   (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_subs_epu8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubusb256_mask ((__v32qi) __A,
						   (__v32qi) __B,
						   (__v32qi)
						   _mm256_setzero_si256 (),
						   (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_subs_epu16 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_psubusw256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v16hi) __W,
						   (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_subs_epu16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubusw256_mask ((__v16hi) __A,
						   (__v16hi) __B,
						   (__v16hi)
						   _mm256_setzero_si256 (),
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
		   __m128i __B)
{
  return (__m128i) __builtin_ia32_paddb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__v16qi) __W,
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_epi8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__v16qi)
						 _mm_setzero_si128 (),
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_paddw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpackhi_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
			   __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhbw256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     (__v32qi) __W,
						     (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpackhi_epi8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhbw256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     (__v32qi)
						     _mm256_setzero_si256 (),
						     (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpackhi_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
			__m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhbw128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     (__v16qi) __W,
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpackhi_epi8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhbw128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     (__v16qi)
						     _mm_setzero_si128 (),
						     (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpackhi_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			    __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhwd256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     (__v16hi) __W,
						     (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpackhi_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhwd256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpackhi_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
			 __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhwd128_mask ((__v8hi) __A,
						     (__v8hi) __B,
						     (__v8hi) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpackhi_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhwd128_mask ((__v8hi) __A,
						     (__v8hi) __B,
						     (__v8hi)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpacklo_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
			   __m256i __B)
{
  return (__m256i) __builtin_ia32_punpcklbw256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     (__v32qi) __W,
						     (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpacklo_epi8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpcklbw256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     (__v32qi)
						     _mm256_setzero_si256 (),
						     (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpacklo_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
			__m128i __B)
{
  return (__m128i) __builtin_ia32_punpcklbw128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     (__v16qi) __W,
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpacklo_epi8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpcklbw128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     (__v16qi)
						     _mm_setzero_si128 (),
						     (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpacklo_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			    __m256i __B)
{
  return (__m256i) __builtin_ia32_punpcklwd256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     (__v16hi) __W,
						     (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpacklo_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpcklwd256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpacklo_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
			 __m128i __B)
{
  return (__m128i) __builtin_ia32_punpcklwd128_mask ((__v8hi) __A,
						     (__v8hi) __B,
						     (__v8hi) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpacklo_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpcklwd128_mask ((__v8hi) __A,
						     (__v8hi) __B,
						     (__v8hi)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epi8_mask (__m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_pcmpeqb128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epu8_mask (__m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __A,
						    (__v16qi) __B, 0,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epu8_mask (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __A,
						    (__v16qi) __B, 0,
						    __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epi8_mask (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_pcmpeqb128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epu8_mask (__m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __A,
						    (__v32qi) __B, 0,
						    (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi8_mask (__m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_pcmpeqb256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epu8_mask (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __A,
						    (__v32qi) __B, 0,
						    __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epi8_mask (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_pcmpeqb256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epu16_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __A,
						   (__v8hi) __B, 0,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epi16_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqw128_mask ((__v8hi) __A,
						    (__v8hi) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epu16_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __A,
						   (__v8hi) __B, 0, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epi16_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqw128_mask ((__v8hi) __A,
						    (__v8hi) __B, __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epu16_mask (__m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __A,
						    (__v16hi) __B, 0,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi16_mask (__m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_pcmpeqw256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epu16_mask (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __A,
						    (__v16hi) __B, 0,
						    __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epi16_mask (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_pcmpeqw256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epu8_mask (__m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __A,
						    (__v16qi) __B, 6,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epi8_mask (__m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_pcmpgtb128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epu8_mask (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __A,
						    (__v16qi) __B, 6,
						    __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epi8_mask (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_pcmpgtb128_mask ((__v16qi) __A,
						     (__v16qi) __B,
						     __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epu8_mask (__m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __A,
						    (__v32qi) __B, 6,
						    (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi8_mask (__m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_pcmpgtb256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epu8_mask (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __A,
						    (__v32qi) __B, 6,
						    __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epi8_mask (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_pcmpgtb256_mask ((__v32qi) __A,
						     (__v32qi) __B,
						     __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epu16_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __A,
						   (__v8hi) __B, 6,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epi16_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtw128_mask ((__v8hi) __A,
						    (__v8hi) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epu16_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __A,
						   (__v8hi) __B, 6, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epi16_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtw128_mask ((__v8hi) __A,
						    (__v8hi) __B, __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epu16_mask (__m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __A,
						    (__v16hi) __B, 6,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi16_mask (__m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_pcmpgtw256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epu16_mask (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __A,
						    (__v16hi) __B, 6,
						    __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epi16_mask (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_pcmpgtw256_mask ((__v16hi) __A,
						     (__v16hi) __B,
						     __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_testn_epi8_mask (__m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ptestnmb128 ((__v16qi) __A,
						 (__v16qi) __B,
						 (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_testn_epi8_mask (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_ptestnmb128 ((__v16qi) __A,
						 (__v16qi) __B, __U);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testn_epi8_mask (__m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ptestnmb256 ((__v32qi) __A,
						 (__v32qi) __B,
						 (__mmask32) -1);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_testn_epi8_mask (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_ptestnmb256 ((__v32qi) __A,
						 (__v32qi) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_testn_epi16_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmw128 ((__v8hi) __A,
						(__v8hi) __B,
						(__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_testn_epi16_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmw128 ((__v8hi) __A,
						(__v8hi) __B, __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testn_epi16_mask (__m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ptestnmw256 ((__v16hi) __A,
						 (__v16hi) __B,
						 (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_testn_epi16_mask (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__mmask16) __builtin_ia32_ptestnmw256 ((__v16hi) __A,
						 (__v16hi) __B, __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_epi8 (__m256i __W, __mmask32 __U, __m256i __A,
			  __m256i __B)
{
  return (__m256i) __builtin_ia32_pshufb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi) __W,
						  (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_epi8 (__mmask32 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pshufb256_mask ((__v32qi) __A,
						  (__v32qi) __B,
						  (__v32qi)
						  _mm256_setzero_si256 (),
						  (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shuffle_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
		       __m128i __B)
{
  return (__m128i) __builtin_ia32_pshufb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi) __W,
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shuffle_epi8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pshufb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_packs_epi16 (__mmask32 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_packsswb256_mask ((__v16hi) __A,
						    (__v16hi) __B,
						    (__v32qi)
						    _mm256_setzero_si256 (),
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_packs_epi16 (__m256i __W, __mmask32 __M, __m256i __A,
			 __m256i __B)
{
  return (__m256i) __builtin_ia32_packsswb256_mask ((__v16hi) __A,
						    (__v16hi) __B,
						    (__v32qi) __W,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_packs_epi16 (__mmask16 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_packsswb128_mask ((__v8hi) __A,
						    (__v8hi) __B,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_packs_epi16 (__m128i __W, __mmask16 __M, __m128i __A,
		      __m128i __B)
{
  return (__m128i) __builtin_ia32_packsswb128_mask ((__v8hi) __A,
						    (__v8hi) __B,
						    (__v16qi) __W,
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_packus_epi16 (__mmask32 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_packuswb256_mask ((__v16hi) __A,
						    (__v16hi) __B,
						    (__v32qi)
						    _mm256_setzero_si256 (),
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_packus_epi16 (__m256i __W, __mmask32 __M, __m256i __A,
			  __m256i __B)
{
  return (__m256i) __builtin_ia32_packuswb256_mask ((__v16hi) __A,
						    (__v16hi) __B,
						    (__v32qi) __W,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_packus_epi16 (__mmask16 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_packuswb128_mask ((__v8hi) __A,
						    (__v8hi) __B,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_packus_epi16 (__m128i __W, __mmask16 __M, __m128i __A,
		       __m128i __B)
{
  return (__m128i) __builtin_ia32_packuswb128_mask ((__v8hi) __A,
						    (__v8hi) __B,
						    (__v16qi) __W,
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_abs_epi8 (__m256i __W, __mmask32 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsb256_mask ((__v32qi) __A,
						 (__v32qi) __W,
						 (__mmask32) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_abs_epi8 (__mmask32 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsb256_mask ((__v32qi) __A,
						 (__v32qi)
						 _mm256_setzero_si256 (),
						 (__mmask32) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_abs_epi8 (__m128i __W, __mmask16 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsb128_mask ((__v16qi) __A,
						 (__v16qi) __W,
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_abs_epi8 (__mmask16 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsb128_mask ((__v16qi) __A,
						 (__v16qi)
						 _mm_setzero_si128 (),
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_abs_epi16 (__m256i __W, __mmask16 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsw256_mask ((__v16hi) __A,
						 (__v16hi) __W,
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_abs_epi16 (__mmask16 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsw256_mask ((__v16hi) __A,
						 (__v16hi)
						 _mm256_setzero_si256 (),
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_abs_epi16 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsw128_mask ((__v8hi) __A,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_abs_epi16 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsw128_mask ((__v8hi) __A,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epu8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 4,
						   (__mmask32) -1);
}

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epu8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 1,
						   (__mmask32) -1);
}

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epu8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 5,
						   (__mmask32) -1);
}

extern __inline __mmask32
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epu8_mask (__m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 2,
						   (__mmask32) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epu16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 4,
						   (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epu16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 1,
						   (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epu16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 5,
						   (__mmask16) -1);
}

extern __inline __mmask16
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epu16_mask (__m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 2,
						   (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_storeu_epi16 (void *__P, __mmask16 __U, __m256i __A)
{
  __builtin_ia32_storedquhi256_mask ((short *) __P,
				     (__v16hi) __A,
				     (__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_storeu_epi16 (void *__P, __mmask8 __U, __m128i __A)
{
  __builtin_ia32_storedquhi128_mask ((short *) __P,
				     (__v8hi) __A,
				     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_adds_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_paddsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_subs_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psubsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi) __W,
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_subs_epi8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_subs_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_psubsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_subs_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_subs_epu8 (__m128i __W, __mmask16 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psubusb128_mask ((__v16qi) __A,
						   (__v16qi) __B,
						   (__v16qi) __W,
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_subs_epu8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubusb128_mask ((__v16qi) __A,
						   (__v16qi) __B,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_subs_epu16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_psubusw128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v8hi) __W,
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_subs_epu16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubusw128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srl_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psrlw256_mask ((__v16hi) __A,
						 (__v8hi) __B,
						 (__v16hi) __W,
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srl_epi16 (__mmask16 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psrlw256_mask ((__v16hi) __A,
						 (__v8hi) __B,
						 (__v16hi)
						 _mm256_setzero_si256 (),
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srl_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psrlw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srl_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrlw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sra_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psraw256_mask ((__v16hi) __A,
						 (__v8hi) __B,
						 (__v16hi) __W,
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sra_epi16 (__mmask16 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psraw256_mask ((__v16hi) __A,
						 (__v8hi) __B,
						 (__v16hi)
						 _mm256_setzero_si256 (),
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sra_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psraw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sra_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psraw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_adds_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddsw128_mask ((__v8hi) __A,
						  (__v8hi) __B,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_adds_epu8 (__m128i __W, __mmask16 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_paddusb128_mask ((__v16qi) __A,
						   (__v16qi) __B,
						   (__v16qi) __W,
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_adds_epu8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddusb128_mask ((__v16qi) __A,
						   (__v16qi) __B,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_adds_epu16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_paddusw128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v8hi) __W,
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_adds_epu16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddusw128_mask ((__v8hi) __A,
						   (__v8hi) __B,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
		   __m128i __B)
{
  return (__m128i) __builtin_ia32_psubb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__v16qi) __W,
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_epi8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__v16qi)
						 _mm_setzero_si128 (),
						 (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psubw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_adds_epi8 (__m128i __W, __mmask16 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_paddsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi) __W,
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_adds_epi8 (__mmask16 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddsb128_mask ((__v16qi) __A,
						  (__v16qi) __B,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi16_epi8 (__m128i __A)
{

  return (__m128i) __builtin_ia32_pmovwb128_mask ((__v8hi) __A,
						  (__v16qi)_mm_undefined_si128(),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi16_storeu_epi8 (void * __P, __mmask8 __M,__m128i __A)
{
  __builtin_ia32_pmovwb128mem_mask ((__v8qi *) __P , (__v8hi) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi16_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovwb128_mask ((__v8hi) __A,
						  (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi16_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovwb128_mask ((__v8hi) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srav_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psrav16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srav_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_psrav16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srav_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psrav16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_srav_epi16 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrav8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srav_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_psrav8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srav_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrav8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srlv_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psrlv16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srlv_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_psrlv16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srlv_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psrlv16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_srlv_epi16 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrlv8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srlv_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_psrlv8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srlv_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrlv8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sllv_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psllv16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sllv_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_psllv16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi) __W,
						  (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sllv_epi16 (__mmask16 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psllv16hi_mask ((__v16hi) __A,
						  (__v16hi) __B,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  (__mmask16) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sllv_epi16 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psllv8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sllv_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_psllv8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sllv_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psllv8hi_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sll_epi16 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psllw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sll_epi16 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psllw128_mask ((__v8hi) __A,
						 (__v8hi) __B,
						 (__v8hi)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sll_epi16 (__m256i __W, __mmask16 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psllw256_mask ((__v16hi) __A,
						 (__v8hi) __B,
						 (__v16hi) __W,
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sll_epi16 (__mmask16 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psllw256_mask ((__v16hi) __A,
						 (__v8hi) __B,
						 (__v16hi)
						 _mm256_setzero_si256 (),
						 (__mmask16) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_packus_epi32 (__mmask16 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_packusdw256_mask ((__v8si) __A,
						    (__v8si) __B,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_packus_epi32 (__m256i __W, __mmask16 __M, __m256i __A,
			  __m256i __B)
{
  return (__m256i) __builtin_ia32_packusdw256_mask ((__v8si) __A,
						    (__v8si) __B,
						    (__v16hi) __W,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_packus_epi32 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_packusdw128_mask ((__v4si) __A,
						    (__v4si) __B,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_packus_epi32 (__m128i __W, __mmask8 __M, __m128i __A,
		       __m128i __B)
{
  return (__m128i) __builtin_ia32_packusdw128_mask ((__v4si) __A,
						    (__v4si) __B,
						    (__v8hi) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_packs_epi32 (__mmask16 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_packssdw256_mask ((__v8si) __A,
						    (__v8si) __B,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_packs_epi32 (__m256i __W, __mmask16 __M, __m256i __A,
			 __m256i __B)
{
  return (__m256i) __builtin_ia32_packssdw256_mask ((__v8si) __A,
						    (__v8si) __B,
						    (__v16hi) __W,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_packs_epi32 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_packssdw128_mask ((__v4si) __A,
						    (__v4si) __B,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_packs_epi32 (__m128i __W, __mmask8 __M, __m128i __A,
		      __m128i __B)
{
  return (__m128i) __builtin_ia32_packssdw128_mask ((__v4si) __A,
						    (__v4si) __B,
						    (__v8hi) __W, __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epu8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 4,
						   (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epu8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 1,
						   (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epu8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 5,
						   (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epu8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpb128_mask ((__v16qi) __X,
						   (__v16qi) __Y, 2,
						   (__mmask16) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epu16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 4,
						  (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epu16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 1,
						  (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epu16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 5,
						  (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epu16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpw128_mask ((__v8hi) __X,
						  (__v8hi) __Y, 2,
						  (__mmask8) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epi8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 4,
						  (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epi8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 1,
						  (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epi8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 5,
						  (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epi8_mask (__mmask16 __M, __m128i __X, __m128i __Y)
{
  return (__mmask16) __builtin_ia32_cmpb128_mask ((__v16qi) __X,
						  (__v16qi) __Y, 2,
						  (__mmask16) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epi16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 4,
						 (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epi16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 1,
						 (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epi16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 5,
						 (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epi16_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpw128_mask ((__v8hi) __X,
						 (__v8hi) __Y, 2,
						 (__mmask8) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epu8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 4,
						   (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epu8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 1,
						   (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epu8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 5,
						   (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epu8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_ucmpb256_mask ((__v32qi) __X,
						   (__v32qi) __Y, 2,
						   (__mmask32) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epu16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 4,
						   (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epu16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 1,
						   (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epu16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 5,
						   (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epu16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpw256_mask ((__v16hi) __X,
						   (__v16hi) __Y, 2,
						   (__mmask16) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epi8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 4,
						  (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epi8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 1,
						  (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epi8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 5,
						  (__mmask32) __M);
}

extern __inline __mmask32
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epi8_mask (__mmask32 __M, __m256i __X, __m256i __Y)
{
  return (__mmask32) __builtin_ia32_cmpb256_mask ((__v32qi) __X,
						  (__v32qi) __Y, 2,
						  (__mmask32) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epi16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 4,
						  (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epi16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 1,
						  (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epi16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 5,
						  (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epi16_mask (__mmask16 __M, __m256i __X, __m256i __Y)
{
  return (__mmask16) __builtin_ia32_cmpw256_mask ((__v16hi) __X,
						  (__v16hi) __Y, 2,
						  (__mmask16) __M);
}

#ifdef __DISABLE_AVX512VLBW__
#undef __DISABLE_AVX512VLBW__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VLBW__ */

#endif /* _AVX512VLBWINTRIN_H_INCLUDED */
