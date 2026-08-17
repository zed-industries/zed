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
#error "Never use <avx512vbmi2vlintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512VBMI2VLINTRIN_H_INCLUDED
#define _AVX512VBMI2VLINTRIN_H_INCLUDED

#if !defined(__AVX512VL__) || !defined(__AVX512VBMI2__)
#pragma GCC push_options
#pragma GCC target("avx512vbmi2,avx512vl")
#define __DISABLE_AVX512VBMI2VL__
#endif /* __AVX512VBMIVL__ */

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compress_epi8 (__m128i __A, __mmask16 __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_compressqi128_mask ((__v16qi)__C,
						(__v16qi)__A, (__mmask16)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_compress_epi8 (__mmask16 __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_compressqi128_mask ((__v16qi) __B,
			(__v16qi) _mm_setzero_si128 (), (__mmask16) __A);
}


extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compressstoreu_epi16 (void * __A, __mmask16 __B, __m256i __C)
{
  __builtin_ia32_compressstoreuhi256_mask ((__v16hi *) __A, (__v16hi) __C,
							(__mmask16) __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compress_epi16 (__m128i __A, __mmask8 __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_compresshi128_mask ((__v8hi)__C, (__v8hi)__A,
								(__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_compress_epi16 (__mmask8 __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_compresshi128_mask ((__v8hi) __B,
				(__v8hi) _mm_setzero_si128 (), (__mmask8) __A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compress_epi16 (__m256i __A, __mmask16 __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_compresshi256_mask ((__v16hi)__C,
						(__v16hi)__A, (__mmask16)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_compress_epi16 (__mmask16 __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_compresshi256_mask ((__v16hi) __B,
			(__v16hi) _mm256_setzero_si256 (), (__mmask16) __A);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compressstoreu_epi8 (void * __A, __mmask16 __B, __m128i __C)
{
  __builtin_ia32_compressstoreuqi128_mask ((__v16qi *) __A, (__v16qi) __C,
							(__mmask16) __B);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compressstoreu_epi16 (void * __A, __mmask8 __B, __m128i __C)
{
  __builtin_ia32_compressstoreuhi128_mask ((__v8hi *) __A, (__v8hi) __C,
							(__mmask8) __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expand_epi8 (__m128i __A, __mmask16 __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_expandqi128_mask ((__v16qi) __C,
						    (__v16qi) __A,
						    (__mmask16) __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expand_epi8 (__mmask16 __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_expandqi128_maskz ((__v16qi) __B,
			(__v16qi) _mm_setzero_si128 (), (__mmask16) __A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expandloadu_epi8 (__m128i __A, __mmask16 __B, const void * __C)
{
  return (__m128i) __builtin_ia32_expandloadqi128_mask ((const __v16qi *) __C,
					(__v16qi) __A, (__mmask16) __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expandloadu_epi8 (__mmask16 __A, const void * __B)
{
  return (__m128i) __builtin_ia32_expandloadqi128_maskz ((const __v16qi *) __B,
			(__v16qi) _mm_setzero_si128 (), (__mmask16) __A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expand_epi16 (__m128i __A, __mmask8 __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_expandhi128_mask ((__v8hi) __C,
						    (__v8hi) __A,
						    (__mmask8) __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expand_epi16 (__mmask8 __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_expandhi128_maskz ((__v8hi) __B,
				(__v8hi) _mm_setzero_si128 (), (__mmask8) __A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expandloadu_epi16 (__m128i __A, __mmask8 __B, const void * __C)
{
  return (__m128i) __builtin_ia32_expandloadhi128_mask ((const __v8hi *) __C,
						(__v8hi) __A, (__mmask8) __B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expandloadu_epi16 (__mmask8 __A, const void * __B)
{
  return (__m128i) __builtin_ia32_expandloadhi128_maskz ((const __v8hi *) __B,
				(__v8hi) _mm_setzero_si128 (), (__mmask8) __A);
}
extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expand_epi16 (__m256i __A, __mmask16 __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_expandhi256_mask ((__v16hi) __C,
						    (__v16hi) __A,
						    (__mmask16) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expand_epi16 (__mmask16 __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_expandhi256_maskz ((__v16hi) __B,
			(__v16hi) _mm256_setzero_si256 (), (__mmask16) __A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expandloadu_epi16 (__m256i __A, __mmask16 __B, const void * __C)
{
  return (__m256i) __builtin_ia32_expandloadhi256_mask ((const __v16hi *) __C,
					(__v16hi) __A, (__mmask16) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expandloadu_epi16 (__mmask16 __A, const void * __B)
{
  return (__m256i) __builtin_ia32_expandloadhi256_maskz ((const __v16hi *) __B,
			(__v16hi) _mm256_setzero_si256 (), (__mmask16) __A);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shrdi_epi16 (__m256i __A, __m256i __B, int __C)
{
  return (__m256i) __builtin_ia32_vpshrd_v16hi ((__v16hi)__A, (__v16hi) __B,
									__C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shrdi_epi16 (__m256i __A, __mmask16 __B, __m256i __C, __m256i __D,
								int __E)
{
  return (__m256i)__builtin_ia32_vpshrd_v16hi_mask ((__v16hi)__C,
			(__v16hi) __D, __E, (__v16hi) __A, (__mmask16)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shrdi_epi16 (__mmask16 __A, __m256i __B, __m256i __C, int __D)
{
  return (__m256i)__builtin_ia32_vpshrd_v16hi_mask ((__v16hi)__B,
	(__v16hi) __C, __D, (__v16hi) _mm256_setzero_si256 (), (__mmask16)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shrdi_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D,
								int __E)
{
  return (__m256i)__builtin_ia32_vpshrd_v8si_mask ((__v8si)__C, (__v8si) __D,
					__E, (__v8si) __A, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shrdi_epi32 (__mmask8 __A, __m256i __B, __m256i __C, int __D)
{
  return (__m256i)__builtin_ia32_vpshrd_v8si_mask ((__v8si)__B, (__v8si) __C,
			__D, (__v8si) _mm256_setzero_si256 (), (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shrdi_epi32 (__m256i __A, __m256i __B, int __C)
{
  return (__m256i) __builtin_ia32_vpshrd_v8si ((__v8si)__A, (__v8si) __B, __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shrdi_epi64 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D,
								int __E)
{
  return (__m256i)__builtin_ia32_vpshrd_v4di_mask ((__v4di)__C, (__v4di) __D,
					__E, (__v4di) __A, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shrdi_epi64 (__mmask8 __A, __m256i __B, __m256i __C, int __D)
{
  return (__m256i)__builtin_ia32_vpshrd_v4di_mask ((__v4di)__B, (__v4di) __C,
			__D, (__v4di) _mm256_setzero_si256 (), (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shrdi_epi64 (__m256i __A, __m256i __B, int __C)
{
  return (__m256i) __builtin_ia32_vpshrd_v4di ((__v4di)__A, (__v4di) __B, __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shrdi_epi16 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D,
								int __E)
{
  return (__m128i)__builtin_ia32_vpshrd_v8hi_mask ((__v8hi)__C, (__v8hi) __D,
					__E, (__v8hi) __A, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shrdi_epi16 (__mmask8 __A, __m128i __B, __m128i __C, int __D)
{
  return (__m128i)__builtin_ia32_vpshrd_v8hi_mask ((__v8hi)__B, (__v8hi) __C,
			__D, (__v8hi) _mm_setzero_si128 (), (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shrdi_epi16 (__m128i __A, __m128i __B, int __C)
{
  return (__m128i) __builtin_ia32_vpshrd_v8hi ((__v8hi)__A, (__v8hi) __B, __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shrdi_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D,
								int __E)
{
  return (__m128i)__builtin_ia32_vpshrd_v4si_mask ((__v4si)__C, (__v4si) __D,
					__E, (__v4si) __A, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shrdi_epi32 (__mmask8 __A, __m128i __B, __m128i __C, int __D)
{
  return (__m128i)__builtin_ia32_vpshrd_v4si_mask ((__v4si)__B, (__v4si) __C,
			__D, (__v4si) _mm_setzero_si128 (), (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shrdi_epi32 (__m128i __A, __m128i __B, int __C)
{
  return (__m128i) __builtin_ia32_vpshrd_v4si ((__v4si)__A, (__v4si) __B, __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shrdi_epi64 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D,
								int __E)
{
  return (__m128i)__builtin_ia32_vpshrd_v2di_mask ((__v2di)__C, (__v2di) __D,
					__E, (__v2di) __A, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shrdi_epi64 (__mmask8 __A, __m128i __B, __m128i __C, int __D)
{
  return (__m128i)__builtin_ia32_vpshrd_v2di_mask ((__v2di)__B, (__v2di) __C,
			__D, (__v2di) _mm_setzero_si128 (), (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shrdi_epi64 (__m128i __A, __m128i __B, int __C)
{
  return (__m128i) __builtin_ia32_vpshrd_v2di ((__v2di)__A, (__v2di) __B, __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shldi_epi16 (__m256i __A, __m256i __B, int __C)
{
  return (__m256i) __builtin_ia32_vpshld_v16hi ((__v16hi)__A, (__v16hi) __B,
									__C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shldi_epi16 (__m256i __A, __mmask16 __B, __m256i __C, __m256i __D,
								int __E)
{
  return (__m256i)__builtin_ia32_vpshld_v16hi_mask ((__v16hi)__C,
			(__v16hi) __D, __E, (__v16hi) __A, (__mmask16)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shldi_epi16 (__mmask16 __A, __m256i __B, __m256i __C, int __D)
{
  return (__m256i)__builtin_ia32_vpshld_v16hi_mask ((__v16hi)__B,
	(__v16hi) __C, __D, (__v16hi) _mm256_setzero_si256 (), (__mmask16)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shldi_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D,
								int __E)
{
  return (__m256i)__builtin_ia32_vpshld_v8si_mask ((__v8si)__C, (__v8si) __D,
					__E, (__v8si) __A, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shldi_epi32 (__mmask8 __A, __m256i __B, __m256i __C, int __D)
{
  return (__m256i)__builtin_ia32_vpshld_v8si_mask ((__v8si)__B, (__v8si) __C,
			__D, (__v8si) _mm256_setzero_si256 (), (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shldi_epi32 (__m256i __A, __m256i __B, int __C)
{
  return (__m256i) __builtin_ia32_vpshld_v8si ((__v8si)__A, (__v8si) __B, __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shldi_epi64 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D,
								int __E)
{
  return (__m256i)__builtin_ia32_vpshld_v4di_mask ((__v4di)__C, (__v4di) __D,
					__E, (__v4di) __A, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shldi_epi64 (__mmask8 __A, __m256i __B, __m256i __C, int __D)
{
  return (__m256i)__builtin_ia32_vpshld_v4di_mask ((__v4di)__B, (__v4di) __C,
			__D, (__v4di) _mm256_setzero_si256 (), (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shldi_epi64 (__m256i __A, __m256i __B, int __C)
{
  return (__m256i) __builtin_ia32_vpshld_v4di ((__v4di)__A, (__v4di) __B, __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shldi_epi16 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D,
								int __E)
{
  return (__m128i)__builtin_ia32_vpshld_v8hi_mask ((__v8hi)__C, (__v8hi) __D,
					__E, (__v8hi) __A, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shldi_epi16 (__mmask8 __A, __m128i __B, __m128i __C, int __D)
{
  return (__m128i)__builtin_ia32_vpshld_v8hi_mask ((__v8hi)__B, (__v8hi) __C,
			__D, (__v8hi) _mm_setzero_si128 (), (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shldi_epi16 (__m128i __A, __m128i __B, int __C)
{
  return (__m128i) __builtin_ia32_vpshld_v8hi ((__v8hi)__A, (__v8hi) __B, __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shldi_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D,
								int __E)
{
  return (__m128i)__builtin_ia32_vpshld_v4si_mask ((__v4si)__C, (__v4si) __D,
					__E, (__v4si) __A, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shldi_epi32 (__mmask8 __A, __m128i __B, __m128i __C, int __D)
{
  return (__m128i)__builtin_ia32_vpshld_v4si_mask ((__v4si)__B, (__v4si) __C,
			__D, (__v4si) _mm_setzero_si128 (), (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shldi_epi32 (__m128i __A, __m128i __B, int __C)
{
  return (__m128i) __builtin_ia32_vpshld_v4si ((__v4si)__A, (__v4si) __B, __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shldi_epi64 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D,
								int __E)
{
  return (__m128i)__builtin_ia32_vpshld_v2di_mask ((__v2di)__C, (__v2di) __D,
					__E, (__v2di) __A, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shldi_epi64 (__mmask8 __A, __m128i __B, __m128i __C, int __D)
{
  return (__m128i)__builtin_ia32_vpshld_v2di_mask ((__v2di)__B, (__v2di) __C,
			__D, (__v2di) _mm_setzero_si128 (), (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shldi_epi64 (__m128i __A, __m128i __B, int __C)
{
  return (__m128i) __builtin_ia32_vpshld_v2di ((__v2di)__A, (__v2di) __B, __C);
}
#else
#define _mm256_shrdi_epi16(A, B, C) \
  ((__m256i) __builtin_ia32_vpshrd_v16hi ((__v16hi)(__m256i)(A), \
					  (__v16hi)(__m256i)(B),(int)(C)))
#define _mm256_mask_shrdi_epi16(A, B, C, D, E) \
  ((__m256i) __builtin_ia32_vpshrd_v16hi_mask ((__v16hi)(__m256i)(C), \
					       (__v16hi)(__m256i)(D), \
					       (int)(E),		\
					       (__v16hi)(__m256i)(A), \
					       (__mmask16)(B)))
#define _mm256_maskz_shrdi_epi16(A, B, C, D) \
  ((__m256i) \
   __builtin_ia32_vpshrd_v16hi_mask ((__v16hi)(__m256i)(B),		\
				     (__v16hi)(__m256i)(C),(int)(D),	\
				     (__v16hi)(__m256i)_mm256_setzero_si256 (), \
				     (__mmask16)(A)))
#define _mm256_shrdi_epi32(A, B, C) \
  ((__m256i) __builtin_ia32_vpshrd_v8si ((__v8si)(__m256i)(A), \
					 (__v8si)(__m256i)(B),(int)(C)))
#define _mm256_mask_shrdi_epi32(A, B, C, D, E) \
  ((__m256i) __builtin_ia32_vpshrd_v8si_mask ((__v8si)(__m256i)(C), \
					      (__v8si)(__m256i)(D), \
					      (int)(E), \
					      (__v8si)(__m256i)(A), \
					      (__mmask8)(B)))
#define _mm256_maskz_shrdi_epi32(A, B, C, D) \
  ((__m256i) \
   __builtin_ia32_vpshrd_v8si_mask ((__v8si)(__m256i)(B),		\
				    (__v8si)(__m256i)(C),(int)(D),	\
				    (__v8si)(__m256i)_mm256_setzero_si256 (), \
				    (__mmask8)(A)))
#define _mm256_shrdi_epi64(A, B, C) \
  ((__m256i) __builtin_ia32_vpshrd_v4di ((__v4di)(__m256i)(A), \
					 (__v4di)(__m256i)(B),(int)(C)))
#define _mm256_mask_shrdi_epi64(A, B, C, D, E) \
  ((__m256i) __builtin_ia32_vpshrd_v4di_mask ((__v4di)(__m256i)(C), \
					      (__v4di)(__m256i)(D), (int)(E), \
					      (__v4di)(__m256i)(A), \
					      (__mmask8)(B)))
#define _mm256_maskz_shrdi_epi64(A, B, C, D) \
  ((__m256i) \
   __builtin_ia32_vpshrd_v4di_mask ((__v4di)(__m256i)(B),		\
				    (__v4di)(__m256i)(C),(int)(D),	\
				    (__v4di)(__m256i)_mm256_setzero_si256 (), \
				    (__mmask8)(A)))
#define _mm_shrdi_epi16(A, B, C) \
  ((__m128i) __builtin_ia32_vpshrd_v8hi ((__v8hi)(__m128i)(A), \
					 (__v8hi)(__m128i)(B),(int)(C)))
#define _mm_mask_shrdi_epi16(A, B, C, D, E) \
  ((__m128i) __builtin_ia32_vpshrd_v8hi_mask ((__v8hi)(__m128i)(C), \
					      (__v8hi)(__m128i)(D), (int)(E), \
					      (__v8hi)(__m128i)(A), \
					      (__mmask8)(B)))
#define _mm_maskz_shrdi_epi16(A, B, C, D) \
  ((__m128i) \
   __builtin_ia32_vpshrd_v8hi_mask ((__v8hi)(__m128i)(B),		\
				    (__v8hi)(__m128i)(C),(int)(D),	\
				    (__v8hi)(__m128i)_mm_setzero_si128 (), \
				    (__mmask8)(A)))
#define _mm_shrdi_epi32(A, B, C) \
  ((__m128i) __builtin_ia32_vpshrd_v4si ((__v4si)(__m128i)(A), \
					 (__v4si)(__m128i)(B),(int)(C)))
#define _mm_mask_shrdi_epi32(A, B, C, D, E) \
  ((__m128i) __builtin_ia32_vpshrd_v4si_mask ((__v4si)(__m128i)(C),	\
					      (__v4si)(__m128i)(D), (int)(E), \
					      (__v4si)(__m128i)(A), \
					      (__mmask8)(B)))
#define _mm_maskz_shrdi_epi32(A, B, C, D) \
  ((__m128i) \
   __builtin_ia32_vpshrd_v4si_mask ((__v4si)(__m128i)(B),		\
				    (__v4si)(__m128i)(C),(int)(D),	\
				    (__v4si)(__m128i)_mm_setzero_si128 (), \
				    (__mmask8)(A)))
#define _mm_shrdi_epi64(A, B, C) \
  ((__m128i) __builtin_ia32_vpshrd_v2di ((__v2di)(__m128i)(A), \
					 (__v2di)(__m128i)(B),(int)(C)))
#define _mm_mask_shrdi_epi64(A, B, C, D, E) \
  ((__m128i) __builtin_ia32_vpshrd_v2di_mask ((__v2di)(__m128i)(C), \
					      (__v2di)(__m128i)(D), (int)(E), \
					      (__v2di)(__m128i)(A), \
					      (__mmask8)(B)))
#define _mm_maskz_shrdi_epi64(A, B, C, D) \
  ((__m128i) \
   __builtin_ia32_vpshrd_v2di_mask ((__v2di)(__m128i)(B),		\
				    (__v2di)(__m128i)(C),(int)(D),	\
				    (__v2di)(__m128i)_mm_setzero_si128 (), \
				    (__mmask8)(A)))
#define _mm256_shldi_epi16(A, B, C) \
  ((__m256i) __builtin_ia32_vpshld_v16hi ((__v16hi)(__m256i)(A), \
					  (__v16hi)(__m256i)(B),(int)(C)))
#define _mm256_mask_shldi_epi16(A, B, C, D, E) \
  ((__m256i) __builtin_ia32_vpshld_v16hi_mask ((__v16hi)(__m256i)(C), \
					       (__v16hi)(__m256i)(D), \
					       (int)(E),		\
					       (__v16hi)(__m256i)(A), \
					       (__mmask16)(B)))
#define _mm256_maskz_shldi_epi16(A, B, C, D) \
  ((__m256i) \
   __builtin_ia32_vpshld_v16hi_mask ((__v16hi)(__m256i)(B),		\
				     (__v16hi)(__m256i)(C),(int)(D),	\
				     (__v16hi)(__m256i)_mm256_setzero_si256 (), \
				     (__mmask16)(A)))
#define _mm256_shldi_epi32(A, B, C) \
  ((__m256i) __builtin_ia32_vpshld_v8si ((__v8si)(__m256i)(A), \
					 (__v8si)(__m256i)(B),(int)(C)))
#define _mm256_mask_shldi_epi32(A, B, C, D, E) \
  ((__m256i) __builtin_ia32_vpshld_v8si_mask ((__v8si)(__m256i)(C), \
					      (__v8si)(__m256i)(D), (int)(E), \
					      (__v8si)(__m256i)(A), \
					      (__mmask8)(B)))
#define _mm256_maskz_shldi_epi32(A, B, C, D) \
  ((__m256i) \
   __builtin_ia32_vpshld_v8si_mask ((__v8si)(__m256i)(B),		\
				    (__v8si)(__m256i)(C),(int)(D),	\
				    (__v8si)(__m256i)_mm256_setzero_si256 (), \
				    (__mmask8)(A)))
#define _mm256_shldi_epi64(A, B, C) \
  ((__m256i) __builtin_ia32_vpshld_v4di ((__v4di)(__m256i)(A), \
					 (__v4di)(__m256i)(B),(int)(C)))
#define _mm256_mask_shldi_epi64(A, B, C, D, E) \
  ((__m256i) __builtin_ia32_vpshld_v4di_mask ((__v4di)(__m256i)(C), \
					      (__v4di)(__m256i)(D), (int)(E), \
					      (__v4di)(__m256i)(A), \
					      (__mmask8)(B)))
#define _mm256_maskz_shldi_epi64(A, B, C, D) \
  ((__m256i) \
   __builtin_ia32_vpshld_v4di_mask ((__v4di)(__m256i)(B),		\
				    (__v4di)(__m256i)(C),(int)(D),	\
				    (__v4di)(__m256i)_mm256_setzero_si256 (), \
				    (__mmask8)(A)))
#define _mm_shldi_epi16(A, B, C) \
  ((__m128i) __builtin_ia32_vpshld_v8hi ((__v8hi)(__m128i)(A), \
					 (__v8hi)(__m128i)(B),(int)(C)))
#define _mm_mask_shldi_epi16(A, B, C, D, E) \
  ((__m128i) __builtin_ia32_vpshld_v8hi_mask ((__v8hi)(__m128i)(C), \
					      (__v8hi)(__m128i)(D), (int)(E), \
					      (__v8hi)(__m128i)(A), \
					      (__mmask8)(B)))
#define _mm_maskz_shldi_epi16(A, B, C, D) \
  ((__m128i) \
   __builtin_ia32_vpshld_v8hi_mask ((__v8hi)(__m128i)(B),		\
				    (__v8hi)(__m128i)(C),(int)(D),	\
				    (__v8hi)(__m128i)_mm_setzero_si128 (), \
				    (__mmask8)(A)))
#define _mm_shldi_epi32(A, B, C) \
  ((__m128i) __builtin_ia32_vpshld_v4si ((__v4si)(__m128i)(A), \
					 (__v4si)(__m128i)(B),(int)(C)))
#define _mm_mask_shldi_epi32(A, B, C, D, E) \
  ((__m128i) __builtin_ia32_vpshld_v4si_mask ((__v4si)(__m128i)(C), \
					      (__v4si)(__m128i)(D), (int)(E), \
					      (__v4si)(__m128i)(A), \
					      (__mmask8)(B)))
#define _mm_maskz_shldi_epi32(A, B, C, D) \
  ((__m128i) \
   __builtin_ia32_vpshld_v4si_mask ((__v4si)(__m128i)(B),		\
				    (__v4si)(__m128i)(C),(int)(D),	\
				    (__v4si)(__m128i)_mm_setzero_si128 (), \
				    (__mmask8)(A)))
#define _mm_shldi_epi64(A, B, C) \
  ((__m128i) __builtin_ia32_vpshld_v2di ((__v2di)(__m128i)(A), \
					 (__v2di)(__m128i)(B),(int)(C)))
#define _mm_mask_shldi_epi64(A, B, C, D, E) \
  ((__m128i) __builtin_ia32_vpshld_v2di_mask ((__v2di)(__m128i)(C), \
					      (__v2di)(__m128i)(D), (int)(E), \
					      (__v2di)(__m128i)(A), \
					      (__mmask8)(B)))
#define _mm_maskz_shldi_epi64(A, B, C, D) \
  ((__m128i) \
   __builtin_ia32_vpshld_v2di_mask ((__v2di)(__m128i)(B),		\
				    (__v2di)(__m128i)(C),(int)(D),	\
				    (__v2di)(__m128i)_mm_setzero_si128 (), \
				    (__mmask8)(A)))
#endif

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shrdv_epi16 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpshrdv_v16hi ((__v16hi)__A, (__v16hi) __B,
								(__v16hi) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shrdv_epi16 (__m256i __A, __mmask16 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshrdv_v16hi_mask ((__v16hi)__A,
				(__v16hi) __C, (__v16hi) __D, (__mmask16)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shrdv_epi16 (__mmask16 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshrdv_v16hi_maskz ((__v16hi)__B,
				(__v16hi) __C, (__v16hi) __D, (__mmask16)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shrdv_epi32 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpshrdv_v8si ((__v8si)__A, (__v8si) __B,
								(__v8si) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shrdv_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshrdv_v8si_mask ((__v8si)__A, (__v8si) __C,
						(__v8si) __D, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shrdv_epi32 (__mmask8 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshrdv_v8si_maskz ((__v8si)__B, (__v8si) __C,
						 (__v8si) __D, (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shrdv_epi64 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpshrdv_v4di ((__v4di)__A, (__v4di) __B,
								(__v4di) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shrdv_epi64 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshrdv_v4di_mask ((__v4di)__A, (__v4di) __C,
						(__v4di) __D, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shrdv_epi64 (__mmask8 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshrdv_v4di_maskz ((__v4di)__B, (__v4di) __C,
						 (__v4di) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shrdv_epi16 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpshrdv_v8hi ((__v8hi)__A, (__v8hi) __B,
								(__v8hi) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shrdv_epi16 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshrdv_v8hi_mask ((__v8hi)__A, (__v8hi) __C,
						(__v8hi) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shrdv_epi16 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshrdv_v8hi_maskz ((__v8hi)__B, (__v8hi) __C,
						 (__v8hi) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shrdv_epi32 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpshrdv_v4si ((__v4si)__A, (__v4si) __B,
								(__v4si) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shrdv_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshrdv_v4si_mask ((__v4si)__A, (__v4si) __C,
						(__v4si) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shrdv_epi32 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshrdv_v4si_maskz ((__v4si)__B, (__v4si) __C,
						 (__v4si) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shrdv_epi64 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpshrdv_v2di ((__v2di)__A, (__v2di) __B,
								(__v2di) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shrdv_epi64 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshrdv_v2di_mask ((__v2di)__A, (__v2di) __C,
						(__v2di) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shrdv_epi64 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshrdv_v2di_maskz ((__v2di)__B, (__v2di) __C,
						 (__v2di) __D, (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shldv_epi16 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpshldv_v16hi ((__v16hi)__A, (__v16hi) __B,
								(__v16hi) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shldv_epi16 (__m256i __A, __mmask16 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshldv_v16hi_mask ((__v16hi)__A,
				(__v16hi) __C, (__v16hi) __D, (__mmask16)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shldv_epi16 (__mmask16 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshldv_v16hi_maskz ((__v16hi)__B,
				(__v16hi) __C, (__v16hi) __D, (__mmask16)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shldv_epi32 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpshldv_v8si ((__v8si)__A, (__v8si) __B,
								(__v8si) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shldv_epi32 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshldv_v8si_mask ((__v8si)__A, (__v8si) __C,
						(__v8si) __D, (__mmask8)__B) ;
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shldv_epi32 (__mmask8 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshldv_v8si_maskz ((__v8si)__B, (__v8si) __C,
						(__v8si) __D, (__mmask8)__A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shldv_epi64 (__m256i __A, __m256i __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_vpshldv_v4di ((__v4di)__A, (__v4di) __B,
								(__v4di) __C);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shldv_epi64 (__m256i __A, __mmask8 __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshldv_v4di_mask ((__v4di)__A, (__v4di) __C,
						(__v4di) __D, (__mmask8)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shldv_epi64 (__mmask8 __A, __m256i __B, __m256i __C, __m256i __D)
{
  return (__m256i)__builtin_ia32_vpshldv_v4di_maskz ((__v4di)__B, (__v4di) __C,
						 (__v4di) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shldv_epi16 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpshldv_v8hi ((__v8hi)__A, (__v8hi) __B,
								(__v8hi) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shldv_epi16 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshldv_v8hi_mask ((__v8hi)__A, (__v8hi) __C,
						(__v8hi) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shldv_epi16 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshldv_v8hi_maskz ((__v8hi)__B, (__v8hi) __C,
						 (__v8hi) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shldv_epi32 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpshldv_v4si ((__v4si)__A, (__v4si) __B,
								(__v4si) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shldv_epi32 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshldv_v4si_mask ((__v4si)__A, (__v4si) __C,
						(__v4si) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shldv_epi32 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshldv_v4si_maskz ((__v4si)__B, (__v4si) __C,
						 (__v4si) __D, (__mmask8)__A);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shldv_epi64 (__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpshldv_v2di ((__v2di)__A, (__v2di) __B,
								(__v2di) __C);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shldv_epi64 (__m128i __A, __mmask8 __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshldv_v2di_mask ((__v2di)__A, (__v2di) __C,
						(__v2di) __D, (__mmask8)__B);
}

extern __inline __m128i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shldv_epi64 (__mmask8 __A, __m128i __B, __m128i __C, __m128i __D)
{
  return (__m128i)__builtin_ia32_vpshldv_v2di_maskz ((__v2di)__B, (__v2di) __C,
						(__v2di) __D, (__mmask8)__A);
}




#ifdef __DISABLE_AVX512VBMI2VL__
#undef __DISABLE_AVX512VBMI2VL__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VBMIVL__ */

#if !defined(__AVX512VL__) || !defined(__AVX512VBMI2__) || \
    !defined(__AVX512BW__)
#pragma GCC push_options
#pragma GCC target("avx512vbmi2,avx512vl,avx512bw")
#define __DISABLE_AVX512VBMI2VLBW__
#endif /* __AVX512VBMIVLBW__ */

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compress_epi8 (__m256i __A, __mmask32 __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_compressqi256_mask ((__v32qi)__C,
						(__v32qi)__A, (__mmask32)__B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_compress_epi8 (__mmask32 __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_compressqi256_mask ((__v32qi) __B,
			(__v32qi) _mm256_setzero_si256 (), (__mmask32) __A);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compressstoreu_epi8 (void * __A, __mmask32 __B, __m256i __C)
{
  __builtin_ia32_compressstoreuqi256_mask ((__v32qi *) __A, (__v32qi) __C,
							(__mmask32) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expand_epi8 (__m256i __A, __mmask32 __B, __m256i __C)
{
  return (__m256i) __builtin_ia32_expandqi256_mask ((__v32qi) __C,
						    (__v32qi) __A,
						    (__mmask32) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expand_epi8 (__mmask32 __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_expandqi256_maskz ((__v32qi) __B,
			(__v32qi) _mm256_setzero_si256 (), (__mmask32) __A);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expandloadu_epi8 (__m256i __A, __mmask32 __B, const void * __C)
{
  return (__m256i) __builtin_ia32_expandloadqi256_mask ((const __v32qi *) __C,
					(__v32qi) __A, (__mmask32) __B);
}

extern __inline __m256i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expandloadu_epi8 (__mmask32 __A, const void * __B)
{
  return (__m256i) __builtin_ia32_expandloadqi256_maskz ((const __v32qi *) __B,
			(__v32qi) _mm256_setzero_si256 (), (__mmask32) __A);
}

#ifdef __DISABLE_AVX512VBMI2VLBW__
#undef __DISABLE_AVX512VBMI2VLBW__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VBMIVLBW__ */

#endif /* _AVX512VBMIVLINTRIN_H_INCLUDED */
