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

#ifndef _IMMINTRIN_H_INCLUDED
#error "Never use <avx512bf16intrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512BF16INTRIN_H_INCLUDED
#define _AVX512BF16INTRIN_H_INCLUDED

#ifndef __AVX512BF16__
#pragma GCC push_options
#pragma GCC target("avx512bf16")
#define __DISABLE_AVX512BF16__
#endif /* __AVX512BF16__ */

/* Internal data types for implementing the intrinsics.  */
typedef short __v32bh __attribute__ ((__vector_size__ (64)));

/* The Intel API is flexible enough that we must allow aliasing with other
   vector types, and their scalar components.  */
typedef short __m512bh __attribute__ ((__vector_size__ (64), __may_alias__));

/* vcvtne2ps2bf16 */

extern __inline __m512bh
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtne2ps_pbh (__m512 __A, __m512 __B)
{
  return (__m512bh)__builtin_ia32_cvtne2ps2bf16_v32hi(__A, __B);
}

extern __inline __m512bh
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtne2ps_pbh (__m512bh __A, __mmask32 __B, __m512 __C, __m512 __D)
{
  return (__m512bh)__builtin_ia32_cvtne2ps2bf16_v32hi_mask(__C, __D, __A, __B);
}

extern __inline __m512bh
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtne2ps_pbh (__mmask32 __A, __m512 __B, __m512 __C)
{
  return (__m512bh)__builtin_ia32_cvtne2ps2bf16_v32hi_maskz(__B, __C, __A);
}

/* vcvtneps2bf16 */

extern __inline __m256bh
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtneps_pbh (__m512 __A)
{
  return (__m256bh)__builtin_ia32_cvtneps2bf16_v16sf(__A);
}

extern __inline __m256bh
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtneps_pbh (__m256bh __A, __mmask16 __B, __m512 __C)
{
  return (__m256bh)__builtin_ia32_cvtneps2bf16_v16sf_mask(__C, __A, __B);
}

extern __inline __m256bh
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtneps_pbh (__mmask16 __A, __m512 __B)
{
  return (__m256bh)__builtin_ia32_cvtneps2bf16_v16sf_maskz(__B, __A);
}

/* vdpbf16ps */

extern __inline __m512
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_dpbf16_ps (__m512 __A, __m512bh __B, __m512bh __C)
{
  return (__m512)__builtin_ia32_dpbf16ps_v16sf(__A, __B, __C);
}

extern __inline __m512
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_dpbf16_ps (__m512 __A, __mmask16 __B, __m512bh __C, __m512bh __D)
{
  return (__m512)__builtin_ia32_dpbf16ps_v16sf_mask(__A, __C, __D, __B);
}

extern __inline __m512
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_dpbf16_ps (__mmask16 __A, __m512 __B, __m512bh __C, __m512bh __D)
{
  return (__m512)__builtin_ia32_dpbf16ps_v16sf_maskz(__B, __C, __D, __A);
}

#ifdef __DISABLE_AVX512BF16__
#undef __DISABLE_AVX512BF16__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512BF16__ */

#endif /* _AVX512BF16INTRIN_H_INCLUDED */
