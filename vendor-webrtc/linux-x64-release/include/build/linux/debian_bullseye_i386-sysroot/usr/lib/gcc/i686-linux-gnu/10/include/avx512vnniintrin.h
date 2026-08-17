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
#error "Never use <avx512vnniintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef __AVX512VNNIINTRIN_H_INCLUDED
#define __AVX512VNNIINTRIN_H_INCLUDED

#if !defined(__AVX512VNNI__)
#pragma GCC push_options
#pragma GCC target("avx512vnni")
#define __DISABLE_AVX512VNNI__
#endif /* __AVX512VNNI__ */

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_dpbusd_epi32 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpdpbusd_v16si ((__v16si)__A, (__v16si) __B,
								(__v16si) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_dpbusd_epi32 (__m512i __A, __mmask16 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpbusd_v16si_mask ((__v16si)__A,
				(__v16si) __C, (__v16si) __D, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_dpbusd_epi32 (__mmask16 __A, __m512i __B, __m512i __C,
							__m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpbusd_v16si_maskz ((__v16si)__B,
				(__v16si) __C, (__v16si) __D, (__mmask16)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_dpbusds_epi32 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpdpbusds_v16si ((__v16si)__A, (__v16si) __B,
							 (__v16si) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_dpbusds_epi32 (__m512i __A, __mmask16 __B, __m512i __C,
							__m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpbusds_v16si_mask ((__v16si)__A,
				(__v16si) __C, (__v16si) __D, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_dpbusds_epi32 (__mmask16 __A, __m512i __B, __m512i __C,
							__m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpbusds_v16si_maskz ((__v16si)__B,
				(__v16si) __C, (__v16si) __D, (__mmask16)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_dpwssd_epi32 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpdpwssd_v16si ((__v16si)__A, (__v16si) __B,
								(__v16si) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_dpwssd_epi32 (__m512i __A, __mmask16 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpwssd_v16si_mask ((__v16si)__A,
				(__v16si) __C, (__v16si) __D, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_dpwssd_epi32 (__mmask16 __A, __m512i __B, __m512i __C,
							__m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpwssd_v16si_maskz ((__v16si)__B,
				(__v16si) __C, (__v16si) __D, (__mmask16)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_dpwssds_epi32 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpdpwssds_v16si ((__v16si)__A, (__v16si) __B,
								(__v16si) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_dpwssds_epi32 (__m512i __A, __mmask16 __B, __m512i __C,
							__m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpwssds_v16si_mask ((__v16si)__A,
				(__v16si) __C, (__v16si) __D, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_dpwssds_epi32 (__mmask16 __A, __m512i __B, __m512i __C,
							__m512i __D)
{
  return (__m512i)__builtin_ia32_vpdpwssds_v16si_maskz ((__v16si)__B,
				(__v16si) __C, (__v16si) __D, (__mmask16)__A);
}

#ifdef __DISABLE_AVX512VNNI__
#undef __DISABLE_AVX512VNNI__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VNNI__ */

#endif /* __AVX512VNNIINTRIN_H_INCLUDED */
