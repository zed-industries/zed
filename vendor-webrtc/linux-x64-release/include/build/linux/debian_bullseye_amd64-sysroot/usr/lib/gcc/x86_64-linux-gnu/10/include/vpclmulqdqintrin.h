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
#error "Never use <vpclmulqdqintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _VPCLMULQDQINTRIN_H_INCLUDED
#define _VPCLMULQDQINTRIN_H_INCLUDED

#if !defined(__VPCLMULQDQ__) || !defined(__AVX512F__)
#pragma GCC push_options
#pragma GCC target("vpclmulqdq,avx512f")
#define __DISABLE_VPCLMULQDQF__
#endif /* __VPCLMULQDQF__ */

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_clmulepi64_epi128 (__m512i __A, __m512i __B, const int __C)
{
  return (__m512i) __builtin_ia32_vpclmulqdq_v8di ((__v8di)__A,
						  (__v8di) __B, __C);
}
#else
#define _mm512_clmulepi64_epi128(A, B, C)				   \
  ((__m512i) __builtin_ia32_vpclmulqdq_v8di ((__v8di)(__m512i)(A),	\
				(__v8di)(__m512i)(B), (int)(C)))
#endif

#ifdef __DISABLE_VPCLMULQDQF__
#undef __DISABLE_VPCLMULQDQF__
#pragma GCC pop_options
#endif /* __DISABLE_VPCLMULQDQF__ */

#if !defined(__VPCLMULQDQ__) || !defined(__AVX__)
#pragma GCC push_options
#pragma GCC target("vpclmulqdq,avx")
#define __DISABLE_VPCLMULQDQ__
#endif /* __VPCLMULQDQ__ */

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_clmulepi64_epi128 (__m256i __A, __m256i __B, const int __C)
{
  return (__m256i) __builtin_ia32_vpclmulqdq_v4di ((__v4di)__A,
						   (__v4di) __B, __C);
}
#else
#define _mm256_clmulepi64_epi128(A, B, C)			   \
  ((__m256i) __builtin_ia32_vpclmulqdq_v4di ((__v4di)(__m256i)(A), \
				(__v4di)(__m256i)(B), (int)(C)))
#endif

#ifdef __DISABLE_VPCLMULQDQ__
#undef __DISABLE_VPCLMULQDQ__
#pragma GCC pop_options
#endif /* __DISABLE_VPCLMULQDQ__ */

#endif /* _VPCLMULQDQINTRIN_H_INCLUDED */
