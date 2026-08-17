/* Copyright (C) 2013-2020 Free Software Foundation, Inc.

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
#error "Never use <avx512vbmiintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512VBMIINTRIN_H_INCLUDED
#define _AVX512VBMIINTRIN_H_INCLUDED

#ifndef __AVX512VBMI__
#pragma GCC push_options
#pragma GCC target("avx512vbmi")
#define __DISABLE_AVX512VBMI__
#endif /* __AVX512VBMI__ */

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_multishift_epi64_epi8 (__m512i __W, __mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_vpmultishiftqb512_mask ((__v64qi) __X,
							  (__v64qi) __Y,
							  (__v64qi) __W,
							  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_multishift_epi64_epi8 (__mmask64 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_vpmultishiftqb512_mask ((__v64qi) __X,
							  (__v64qi) __Y,
							  (__v64qi)
							  _mm512_setzero_si512 (),
							  (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_multishift_epi64_epi8 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_vpmultishiftqb512_mask ((__v64qi) __X,
							  (__v64qi) __Y,
							  (__v64qi)
							  _mm512_undefined_epi32 (),
							  (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutexvar_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_permvarqi512_mask ((__v64qi) __B,
						     (__v64qi) __A,
						     (__v64qi)
						     _mm512_undefined_epi32 (),
						     (__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutexvar_epi8 (__mmask64 __M, __m512i __A,
				__m512i __B)
{
  return (__m512i) __builtin_ia32_permvarqi512_mask ((__v64qi) __B,
						     (__v64qi) __A,
						     (__v64qi)
						     _mm512_setzero_si512(),
						     (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutexvar_epi8 (__m512i __W, __mmask64 __M, __m512i __A,
			       __m512i __B)
{
  return (__m512i) __builtin_ia32_permvarqi512_mask ((__v64qi) __B,
						     (__v64qi) __A,
						     (__v64qi) __W,
						     (__mmask64) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex2var_epi8 (__m512i __A, __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varqi512_mask ((__v64qi) __I
							/* idx */ ,
							(__v64qi) __A,
							(__v64qi) __B,
							(__mmask64) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex2var_epi8 (__m512i __A, __mmask64 __U,
				__m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varqi512_mask ((__v64qi) __I
							/* idx */ ,
							(__v64qi) __A,
							(__v64qi) __B,
							(__mmask64)
							__U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask2_permutex2var_epi8 (__m512i __A, __m512i __I,
				 __mmask64 __U, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermi2varqi512_mask ((__v64qi) __A,
							(__v64qi) __I
							/* idx */ ,
							(__v64qi) __B,
							(__mmask64)
							__U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex2var_epi8 (__mmask64 __U, __m512i __A,
				 __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varqi512_maskz ((__v64qi) __I
							 /* idx */ ,
							 (__v64qi) __A,
							 (__v64qi) __B,
							 (__mmask64)
							 __U);
}

#ifdef __DISABLE_AVX512VBMI__
#undef __DISABLE_AVX512VBMI__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VBMI__ */

#endif /* _AVX512VBMIINTRIN_H_INCLUDED */
