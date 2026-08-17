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

#ifndef _IMMINTRIN_H_INCLUDED
#error "Never use <gfniintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _GFNIINTRIN_H_INCLUDED
#define _GFNIINTRIN_H_INCLUDED

#if !defined(__GFNI__) || !defined(__SSE2__)
#pragma GCC push_options
#pragma GCC target("gfni,sse2")
#define __DISABLE_GFNI__
#endif /* __GFNI__ */

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_gf2p8mul_epi8 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vgf2p8mulb_v16qi((__v16qi) __A,
						   (__v16qi) __B);
}

#ifdef __OPTIMIZE__
extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_gf2p8affineinv_epi64_epi8 (__m128i __A, __m128i __B, const int __C)
{
  return (__m128i) __builtin_ia32_vgf2p8affineinvqb_v16qi ((__v16qi) __A,
							   (__v16qi) __B,
							    __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_gf2p8affine_epi64_epi8 (__m128i __A, __m128i __B, const int __C)
{
  return (__m128i) __builtin_ia32_vgf2p8affineqb_v16qi ((__v16qi) __A,
							(__v16qi) __B, __C);
}
#else
#define _mm_gf2p8affineinv_epi64_epi8(A, B, C)				   \
  ((__m128i) __builtin_ia32_vgf2p8affineinvqb_v16qi((__v16qi)(__m128i)(A), \
					   (__v16qi)(__m128i)(B), (int)(C)))
#define _mm_gf2p8affine_epi64_epi8(A, B, C)				   \
  ((__m128i) __builtin_ia32_vgf2p8affineqb_v16qi ((__v16qi)(__m128i)(A),   \
					   (__v16qi)(__m128i)(B), (int)(C)))
#endif

#ifdef __DISABLE_GFNI__
#undef __DISABLE_GFNI__
#pragma GCC pop_options
#endif /* __DISABLE_GFNI__ */

#if !defined(__GFNI__) || !defined(__AVX__)
#pragma GCC push_options
#pragma GCC target("gfni,avx")
#define __DISABLE_GFNIAVX__
#endif /* __GFNIAVX__ */

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_gf2p8mul_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_vgf2p8mulb_v32qi ((__v32qi) __A,
						    (__v32qi) __B);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_gf2p8affineinv_epi64_epi8 (__m256i __A, __m256i __B, const int __C)
{
  return (__m256i) __builtin_ia32_vgf2p8affineinvqb_v32qi ((__v32qi) __A,
							   (__v32qi) __B,
							    __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_gf2p8affine_epi64_epi8 (__m256i __A, __m256i __B, const int __C)
{
  return (__m256i) __builtin_ia32_vgf2p8affineqb_v32qi ((__v32qi) __A,
							(__v32qi) __B, __C);
}
#else
#define _mm256_gf2p8affineinv_epi64_epi8(A, B, C)			   \
  ((__m256i) __builtin_ia32_vgf2p8affineinvqb_v32qi((__v32qi)(__m256i)(A), \
						    (__v32qi)(__m256i)(B), \
						    (int)(C)))
#define _mm256_gf2p8affine_epi64_epi8(A, B, C)				   \
  ((__m256i) __builtin_ia32_vgf2p8affineqb_v32qi ((__v32qi)(__m256i)(A),   \
					(   __v32qi)(__m256i)(B), (int)(C)))
#endif

#ifdef __DISABLE_GFNIAVX__
#undef __DISABLE_GFNIAVX__
#pragma GCC pop_options
#endif /* __GFNIAVX__ */

#if !defined(__GFNI__) || !defined(__AVX512VL__)
#pragma GCC push_options
#pragma GCC target("gfni,avx512vl")
#define __DISABLE_GFNIAVX512VL__
#endif /* __GFNIAVX512VL__ */

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_gf2p8mul_epi8 (__m128i __A, __mmask16 __B, __m128i __C, __m128i __D)
{
  return (__m128i) __builtin_ia32_vgf2p8mulb_v16qi_mask ((__v16qi) __C,
							 (__v16qi) __D,
							 (__v16qi)__A, __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_gf2p8mul_epi8 (__mmask16 __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vgf2p8mulb_v16qi_mask ((__v16qi) __B,
			(__v16qi) __C, (__v16qi) _mm_setzero_si128 (), __A);
}

#ifdef __OPTIMIZE__
extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_gf2p8affineinv_epi64_epi8 (__m128i __A, __mmask16 __B, __m128i __C,
				    __m128i __D, const int __E)
{
  return (__m128i) __builtin_ia32_vgf2p8affineinvqb_v16qi_mask ((__v16qi) __C,
								(__v16qi) __D,
								 __E,
								(__v16qi)__A,
								 __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_gf2p8affineinv_epi64_epi8 (__mmask16 __A, __m128i __B, __m128i __C,
				     const int __D)
{
  return (__m128i) __builtin_ia32_vgf2p8affineinvqb_v16qi_mask ((__v16qi) __B,
						(__v16qi) __C, __D,
						(__v16qi) _mm_setzero_si128 (),
						 __A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_gf2p8affine_epi64_epi8 (__m128i __A, __mmask16 __B, __m128i __C,
				 __m128i __D, const int __E)
{
  return (__m128i) __builtin_ia32_vgf2p8affineqb_v16qi_mask ((__v16qi) __C,
					(__v16qi) __D, __E, (__v16qi)__A, __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_gf2p8affine_epi64_epi8 (__mmask16 __A, __m128i __B, __m128i __C,
				  const int __D)
{
  return (__m128i) __builtin_ia32_vgf2p8affineqb_v16qi_mask ((__v16qi) __B,
		     (__v16qi) __C, __D, (__v16qi) _mm_setzero_si128 (), __A);
}
#else
#define _mm_mask_gf2p8affineinv_epi64_epi8(A, B, C, D, E) 		   \
  ((__m128i) __builtin_ia32_vgf2p8affineinvqb_v16qi_mask(		   \
			(__v16qi)(__m128i)(C), (__v16qi)(__m128i)(D),      \
			(int)(E), (__v16qi)(__m128i)(A), (__mmask16)(B)))
#define _mm_maskz_gf2p8affineinv_epi64_epi8(A, B, C, D) \
  ((__m128i) __builtin_ia32_vgf2p8affineinvqb_v16qi_mask(		   \
			(__v16qi)(__m128i)(B), (__v16qi)(__m128i)(C),	   \
			(int)(D), (__v16qi)(__m128i) _mm_setzero_si128 (), \
			(__mmask16)(A)))
#define _mm_mask_gf2p8affine_epi64_epi8(A, B, C, D, E) \
  ((__m128i) __builtin_ia32_vgf2p8affineqb_v16qi_mask((__v16qi)(__m128i)(C),\
      (__v16qi)(__m128i)(D), (int)(E), (__v16qi)(__m128i)(A), (__mmask16)(B)))
#define _mm_maskz_gf2p8affine_epi64_epi8(A, B, C, D)			    \
  ((__m128i) __builtin_ia32_vgf2p8affineqb_v16qi_mask((__v16qi)(__m128i)(B),\
		(__v16qi)(__m128i)(C), (int)(D),			    \
		(__v16qi)(__m128i) _mm_setzero_si128 (), (__mmask16)(A)))
#endif

#ifdef __DISABLE_GFNIAVX512VL__
#undef __DISABLE_GFNIAVX512VL__
#pragma GCC pop_options
#endif /* __GFNIAVX512VL__ */

#if !defined(__GFNI__) || !defined(__AVX512VL__) || !defined(__AVX512BW__)
#pragma GCC push_options
#pragma GCC target("gfni,avx512vl,avx512bw")
#define __DISABLE_GFNIAVX512VLBW__
#endif /* __GFNIAVX512VLBW__ */

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_gf2p8mul_epi8 (__m256i __A, __mmask32 __B, __m256i __C,
			   __m256i __D)
{
  return (__m256i) __builtin_ia32_vgf2p8mulb_v32qi_mask ((__v32qi) __C,
							 (__v32qi) __D,
							 (__v32qi)__A, __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_gf2p8mul_epi8 (__mmask32 __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vgf2p8mulb_v32qi_mask ((__v32qi) __B,
			(__v32qi) __C, (__v32qi) _mm256_setzero_si256 (), __A);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_gf2p8affineinv_epi64_epi8 (__m256i __A, __mmask32 __B,
				       __m256i __C, __m256i __D, const int __E)
{
  return (__m256i) __builtin_ia32_vgf2p8affineinvqb_v32qi_mask ((__v32qi) __C,
								(__v32qi) __D,
							 	 __E,
								(__v32qi)__A,
								 __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_gf2p8affineinv_epi64_epi8 (__mmask32 __A, __m256i __B,
					__m256i __C, const int __D)
{
  return (__m256i) __builtin_ia32_vgf2p8affineinvqb_v32qi_mask ((__v32qi) __B,
				      (__v32qi) __C, __D,
				      (__v32qi) _mm256_setzero_si256 (), __A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_gf2p8affine_epi64_epi8 (__m256i __A, __mmask32 __B, __m256i __C,
				    __m256i __D, const int __E)
{
  return (__m256i) __builtin_ia32_vgf2p8affineqb_v32qi_mask ((__v32qi) __C,
							     (__v32qi) __D,
							      __E,
							     (__v32qi)__A,
							      __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_gf2p8affine_epi64_epi8 (__mmask32 __A, __m256i __B,
				     __m256i __C, const int __D)
{
  return (__m256i) __builtin_ia32_vgf2p8affineqb_v32qi_mask ((__v32qi) __B,
		(__v32qi) __C, __D, (__v32qi)_mm256_setzero_si256 (), __A);
}
#else
#define _mm256_mask_gf2p8affineinv_epi64_epi8(A, B, C, D, E)		\
  ((__m256i) __builtin_ia32_vgf2p8affineinvqb_v32qi_mask(		\
	(__v32qi)(__m256i)(C), (__v32qi)(__m256i)(D), (int)(E),		\
	(__v32qi)(__m256i)(A), (__mmask32)(B)))
#define _mm256_maskz_gf2p8affineinv_epi64_epi8(A, B, C, D)		\
  ((__m256i) __builtin_ia32_vgf2p8affineinvqb_v32qi_mask(		\
	(__v32qi)(__m256i)(B), (__v32qi)(__m256i)(C), (int)(D),		\
	(__v32qi)(__m256i) _mm256_setzero_si256 (), (__mmask32)(A)))
#define _mm256_mask_gf2p8affine_epi64_epi8(A, B, C, D, E) 		    \
  ((__m256i) __builtin_ia32_vgf2p8affineqb_v32qi_mask((__v32qi)(__m256i)(C),\
	(__v32qi)(__m256i)(D), (int)(E), (__v32qi)(__m256i)(A), (__mmask32)(B)))
#define _mm256_maskz_gf2p8affine_epi64_epi8(A, B, C, D)			    \
  ((__m256i) __builtin_ia32_vgf2p8affineqb_v32qi_mask((__v32qi)(__m256i)(B),\
	 (__v32qi)(__m256i)(C), (int)(D),				    \
	 (__v32qi)(__m256i) _mm256_setzero_si256 (), (__mmask32)(A)))
#endif

#ifdef __DISABLE_GFNIAVX512VLBW__
#undef __DISABLE_GFNIAVX512VLBW__
#pragma GCC pop_options
#endif /* __GFNIAVX512VLBW__ */

#if !defined(__GFNI__) || !defined(__AVX512F__) || !defined(__AVX512BW__)
#pragma GCC push_options
#pragma GCC target("gfni,avx512f,avx512bw")
#define __DISABLE_GFNIAVX512FBW__
#endif /* __GFNIAVX512FBW__ */

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_gf2p8mul_epi8 (__m512i __A, __mmask64 __B, __m512i __C,
			   __m512i __D)
{
  return (__m512i) __builtin_ia32_vgf2p8mulb_v64qi_mask ((__v64qi) __C,
					(__v64qi) __D, (__v64qi)__A, __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_gf2p8mul_epi8 (__mmask64 __A, __m512i __B, __m512i __C)
{
  return (__m512i) __builtin_ia32_vgf2p8mulb_v64qi_mask ((__v64qi) __B,
			(__v64qi) __C, (__v64qi) _mm512_setzero_si512 (), __A);
}
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_gf2p8mul_epi8 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_vgf2p8mulb_v64qi ((__v64qi) __A,
						    (__v64qi) __B);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_gf2p8affineinv_epi64_epi8 (__m512i __A, __mmask64 __B, __m512i __C,
				       __m512i __D, const int __E)
{
  return (__m512i) __builtin_ia32_vgf2p8affineinvqb_v64qi_mask ((__v64qi) __C,
								(__v64qi) __D,
								 __E,
								(__v64qi)__A,
								 __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_gf2p8affineinv_epi64_epi8 (__mmask64 __A, __m512i __B,
					__m512i __C, const int __D)
{
  return (__m512i) __builtin_ia32_vgf2p8affineinvqb_v64qi_mask ((__v64qi) __B,
				(__v64qi) __C, __D,
				(__v64qi) _mm512_setzero_si512 (), __A);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_gf2p8affineinv_epi64_epi8 (__m512i __A, __m512i __B, const int __C)
{
  return (__m512i) __builtin_ia32_vgf2p8affineinvqb_v64qi ((__v64qi) __A,
							   (__v64qi) __B, __C);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_gf2p8affine_epi64_epi8 (__m512i __A, __mmask64 __B, __m512i __C,
				    __m512i __D, const int __E)
{
  return (__m512i) __builtin_ia32_vgf2p8affineqb_v64qi_mask ((__v64qi) __C,
					(__v64qi) __D, __E, (__v64qi)__A, __B);
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_gf2p8affine_epi64_epi8 (__mmask64 __A, __m512i __B, __m512i __C,
				     const int __D)
{
  return (__m512i) __builtin_ia32_vgf2p8affineqb_v64qi_mask ((__v64qi) __B,
		  (__v64qi) __C, __D, (__v64qi) _mm512_setzero_si512 (), __A);
}
extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_gf2p8affine_epi64_epi8 (__m512i __A, __m512i __B, const int __C)
{
  return (__m512i) __builtin_ia32_vgf2p8affineqb_v64qi ((__v64qi) __A,
							(__v64qi) __B, __C);
}
#else
#define _mm512_mask_gf2p8affineinv_epi64_epi8(A, B, C, D, E) 		\
  ((__m512i) __builtin_ia32_vgf2p8affineinvqb_v64qi_mask(		\
	(__v64qi)(__m512i)(C), (__v64qi)(__m512i)(D), (int)(E),		\
	(__v64qi)(__m512i)(A), (__mmask64)(B)))
#define _mm512_maskz_gf2p8affineinv_epi64_epi8(A, B, C, D)		\
  ((__m512i) __builtin_ia32_vgf2p8affineinvqb_v64qi_mask(		\
	(__v64qi)(__m512i)(B), (__v64qi)(__m512i)(C), (int)(D),		\
	(__v64qi)(__m512i) _mm512_setzero_si512 (), (__mmask64)(A)))
#define _mm512_gf2p8affineinv_epi64_epi8(A, B, C)			\
  ((__m512i) __builtin_ia32_vgf2p8affineinvqb_v64qi (			\
	(__v64qi)(__m512i)(A), (__v64qi)(__m512i)(B), (int)(C)))
#define _mm512_mask_gf2p8affine_epi64_epi8(A, B, C, D, E)		    \
  ((__m512i) __builtin_ia32_vgf2p8affineqb_v64qi_mask((__v64qi)(__m512i)(C),\
     (__v64qi)(__m512i)(D), (int)(E), (__v64qi)(__m512i)(A), (__mmask64)(B)))
#define _mm512_maskz_gf2p8affine_epi64_epi8(A, B, C, D)			    \
  ((__m512i) __builtin_ia32_vgf2p8affineqb_v64qi_mask((__v64qi)(__m512i)(B),\
	 (__v64qi)(__m512i)(C), (int)(D),				    \
	 (__v64qi)(__m512i) _mm512_setzero_si512 (), (__mmask64)(A)))
#define _mm512_gf2p8affine_epi64_epi8(A, B, C)				    \
  ((__m512i) __builtin_ia32_vgf2p8affineqb_v64qi ((__v64qi)(__m512i)(A),    \
	 (__v64qi)(__m512i)(B), (int)(C)))
#endif

#ifdef __DISABLE_GFNIAVX512FBW__
#undef __DISABLE_GFNIAVX512FBW__
#pragma GCC pop_options
#endif /* __GFNIAVX512FBW__ */

#endif /* _GFNIINTRIN_H_INCLUDED */
