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
#error "Never use <avx512erintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512ERINTRIN_H_INCLUDED
#define _AVX512ERINTRIN_H_INCLUDED

#ifndef __AVX512ER__
#pragma GCC push_options
#pragma GCC target("avx512er")
#define __DISABLE_AVX512ER__
#endif /* __AVX512ER__ */

/* Internal data types for implementing the intrinsics.  */
typedef double __v8df __attribute__ ((__vector_size__ (64)));
typedef float __v16sf __attribute__ ((__vector_size__ (64)));

/* The Intel API is flexible enough that we must allow aliasing with other
   vector types, and their scalar components.  */
typedef float __m512 __attribute__ ((__vector_size__ (64), __may_alias__));
typedef double __m512d __attribute__ ((__vector_size__ (64), __may_alias__));

typedef unsigned char  __mmask8;
typedef unsigned short __mmask16;

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_exp2a23_round_pd (__m512d __A, int __R)
{
  __m512d __W;
  return (__m512d) __builtin_ia32_exp2pd_mask ((__v8df) __A,
					       (__v8df) __W,
					       (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_exp2a23_round_pd (__m512d __W, __mmask8 __U, __m512d __A, int __R)
{
  return (__m512d) __builtin_ia32_exp2pd_mask ((__v8df) __A,
					       (__v8df) __W,
					       (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_exp2a23_round_pd (__mmask8 __U, __m512d __A, int __R)
{
  return (__m512d) __builtin_ia32_exp2pd_mask ((__v8df) __A,
					       (__v8df) _mm512_setzero_pd (),
					       (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_exp2a23_round_ps (__m512 __A, int __R)
{
  __m512 __W;
  return (__m512) __builtin_ia32_exp2ps_mask ((__v16sf) __A,
					      (__v16sf) __W,
					      (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_exp2a23_round_ps (__m512 __W, __mmask16 __U, __m512 __A, int __R)
{
  return (__m512) __builtin_ia32_exp2ps_mask ((__v16sf) __A,
					      (__v16sf) __W,
					      (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_exp2a23_round_ps (__mmask16 __U, __m512 __A, int __R)
{
  return (__m512) __builtin_ia32_exp2ps_mask ((__v16sf) __A,
					      (__v16sf) _mm512_setzero_ps (),
					      (__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rcp28_round_pd (__m512d __A, int __R)
{
  __m512d __W;
  return (__m512d) __builtin_ia32_rcp28pd_mask ((__v8df) __A,
						(__v8df) __W,
						(__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rcp28_round_pd (__m512d __W, __mmask8 __U, __m512d __A, int __R)
{
  return (__m512d) __builtin_ia32_rcp28pd_mask ((__v8df) __A,
						(__v8df) __W,
						(__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rcp28_round_pd (__mmask8 __U, __m512d __A, int __R)
{
  return (__m512d) __builtin_ia32_rcp28pd_mask ((__v8df) __A,
						(__v8df) _mm512_setzero_pd (),
						(__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rcp28_round_ps (__m512 __A, int __R)
{
  __m512 __W;
  return (__m512) __builtin_ia32_rcp28ps_mask ((__v16sf) __A,
					       (__v16sf) __W,
					       (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rcp28_round_ps (__m512 __W, __mmask16 __U, __m512 __A, int __R)
{
  return (__m512) __builtin_ia32_rcp28ps_mask ((__v16sf) __A,
					       (__v16sf) __W,
					       (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rcp28_round_ps (__mmask16 __U, __m512 __A, int __R)
{
  return (__m512) __builtin_ia32_rcp28ps_mask ((__v16sf) __A,
					       (__v16sf) _mm512_setzero_ps (),
					       (__mmask16) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rcp28_round_sd (__m128d __A, __m128d __B, int __R)
{
  return (__m128d) __builtin_ia32_rcp28sd_round ((__v2df) __B,
						 (__v2df) __A,
						 __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rcp28_round_ss (__m128 __A, __m128 __B, int __R)
{
  return (__m128) __builtin_ia32_rcp28ss_round ((__v4sf) __B,
						(__v4sf) __A,
						__R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rsqrt28_round_pd (__m512d __A, int __R)
{
  __m512d __W;
  return (__m512d) __builtin_ia32_rsqrt28pd_mask ((__v8df) __A,
						  (__v8df) __W,
						  (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rsqrt28_round_pd (__m512d __W, __mmask8 __U, __m512d __A, int __R)
{
  return (__m512d) __builtin_ia32_rsqrt28pd_mask ((__v8df) __A,
						  (__v8df) __W,
						  (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rsqrt28_round_pd (__mmask8 __U, __m512d __A, int __R)
{
  return (__m512d) __builtin_ia32_rsqrt28pd_mask ((__v8df) __A,
						  (__v8df) _mm512_setzero_pd (),
						  (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rsqrt28_round_ps (__m512 __A, int __R)
{
  __m512 __W;
  return (__m512) __builtin_ia32_rsqrt28ps_mask ((__v16sf) __A,
						 (__v16sf) __W,
						 (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rsqrt28_round_ps (__m512 __W, __mmask16 __U, __m512 __A, int __R)
{
  return (__m512) __builtin_ia32_rsqrt28ps_mask ((__v16sf) __A,
						 (__v16sf) __W,
						 (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rsqrt28_round_ps (__mmask16 __U, __m512 __A, int __R)
{
  return (__m512) __builtin_ia32_rsqrt28ps_mask ((__v16sf) __A,
						 (__v16sf) _mm512_setzero_ps (),
						 (__mmask16) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rsqrt28_round_sd (__m128d __A, __m128d __B, int __R)
{
  return (__m128d) __builtin_ia32_rsqrt28sd_round ((__v2df) __B,
						   (__v2df) __A,
						   __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rsqrt28_round_ss (__m128 __A, __m128 __B, int __R)
{
  return (__m128) __builtin_ia32_rsqrt28ss_round ((__v4sf) __B,
						  (__v4sf) __A,
						  __R);
}

#else
#define _mm512_exp2a23_round_pd(A, C)            \
    __builtin_ia32_exp2pd_mask(A, (__v8df)_mm512_setzero_pd(), -1, C)

#define _mm512_mask_exp2a23_round_pd(W, U, A, C) \
    __builtin_ia32_exp2pd_mask(A, W, U, C)

#define _mm512_maskz_exp2a23_round_pd(U, A, C)   \
    __builtin_ia32_exp2pd_mask(A, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_exp2a23_round_ps(A, C)            \
    __builtin_ia32_exp2ps_mask(A, (__v16sf)_mm512_setzero_ps(), -1, C)

#define _mm512_mask_exp2a23_round_ps(W, U, A, C) \
    __builtin_ia32_exp2ps_mask(A, W, U, C)

#define _mm512_maskz_exp2a23_round_ps(U, A, C)   \
    __builtin_ia32_exp2ps_mask(A, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm512_rcp28_round_pd(A, C)            \
    __builtin_ia32_rcp28pd_mask(A, (__v8df)_mm512_setzero_pd(), -1, C)

#define _mm512_mask_rcp28_round_pd(W, U, A, C) \
    __builtin_ia32_rcp28pd_mask(A, W, U, C)

#define _mm512_maskz_rcp28_round_pd(U, A, C)   \
    __builtin_ia32_rcp28pd_mask(A, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_rcp28_round_ps(A, C)            \
    __builtin_ia32_rcp28ps_mask(A, (__v16sf)_mm512_setzero_ps(), -1, C)

#define _mm512_mask_rcp28_round_ps(W, U, A, C) \
    __builtin_ia32_rcp28ps_mask(A, W, U, C)

#define _mm512_maskz_rcp28_round_ps(U, A, C)   \
    __builtin_ia32_rcp28ps_mask(A, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm512_rsqrt28_round_pd(A, C)            \
    __builtin_ia32_rsqrt28pd_mask(A, (__v8df)_mm512_setzero_pd(), -1, C)

#define _mm512_mask_rsqrt28_round_pd(W, U, A, C) \
    __builtin_ia32_rsqrt28pd_mask(A, W, U, C)

#define _mm512_maskz_rsqrt28_round_pd(U, A, C)   \
    __builtin_ia32_rsqrt28pd_mask(A, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_rsqrt28_round_ps(A, C)            \
    __builtin_ia32_rsqrt28ps_mask(A, (__v16sf)_mm512_setzero_ps(), -1, C)

#define _mm512_mask_rsqrt28_round_ps(W, U, A, C) \
    __builtin_ia32_rsqrt28ps_mask(A, W, U, C)

#define _mm512_maskz_rsqrt28_round_ps(U, A, C)   \
    __builtin_ia32_rsqrt28ps_mask(A, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm_rcp28_round_sd(A, B, R)	\
    __builtin_ia32_rcp28sd_round(A, B, R)

#define _mm_rcp28_round_ss(A, B, R)	\
    __builtin_ia32_rcp28ss_round(A, B, R)

#define _mm_rsqrt28_round_sd(A, B, R)	\
    __builtin_ia32_rsqrt28sd_round(A, B, R)

#define _mm_rsqrt28_round_ss(A, B, R)	\
    __builtin_ia32_rsqrt28ss_round(A, B, R)

#endif

#define _mm512_exp2a23_pd(A)                    \
    _mm512_exp2a23_round_pd(A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_mask_exp2a23_pd(W, U, A)   \
    _mm512_mask_exp2a23_round_pd(W, U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_maskz_exp2a23_pd(U, A)     \
    _mm512_maskz_exp2a23_round_pd(U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_exp2a23_ps(A)                    \
    _mm512_exp2a23_round_ps(A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_mask_exp2a23_ps(W, U, A)   \
    _mm512_mask_exp2a23_round_ps(W, U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_maskz_exp2a23_ps(U, A)     \
    _mm512_maskz_exp2a23_round_ps(U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_rcp28_pd(A)                    \
    _mm512_rcp28_round_pd(A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_mask_rcp28_pd(W, U, A)   \
    _mm512_mask_rcp28_round_pd(W, U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_maskz_rcp28_pd(U, A)     \
    _mm512_maskz_rcp28_round_pd(U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_rcp28_ps(A)                    \
    _mm512_rcp28_round_ps(A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_mask_rcp28_ps(W, U, A)   \
    _mm512_mask_rcp28_round_ps(W, U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_maskz_rcp28_ps(U, A)     \
    _mm512_maskz_rcp28_round_ps(U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_rsqrt28_pd(A)                    \
    _mm512_rsqrt28_round_pd(A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_mask_rsqrt28_pd(W, U, A)   \
    _mm512_mask_rsqrt28_round_pd(W, U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_maskz_rsqrt28_pd(U, A)     \
    _mm512_maskz_rsqrt28_round_pd(U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_rsqrt28_ps(A)                    \
    _mm512_rsqrt28_round_ps(A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_mask_rsqrt28_ps(W, U, A)   \
    _mm512_mask_rsqrt28_round_ps(W, U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm512_maskz_rsqrt28_ps(U, A)     \
    _mm512_maskz_rsqrt28_round_ps(U, A, _MM_FROUND_CUR_DIRECTION)

#define _mm_rcp28_sd(A, B)	\
    __builtin_ia32_rcp28sd_round(B, A, _MM_FROUND_CUR_DIRECTION)

#define _mm_rcp28_ss(A, B)	\
    __builtin_ia32_rcp28ss_round(B, A, _MM_FROUND_CUR_DIRECTION)

#define _mm_rsqrt28_sd(A, B)	\
    __builtin_ia32_rsqrt28sd_round(B, A, _MM_FROUND_CUR_DIRECTION)

#define _mm_rsqrt28_ss(A, B)	\
    __builtin_ia32_rsqrt28ss_round(B, A, _MM_FROUND_CUR_DIRECTION)

#ifdef __DISABLE_AVX512ER__
#undef __DISABLE_AVX512ER__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512ER__ */

#endif /* _AVX512ERINTRIN_H_INCLUDED */
