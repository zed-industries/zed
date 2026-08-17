/* Copyright (C) 2017-2020 Free Software Foundation, Inc.

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

#if !defined _IMMINTRIN_H_INCLUDED
# error "Never use <avx512bitalgintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _AVX512BITALGINTRIN_H_INCLUDED
#define _AVX512BITALGINTRIN_H_INCLUDED

#ifndef __AVX512BITALG__
#pragma GCC push_options
#pragma GCC target("avx512bitalg")
#define __DISABLE_AVX512BITALG__
#endif /* __AVX512BITALG__ */

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_popcnt_epi8 (__m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountb_v64qi ((__v64qi) __A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_popcnt_epi16 (__m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountw_v32hi ((__v32hi) __A);
}

#ifdef __DISABLE_AVX512BITALG__
#undef __DISABLE_AVX512BITALG__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512BITALG__ */

#if !defined(__AVX512BITALG__) || !defined(__AVX512BW__)
#pragma GCC push_options
#pragma GCC target("avx512bitalg,avx512bw")
#define __DISABLE_AVX512BITALGBW__
#endif /* __AVX512VLBW__ */

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_popcnt_epi8 (__m512i __W, __mmask64 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountb_v64qi_mask ((__v64qi) __A,
							 (__v64qi) __W,
							 (__mmask64) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_popcnt_epi8 (__mmask64 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountb_v64qi_mask ((__v64qi) __A,
						(__v64qi)
						_mm512_setzero_si512 (),
						(__mmask64) __U);
}
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_popcnt_epi16 (__m512i __W, __mmask32 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountw_v32hi_mask ((__v32hi) __A,
							(__v32hi) __W,
							(__mmask32) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_popcnt_epi16 (__mmask32 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountw_v32hi_mask ((__v32hi) __A,
						(__v32hi)
						_mm512_setzero_si512 (),
						(__mmask32) __U);
}

extern __inline __mmask64
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_bitshuffle_epi64_mask (__m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_vpshufbitqmb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__mmask64) -1);
}

extern __inline __mmask64
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_bitshuffle_epi64_mask (__mmask64 __M, __m512i __A, __m512i __B)
{
  return (__mmask64) __builtin_ia32_vpshufbitqmb512_mask ((__v64qi) __A,
						 (__v64qi) __B,
						 (__mmask64) __M);
}

#ifdef __DISABLE_AVX512BITALGBW__
#undef __DISABLE_AVX512BITALGBW__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512BITALGBW__ */

#if !defined(__AVX512BITALG__) || !defined(__AVX512VL__) || !defined(__AVX512BW__)
#pragma GCC push_options
#pragma GCC target("avx512bitalg,avx512vl,avx512bw")
#define __DISABLE_AVX512BITALGVLBW__
#endif /* __AVX512VLBW__ */

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_popcnt_epi8 (__m256i __W, __mmask32 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpopcountb_v32qi_mask ((__v32qi) __A,
							 (__v32qi) __W,
							 (__mmask32) __U);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_popcnt_epi8 (__mmask32 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpopcountb_v32qi_mask ((__v32qi) __A,
						(__v32qi)
						 _mm256_setzero_si256 (),
						(__mmask32) __U);
}

extern __inline __mmask32
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_bitshuffle_epi64_mask (__m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_vpshufbitqmb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__mmask32) -1);
}

extern __inline __mmask32
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_bitshuffle_epi64_mask (__mmask32 __M, __m256i __A, __m256i __B)
{
  return (__mmask32) __builtin_ia32_vpshufbitqmb256_mask ((__v32qi) __A,
						 (__v32qi) __B,
						 (__mmask32) __M);
}

#ifdef __DISABLE_AVX512BITALGVLBW__
#undef __DISABLE_AVX512BITALGVLBW__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512BITALGVLBW__ */


#if !defined(__AVX512BITALG__) || !defined(__AVX512VL__)
#pragma GCC push_options
#pragma GCC target("avx512bitalg,avx512vl")
#define __DISABLE_AVX512BITALGVL__
#endif /* __AVX512VLBW__ */

extern __inline __mmask16
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_bitshuffle_epi64_mask (__m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_vpshufbitqmb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__mmask16) -1);
}

extern __inline __mmask16
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_bitshuffle_epi64_mask (__mmask16 __M, __m128i __A, __m128i __B)
{
  return (__mmask16) __builtin_ia32_vpshufbitqmb128_mask ((__v16qi) __A,
						 (__v16qi) __B,
						 (__mmask16) __M);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_popcnt_epi8 (__m256i __A)
{
  return (__m256i) __builtin_ia32_vpopcountb_v32qi ((__v32qi) __A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_popcnt_epi16 (__m256i __A)
{
  return (__m256i) __builtin_ia32_vpopcountw_v16hi ((__v16hi) __A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_popcnt_epi8 (__m128i __A)
{
  return (__m128i) __builtin_ia32_vpopcountb_v16qi ((__v16qi) __A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_popcnt_epi16 (__m128i __A)
{
  return (__m128i) __builtin_ia32_vpopcountw_v8hi ((__v8hi) __A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_popcnt_epi16 (__m256i __W, __mmask16 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpopcountw_v16hi_mask ((__v16hi) __A,
							(__v16hi) __W,
							(__mmask16) __U);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_popcnt_epi16 (__mmask16 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpopcountw_v16hi_mask ((__v16hi) __A,
						(__v16hi)
						_mm256_setzero_si256 (),
						(__mmask16) __U);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_popcnt_epi8 (__m128i __W, __mmask16 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpopcountb_v16qi_mask ((__v16qi) __A,
							 (__v16qi) __W,
							 (__mmask16) __U);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_popcnt_epi8 (__mmask16 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpopcountb_v16qi_mask ((__v16qi) __A,
							 (__v16qi)
							 _mm_setzero_si128 (),
							 (__mmask16) __U);
}
extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_popcnt_epi16 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpopcountw_v8hi_mask ((__v8hi) __A,
							(__v8hi) __W,
							(__mmask8) __U);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_popcnt_epi16 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpopcountw_v8hi_mask ((__v8hi) __A,
							(__v8hi)
							_mm_setzero_si128 (),
							(__mmask8) __U);
}
#ifdef __DISABLE_AVX512BITALGVL__
#undef __DISABLE_AVX512BITALGVL__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512BITALGBW__ */

#endif /* _AVX512BITALGINTRIN_H_INCLUDED */
