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

#ifndef __VAESINTRIN_H_INCLUDED
#define __VAESINTRIN_H_INCLUDED

#if !defined(__VAES__) || !defined(__AVX__)
#pragma GCC push_options
#pragma GCC target("vaes,avx")
#define __DISABLE_VAES__
#endif /* __VAES__ */

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_aesdec_epi128 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_vaesdec_v32qi ((__v32qi) __A, (__v32qi) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_aesdeclast_epi128 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_vaesdeclast_v32qi ((__v32qi) __A,
								(__v32qi) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_aesenc_epi128 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_vaesenc_v32qi ((__v32qi) __A, (__v32qi) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_aesenclast_epi128 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_vaesenclast_v32qi ((__v32qi) __A,
								(__v32qi) __B);
}

#ifdef __DISABLE_VAES__
#undef __DISABLE_VAES__
#pragma GCC pop_options
#endif /* __DISABLE_VAES__ */


#if !defined(__VAES__) || !defined(__AVX512F__)
#pragma GCC push_options
#pragma GCC target("vaes,avx512f")
#define __DISABLE_VAESF__
#endif /* __VAES__ */


extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_aesdec_epi128 (__m512i __A, __m512i __B)
{
  return (__m512i)__builtin_ia32_vaesdec_v64qi ((__v64qi) __A, (__v64qi) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_aesdeclast_epi128 (__m512i __A, __m512i __B)
{
  return (__m512i)__builtin_ia32_vaesdeclast_v64qi ((__v64qi) __A,
						    (__v64qi) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_aesenc_epi128 (__m512i __A, __m512i __B)
{
  return (__m512i)__builtin_ia32_vaesenc_v64qi ((__v64qi) __A, (__v64qi) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_aesenclast_epi128 (__m512i __A, __m512i __B)
{
  return (__m512i)__builtin_ia32_vaesenclast_v64qi ((__v64qi) __A,
						    (__v64qi) __B);
}

#ifdef __DISABLE_VAESF__
#undef __DISABLE_VAESF__
#pragma GCC pop_options
#endif /* __DISABLE_VAES__ */

#endif /* __VAESINTRIN_H_INCLUDED */
