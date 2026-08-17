/* Copyright (C) 2019-2020 Free Software Foundation, Inc.

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
#error "Never use <avx512vp2intersectintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512VP2INTERSECTVLINTRIN_H_INCLUDED
#define _AVX512VP2INTERSECTVLINTRIN_H_INCLUDED

#if !defined(__AVX512VP2INTERSECT__) || !defined(__AVX512VL__)
#pragma GCC push_options
#pragma GCC target("avx512vp2intersect,avx512vl")
#define __DISABLE_AVX512VP2INTERSECTVL__
#endif /* __AVX512VP2INTERSECTVL__ */

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_2intersect_epi32 (__m128i __A, __m128i __B, __mmask8 *__U, __mmask8 *__M)
{
  __builtin_ia32_2intersectd128 (__U, __M, (__v4si) __A, (__v4si) __B);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_2intersect_epi32 (__m256i __A, __m256i __B, __mmask8 *__U,
			 __mmask8 *__M)
{
  __builtin_ia32_2intersectd256 (__U, __M, (__v8si) __A, (__v8si) __B);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_2intersect_epi64 (__m128i __A, __m128i __B, __mmask8 *__U, __mmask8 *__M)
{
  __builtin_ia32_2intersectq128 (__U, __M, (__v2di) __A, (__v2di) __B);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_2intersect_epi64 (__m256i __A, __m256i __B, __mmask8 *__U,
			 __mmask8 *__M)
{
  __builtin_ia32_2intersectq256 (__U, __M, (__v4di) __A, (__v4di) __B);
}

#ifdef __DISABLE_AVX512VP2INTERSECTVL__
#undef __DISABLE_AVX512VP2INTERSECTVL__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VP2INTERSECTVL__ */

#endif /* _AVX512VP2INTERSECTVLINTRIN_H_INCLUDED */
