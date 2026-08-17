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
#error "Never use <avx512ifmaintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512IFMAINTRIN_H_INCLUDED
#define _AVX512IFMAINTRIN_H_INCLUDED

#ifndef __AVX512IFMA__
#pragma GCC push_options
#pragma GCC target("avx512ifma")
#define __DISABLE_AVX512IFMA__
#endif /* __AVX512IFMA__ */

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_madd52lo_epu64 (__m512i __X, __m512i __Y, __m512i __Z)
{
  return (__m512i) __builtin_ia32_vpmadd52luq512_mask ((__v8di) __X,
						       (__v8di) __Y,
						       (__v8di) __Z,
						       (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_madd52hi_epu64 (__m512i __X, __m512i __Y, __m512i __Z)
{
  return (__m512i) __builtin_ia32_vpmadd52huq512_mask ((__v8di) __X,
						       (__v8di) __Y,
						       (__v8di) __Z,
						       (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_madd52lo_epu64 (__m512i __W, __mmask8 __M, __m512i __X,
			    __m512i __Y)
{
  return (__m512i) __builtin_ia32_vpmadd52luq512_mask ((__v8di) __W,
						       (__v8di) __X,
						       (__v8di) __Y,
						       (__mmask8) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_madd52hi_epu64 (__m512i __W, __mmask8 __M, __m512i __X,
			    __m512i __Y)
{
  return (__m512i) __builtin_ia32_vpmadd52huq512_mask ((__v8di) __W,
						       (__v8di) __X,
						       (__v8di) __Y,
						       (__mmask8) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_madd52lo_epu64 (__mmask8 __M, __m512i __X, __m512i __Y, __m512i __Z)
{
  return (__m512i) __builtin_ia32_vpmadd52luq512_maskz ((__v8di) __X,
							(__v8di) __Y,
							(__v8di) __Z,
							(__mmask8) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_madd52hi_epu64 (__mmask8 __M, __m512i __X, __m512i __Y, __m512i __Z)
{
  return (__m512i) __builtin_ia32_vpmadd52huq512_maskz ((__v8di) __X,
							(__v8di) __Y,
							(__v8di) __Z,
							(__mmask8) __M);
}

#ifdef __DISABLE_AVX512IFMA__
#undef __DISABLE_AVX512IFMA__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512IFMA__ */

#endif /* _AVX512IFMAINTRIN_H_INCLUDED */
