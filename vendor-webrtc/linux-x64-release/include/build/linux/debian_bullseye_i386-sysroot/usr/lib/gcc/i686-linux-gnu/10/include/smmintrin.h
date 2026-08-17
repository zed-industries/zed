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

/* Implemented from the specification included in the Intel C++ Compiler
   User Guide and Reference, version 10.0.  */

#ifndef _SMMINTRIN_H_INCLUDED
#define _SMMINTRIN_H_INCLUDED

/* We need definitions from the SSSE3, SSE3, SSE2 and SSE header
   files.  */
#include <tmmintrin.h>

#ifndef __SSE4_1__
#pragma GCC push_options
#pragma GCC target("sse4.1")
#define __DISABLE_SSE4_1__
#endif /* __SSE4_1__ */

/* Rounding mode macros. */
#define _MM_FROUND_TO_NEAREST_INT	0x00
#define _MM_FROUND_TO_NEG_INF		0x01
#define _MM_FROUND_TO_POS_INF		0x02
#define _MM_FROUND_TO_ZERO		0x03
#define _MM_FROUND_CUR_DIRECTION	0x04

#define _MM_FROUND_RAISE_EXC		0x00
#define _MM_FROUND_NO_EXC		0x08

#define _MM_FROUND_NINT		\
  (_MM_FROUND_TO_NEAREST_INT | _MM_FROUND_RAISE_EXC)
#define _MM_FROUND_FLOOR	\
  (_MM_FROUND_TO_NEG_INF | _MM_FROUND_RAISE_EXC)
#define _MM_FROUND_CEIL		\
  (_MM_FROUND_TO_POS_INF | _MM_FROUND_RAISE_EXC)
#define _MM_FROUND_TRUNC	\
  (_MM_FROUND_TO_ZERO | _MM_FROUND_RAISE_EXC)
#define _MM_FROUND_RINT		\
  (_MM_FROUND_CUR_DIRECTION | _MM_FROUND_RAISE_EXC)
#define _MM_FROUND_NEARBYINT	\
  (_MM_FROUND_CUR_DIRECTION | _MM_FROUND_NO_EXC)

/* Test Instruction */
/* Packed integer 128-bit bitwise comparison. Return 1 if
   (__V & __M) == 0.  */
extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testz_si128 (__m128i __M, __m128i __V)
{
  return __builtin_ia32_ptestz128 ((__v2di)__M, (__v2di)__V);
}

/* Packed integer 128-bit bitwise comparison. Return 1 if
   (__V & ~__M) == 0.  */
extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testc_si128 (__m128i __M, __m128i __V)
{
  return __builtin_ia32_ptestc128 ((__v2di)__M, (__v2di)__V);
}

/* Packed integer 128-bit bitwise comparison. Return 1 if
   (__V & __M) != 0 && (__V & ~__M) != 0.  */
extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testnzc_si128 (__m128i __M, __m128i __V)
{
  return __builtin_ia32_ptestnzc128 ((__v2di)__M, (__v2di)__V);
}

/* Macros for packed integer 128-bit comparison intrinsics.  */
#define _mm_test_all_zeros(M, V) _mm_testz_si128 ((M), (V))

#define _mm_test_all_ones(V) \
  _mm_testc_si128 ((V), _mm_cmpeq_epi32 ((V), (V)))

#define _mm_test_mix_ones_zeros(M, V) _mm_testnzc_si128 ((M), (V))

/* Packed/scalar double precision floating point rounding.  */

