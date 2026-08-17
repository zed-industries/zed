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
# error "Never use <avx512vpopcntdqintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _AVX512VPOPCNTDQINTRIN_H_INCLUDED
#define _AVX512VPOPCNTDQINTRIN_H_INCLUDED

#ifndef __AVX512VPOPCNTDQ__
#pragma GCC push_options
#pragma GCC target("avx512vpopcntdq")
#define __DISABLE_AVX512VPOPCNTDQ__
#endif /* __AVX512VPOPCNTDQ__ */

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_popcnt_epi32 (__m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountd_v16si ((__v16si) __A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_popcnt_epi32 (__m512i __W, __mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountd_v16si_mask ((__v16si) __A,
							 (__v16si) __W,
							 (__mmask16) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_popcnt_epi32 (__mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountd_v16si_mask ((__v16si) __A,
							 (__v16si)
							 _mm512_setzero_si512 (),
							 (__mmask16) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_popcnt_epi64 (__m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountq_v8di ((__v8di) __A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_popcnt_epi64 (__m512i __W, __mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountq_v8di_mask ((__v8di) __A,
							(__v8di) __W,
							(__mmask8) __U);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_popcnt_epi64 (__mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpopcountq_v8di_mask ((__v8di) __A,
							(__v8di)
							_mm512_setzero_si512 (),
							(__mmask8) __U);
}

#ifdef __DISABLE_AVX512VPOPCNTDQ__
#undef __DISABLE_AVX512VPOPCNTDQ__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VPOPCNTDQ__ */

#endif /* _AVX512VPOPCNTDQINTRIN_H_INCLUDED */
