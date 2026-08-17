/* Copyright (C) 2007-2020 Free Software Foundation, Inc.

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

/* Implemented from the specification included in the AMD Programmers
   Manual Update, version 2.x */

#ifndef _AMMINTRIN_H_INCLUDED
#define _AMMINTRIN_H_INCLUDED

/* We need definitions from the SSE3, SSE2 and SSE header files*/
#include <pmmintrin.h>

#ifndef __SSE4A__
#pragma GCC push_options
#pragma GCC target("sse4a")
#define __DISABLE_SSE4A__
#endif /* __SSE4A__ */

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_stream_sd (double * __P, __m128d __Y)
{
  __builtin_ia32_movntsd (__P, (__v2df) __Y);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_stream_ss (float * __P, __m128 __Y)
{
  __builtin_ia32_movntss (__P, (__v4sf) __Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_extract_si64 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_extrq ((__v2di) __X, (__v16qi) __Y);
}

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_extracti_si64 (__m128i __X, unsigned const int __I, unsigned const int __L)
{
  return (__m128i) __builtin_ia32_extrqi ((__v2di) __X, __I, __L);
}
#else
#define _mm_extracti_si64(X, I, L)					\
  ((__m128i) __builtin_ia32_extrqi ((__v2di)(__m128i)(X),		\
				    (unsigned int)(I), (unsigned int)(L)))
#endif

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_insert_si64 (__m128i __X,__m128i __Y)
{
  return (__m128i) __builtin_ia32_insertq ((__v2di)__X, (__v2di)__Y);
}

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_inserti_si64(__m128i __X, __m128i __Y, unsigned const int __I, unsigned const int __L)
{
  return (__m128i) __builtin_ia32_insertqi ((__v2di)__X, (__v2di)__Y, __I, __L);
}
#else
#define _mm_inserti_si64(X, Y, I, L)					\
  ((__m128i) __builtin_ia32_insertqi ((__v2di)(__m128i)(X),		\
				      (__v2di)(__m128i)(Y),		\
				      (unsigned int)(I), (unsigned int)(L)))
#endif

#ifdef __DISABLE_SSE4A__
#undef __DISABLE_SSE4A__
#pragma GCC pop_options
#endif /* __DISABLE_SSE4A__ */

#endif /* _AMMINTRIN_H_INCLUDED */
