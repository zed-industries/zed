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
#error "Never use <avx512cdintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512CDINTRIN_H_INCLUDED
#define _AVX512CDINTRIN_H_INCLUDED

#ifndef __AVX512CD__
#pragma GCC push_options
#pragma GCC target("avx512cd")
#define __DISABLE_AVX512CD__
#endif /* __AVX512CD__ */

/* Internal data types for implementing the intrinsics.  */
typedef long long __v8di __attribute__ ((__vector_size__ (64)));
typedef int __v16si __attribute__ ((__vector_size__ (64)));

/* The Intel API is flexible enough that we must allow aliasing with other
   vector types, and their scalar components.  */
typedef long long __m512i __attribute__ ((__vector_size__ (64), __may_alias__));
typedef double __m512d __attribute__ ((__vector_size__ (64), __may_alias__));

typedef unsigned char  __mmask8;
typedef unsigned short __mmask16;

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_conflict_epi32 (__m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vpconflictsi_512_mask ((__v16si) __A,
					       (__v16si) _mm512_setzero_si512 (),
					       (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_conflict_epi32 (__m512i __W, __mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpconflictsi_512_mask ((__v16si) __A,
							 (__v16si) __W,
							 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_conflict_epi32 (__mmask16 __U, __m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vpconflictsi_512_mask ((__v16si) __A,
					       (__v16si) _mm512_setzero_si512 (),
					       (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_conflict_epi64 (__m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vpconflictdi_512_mask ((__v8di) __A,
					       (__v8di) _mm512_setzero_si512 (),
					       (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_conflict_epi64 (__m512i __W, __mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vpconflictdi_512_mask ((__v8di) __A,
							 (__v8di) __W,
							 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_conflict_epi64 (__mmask8 __U, __m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vpconflictdi_512_mask ((__v8di) __A,
					       (__v8di) _mm512_setzero_si512 (),
					       (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_lzcnt_epi64 (__m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vplzcntq_512_mask ((__v8di) __A,
					   (__v8di) _mm512_setzero_si512 (),
					   (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_lzcnt_epi64 (__m512i __W, __mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vplzcntq_512_mask ((__v8di) __A,
						     (__v8di) __W,
						     (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_lzcnt_epi64 (__mmask8 __U, __m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vplzcntq_512_mask ((__v8di) __A,
					   (__v8di) _mm512_setzero_si512 (),
					   (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_lzcnt_epi32 (__m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vplzcntd_512_mask ((__v16si) __A,
					   (__v16si) _mm512_setzero_si512 (),
					   (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_lzcnt_epi32 (__m512i __W, __mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_vplzcntd_512_mask ((__v16si) __A,
						     (__v16si) __W,
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_lzcnt_epi32 (__mmask16 __U, __m512i __A)
{
  return (__m512i)
	 __builtin_ia32_vplzcntd_512_mask ((__v16si) __A,
					   (__v16si) _mm512_setzero_si512 (),
					   (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastmb_epi64 (__mmask8 __A)
{
  return (__m512i) __builtin_ia32_broadcastmb512 (__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastmw_epi32 (__mmask16 __A)
{
  return (__m512i) __builtin_ia32_broadcastmw512 (__A);
}

#ifdef __DISABLE_AVX512CD__
#undef __DISABLE_AVX512CD__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512CD__ */

#endif /* _AVX512CDINTRIN_H_INCLUDED */