#ifdef __OPTIMIZE__
extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_round_pd (__m128d __V, const int __M)
{
  return (__m128d) __builtin_ia32_roundpd ((__v2df)__V, __M);
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_round_sd(__m128d __D, __m128d __V, const int __M)
{
  return (__m128d) __builtin_ia32_roundsd ((__v2df)__D,
					   (__v2df)__V,
					   __M);
}
#else
#define _mm_round_pd(V, M) \
  ((__m128d) __builtin_ia32_roundpd ((__v2df)(__m128d)(V), (int)(M)))

#define _mm_round_sd(D, V, M)						\
  ((__m128d) __builtin_ia32_roundsd ((__v2df)(__m128d)(D),		\
				     (__v2df)(__m128d)(V), (int)(M)))
#endif

/* Packed/scalar single precision floating point rounding.  */

#ifdef __OPTIMIZE__
extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_round_ps (__m128 __V, const int __M)
{
  return (__m128) __builtin_ia32_roundps ((__v4sf)__V, __M);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_round_ss (__m128 __D, __m128 __V, const int __M)
{
  return (__m128) __builtin_ia32_roundss ((__v4sf)__D,
					  (__v4sf)__V,
					  __M);
}
#else
#define _mm_round_ps(V, M) \
  ((__m128) __builtin_ia32_roundps ((__v4sf)(__m128)(V), (int)(M)))

#define _mm_round_ss(D, V, M)						\
  ((__m128) __builtin_ia32_roundss ((__v4sf)(__m128)(D),		\
				    (__v4sf)(__m128)(V), (int)(M)))
#endif

/* Macros for ceil/floor intrinsics.  */
#define _mm_ceil_pd(V)	   _mm_round_pd ((V), _MM_FROUND_CEIL)
#define _mm_ceil_sd(D, V)  _mm_round_sd ((D), (V), _MM_FROUND_CEIL)

#define _mm_floor_pd(V)	   _mm_round_pd((V), _MM_FROUND_FLOOR)
#define _mm_floor_sd(D, V) _mm_round_sd ((D), (V), _MM_FROUND_FLOOR)

#define _mm_ceil_ps(V)	   _mm_round_ps ((V), _MM_FROUND_CEIL)
#define _mm_ceil_ss(D, V)  _mm_round_ss ((D), (V), _MM_FROUND_CEIL)

#define _mm_floor_ps(V)	   _mm_round_ps ((V), _MM_FROUND_FLOOR)
#define _mm_floor_ss(D, V) _mm_round_ss ((D), (V), _MM_FROUND_FLOOR)

/* SSE4.1 */

/* Integer blend instructions - select data from 2 sources using
   constant/variable mask.  */

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_blend_epi16 (__m128i __X, __m128i __Y, const int __M)
{
  return (__m128i) __builtin_ia32_pblendw128 ((__v8hi)__X,
					      (__v8hi)__Y,
					      __M);
}
#else
#define _mm_blend_epi16(X, Y, M)					\
  ((__m128i) __builtin_ia32_pblendw128 ((__v8hi)(__m128i)(X),		\
					(__v8hi)(__m128i)(Y), (int)(M)))
#endif

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_blendv_epi8 (__m128i __X, __m128i __Y, __m128i __M)
{
  return (__m128i) __builtin_ia32_pblendvb128 ((__v16qi)__X,
					       (__v16qi)__Y,
					       (__v16qi)__M);
}

/* Single precision floating point blend instructions - select data
   from 2 sources using constant/variable mask.  */

#ifdef __OPTIMIZE__
extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_blend_ps (__m128 __X, __m128 __Y, const int __M)
{
  return (__m128) __builtin_ia32_blendps ((__v4sf)__X,
					  (__v4sf)__Y,
					  __M);
}
#else
#define _mm_blend_ps(X, Y, M)						\
  ((__m128) __builtin_ia32_blendps ((__v4sf)(__m128)(X),		\
				    (__v4sf)(__m128)(Y), (int)(M)))
#endif

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_blendv_ps (__m128 __X, __m128 __Y, __m128 __M)
{
  return (__m128) __builtin_ia32_blendvps ((__v4sf)__X,
					   (__v4sf)__Y,
					   (__v4sf)__M);
}

/* Double precision floating point blend instructions - select data
   from 2 sources using constant/variable mask.  */

#ifdef __OPTIMIZE__
extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_blend_pd (__m128d __X, __m128d __Y, const int __M)
{
  return (__m128d) __builtin_ia32_blendpd ((__v2df)__X,
					   (__v2df)__Y,
					   __M);
}
#else
#define _mm_blend_pd(X, Y, M)						\
  ((__m128d) __builtin_ia32_blendpd ((__v2df)(__m128d)(X),		\
				     (__v2df)(__m128d)(Y), (int)(M)))
#endif

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_blendv_pd (__m128d __X, __m128d __Y, __m128d __M)
{
  return (__m128d) __builtin_ia32_blendvpd ((__v2df)__X,
					    (__v2df)__Y,
					    (__v2df)__M);
}

/* Dot product instructions with mask-defined summing and zeroing parts
   of result.  */

#ifdef __OPTIMIZE__
extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_dp_ps (__m128 __X, __m128 __Y, const int __M)
{
  return (__m128) __builtin_ia32_dpps ((__v4sf)__X,
				       (__v4sf)__Y,
				       __M);
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_dp_pd (__m128d __X, __m128d __Y, const int __M)
{
  return (__m128d) __builtin_ia32_dppd ((__v2df)__X,
					(__v2df)__Y,
					__M);
}
#else
#define _mm_dp_ps(X, Y, M)						\
  ((__m128) __builtin_ia32_dpps ((__v4sf)(__m128)(X),			\
				 (__v4sf)(__m128)(Y), (int)(M)))

#define _mm_dp_pd(X, Y, M)						\
  ((__m128d) __builtin_ia32_dppd ((__v2df)(__m128d)(X),			\
				  (__v2df)(__m128d)(Y), (int)(M)))
#endif

/* Packed integer 64-bit comparison, zeroing or filling with ones
   corresponding parts of result.  */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epi64 (__m128i __X, __m128i __Y)
{
  return (__m128i) ((__v2di)__X == (__v2di)__Y);
}

/*  Min/max packed integer instructions.  */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_epi8 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pminsb128 ((__v16qi)__X, (__v16qi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_epi8 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmaxsb128 ((__v16qi)__X, (__v16qi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_epu16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pminuw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_epu16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmaxuw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pminsd128 ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmaxsd128 ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_epu32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pminud128 ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_epu32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmaxud128 ((__v4si)__X, (__v4si)__Y);
}

/* Packed integer 32-bit multiplication with truncation of upper
   halves of results.  */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mullo_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) ((__v4su)__X * (__v4su)__Y);
}

/* Packed integer 32-bit multiplication of 2 pairs of operands
   with two 64-bit results.  */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mul_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmuldq128 ((__v4si)__X, (__v4si)__Y);
}

/* Insert single precision float into packed single precision array
   element selected by index N.  The bits [7-6] of N define S
   index, the bits [5-4] define D index, and bits [3-0] define
   zeroing mask for D.  */

#ifdef __OPTIMIZE__
extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_insert_ps (__m128 __D, __m128 __S, const int __N)
{
  return (__m128) __builtin_ia32_insertps128 ((__v4sf)__D,
					      (__v4sf)__S,
					      __N);
}
#else
#define _mm_insert_ps(D, S, N)						\
  ((__m128) __builtin_ia32_insertps128 ((__v4sf)(__m128)(D),		\
					(__v4sf)(__m128)(S), (int)(N)))
#endif

/* Helper macro to create the N value for _mm_insert_ps.  */
#define _MM_MK_INSERTPS_NDX(S, D, M) (((S) << 6) | ((D) << 4) | (M))

/* Extract binary representation of single precision float from packed
   single precision array element of X selected by index N.  */

#ifdef __OPTIMIZE__
extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_extract_ps (__m128 __X, const int __N)
{
  union { int i; float f; } __tmp;
  __tmp.f = __builtin_ia32_vec_ext_v4sf ((__v4sf)__X, __N);
  return __tmp.i;
}
#else
#define _mm_extract_ps(X, N)						\
  (__extension__							\
   ({									\
     union { int i; float f; } __tmp;					\
     __tmp.f = __builtin_ia32_vec_ext_v4sf ((__v4sf)(__m128)(X), (int)(N)); \
     __tmp.i;								\
   }))
#endif

/* Extract binary representation of single precision float into
   D from packed single precision array element of S selected
   by index N.  */
#define _MM_EXTRACT_FLOAT(D, S, N) \
  { (D) = __builtin_ia32_vec_ext_v4sf ((__v4sf)(S), (N)); }
  
/* Extract specified single precision float element into the lower
   part of __m128.  */
#define _MM_PICK_OUT_PS(X, N)				\
  _mm_insert_ps (_mm_setzero_ps (), (X), 		\
		 _MM_MK_INSERTPS_NDX ((N), 0, 0x0e))

/* Insert integer, S, into packed integer array element of D
   selected by index N.  */

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_insert_epi8 (__m128i __D, int __S, const int __N)
{
  return (__m128i) __builtin_ia32_vec_set_v16qi ((__v16qi)__D,
						 __S, __N);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_insert_epi32 (__m128i __D, int __S, const int __N)
{
  return (__m128i) __builtin_ia32_vec_set_v4si ((__v4si)__D,
						 __S, __N);
}

#ifdef __x86_64__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_insert_epi64 (__m128i __D, long long __S, const int __N)
{
  return (__m128i) __builtin_ia32_vec_set_v2di ((__v2di)__D,
						 __S, __N);
}
#endif
#else
#define _mm_insert_epi8(D, S, N)					\
  ((__m128i) __builtin_ia32_vec_set_v16qi ((__v16qi)(__m128i)(D),	\
					   (int)(S), (int)(N)))

#define _mm_insert_epi32(D, S, N)				\
  ((__m128i) __builtin_ia32_vec_set_v4si ((__v4si)(__m128i)(D),	\
					  (int)(S), (int)(N)))

#ifdef __x86_64__
#define _mm_insert_epi64(D, S, N)					\
  ((__m128i) __builtin_ia32_vec_set_v2di ((__v2di)(__m128i)(D),		\
					  (long long)(S), (int)(N)))
#endif
#endif

/* Extract integer from packed integer array element of X selected by
   index N.  */

#ifdef __OPTIMIZE__
extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_extract_epi8 (__m128i __X, const int __N)
{
   return (unsigned char) __builtin_ia32_vec_ext_v16qi ((__v16qi)__X, __N);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_extract_epi32 (__m128i __X, const int __N)
{
   return __builtin_ia32_vec_ext_v4si ((__v4si)__X, __N);
}

#ifdef __x86_64__
extern __inline long long  __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_extract_epi64 (__m128i __X, const int __N)
{
  return __builtin_ia32_vec_ext_v2di ((__v2di)__X, __N);
}
#endif
#else
#define _mm_extract_epi8(X, N) \
  ((int) (unsigned char) __builtin_ia32_vec_ext_v16qi ((__v16qi)(__m128i)(X), (int)(N)))
#define _mm_extract_epi32(X, N) \
  ((int) __builtin_ia32_vec_ext_v4si ((__v4si)(__m128i)(X), (int)(N)))

#ifdef __x86_64__
#define _mm_extract_epi64(X, N) \
  ((long long) __builtin_ia32_vec_ext_v2di ((__v2di)(__m128i)(X), (int)(N)))
#endif
#endif

/* Return horizontal packed word minimum and its index in bits [15:0]
   and bits [18:16] respectively.  */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_minpos_epu16 (__m128i __X)
{
  return (__m128i) __builtin_ia32_phminposuw128 ((__v8hi)__X);
}

/* Packed integer sign-extension.  */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi8_epi32 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxbd128 ((__v16qi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi16_epi32 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxwd128 ((__v8hi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi8_epi64 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxbq128 ((__v16qi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi32_epi64 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxdq128 ((__v4si)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi16_epi64 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxwq128 ((__v8hi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi8_epi16 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxbw128 ((__v16qi)__X);
}

/* Packed integer zero-extension. */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu8_epi32 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxbd128 ((__v16qi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu16_epi32 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxwd128 ((__v8hi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu8_epi64 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxbq128 ((__v16qi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu32_epi64 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxdq128 ((__v4si)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu16_epi64 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxwq128 ((__v8hi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu8_epi16 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxbw128 ((__v16qi)__X);
}

/* Pack 8 double words from 2 operands into 8 words of result with
   unsigned saturation. */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_packus_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_packusdw128 ((__v4si)__X, (__v4si)__Y);
}

/* Sum absolute 8-bit integer difference of adjacent groups of 4
   byte integers in the first 2 operands.  Starting offsets within
   operands are determined by the 3rd mask operand.  */

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mpsadbw_epu8 (__m128i __X, __m128i __Y, const int __M)
{
  return (__m128i) __builtin_ia32_mpsadbw128 ((__v16qi)__X,
					      (__v16qi)__Y, __M);
}
#else
#define _mm_mpsadbw_epu8(X, Y, M)					\
  ((__m128i) __builtin_ia32_mpsadbw128 ((__v16qi)(__m128i)(X),		\
					(__v16qi)(__m128i)(Y), (int)(M)))
#endif

/* Load double quadword using non-temporal aligned hint.  */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_stream_load_si128 (__m128i *__X)
{
  return (__m128i) __builtin_ia32_movntdqa ((__v2di *) __X);
}

#ifndef __SSE4_2__
#pragma GCC push_options
#pragma GCC target("sse4.2")
#define __DISABLE_SSE4_2__
#endif /* __SSE4_2__ */

/* These macros specify the source data format.  */
#define _SIDD_UBYTE_OPS			0x00
#define _SIDD_UWORD_OPS			0x01
#define _SIDD_SBYTE_OPS			0x02
#define _SIDD_SWORD_OPS			0x03

/* These macros specify the comparison operation.  */
#define _SIDD_CMP_EQUAL_ANY		0x00
#define _SIDD_CMP_RANGES		0x04
#define _SIDD_CMP_EQUAL_EACH		0x08
#define _SIDD_CMP_EQUAL_ORDERED		0x0c

/* These macros specify the polarity.  */
#define _SIDD_POSITIVE_POLARITY		0x00
#define _SIDD_NEGATIVE_POLARITY		0x10
#define _SIDD_MASKED_POSITIVE_POLARITY	0x20
#define _SIDD_MASKED_NEGATIVE_POLARITY	0x30

/* These macros specify the output selection in _mm_cmpXstri ().  */
#define _SIDD_LEAST_SIGNIFICANT		0x00
#define _SIDD_MOST_SIGNIFICANT		0x40

/* These macros specify the output selection in _mm_cmpXstrm ().  */
#define _SIDD_BIT_MASK			0x00
#define _SIDD_UNIT_MASK			0x40

/* Intrinsics for text/string processing.  */

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpistrm (__m128i __X, __m128i __Y, const int __M)
{
  return (__m128i) __builtin_ia32_pcmpistrm128 ((__v16qi)__X,
						(__v16qi)__Y,
						__M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpistri (__m128i __X, __m128i __Y, const int __M)
{
  return __builtin_ia32_pcmpistri128 ((__v16qi)__X,
				      (__v16qi)__Y,
				      __M);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpestrm (__m128i __X, int __LX, __m128i __Y, int __LY, const int __M)
{
  return (__m128i) __builtin_ia32_pcmpestrm128 ((__v16qi)__X, __LX,
						(__v16qi)__Y, __LY,
						__M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpestri (__m128i __X, int __LX, __m128i __Y, int __LY, const int __M)
{
  return __builtin_ia32_pcmpestri128 ((__v16qi)__X, __LX,
				      (__v16qi)__Y, __LY,
				      __M);
}
#else
#define _mm_cmpistrm(X, Y, M)						\
  ((__m128i) __builtin_ia32_pcmpistrm128 ((__v16qi)(__m128i)(X),	\
					  (__v16qi)(__m128i)(Y), (int)(M)))
#define _mm_cmpistri(X, Y, M)						\
  ((int) __builtin_ia32_pcmpistri128 ((__v16qi)(__m128i)(X),		\
				      (__v16qi)(__m128i)(Y), (int)(M)))

#define _mm_cmpestrm(X, LX, Y, LY, M)					\
  ((__m128i) __builtin_ia32_pcmpestrm128 ((__v16qi)(__m128i)(X),	\
					  (int)(LX), (__v16qi)(__m128i)(Y), \
					  (int)(LY), (int)(M)))
#define _mm_cmpestri(X, LX, Y, LY, M)					\
  ((int) __builtin_ia32_pcmpestri128 ((__v16qi)(__m128i)(X), (int)(LX),	\
				      (__v16qi)(__m128i)(Y), (int)(LY),	\
				      (int)(M)))
#endif

/* Intrinsics for text/string processing and reading values of
   EFlags.  */

#ifdef __OPTIMIZE__
extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpistra (__m128i __X, __m128i __Y, const int __M)
{
  return __builtin_ia32_pcmpistria128 ((__v16qi)__X,
				       (__v16qi)__Y,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpistrc (__m128i __X, __m128i __Y, const int __M)
{
  return __builtin_ia32_pcmpistric128 ((__v16qi)__X,
				       (__v16qi)__Y,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpistro (__m128i __X, __m128i __Y, const int __M)
{
  return __builtin_ia32_pcmpistrio128 ((__v16qi)__X,
				       (__v16qi)__Y,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpistrs (__m128i __X, __m128i __Y, const int __M)
{
  return __builtin_ia32_pcmpistris128 ((__v16qi)__X,
				       (__v16qi)__Y,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpistrz (__m128i __X, __m128i __Y, const int __M)
{
  return __builtin_ia32_pcmpistriz128 ((__v16qi)__X,
				       (__v16qi)__Y,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpestra (__m128i __X, int __LX, __m128i __Y, int __LY, const int __M)
{
  return __builtin_ia32_pcmpestria128 ((__v16qi)__X, __LX,
				       (__v16qi)__Y, __LY,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpestrc (__m128i __X, int __LX, __m128i __Y, int __LY, const int __M)
{
  return __builtin_ia32_pcmpestric128 ((__v16qi)__X, __LX,
				       (__v16qi)__Y, __LY,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpestro (__m128i __X, int __LX, __m128i __Y, int __LY, const int __M)
{
  return __builtin_ia32_pcmpestrio128 ((__v16qi)__X, __LX,
				       (__v16qi)__Y, __LY,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpestrs (__m128i __X, int __LX, __m128i __Y, int __LY, const int __M)
{
  return __builtin_ia32_pcmpestris128 ((__v16qi)__X, __LX,
				       (__v16qi)__Y, __LY,
				       __M);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpestrz (__m128i __X, int __LX, __m128i __Y, int __LY, const int __M)
{
  return __builtin_ia32_pcmpestriz128 ((__v16qi)__X, __LX,
				       (__v16qi)__Y, __LY,
				       __M);
}
#else
#define _mm_cmpistra(X, Y, M)						\
  ((int) __builtin_ia32_pcmpistria128 ((__v16qi)(__m128i)(X),		\
				       (__v16qi)(__m128i)(Y), (int)(M)))
#define _mm_cmpistrc(X, Y, M)						\
  ((int) __builtin_ia32_pcmpistric128 ((__v16qi)(__m128i)(X),		\
				       (__v16qi)(__m128i)(Y), (int)(M)))
#define _mm_cmpistro(X, Y, M)						\
  ((int) __builtin_ia32_pcmpistrio128 ((__v16qi)(__m128i)(X),		\
				       (__v16qi)(__m128i)(Y), (int)(M)))
#define _mm_cmpistrs(X, Y, M)						\
  ((int) __builtin_ia32_pcmpistris128 ((__v16qi)(__m128i)(X),		\
				       (__v16qi)(__m128i)(Y), (int)(M)))
#define _mm_cmpistrz(X, Y, M)						\
  ((int) __builtin_ia32_pcmpistriz128 ((__v16qi)(__m128i)(X),		\
				       (__v16qi)(__m128i)(Y), (int)(M)))

#define _mm_cmpestra(X, LX, Y, LY, M)					\
  ((int) __builtin_ia32_pcmpestria128 ((__v16qi)(__m128i)(X), (int)(LX), \
				       (__v16qi)(__m128i)(Y), (int)(LY), \
				       (int)(M)))
#define _mm_cmpestrc(X, LX, Y, LY, M)					\
  ((int) __builtin_ia32_pcmpestric128 ((__v16qi)(__m128i)(X), (int)(LX), \
				       (__v16qi)(__m128i)(Y), (int)(LY), \
				       (int)(M)))
#define _mm_cmpestro(X, LX, Y, LY, M)					\
  ((int) __builtin_ia32_pcmpestrio128 ((__v16qi)(__m128i)(X), (int)(LX), \
				       (__v16qi)(__m128i)(Y), (int)(LY), \
				       (int)(M)))
#define _mm_cmpestrs(X, LX, Y, LY, M)					\
  ((int) __builtin_ia32_pcmpestris128 ((__v16qi)(__m128i)(X), (int)(LX), \
				       (__v16qi)(__m128i)(Y), (int)(LY), \
				       (int)(M)))
#define _mm_cmpestrz(X, LX, Y, LY, M)					\
  ((int) __builtin_ia32_pcmpestriz128 ((__v16qi)(__m128i)(X), (int)(LX), \
				       (__v16qi)(__m128i)(Y), (int)(LY), \
				       (int)(M)))
#endif

/* Packed integer 64-bit comparison, zeroing or filling with ones
   corresponding parts of result.  */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epi64 (__m128i __X, __m128i __Y)
{
  return (__m128i) ((__v2di)__X > (__v2di)__Y);
}

#ifdef __DISABLE_SSE4_2__
#undef __DISABLE_SSE4_2__
#pragma GCC pop_options
#endif /* __DISABLE_SSE4_2__ */

#ifdef __DISABLE_SSE4_1__
#undef __DISABLE_SSE4_1__
#pragma GCC pop_options
#endif /* __DISABLE_SSE4_1__ */

#include <popcntintrin.h>

#ifndef __SSE4_1__
#pragma GCC push_options
#pragma GCC target("sse4.1")
#define __DISABLE_SSE4_1__
#endif /* __SSE4_1__ */

#ifndef __SSE4_2__
#pragma GCC push_options
#pragma GCC target("sse4.2")
#define __DISABLE_SSE4_2__
#endif /* __SSE4_1__ */

/* Accumulate CRC32 (polynomial 0x11EDC6F41) value.  */
extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_crc32_u8 (unsigned int __C, unsigned char __V)
{
  return __builtin_ia32_crc32qi (__C, __V);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_crc32_u16 (unsigned int __C, unsigned short __V)
{
  return __builtin_ia32_crc32hi (__C, __V);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_crc32_u32 (unsigned int __C, unsigned int __V)
{
  return __builtin_ia32_crc32si (__C, __V);
}

#ifdef __x86_64__
extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_crc32_u64 (unsigned long long __C, unsigned long long __V)
{
  return __builtin_ia32_crc32di (__C, __V);
}
#endif

#ifdef __DISABLE_SSE4_2__
#undef __DISABLE_SSE4_2__
#pragma GCC pop_options
#endif /* __DISABLE_SSE4_2__ */

#ifdef __DISABLE_SSE4_1__
#undef __DISABLE_SSE4_1__
#pragma GCC pop_options
#endif /* __DISABLE_SSE4_1__ */

#endif /* _SMMINTRIN_H_INCLUDED */
