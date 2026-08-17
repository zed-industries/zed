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
#error "Never use <avx512vbmi2intrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef __AVX512VBMI2INTRIN_H_INCLUDED
#define __AVX512VBMI2INTRIN_H_INCLUDED

#if !defined(__AVX512VBMI2__)
#pragma GCC push_options
#pragma GCC target("avx512vbmi2")
#define __DISABLE_AVX512VBMI2__
#endif /* __AVX512VBMI2__ */

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shrdi_epi16 (__m512i __A, __m512i __B, int __C)
{
  return (__m512i) __builtin_ia32_vpshrd_v32hi ((__v32hi)__A, (__v32hi) __B,
									__C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shrdi_epi32 (__m512i __A, __m512i __B, int __C)
{
  return (__m512i) __builtin_ia32_vpshrd_v16si ((__v16si)__A, (__v16si) __B,
									__C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shrdi_epi32 (__m512i __A, __mmask16 __B, __m512i __C, __m512i __D,
								int __E)
{
  return (__m512i)__builtin_ia32_vpshrd_v16si_mask ((__v16si)__C,
			(__v16si) __D, __E, (__v16si) __A, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shrdi_epi32 (__mmask16 __A, __m512i __B, __m512i __C, int __D)
{
  return (__m512i)__builtin_ia32_vpshrd_v16si_mask ((__v16si)__B,
	(__v16si) __C, __D, (__v16si) _mm512_setzero_si512 (), (__mmask16)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shrdi_epi64 (__m512i __A, __m512i __B, int __C)
{
  return (__m512i) __builtin_ia32_vpshrd_v8di ((__v8di)__A, (__v8di) __B, __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shrdi_epi64 (__m512i __A, __mmask8 __B, __m512i __C, __m512i __D,
								int __E)
{
  return (__m512i)__builtin_ia32_vpshrd_v8di_mask ((__v8di)__C, (__v8di) __D,
					__E, (__v8di) __A, (__mmask8)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shrdi_epi64 (__mmask8 __A, __m512i __B, __m512i __C, int __D)
{
  return (__m512i)__builtin_ia32_vpshrd_v8di_mask ((__v8di)__B, (__v8di) __C,
			__D, (__v8di) _mm512_setzero_si512 (), (__mmask8)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shldi_epi16 (__m512i __A, __m512i __B, int __C)
{
  return (__m512i) __builtin_ia32_vpshld_v32hi ((__v32hi)__A, (__v32hi) __B,
									__C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shldi_epi32 (__m512i __A, __m512i __B, int __C)
{
  return (__m512i) __builtin_ia32_vpshld_v16si ((__v16si)__A, (__v16si) __B,
									__C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shldi_epi32 (__m512i __A, __mmask16 __B, __m512i __C, __m512i __D,
								int __E)
{
  return (__m512i)__builtin_ia32_vpshld_v16si_mask ((__v16si)__C,
			(__v16si) __D, __E, (__v16si) __A, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shldi_epi32 (__mmask16 __A, __m512i __B, __m512i __C, int __D)
{
  return (__m512i)__builtin_ia32_vpshld_v16si_mask ((__v16si)__B,
	(__v16si) __C, __D, (__v16si) _mm512_setzero_si512 (), (__mmask16)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shldi_epi64 (__m512i __A, __m512i __B, int __C)
{
  return (__m512i) __builtin_ia32_vpshld_v8di ((__v8di)__A, (__v8di) __B, __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shldi_epi64 (__m512i __A, __mmask8 __B, __m512i __C, __m512i __D,
								int __E)
{
  return (__m512i)__builtin_ia32_vpshld_v8di_mask ((__v8di)__C, (__v8di) __D,
					__E, (__v8di) __A, (__mmask8)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shldi_epi64 (__mmask8 __A, __m512i __B, __m512i __C, int __D)
{
  return (__m512i)__builtin_ia32_vpshld_v8di_mask ((__v8di)__B, (__v8di) __C,
			__D, (__v8di) _mm512_setzero_si512 (), (__mmask8)__A);
}
#else
#define _mm512_shrdi_epi16(A, B, C) \
  ((__m512i) __builtin_ia32_vpshrd_v32hi ((__v32hi)(__m512i)(A), \
					  (__v32hi)(__m512i)(B),(int)(C)))
#define _mm512_shrdi_epi32(A, B, C) \
  ((__m512i) __builtin_ia32_vpshrd_v16si ((__v16si)(__m512i)(A), \
					  (__v16si)(__m512i)(B),(int)(C)))
#define _mm512_mask_shrdi_epi32(A, B, C, D, E) \
  ((__m512i) __builtin_ia32_vpshrd_v16si_mask ((__v16si)(__m512i)(C), \
					       (__v16si)(__m512i)(D), \
					       (int)(E),		\
					       (__v16si)(__m512i)(A),	\
					       (__mmask16)(B)))
#define _mm512_maskz_shrdi_epi32(A, B, C, D) \
  ((__m512i) \
   __builtin_ia32_vpshrd_v16si_mask ((__v16si)(__m512i)(B),		\
				     (__v16si)(__m512i)(C),(int)(D),	\
				     (__v16si)(__m512i)_mm512_setzero_si512 (), \
				     (__mmask16)(A)))
#define _mm512_shrdi_epi64(A, B, C) \
  ((__m512i) __builtin_ia32_vpshrd_v8di ((__v8di)(__m512i)(A), \
					 (__v8di)(__m512i)(B),(int)(C)))
#define _mm512_mask_shrdi_epi64(A, B, C, D, E) \
  ((__m512i) __builtin_ia32_vpshrd_v8di_mask ((__v8di)(__m512i)(C), \
					      (__v8di)(__m512i)(D), (int)(E), \
					      (__v8di)(__m512i)(A), \
					      (__mmask8)(B)))
#define _mm512_maskz_shrdi_epi64(A, B, C, D) \
  ((__m512i) \
   __builtin_ia32_vpshrd_v8di_mask ((__v8di)(__m512i)(B),		\
				    (__v8di)(__m512i)(C),(int)(D),	\
				    (__v8di)(__m512i)_mm512_setzero_si512 (), \
				    (__mmask8)(A)))
#define _mm512_shldi_epi16(A, B, C) \
  ((__m512i) __builtin_ia32_vpshld_v32hi ((__v32hi)(__m512i)(A), \
					  (__v32hi)(__m512i)(B),(int)(C)))
#define _mm512_shldi_epi32(A, B, C) \
  ((__m512i) __builtin_ia32_vpshld_v16si ((__v16si)(__m512i)(A),	\
					  (__v16si)(__m512i)(B),(int)(C)))
#define _mm512_mask_shldi_epi32(A, B, C, D, E) \
  ((__m512i) __builtin_ia32_vpshld_v16si_mask ((__v16si)(__m512i)(C), \
					       (__v16si)(__m512i)(D), \
					       (int)(E),		\
					       (__v16si)(__m512i)(A), \
					       (__mmask16)(B)))
#define _mm512_maskz_shldi_epi32(A, B, C, D) \
  ((__m512i) \
   __builtin_ia32_vpshld_v16si_mask ((__v16si)(__m512i)(B),		\
				     (__v16si)(__m512i)(C),(int)(D),	\
				     (__v16si)(__m512i)_mm512_setzero_si512 (), \
				     (__mmask16)(A)))
#define _mm512_shldi_epi64(A, B, C) \
  ((__m512i) __builtin_ia32_vpshld_v8di ((__v8di)(__m512i)(A), \
					 (__v8di)(__m512i)(B), (int)(C)))
#define _mm512_mask_shldi_epi64(A, B, C, D, E) \
  ((__m512i) __builtin_ia32_vpshld_v8di_mask ((__v8di)(__m512i)(C), \
					      (__v8di)(__m512i)(D), (int)(E), \
					      (__v8di)(__m512i)(A), \
					      (__mmask8)(B)))
#define _mm512_maskz_shldi_epi64(A, B, C, D) \
  ((__m512i) \
   __builtin_ia32_vpshld_v8di_mask ((__v8di)(__m512i)(B),		\
				    (__v8di)(__m512i)(C),(int)(D),	\
				    (__v8di)(__m512i)_mm512_setzero_si512 (), \
				    (__mmask8)(A)))
#endif

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shrdv_epi16 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpshrdv_v32hi ((__v32hi)__A, (__v32hi) __B,
								(__v32hi) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shrdv_epi32 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpshrdv_v16si ((__v16si)__A, (__v16si) __B,
								(__v16si) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shrdv_epi32 (__m512i __A, __mmask16 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshrdv_v16si_mask ((__v16si)__A,
				(__v16si) __C, (__v16si) __D, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shrdv_epi32 (__mmask16 __A, __m512i __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshrdv_v16si_maskz ((__v16si)__B,
				(__v16si) __C, (__v16si) __D, (__mmask16)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shrdv_epi64 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpshrdv_v8di ((__v8di)__A, (__v8di) __B,
								(__v8di) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shrdv_epi64 (__m512i __A, __mmask8 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshrdv_v8di_mask ((__v8di)__A, (__v8di) __C,
						(__v8di) __D, (__mmask8)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shrdv_epi64 (__mmask8 __A, __m512i __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshrdv_v8di_maskz ((__v8di)__B, (__v8di) __C,
						 (__v8di) __D, (__mmask8)__A);
}
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shldv_epi16 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpshldv_v32hi ((__v32hi)__A, (__v32hi) __B,
							 (__v32hi) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shldv_epi32 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpshldv_v16si ((__v16si)__A, (__v16si) __B,
								(__v16si) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shldv_epi32 (__m512i __A, __mmask16 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshldv_v16si_mask ((__v16si)__A,
				(__v16si) __C, (__v16si) __D, (__mmask16)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shldv_epi32 (__mmask16 __A, __m512i __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshldv_v16si_maskz ((__v16si)__B,
				(__v16si) __C, (__v16si) __D, (__mmask16)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shldv_epi64 (__m512i __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vpshldv_v8di ((__v8di)__A, (__v8di) __B,
								(__v8di) __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shldv_epi64 (__m512i __A, __mmask8 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshldv_v8di_mask ((__v8di)__A, (__v8di) __C,
						(__v8di) __D, (__mmask8)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shldv_epi64 (__mmask8 __A, __m512i __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshldv_v8di_maskz ((__v8di)__B, (__v8di) __C,
						(__v8di) __D, (__mmask8)__A);
}

#ifdef __DISABLE_AVX512VBMI2__
#undef __DISABLE_AVX512VBMI2__

#pragma GCC pop_options
#endif /* __DISABLE_AVX512VBMI2__ */

#if !defined(__AVX512VBMI2__) || !defined(__AVX512BW__)
#pragma GCC push_options
#pragma GCC target("avx512vbmi2,avx512bw")
#define __DISABLE_AVX512VBMI2BW__
#endif /* __AVX512VBMI2BW__ */

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compress_epi8 (__m512i __A, __mmask64 __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_compressqi512_mask ((__v64qi)__C,
						(__v64qi)__A, (__mmask64)__B);
}


extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_compress_epi8 (__mmask64 __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_compressqi512_mask ((__v64qi)__B,
			(__v64qi)_mm512_setzero_si512 (), (__mmask64)__A);
}


extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compressstoreu_epi8 (void * __A, __mmask64 __B, __m512i __C)
{
  __builtin_ia32_compressstoreuqi512_mask ((__v64qi *) __A, (__v64qi) __C,
							(__mmask64) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compress_epi16 (__m512i __A, __mmask32 __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_compresshi512_mask ((__v32hi)__C,
						(__v32hi)__A, (__mmask32)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_compress_epi16 (__mmask32 __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_compresshi512_mask ((__v32hi)__B,
			(__v32hi)_mm512_setzero_si512 (), (__mmask32)__A);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compressstoreu_epi16 (void * __A, __mmask32 __B, __m512i __C)
{
  __builtin_ia32_compressstoreuhi512_mask ((__v32hi *) __A, (__v32hi) __C,
							(__mmask32) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expand_epi8 (__m512i __A, __mmask64 __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_expandqi512_mask ((__v64qi) __C,
						    (__v64qi) __A,
						    (__mmask64) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expand_epi8 (__mmask64 __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_expandqi512_maskz ((__v64qi) __B,
			(__v64qi) _mm512_setzero_si512 (), (__mmask64) __A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expandloadu_epi8 (__m512i __A, __mmask64 __B, const void * __C)
{
  return (__m512i) __builtin_ia32_expandloadqi512_mask ((const __v64qi *) __C,
					(__v64qi) __A, (__mmask64) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expandloadu_epi8 (__mmask64 __A, const void * __B)
{
  return (__m512i) __builtin_ia32_expandloadqi512_maskz ((const __v64qi *) __B,
			(__v64qi) _mm512_setzero_si512 (), (__mmask64) __A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expand_epi16 (__m512i __A, __mmask32 __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_expandhi512_mask ((__v32hi) __C,
						    (__v32hi) __A,
						    (__mmask32) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expand_epi16 (__mmask32 __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_expandhi512_maskz ((__v32hi) __B,
			(__v32hi) _mm512_setzero_si512 (), (__mmask32) __A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expandloadu_epi16 (__m512i __A, __mmask32 __B, const void * __C)
{
  return (__m512i) __builtin_ia32_expandloadhi512_mask ((const __v32hi *) __C,
					(__v32hi) __A, (__mmask32) __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expandloadu_epi16 (__mmask32 __A, const void * __B)
{
  return (__m512i) __builtin_ia32_expandloadhi512_maskz ((const __v32hi *) __B,
			(__v32hi) _mm512_setzero_si512 (), (__mmask32) __A);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shrdi_epi16 (__m512i __A, __mmask32 __B, __m512i __C, __m512i __D,
								int __E)
{
  return (__m512i)__builtin_ia32_vpshrd_v32hi_mask ((__v32hi)__C,
			(__v32hi) __D, __E, (__v32hi) __A, (__mmask32)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shrdi_epi16 (__mmask32 __A, __m512i __B, __m512i __C, int __D)
{
  return (__m512i)__builtin_ia32_vpshrd_v32hi_mask ((__v32hi)__B,
	(__v32hi) __C, __D, (__v32hi) _mm512_setzero_si512 (), (__mmask32)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shldi_epi16 (__m512i __A, __mmask32 __B, __m512i __C, __m512i __D,
								int __E)
{
  return (__m512i)__builtin_ia32_vpshld_v32hi_mask ((__v32hi)__C,
			(__v32hi) __D, __E, (__v32hi) __A, (__mmask32)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shldi_epi16 (__mmask32 __A, __m512i __B, __m512i __C, int __D)
{
  return (__m512i)__builtin_ia32_vpshld_v32hi_mask ((__v32hi)__B,
	(__v32hi) __C, __D, (__v32hi) _mm512_setzero_si512 (), (__mmask32)__A);
}

#else
#define _mm512_mask_shrdi_epi16(A, B, C, D, E) \
  ((__m512i) __builtin_ia32_vpshrd_v32hi_mask ((__v32hi)(__m512i)(C), \
					       (__v32hi)(__m512i)(D), \
					       (int)(E),		\
					       (__v32hi)(__m512i)(A),	\
					       (__mmask32)(B)))
#define _mm512_maskz_shrdi_epi16(A, B, C, D) \
  ((__m512i) \
   __builtin_ia32_vpshrd_v32hi_mask ((__v32hi)(__m512i)(B),		\
				     (__v32hi)(__m512i)(C),(int)(D),	\
				     (__v32hi)(__m512i)_mm512_setzero_si512 (), \
				     (__mmask32)(A)))
#define _mm512_mask_shldi_epi16(A, B, C, D, E) \
  ((__m512i) __builtin_ia32_vpshld_v32hi_mask ((__v32hi)(__m512i)(C), \
					       (__v32hi)(__m512i)(D), \
					       (int)(E), \
					       (__v32hi)(__m512i)(A),	\
					       (__mmask32)(B)))
#define _mm512_maskz_shldi_epi16(A, B, C, D) \
  ((__m512i) \
   __builtin_ia32_vpshld_v32hi_mask ((__v32hi)(__m512i)(B),		\
				     (__v32hi)(__m512i)(C),(int)(D),	\
				     (__v32hi)(__m512i)_mm512_setzero_si512 (), \
				     (__mmask32)(A)))
#endif

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shrdv_epi16 (__m512i __A, __mmask32 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshrdv_v32hi_mask ((__v32hi)__A,
				(__v32hi) __C, (__v32hi) __D, (__mmask32)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shrdv_epi16 (__mmask32 __A, __m512i __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshrdv_v32hi_maskz ((__v32hi)__B,
				(__v32hi) __C, (__v32hi) __D, (__mmask32)__A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shldv_epi16 (__m512i __A, __mmask32 __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshldv_v32hi_mask ((__v32hi)__A,
				(__v32hi) __C, (__v32hi) __D, (__mmask32)__B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shldv_epi16 (__mmask32 __A, __m512i __B, __m512i __C, __m512i __D)
{
  return (__m512i)__builtin_ia32_vpshldv_v32hi_maskz ((__v32hi)__B,
				(__v32hi) __C, (__v32hi) __D, (__mmask32)__A);
}

#ifdef __DISABLE_AVX512VBMI2BW__
#undef __DISABLE_AVX512VBMI2BW__

#pragma GCC pop_options
#endif /* __DISABLE_AVX512VBMI2BW__ */

#endif /* __AVX512VBMI2INTRIN_H_INCLUDED */
