/* Copyright (C) 2015-2020 Free Software Foundation, Inc.

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
# error "Never use <avx5124vnniwintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _AVX5124VNNIWINTRIN_H_INCLUDED
#define _AVX5124VNNIWINTRIN_H_INCLUDED

#ifndef __AVX5124VNNIW__
#pragma GCC push_options
#pragma GCC target("avx5124vnniw")
#define __DISABLE_AVX5124VNNIW__
#endif /* __AVX5124VNNIW__ */

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_4dpwssd_epi32 (__m512i __A, __m512i __B, __m512i __C,
		      __m512i __D, __m512i __E, __m128i *__F)
{
  return (__m512i) __builtin_ia32_vp4dpwssd ((__v16si) __B,
					     (__v16si) __C,
					     (__v16si) __D,
					     (__v16si) __E,
					     (__v16si) __A,
					     (const __v4si *) __F);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_4dpwssd_epi32 (__m512i __A, __mmask16 __U, __m512i __B,
			   __m512i __C, __m512i __D, __m512i __E,
			   __m128i *__F)
{
  return (__m512i) __builtin_ia32_vp4dpwssd_mask ((__v16si) __B,
						  (__v16si) __C,
						  (__v16si) __D,
						  (__v16si) __E,
						  (__v16si) __A,
						  (const __v4si *) __F,
						  (__v16si) __A,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_4dpwssd_epi32 (__mmask16 __U, __m512i __A, __m512i __B,
			    __m512i __C, __m512i __D, __m512i __E,
			    __m128i *__F)
{
  return (__m512i) __builtin_ia32_vp4dpwssd_mask ((__v16si) __B,
						  (__v16si) __C,
						  (__v16si) __D,
						  (__v16si) __E,
						  (__v16si) __A,
						  (const __v4si *) __F,
						  (__v16si) _mm512_setzero_ps (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_4dpwssds_epi32 (__m512i __A, __m512i __B, __m512i __C,
		       __m512i __D, __m512i __E, __m128i *__F)
{
  return (__m512i) __builtin_ia32_vp4dpwssds ((__v16si) __B,
					      (__v16si) __C,
					      (__v16si) __D,
					      (__v16si) __E,
					      (__v16si) __A,
					      (const __v4si *) __F);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_4dpwssds_epi32 (__m512i __A, __mmask16 __U, __m512i __B,
			    __m512i __C, __m512i __D, __m512i __E,
			    __m128i *__F)
{
  return (__m512i) __builtin_ia32_vp4dpwssds_mask ((__v16si) __B,
						   (__v16si) __C,
						   (__v16si) __D,
						   (__v16si) __E,
						   (__v16si) __A,
						   (const __v4si *) __F,
						   (__v16si) __A,
						   (__mmask16) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_4dpwssds_epi32 (__mmask16 __U, __m512i __A, __m512i __B,
			     __m512i __C, __m512i __D, __m512i __E,
			     __m128i *__F)
{
  return (__m512i) __builtin_ia32_vp4dpwssds_mask ((__v16si) __B,
						   (__v16si) __C,
						   (__v16si) __D,
						   (__v16si) __E,
						   (__v16si) __A,
						   (const __v4si *) __F,
						   (__v16si) _mm512_setzero_ps (),
						   (__mmask16) __U);
}

#ifdef __DISABLE_AVX5124VNNIW__
#undef __DISABLE_AVX5124VNNIW__
#pragma GCC pop_options
#endif /* __DISABLE_AVX5124VNNIW__ */

#endif /* _AVX5124VNNIWINTRIN_H_INCLUDED */
