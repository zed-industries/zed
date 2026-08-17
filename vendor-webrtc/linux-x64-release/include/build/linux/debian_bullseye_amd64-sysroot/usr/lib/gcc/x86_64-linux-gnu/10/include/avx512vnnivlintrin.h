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
#error "Never use <avx512vnnivlintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512VNNIVLINTRIN_H_INCLUDED
#define _AVX512VNNIVLINTRIN_H_INCLUDED

#if !defined(__AVX512VL__) || !defined(__AVX512VNNI__)
#pragma GCC push_options
#pragma GCC target("avx512vnni,avx512vl")
#define __DISABLE_AVX512VNNIVL__
#endif /* __AVX512VNNIVL__ */

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_dpbusd_epi32 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpdpbusd_v8si ((__v8si)__A, (__v8si) __B,
								(__v8si) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_dpbusd_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpbusd_v8si_mask ((__v8si)__A, (__v8si) __C,
						(__v8si) __D, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_dpbusd_epi32 (__mmask8 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpbusd_v8si_maskz ((__v8si)__B,
				(__v8si) __C, (__v8si) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_dpbusd_epi32 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpdpbusd_v4si ((__v4si)__A, (__v4si) __B,
								(__v4si) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_dpbusd_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpbusd_v4si_mask ((__v4si)__A, (__v4si) __C,
						(__v4si) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_dpbusd_epi32 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpbusd_v4si_maskz ((__v4si)__B,
				(__v4si) __C, (__v4si) __D, (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_dpbusds_epi32 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpdpbusds_v8si ((__v8si)__A, (__v8si) __B,
								(__v8si) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_dpbusds_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpbusds_v8si_mask ((__v8si)__A,
				(__v8si) __C, (__v8si) __D, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_dpbusds_epi32 (__mmask8 __A, __m256i __B, __m256i __C,
								__m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpbusds_v8si_maskz ((__v8si)__B,
				(__v8si) __C, (__v8si) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_dpbusds_epi32 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpdpbusds_v4si ((__v4si)__A, (__v4si) __B,
								(__v4si) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_dpbusds_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpbusds_v4si_mask ((__v4si)__A,
				(__v4si) __C, (__v4si) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_dpbusds_epi32 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpbusds_v4si_maskz ((__v4si)__B,
				(__v4si) __C, (__v4si) __D, (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_dpwssd_epi32 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpdpwssd_v8si ((__v8si)__A, (__v8si) __B,
								(__v8si) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_dpwssd_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpwssd_v8si_mask ((__v8si)__A, (__v8si) __C,
						(__v8si) __D, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_dpwssd_epi32 (__mmask8 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpwssd_v8si_maskz ((__v8si)__B,
				(__v8si) __C, (__v8si) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_dpwssd_epi32 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpdpwssd_v4si ((__v4si)__A, (__v4si) __B,
								(__v4si) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_dpwssd_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpwssd_v4si_mask ((__v4si)__A, (__v4si) __C,
						(__v4si) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_dpwssd_epi32 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpwssd_v4si_maskz ((__v4si)__B,
				(__v4si) __C, (__v4si) __D, (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_dpwssds_epi32 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpdpwssds_v8si ((__v8si)__A, (__v8si) __B,
								(__v8si) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_dpwssds_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpwssds_v8si_mask ((__v8si)__A,
				(__v8si) __C, (__v8si) __D, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_dpwssds_epi32 (__mmask8 __A, __m256i __B, __m256i __C,
							__m256i __D)
{
  return (__m256i)__builtin_ia32_vpdpwssds_v8si_maskz ((__v8si)__B,
				(__v8si) __C, (__v8si) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_dpwssds_epi32 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpdpwssds_v4si ((__v4si)__A, (__v4si) __B,
								(__v4si) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_dpwssds_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpwssds_v4si_mask ((__v4si)__A,
				(__v4si) __C, (__v4si) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_dpwssds_epi32 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpdpwssds_v4si_maskz ((__v4si)__B,
				(__v4si) __C, (__v4si) __D, (__mmask8)__A);
}
#ifdef __DISABLE_AVX512VNNIVL__
#undef __DISABLE_AVX512VNNIVL__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VNNIVL__ */
#endif /* __DISABLE_AVX512VNNIVL__ */
