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
#error "Never use <avx512vlintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512VLINTRIN_H_INCLUDED
#define _AVX512VLINTRIN_H_INCLUDED

#ifndef __AVX512VL__
#pragma GCC push_options
#pragma GCC target("avx512vl")
#define __DISABLE_AVX512VL__
#endif /* __AVX512VL__ */

/* Internal data types for implementing the intrinsics.  */
typedef unsigned int __mmask32;

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mov_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_movapd256_mask ((__v4df) __A,
						  (__v4df) __W,
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mov_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_movapd256_mask ((__v4df) __A,
						  (__v4df)
						  _mm256_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mov_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_movapd128_mask ((__v2df) __A,
						  (__v2df) __W,
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mov_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_movapd128_mask ((__v2df) __A,
						  (__v2df)
						  _mm_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_load_pd (__m256d __W, __mmask8 __U, void const *__P)
{
  return (__m256d) __builtin_ia32_loadapd256_mask ((__v4df *) __P,
						   (__v4df) __W,
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_load_pd (__mmask8 __U, void const *__P)
{
  return (__m256d) __builtin_ia32_loadapd256_mask ((__v4df *) __P,
						   (__v4df)
						   _mm256_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_load_pd (__m128d __W, __mmask8 __U, void const *__P)
{
  return (__m128d) __builtin_ia32_loadapd128_mask ((__v2df *) __P,
						   (__v2df) __W,
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_load_pd (__mmask8 __U, void const *__P)
{
  return (__m128d) __builtin_ia32_loadapd128_mask ((__v2df *) __P,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_store_pd (void *__P, __mmask8 __U, __m256d __A)
{
  __builtin_ia32_storeapd256_mask ((__v4df *) __P,
				   (__v4df) __A,
				   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_store_pd (void *__P, __mmask8 __U, __m128d __A)
{
  __builtin_ia32_storeapd128_mask ((__v2df *) __P,
				   (__v2df) __A,
				   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mov_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_movaps256_mask ((__v8sf) __A,
						 (__v8sf) __W,
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mov_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_movaps256_mask ((__v8sf) __A,
						 (__v8sf)
						 _mm256_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mov_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_movaps128_mask ((__v4sf) __A,
						 (__v4sf) __W,
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mov_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_movaps128_mask ((__v4sf) __A,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_load_ps (__m256 __W, __mmask8 __U, void const *__P)
{
  return (__m256) __builtin_ia32_loadaps256_mask ((__v8sf *) __P,
						  (__v8sf) __W,
						  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_load_ps (__mmask8 __U, void const *__P)
{
  return (__m256) __builtin_ia32_loadaps256_mask ((__v8sf *) __P,
						  (__v8sf)
						  _mm256_setzero_ps (),
						  (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_load_ps (__m128 __W, __mmask8 __U, void const *__P)
{
  return (__m128) __builtin_ia32_loadaps128_mask ((__v4sf *) __P,
						  (__v4sf) __W,
						  (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_load_ps (__mmask8 __U, void const *__P)
{
  return (__m128) __builtin_ia32_loadaps128_mask ((__v4sf *) __P,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_store_ps (void *__P, __mmask8 __U, __m256 __A)
{
  __builtin_ia32_storeaps256_mask ((__v8sf *) __P,
				   (__v8sf) __A,
				   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_store_ps (void *__P, __mmask8 __U, __m128 __A)
{
  __builtin_ia32_storeaps128_mask ((__v4sf *) __P,
				   (__v4sf) __A,
				   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mov_epi64 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdqa64_256_mask ((__v4di) __A,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mov_epi64 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdqa64_256_mask ((__v4di) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mov_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdqa64_128_mask ((__v2di) __A,
						     (__v2di) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mov_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdqa64_128_mask ((__v2di) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_load_epi64 (__m256i __W, __mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_movdqa64load256_mask ((__v4di *) __P,
							(__v4di) __W,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_load_epi64 (__mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_movdqa64load256_mask ((__v4di *) __P,
							(__v4di)
							_mm256_setzero_si256 (),
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_load_epi64 (__m128i __W, __mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_movdqa64load128_mask ((__v2di *) __P,
							(__v2di) __W,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_load_epi64 (__mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_movdqa64load128_mask ((__v2di *) __P,
							(__v2di)
							_mm_setzero_si128 (),
							(__mmask8)
							__U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_store_epi64 (void *__P, __mmask8 __U, __m256i __A)
{
  __builtin_ia32_movdqa64store256_mask ((__v4di *) __P,
					(__v4di) __A,
					(__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_store_epi64 (void *__P, __mmask8 __U, __m128i __A)
{
  __builtin_ia32_movdqa64store128_mask ((__v2di *) __P,
					(__v2di) __A,
					(__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mov_epi32 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdqa32_256_mask ((__v8si) __A,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mov_epi32 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_movdqa32_256_mask ((__v8si) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mov_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdqa32_128_mask ((__v4si) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mov_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_movdqa32_128_mask ((__v4si) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_load_epi32 (__m256i __W, __mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_movdqa32load256_mask ((__v8si *) __P,
							(__v8si) __W,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_load_epi32 (__mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_movdqa32load256_mask ((__v8si *) __P,
							(__v8si)
							_mm256_setzero_si256 (),
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_load_epi32 (__m128i __W, __mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_movdqa32load128_mask ((__v4si *) __P,
							(__v4si) __W,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_load_epi32 (__mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_movdqa32load128_mask ((__v4si *) __P,
							(__v4si)
							_mm_setzero_si128 (),
							(__mmask8)
							__U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_store_epi32 (void *__P, __mmask8 __U, __m256i __A)
{
  __builtin_ia32_movdqa32store256_mask ((__v8si *) __P,
					(__v8si) __A,
					(__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_store_epi32 (void *__P, __mmask8 __U, __m128i __A)
{
  __builtin_ia32_movdqa32store128_mask ((__v4si *) __P,
					(__v4si) __A,
					(__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_addpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_addpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_add_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_addpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_add_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_addpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_addps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_addps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_add_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_addps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_add_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_addps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_subpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_subpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sub_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_subpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sub_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_subpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_subps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_subps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sub_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_subps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sub_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_subps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_store_epi64 (void *__P, __m256i __A)
{
  *(__m256i *) __P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_store_epi64 (void *__P, __m128i __A)
{
  *(__m128i *) __P = __A;
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_loadu_pd (__m256d __W, __mmask8 __U, void const *__P)
{
  return (__m256d) __builtin_ia32_loadupd256_mask ((const double *) __P,
						   (__v4df) __W,
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_loadu_pd (__mmask8 __U, void const *__P)
{
  return (__m256d) __builtin_ia32_loadupd256_mask ((const double *) __P,
						   (__v4df)
						   _mm256_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_loadu_pd (__m128d __W, __mmask8 __U, void const *__P)
{
  return (__m128d) __builtin_ia32_loadupd128_mask ((const double *) __P,
						   (__v2df) __W,
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_loadu_pd (__mmask8 __U, void const *__P)
{
  return (__m128d) __builtin_ia32_loadupd128_mask ((const double *) __P,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_storeu_pd (void *__P, __mmask8 __U, __m256d __A)
{
  __builtin_ia32_storeupd256_mask ((double *) __P,
				   (__v4df) __A,
				   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_storeu_pd (void *__P, __mmask8 __U, __m128d __A)
{
  __builtin_ia32_storeupd128_mask ((double *) __P,
				   (__v2df) __A,
				   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_loadu_ps (__m256 __W, __mmask8 __U, void const *__P)
{
  return (__m256) __builtin_ia32_loadups256_mask ((const float *) __P,
						  (__v8sf) __W,
						  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_loadu_ps (__mmask8 __U, void const *__P)
{
  return (__m256) __builtin_ia32_loadups256_mask ((const float *) __P,
						  (__v8sf)
						  _mm256_setzero_ps (),
						  (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_loadu_ps (__m128 __W, __mmask8 __U, void const *__P)
{
  return (__m128) __builtin_ia32_loadups128_mask ((const float *) __P,
						  (__v4sf) __W,
						  (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_loadu_ps (__mmask8 __U, void const *__P)
{
  return (__m128) __builtin_ia32_loadups128_mask ((const float *) __P,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_storeu_ps (void *__P, __mmask8 __U, __m256 __A)
{
  __builtin_ia32_storeups256_mask ((float *) __P,
				   (__v8sf) __A,
				   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_storeu_ps (void *__P, __mmask8 __U, __m128 __A)
{
  __builtin_ia32_storeups128_mask ((float *) __P,
				   (__v4sf) __A,
				   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_loadu_epi64 (__m256i __W, __mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddqudi256_mask ((const long long *) __P,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_loadu_epi64 (__mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddqudi256_mask ((const long long *) __P,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_loadu_epi64 (__m128i __W, __mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddqudi128_mask ((const long long *) __P,
						     (__v2di) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_loadu_epi64 (__mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddqudi128_mask ((const long long *) __P,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu_epi64 (void *__P, __m256i __A)
{
  *(__m256i_u *) __P = (__m256i_u) __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_storeu_epi64 (void *__P, __mmask8 __U, __m256i __A)
{
  __builtin_ia32_storedqudi256_mask ((long long *) __P,
				     (__v4di) __A,
				     (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_storeu_epi64 (void *__P, __m128i __A)
{
  *(__m128i_u *) __P = (__m128i_u) __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_storeu_epi64 (void *__P, __mmask8 __U, __m128i __A)
{
  __builtin_ia32_storedqudi128_mask ((long long *) __P,
				     (__v2di) __A,
				     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_loadu_epi32 (__m256i __W, __mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddqusi256_mask ((const int *) __P,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_loadu_epi32 (__mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_loaddqusi256_mask ((const int *) __P,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_loadu_epi32 (__m128i __W, __mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddqusi128_mask ((const int *) __P,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_loadu_epi32 (__mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_loaddqusi128_mask ((const int *) __P,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu_epi32 (void *__P, __m256i __A)
{
  *(__m256i_u *) __P = (__m256i_u) __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_storeu_epi32 (void *__P, __mmask8 __U, __m256i __A)
{
  __builtin_ia32_storedqusi256_mask ((int *) __P,
				     (__v8si) __A,
				     (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_storeu_epi32 (void *__P, __m128i __A)
{
  *(__m128i_u *) __P = (__m128i_u) __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_storeu_epi32 (void *__P, __mmask8 __U, __m128i __A)
{
  __builtin_ia32_storedqusi128_mask ((int *) __P,
				     (__v4si) __A,
				     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_abs_epi32 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsd256_mask ((__v8si) __A,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_abs_epi32 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsd256_mask ((__v8si) __A,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_abs_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsd128_mask ((__v4si) __A,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_abs_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsd128_mask ((__v4si) __A,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_abs_epi64 (__m256i __A)
{
  return (__m256i) __builtin_ia32_pabsq256_mask ((__v4di) __A,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_abs_epi64 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsq256_mask ((__v4di) __A,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_abs_epi64 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_pabsq256_mask ((__v4di) __A,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_abs_epi64 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pabsq128_mask ((__v2di) __A,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_abs_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsq128_mask ((__v2di) __A,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_abs_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pabsq128_mask ((__v2di) __A,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtpd_epu32 (__m256d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2udq256_mask ((__v4df) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtpd_epu32 (__m128i __W, __mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2udq256_mask ((__v4df) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtpd_epu32 (__mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2udq256_mask ((__v4df) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtpd_epu32 (__m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2udq128_mask ((__v2df) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtpd_epu32 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2udq128_mask ((__v2df) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtpd_epu32 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2udq128_mask ((__v2df) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttps_epi32 (__m256i __W, __mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvttps2dq256_mask ((__v8sf) __A,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttps_epi32 (__mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvttps2dq256_mask ((__v8sf) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttps_epi32 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2dq128_mask ((__v4sf) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttps_epi32 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2dq128_mask ((__v4sf) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttps_epu32 (__m256 __A)
{
  return (__m256i) __builtin_ia32_cvttps2udq256_mask ((__v8sf) __A,
						      (__v8si)
						      _mm256_setzero_si256 (),
						      (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttps_epu32 (__m256i __W, __mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvttps2udq256_mask ((__v8sf) __A,
						      (__v8si) __W,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttps_epu32 (__mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvttps2udq256_mask ((__v8sf) __A,
						      (__v8si)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttps_epu32 (__m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2udq128_mask ((__v4sf) __A,
						      (__v4si)
						      _mm_setzero_si128 (),
						      (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttps_epu32 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2udq128_mask ((__v4sf) __A,
						      (__v4si) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttps_epu32 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2udq128_mask ((__v4sf) __A,
						      (__v4si)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttpd_epi32 (__m128i __W, __mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2dq256_mask ((__v4df) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttpd_epi32 (__mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2dq256_mask ((__v4df) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttpd_epi32 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2dq128_mask ((__v2df) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttpd_epi32 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2dq128_mask ((__v2df) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttpd_epu32 (__m256d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2udq256_mask ((__v4df) __A,
						      (__v4si)
						      _mm_setzero_si128 (),
						      (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttpd_epu32 (__m128i __W, __mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2udq256_mask ((__v4df) __A,
						      (__v4si) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttpd_epu32 (__mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2udq256_mask ((__v4df) __A,
						      (__v4si)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttpd_epu32 (__m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2udq128_mask ((__v2df) __A,
						      (__v4si)
						      _mm_setzero_si128 (),
						      (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttpd_epu32 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2udq128_mask ((__v2df) __A,
						      (__v4si) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttpd_epu32 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2udq128_mask ((__v2df) __A,
						      (__v4si)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtpd_epi32 (__m128i __W, __mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2dq256_mask ((__v4df) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtpd_epi32 (__mmask8 __U, __m256d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2dq256_mask ((__v4df) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtpd_epi32 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2dq128_mask ((__v2df) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtpd_epi32 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2dq128_mask ((__v2df) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi32_pd (__m256d __W, __mmask8 __U, __m128i __A)
{
  return (__m256d) __builtin_ia32_cvtdq2pd256_mask ((__v4si) __A,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi32_pd (__mmask8 __U, __m128i __A)
{
  return (__m256d) __builtin_ia32_cvtdq2pd256_mask ((__v4si) __A,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi32_pd (__m128d __W, __mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtdq2pd128_mask ((__v4si) __A,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi32_pd (__mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtdq2pd128_mask ((__v4si) __A,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu32_pd (__m128i __A)
{
  return (__m256d) __builtin_ia32_cvtudq2pd256_mask ((__v4si) __A,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu32_pd (__m256d __W, __mmask8 __U, __m128i __A)
{
  return (__m256d) __builtin_ia32_cvtudq2pd256_mask ((__v4si) __A,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu32_pd (__mmask8 __U, __m128i __A)
{
  return (__m256d) __builtin_ia32_cvtudq2pd256_mask ((__v4si) __A,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu32_pd (__m128i __A)
{
  return (__m128d) __builtin_ia32_cvtudq2pd128_mask ((__v4si) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu32_pd (__m128d __W, __mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtudq2pd128_mask ((__v4si) __A,
						     (__v2df) __W,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu32_pd (__mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtudq2pd128_mask ((__v4si) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi32_ps (__m256 __W, __mmask8 __U, __m256i __A)
{
  return (__m256) __builtin_ia32_cvtdq2ps256_mask ((__v8si) __A,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi32_ps (__mmask8 __U, __m256i __A)
{
  return (__m256) __builtin_ia32_cvtdq2ps256_mask ((__v8si) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi32_ps (__m128 __W, __mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtdq2ps128_mask ((__v4si) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi32_ps (__mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtdq2ps128_mask ((__v4si) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu32_ps (__m256i __A)
{
  return (__m256) __builtin_ia32_cvtudq2ps256_mask ((__v8si) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu32_ps (__m256 __W, __mmask8 __U, __m256i __A)
{
  return (__m256) __builtin_ia32_cvtudq2ps256_mask ((__v8si) __A,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu32_ps (__mmask8 __U, __m256i __A)
{
  return (__m256) __builtin_ia32_cvtudq2ps256_mask ((__v8si) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu32_ps (__m128i __A)
{
  return (__m128) __builtin_ia32_cvtudq2ps128_mask ((__v4si) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu32_ps (__m128 __W, __mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtudq2ps128_mask ((__v4si) __A,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu32_ps (__mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtudq2ps128_mask ((__v4si) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtps_pd (__m256d __W, __mmask8 __U, __m128 __A)
{
  return (__m256d) __builtin_ia32_cvtps2pd256_mask ((__v4sf) __A,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtps_pd (__mmask8 __U, __m128 __A)
{
  return (__m256d) __builtin_ia32_cvtps2pd256_mask ((__v4sf) __A,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtps_pd (__m128d __W, __mmask8 __U, __m128 __A)
{
  return (__m128d) __builtin_ia32_cvtps2pd128_mask ((__v4sf) __A,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtps_pd (__mmask8 __U, __m128 __A)
{
  return (__m128d) __builtin_ia32_cvtps2pd128_mask ((__v4sf) __A,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi32_epi8 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovdb128_mask ((__v4si) __A,
						  (__v16qi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi32_storeu_epi8 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovdb128mem_mask ((__v16qi *) __P, (__v4si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi32_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovdb128_mask ((__v4si) __A,
						  (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi32_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovdb128_mask ((__v4si) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi32_epi8 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovdb256_mask ((__v8si) __A,
						  (__v16qi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi32_epi8 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovdb256_mask ((__v8si) __A,
						  (__v16qi) __O, __M);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi32_storeu_epi8 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovdb256mem_mask ((__v16qi *) __P, (__v8si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi32_epi8 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovdb256_mask ((__v8si) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsepi32_epi8 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb128_mask ((__v4si) __A,
						   (__v16qi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi32_storeu_epi8 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovsdb128mem_mask ((__v16qi *) __P, (__v4si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi32_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb128_mask ((__v4si) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtsepi32_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb128_mask ((__v4si) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtsepi32_epi8 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb256_mask ((__v8si) __A,
						   (__v16qi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi32_storeu_epi8 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovsdb256mem_mask ((__v16qi *) __P, (__v8si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi32_epi8 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb256_mask ((__v8si) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtsepi32_epi8 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb256_mask ((__v8si) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtusepi32_epi8 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb128_mask ((__v4si) __A,
						    (__v16qi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi32_storeu_epi8 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovusdb128mem_mask ((__v16qi *) __P, (__v4si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi32_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb128_mask ((__v4si) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtusepi32_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb128_mask ((__v4si) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtusepi32_epi8 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb256_mask ((__v8si) __A,
						    (__v16qi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi32_storeu_epi8 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovusdb256mem_mask ((__v16qi*) __P, (__v8si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi32_epi8 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb256_mask ((__v8si) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtusepi32_epi8 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb256_mask ((__v8si) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi32_epi16 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovdw128_mask ((__v4si) __A,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi32_storeu_epi16 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovdw128mem_mask ((__v8hi *) __P, (__v4si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi32_epi16 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovdw128_mask ((__v4si) __A,
						  (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi32_epi16 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovdw128_mask ((__v4si) __A,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi32_epi16 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovdw256_mask ((__v8si) __A,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi32_storeu_epi16 (void *  __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovdw256mem_mask ((__v8hi *) __P, (__v8si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi32_epi16 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovdw256_mask ((__v8si) __A,
						  (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi32_epi16 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovdw256_mask ((__v8si) __A,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsepi32_epi16 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsdw128_mask ((__v4si) __A,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi32_storeu_epi16 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovsdw128mem_mask ((__v8hi *) __P, (__v4si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi32_epi16 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsdw128_mask ((__v4si) __A,
						   (__v8hi)__O,
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtsepi32_epi16 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsdw128_mask ((__v4si) __A,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtsepi32_epi16 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsdw256_mask ((__v8si) __A,
						   (__v8hi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi32_storeu_epi16 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovsdw256mem_mask ((__v8hi *) __P, (__v8si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi32_epi16 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsdw256_mask ((__v8si) __A,
						   (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtsepi32_epi16 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsdw256_mask ((__v8si) __A,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtusepi32_epi16 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusdw128_mask ((__v4si) __A,
						    (__v8hi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi32_storeu_epi16 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovusdw128mem_mask ((__v8hi *) __P, (__v4si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi32_epi16 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusdw128_mask ((__v4si) __A,
						    (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtusepi32_epi16 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusdw128_mask ((__v4si) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtusepi32_epi16 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusdw256_mask ((__v8si) __A,
						    (__v8hi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi32_storeu_epi16 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovusdw256mem_mask ((__v8hi *) __P, (__v8si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi32_epi16 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusdw256_mask ((__v8si) __A,
						    (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtusepi32_epi16 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusdw256_mask ((__v8si) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi64_epi8 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqb128_mask ((__v2di) __A,
						  (__v16qi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_storeu_epi8 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovqb128mem_mask ((__v16qi *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqb128_mask ((__v2di) __A,
						  (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi64_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqb128_mask ((__v2di) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi64_epi8 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqb256_mask ((__v4di) __A,
						  (__v16qi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_storeu_epi8 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovqb256mem_mask ((__v16qi *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_epi8 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqb256_mask ((__v4di) __A,
						  (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi64_epi8 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqb256_mask ((__v4di) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsepi64_epi8 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb128_mask ((__v2di) __A,
						   (__v16qi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi64_storeu_epi8 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovsqb128mem_mask ((__v16qi *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi64_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb128_mask ((__v2di) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtsepi64_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb128_mask ((__v2di) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtsepi64_epi8 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb256_mask ((__v4di) __A,
						   (__v16qi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi64_storeu_epi8 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovsqb256mem_mask ((__v16qi *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi64_epi8 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb256_mask ((__v4di) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtsepi64_epi8 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb256_mask ((__v4di) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtusepi64_epi8 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb128_mask ((__v2di) __A,
						    (__v16qi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi64_storeu_epi8 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovusqb128mem_mask ((__v16qi *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi64_epi8 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb128_mask ((__v2di) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtusepi64_epi8 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb128_mask ((__v2di) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtusepi64_epi8 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb256_mask ((__v4di) __A,
						    (__v16qi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi64_storeu_epi8 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovusqb256mem_mask ((__v16qi *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi64_epi8 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb256_mask ((__v4di) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtusepi64_epi8 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb256_mask ((__v4di) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi64_epi16 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqw128_mask ((__v2di) __A,
						  (__v8hi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_storeu_epi16 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovqw128mem_mask ((__v8hi *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_epi16 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqw128_mask ((__v2di) __A,
						  (__v8hi)__O,
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi64_epi16 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqw128_mask ((__v2di) __A,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi64_epi16 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqw256_mask ((__v4di) __A,
						  (__v8hi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_storeu_epi16 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovqw256mem_mask ((__v8hi *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_epi16 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqw256_mask ((__v4di) __A,
						  (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi64_epi16 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqw256_mask ((__v4di) __A,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsepi64_epi16 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw128_mask ((__v2di) __A,
						   (__v8hi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi64_storeu_epi16 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovsqw128mem_mask ((__v8hi *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi64_epi16 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw128_mask ((__v2di) __A,
						   (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtsepi64_epi16 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw128_mask ((__v2di) __A,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtsepi64_epi16 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw256_mask ((__v4di) __A,
						   (__v8hi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi64_storeu_epi16 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovsqw256mem_mask ((__v8hi *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi64_epi16 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw256_mask ((__v4di) __A,
						   (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtsepi64_epi16 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw256_mask ((__v4di) __A,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtusepi64_epi16 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw128_mask ((__v2di) __A,
						    (__v8hi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi64_storeu_epi16 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovusqw128mem_mask ((__v8hi *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi64_epi16 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw128_mask ((__v2di) __A,
						    (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtusepi64_epi16 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw128_mask ((__v2di) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtusepi64_epi16 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw256_mask ((__v4di) __A,
						    (__v8hi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi64_storeu_epi16 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovusqw256mem_mask ((__v8hi *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi64_epi16 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw256_mask ((__v4di) __A,
						    (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtusepi64_epi16 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw256_mask ((__v4di) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi64_epi32 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqd128_mask ((__v2di) __A,
						  (__v4si)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_storeu_epi32 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovqd128mem_mask ((__v4si *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_epi32 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqd128_mask ((__v2di) __A,
						  (__v4si) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi64_epi32 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovqd128_mask ((__v2di) __A,
						  (__v4si)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi64_epi32 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqd256_mask ((__v4di) __A,
						  (__v4si)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_storeu_epi32 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovqd256mem_mask ((__v4si *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_epi32 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqd256_mask ((__v4di) __A,
						  (__v4si) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi64_epi32 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovqd256_mask ((__v4di) __A,
						  (__v4si)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsepi64_epi32 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqd128_mask ((__v2di) __A,
						   (__v4si)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi64_storeu_epi32 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovsqd128mem_mask ((__v4si *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtsepi64_epi32 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqd128_mask ((__v2di) __A,
						   (__v4si) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtsepi64_epi32 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsqd128_mask ((__v2di) __A,
						   (__v4si)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtsepi64_epi32 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqd256_mask ((__v4di) __A,
						   (__v4si)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi64_storeu_epi32 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovsqd256mem_mask ((__v4si *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtsepi64_epi32 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqd256_mask ((__v4di) __A,
						   (__v4si)__O,
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtsepi64_epi32 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovsqd256_mask ((__v4di) __A,
						   (__v4si)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtusepi64_epi32 (__m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqd128_mask ((__v2di) __A,
						    (__v4si)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi64_storeu_epi32 (void * __P, __mmask8 __M, __m128i __A)
{
  __builtin_ia32_pmovusqd128mem_mask ((__v4si *) __P, (__v2di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtusepi64_epi32 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqd128_mask ((__v2di) __A,
						    (__v4si) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtusepi64_epi32 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovusqd128_mask ((__v2di) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtusepi64_epi32 (__m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqd256_mask ((__v4di) __A,
						    (__v4si)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi64_storeu_epi32 (void * __P, __mmask8 __M, __m256i __A)
{
  __builtin_ia32_pmovusqd256mem_mask ((__v4si *) __P, (__v4di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtusepi64_epi32 (__m128i __O, __mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqd256_mask ((__v4di) __A,
						    (__v4si) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtusepi64_epi32 (__mmask8 __M, __m256i __A)
{
  return (__m128i) __builtin_ia32_pmovusqd256_mask ((__v4di) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcastss_ps (__m256 __O, __mmask8 __M, __m128 __A)
{
  return (__m256) __builtin_ia32_broadcastss256_mask ((__v4sf) __A,
						      (__v8sf) __O,
						      __M);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcastss_ps (__mmask8 __M, __m128 __A)
{
  return (__m256) __builtin_ia32_broadcastss256_mask ((__v4sf) __A,
						      (__v8sf)
						      _mm256_setzero_ps (),
						      __M);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_broadcastss_ps (__m128 __O, __mmask8 __M, __m128 __A)
{
  return (__m128) __builtin_ia32_broadcastss128_mask ((__v4sf) __A,
						      (__v4sf) __O,
						      __M);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_broadcastss_ps (__mmask8 __M, __m128 __A)
{
  return (__m128) __builtin_ia32_broadcastss128_mask ((__v4sf) __A,
						      (__v4sf)
						      _mm_setzero_ps (),
						      __M);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcastsd_pd (__m256d __O, __mmask8 __M, __m128d __A)
{
  return (__m256d) __builtin_ia32_broadcastsd256_mask ((__v2df) __A,
						       (__v4df) __O,
						       __M);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcastsd_pd (__mmask8 __M, __m128d __A)
{
  return (__m256d) __builtin_ia32_broadcastsd256_mask ((__v2df) __A,
						       (__v4df)
						       _mm256_setzero_pd (),
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcastd_epi32 (__m256i __O, __mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastd256_mask ((__v4si) __A,
						       (__v8si) __O,
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcastd_epi32 (__mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastd256_mask ((__v4si) __A,
						       (__v8si)
						       _mm256_setzero_si256 (),
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_set1_epi32 (__m256i __O, __mmask8 __M, int __A)
{
  return (__m256i) __builtin_ia32_pbroadcastd256_gpr_mask (__A, (__v8si) __O,
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_set1_epi32 (__mmask8 __M, int __A)
{
  return (__m256i) __builtin_ia32_pbroadcastd256_gpr_mask (__A,
							   (__v8si)
							   _mm256_setzero_si256 (),
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_broadcastd_epi32 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastd128_mask ((__v4si) __A,
						       (__v4si) __O,
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_broadcastd_epi32 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastd128_mask ((__v4si) __A,
						       (__v4si)
						       _mm_setzero_si128 (),
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_set1_epi32 (__m128i __O, __mmask8 __M, int __A)
{
  return (__m128i) __builtin_ia32_pbroadcastd128_gpr_mask (__A, (__v4si) __O,
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_set1_epi32 (__mmask8 __M, int __A)
{
  return (__m128i)
	 __builtin_ia32_pbroadcastd128_gpr_mask (__A,
						 (__v4si) _mm_setzero_si128 (),
						 __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcastq_epi64 (__m256i __O, __mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastq256_mask ((__v2di) __A,
						       (__v4di) __O,
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcastq_epi64 (__mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_pbroadcastq256_mask ((__v2di) __A,
						       (__v4di)
						       _mm256_setzero_si256 (),
						       __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_set1_epi64 (__m256i __O, __mmask8 __M, long long __A)
{
  return (__m256i) __builtin_ia32_pbroadcastq256_gpr_mask (__A, (__v4di) __O,
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_set1_epi64 (__mmask8 __M, long long __A)
{
  return (__m256i) __builtin_ia32_pbroadcastq256_gpr_mask (__A,
							   (__v4di)
							   _mm256_setzero_si256 (),
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_broadcastq_epi64 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastq128_mask ((__v2di) __A,
						       (__v2di) __O,
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_broadcastq_epi64 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_pbroadcastq128_mask ((__v2di) __A,
						       (__v2di)
						       _mm_setzero_si128 (),
						       __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_set1_epi64 (__m128i __O, __mmask8 __M, long long __A)
{
  return (__m128i) __builtin_ia32_pbroadcastq128_gpr_mask (__A, (__v2di) __O,
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_set1_epi64 (__mmask8 __M, long long __A)
{
  return (__m128i)
	 __builtin_ia32_pbroadcastq128_gpr_mask (__A,
						 (__v2di) _mm_setzero_si128 (),
						 __M);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_f32x4 (__m128 __A)
{
  return (__m256) __builtin_ia32_broadcastf32x4_256_mask ((__v4sf) __A,
						          (__v8sf)_mm256_undefined_pd (),
							  (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcast_f32x4 (__m256 __O, __mmask8 __M, __m128 __A)
{
  return (__m256) __builtin_ia32_broadcastf32x4_256_mask ((__v4sf) __A,
							  (__v8sf) __O,
							  __M);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcast_f32x4 (__mmask8 __M, __m128 __A)
{
  return (__m256) __builtin_ia32_broadcastf32x4_256_mask ((__v4sf) __A,
							  (__v8sf)
							  _mm256_setzero_ps (),
							  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_i32x4 (__m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti32x4_256_mask ((__v4si)
							   __A,
						           (__v8si)_mm256_undefined_si256 (),
							   (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcast_i32x4 (__m256i __O, __mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti32x4_256_mask ((__v4si)
							   __A,
							   (__v8si)
							   __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcast_i32x4 (__mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti32x4_256_mask ((__v4si)
							   __A,
							   (__v8si)
							   _mm256_setzero_si256 (),
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi8_epi32 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxbd256_mask ((__v16qi) __A,
						    (__v8si) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi8_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxbd256_mask ((__v16qi) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi8_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxbd128_mask ((__v16qi) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi8_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxbd128_mask ((__v16qi) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi8_epi64 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxbq256_mask ((__v16qi) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi8_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxbq256_mask ((__v16qi) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi8_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxbq128_mask ((__v16qi) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi8_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxbq128_mask ((__v16qi) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi16_epi32 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxwd256_mask ((__v8hi) __A,
						    (__v8si) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi16_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxwd256_mask ((__v8hi) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi16_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxwd128_mask ((__v8hi) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi16_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxwd128_mask ((__v8hi) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi16_epi64 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxwq256_mask ((__v8hi) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi16_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovsxwq256_mask ((__v8hi) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi16_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxwq128_mask ((__v8hi) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi16_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovsxwq128_mask ((__v8hi) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi32_epi64 (__m256i __W, __mmask8 __U, __m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxdq256_mask ((__v4si) __X,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi32_epi64 (__mmask8 __U, __m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxdq256_mask ((__v4si) __X,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi32_epi64 (__m128i __W, __mmask8 __U, __m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxdq128_mask ((__v4si) __X,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi32_epi64 (__mmask8 __U, __m128i __X)
{
  return (__m128i) __builtin_ia32_pmovsxdq128_mask ((__v4si) __X,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu8_epi32 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxbd256_mask ((__v16qi) __A,
						    (__v8si) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu8_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxbd256_mask ((__v16qi) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu8_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxbd128_mask ((__v16qi) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu8_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxbd128_mask ((__v16qi) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu8_epi64 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxbq256_mask ((__v16qi) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu8_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxbq256_mask ((__v16qi) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu8_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxbq128_mask ((__v16qi) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu8_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxbq128_mask ((__v16qi) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu16_epi32 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxwd256_mask ((__v8hi) __A,
						    (__v8si) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu16_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxwd256_mask ((__v8hi) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu16_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxwd128_mask ((__v8hi) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu16_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxwd128_mask ((__v8hi) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu16_epi64 (__m256i __W, __mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxwq256_mask ((__v8hi) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu16_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m256i) __builtin_ia32_pmovzxwq256_mask ((__v8hi) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu16_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxwq128_mask ((__v8hi) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu16_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_pmovzxwq128_mask ((__v8hi) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu32_epi64 (__m256i __W, __mmask8 __U, __m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxdq256_mask ((__v4si) __X,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu32_epi64 (__mmask8 __U, __m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxdq256_mask ((__v4si) __X,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu32_epi64 (__m128i __W, __mmask8 __U, __m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxdq128_mask ((__v4si) __X,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu32_epi64 (__mmask8 __U, __m128i __X)
{
  return (__m128i) __builtin_ia32_pmovzxdq128_mask ((__v4si) __X,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rcp14_pd (__m256d __A)
{
  return (__m256d) __builtin_ia32_rcp14pd256_mask ((__v4df) __A,
					      (__v4df)
					      _mm256_setzero_pd (),
					      (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rcp14_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_rcp14pd256_mask ((__v4df) __A,
					      (__v4df) __W,
					      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rcp14_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_rcp14pd256_mask ((__v4df) __A,
					      (__v4df)
					      _mm256_setzero_pd (),
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rcp14_pd (__m128d __A)
{
  return (__m128d) __builtin_ia32_rcp14pd128_mask ((__v2df) __A,
					      (__v2df)
					      _mm_setzero_pd (),
					      (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rcp14_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_rcp14pd128_mask ((__v2df) __A,
					      (__v2df) __W,
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rcp14_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_rcp14pd128_mask ((__v2df) __A,
					      (__v2df)
					      _mm_setzero_pd (),
					      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rcp14_ps (__m256 __A)
{
  return (__m256) __builtin_ia32_rcp14ps256_mask ((__v8sf) __A,
					     (__v8sf)
					     _mm256_setzero_ps (),
					     (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rcp14_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_rcp14ps256_mask ((__v8sf) __A,
					     (__v8sf) __W,
					     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rcp14_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_rcp14ps256_mask ((__v8sf) __A,
					     (__v8sf)
					     _mm256_setzero_ps (),
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rcp14_ps (__m128 __A)
{
  return (__m128) __builtin_ia32_rcp14ps128_mask ((__v4sf) __A,
					     (__v4sf)
					     _mm_setzero_ps (),
					     (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rcp14_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_rcp14ps128_mask ((__v4sf) __A,
					     (__v4sf) __W,
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rcp14_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_rcp14ps128_mask ((__v4sf) __A,
					     (__v4sf)
					     _mm_setzero_ps (),
					     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rsqrt14_pd (__m256d __A)
{
  return (__m256d) __builtin_ia32_rsqrt14pd256_mask ((__v4df) __A,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rsqrt14_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_rsqrt14pd256_mask ((__v4df) __A,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rsqrt14_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_rsqrt14pd256_mask ((__v4df) __A,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rsqrt14_pd (__m128d __A)
{
  return (__m128d) __builtin_ia32_rsqrt14pd128_mask ((__v2df) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rsqrt14_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_rsqrt14pd128_mask ((__v2df) __A,
						     (__v2df) __W,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rsqrt14_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_rsqrt14pd128_mask ((__v2df) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rsqrt14_ps (__m256 __A)
{
  return (__m256) __builtin_ia32_rsqrt14ps256_mask ((__v8sf) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rsqrt14_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_rsqrt14ps256_mask ((__v8sf) __A,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rsqrt14_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_rsqrt14ps256_mask ((__v8sf) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rsqrt14_ps (__m128 __A)
{
  return (__m128) __builtin_ia32_rsqrt14ps128_mask ((__v4sf) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rsqrt14_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_rsqrt14ps128_mask ((__v4sf) __A,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rsqrt14_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_rsqrt14ps128_mask ((__v4sf) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sqrt_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_sqrtpd256_mask ((__v4df) __A,
						  (__v4df) __W,
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sqrt_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_sqrtpd256_mask ((__v4df) __A,
						  (__v4df)
						  _mm256_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sqrt_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_sqrtpd128_mask ((__v2df) __A,
						  (__v2df) __W,
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sqrt_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_sqrtpd128_mask ((__v2df) __A,
						  (__v2df)
						  _mm_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sqrt_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_sqrtps256_mask ((__v8sf) __A,
						 (__v8sf) __W,
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sqrt_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_sqrtps256_mask ((__v8sf) __A,
						 (__v8sf)
						 _mm256_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sqrt_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_sqrtps128_mask ((__v4sf) __A,
						 (__v4sf) __W,
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sqrt_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_sqrtps128_mask ((__v4sf) __A,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_add_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_paddd256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_add_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddd256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_add_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_paddq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_add_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_paddq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sub_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_psubd256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sub_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubd256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sub_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_psubq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sub_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_psubq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_paddd128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddd128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_paddq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_paddq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psubd128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubd128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psubq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psubq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_getexp_ps (__m256 __A)
{
  return (__m256) __builtin_ia32_getexpps256_mask ((__v8sf) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_getexp_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_getexpps256_mask ((__v8sf) __A,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_getexp_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_getexpps256_mask ((__v8sf) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_getexp_pd (__m256d __A)
{
  return (__m256d) __builtin_ia32_getexppd256_mask ((__v4df) __A,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_getexp_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_getexppd256_mask ((__v4df) __A,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_getexp_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_getexppd256_mask ((__v4df) __A,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getexp_ps (__m128 __A)
{
  return (__m128) __builtin_ia32_getexpps128_mask ((__v4sf) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getexp_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_getexpps128_mask ((__v4sf) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getexp_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_getexpps128_mask ((__v4sf) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getexp_pd (__m128d __A)
{
  return (__m128d) __builtin_ia32_getexppd128_mask ((__v2df) __A,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getexp_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_getexppd128_mask ((__v2df) __A,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getexp_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_getexppd128_mask ((__v2df) __A,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srl_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psrld256_mask ((__v8si) __A,
						 (__v4si) __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srl_epi32 (__mmask8 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psrld256_mask ((__v8si) __A,
						 (__v4si) __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srl_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psrld128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srl_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrld128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srl_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psrlq256_mask ((__v4di) __A,
						 (__v2di) __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srl_epi64 (__mmask8 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psrlq256_mask ((__v4di) __A,
						 (__v2di) __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srl_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psrlq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srl_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrlq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_and_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pandd256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_and_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pandd256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_scalef_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_scalefpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_scalef_pd (__m256d __W, __mmask8 __U, __m256d __A,
		       __m256d __B)
{
  return (__m256d) __builtin_ia32_scalefpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_scalef_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_scalefpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_scalef_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_scalefps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_scalef_ps (__m256 __W, __mmask8 __U, __m256 __A,
		       __m256 __B)
{
  return (__m256) __builtin_ia32_scalefps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_scalef_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_scalefps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_scalef_pd (__m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_scalefpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_scalef_pd (__m128d __W, __mmask8 __U, __m128d __A,
		    __m128d __B)
{
  return (__m128d) __builtin_ia32_scalefpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_scalef_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_scalefpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_scalef_ps (__m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_scalefps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_scalef_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_scalefps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_scalef_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_scalefps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmadd_pd (__m256d __A, __mmask8 __U, __m256d __B,
		      __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmaddpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df) __C,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmadd_pd (__m256d __A, __m256d __B, __m256d __C,
		       __mmask8 __U)
{
  return (__m256d) __builtin_ia32_vfmaddpd256_mask3 ((__v4df) __A,
						     (__v4df) __B,
						     (__v4df) __C,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmadd_pd (__mmask8 __U, __m256d __A, __m256d __B,
		       __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmaddpd256_maskz ((__v4df) __A,
						     (__v4df) __B,
						     (__v4df) __C,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmadd_pd (__m128d __A, __mmask8 __U, __m128d __B, __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmaddpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df) __C,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmadd_pd (__m128d __A, __m128d __B, __m128d __C,
		    __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmaddpd128_mask3 ((__v2df) __A,
						     (__v2df) __B,
						     (__v2df) __C,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmadd_pd (__mmask8 __U, __m128d __A, __m128d __B,
		    __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmaddpd128_maskz ((__v2df) __A,
						     (__v2df) __B,
						     (__v2df) __C,
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmadd_ps (__m256 __A, __mmask8 __U, __m256 __B, __m256 __C)
{
  return (__m256) __builtin_ia32_vfmaddps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf) __C,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmadd_ps (__m256 __A, __m256 __B, __m256 __C,
		       __mmask8 __U)
{
  return (__m256) __builtin_ia32_vfmaddps256_mask3 ((__v8sf) __A,
						    (__v8sf) __B,
						    (__v8sf) __C,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmadd_ps (__mmask8 __U, __m256 __A, __m256 __B,
		       __m256 __C)
{
  return (__m256) __builtin_ia32_vfmaddps256_maskz ((__v8sf) __A,
						    (__v8sf) __B,
						    (__v8sf) __C,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmadd_ps (__m128 __A, __mmask8 __U, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfmaddps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf) __C,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmadd_ps (__m128 __A, __m128 __B, __m128 __C, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmaddps128_mask3 ((__v4sf) __A,
						    (__v4sf) __B,
						    (__v4sf) __C,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmadd_ps (__mmask8 __U, __m128 __A, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfmaddps128_maskz ((__v4sf) __A,
						    (__v4sf) __B,
						    (__v4sf) __C,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmsub_pd (__m256d __A, __mmask8 __U, __m256d __B,
		      __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmsubpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df) __C,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmsub_pd (__m256d __A, __m256d __B, __m256d __C,
		       __mmask8 __U)
{
  return (__m256d) __builtin_ia32_vfmsubpd256_mask3 ((__v4df) __A,
						     (__v4df) __B,
						     (__v4df) __C,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmsub_pd (__mmask8 __U, __m256d __A, __m256d __B,
		       __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmsubpd256_maskz ((__v4df) __A,
						     (__v4df) __B,
						     (__v4df) __C,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsub_pd (__m128d __A, __mmask8 __U, __m128d __B, __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmsubpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df) __C,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsub_pd (__m128d __A, __m128d __B, __m128d __C,
		    __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmsubpd128_mask3 ((__v2df) __A,
						     (__v2df) __B,
						     (__v2df) __C,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsub_pd (__mmask8 __U, __m128d __A, __m128d __B,
		    __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmsubpd128_maskz ((__v2df) __A,
						     (__v2df) __B,
						     (__v2df) __C,
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmsub_ps (__m256 __A, __mmask8 __U, __m256 __B, __m256 __C)
{
  return (__m256) __builtin_ia32_vfmsubps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf) __C,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmsub_ps (__m256 __A, __m256 __B, __m256 __C,
		       __mmask8 __U)
{
  return (__m256) __builtin_ia32_vfmsubps256_mask3 ((__v8sf) __A,
						    (__v8sf) __B,
						    (__v8sf) __C,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmsub_ps (__mmask8 __U, __m256 __A, __m256 __B,
		       __m256 __C)
{
  return (__m256) __builtin_ia32_vfmsubps256_maskz ((__v8sf) __A,
						    (__v8sf) __B,
						    (__v8sf) __C,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsub_ps (__m128 __A, __mmask8 __U, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfmsubps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf) __C,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsub_ps (__m128 __A, __m128 __B, __m128 __C, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmsubps128_mask3 ((__v4sf) __A,
						    (__v4sf) __B,
						    (__v4sf) __C,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsub_ps (__mmask8 __U, __m128 __A, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfmsubps128_maskz ((__v4sf) __A,
						    (__v4sf) __B,
						    (__v4sf) __C,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmaddsub_pd (__m256d __A, __mmask8 __U, __m256d __B,
			 __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmaddsubpd256_mask ((__v4df) __A,
						       (__v4df) __B,
						       (__v4df) __C,
						       (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmaddsub_pd (__m256d __A, __m256d __B, __m256d __C,
			  __mmask8 __U)
{
  return (__m256d) __builtin_ia32_vfmaddsubpd256_mask3 ((__v4df) __A,
							(__v4df) __B,
							(__v4df) __C,
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmaddsub_pd (__mmask8 __U, __m256d __A, __m256d __B,
			  __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmaddsubpd256_maskz ((__v4df) __A,
							(__v4df) __B,
							(__v4df) __C,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmaddsub_pd (__m128d __A, __mmask8 __U, __m128d __B,
		      __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmaddsubpd128_mask ((__v2df) __A,
						       (__v2df) __B,
						       (__v2df) __C,
						       (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmaddsub_pd (__m128d __A, __m128d __B, __m128d __C,
		       __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmaddsubpd128_mask3 ((__v2df) __A,
							(__v2df) __B,
							(__v2df) __C,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmaddsub_pd (__mmask8 __U, __m128d __A, __m128d __B,
		       __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmaddsubpd128_maskz ((__v2df) __A,
							(__v2df) __B,
							(__v2df) __C,
							(__mmask8)
							__U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmaddsub_ps (__m256 __A, __mmask8 __U, __m256 __B,
			 __m256 __C)
{
  return (__m256) __builtin_ia32_vfmaddsubps256_mask ((__v8sf) __A,
						      (__v8sf) __B,
						      (__v8sf) __C,
						      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmaddsub_ps (__m256 __A, __m256 __B, __m256 __C,
			  __mmask8 __U)
{
  return (__m256) __builtin_ia32_vfmaddsubps256_mask3 ((__v8sf) __A,
						       (__v8sf) __B,
						       (__v8sf) __C,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmaddsub_ps (__mmask8 __U, __m256 __A, __m256 __B,
			  __m256 __C)
{
  return (__m256) __builtin_ia32_vfmaddsubps256_maskz ((__v8sf) __A,
						       (__v8sf) __B,
						       (__v8sf) __C,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmaddsub_ps (__m128 __A, __mmask8 __U, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfmaddsubps128_mask ((__v4sf) __A,
						      (__v4sf) __B,
						      (__v4sf) __C,
						      (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmaddsub_ps (__m128 __A, __m128 __B, __m128 __C,
		       __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmaddsubps128_mask3 ((__v4sf) __A,
						       (__v4sf) __B,
						       (__v4sf) __C,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmaddsub_ps (__mmask8 __U, __m128 __A, __m128 __B,
		       __m128 __C)
{
  return (__m128) __builtin_ia32_vfmaddsubps128_maskz ((__v4sf) __A,
						       (__v4sf) __B,
						       (__v4sf) __C,
						       (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmsubadd_pd (__m256d __A, __mmask8 __U, __m256d __B,
			 __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmaddsubpd256_mask ((__v4df) __A,
						       (__v4df) __B,
						       -(__v4df) __C,
						       (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmsubadd_pd (__m256d __A, __m256d __B, __m256d __C,
			  __mmask8 __U)
{
  return (__m256d) __builtin_ia32_vfmsubaddpd256_mask3 ((__v4df) __A,
							(__v4df) __B,
							(__v4df) __C,
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmsubadd_pd (__mmask8 __U, __m256d __A, __m256d __B,
			  __m256d __C)
{
  return (__m256d) __builtin_ia32_vfmaddsubpd256_maskz ((__v4df) __A,
							(__v4df) __B,
							-(__v4df) __C,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsubadd_pd (__m128d __A, __mmask8 __U, __m128d __B,
		      __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmaddsubpd128_mask ((__v2df) __A,
						       (__v2df) __B,
						       -(__v2df) __C,
						       (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsubadd_pd (__m128d __A, __m128d __B, __m128d __C,
		       __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmsubaddpd128_mask3 ((__v2df) __A,
							(__v2df) __B,
							(__v2df) __C,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsubadd_pd (__mmask8 __U, __m128d __A, __m128d __B,
		       __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmaddsubpd128_maskz ((__v2df) __A,
							(__v2df) __B,
							-(__v2df) __C,
							(__mmask8)
							__U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fmsubadd_ps (__m256 __A, __mmask8 __U, __m256 __B,
			 __m256 __C)
{
  return (__m256) __builtin_ia32_vfmaddsubps256_mask ((__v8sf) __A,
						      (__v8sf) __B,
						      -(__v8sf) __C,
						      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fmsubadd_ps (__m256 __A, __m256 __B, __m256 __C,
			  __mmask8 __U)
{
  return (__m256) __builtin_ia32_vfmsubaddps256_mask3 ((__v8sf) __A,
						       (__v8sf) __B,
						       (__v8sf) __C,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fmsubadd_ps (__mmask8 __U, __m256 __A, __m256 __B,
			  __m256 __C)
{
  return (__m256) __builtin_ia32_vfmaddsubps256_maskz ((__v8sf) __A,
						       (__v8sf) __B,
						       -(__v8sf) __C,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsubadd_ps (__m128 __A, __mmask8 __U, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfmaddsubps128_mask ((__v4sf) __A,
						      (__v4sf) __B,
						      -(__v4sf) __C,
						      (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsubadd_ps (__m128 __A, __m128 __B, __m128 __C,
		       __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmsubaddps128_mask3 ((__v4sf) __A,
						       (__v4sf) __B,
						       (__v4sf) __C,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsubadd_ps (__mmask8 __U, __m128 __A, __m128 __B,
		       __m128 __C)
{
  return (__m128) __builtin_ia32_vfmaddsubps128_maskz ((__v4sf) __A,
						       (__v4sf) __B,
						       -(__v4sf) __C,
						       (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fnmadd_pd (__m256d __A, __mmask8 __U, __m256d __B,
		       __m256d __C)
{
  return (__m256d) __builtin_ia32_vfnmaddpd256_mask ((__v4df) __A,
						     (__v4df) __B,
						     (__v4df) __C,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fnmadd_pd (__m256d __A, __m256d __B, __m256d __C,
			__mmask8 __U)
{
  return (__m256d) __builtin_ia32_vfnmaddpd256_mask3 ((__v4df) __A,
						      (__v4df) __B,
						      (__v4df) __C,
						      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fnmadd_pd (__mmask8 __U, __m256d __A, __m256d __B,
			__m256d __C)
{
  return (__m256d) __builtin_ia32_vfnmaddpd256_maskz ((__v4df) __A,
						      (__v4df) __B,
						      (__v4df) __C,
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmadd_pd (__m128d __A, __mmask8 __U, __m128d __B,
		    __m128d __C)
{
  return (__m128d) __builtin_ia32_vfnmaddpd128_mask ((__v2df) __A,
						     (__v2df) __B,
						     (__v2df) __C,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmadd_pd (__m128d __A, __m128d __B, __m128d __C,
		     __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfnmaddpd128_mask3 ((__v2df) __A,
						      (__v2df) __B,
						      (__v2df) __C,
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmadd_pd (__mmask8 __U, __m128d __A, __m128d __B,
		     __m128d __C)
{
  return (__m128d) __builtin_ia32_vfnmaddpd128_maskz ((__v2df) __A,
						      (__v2df) __B,
						      (__v2df) __C,
						      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fnmadd_ps (__m256 __A, __mmask8 __U, __m256 __B,
		       __m256 __C)
{
  return (__m256) __builtin_ia32_vfnmaddps256_mask ((__v8sf) __A,
						    (__v8sf) __B,
						    (__v8sf) __C,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fnmadd_ps (__m256 __A, __m256 __B, __m256 __C,
			__mmask8 __U)
{
  return (__m256) __builtin_ia32_vfnmaddps256_mask3 ((__v8sf) __A,
						     (__v8sf) __B,
						     (__v8sf) __C,
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fnmadd_ps (__mmask8 __U, __m256 __A, __m256 __B,
			__m256 __C)
{
  return (__m256) __builtin_ia32_vfnmaddps256_maskz ((__v8sf) __A,
						     (__v8sf) __B,
						     (__v8sf) __C,
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmadd_ps (__m128 __A, __mmask8 __U, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfnmaddps128_mask ((__v4sf) __A,
						    (__v4sf) __B,
						    (__v4sf) __C,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmadd_ps (__m128 __A, __m128 __B, __m128 __C, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfnmaddps128_mask3 ((__v4sf) __A,
						     (__v4sf) __B,
						     (__v4sf) __C,
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmadd_ps (__mmask8 __U, __m128 __A, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfnmaddps128_maskz ((__v4sf) __A,
						     (__v4sf) __B,
						     (__v4sf) __C,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fnmsub_pd (__m256d __A, __mmask8 __U, __m256d __B,
		       __m256d __C)
{
  return (__m256d) __builtin_ia32_vfnmsubpd256_mask ((__v4df) __A,
						     (__v4df) __B,
						     (__v4df) __C,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fnmsub_pd (__m256d __A, __m256d __B, __m256d __C,
			__mmask8 __U)
{
  return (__m256d) __builtin_ia32_vfnmsubpd256_mask3 ((__v4df) __A,
						      (__v4df) __B,
						      (__v4df) __C,
						      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fnmsub_pd (__mmask8 __U, __m256d __A, __m256d __B,
			__m256d __C)
{
  return (__m256d) __builtin_ia32_vfnmsubpd256_maskz ((__v4df) __A,
						      (__v4df) __B,
						      (__v4df) __C,
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmsub_pd (__m128d __A, __mmask8 __U, __m128d __B,
		    __m128d __C)
{
  return (__m128d) __builtin_ia32_vfnmsubpd128_mask ((__v2df) __A,
						     (__v2df) __B,
						     (__v2df) __C,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmsub_pd (__m128d __A, __m128d __B, __m128d __C,
		     __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfnmsubpd128_mask3 ((__v2df) __A,
						      (__v2df) __B,
						      (__v2df) __C,
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmsub_pd (__mmask8 __U, __m128d __A, __m128d __B,
		     __m128d __C)
{
  return (__m128d) __builtin_ia32_vfnmsubpd128_maskz ((__v2df) __A,
						      (__v2df) __B,
						      (__v2df) __C,
						      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fnmsub_ps (__m256 __A, __mmask8 __U, __m256 __B,
		       __m256 __C)
{
  return (__m256) __builtin_ia32_vfnmsubps256_mask ((__v8sf) __A,
						    (__v8sf) __B,
						    (__v8sf) __C,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask3_fnmsub_ps (__m256 __A, __m256 __B, __m256 __C,
			__mmask8 __U)
{
  return (__m256) __builtin_ia32_vfnmsubps256_mask3 ((__v8sf) __A,
						     (__v8sf) __B,
						     (__v8sf) __C,
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fnmsub_ps (__mmask8 __U, __m256 __A, __m256 __B,
			__m256 __C)
{
  return (__m256) __builtin_ia32_vfnmsubps256_maskz ((__v8sf) __A,
						     (__v8sf) __B,
						     (__v8sf) __C,
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmsub_ps (__m128 __A, __mmask8 __U, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfnmsubps128_mask ((__v4sf) __A,
						    (__v4sf) __B,
						    (__v4sf) __C,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmsub_ps (__m128 __A, __m128 __B, __m128 __C, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfnmsubps128_mask3 ((__v4sf) __A,
						     (__v4sf) __B,
						     (__v4sf) __C,
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmsub_ps (__mmask8 __U, __m128 __A, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfnmsubps128_maskz ((__v4sf) __A,
						     (__v4sf) __B,
						     (__v4sf) __C,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_and_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pandd128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_and_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pandd128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_andnot_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			  __m256i __B)
{
  return (__m256i) __builtin_ia32_pandnd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_andnot_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pandnd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_andnot_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		       __m128i __B)
{
  return (__m128i) __builtin_ia32_pandnd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_andnot_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pandnd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_or_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_pord256_mask ((__v8si) __A,
						(__v8si) __B,
						(__v8si) __W,
						(__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_or_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pord256_mask ((__v8si) __A,
						(__v8si) __B,
						(__v8si)
						_mm256_setzero_si256 (),
						(__mmask8) __U);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_or_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v8su)__A | (__v8su)__B);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_or_epi32 (__m128i __W, __mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pord128_mask ((__v4si) __A,
						(__v4si) __B,
						(__v4si) __W,
						(__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_or_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pord128_mask ((__v4si) __A,
						(__v4si) __B,
						(__v4si)
						_mm_setzero_si128 (),
						(__mmask8) __U);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_or_epi32 (__m128i __A, __m128i __B)
{
  return (__m128i) ((__v4su)__A | (__v4su)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_xor_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pxord256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_xor_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pxord256_mask ((__v8si) __A,
						 (__v8si) __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_xor_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v8su)__A ^ (__v8su)__B);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_xor_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pxord128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_xor_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pxord128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_xor_epi32 (__m128i __A, __m128i __B)
{
  return (__m128i) ((__v4su)__A ^ (__v4su)__B);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtpd_ps (__m128 __W, __mmask8 __U, __m128d __A)
{
  return (__m128) __builtin_ia32_cvtpd2ps_mask ((__v2df) __A,
						(__v4sf) __W,
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtpd_ps (__mmask8 __U, __m128d __A)
{
  return (__m128) __builtin_ia32_cvtpd2ps_mask ((__v2df) __A,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtpd_ps (__m128 __W, __mmask8 __U, __m256d __A)
{
  return (__m128) __builtin_ia32_cvtpd2ps256_mask ((__v4df) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtpd_ps (__mmask8 __U, __m256d __A)
{
  return (__m128) __builtin_ia32_cvtpd2ps256_mask ((__v4df) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtps_epi32 (__m256i __W, __mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvtps2dq256_mask ((__v8sf) __A,
						    (__v8si) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtps_epi32 (__mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvtps2dq256_mask ((__v8sf) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtps_epi32 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2dq128_mask ((__v4sf) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtps_epi32 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2dq128_mask ((__v4sf) __A,
						    (__v4si)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtps_epu32 (__m256 __A)
{
  return (__m256i) __builtin_ia32_cvtps2udq256_mask ((__v8sf) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtps_epu32 (__m256i __W, __mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvtps2udq256_mask ((__v8sf) __A,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtps_epu32 (__mmask8 __U, __m256 __A)
{
  return (__m256i) __builtin_ia32_cvtps2udq256_mask ((__v8sf) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtps_epu32 (__m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2udq128_mask ((__v4sf) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtps_epu32 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2udq128_mask ((__v4sf) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtps_epu32 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2udq128_mask ((__v4sf) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_movedup_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_movddup256_mask ((__v4df) __A,
						   (__v4df) __W,
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_movedup_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_movddup256_mask ((__v4df) __A,
						   (__v4df)
						   _mm256_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_movedup_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_movddup128_mask ((__v2df) __A,
						   (__v2df) __W,
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_movedup_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_movddup128_mask ((__v2df) __A,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_movehdup_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_movshdup256_mask ((__v8sf) __A,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_movehdup_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_movshdup256_mask ((__v8sf) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_movehdup_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_movshdup128_mask ((__v4sf) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_movehdup_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_movshdup128_mask ((__v4sf) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_moveldup_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_movsldup256_mask ((__v8sf) __A,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_moveldup_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_movsldup256_mask ((__v8sf) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_moveldup_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_movsldup128_mask ((__v4sf) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_moveldup_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_movsldup128_mask ((__v4sf) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpackhi_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
			 __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhdq128_mask ((__v4si) __A,
						     (__v4si) __B,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpackhi_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhdq128_mask ((__v4si) __A,
						     (__v4si) __B,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpackhi_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			    __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhdq256_mask ((__v8si) __A,
						     (__v8si) __B,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpackhi_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhdq256_mask ((__v8si) __A,
						     (__v8si) __B,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpackhi_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
			 __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhqdq128_mask ((__v2di) __A,
						      (__v2di) __B,
						      (__v2di) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpackhi_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckhqdq128_mask ((__v2di) __A,
						      (__v2di) __B,
						      (__v2di)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpackhi_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			    __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhqdq256_mask ((__v4di) __A,
						      (__v4di) __B,
						      (__v4di) __W,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpackhi_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckhqdq256_mask ((__v4di) __A,
						      (__v4di) __B,
						      (__v4di)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpacklo_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
			 __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckldq128_mask ((__v4si) __A,
						     (__v4si) __B,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpacklo_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpckldq128_mask ((__v4si) __A,
						     (__v4si) __B,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpacklo_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			    __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckldq256_mask ((__v8si) __A,
						     (__v8si) __B,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpacklo_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpckldq256_mask ((__v8si) __A,
						     (__v8si) __B,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpacklo_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
			 __m128i __B)
{
  return (__m128i) __builtin_ia32_punpcklqdq128_mask ((__v2di) __A,
						      (__v2di) __B,
						      (__v2di) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpacklo_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_punpcklqdq128_mask ((__v2di) __A,
						      (__v2di) __B,
						      (__v2di)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpacklo_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			    __m256i __B)
{
  return (__m256i) __builtin_ia32_punpcklqdq256_mask ((__v4di) __A,
						      (__v4di) __B,
						      (__v4di) __W,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpacklo_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_punpcklqdq256_mask ((__v4di) __A,
						      (__v4di) __B,
						      (__v4di)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epu32_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __A,
						   (__v4si) __B, 0,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epi32_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqd128_mask ((__v4si) __A,
						    (__v4si) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epu32_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __A,
						   (__v4si) __B, 0, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epi32_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqd128_mask ((__v4si) __A,
						    (__v4si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epu32_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __A,
						   (__v8si) __B, 0,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi32_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqd256_mask ((__v8si) __A,
						    (__v8si) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epu32_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __A,
						   (__v8si) __B, 0, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epi32_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqd256_mask ((__v8si) __A,
						    (__v8si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epu64_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __A,
						   (__v2di) __B, 0,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpeq_epi64_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqq128_mask ((__v2di) __A,
						    (__v2di) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epu64_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __A,
						   (__v2di) __B, 0, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpeq_epi64_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqq128_mask ((__v2di) __A,
						    (__v2di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epu64_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __A,
						   (__v4di) __B, 0,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi64_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqq256_mask ((__v4di) __A,
						    (__v4di) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epu64_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __A,
						   (__v4di) __B, 0, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpeq_epi64_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqq256_mask ((__v4di) __A,
						    (__v4di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epu32_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __A,
						   (__v4si) __B, 6,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epi32_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtd128_mask ((__v4si) __A,
						    (__v4si) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epu32_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __A,
						   (__v4si) __B, 6, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epi32_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtd128_mask ((__v4si) __A,
						    (__v4si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epu32_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __A,
						   (__v8si) __B, 6,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi32_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtd256_mask ((__v8si) __A,
						    (__v8si) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epu32_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __A,
						   (__v8si) __B, 6, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epi32_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtd256_mask ((__v8si) __A,
						    (__v8si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epu64_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __A,
						   (__v2di) __B, 6,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpgt_epi64_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtq128_mask ((__v2di) __A,
						    (__v2di) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epu64_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __A,
						   (__v2di) __B, 6, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpgt_epi64_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtq128_mask ((__v2di) __A,
						    (__v2di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epu64_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __A,
						   (__v4di) __B, 6,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi64_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtq256_mask ((__v4di) __A,
						    (__v4di) __B,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epu64_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __A,
						   (__v4di) __B, 6, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpgt_epi64_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtq256_mask ((__v4di) __A,
						    (__v4di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_test_epi32_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestmd128 ((__v4si) __A,
					       (__v4si) __B,
					       (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_test_epi32_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestmd128 ((__v4si) __A,
					       (__v4si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_test_epi32_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestmd256 ((__v8si) __A,
					       (__v8si) __B,
					       (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_test_epi32_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestmd256 ((__v8si) __A,
					       (__v8si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_test_epi64_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestmq128 ((__v2di) __A,
					       (__v2di) __B,
					       (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_test_epi64_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestmq128 ((__v2di) __A,
					       (__v2di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_test_epi64_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestmq256 ((__v4di) __A,
					       (__v4di) __B,
					       (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_test_epi64_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestmq256 ((__v4di) __A,
					       (__v4di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_testn_epi32_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmd128 ((__v4si) __A,
						(__v4si) __B,
						(__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_testn_epi32_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmd128 ((__v4si) __A,
						(__v4si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testn_epi32_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmd256 ((__v8si) __A,
						(__v8si) __B,
						(__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_testn_epi32_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmd256 ((__v8si) __A,
						(__v8si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_testn_epi64_mask (__m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmq128 ((__v2di) __A,
						(__v2di) __B,
						(__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_testn_epi64_mask (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmq128 ((__v2di) __A,
						(__v2di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testn_epi64_mask (__m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmq256 ((__v4di) __A,
						(__v4di) __B,
						(__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_testn_epi64_mask (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmq256 ((__v4di) __A,
						(__v4di) __B, __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compress_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_compressdf256_mask ((__v4df) __A,
						      (__v4df) __W,
						      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_compress_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_compressdf256_mask ((__v4df) __A,
						      (__v4df)
						      _mm256_setzero_pd (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compressstoreu_pd (void *__P, __mmask8 __U, __m256d __A)
{
  __builtin_ia32_compressstoredf256_mask ((__v4df *) __P,
					  (__v4df) __A,
					  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compress_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_compressdf128_mask ((__v2df) __A,
						      (__v2df) __W,
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_compress_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_compressdf128_mask ((__v2df) __A,
						      (__v2df)
						      _mm_setzero_pd (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compressstoreu_pd (void *__P, __mmask8 __U, __m128d __A)
{
  __builtin_ia32_compressstoredf128_mask ((__v2df *) __P,
					  (__v2df) __A,
					  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compress_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_compresssf256_mask ((__v8sf) __A,
						     (__v8sf) __W,
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_compress_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_compresssf256_mask ((__v8sf) __A,
						     (__v8sf)
						     _mm256_setzero_ps (),
						     (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compressstoreu_ps (void *__P, __mmask8 __U, __m256 __A)
{
  __builtin_ia32_compressstoresf256_mask ((__v8sf *) __P,
					  (__v8sf) __A,
					  (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compress_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_compresssf128_mask ((__v4sf) __A,
						     (__v4sf) __W,
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_compress_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_compresssf128_mask ((__v4sf) __A,
						     (__v4sf)
						     _mm_setzero_ps (),
						     (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compressstoreu_ps (void *__P, __mmask8 __U, __m128 __A)
{
  __builtin_ia32_compressstoresf128_mask ((__v4sf *) __P,
					  (__v4sf) __A,
					  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compress_epi64 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_compressdi256_mask ((__v4di) __A,
						      (__v4di) __W,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_compress_epi64 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_compressdi256_mask ((__v4di) __A,
						      (__v4di)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compressstoreu_epi64 (void *__P, __mmask8 __U, __m256i __A)
{
  __builtin_ia32_compressstoredi256_mask ((__v4di *) __P,
					  (__v4di) __A,
					  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compress_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_compressdi128_mask ((__v2di) __A,
						      (__v2di) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_compress_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_compressdi128_mask ((__v2di) __A,
						      (__v2di)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compressstoreu_epi64 (void *__P, __mmask8 __U, __m128i __A)
{
  __builtin_ia32_compressstoredi128_mask ((__v2di *) __P,
					  (__v2di) __A,
					  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compress_epi32 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_compresssi256_mask ((__v8si) __A,
						      (__v8si) __W,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_compress_epi32 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_compresssi256_mask ((__v8si) __A,
						      (__v8si)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_compressstoreu_epi32 (void *__P, __mmask8 __U, __m256i __A)
{
  __builtin_ia32_compressstoresi256_mask ((__v8si *) __P,
					  (__v8si) __A,
					  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compress_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_compresssi128_mask ((__v4si) __A,
						      (__v4si) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_compress_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_compresssi128_mask ((__v4si) __A,
						      (__v4si)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_compressstoreu_epi32 (void *__P, __mmask8 __U, __m128i __A)
{
  __builtin_ia32_compressstoresi128_mask ((__v4si *) __P,
					  (__v4si) __A,
					  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expand_pd (__m256d __W, __mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_expanddf256_mask ((__v4df) __A,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expand_pd (__mmask8 __U, __m256d __A)
{
  return (__m256d) __builtin_ia32_expanddf256_maskz ((__v4df) __A,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expandloadu_pd (__m256d __W, __mmask8 __U, void const *__P)
{
  return (__m256d) __builtin_ia32_expandloaddf256_mask ((__v4df *) __P,
							(__v4df) __W,
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expandloadu_pd (__mmask8 __U, void const *__P)
{
  return (__m256d) __builtin_ia32_expandloaddf256_maskz ((__v4df *) __P,
							 (__v4df)
							 _mm256_setzero_pd (),
							 (__mmask8)
							 __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expand_pd (__m128d __W, __mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_expanddf128_mask ((__v2df) __A,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expand_pd (__mmask8 __U, __m128d __A)
{
  return (__m128d) __builtin_ia32_expanddf128_maskz ((__v2df) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expandloadu_pd (__m128d __W, __mmask8 __U, void const *__P)
{
  return (__m128d) __builtin_ia32_expandloaddf128_mask ((__v2df *) __P,
							(__v2df) __W,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expandloadu_pd (__mmask8 __U, void const *__P)
{
  return (__m128d) __builtin_ia32_expandloaddf128_maskz ((__v2df *) __P,
							 (__v2df)
							 _mm_setzero_pd (),
							 (__mmask8)
							 __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expand_ps (__m256 __W, __mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_expandsf256_mask ((__v8sf) __A,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expand_ps (__mmask8 __U, __m256 __A)
{
  return (__m256) __builtin_ia32_expandsf256_maskz ((__v8sf) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expandloadu_ps (__m256 __W, __mmask8 __U, void const *__P)
{
  return (__m256) __builtin_ia32_expandloadsf256_mask ((__v8sf *) __P,
						       (__v8sf) __W,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expandloadu_ps (__mmask8 __U, void const *__P)
{
  return (__m256) __builtin_ia32_expandloadsf256_maskz ((__v8sf *) __P,
							(__v8sf)
							_mm256_setzero_ps (),
							(__mmask8)
							__U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expand_ps (__m128 __W, __mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_expandsf128_mask ((__v4sf) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expand_ps (__mmask8 __U, __m128 __A)
{
  return (__m128) __builtin_ia32_expandsf128_maskz ((__v4sf) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expandloadu_ps (__m128 __W, __mmask8 __U, void const *__P)
{
  return (__m128) __builtin_ia32_expandloadsf128_mask ((__v4sf *) __P,
						       (__v4sf) __W,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expandloadu_ps (__mmask8 __U, void const *__P)
{
  return (__m128) __builtin_ia32_expandloadsf128_maskz ((__v4sf *) __P,
							(__v4sf)
							_mm_setzero_ps (),
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expand_epi64 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_expanddi256_mask ((__v4di) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expand_epi64 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_expanddi256_maskz ((__v4di) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expandloadu_epi64 (__m256i __W, __mmask8 __U,
			       void const *__P)
{
  return (__m256i) __builtin_ia32_expandloaddi256_mask ((__v4di *) __P,
							(__v4di) __W,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expandloadu_epi64 (__mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_expandloaddi256_maskz ((__v4di *) __P,
							 (__v4di)
							 _mm256_setzero_si256 (),
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expand_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_expanddi128_mask ((__v2di) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expand_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_expanddi128_maskz ((__v2di) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expandloadu_epi64 (__m128i __W, __mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_expandloaddi128_mask ((__v2di *) __P,
							(__v2di) __W,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expandloadu_epi64 (__mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_expandloaddi128_maskz ((__v2di *) __P,
							 (__v2di)
							 _mm_setzero_si128 (),
							 (__mmask8)
							 __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expand_epi32 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_expandsi256_mask ((__v8si) __A,
						    (__v8si) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expand_epi32 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_expandsi256_maskz ((__v8si) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_expandloadu_epi32 (__m256i __W, __mmask8 __U,
			       void const *__P)
{
  return (__m256i) __builtin_ia32_expandloadsi256_mask ((__v8si *) __P,
							(__v8si) __W,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_expandloadu_epi32 (__mmask8 __U, void const *__P)
{
  return (__m256i) __builtin_ia32_expandloadsi256_maskz ((__v8si *) __P,
							 (__v8si)
							 _mm256_setzero_si256 (),
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expand_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_expandsi128_mask ((__v4si) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expand_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_expandsi128_maskz ((__v4si) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_expandloadu_epi32 (__m128i __W, __mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_expandloadsi128_mask ((__v4si *) __P,
							(__v4si) __W,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_expandloadu_epi32 (__mmask8 __U, void const *__P)
{
  return (__m128i) __builtin_ia32_expandloadsi128_maskz ((__v4si *) __P,
							 (__v4si)
							 _mm_setzero_si128 (),
							 (__mmask8)
							 __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutex2var_pd (__m256d __A, __m256i __I, __m256d __B)
{
  return (__m256d) __builtin_ia32_vpermt2varpd256_mask ((__v4di) __I
							/* idx */ ,
							(__v4df) __A,
							(__v4df) __B,
							(__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutex2var_pd (__m256d __A, __mmask8 __U, __m256i __I,
			     __m256d __B)
{
  return (__m256d) __builtin_ia32_vpermt2varpd256_mask ((__v4di) __I
							/* idx */ ,
							(__v4df) __A,
							(__v4df) __B,
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask2_permutex2var_pd (__m256d __A, __m256i __I, __mmask8 __U,
			      __m256d __B)
{
  return (__m256d) __builtin_ia32_vpermi2varpd256_mask ((__v4df) __A,
							(__v4di) __I
							/* idx */ ,
							(__v4df) __B,
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutex2var_pd (__mmask8 __U, __m256d __A, __m256i __I,
			      __m256d __B)
{
  return (__m256d) __builtin_ia32_vpermt2varpd256_maskz ((__v4di) __I
							 /* idx */ ,
							 (__v4df) __A,
							 (__v4df) __B,
							 (__mmask8)
							 __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutex2var_ps (__m256 __A, __m256i __I, __m256 __B)
{
  return (__m256) __builtin_ia32_vpermt2varps256_mask ((__v8si) __I
						       /* idx */ ,
						       (__v8sf) __A,
						       (__v8sf) __B,
						       (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutex2var_ps (__m256 __A, __mmask8 __U, __m256i __I,
			     __m256 __B)
{
  return (__m256) __builtin_ia32_vpermt2varps256_mask ((__v8si) __I
						       /* idx */ ,
						       (__v8sf) __A,
						       (__v8sf) __B,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask2_permutex2var_ps (__m256 __A, __m256i __I, __mmask8 __U,
			      __m256 __B)
{
  return (__m256) __builtin_ia32_vpermi2varps256_mask ((__v8sf) __A,
						       (__v8si) __I
						       /* idx */ ,
						       (__v8sf) __B,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutex2var_ps (__mmask8 __U, __m256 __A, __m256i __I,
			      __m256 __B)
{
  return (__m256) __builtin_ia32_vpermt2varps256_maskz ((__v8si) __I
							/* idx */ ,
							(__v8sf) __A,
							(__v8sf) __B,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutex2var_epi64 (__m128i __A, __m128i __I, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2varq128_mask ((__v2di) __I
						       /* idx */ ,
						       (__v2di) __A,
						       (__v2di) __B,
						       (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutex2var_epi64 (__m128i __A, __mmask8 __U, __m128i __I,
			     __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2varq128_mask ((__v2di) __I
						       /* idx */ ,
						       (__v2di) __A,
						       (__v2di) __B,
						       (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask2_permutex2var_epi64 (__m128i __A, __m128i __I, __mmask8 __U,
			      __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermi2varq128_mask ((__v2di) __A,
						       (__v2di) __I
						       /* idx */ ,
						       (__v2di) __B,
						       (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutex2var_epi64 (__mmask8 __U, __m128i __A, __m128i __I,
			      __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2varq128_maskz ((__v2di) __I
							/* idx */ ,
							(__v2di) __A,
							(__v2di) __B,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutex2var_epi32 (__m128i __A, __m128i __I, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2vard128_mask ((__v4si) __I
						       /* idx */ ,
						       (__v4si) __A,
						       (__v4si) __B,
						       (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutex2var_epi32 (__m128i __A, __mmask8 __U, __m128i __I,
			     __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2vard128_mask ((__v4si) __I
						       /* idx */ ,
						       (__v4si) __A,
						       (__v4si) __B,
						       (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask2_permutex2var_epi32 (__m128i __A, __m128i __I, __mmask8 __U,
			      __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermi2vard128_mask ((__v4si) __A,
						       (__v4si) __I
						       /* idx */ ,
						       (__v4si) __B,
						       (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutex2var_epi32 (__mmask8 __U, __m128i __A, __m128i __I,
			      __m128i __B)
{
  return (__m128i) __builtin_ia32_vpermt2vard128_maskz ((__v4si) __I
							/* idx */ ,
							(__v4si) __A,
							(__v4si) __B,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutex2var_epi64 (__m256i __A, __m256i __I, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2varq256_mask ((__v4di) __I
						       /* idx */ ,
						       (__v4di) __A,
						       (__v4di) __B,
						       (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutex2var_epi64 (__m256i __A, __mmask8 __U, __m256i __I,
				__m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2varq256_mask ((__v4di) __I
						       /* idx */ ,
						       (__v4di) __A,
						       (__v4di) __B,
						       (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask2_permutex2var_epi64 (__m256i __A, __m256i __I,
				 __mmask8 __U, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermi2varq256_mask ((__v4di) __A,
						       (__v4di) __I
						       /* idx */ ,
						       (__v4di) __B,
						       (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutex2var_epi64 (__mmask8 __U, __m256i __A,
				 __m256i __I, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2varq256_maskz ((__v4di) __I
							/* idx */ ,
							(__v4di) __A,
							(__v4di) __B,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutex2var_epi32 (__m256i __A, __m256i __I, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2vard256_mask ((__v8si) __I
						       /* idx */ ,
						       (__v8si) __A,
						       (__v8si) __B,
						       (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutex2var_epi32 (__m256i __A, __mmask8 __U, __m256i __I,
				__m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2vard256_mask ((__v8si) __I
						       /* idx */ ,
						       (__v8si) __A,
						       (__v8si) __B,
						       (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask2_permutex2var_epi32 (__m256i __A, __m256i __I,
				 __mmask8 __U, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermi2vard256_mask ((__v8si) __A,
						       (__v8si) __I
						       /* idx */ ,
						       (__v8si) __B,
						       (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutex2var_epi32 (__mmask8 __U, __m256i __A,
				 __m256i __I, __m256i __B)
{
  return (__m256i) __builtin_ia32_vpermt2vard256_maskz ((__v8si) __I
							/* idx */ ,
							(__v8si) __A,
							(__v8si) __B,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutex2var_pd (__m128d __A, __m128i __I, __m128d __B)
{
  return (__m128d) __builtin_ia32_vpermt2varpd128_mask ((__v2di) __I
							/* idx */ ,
							(__v2df) __A,
							(__v2df) __B,
							(__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutex2var_pd (__m128d __A, __mmask8 __U, __m128i __I,
			  __m128d __B)
{
  return (__m128d) __builtin_ia32_vpermt2varpd128_mask ((__v2di) __I
							/* idx */ ,
							(__v2df) __A,
							(__v2df) __B,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask2_permutex2var_pd (__m128d __A, __m128i __I, __mmask8 __U,
			   __m128d __B)
{
  return (__m128d) __builtin_ia32_vpermi2varpd128_mask ((__v2df) __A,
							(__v2di) __I
							/* idx */ ,
							(__v2df) __B,
							(__mmask8)
							__U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutex2var_pd (__mmask8 __U, __m128d __A, __m128i __I,
			   __m128d __B)
{
  return (__m128d) __builtin_ia32_vpermt2varpd128_maskz ((__v2di) __I
							 /* idx */ ,
							 (__v2df) __A,
							 (__v2df) __B,
							 (__mmask8)
							 __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutex2var_ps (__m128 __A, __m128i __I, __m128 __B)
{
  return (__m128) __builtin_ia32_vpermt2varps128_mask ((__v4si) __I
						       /* idx */ ,
						       (__v4sf) __A,
						       (__v4sf) __B,
						       (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutex2var_ps (__m128 __A, __mmask8 __U, __m128i __I,
			  __m128 __B)
{
  return (__m128) __builtin_ia32_vpermt2varps128_mask ((__v4si) __I
						       /* idx */ ,
						       (__v4sf) __A,
						       (__v4sf) __B,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask2_permutex2var_ps (__m128 __A, __m128i __I, __mmask8 __U,
			   __m128 __B)
{
  return (__m128) __builtin_ia32_vpermi2varps128_mask ((__v4sf) __A,
						       (__v4si) __I
						       /* idx */ ,
						       (__v4sf) __B,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutex2var_ps (__mmask8 __U, __m128 __A, __m128i __I,
			   __m128 __B)
{
  return (__m128) __builtin_ia32_vpermt2varps128_maskz ((__v4si) __I
							/* idx */ ,
							(__v4sf) __A,
							(__v4sf) __B,
							(__mmask8)
							__U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_srav_epi64 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psravq128_mask ((__v2di) __X,
						  (__v2di) __Y,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srav_epi64 (__m128i __W, __mmask8 __U, __m128i __X,
		     __m128i __Y)
{
  return (__m128i) __builtin_ia32_psravq128_mask ((__v2di) __X,
						  (__v2di) __Y,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srav_epi64 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psravq128_mask ((__v2di) __X,
						  (__v2di) __Y,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sllv_epi32 (__m256i __W, __mmask8 __U, __m256i __X,
			__m256i __Y)
{
  return (__m256i) __builtin_ia32_psllv8si_mask ((__v8si) __X,
						 (__v8si) __Y,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sllv_epi32 (__mmask8 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psllv8si_mask ((__v8si) __X,
						 (__v8si) __Y,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sllv_epi32 (__m128i __W, __mmask8 __U, __m128i __X,
		     __m128i __Y)
{
  return (__m128i) __builtin_ia32_psllv4si_mask ((__v4si) __X,
						 (__v4si) __Y,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sllv_epi32 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psllv4si_mask ((__v4si) __X,
						 (__v4si) __Y,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sllv_epi64 (__m256i __W, __mmask8 __U, __m256i __X,
			__m256i __Y)
{
  return (__m256i) __builtin_ia32_psllv4di_mask ((__v4di) __X,
						 (__v4di) __Y,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sllv_epi64 (__mmask8 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psllv4di_mask ((__v4di) __X,
						 (__v4di) __Y,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sllv_epi64 (__m128i __W, __mmask8 __U, __m128i __X,
		     __m128i __Y)
{
  return (__m128i) __builtin_ia32_psllv2di_mask ((__v2di) __X,
						 (__v2di) __Y,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sllv_epi64 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psllv2di_mask ((__v2di) __X,
						 (__v2di) __Y,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srav_epi32 (__m256i __W, __mmask8 __U, __m256i __X,
			__m256i __Y)
{
  return (__m256i) __builtin_ia32_psrav8si_mask ((__v8si) __X,
						 (__v8si) __Y,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srav_epi32 (__mmask8 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psrav8si_mask ((__v8si) __X,
						 (__v8si) __Y,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srav_epi32 (__m128i __W, __mmask8 __U, __m128i __X,
		     __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrav4si_mask ((__v4si) __X,
						 (__v4si) __Y,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srav_epi32 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrav4si_mask ((__v4si) __X,
						 (__v4si) __Y,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srlv_epi32 (__m256i __W, __mmask8 __U, __m256i __X,
			__m256i __Y)
{
  return (__m256i) __builtin_ia32_psrlv8si_mask ((__v8si) __X,
						 (__v8si) __Y,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srlv_epi32 (__mmask8 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psrlv8si_mask ((__v8si) __X,
						 (__v8si) __Y,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srlv_epi32 (__m128i __W, __mmask8 __U, __m128i __X,
		     __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrlv4si_mask ((__v4si) __X,
						 (__v4si) __Y,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srlv_epi32 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrlv4si_mask ((__v4si) __X,
						 (__v4si) __Y,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srlv_epi64 (__m256i __W, __mmask8 __U, __m256i __X,
			__m256i __Y)
{
  return (__m256i) __builtin_ia32_psrlv4di_mask ((__v4di) __X,
						 (__v4di) __Y,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srlv_epi64 (__mmask8 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psrlv4di_mask ((__v4di) __X,
						 (__v4di) __Y,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srlv_epi64 (__m128i __W, __mmask8 __U, __m128i __X,
		     __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrlv2di_mask ((__v2di) __X,
						 (__v2di) __Y,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srlv_epi64 (__mmask8 __U, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrlv2di_mask ((__v2di) __X,
						 (__v2di) __Y,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rolv_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prolvd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rolv_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_prolvd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rolv_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prolvd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rolv_epi32 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prolvd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rolv_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_prolvd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rolv_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prolvd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rorv_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prorvd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rorv_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_prorvd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rorv_epi32 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prorvd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rorv_epi32 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prorvd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rorv_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_prorvd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rorv_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prorvd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rolv_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prolvq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rolv_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_prolvq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rolv_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prolvq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rolv_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prolvq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rolv_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_prolvq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rolv_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prolvq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rorv_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prorvq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rorv_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			__m256i __B)
{
  return (__m256i) __builtin_ia32_prorvq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rorv_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_prorvq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rorv_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prorvq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rorv_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		     __m128i __B)
{
  return (__m128i) __builtin_ia32_prorvq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rorv_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_prorvq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srav_epi64 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psravq256_mask ((__v4di) __X,
						  (__v4di) __Y,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srav_epi64 (__m256i __W, __mmask8 __U, __m256i __X,
			__m256i __Y)
{
  return (__m256i) __builtin_ia32_psravq256_mask ((__v4di) __X,
						  (__v4di) __Y,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srav_epi64 (__mmask8 __U, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psravq256_mask ((__v4di) __X,
						  (__v4di) __Y,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_and_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pandq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di) __W, __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_and_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pandq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di)
						 _mm256_setzero_pd (),
						 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_and_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pandq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di) __W, __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_and_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pandq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_pd (),
						 __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_andnot_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			  __m256i __B)
{
  return (__m256i) __builtin_ia32_pandnq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W, __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_andnot_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pandnq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_pd (),
						  __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_andnot_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		       __m128i __B)
{
  return (__m128i) __builtin_ia32_pandnq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W, __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_andnot_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pandnq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_pd (),
						  __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_or_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		      __m256i __B)
{
  return (__m256i) __builtin_ia32_porq256_mask ((__v4di) __A,
						(__v4di) __B,
						(__v4di) __W,
						(__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_or_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_porq256_mask ((__v4di) __A,
						(__v4di) __B,
						(__v4di)
						_mm256_setzero_si256 (),
						(__mmask8) __U);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_or_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du)__A | (__v4du)__B);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_or_epi64 (__m128i __W, __mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_porq128_mask ((__v2di) __A,
						(__v2di) __B,
						(__v2di) __W,
						(__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_or_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_porq128_mask ((__v2di) __A,
						(__v2di) __B,
						(__v2di)
						_mm_setzero_si128 (),
						(__mmask8) __U);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_or_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) ((__v2du)__A | (__v2du)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_xor_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pxorq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_xor_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pxorq256_mask ((__v4di) __A,
						 (__v4di) __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_xor_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du)__A ^ (__v4du)__B);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_xor_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pxorq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_xor_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pxorq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_xor_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) ((__v2du)__A ^ (__v2du)__B);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_maxpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_maxpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_maxps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_maxps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_div_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_divps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf) __W,
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_div_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_divps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf)
					     _mm_setzero_ps (),
					     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_div_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_divpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df) __W,
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_div_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_divpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df)
					      _mm_setzero_pd (),
					      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_minpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_div_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_divpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_minpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_minps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_div_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_divpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_div_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_divps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_minps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_div_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_divps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_minps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf) __W,
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_mulps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf) __W,
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_minps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf)
					     _mm_setzero_ps (),
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_mulps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf)
					     _mm_setzero_ps (),
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_maxps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf) __W,
					     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_maxps_mask ((__v4sf) __A,
					     (__v4sf) __B,
					     (__v4sf)
					     _mm_setzero_ps (),
					     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_minpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df) __W,
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_minpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df)
					      _mm_setzero_pd (),
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_maxpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df) __W,
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_maxpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df)
					      _mm_setzero_pd (),
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_mulpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df) __W,
					      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_mulpd_mask ((__v2df) __A,
					      (__v2df) __B,
					      (__v2df)
					      _mm_setzero_pd (),
					      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mul_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_mulps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mul_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_mulps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mul_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_mulpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mul_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_mulpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epi64 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epi64 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epi64 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epi64 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epu64 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxuq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epu64 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxuq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epu64 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxuq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epu64 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminuq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epu64 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pminuq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epu64 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminuq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epi32 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epi32 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxsd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epi32 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epi32 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pminsd256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_max_epu32 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxud256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_max_epu32 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pmaxud256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_min_epu32 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pminud256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_min_epu32 (__m256i __W, __mmask8 __M, __m256i __A,
		       __m256i __B)
{
  return (__m256i) __builtin_ia32_pminud256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epi64 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epi64 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epi64 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epi64 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epu64 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxuq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_epu64 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxuq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epu64 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxuq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_epu64 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminuq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epu64 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pminuq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epu64 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminuq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epi32 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epi32 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxsd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epi32 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epi32 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pminsd128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_epu32 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxud128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_epu32 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pmaxud128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_epu32 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pminud128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_epu32 (__m128i __W, __mmask8 __M, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pminud128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W, __M);
}

#ifndef __AVX512CD__
#pragma GCC push_options
#pragma GCC target("avx512vl,avx512cd")
#define __DISABLE_AVX512VLCD__
#endif

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcastmb_epi64 (__mmask8 __A)
{
  return (__m128i) __builtin_ia32_broadcastmb128 (__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastmb_epi64 (__mmask8 __A)
{
  return (__m256i) __builtin_ia32_broadcastmb256 (__A);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcastmw_epi32 (__mmask16 __A)
{
  return (__m128i) __builtin_ia32_broadcastmw128 (__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastmw_epi32 (__mmask16 __A)
{
  return (__m256i) __builtin_ia32_broadcastmw256 (__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_lzcnt_epi32 (__m256i __A)
{
  return (__m256i) __builtin_ia32_vplzcntd_256_mask ((__v8si) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_lzcnt_epi32 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vplzcntd_256_mask ((__v8si) __A,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_lzcnt_epi32 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vplzcntd_256_mask ((__v8si) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_lzcnt_epi64 (__m256i __A)
{
  return (__m256i) __builtin_ia32_vplzcntq_256_mask ((__v4di) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_lzcnt_epi64 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vplzcntq_256_mask ((__v4di) __A,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_lzcnt_epi64 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vplzcntq_256_mask ((__v4di) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_conflict_epi64 (__m256i __A)
{
  return (__m256i) __builtin_ia32_vpconflictdi_256_mask ((__v4di) __A,
							 (__v4di)
							 _mm256_setzero_si256 (),
							 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_conflict_epi64 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpconflictdi_256_mask ((__v4di) __A,
							 (__v4di) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_conflict_epi64 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpconflictdi_256_mask ((__v4di) __A,
							 (__v4di)
							 _mm256_setzero_si256 (),
							 (__mmask8)
							 __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_conflict_epi32 (__m256i __A)
{
  return (__m256i) __builtin_ia32_vpconflictsi_256_mask ((__v8si) __A,
							 (__v8si)
							 _mm256_setzero_si256 (),
							 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_conflict_epi32 (__m256i __W, __mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpconflictsi_256_mask ((__v8si) __A,
							 (__v8si) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_conflict_epi32 (__mmask8 __U, __m256i __A)
{
  return (__m256i) __builtin_ia32_vpconflictsi_256_mask ((__v8si) __A,
							 (__v8si)
							 _mm256_setzero_si256 (),
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_lzcnt_epi32 (__m128i __A)
{
  return (__m128i) __builtin_ia32_vplzcntd_128_mask ((__v4si) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_lzcnt_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vplzcntd_128_mask ((__v4si) __A,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_lzcnt_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vplzcntd_128_mask ((__v4si) __A,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_lzcnt_epi64 (__m128i __A)
{
  return (__m128i) __builtin_ia32_vplzcntq_128_mask ((__v2di) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_lzcnt_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vplzcntq_128_mask ((__v2di) __A,
						     (__v2di) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_lzcnt_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vplzcntq_128_mask ((__v2di) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_conflict_epi64 (__m128i __A)
{
  return (__m128i) __builtin_ia32_vpconflictdi_128_mask ((__v2di) __A,
							 (__v2di)
							 _mm_setzero_si128 (),
							 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_conflict_epi64 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpconflictdi_128_mask ((__v2di) __A,
							 (__v2di) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_conflict_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpconflictdi_128_mask ((__v2di) __A,
							 (__v2di)
							 _mm_setzero_si128 (),
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_conflict_epi32 (__m128i __A)
{
  return (__m128i) __builtin_ia32_vpconflictsi_128_mask ((__v4si) __A,
							 (__v4si)
							 _mm_setzero_si128 (),
							 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_conflict_epi32 (__m128i __W, __mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpconflictsi_128_mask ((__v4si) __A,
							 (__v4si) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_conflict_epi32 (__mmask8 __U, __m128i __A)
{
  return (__m128i) __builtin_ia32_vpconflictsi_128_mask ((__v4si) __A,
							 (__v4si)
							 _mm_setzero_si128 (),
							 (__mmask8)
							 __U);
}

#ifdef __DISABLE_AVX512VLCD__
#pragma GCC pop_options
#endif

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpacklo_pd (__m256d __W, __mmask8 __U, __m256d __A,
			 __m256d __B)
{
  return (__m256d) __builtin_ia32_unpcklpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpacklo_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_unpcklpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpacklo_pd (__m128d __W, __mmask8 __U, __m128d __A,
		      __m128d __B)
{
  return (__m128d) __builtin_ia32_unpcklpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpacklo_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_unpcklpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpacklo_ps (__m256 __W, __mmask8 __U, __m256 __A,
			 __m256 __B)
{
  return (__m256) __builtin_ia32_unpcklps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpackhi_pd (__m256d __W, __mmask8 __U, __m256d __A,
			 __m256d __B)
{
  return (__m256d) __builtin_ia32_unpckhpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpackhi_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_unpckhpd256_mask ((__v4df) __A,
						    (__v4df) __B,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpackhi_pd (__m128d __W, __mmask8 __U, __m128d __A,
		      __m128d __B)
{
  return (__m128d) __builtin_ia32_unpckhpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpackhi_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_unpckhpd128_mask ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_unpackhi_ps (__m256 __W, __mmask8 __U, __m256 __A,
			 __m256 __B)
{
  return (__m256) __builtin_ia32_unpckhps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpackhi_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_unpckhps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpackhi_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_unpckhps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpackhi_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_unpckhps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtph_ps (__m128 __W, __mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_vcvtph2ps_mask ((__v8hi) __A,
						 (__v4sf) __W,
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtph_ps (__mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_vcvtph2ps_mask ((__v8hi) __A,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_unpacklo_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_unpcklps256_mask ((__v8sf) __A,
						   (__v8sf) __B,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtph_ps (__m256 __W, __mmask8 __U, __m128i __A)
{
  return (__m256) __builtin_ia32_vcvtph2ps256_mask ((__v8hi) __A,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtph_ps (__mmask8 __U, __m128i __A)
{
  return (__m256) __builtin_ia32_vcvtph2ps256_mask ((__v8hi) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_unpacklo_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_unpcklps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_unpacklo_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_unpcklps128_mask ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sra_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psrad256_mask ((__v8si) __A,
						 (__v4si) __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sra_epi32 (__mmask8 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psrad256_mask ((__v8si) __A,
						 (__v4si) __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sra_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psrad128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sra_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psrad128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sra_epi64 (__m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psraq256_mask ((__v4di) __A,
						 (__v2di) __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sra_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psraq256_mask ((__v4di) __A,
						 (__v2di) __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sra_epi64 (__mmask8 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psraq256_mask ((__v4di) __A,
						 (__v2di) __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sra_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psraq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sra_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psraq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sra_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psraq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sll_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_pslld128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sll_epi32 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pslld128_mask ((__v4si) __A,
						 (__v4si) __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sll_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    __m128i __B)
{
  return (__m128i) __builtin_ia32_psllq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sll_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_psllq128_mask ((__v2di) __A,
						 (__v2di) __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sll_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_pslld256_mask ((__v8si) __A,
						 (__v4si) __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sll_epi32 (__mmask8 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_pslld256_mask ((__v8si) __A,
						 (__v4si) __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_sll_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       __m128i __B)
{
  return (__m256i) __builtin_ia32_psllq256_mask ((__v4di) __A,
						 (__v2di) __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_sll_epi64 (__mmask8 __U, __m256i __A, __m128i __B)
{
  return (__m256i) __builtin_ia32_psllq256_mask ((__v4di) __A,
						 (__v2di) __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutexvar_ps (__m256 __W, __mmask8 __U, __m256i __X,
			    __m256 __Y)
{
  return (__m256) __builtin_ia32_permvarsf256_mask ((__v8sf) __Y,
						    (__v8si) __X,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutexvar_ps (__mmask8 __U, __m256i __X, __m256 __Y)
{
  return (__m256) __builtin_ia32_permvarsf256_mask ((__v8sf) __Y,
						    (__v8si) __X,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutexvar_pd (__m256i __X, __m256d __Y)
{
  return (__m256d) __builtin_ia32_permvardf256_mask ((__v4df) __Y,
						     (__v4di) __X,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutexvar_pd (__m256d __W, __mmask8 __U, __m256i __X,
			    __m256d __Y)
{
  return (__m256d) __builtin_ia32_permvardf256_mask ((__v4df) __Y,
						     (__v4di) __X,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutexvar_pd (__mmask8 __U, __m256i __X, __m256d __Y)
{
  return (__m256d) __builtin_ia32_permvardf256_mask ((__v4df) __Y,
						     (__v4di) __X,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutevar_pd (__m256d __W, __mmask8 __U, __m256d __A,
			   __m256i __C)
{
  return (__m256d) __builtin_ia32_vpermilvarpd256_mask ((__v4df) __A,
							(__v4di) __C,
							(__v4df) __W,
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutevar_pd (__mmask8 __U, __m256d __A, __m256i __C)
{
  return (__m256d) __builtin_ia32_vpermilvarpd256_mask ((__v4df) __A,
							(__v4di) __C,
							(__v4df)
							_mm256_setzero_pd (),
							(__mmask8)
							__U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutevar_ps (__m256 __W, __mmask8 __U, __m256 __A,
			   __m256i __C)
{
  return (__m256) __builtin_ia32_vpermilvarps256_mask ((__v8sf) __A,
						       (__v8si) __C,
						       (__v8sf) __W,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutevar_ps (__mmask8 __U, __m256 __A, __m256i __C)
{
  return (__m256) __builtin_ia32_vpermilvarps256_mask ((__v8sf) __A,
						       (__v8si) __C,
						       (__v8sf)
						       _mm256_setzero_ps (),
						       (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutevar_pd (__m128d __W, __mmask8 __U, __m128d __A,
			__m128i __C)
{
  return (__m128d) __builtin_ia32_vpermilvarpd_mask ((__v2df) __A,
						     (__v2di) __C,
						     (__v2df) __W,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutevar_pd (__mmask8 __U, __m128d __A, __m128i __C)
{
  return (__m128d) __builtin_ia32_vpermilvarpd_mask ((__v2df) __A,
						     (__v2di) __C,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permutevar_ps (__m128 __W, __mmask8 __U, __m128 __A,
			__m128i __C)
{
  return (__m128) __builtin_ia32_vpermilvarps_mask ((__v4sf) __A,
						    (__v4si) __C,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permutevar_ps (__mmask8 __U, __m128 __A, __m128i __C)
{
  return (__m128) __builtin_ia32_vpermilvarps_mask ((__v4sf) __A,
						    (__v4si) __C,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mullo_epi32 (__mmask8 __M, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmulld256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutexvar_epi64 (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_permvardi256_mask ((__v4di) __Y,
						     (__v4di) __X,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mullo_epi32 (__m256i __W, __mmask8 __M, __m256i __A,
			 __m256i __B)
{
  return (__m256i) __builtin_ia32_pmulld256_mask ((__v8si) __A,
						  (__v8si) __B,
						  (__v8si) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mullo_epi32 (__mmask8 __M, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmulld128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mullo_epi32 (__m128i __W, __mmask8 __M, __m128i __A,
		      __m128i __B)
{
  return (__m128i) __builtin_ia32_pmulld128_mask ((__v4si) __A,
						  (__v4si) __B,
						  (__v4si) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mul_epi32 (__m256i __W, __mmask8 __M, __m256i __X,
		       __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmuldq256_mask ((__v8si) __X,
						  (__v8si) __Y,
						  (__v4di) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mul_epi32 (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmuldq256_mask ((__v8si) __X,
						  (__v8si) __Y,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_epi32 (__m128i __W, __mmask8 __M, __m128i __X,
		    __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmuldq128_mask ((__v4si) __X,
						  (__v4si) __Y,
						  (__v2di) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_epi32 (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmuldq128_mask ((__v4si) __X,
						  (__v4si) __Y,
						  (__v2di)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutexvar_epi64 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_permvardi256_mask ((__v4di) __Y,
						     (__v4di) __X,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutexvar_epi64 (__m256i __W, __mmask8 __M, __m256i __X,
			       __m256i __Y)
{
  return (__m256i) __builtin_ia32_permvardi256_mask ((__v4di) __Y,
						     (__v4di) __X,
						     (__v4di) __W,
						     __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mul_epu32 (__m256i __W, __mmask8 __M, __m256i __X,
		       __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmuludq256_mask ((__v8si) __X,
						   (__v8si) __Y,
						   (__v4di) __W, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutexvar_epi32 (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_permvarsi256_mask ((__v8si) __Y,
						     (__v8si) __X,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mul_epu32 (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmuludq256_mask ((__v8si) __X,
						   (__v8si) __Y,
						   (__v4di)
						   _mm256_setzero_si256 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_epu32 (__m128i __W, __mmask8 __M, __m128i __X,
		    __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmuludq128_mask ((__v4si) __X,
						   (__v4si) __Y,
						   (__v2di) __W, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_epu32 (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmuludq128_mask ((__v4si) __X,
						   (__v4si) __Y,
						   (__v2di)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutexvar_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_permvarsi256_mask ((__v8si) __Y,
						     (__v8si) __X,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutexvar_epi32 (__m256i __W, __mmask8 __M, __m256i __X,
			       __m256i __Y)
{
  return (__m256i) __builtin_ia32_permvarsi256_mask ((__v8si) __Y,
						     (__v8si) __X,
						     (__v8si) __W,
						     __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epu32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 4,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epu32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 4,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epu32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 1,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epu32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 1,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epu32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 5,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epu32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 5,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epu32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 2,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epu32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, 2,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epu64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 4,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epu64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 4,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epu64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 1,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epu64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 1,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epu64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 5,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epu64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 5,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epu64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 2,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epu64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, 2,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epi32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 4,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epi32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 4,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epi32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 1,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epi32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 1,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epi32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 5,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epi32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 5,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epi32_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 2,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epi32_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, 2,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpneq_epi64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 4,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpneq_epi64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 4,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmplt_epi64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 1,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmplt_epi64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 1,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmpge_epi64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 5,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpge_epi64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 5,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmple_epi64_mask (__mmask8 __M, __m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 2,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmple_epi64_mask (__m256i __X, __m256i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, 2,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epu32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 4,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epu32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 4,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epu32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 1,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epu32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 1,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epu32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 5,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epu32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 5,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epu32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 2,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epu32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, 2,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epu64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 4,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epu64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 4,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epu64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 1,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epu64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 1,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epu64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 5,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epu64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 5,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epu64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 2,
						  (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epu64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, 2,
						  (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epi32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 4,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epi32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 4,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epi32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 1,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epi32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 1,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epi32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 5,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epi32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 5,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epi32_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 2,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epi32_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, 2,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpneq_epi64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 4,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpneq_epi64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 4,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmplt_epi64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 1,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmplt_epi64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 1,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmpge_epi64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 5,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmpge_epi64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 5,
						 (__mmask8) -1);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmple_epi64_mask (__mmask8 __M, __m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 2,
						 (__mmask8) __M);
}

extern __inline __mmask8
  __attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmple_epi64_mask (__m128i __X, __m128i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, 2,
						 (__mmask8) -1);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutex_epi64 (__m256i __X, const int __I)
{
  return (__m256i) __builtin_ia32_permdi256_mask ((__v4di) __X,
					      __I,
					      (__v4di)
					      _mm256_setzero_si256(),
					      (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutex_epi64 (__m256i __W, __mmask8 __M,
			    __m256i __X, const int __I)
{
  return (__m256i) __builtin_ia32_permdi256_mask ((__v4di) __X,
						  __I,
						  (__v4di) __W,
						  (__mmask8) __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutex_epi64 (__mmask8 __M, __m256i __X, const int __I)
{
  return (__m256i) __builtin_ia32_permdi256_mask ((__v4di) __X,
						  __I,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __M);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_pd (__m256d __W, __mmask8 __U, __m256d __A,
			__m256d __B, const int __imm)
{
  return (__m256d) __builtin_ia32_shufpd256_mask ((__v4df) __A,
						  (__v4df) __B, __imm,
						  (__v4df) __W,
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_pd (__mmask8 __U, __m256d __A, __m256d __B,
			 const int __imm)
{
  return (__m256d) __builtin_ia32_shufpd256_mask ((__v4df) __A,
						  (__v4df) __B, __imm,
						  (__v4df)
						  _mm256_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shuffle_pd (__m128d __W, __mmask8 __U, __m128d __A,
		     __m128d __B, const int __imm)
{
  return (__m128d) __builtin_ia32_shufpd128_mask ((__v2df) __A,
						  (__v2df) __B, __imm,
						  (__v2df) __W,
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shuffle_pd (__mmask8 __U, __m128d __A, __m128d __B,
		      const int __imm)
{
  return (__m128d) __builtin_ia32_shufpd128_mask ((__v2df) __A,
						  (__v2df) __B, __imm,
						  (__v2df)
						  _mm_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_ps (__m256 __W, __mmask8 __U, __m256 __A,
			__m256 __B, const int __imm)
{
  return (__m256) __builtin_ia32_shufps256_mask ((__v8sf) __A,
						 (__v8sf) __B, __imm,
						 (__v8sf) __W,
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_ps (__mmask8 __U, __m256 __A, __m256 __B,
			 const int __imm)
{
  return (__m256) __builtin_ia32_shufps256_mask ((__v8sf) __A,
						 (__v8sf) __B, __imm,
						 (__v8sf)
						 _mm256_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shuffle_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
		     const int __imm)
{
  return (__m128) __builtin_ia32_shufps128_mask ((__v4sf) __A,
						 (__v4sf) __B, __imm,
						 (__v4sf) __W,
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shuffle_ps (__mmask8 __U, __m128 __A, __m128 __B,
		      const int __imm)
{
  return (__m128) __builtin_ia32_shufps128_mask ((__v4sf) __A,
						 (__v4sf) __B, __imm,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_inserti32x4 (__m256i __A, __m128i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_inserti32x4_256_mask ((__v8si) __A,
							(__v4si) __B,
							__imm,
							(__v8si)
							_mm256_setzero_si256 (),
							(__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_inserti32x4 (__m256i __W, __mmask8 __U, __m256i __A,
			 __m128i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_inserti32x4_256_mask ((__v8si) __A,
							(__v4si) __B,
							__imm,
							(__v8si) __W,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_inserti32x4 (__mmask8 __U, __m256i __A, __m128i __B,
			  const int __imm)
{
  return (__m256i) __builtin_ia32_inserti32x4_256_mask ((__v8si) __A,
							(__v4si) __B,
							__imm,
							(__v8si)
							_mm256_setzero_si256 (),
							(__mmask8)
							__U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insertf32x4 (__m256 __A, __m128 __B, const int __imm)
{
  return (__m256) __builtin_ia32_insertf32x4_256_mask ((__v8sf) __A,
						       (__v4sf) __B,
						       __imm,
						       (__v8sf)
						       _mm256_setzero_ps (),
						       (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_insertf32x4 (__m256 __W, __mmask8 __U, __m256 __A,
			 __m128 __B, const int __imm)
{
  return (__m256) __builtin_ia32_insertf32x4_256_mask ((__v8sf) __A,
						       (__v4sf) __B,
						       __imm,
						       (__v8sf) __W,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_insertf32x4 (__mmask8 __U, __m256 __A, __m128 __B,
			  const int __imm)
{
  return (__m256) __builtin_ia32_insertf32x4_256_mask ((__v8sf) __A,
						       (__v4sf) __B,
						       __imm,
						       (__v8sf)
						       _mm256_setzero_ps (),
						       (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extracti32x4_epi32 (__m256i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_extracti32x4_256_mask ((__v8si) __A,
							 __imm,
							 (__v4si)
							 _mm_setzero_si128 (),
							 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_extracti32x4_epi32 (__m128i __W, __mmask8 __U, __m256i __A,
				const int __imm)
{
  return (__m128i) __builtin_ia32_extracti32x4_256_mask ((__v8si) __A,
							 __imm,
							 (__v4si) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_extracti32x4_epi32 (__mmask8 __U, __m256i __A,
				 const int __imm)
{
  return (__m128i) __builtin_ia32_extracti32x4_256_mask ((__v8si) __A,
							 __imm,
							 (__v4si)
							 _mm_setzero_si128 (),
							 (__mmask8)
							 __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extractf32x4_ps (__m256 __A, const int __imm)
{
  return (__m128) __builtin_ia32_extractf32x4_256_mask ((__v8sf) __A,
							__imm,
							(__v4sf)
							_mm_setzero_ps (),
							(__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_extractf32x4_ps (__m128 __W, __mmask8 __U, __m256 __A,
			     const int __imm)
{
  return (__m128) __builtin_ia32_extractf32x4_256_mask ((__v8sf) __A,
							__imm,
							(__v4sf) __W,
							(__mmask8)
							__U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_extractf32x4_ps (__mmask8 __U, __m256 __A,
			      const int __imm)
{
  return (__m128) __builtin_ia32_extractf32x4_256_mask ((__v8sf) __A,
							__imm,
							(__v4sf)
							_mm_setzero_ps (),
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_i64x2 (__m256i __A, __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_shuf_i64x2_256_mask ((__v4di) __A,
						       (__v4di) __B,
						       __imm,
						       (__v4di)
						       _mm256_setzero_si256 (),
						       (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_i64x2 (__m256i __W, __mmask8 __U, __m256i __A,
			   __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_shuf_i64x2_256_mask ((__v4di) __A,
						       (__v4di) __B,
						       __imm,
						       (__v4di) __W,
						       (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_i64x2 (__mmask8 __U, __m256i __A, __m256i __B,
			    const int __imm)
{
  return (__m256i) __builtin_ia32_shuf_i64x2_256_mask ((__v4di) __A,
						       (__v4di) __B,
						       __imm,
						       (__v4di)
						       _mm256_setzero_si256 (),
						       (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_i32x4 (__m256i __A, __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_shuf_i32x4_256_mask ((__v8si) __A,
						       (__v8si) __B,
						       __imm,
						       (__v8si)
						       _mm256_setzero_si256 (),
						       (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_i32x4 (__m256i __W, __mmask8 __U, __m256i __A,
			   __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_shuf_i32x4_256_mask ((__v8si) __A,
						       (__v8si) __B,
						       __imm,
						       (__v8si) __W,
						       (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_i32x4 (__mmask8 __U, __m256i __A, __m256i __B,
			    const int __imm)
{
  return (__m256i) __builtin_ia32_shuf_i32x4_256_mask ((__v8si) __A,
						       (__v8si) __B,
						       __imm,
						       (__v8si)
						       _mm256_setzero_si256 (),
						       (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_f64x2 (__m256d __A, __m256d __B, const int __imm)
{
  return (__m256d) __builtin_ia32_shuf_f64x2_256_mask ((__v4df) __A,
						       (__v4df) __B,
						       __imm,
						       (__v4df)
						       _mm256_setzero_pd (),
						       (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_f64x2 (__m256d __W, __mmask8 __U, __m256d __A,
			   __m256d __B, const int __imm)
{
  return (__m256d) __builtin_ia32_shuf_f64x2_256_mask ((__v4df) __A,
						       (__v4df) __B,
						       __imm,
						       (__v4df) __W,
						       (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_f64x2 (__mmask8 __U, __m256d __A, __m256d __B,
			    const int __imm)
{
  return (__m256d) __builtin_ia32_shuf_f64x2_256_mask ((__v4df) __A,
						       (__v4df) __B,
						       __imm,
						       (__v4df)
						       _mm256_setzero_pd (),
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_f32x4 (__m256 __A, __m256 __B, const int __imm)
{
  return (__m256) __builtin_ia32_shuf_f32x4_256_mask ((__v8sf) __A,
						      (__v8sf) __B,
						      __imm,
						      (__v8sf)
						      _mm256_setzero_ps (),
						      (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_f32x4 (__m256 __W, __mmask8 __U, __m256 __A,
			   __m256 __B, const int __imm)
{
  return (__m256) __builtin_ia32_shuf_f32x4_256_mask ((__v8sf) __A,
						      (__v8sf) __B,
						      __imm,
						      (__v8sf) __W,
						      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_f32x4 (__mmask8 __U, __m256 __A, __m256 __B,
			    const int __imm)
{
  return (__m256) __builtin_ia32_shuf_f32x4_256_mask ((__v8sf) __A,
						      (__v8sf) __B,
						      __imm,
						      (__v8sf)
						      _mm256_setzero_ps (),
						      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fixupimm_pd (__m256d __A, __m256d __B, __m256i __C,
		    const int __imm)
{
  return (__m256d) __builtin_ia32_fixupimmpd256_mask ((__v4df) __A,
						      (__v4df) __B,
						      (__v4di) __C,
						      __imm,
						      (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fixupimm_pd (__m256d __A, __mmask8 __U, __m256d __B,
			 __m256i __C, const int __imm)
{
  return (__m256d) __builtin_ia32_fixupimmpd256_mask ((__v4df) __A,
						      (__v4df) __B,
						      (__v4di) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fixupimm_pd (__mmask8 __U, __m256d __A, __m256d __B,
			  __m256i __C, const int __imm)
{
  return (__m256d) __builtin_ia32_fixupimmpd256_maskz ((__v4df) __A,
						       (__v4df) __B,
						       (__v4di) __C,
						       __imm,
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fixupimm_ps (__m256 __A, __m256 __B, __m256i __C,
		    const int __imm)
{
  return (__m256) __builtin_ia32_fixupimmps256_mask ((__v8sf) __A,
						     (__v8sf) __B,
						     (__v8si) __C,
						     __imm,
						     (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fixupimm_ps (__m256 __A, __mmask8 __U, __m256 __B,
			 __m256i __C, const int __imm)
{
  return (__m256) __builtin_ia32_fixupimmps256_mask ((__v8sf) __A,
						     (__v8sf) __B,
						     (__v8si) __C,
						     __imm,
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_fixupimm_ps (__mmask8 __U, __m256 __A, __m256 __B,
			  __m256i __C, const int __imm)
{
  return (__m256) __builtin_ia32_fixupimmps256_maskz ((__v8sf) __A,
						      (__v8sf) __B,
						      (__v8si) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fixupimm_pd (__m128d __A, __m128d __B, __m128i __C,
		 const int __imm)
{
  return (__m128d) __builtin_ia32_fixupimmpd128_mask ((__v2df) __A,
						      (__v2df) __B,
						      (__v2di) __C,
						      __imm,
						      (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fixupimm_pd (__m128d __A, __mmask8 __U, __m128d __B,
		      __m128i __C, const int __imm)
{
  return (__m128d) __builtin_ia32_fixupimmpd128_mask ((__v2df) __A,
						      (__v2df) __B,
						      (__v2di) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fixupimm_pd (__mmask8 __U, __m128d __A, __m128d __B,
		       __m128i __C, const int __imm)
{
  return (__m128d) __builtin_ia32_fixupimmpd128_maskz ((__v2df) __A,
						       (__v2df) __B,
						       (__v2di) __C,
						       __imm,
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fixupimm_ps (__m128 __A, __m128 __B, __m128i __C, const int __imm)
{
  return (__m128) __builtin_ia32_fixupimmps128_mask ((__v4sf) __A,
						     (__v4sf) __B,
						     (__v4si) __C,
						     __imm,
						     (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fixupimm_ps (__m128 __A, __mmask8 __U, __m128 __B,
		      __m128i __C, const int __imm)
{
  return (__m128) __builtin_ia32_fixupimmps128_mask ((__v4sf) __A,
						     (__v4sf) __B,
						     (__v4si) __C,
						     __imm,
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fixupimm_ps (__mmask8 __U, __m128 __A, __m128 __B,
		       __m128i __C, const int __imm)
{
  return (__m128) __builtin_ia32_fixupimmps128_maskz ((__v4sf) __A,
						      (__v4sf) __B,
						      (__v4si) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srli_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			const int __imm)
{
  return (__m256i) __builtin_ia32_psrldi256_mask ((__v8si) __A, __imm,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srli_epi32 (__mmask8 __U, __m256i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_psrldi256_mask ((__v8si) __A, __imm,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srli_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		     const int __imm)
{
  return (__m128i) __builtin_ia32_psrldi128_mask ((__v4si) __A, __imm,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srli_epi32 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_psrldi128_mask ((__v4si) __A, __imm,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srli_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			const int __imm)
{
  return (__m256i) __builtin_ia32_psrlqi256_mask ((__v4di) __A, __imm,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srli_epi64 (__mmask8 __U, __m256i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_psrlqi256_mask ((__v4di) __A, __imm,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srli_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		     const int __imm)
{
  return (__m128i) __builtin_ia32_psrlqi128_mask ((__v2di) __A, __imm,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srli_epi64 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_psrlqi128_mask ((__v2di) __A, __imm,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_ternarylogic_epi64 (__m256i __A, __m256i __B, __m256i __C,
			   const int __imm)
{
  return (__m256i) __builtin_ia32_pternlogq256_mask ((__v4di) __A,
						     (__v4di) __B,
						     (__v4di) __C, __imm,
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_ternarylogic_epi64 (__m256i __A, __mmask8 __U,
				__m256i __B, __m256i __C,
				const int __imm)
{
  return (__m256i) __builtin_ia32_pternlogq256_mask ((__v4di) __A,
						     (__v4di) __B,
						     (__v4di) __C, __imm,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_ternarylogic_epi64 (__mmask8 __U, __m256i __A,
				 __m256i __B, __m256i __C,
				 const int __imm)
{
  return (__m256i) __builtin_ia32_pternlogq256_maskz ((__v4di) __A,
						      (__v4di) __B,
						      (__v4di) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_ternarylogic_epi32 (__m256i __A, __m256i __B, __m256i __C,
			   const int __imm)
{
  return (__m256i) __builtin_ia32_pternlogd256_mask ((__v8si) __A,
						     (__v8si) __B,
						     (__v8si) __C, __imm,
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_ternarylogic_epi32 (__m256i __A, __mmask8 __U,
				__m256i __B, __m256i __C,
				const int __imm)
{
  return (__m256i) __builtin_ia32_pternlogd256_mask ((__v8si) __A,
						     (__v8si) __B,
						     (__v8si) __C, __imm,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_ternarylogic_epi32 (__mmask8 __U, __m256i __A,
				 __m256i __B, __m256i __C,
				 const int __imm)
{
  return (__m256i) __builtin_ia32_pternlogd256_maskz ((__v8si) __A,
						      (__v8si) __B,
						      (__v8si) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_ternarylogic_epi64 (__m128i __A, __m128i __B, __m128i __C,
			const int __imm)
{
  return (__m128i) __builtin_ia32_pternlogq128_mask ((__v2di) __A,
						     (__v2di) __B,
						     (__v2di) __C, __imm,
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_ternarylogic_epi64 (__m128i __A, __mmask8 __U,
			     __m128i __B, __m128i __C, const int __imm)
{
  return (__m128i) __builtin_ia32_pternlogq128_mask ((__v2di) __A,
						     (__v2di) __B,
						     (__v2di) __C, __imm,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_ternarylogic_epi64 (__mmask8 __U, __m128i __A,
			      __m128i __B, __m128i __C, const int __imm)
{
  return (__m128i) __builtin_ia32_pternlogq128_maskz ((__v2di) __A,
						      (__v2di) __B,
						      (__v2di) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_ternarylogic_epi32 (__m128i __A, __m128i __B, __m128i __C,
			const int __imm)
{
  return (__m128i) __builtin_ia32_pternlogd128_mask ((__v4si) __A,
						     (__v4si) __B,
						     (__v4si) __C, __imm,
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_ternarylogic_epi32 (__m128i __A, __mmask8 __U,
			     __m128i __B, __m128i __C, const int __imm)
{
  return (__m128i) __builtin_ia32_pternlogd128_mask ((__v4si) __A,
						     (__v4si) __B,
						     (__v4si) __C, __imm,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_ternarylogic_epi32 (__mmask8 __U, __m128i __A,
			      __m128i __B, __m128i __C, const int __imm)
{
  return (__m128i) __builtin_ia32_pternlogd128_maskz ((__v4si) __A,
						      (__v4si) __B,
						      (__v4si) __C,
						      __imm,
						      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_roundscale_ps (__m256 __A, const int __imm)
{
  return (__m256) __builtin_ia32_rndscaleps_256_mask ((__v8sf) __A,
						      __imm,
						      (__v8sf)
						      _mm256_setzero_ps (),
						      (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_roundscale_ps (__m256 __W, __mmask8 __U, __m256 __A,
			   const int __imm)
{
  return (__m256) __builtin_ia32_rndscaleps_256_mask ((__v8sf) __A,
						      __imm,
						      (__v8sf) __W,
						      (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_roundscale_ps (__mmask8 __U, __m256 __A, const int __imm)
{
  return (__m256) __builtin_ia32_rndscaleps_256_mask ((__v8sf) __A,
						      __imm,
						      (__v8sf)
						      _mm256_setzero_ps (),
						      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_roundscale_pd (__m256d __A, const int __imm)
{
  return (__m256d) __builtin_ia32_rndscalepd_256_mask ((__v4df) __A,
						       __imm,
						       (__v4df)
						       _mm256_setzero_pd (),
						       (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_roundscale_pd (__m256d __W, __mmask8 __U, __m256d __A,
			   const int __imm)
{
  return (__m256d) __builtin_ia32_rndscalepd_256_mask ((__v4df) __A,
						       __imm,
						       (__v4df) __W,
						       (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_roundscale_pd (__mmask8 __U, __m256d __A, const int __imm)
{
  return (__m256d) __builtin_ia32_rndscalepd_256_mask ((__v4df) __A,
						       __imm,
						       (__v4df)
						       _mm256_setzero_pd (),
						       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_roundscale_ps (__m128 __A, const int __imm)
{
  return (__m128) __builtin_ia32_rndscaleps_128_mask ((__v4sf) __A,
						      __imm,
						      (__v4sf)
						      _mm_setzero_ps (),
						      (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_roundscale_ps (__m128 __W, __mmask8 __U, __m128 __A,
			const int __imm)
{
  return (__m128) __builtin_ia32_rndscaleps_128_mask ((__v4sf) __A,
						      __imm,
						      (__v4sf) __W,
						      (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_roundscale_ps (__mmask8 __U, __m128 __A, const int __imm)
{
  return (__m128) __builtin_ia32_rndscaleps_128_mask ((__v4sf) __A,
						      __imm,
						      (__v4sf)
						      _mm_setzero_ps (),
						      (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_roundscale_pd (__m128d __A, const int __imm)
{
  return (__m128d) __builtin_ia32_rndscalepd_128_mask ((__v2df) __A,
						       __imm,
						       (__v2df)
						       _mm_setzero_pd (),
						       (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_roundscale_pd (__m128d __W, __mmask8 __U, __m128d __A,
			const int __imm)
{
  return (__m128d) __builtin_ia32_rndscalepd_128_mask ((__v2df) __A,
						       __imm,
						       (__v2df) __W,
						       (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_roundscale_pd (__mmask8 __U, __m128d __A, const int __imm)
{
  return (__m128d) __builtin_ia32_rndscalepd_128_mask ((__v2df) __A,
						       __imm,
						       (__v2df)
						       _mm_setzero_pd (),
						       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_getmant_ps (__m256 __A, _MM_MANTISSA_NORM_ENUM __B,
		   _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m256) __builtin_ia32_getmantps256_mask ((__v8sf) __A,
						    (__C << 2) | __B,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_getmant_ps (__m256 __W, __mmask8 __U, __m256 __A,
			_MM_MANTISSA_NORM_ENUM __B,
			_MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m256) __builtin_ia32_getmantps256_mask ((__v8sf) __A,
						    (__C << 2) | __B,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_getmant_ps (__mmask8 __U, __m256 __A,
			 _MM_MANTISSA_NORM_ENUM __B,
			 _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m256) __builtin_ia32_getmantps256_mask ((__v8sf) __A,
						    (__C << 2) | __B,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getmant_ps (__m128 __A, _MM_MANTISSA_NORM_ENUM __B,
		_MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m128) __builtin_ia32_getmantps128_mask ((__v4sf) __A,
						    (__C << 2) | __B,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getmant_ps (__m128 __W, __mmask8 __U, __m128 __A,
		     _MM_MANTISSA_NORM_ENUM __B,
		     _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m128) __builtin_ia32_getmantps128_mask ((__v4sf) __A,
						    (__C << 2) | __B,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getmant_ps (__mmask8 __U, __m128 __A,
		      _MM_MANTISSA_NORM_ENUM __B,
		      _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m128) __builtin_ia32_getmantps128_mask ((__v4sf) __A,
						    (__C << 2) | __B,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_getmant_pd (__m256d __A, _MM_MANTISSA_NORM_ENUM __B,
		   _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m256d) __builtin_ia32_getmantpd256_mask ((__v4df) __A,
						     (__C << 2) | __B,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_getmant_pd (__m256d __W, __mmask8 __U, __m256d __A,
			_MM_MANTISSA_NORM_ENUM __B,
			_MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m256d) __builtin_ia32_getmantpd256_mask ((__v4df) __A,
						     (__C << 2) | __B,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_getmant_pd (__mmask8 __U, __m256d __A,
			 _MM_MANTISSA_NORM_ENUM __B,
			 _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m256d) __builtin_ia32_getmantpd256_mask ((__v4df) __A,
						     (__C << 2) | __B,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getmant_pd (__m128d __A, _MM_MANTISSA_NORM_ENUM __B,
		_MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m128d) __builtin_ia32_getmantpd128_mask ((__v2df) __A,
						     (__C << 2) | __B,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getmant_pd (__m128d __W, __mmask8 __U, __m128d __A,
		     _MM_MANTISSA_NORM_ENUM __B,
		     _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m128d) __builtin_ia32_getmantpd128_mask ((__v2df) __A,
						     (__C << 2) | __B,
						     (__v2df) __W,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getmant_pd (__mmask8 __U, __m128d __A,
		      _MM_MANTISSA_NORM_ENUM __B,
		      _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m128d) __builtin_ia32_getmantpd128_mask ((__v2df) __A,
						     (__C << 2) | __B,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i32gather_ps (__m256 __v1_old, __mmask8 __mask,
			   __m256i __index, void const *__addr,
			   int __scale)
{
  return (__m256) __builtin_ia32_gather3siv8sf ((__v8sf) __v1_old,
						__addr,
						(__v8si) __index,
						__mask, __scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i32gather_ps (__m128 __v1_old, __mmask8 __mask,
			__m128i __index, void const *__addr,
			int __scale)
{
  return (__m128) __builtin_ia32_gather3siv4sf ((__v4sf) __v1_old,
						__addr,
						(__v4si) __index,
						__mask, __scale);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i32gather_pd (__m256d __v1_old, __mmask8 __mask,
			   __m128i __index, void const *__addr,
			   int __scale)
{
  return (__m256d) __builtin_ia32_gather3siv4df ((__v4df) __v1_old,
						 __addr,
						 (__v4si) __index,
						 __mask, __scale);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i32gather_pd (__m128d __v1_old, __mmask8 __mask,
			__m128i __index, void const *__addr,
			int __scale)
{
  return (__m128d) __builtin_ia32_gather3siv2df ((__v2df) __v1_old,
						 __addr,
						 (__v4si) __index,
						 __mask, __scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i64gather_ps (__m128 __v1_old, __mmask8 __mask,
			   __m256i __index, void const *__addr,
			   int __scale)
{
  return (__m128) __builtin_ia32_gather3div8sf ((__v4sf) __v1_old,
						__addr,
						(__v4di) __index,
						__mask, __scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i64gather_ps (__m128 __v1_old, __mmask8 __mask,
			__m128i __index, void const *__addr,
			int __scale)
{
  return (__m128) __builtin_ia32_gather3div4sf ((__v4sf) __v1_old,
						__addr,
						(__v2di) __index,
						__mask, __scale);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i64gather_pd (__m256d __v1_old, __mmask8 __mask,
			   __m256i __index, void const *__addr,
			   int __scale)
{
  return (__m256d) __builtin_ia32_gather3div4df ((__v4df) __v1_old,
						 __addr,
						 (__v4di) __index,
						 __mask, __scale);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i64gather_pd (__m128d __v1_old, __mmask8 __mask,
			__m128i __index, void const *__addr,
			int __scale)
{
  return (__m128d) __builtin_ia32_gather3div2df ((__v2df) __v1_old,
						 __addr,
						 (__v2di) __index,
						 __mask, __scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i32gather_epi32 (__m256i __v1_old, __mmask8 __mask,
			      __m256i __index, void const *__addr,
			      int __scale)
{
  return (__m256i) __builtin_ia32_gather3siv8si ((__v8si) __v1_old,
						 __addr,
						 (__v8si) __index,
						 __mask, __scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i32gather_epi32 (__m128i __v1_old, __mmask8 __mask,
			   __m128i __index, void const *__addr,
			   int __scale)
{
  return (__m128i) __builtin_ia32_gather3siv4si ((__v4si) __v1_old,
						 __addr,
						 (__v4si) __index,
						 __mask, __scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i32gather_epi64 (__m256i __v1_old, __mmask8 __mask,
			      __m128i __index, void const *__addr,
			      int __scale)
{
  return (__m256i) __builtin_ia32_gather3siv4di ((__v4di) __v1_old,
						 __addr,
						 (__v4si) __index,
						 __mask, __scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i32gather_epi64 (__m128i __v1_old, __mmask8 __mask,
			   __m128i __index, void const *__addr,
			   int __scale)
{
  return (__m128i) __builtin_ia32_gather3siv2di ((__v2di) __v1_old,
						 __addr,
						 (__v4si) __index,
						 __mask, __scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i64gather_epi32 (__m128i __v1_old, __mmask8 __mask,
			      __m256i __index, void const *__addr,
			      int __scale)
{
  return (__m128i) __builtin_ia32_gather3div8si ((__v4si) __v1_old,
						 __addr,
						 (__v4di) __index,
						 __mask, __scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i64gather_epi32 (__m128i __v1_old, __mmask8 __mask,
			   __m128i __index, void const *__addr,
			   int __scale)
{
  return (__m128i) __builtin_ia32_gather3div4si ((__v4si) __v1_old,
						 __addr,
						 (__v2di) __index,
						 __mask, __scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mmask_i64gather_epi64 (__m256i __v1_old, __mmask8 __mask,
			      __m256i __index, void const *__addr,
			      int __scale)
{
  return (__m256i) __builtin_ia32_gather3div4di ((__v4di) __v1_old,
						 __addr,
						 (__v4di) __index,
						 __mask, __scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mmask_i64gather_epi64 (__m128i __v1_old, __mmask8 __mask,
			   __m128i __index, void const *__addr,
			   int __scale)
{
  return (__m128i) __builtin_ia32_gather3div2di ((__v2di) __v1_old,
						 __addr,
						 (__v2di) __index,
						 __mask, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32scatter_ps (void *__addr, __m256i __index,
		      __m256 __v1, const int __scale)
{
  __builtin_ia32_scattersiv8sf (__addr, (__mmask8) 0xFF,
				(__v8si) __index, (__v8sf) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32scatter_ps (void *__addr, __mmask8 __mask,
			   __m256i __index, __m256 __v1,
			   const int __scale)
{
  __builtin_ia32_scattersiv8sf (__addr, __mask, (__v8si) __index,
				(__v8sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32scatter_ps (void *__addr, __m128i __index, __m128 __v1,
		   const int __scale)
{
  __builtin_ia32_scattersiv4sf (__addr, (__mmask8) 0xFF,
				(__v4si) __index, (__v4sf) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32scatter_ps (void *__addr, __mmask8 __mask,
			__m128i __index, __m128 __v1,
			const int __scale)
{
  __builtin_ia32_scattersiv4sf (__addr, __mask, (__v4si) __index,
				(__v4sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32scatter_pd (void *__addr, __m128i __index,
		      __m256d __v1, const int __scale)
{
  __builtin_ia32_scattersiv4df (__addr, (__mmask8) 0xFF,
				(__v4si) __index, (__v4df) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32scatter_pd (void *__addr, __mmask8 __mask,
			   __m128i __index, __m256d __v1,
			   const int __scale)
{
  __builtin_ia32_scattersiv4df (__addr, __mask, (__v4si) __index,
				(__v4df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32scatter_pd (void *__addr, __m128i __index,
		   __m128d __v1, const int __scale)
{
  __builtin_ia32_scattersiv2df (__addr, (__mmask8) 0xFF,
				(__v4si) __index, (__v2df) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32scatter_pd (void *__addr, __mmask8 __mask,
			__m128i __index, __m128d __v1,
			const int __scale)
{
  __builtin_ia32_scattersiv2df (__addr, __mask, (__v4si) __index,
				(__v2df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64scatter_ps (void *__addr, __m256i __index,
		      __m128 __v1, const int __scale)
{
  __builtin_ia32_scatterdiv8sf (__addr, (__mmask8) 0xFF,
				(__v4di) __index, (__v4sf) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64scatter_ps (void *__addr, __mmask8 __mask,
			   __m256i __index, __m128 __v1,
			   const int __scale)
{
  __builtin_ia32_scatterdiv8sf (__addr, __mask, (__v4di) __index,
				(__v4sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64scatter_ps (void *__addr, __m128i __index, __m128 __v1,
		   const int __scale)
{
  __builtin_ia32_scatterdiv4sf (__addr, (__mmask8) 0xFF,
				(__v2di) __index, (__v4sf) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64scatter_ps (void *__addr, __mmask8 __mask,
			__m128i __index, __m128 __v1,
			const int __scale)
{
  __builtin_ia32_scatterdiv4sf (__addr, __mask, (__v2di) __index,
				(__v4sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64scatter_pd (void *__addr, __m256i __index,
		      __m256d __v1, const int __scale)
{
  __builtin_ia32_scatterdiv4df (__addr, (__mmask8) 0xFF,
				(__v4di) __index, (__v4df) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64scatter_pd (void *__addr, __mmask8 __mask,
			   __m256i __index, __m256d __v1,
			   const int __scale)
{
  __builtin_ia32_scatterdiv4df (__addr, __mask, (__v4di) __index,
				(__v4df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64scatter_pd (void *__addr, __m128i __index,
		   __m128d __v1, const int __scale)
{
  __builtin_ia32_scatterdiv2df (__addr, (__mmask8) 0xFF,
				(__v2di) __index, (__v2df) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64scatter_pd (void *__addr, __mmask8 __mask,
			__m128i __index, __m128d __v1,
			const int __scale)
{
  __builtin_ia32_scatterdiv2df (__addr, __mask, (__v2di) __index,
				(__v2df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32scatter_epi32 (void *__addr, __m256i __index,
			 __m256i __v1, const int __scale)
{
  __builtin_ia32_scattersiv8si (__addr, (__mmask8) 0xFF,
				(__v8si) __index, (__v8si) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32scatter_epi32 (void *__addr, __mmask8 __mask,
			      __m256i __index, __m256i __v1,
			      const int __scale)
{
  __builtin_ia32_scattersiv8si (__addr, __mask, (__v8si) __index,
				(__v8si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32scatter_epi32 (void *__addr, __m128i __index,
		      __m128i __v1, const int __scale)
{
  __builtin_ia32_scattersiv4si (__addr, (__mmask8) 0xFF,
				(__v4si) __index, (__v4si) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32scatter_epi32 (void *__addr, __mmask8 __mask,
			   __m128i __index, __m128i __v1,
			   const int __scale)
{
  __builtin_ia32_scattersiv4si (__addr, __mask, (__v4si) __index,
				(__v4si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32scatter_epi64 (void *__addr, __m128i __index,
			 __m256i __v1, const int __scale)
{
  __builtin_ia32_scattersiv4di (__addr, (__mmask8) 0xFF,
				(__v4si) __index, (__v4di) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32scatter_epi64 (void *__addr, __mmask8 __mask,
			      __m128i __index, __m256i __v1,
			      const int __scale)
{
  __builtin_ia32_scattersiv4di (__addr, __mask, (__v4si) __index,
				(__v4di) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32scatter_epi64 (void *__addr, __m128i __index,
		      __m128i __v1, const int __scale)
{
  __builtin_ia32_scattersiv2di (__addr, (__mmask8) 0xFF,
				(__v4si) __index, (__v2di) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32scatter_epi64 (void *__addr, __mmask8 __mask,
			   __m128i __index, __m128i __v1,
			   const int __scale)
{
  __builtin_ia32_scattersiv2di (__addr, __mask, (__v4si) __index,
				(__v2di) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64scatter_epi32 (void *__addr, __m256i __index,
			 __m128i __v1, const int __scale)
{
  __builtin_ia32_scatterdiv8si (__addr, (__mmask8) 0xFF,
				(__v4di) __index, (__v4si) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64scatter_epi32 (void *__addr, __mmask8 __mask,
			      __m256i __index, __m128i __v1,
			      const int __scale)
{
  __builtin_ia32_scatterdiv8si (__addr, __mask, (__v4di) __index,
				(__v4si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64scatter_epi32 (void *__addr, __m128i __index,
		      __m128i __v1, const int __scale)
{
  __builtin_ia32_scatterdiv4si (__addr, (__mmask8) 0xFF,
				(__v2di) __index, (__v4si) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64scatter_epi32 (void *__addr, __mmask8 __mask,
			   __m128i __index, __m128i __v1,
			   const int __scale)
{
  __builtin_ia32_scatterdiv4si (__addr, __mask, (__v2di) __index,
				(__v4si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64scatter_epi64 (void *__addr, __m256i __index,
			 __m256i __v1, const int __scale)
{
  __builtin_ia32_scatterdiv4di (__addr, (__mmask8) 0xFF,
				(__v4di) __index, (__v4di) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64scatter_epi64 (void *__addr, __mmask8 __mask,
			      __m256i __index, __m256i __v1,
			      const int __scale)
{
  __builtin_ia32_scatterdiv4di (__addr, __mask, (__v4di) __index,
				(__v4di) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64scatter_epi64 (void *__addr, __m128i __index,
		      __m128i __v1, const int __scale)
{
  __builtin_ia32_scatterdiv2di (__addr, (__mmask8) 0xFF,
				(__v2di) __index, (__v2di) __v1,
				__scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64scatter_epi64 (void *__addr, __mmask8 __mask,
			   __m128i __index, __m128i __v1,
			   const int __scale)
{
  __builtin_ia32_scatterdiv2di (__addr, __mask, (__v2di) __index,
				(__v2di) __v1, __scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_shuffle_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			   _MM_PERM_ENUM __mask)
{
  return (__m256i) __builtin_ia32_pshufd256_mask ((__v8si) __A, __mask,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_shuffle_epi32 (__mmask8 __U, __m256i __A,
			    _MM_PERM_ENUM __mask)
{
  return (__m256i) __builtin_ia32_pshufd256_mask ((__v8si) __A, __mask,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_shuffle_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
			_MM_PERM_ENUM __mask)
{
  return (__m128i) __builtin_ia32_pshufd128_mask ((__v4si) __A, __mask,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_shuffle_epi32 (__mmask8 __U, __m128i __A,
			 _MM_PERM_ENUM __mask)
{
  return (__m128i) __builtin_ia32_pshufd128_mask ((__v4si) __A, __mask,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rol_epi32 (__m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prold256_mask ((__v8si) __A, __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rol_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       const int __B)
{
  return (__m256i) __builtin_ia32_prold256_mask ((__v8si) __A, __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rol_epi32 (__mmask8 __U, __m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prold256_mask ((__v8si) __A, __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rol_epi32 (__m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prold128_mask ((__v4si) __A, __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rol_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    const int __B)
{
  return (__m128i) __builtin_ia32_prold128_mask ((__v4si) __A, __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rol_epi32 (__mmask8 __U, __m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prold128_mask ((__v4si) __A, __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_ror_epi32 (__m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prord256_mask ((__v8si) __A, __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_ror_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
		       const int __B)
{
  return (__m256i) __builtin_ia32_prord256_mask ((__v8si) __A, __B,
						 (__v8si) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_ror_epi32 (__mmask8 __U, __m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prord256_mask ((__v8si) __A, __B,
						 (__v8si)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_ror_epi32 (__m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prord128_mask ((__v4si) __A, __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_ror_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		    const int __B)
{
  return (__m128i) __builtin_ia32_prord128_mask ((__v4si) __A, __B,
						 (__v4si) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_ror_epi32 (__mmask8 __U, __m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prord128_mask ((__v4si) __A, __B,
						 (__v4si)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rol_epi64 (__m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prolq256_mask ((__v4di) __A, __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_rol_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       const int __B)
{
  return (__m256i) __builtin_ia32_prolq256_mask ((__v4di) __A, __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_rol_epi64 (__mmask8 __U, __m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prolq256_mask ((__v4di) __A, __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rol_epi64 (__m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prolq128_mask ((__v2di) __A, __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rol_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    const int __B)
{
  return (__m128i) __builtin_ia32_prolq128_mask ((__v2di) __A, __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rol_epi64 (__mmask8 __U, __m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prolq128_mask ((__v2di) __A, __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_ror_epi64 (__m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prorq256_mask ((__v4di) __A, __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_ror_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
		       const int __B)
{
  return (__m256i) __builtin_ia32_prorq256_mask ((__v4di) __A, __B,
						 (__v4di) __W,
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_ror_epi64 (__mmask8 __U, __m256i __A, const int __B)
{
  return (__m256i) __builtin_ia32_prorq256_mask ((__v4di) __A, __B,
						 (__v4di)
						 _mm256_setzero_si256 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_ror_epi64 (__m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prorq128_mask ((__v2di) __A, __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_ror_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		    const int __B)
{
  return (__m128i) __builtin_ia32_prorq128_mask ((__v2di) __A, __B,
						 (__v2di) __W,
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_ror_epi64 (__mmask8 __U, __m128i __A, const int __B)
{
  return (__m128i) __builtin_ia32_prorq128_mask ((__v2di) __A, __B,
						 (__v2di)
						 _mm_setzero_si128 (),
						 (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_alignr_epi32 (__m128i __A, __m128i __B, const int __imm)
{
  return (__m128i) __builtin_ia32_alignd128_mask ((__v4si) __A,
						  (__v4si) __B, __imm,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_alignr_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		       __m128i __B, const int __imm)
{
  return (__m128i) __builtin_ia32_alignd128_mask ((__v4si) __A,
						  (__v4si) __B, __imm,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_alignr_epi32 (__mmask8 __U, __m128i __A, __m128i __B,
			const int __imm)
{
  return (__m128i) __builtin_ia32_alignd128_mask ((__v4si) __A,
						  (__v4si) __B, __imm,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_alignr_epi64 (__m128i __A, __m128i __B, const int __imm)
{
  return (__m128i) __builtin_ia32_alignq128_mask ((__v2di) __A,
						  (__v2di) __B, __imm,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_alignr_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		       __m128i __B, const int __imm)
{
  return (__m128i) __builtin_ia32_alignq128_mask ((__v2di) __A,
						  (__v2di) __B, __imm,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_alignr_epi64 (__mmask8 __U, __m128i __A, __m128i __B,
			const int __imm)
{
  return (__m128i) __builtin_ia32_alignq128_mask ((__v2di) __A,
						  (__v2di) __B, __imm,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_alignr_epi32 (__m256i __A, __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_alignd256_mask ((__v8si) __A,
						  (__v8si) __B, __imm,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_alignr_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			  __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_alignd256_mask ((__v8si) __A,
						  (__v8si) __B, __imm,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_alignr_epi32 (__mmask8 __U, __m256i __A, __m256i __B,
			   const int __imm)
{
  return (__m256i) __builtin_ia32_alignd256_mask ((__v8si) __A,
						  (__v8si) __B, __imm,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_alignr_epi64 (__m256i __A, __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_alignq256_mask ((__v4di) __A,
						  (__v4di) __B, __imm,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_alignr_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			  __m256i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_alignq256_mask ((__v4di) __A,
						  (__v4di) __B, __imm,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_alignr_epi64 (__mmask8 __U, __m256i __A, __m256i __B,
			   const int __imm)
{
  return (__m256i) __builtin_ia32_alignq256_mask ((__v4di) __A,
						  (__v4di) __B, __imm,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtps_ph (__m128i __W, __mmask8 __U, __m128 __A,
		   const int __I)
{
  return (__m128i) __builtin_ia32_vcvtps2ph_mask ((__v4sf) __A, __I,
						  (__v8hi) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtps_ph (__mmask8 __U, __m128 __A, const int __I)
{
  return (__m128i) __builtin_ia32_vcvtps2ph_mask ((__v4sf) __A, __I,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtps_ph (__m128i __W, __mmask8 __U, __m256 __A,
		      const int __I)
{
  return (__m128i) __builtin_ia32_vcvtps2ph256_mask ((__v8sf) __A, __I,
						     (__v8hi) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtps_ph (__mmask8 __U, __m256 __A, const int __I)
{
  return (__m128i) __builtin_ia32_vcvtps2ph256_mask ((__v8sf) __A, __I,
						     (__v8hi)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srai_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			const int __imm)
{
  return (__m256i) __builtin_ia32_psradi256_mask ((__v8si) __A, __imm,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srai_epi32 (__mmask8 __U, __m256i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_psradi256_mask ((__v8si) __A, __imm,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srai_epi32 (__m128i __W, __mmask8 __U, __m128i __A,
		     const int __imm)
{
  return (__m128i) __builtin_ia32_psradi128_mask ((__v4si) __A, __imm,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srai_epi32 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_psradi128_mask ((__v4si) __A, __imm,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srai_epi64 (__m256i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_psraqi256_mask ((__v4di) __A, __imm,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_srai_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			const int __imm)
{
  return (__m256i) __builtin_ia32_psraqi256_mask ((__v4di) __A, __imm,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_srai_epi64 (__mmask8 __U, __m256i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_psraqi256_mask ((__v4di) __A, __imm,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_srai_epi64 (__m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_psraqi128_mask ((__v2di) __A, __imm,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_srai_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		     const int __imm)
{
  return (__m128i) __builtin_ia32_psraqi128_mask ((__v2di) __A, __imm,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_srai_epi64 (__mmask8 __U, __m128i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_psraqi128_mask ((__v2di) __A, __imm,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_slli_epi32 (__m128i __W, __mmask8 __U, __m128i __A, int __B)
{
  return (__m128i) __builtin_ia32_pslldi128_mask ((__v4si) __A, __B,
						  (__v4si) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_slli_epi32 (__mmask8 __U, __m128i __A, int __B)
{
  return (__m128i) __builtin_ia32_pslldi128_mask ((__v4si) __A, __B,
						  (__v4si)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_slli_epi64 (__m128i __W, __mmask8 __U, __m128i __A, int __B)
{
  return (__m128i) __builtin_ia32_psllqi128_mask ((__v2di) __A, __B,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_slli_epi64 (__mmask8 __U, __m128i __A, int __B)
{
  return (__m128i) __builtin_ia32_psllqi128_mask ((__v2di) __A, __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_slli_epi32 (__m256i __W, __mmask8 __U, __m256i __A,
			int __B)
{
  return (__m256i) __builtin_ia32_pslldi256_mask ((__v8si) __A, __B,
						  (__v8si) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_slli_epi32 (__mmask8 __U, __m256i __A, int __B)
{
  return (__m256i) __builtin_ia32_pslldi256_mask ((__v8si) __A, __B,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_slli_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			int __B)
{
  return (__m256i) __builtin_ia32_psllqi256_mask ((__v4di) __A, __B,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_slli_epi64 (__mmask8 __U, __m256i __A, int __B)
{
  return (__m256i) __builtin_ia32_psllqi256_mask ((__v4di) __A, __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permutex_pd (__m256d __W, __mmask8 __U, __m256d __X,
			 const int __imm)
{
  return (__m256d) __builtin_ia32_permdf256_mask ((__v4df) __X, __imm,
						  (__v4df) __W,
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permutex_pd (__mmask8 __U, __m256d __X, const int __imm)
{
  return (__m256d) __builtin_ia32_permdf256_mask ((__v4df) __X, __imm,
						  (__v4df)
						  _mm256_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permute_pd (__m256d __W, __mmask8 __U, __m256d __X,
			const int __C)
{
  return (__m256d) __builtin_ia32_vpermilpd256_mask ((__v4df) __X, __C,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permute_pd (__mmask8 __U, __m256d __X, const int __C)
{
  return (__m256d) __builtin_ia32_vpermilpd256_mask ((__v4df) __X, __C,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permute_pd (__m128d __W, __mmask8 __U, __m128d __X,
		     const int __C)
{
  return (__m128d) __builtin_ia32_vpermilpd_mask ((__v2df) __X, __C,
						  (__v2df) __W,
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permute_pd (__mmask8 __U, __m128d __X, const int __C)
{
  return (__m128d) __builtin_ia32_vpermilpd_mask ((__v2df) __X, __C,
						  (__v2df)
						  _mm_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_permute_ps (__m256 __W, __mmask8 __U, __m256 __X,
			const int __C)
{
  return (__m256) __builtin_ia32_vpermilps256_mask ((__v8sf) __X, __C,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_permute_ps (__mmask8 __U, __m256 __X, const int __C)
{
  return (__m256) __builtin_ia32_vpermilps256_mask ((__v8sf) __X, __C,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_permute_ps (__m128 __W, __mmask8 __U, __m128 __X,
		     const int __C)
{
  return (__m128) __builtin_ia32_vpermilps_mask ((__v4sf) __X, __C,
						 (__v4sf) __W,
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_permute_ps (__mmask8 __U, __m128 __X, const int __C)
{
  return (__m128) __builtin_ia32_vpermilps_mask ((__v4sf) __X, __C,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_blend_pd (__mmask8 __U, __m256d __A, __m256d __W)
{
  return (__m256d) __builtin_ia32_blendmpd_256_mask ((__v4df) __A,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_blend_ps (__mmask8 __U, __m256 __A, __m256 __W)
{
  return (__m256) __builtin_ia32_blendmps_256_mask ((__v8sf) __A,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_blend_epi64 (__mmask8 __U, __m256i __A, __m256i __W)
{
  return (__m256i) __builtin_ia32_blendmq_256_mask ((__v4di) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_blend_epi32 (__mmask8 __U, __m256i __A, __m256i __W)
{
  return (__m256i) __builtin_ia32_blendmd_256_mask ((__v8si) __A,
						    (__v8si) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_blend_pd (__mmask8 __U, __m128d __A, __m128d __W)
{
  return (__m128d) __builtin_ia32_blendmpd_128_mask ((__v2df) __A,
						     (__v2df) __W,
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_blend_ps (__mmask8 __U, __m128 __A, __m128 __W)
{
  return (__m128) __builtin_ia32_blendmps_128_mask ((__v4sf) __A,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_blend_epi64 (__mmask8 __U, __m128i __A, __m128i __W)
{
  return (__m128i) __builtin_ia32_blendmq_128_mask ((__v2di) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_blend_epi32 (__mmask8 __U, __m128i __A, __m128i __W)
{
  return (__m128i) __builtin_ia32_blendmd_128_mask ((__v4si) __A,
						    (__v4si) __W,
						    (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epi64_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, __P,
						 (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epi32_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, __P,
						 (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epu64_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_epu32_mask (__m256i __X, __m256i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_pd_mask (__m256d __X, __m256d __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmppd256_mask ((__v4df) __X,
						  (__v4df) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_ps_mask (__m256 __X, __m256 __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpps256_mask ((__v8sf) __X,
						  (__v8sf) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epi64_mask (__mmask8 __U, __m256i __X, __m256i __Y,
			    const int __P)
{
  return (__mmask8) __builtin_ia32_cmpq256_mask ((__v4di) __X,
						 (__v4di) __Y, __P,
						 (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epi32_mask (__mmask8 __U, __m256i __X, __m256i __Y,
			    const int __P)
{
  return (__mmask8) __builtin_ia32_cmpd256_mask ((__v8si) __X,
						 (__v8si) __Y, __P,
						 (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epu64_mask (__mmask8 __U, __m256i __X, __m256i __Y,
			    const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di) __X,
						  (__v4di) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_epu32_mask (__mmask8 __U, __m256i __X, __m256i __Y,
			    const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si) __X,
						  (__v8si) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_pd_mask (__mmask8 __U, __m256d __X, __m256d __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_cmppd256_mask ((__v4df) __X,
						  (__v4df) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cmp_ps_mask (__mmask8 __U, __m256 __X, __m256 __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_cmpps256_mask ((__v8sf) __X,
						  (__v8sf) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epi64_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, __P,
						 (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epi32_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, __P,
						 (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epu64_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_epu32_mask (__m128i __X, __m128i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_pd_mask (__m128d __X, __m128d __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmppd128_mask ((__v2df) __X,
						  (__v2df) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_ps_mask (__m128 __X, __m128 __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpps128_mask ((__v4sf) __X,
						  (__v4sf) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epi64_mask (__mmask8 __U, __m128i __X, __m128i __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_cmpq128_mask ((__v2di) __X,
						 (__v2di) __Y, __P,
						 (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epi32_mask (__mmask8 __U, __m128i __X, __m128i __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_cmpd128_mask ((__v4si) __X,
						 (__v4si) __Y, __P,
						 (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epu64_mask (__mmask8 __U, __m128i __X, __m128i __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di) __X,
						  (__v2di) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_epu32_mask (__mmask8 __U, __m128i __X, __m128i __Y,
			 const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si) __X,
						  (__v4si) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_pd_mask (__mmask8 __U, __m128d __X, __m128d __Y,
		      const int __P)
{
  return (__mmask8) __builtin_ia32_cmppd128_mask ((__v2df) __X,
						  (__v2df) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_ps_mask (__mmask8 __U, __m128 __X, __m128 __Y,
		      const int __P)
{
  return (__mmask8) __builtin_ia32_cmpps128_mask ((__v4sf) __X,
						  (__v4sf) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutex_pd (__m256d __X, const int __M)
{
  return (__m256d) __builtin_ia32_permdf256_mask ((__v4df) __X, __M,
						  (__v4df)
						  _mm256_undefined_pd (),
						  (__mmask8) -1);
}

#else
#define _mm256_permutex_pd(X, M)						\
  ((__m256d) __builtin_ia32_permdf256_mask ((__v4df)(__m256d)(X), (int)(M),	\
					    (__v4df)(__m256d)			\
					    _mm256_undefined_pd (),		\
					    (__mmask8)-1))

#define _mm256_permutex_epi64(X, I)               \
  ((__m256i) __builtin_ia32_permdi256_mask ((__v4di)(__m256i)(X), \
					    (int)(I),		\
					    (__v4di)(__m256i)	\
					    (_mm256_setzero_si256 ()),\
					    (__mmask8) -1))

#define _mm256_maskz_permutex_epi64(M, X, I)                    \
  ((__m256i) __builtin_ia32_permdi256_mask ((__v4di)(__m256i)(X),    \
					    (int)(I),                \
					    (__v4di)(__m256i)        \
					    (_mm256_setzero_si256 ()),\
					    (__mmask8)(M)))

#define _mm256_mask_permutex_epi64(W, M, X, I)               \
  ((__m256i) __builtin_ia32_permdi256_mask ((__v4di)(__m256i)(X), \
					    (int)(I),             \
					    (__v4di)(__m256i)(W), \
					    (__mmask8)(M)))

#define _mm256_insertf32x4(X, Y, C)                                     \
  ((__m256) __builtin_ia32_insertf32x4_256_mask ((__v8sf)(__m256) (X),  \
    (__v4sf)(__m128) (Y), (int) (C),					\
    (__v8sf)(__m256)_mm256_setzero_ps (),				\
    (__mmask8)-1))

#define _mm256_mask_insertf32x4(W, U, X, Y, C)                          \
  ((__m256) __builtin_ia32_insertf32x4_256_mask ((__v8sf)(__m256) (X),  \
    (__v4sf)(__m128) (Y), (int) (C),					\
    (__v8sf)(__m256)(W),						\
    (__mmask8)(U)))

#define _mm256_maskz_insertf32x4(U, X, Y, C)                            \
  ((__m256) __builtin_ia32_insertf32x4_256_mask ((__v8sf)(__m256) (X),	\
    (__v4sf)(__m128) (Y), (int) (C),					\
    (__v8sf)(__m256)_mm256_setzero_ps (),				\
    (__mmask8)(U)))

#define _mm256_inserti32x4(X, Y, C)                                     \
  ((__m256i) __builtin_ia32_inserti32x4_256_mask ((__v8si)(__m256i) (X),\
    (__v4si)(__m128i) (Y), (int) (C),					\
    (__v8si)(__m256i)_mm256_setzero_si256 (),				\
    (__mmask8)-1))

#define _mm256_mask_inserti32x4(W, U, X, Y, C)                          \
  ((__m256i) __builtin_ia32_inserti32x4_256_mask ((__v8si)(__m256i) (X),\
    (__v4si)(__m128i) (Y), (int) (C),					\
    (__v8si)(__m256i)(W),						\
    (__mmask8)(U)))

#define _mm256_maskz_inserti32x4(U, X, Y, C)                            \
  ((__m256i) __builtin_ia32_inserti32x4_256_mask ((__v8si)(__m256i) (X),\
    (__v4si)(__m128i) (Y), (int) (C),					\
    (__v8si)(__m256i)_mm256_setzero_si256 (),				\
    (__mmask8)(U)))

#define _mm256_extractf32x4_ps(X, C)                                    \
  ((__m128) __builtin_ia32_extractf32x4_256_mask ((__v8sf)(__m256) (X), \
    (int) (C),								\
    (__v4sf)(__m128)_mm_setzero_ps (),					\
    (__mmask8)-1))

#define _mm256_mask_extractf32x4_ps(W, U, X, C)                         \
  ((__m128) __builtin_ia32_extractf32x4_256_mask ((__v8sf)(__m256) (X), \
    (int) (C),								\
    (__v4sf)(__m128)(W),						\
    (__mmask8)(U)))

#define _mm256_maskz_extractf32x4_ps(U, X, C)                           \
  ((__m128) __builtin_ia32_extractf32x4_256_mask ((__v8sf)(__m256) (X), \
    (int) (C),								\
    (__v4sf)(__m128)_mm_setzero_ps (),					\
    (__mmask8)(U)))

#define _mm256_extracti32x4_epi32(X, C)                                 \
  ((__m128i) __builtin_ia32_extracti32x4_256_mask ((__v8si)(__m256i) (X),\
    (int) (C), (__v4si)(__m128i)_mm_setzero_si128 (), (__mmask8)-1))

#define _mm256_mask_extracti32x4_epi32(W, U, X, C)                      \
  ((__m128i) __builtin_ia32_extracti32x4_256_mask ((__v8si)(__m256i) (X),\
    (int) (C), (__v4si)(__m128i)(W), (__mmask8)(U)))

#define _mm256_maskz_extracti32x4_epi32(U, X, C)                        \
  ((__m128i) __builtin_ia32_extracti32x4_256_mask ((__v8si)(__m256i) (X),\
    (int) (C), (__v4si)(__m128i)_mm_setzero_si128 (), (__mmask8)(U)))

#define _mm256_shuffle_i64x2(X, Y, C)                                                   \
  ((__m256i)  __builtin_ia32_shuf_i64x2_256_mask ((__v4di)(__m256i)(X),                 \
                                                  (__v4di)(__m256i)(Y), (int)(C),       \
                                                  (__v4di)(__m256i)_mm256_setzero_si256 (), \
                                                  (__mmask8)-1))

#define _mm256_mask_shuffle_i64x2(W, U, X, Y, C)                                        \
  ((__m256i)  __builtin_ia32_shuf_i64x2_256_mask ((__v4di)(__m256i)(X),                 \
                                                  (__v4di)(__m256i)(Y), (int)(C),       \
                                                  (__v4di)(__m256i)(W),\
                                                  (__mmask8)(U)))

#define _mm256_maskz_shuffle_i64x2(U, X, Y, C)                                          \
  ((__m256i)  __builtin_ia32_shuf_i64x2_256_mask ((__v4di)(__m256i)(X),                 \
                                                  (__v4di)(__m256i)(Y), (int)(C),       \
                                                  (__v4di)(__m256i)_mm256_setzero_si256 (), \
                                                  (__mmask8)(U)))

#define _mm256_shuffle_i32x4(X, Y, C)                                                   \
  ((__m256i)  __builtin_ia32_shuf_i32x4_256_mask ((__v8si)(__m256i)(X),                 \
                                                  (__v8si)(__m256i)(Y), (int)(C),       \
						  (__v8si)(__m256i)			\
						  _mm256_setzero_si256 (),		\
                                                  (__mmask8)-1))

#define _mm256_mask_shuffle_i32x4(W, U, X, Y, C)                                        \
  ((__m256i)  __builtin_ia32_shuf_i32x4_256_mask ((__v8si)(__m256i)(X),                 \
                                                  (__v8si)(__m256i)(Y), (int)(C),       \
                                                  (__v8si)(__m256i)(W),                 \
                                                  (__mmask8)(U)))

#define _mm256_maskz_shuffle_i32x4(U, X, Y, C)                                          \
  ((__m256i)  __builtin_ia32_shuf_i32x4_256_mask ((__v8si)(__m256i)(X),                 \
                                                  (__v8si)(__m256i)(Y), (int)(C),       \
						  (__v8si)(__m256i)			\
						  _mm256_setzero_si256 (),		\
                                                  (__mmask8)(U)))

#define _mm256_shuffle_f64x2(X, Y, C)                                                   \
  ((__m256d)  __builtin_ia32_shuf_f64x2_256_mask ((__v4df)(__m256d)(X),                 \
                                                  (__v4df)(__m256d)(Y), (int)(C),       \
						  (__v4df)(__m256d)_mm256_setzero_pd (),\
                                                  (__mmask8)-1))

#define _mm256_mask_shuffle_f64x2(W, U, X, Y, C)                                        \
  ((__m256d)  __builtin_ia32_shuf_f64x2_256_mask ((__v4df)(__m256d)(X),                 \
                                                  (__v4df)(__m256d)(Y), (int)(C),       \
                                                  (__v4df)(__m256d)(W),                 \
                                                  (__mmask8)(U)))

#define _mm256_maskz_shuffle_f64x2(U, X, Y, C)                                          \
  ((__m256d)  __builtin_ia32_shuf_f64x2_256_mask ((__v4df)(__m256d)(X),                 \
                                                  (__v4df)(__m256d)(Y), (int)(C),       \
						  (__v4df)(__m256d)_mm256_setzero_pd( ),\
                                                  (__mmask8)(U)))

#define _mm256_shuffle_f32x4(X, Y, C)                                                   \
  ((__m256)  __builtin_ia32_shuf_f32x4_256_mask ((__v8sf)(__m256)(X),                   \
                                                 (__v8sf)(__m256)(Y), (int)(C),         \
						 (__v8sf)(__m256)_mm256_setzero_ps (),  \
                                                 (__mmask8)-1))

#define _mm256_mask_shuffle_f32x4(W, U, X, Y, C)                                        \
  ((__m256)  __builtin_ia32_shuf_f32x4_256_mask ((__v8sf)(__m256)(X),                   \
                                                 (__v8sf)(__m256)(Y), (int)(C),         \
                                                 (__v8sf)(__m256)(W),                   \
                                                 (__mmask8)(U)))

#define _mm256_maskz_shuffle_f32x4(U, X, Y, C)                                          \
  ((__m256)  __builtin_ia32_shuf_f32x4_256_mask ((__v8sf)(__m256)(X),                   \
                                                 (__v8sf)(__m256)(Y), (int)(C),         \
						 (__v8sf)(__m256)_mm256_setzero_ps (),  \
                                                 (__mmask8)(U)))

#define _mm256_mask_shuffle_pd(W, U, A, B, C)                                   \
  ((__m256d)__builtin_ia32_shufpd256_mask ((__v4df)(__m256d)(A),                \
                                           (__v4df)(__m256d)(B), (int)(C),      \
                                           (__v4df)(__m256d)(W),                \
                                           (__mmask8)(U)))

#define _mm256_maskz_shuffle_pd(U, A, B, C)                                     \
  ((__m256d)__builtin_ia32_shufpd256_mask ((__v4df)(__m256d)(A),                \
                                           (__v4df)(__m256d)(B), (int)(C),      \
					   (__v4df)(__m256d)			\
					   _mm256_setzero_pd (),		\
                                           (__mmask8)(U)))

#define _mm_mask_shuffle_pd(W, U, A, B, C)                                      \
  ((__m128d)__builtin_ia32_shufpd128_mask ((__v2df)(__m128d)(A),                \
                                           (__v2df)(__m128d)(B), (int)(C),      \
                                           (__v2df)(__m128d)(W),                \
                                           (__mmask8)(U)))

#define _mm_maskz_shuffle_pd(U, A, B, C)                                        \
  ((__m128d)__builtin_ia32_shufpd128_mask ((__v2df)(__m128d)(A),                \
                                           (__v2df)(__m128d)(B), (int)(C),      \
					   (__v2df)(__m128d)_mm_setzero_pd (),  \
                                           (__mmask8)(U)))

#define _mm256_mask_shuffle_ps(W, U, A, B, C)                                   \
  ((__m256) __builtin_ia32_shufps256_mask ((__v8sf)(__m256)(A),                 \
                                           (__v8sf)(__m256)(B), (int)(C),       \
                                           (__v8sf)(__m256)(W),                 \
                                           (__mmask8)(U)))

#define _mm256_maskz_shuffle_ps(U, A, B, C)                                     \
  ((__m256) __builtin_ia32_shufps256_mask ((__v8sf)(__m256)(A),                 \
                                           (__v8sf)(__m256)(B), (int)(C),       \
					   (__v8sf)(__m256)_mm256_setzero_ps (),\
                                           (__mmask8)(U)))

#define _mm_mask_shuffle_ps(W, U, A, B, C)                                      \
  ((__m128) __builtin_ia32_shufps128_mask ((__v4sf)(__m128)(A),                 \
                                           (__v4sf)(__m128)(B), (int)(C),       \
                                           (__v4sf)(__m128)(W),                 \
                                           (__mmask8)(U)))

#define _mm_maskz_shuffle_ps(U, A, B, C)                                        \
  ((__m128) __builtin_ia32_shufps128_mask ((__v4sf)(__m128)(A),                 \
                                           (__v4sf)(__m128)(B), (int)(C),       \
					   (__v4sf)(__m128)_mm_setzero_ps (),   \
                                           (__mmask8)(U)))

#define _mm256_fixupimm_pd(X, Y, Z, C)                                          \
  ((__m256d)__builtin_ia32_fixupimmpd256_mask ((__v4df)(__m256d)(X),		\
					       (__v4df)(__m256d)(Y),		\
					       (__v4di)(__m256i)(Z), (int)(C),	\
					       (__mmask8)(-1)))

#define _mm256_mask_fixupimm_pd(X, U, Y, Z, C)                                  \
   ((__m256d)__builtin_ia32_fixupimmpd256_mask ((__v4df)(__m256d)(X),           \
						(__v4df)(__m256d)(Y),           \
						(__v4di)(__m256i)(Z), (int)(C), \
						(__mmask8)(U)))

#define _mm256_maskz_fixupimm_pd(U, X, Y, Z, C)                                 \
   ((__m256d)__builtin_ia32_fixupimmpd256_maskz ((__v4df)(__m256d)(X),          \
						 (__v4df)(__m256d)(Y),          \
						 (__v4di)(__m256i)(Z), (int)(C),\
						 (__mmask8)(U)))

#define _mm256_fixupimm_ps(X, Y, Z, C)						\
  ((__m256)__builtin_ia32_fixupimmps256_mask ((__v8sf)(__m256)(X),		\
					      (__v8sf)(__m256)(Y),		\
					      (__v8si)(__m256i)(Z), (int)(C),	\
					      (__mmask8)(-1)))


#define _mm256_mask_fixupimm_ps(X, U, Y, Z, C)                                  \
    ((__m256)__builtin_ia32_fixupimmps256_mask ((__v8sf)(__m256)(X),            \
						(__v8sf)(__m256)(Y),            \
						(__v8si)(__m256i)(Z), (int)(C), \
						(__mmask8)(U)))

#define _mm256_maskz_fixupimm_ps(U, X, Y, Z, C)                                 \
    ((__m256)__builtin_ia32_fixupimmps256_maskz ((__v8sf)(__m256)(X),           \
						 (__v8sf)(__m256)(Y),           \
						 (__v8si)(__m256i)(Z), (int)(C),\
						 (__mmask8)(U)))

#define _mm_fixupimm_pd(X, Y, Z, C)						\
  ((__m128d)__builtin_ia32_fixupimmpd128_mask ((__v2df)(__m128d)(X),		\
					       (__v2df)(__m128d)(Y),		\
					       (__v2di)(__m128i)(Z), (int)(C), 	\
					       (__mmask8)(-1)))


#define _mm_mask_fixupimm_pd(X, U, Y, Z, C)                                       \
     ((__m128d)__builtin_ia32_fixupimmpd128_mask ((__v2df)(__m128d)(X),           \
						  (__v2df)(__m128d)(Y),           \
						  (__v2di)(__m128i)(Z), (int)(C), \
						  (__mmask8)(U)))

#define _mm_maskz_fixupimm_pd(U, X, Y, Z, C)                                      \
     ((__m128d)__builtin_ia32_fixupimmpd128_maskz ((__v2df)(__m128d)(X),          \
						   (__v2df)(__m128d)(Y),          \
						   (__v2di)(__m128i)(Z), (int)(C),\
						   (__mmask8)(U)))

#define _mm_fixupimm_ps(X, Y, Z, C)						\
   ((__m128)__builtin_ia32_fixupimmps128_mask ((__v4sf)(__m128)(X),		\
					       (__v4sf)(__m128)(Y),		\
					       (__v4si)(__m128i)(Z), (int)(C), 	\
					       (__mmask8)(-1)))

#define _mm_mask_fixupimm_ps(X, U, Y, Z, C)                                      \
      ((__m128)__builtin_ia32_fixupimmps128_mask ((__v4sf)(__m128)(X),           \
						  (__v4sf)(__m128)(Y),           \
						  (__v4si)(__m128i)(Z), (int)(C),\
						  (__mmask8)(U)))

#define _mm_maskz_fixupimm_ps(U, X, Y, Z, C)                                      \
      ((__m128)__builtin_ia32_fixupimmps128_maskz ((__v4sf)(__m128)(X),           \
						   (__v4sf)(__m128)(Y),           \
						   (__v4si)(__m128i)(Z), (int)(C),\
						   (__mmask8)(U)))

#define _mm256_mask_srli_epi32(W, U, A, B)				\
  ((__m256i) __builtin_ia32_psrldi256_mask ((__v8si)(__m256i)(A),	\
    (int)(B), (__v8si)(__m256i)(W), (__mmask8)(U)))

#define _mm256_maskz_srli_epi32(U, A, B)				\
  ((__m256i) __builtin_ia32_psrldi256_mask ((__v8si)(__m256i)(A),	\
    (int)(B), (__v8si)_mm256_setzero_si256 (), (__mmask8)(U)))

#define _mm_mask_srli_epi32(W, U, A, B)                                 \
  ((__m128i) __builtin_ia32_psrldi128_mask ((__v4si)(__m128i)(A),       \
    (int)(B), (__v4si)(__m128i)(W), (__mmask8)(U)))

#define _mm_maskz_srli_epi32(U, A, B)                                   \
  ((__m128i) __builtin_ia32_psrldi128_mask ((__v4si)(__m128i)(A),       \
    (int)(B), (__v4si)_mm_setzero_si128 (), (__mmask8)(U)))

#define _mm256_mask_srli_epi64(W, U, A, B)				\
  ((__m256i) __builtin_ia32_psrlqi256_mask ((__v4di)(__m256i)(A),	\
    (int)(B), (__v4di)(__m256i)(W), (__mmask8)(U)))

#define _mm256_maskz_srli_epi64(U, A, B)				\
  ((__m256i) __builtin_ia32_psrlqi256_mask ((__v4di)(__m256i)(A),	\
    (int)(B), (__v4di)_mm256_setzero_si256 (), (__mmask8)(U)))

#define _mm_mask_srli_epi64(W, U, A, B)                                 \
  ((__m128i) __builtin_ia32_psrlqi128_mask ((__v2di)(__m128i)(A),       \
    (int)(B), (__v2di)(__m128i)(W), (__mmask8)(U)))

#define _mm_maskz_srli_epi64(U, A, B)                                   \
  ((__m128i) __builtin_ia32_psrlqi128_mask ((__v2di)(__m128i)(A),       \
    (int)(B), (__v2di)_mm_setzero_si128 (), (__mmask8)(U)))

#define _mm256_mask_slli_epi32(W, U, X, C)                                \
  ((__m256i)__builtin_ia32_pslldi256_mask ((__v8si)(__m256i)(X), (int)(C),\
    (__v8si)(__m256i)(W),						  \
    (__mmask8)(U)))

#define _mm256_maskz_slli_epi32(U, X, C)                                  \
  ((__m256i)__builtin_ia32_pslldi256_mask ((__v8si)(__m256i)(X), (int)(C),\
    (__v8si)(__m256i)_mm256_setzero_si256 (),				  \
    (__mmask8)(U)))

#define _mm256_mask_slli_epi64(W, U, X, C)                                \
  ((__m256i)__builtin_ia32_psllqi256_mask ((__v4di)(__m256i)(X), (int)(C),\
    (__v4di)(__m256i)(W),						  \
    (__mmask8)(U)))

#define _mm256_maskz_slli_epi64(U, X, C)                                  \
  ((__m256i)__builtin_ia32_psllqi256_mask ((__v4di)(__m256i)(X), (int)(C),\
    (__v4di)(__m256i)_mm256_setzero_si256 (),				  \
    (__mmask8)(U)))

#define _mm_mask_slli_epi32(W, U, X, C)					  \
  ((__m128i)__builtin_ia32_pslldi128_mask ((__v4si)(__m128i)(X), (int)(C),\
    (__v4si)(__m128i)(W),\
    (__mmask8)(U)))

#define _mm_maskz_slli_epi32(U, X, C)					  \
  ((__m128i)__builtin_ia32_pslldi128_mask ((__v4si)(__m128i)(X), (int)(C),\
    (__v4si)(__m128i)_mm_setzero_si128 (),\
    (__mmask8)(U)))

#define _mm_mask_slli_epi64(W, U, X, C)					  \
  ((__m128i)__builtin_ia32_psllqi128_mask ((__v2di)(__m128i)(X), (int)(C),\
    (__v2di)(__m128i)(W),\
    (__mmask8)(U)))

#define _mm_maskz_slli_epi64(U, X, C)					  \
  ((__m128i)__builtin_ia32_psllqi128_mask ((__v2di)(__m128i)(X), (int)(C),\
    (__v2di)(__m128i)_mm_setzero_si128 (),\
    (__mmask8)(U)))

#define _mm256_ternarylogic_epi64(A, B, C, I)                           \
  ((__m256i) __builtin_ia32_pternlogq256_mask ((__v4di)(__m256i)(A),	\
    (__v4di)(__m256i)(B), (__v4di)(__m256i)(C), (int)(I), (__mmask8)-1))

#define _mm256_mask_ternarylogic_epi64(A, U, B, C, I)			\
  ((__m256i) __builtin_ia32_pternlogq256_mask ((__v4di)(__m256i)(A),	\
    (__v4di)(__m256i)(B), (__v4di)(__m256i)(C), (int)(I), (__mmask8)(U)))

#define _mm256_maskz_ternarylogic_epi64(U, A, B, C, I)			\
  ((__m256i) __builtin_ia32_pternlogq256_maskz ((__v4di)(__m256i)(A),	\
    (__v4di)(__m256i)(B), (__v4di)(__m256i)(C), (int)(I), (__mmask8)(U)))

#define _mm256_ternarylogic_epi32(A, B, C, I)                           \
  ((__m256i) __builtin_ia32_pternlogd256_mask ((__v8si)(__m256i)(A),	\
    (__v8si)(__m256i)(B), (__v8si)(__m256i)(C), (int)(I), (__mmask8)-1))

#define _mm256_mask_ternarylogic_epi32(A, U, B, C, I)                   \
  ((__m256i) __builtin_ia32_pternlogd256_mask ((__v8si)(__m256i)(A),	\
    (__v8si)(__m256i)(B), (__v8si)(__m256i)(C), (int)(I), (__mmask8)(U)))

#define _mm256_maskz_ternarylogic_epi32(U, A, B, C, I)			\
  ((__m256i) __builtin_ia32_pternlogd256_maskz ((__v8si)(__m256i)(A),	\
    (__v8si)(__m256i)(B), (__v8si)(__m256i)(C), (int)(I), (__mmask8)(U)))

#define _mm_ternarylogic_epi64(A, B, C, I)                              \
  ((__m128i) __builtin_ia32_pternlogq128_mask ((__v2di)(__m128i)(A),	\
    (__v2di)(__m128i)(B), (__v2di)(__m128i)(C), (int)(I), (__mmask8)-1))

#define _mm_mask_ternarylogic_epi64(A, U, B, C, I)			\
  ((__m128i) __builtin_ia32_pternlogq128_mask ((__v2di)(__m128i)(A),	\
    (__v2di)(__m128i)(B), (__v2di)(__m128i)(C), (int)(I), (__mmask8)(U)))

#define _mm_maskz_ternarylogic_epi64(U, A, B, C, I)			\
  ((__m128i) __builtin_ia32_pternlogq128_maskz ((__v2di)(__m128i)(A),	\
    (__v2di)(__m128i)(B), (__v2di)(__m128i)(C), (int)(I), (__mmask8)(U)))

#define _mm_ternarylogic_epi32(A, B, C, I)                              \
  ((__m128i) __builtin_ia32_pternlogd128_mask ((__v4si)(__m128i)(A),	\
    (__v4si)(__m128i)(B), (__v4si)(__m128i)(C), (int)(I), (__mmask8)-1))

#define _mm_mask_ternarylogic_epi32(A, U, B, C, I)			\
  ((__m128i) __builtin_ia32_pternlogd128_mask ((__v4si)(__m128i)(A),	\
    (__v4si)(__m128i)(B), (__v4si)(__m128i)(C), (int)(I), (__mmask8)(U)))

#define _mm_maskz_ternarylogic_epi32(U, A, B, C, I)			\
  ((__m128i) __builtin_ia32_pternlogd128_maskz ((__v4si)(__m128i)(A),	\
    (__v4si)(__m128i)(B), (__v4si)(__m128i)(C), (int)(I), (__mmask8)(U)))

#define _mm256_roundscale_ps(A, B)				        \
  ((__m256) __builtin_ia32_rndscaleps_256_mask ((__v8sf)(__m256)(A),    \
    (int)(B), (__v8sf)(__m256)_mm256_setzero_ps (), (__mmask8)-1))

#define _mm256_mask_roundscale_ps(W, U, A, B)			        \
  ((__m256) __builtin_ia32_rndscaleps_256_mask ((__v8sf)(__m256)(A),    \
    (int)(B), (__v8sf)(__m256)(W), (__mmask8)(U)))

#define _mm256_maskz_roundscale_ps(U, A, B)			        \
  ((__m256) __builtin_ia32_rndscaleps_256_mask ((__v8sf)(__m256)(A),    \
    (int)(B), (__v8sf)(__m256)_mm256_setzero_ps (), (__mmask8)(U)))

#define _mm256_roundscale_pd(A, B)				        \
  ((__m256d) __builtin_ia32_rndscalepd_256_mask ((__v4df)(__m256d)(A),  \
    (int)(B), (__v4df)(__m256d)_mm256_setzero_pd (), (__mmask8)-1))

#define _mm256_mask_roundscale_pd(W, U, A, B)			        \
  ((__m256d) __builtin_ia32_rndscalepd_256_mask ((__v4df)(__m256d)(A),  \
    (int)(B), (__v4df)(__m256d)(W), (__mmask8)(U)))

#define _mm256_maskz_roundscale_pd(U, A, B)			        \
  ((__m256d) __builtin_ia32_rndscalepd_256_mask ((__v4df)(__m256d)(A),  \
    (int)(B), (__v4df)(__m256d)_mm256_setzero_pd (), (__mmask8)(U)))

#define _mm_roundscale_ps(A, B)					        \
  ((__m128) __builtin_ia32_rndscaleps_128_mask ((__v4sf)(__m128)(A),    \
    (int)(B), (__v4sf)(__m128)_mm_setzero_ps (), (__mmask8)-1))

#define _mm_mask_roundscale_ps(W, U, A, B)			        \
  ((__m128) __builtin_ia32_rndscaleps_128_mask ((__v4sf)(__m128)(A),    \
    (int)(B), (__v4sf)(__m128)(W), (__mmask8)(U)))

#define _mm_maskz_roundscale_ps(U, A, B)			        \
  ((__m128) __builtin_ia32_rndscaleps_128_mask ((__v4sf)(__m128)(A),    \
    (int)(B), (__v4sf)(__m128)_mm_setzero_ps (), (__mmask8)(U)))

#define _mm_roundscale_pd(A, B)					        \
  ((__m128d) __builtin_ia32_rndscalepd_128_mask ((__v2df)(__m128d)(A),  \
    (int)(B), (__v2df)(__m128d)_mm_setzero_pd (), (__mmask8)-1))

#define _mm_mask_roundscale_pd(W, U, A, B)			        \
  ((__m128d) __builtin_ia32_rndscalepd_128_mask ((__v2df)(__m128d)(A),  \
    (int)(B), (__v2df)(__m128d)(W), (__mmask8)(U)))

#define _mm_maskz_roundscale_pd(U, A, B)			        \
  ((__m128d) __builtin_ia32_rndscalepd_128_mask ((__v2df)(__m128d)(A),  \
    (int)(B), (__v2df)(__m128d)_mm_setzero_pd (), (__mmask8)(U)))

#define _mm256_getmant_ps(X, B, C)                                              \
  ((__m256) __builtin_ia32_getmantps256_mask ((__v8sf)(__m256) (X),             \
                                         (int)(((C)<<2) | (B)),                 \
					 (__v8sf)(__m256)_mm256_setzero_ps (),  \
                                         (__mmask8)-1))

#define _mm256_mask_getmant_ps(W, U, X, B, C)                                   \
  ((__m256) __builtin_ia32_getmantps256_mask ((__v8sf)(__m256) (X),             \
                                         (int)(((C)<<2) | (B)),                 \
                                         (__v8sf)(__m256)(W),                   \
                                         (__mmask8)(U)))

#define _mm256_maskz_getmant_ps(U, X, B, C)                                     \
  ((__m256) __builtin_ia32_getmantps256_mask ((__v8sf)(__m256) (X),             \
                                         (int)(((C)<<2) | (B)),                 \
					 (__v8sf)(__m256)_mm256_setzero_ps (),  \
                                         (__mmask8)(U)))

#define _mm_getmant_ps(X, B, C)                                                 \
  ((__m128) __builtin_ia32_getmantps128_mask ((__v4sf)(__m128) (X),             \
                                         (int)(((C)<<2) | (B)),                 \
					 (__v4sf)(__m128)_mm_setzero_ps (),     \
                                         (__mmask8)-1))

#define _mm_mask_getmant_ps(W, U, X, B, C)                                      \
  ((__m128) __builtin_ia32_getmantps128_mask ((__v4sf)(__m128) (X),             \
                                         (int)(((C)<<2) | (B)),                 \
                                         (__v4sf)(__m128)(W),                   \
                                         (__mmask8)(U)))

#define _mm_maskz_getmant_ps(U, X, B, C)                                        \
  ((__m128) __builtin_ia32_getmantps128_mask ((__v4sf)(__m128) (X),             \
                                         (int)(((C)<<2) | (B)),                 \
					 (__v4sf)(__m128)_mm_setzero_ps (),     \
                                         (__mmask8)(U)))

#define _mm256_getmant_pd(X, B, C)                                              \
  ((__m256d) __builtin_ia32_getmantpd256_mask ((__v4df)(__m256d) (X),           \
                                         (int)(((C)<<2) | (B)),                 \
					  (__v4df)(__m256d)_mm256_setzero_pd (),\
                                          (__mmask8)-1))

#define _mm256_mask_getmant_pd(W, U, X, B, C)                                   \
  ((__m256d) __builtin_ia32_getmantpd256_mask ((__v4df)(__m256d) (X),           \
                                         (int)(((C)<<2) | (B)),                 \
                                          (__v4df)(__m256d)(W),                 \
                                          (__mmask8)(U)))

#define _mm256_maskz_getmant_pd(U, X, B, C)                                     \
  ((__m256d) __builtin_ia32_getmantpd256_mask ((__v4df)(__m256d) (X),           \
                                         (int)(((C)<<2) | (B)),                 \
					  (__v4df)(__m256d)_mm256_setzero_pd (),\
                                          (__mmask8)(U)))

#define _mm_getmant_pd(X, B, C)                                                 \
  ((__m128d) __builtin_ia32_getmantpd128_mask ((__v2df)(__m128d) (X),           \
                                         (int)(((C)<<2) | (B)),                 \
					  (__v2df)(__m128d)_mm_setzero_pd (),   \
                                          (__mmask8)-1))

#define _mm_mask_getmant_pd(W, U, X, B, C)                                      \
  ((__m128d) __builtin_ia32_getmantpd128_mask ((__v2df)(__m128d) (X),           \
                                         (int)(((C)<<2) | (B)),                 \
                                          (__v2df)(__m128d)(W),                 \
                                          (__mmask8)(U)))

#define _mm_maskz_getmant_pd(U, X, B, C)                                        \
  ((__m128d) __builtin_ia32_getmantpd128_mask ((__v2df)(__m128d) (X),           \
                                         (int)(((C)<<2) | (B)),                 \
					  (__v2df)(__m128d)_mm_setzero_pd (),   \
                                          (__mmask8)(U)))

#define _mm256_mmask_i32gather_ps(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256) __builtin_ia32_gather3siv8sf ((__v8sf)(__m256) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v8si)(__m256i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm_mmask_i32gather_ps(V1OLD, MASK, INDEX, ADDR, SCALE)		\
  (__m128) __builtin_ia32_gather3siv4sf ((__v4sf)(__m128) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v4si)(__m128i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm256_mmask_i32gather_pd(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256d) __builtin_ia32_gather3siv4df ((__v4df)(__m256d) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4si)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm_mmask_i32gather_pd(V1OLD, MASK, INDEX, ADDR, SCALE)		\
  (__m128d) __builtin_ia32_gather3siv2df ((__v2df)(__m128d) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4si)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm256_mmask_i64gather_ps(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m128) __builtin_ia32_gather3div8sf ((__v4sf)(__m128) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v4di)(__m256i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm_mmask_i64gather_ps(V1OLD, MASK, INDEX, ADDR, SCALE)		\
  (__m128) __builtin_ia32_gather3div4sf ((__v4sf)(__m128) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v2di)(__m128i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm256_mmask_i64gather_pd(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256d) __builtin_ia32_gather3div4df ((__v4df)(__m256d) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4di)(__m256i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm_mmask_i64gather_pd(V1OLD, MASK, INDEX, ADDR, SCALE)		\
  (__m128d) __builtin_ia32_gather3div2df ((__v2df)(__m128d) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v2di)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm256_mmask_i32gather_epi32(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256i) __builtin_ia32_gather3siv8si ((__v8si)(__m256i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v8si)(__m256i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm_mmask_i32gather_epi32(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m128i) __builtin_ia32_gather3siv4si ((__v4si)(__m128i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4si)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm256_mmask_i32gather_epi64(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256i) __builtin_ia32_gather3siv4di ((__v4di)(__m256i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4si)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm_mmask_i32gather_epi64(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m128i) __builtin_ia32_gather3siv2di ((__v2di)(__m128i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4si)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm256_mmask_i64gather_epi32(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m128i) __builtin_ia32_gather3div8si ((__v4si)(__m128i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4di)(__m256i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm_mmask_i64gather_epi32(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m128i) __builtin_ia32_gather3div4si ((__v4si)(__m128i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v2di)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm256_mmask_i64gather_epi64(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256i) __builtin_ia32_gather3div4di ((__v4di)(__m256i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v4di)(__m256i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm_mmask_i64gather_epi64(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m128i) __builtin_ia32_gather3div2di ((__v2di)(__m128i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v2di)(__m128i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm256_i32scatter_ps(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv8sf ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8sf)(__m256) (V1), (int) (SCALE))

#define _mm256_mask_i32scatter_ps(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv8sf ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8sf)(__m256) (V1), (int) (SCALE))

#define _mm_i32scatter_ps(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv4sf ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4sf)(__m128) (V1), (int) (SCALE))

#define _mm_mask_i32scatter_ps(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv4sf ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4sf)(__m128) (V1), (int) (SCALE))

#define _mm256_i32scatter_pd(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv4df ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4df)(__m256d) (V1), (int) (SCALE))

#define _mm256_mask_i32scatter_pd(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv4df ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4df)(__m256d) (V1), (int) (SCALE))

#define _mm_i32scatter_pd(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv2df ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4si)(__m128i) (INDEX),		\
				(__v2df)(__m128d) (V1), (int) (SCALE))

#define _mm_mask_i32scatter_pd(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv2df ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4si)(__m128i) (INDEX),		\
				(__v2df)(__m128d) (V1), (int) (SCALE))

#define _mm256_i64scatter_ps(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv8sf ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4sf)(__m128) (V1), (int) (SCALE))

#define _mm256_mask_i64scatter_ps(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv8sf ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4sf)(__m128) (V1), (int) (SCALE))

#define _mm_i64scatter_ps(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv4sf ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v2di)(__m128i) (INDEX),		\
				(__v4sf)(__m128) (V1), (int) (SCALE))

#define _mm_mask_i64scatter_ps(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv4sf ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v2di)(__m128i) (INDEX),		\
				(__v4sf)(__m128) (V1), (int) (SCALE))

#define _mm256_i64scatter_pd(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv4df ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4df)(__m256d) (V1), (int) (SCALE))

#define _mm256_mask_i64scatter_pd(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv4df ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4df)(__m256d) (V1), (int) (SCALE))

#define _mm_i64scatter_pd(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv2df ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v2di)(__m128i) (INDEX),		\
				(__v2df)(__m128d) (V1), (int) (SCALE))

#define _mm_mask_i64scatter_pd(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv2df ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v2di)(__m128i) (INDEX),		\
				(__v2df)(__m128d) (V1), (int) (SCALE))

#define _mm256_i32scatter_epi32(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv8si ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8si)(__m256i) (V1), (int) (SCALE))

#define _mm256_mask_i32scatter_epi32(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scattersiv8si ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8si)(__m256i) (V1), (int) (SCALE))

#define _mm_i32scatter_epi32(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv4si ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4si)(__m128i) (V1), (int) (SCALE))

#define _mm_mask_i32scatter_epi32(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv4si ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4si)(__m128i) (V1), (int) (SCALE))

#define _mm256_i32scatter_epi64(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv4di ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4di)(__m256i) (V1), (int) (SCALE))

#define _mm256_mask_i32scatter_epi64(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scattersiv4di ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4si)(__m128i) (INDEX),		\
				(__v4di)(__m256i) (V1), (int) (SCALE))

#define _mm_i32scatter_epi64(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv2di ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4si)(__m128i) (INDEX),		\
				(__v2di)(__m128i) (V1), (int) (SCALE))

#define _mm_mask_i32scatter_epi64(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv2di ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4si)(__m128i) (INDEX),		\
				(__v2di)(__m128i) (V1), (int) (SCALE))

#define _mm256_i64scatter_epi32(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv8si ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4si)(__m128i) (V1), (int) (SCALE))

#define _mm256_mask_i64scatter_epi32(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scatterdiv8si ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4si)(__m128i) (V1), (int) (SCALE))

#define _mm_i64scatter_epi32(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv4si ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v2di)(__m128i) (INDEX),		\
				(__v4si)(__m128i) (V1), (int) (SCALE))

#define _mm_mask_i64scatter_epi32(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv4si ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v2di)(__m128i) (INDEX),		\
				(__v4si)(__m128i) (V1), (int) (SCALE))

#define _mm256_i64scatter_epi64(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv4di ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4di)(__m256i) (V1), (int) (SCALE))

#define _mm256_mask_i64scatter_epi64(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scatterdiv4di ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v4di)(__m256i) (INDEX),		\
				(__v4di)(__m256i) (V1), (int) (SCALE))

#define _mm_i64scatter_epi64(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv2di ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v2di)(__m128i) (INDEX),		\
				(__v2di)(__m128i) (V1), (int) (SCALE))

#define _mm_mask_i64scatter_epi64(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv2di ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v2di)(__m128i) (INDEX),		\
				(__v2di)(__m128i) (V1), (int) (SCALE))

#define _mm256_mask_shuffle_epi32(W, U, X, C)                                       \
  ((__m256i)  __builtin_ia32_pshufd256_mask ((__v8si)(__m256i)(X), (int)(C),        \
                                             (__v8si)(__m256i)(W),                  \
                                             (__mmask8)(U)))

#define _mm256_maskz_shuffle_epi32(U, X, C)                                         \
  ((__m256i)  __builtin_ia32_pshufd256_mask ((__v8si)(__m256i)(X), (int)(C),        \
					     (__v8si)(__m256i)			    \
					     _mm256_setzero_si256 (),		    \
                                             (__mmask8)(U)))

#define _mm_mask_shuffle_epi32(W, U, X, C)                                          \
  ((__m128i)  __builtin_ia32_pshufd128_mask ((__v4si)(__m128i)(X), (int)(C),        \
                                             (__v4si)(__m128i)(W),                  \
                                             (__mmask8)(U)))

#define _mm_maskz_shuffle_epi32(U, X, C)                                            \
  ((__m128i)  __builtin_ia32_pshufd128_mask ((__v4si)(__m128i)(X), (int)(C),        \
					     (__v4si)(__m128i)_mm_setzero_si128 (), \
                                             (__mmask8)(U)))

#define _mm256_rol_epi64(A, B)                                                 \
  ((__m256i)__builtin_ia32_prolq256_mask ((__v4di)(__m256i)(A), (int)(B),      \
                                          (__v4di)(__m256i)_mm256_setzero_si256 (),\
                                          (__mmask8)-1))

#define _mm256_mask_rol_epi64(W, U, A, B)                                      \
  ((__m256i)__builtin_ia32_prolq256_mask ((__v4di)(__m256i)(A), (int)(B),      \
                                          (__v4di)(__m256i)(W),                \
                                          (__mmask8)(U)))

#define _mm256_maskz_rol_epi64(U, A, B)                                        \
  ((__m256i)__builtin_ia32_prolq256_mask ((__v4di)(__m256i)(A), (int)(B),      \
                                          (__v4di)(__m256i)_mm256_setzero_si256 (),\
                                          (__mmask8)(U)))

#define _mm_rol_epi64(A, B)                                                    \
  ((__m128i)__builtin_ia32_prolq128_mask ((__v2di)(__m128i)(A), (int)(B),      \
					  (__v2di)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)-1))

#define _mm_mask_rol_epi64(W, U, A, B)                                         \
  ((__m128i)__builtin_ia32_prolq128_mask ((__v2di)(__m128i)(A), (int)(B),      \
                                          (__v2di)(__m128i)(W),                \
                                          (__mmask8)(U)))

#define _mm_maskz_rol_epi64(U, A, B)                                           \
  ((__m128i)__builtin_ia32_prolq128_mask ((__v2di)(__m128i)(A), (int)(B),      \
					  (__v2di)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)(U)))

#define _mm256_ror_epi64(A, B)                                                 \
  ((__m256i)__builtin_ia32_prorq256_mask ((__v4di)(__m256i)(A), (int)(B),      \
                                          (__v4di)(__m256i)_mm256_setzero_si256 (),\
                                          (__mmask8)-1))

#define _mm256_mask_ror_epi64(W, U, A, B)                                      \
  ((__m256i)__builtin_ia32_prorq256_mask ((__v4di)(__m256i)(A), (int)(B),      \
                                          (__v4di)(__m256i)(W),                \
                                          (__mmask8)(U)))

#define _mm256_maskz_ror_epi64(U, A, B)                                        \
  ((__m256i)__builtin_ia32_prorq256_mask ((__v4di)(__m256i)(A), (int)(B),      \
                                          (__v4di)(__m256i)_mm256_setzero_si256 (),\
                                          (__mmask8)(U)))

#define _mm_ror_epi64(A, B)                                                    \
  ((__m128i)__builtin_ia32_prorq128_mask ((__v2di)(__m128i)(A), (int)(B),      \
					  (__v2di)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)-1))

#define _mm_mask_ror_epi64(W, U, A, B)                                         \
  ((__m128i)__builtin_ia32_prorq128_mask ((__v2di)(__m128i)(A), (int)(B),      \
                                          (__v2di)(__m128i)(W),                \
                                          (__mmask8)(U)))

#define _mm_maskz_ror_epi64(U, A, B)                                           \
  ((__m128i)__builtin_ia32_prorq128_mask ((__v2di)(__m128i)(A), (int)(B),      \
					  (__v2di)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)(U)))

#define _mm256_rol_epi32(A, B)                                                 \
  ((__m256i)__builtin_ia32_prold256_mask ((__v8si)(__m256i)(A), (int)(B),      \
					  (__v8si)(__m256i)_mm256_setzero_si256 (),\
                                          (__mmask8)-1))

#define _mm256_mask_rol_epi32(W, U, A, B)                                      \
  ((__m256i)__builtin_ia32_prold256_mask ((__v8si)(__m256i)(A), (int)(B),      \
                                          (__v8si)(__m256i)(W),                \
                                          (__mmask8)(U)))

#define _mm256_maskz_rol_epi32(U, A, B)                                        \
  ((__m256i)__builtin_ia32_prold256_mask ((__v8si)(__m256i)(A), (int)(B),      \
					  (__v8si)(__m256i)_mm256_setzero_si256 (),\
                                          (__mmask8)(U)))

#define _mm_rol_epi32(A, B)                                                    \
  ((__m128i)__builtin_ia32_prold128_mask ((__v4si)(__m128i)(A), (int)(B),      \
					  (__v4si)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)-1))

#define _mm_mask_rol_epi32(W, U, A, B)                                         \
  ((__m128i)__builtin_ia32_prold128_mask ((__v4si)(__m128i)(A), (int)(B),      \
                                          (__v4si)(__m128i)(W),                \
                                          (__mmask8)(U)))

#define _mm_maskz_rol_epi32(U, A, B)                                           \
  ((__m128i)__builtin_ia32_prold128_mask ((__v4si)(__m128i)(A), (int)(B),      \
					  (__v4si)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)(U)))

#define _mm256_ror_epi32(A, B)                                                 \
  ((__m256i)__builtin_ia32_prord256_mask ((__v8si)(__m256i)(A), (int)(B),      \
					  (__v8si)(__m256i)_mm256_setzero_si256 (),\
                                          (__mmask8)-1))

#define _mm256_mask_ror_epi32(W, U, A, B)                                      \
  ((__m256i)__builtin_ia32_prord256_mask ((__v8si)(__m256i)(A), (int)(B),      \
                                          (__v8si)(__m256i)(W),                \
                                          (__mmask8)(U)))

#define _mm256_maskz_ror_epi32(U, A, B)                                        \
  ((__m256i)__builtin_ia32_prord256_mask ((__v8si)(__m256i)(A), (int)(B),      \
					  (__v8si)(__m256i)		       \
					  _mm256_setzero_si256 (),	       \
                                          (__mmask8)(U)))

#define _mm_ror_epi32(A, B)                                                    \
  ((__m128i)__builtin_ia32_prord128_mask ((__v4si)(__m128i)(A), (int)(B),      \
					  (__v4si)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)-1))

#define _mm_mask_ror_epi32(W, U, A, B)                                         \
  ((__m128i)__builtin_ia32_prord128_mask ((__v4si)(__m128i)(A), (int)(B),      \
                                          (__v4si)(__m128i)(W),                \
                                          (__mmask8)(U)))

#define _mm_maskz_ror_epi32(U, A, B)                                           \
  ((__m128i)__builtin_ia32_prord128_mask ((__v4si)(__m128i)(A), (int)(B),      \
					  (__v4si)(__m128i)_mm_setzero_si128 (),\
                                          (__mmask8)(U)))

#define _mm256_alignr_epi32(X, Y, C)                                        \
    ((__m256i)__builtin_ia32_alignd256_mask ((__v8si)(__m256i)(X),          \
        (__v8si)(__m256i)(Y), (int)(C), (__v8si)(__m256i)(X), (__mmask8)-1))

#define _mm256_mask_alignr_epi32(W, U, X, Y, C)                             \
    ((__m256i)__builtin_ia32_alignd256_mask ((__v8si)(__m256i)(X),          \
        (__v8si)(__m256i)(Y), (int)(C), (__v8si)(__m256i)(W), (__mmask8)(U)))

#define _mm256_maskz_alignr_epi32(U, X, Y, C)                               \
    ((__m256i)__builtin_ia32_alignd256_mask ((__v8si)(__m256i)(X),          \
        (__v8si)(__m256i)(Y), (int)(C), (__v8si)(__m256i)_mm256_setzero_si256 (),\
        (__mmask8)(U)))

#define _mm256_alignr_epi64(X, Y, C)                                        \
    ((__m256i)__builtin_ia32_alignq256_mask ((__v4di)(__m256i)(X),          \
        (__v4di)(__m256i)(Y), (int)(C), (__v4di)(__m256i)(X), (__mmask8)-1))

#define _mm256_mask_alignr_epi64(W, U, X, Y, C)                             \
    ((__m256i)__builtin_ia32_alignq256_mask ((__v4di)(__m256i)(X),          \
        (__v4di)(__m256i)(Y), (int)(C), (__v4di)(__m256i)(W), (__mmask8)(U)))

#define _mm256_maskz_alignr_epi64(U, X, Y, C)                               \
    ((__m256i)__builtin_ia32_alignq256_mask ((__v4di)(__m256i)(X),          \
        (__v4di)(__m256i)(Y), (int)(C), (__v4di)(__m256i)_mm256_setzero_si256 (),\
        (__mmask8)(U)))

#define _mm_alignr_epi32(X, Y, C)                                           \
    ((__m128i)__builtin_ia32_alignd128_mask ((__v4si)(__m128i)(X),          \
        (__v4si)(__m128i)(Y), (int)(C), (__v4si)(__m128i)(X), (__mmask8)-1))

#define _mm_mask_alignr_epi32(W, U, X, Y, C)                                \
    ((__m128i)__builtin_ia32_alignd128_mask ((__v4si)(__m128i)(X),          \
        (__v4si)(__m128i)(Y), (int)(C), (__v4si)(__m128i)(W), (__mmask8)(U)))

#define _mm_maskz_alignr_epi32(U, X, Y, C)                                  \
    ((__m128i)__builtin_ia32_alignd128_mask ((__v4si)(__m128i)(X),          \
	(__v4si)(__m128i)(Y), (int)(C), (__v4si)(__m128i)_mm_setzero_si128 (),\
        (__mmask8)(U)))

#define _mm_alignr_epi64(X, Y, C)                                           \
    ((__m128i)__builtin_ia32_alignq128_mask ((__v2di)(__m128i)(X),          \
        (__v2di)(__m128i)(Y), (int)(C), (__v2di)(__m128i)(X), (__mmask8)-1))

#define _mm_mask_alignr_epi64(W, U, X, Y, C)                                \
    ((__m128i)__builtin_ia32_alignq128_mask ((__v2di)(__m128i)(X),          \
        (__v2di)(__m128i)(Y), (int)(C), (__v2di)(__m128i)(X), (__mmask8)-1))

#define _mm_maskz_alignr_epi64(U, X, Y, C)                                  \
    ((__m128i)__builtin_ia32_alignq128_mask ((__v2di)(__m128i)(X),          \
	(__v2di)(__m128i)(Y), (int)(C), (__v2di)(__m128i)_mm_setzero_si128 (),\
        (__mmask8)(U)))

#define _mm_mask_cvtps_ph(W, U, A, I)						\
  ((__m128i) __builtin_ia32_vcvtps2ph_mask ((__v4sf)(__m128) (A), (int) (I),	\
      (__v8hi)(__m128i) (W), (__mmask8) (U)))

#define _mm_maskz_cvtps_ph(U, A, I)						\
  ((__m128i) __builtin_ia32_vcvtps2ph_mask ((__v4sf)(__m128) (A), (int) (I),	\
      (__v8hi)(__m128i) _mm_setzero_si128 (), (__mmask8) (U)))

#define _mm256_mask_cvtps_ph(W, U, A, I)					\
  ((__m128i) __builtin_ia32_vcvtps2ph256_mask ((__v8sf)(__m256) (A), (int) (I),	\
      (__v8hi)(__m128i) (W), (__mmask8) (U)))

#define _mm256_maskz_cvtps_ph(U, A, I)						\
  ((__m128i) __builtin_ia32_vcvtps2ph256_mask ((__v8sf)(__m256) (A), (int) (I),	\
      (__v8hi)(__m128i) _mm_setzero_si128 (), (__mmask8) (U)))

#define _mm256_mask_srai_epi32(W, U, A, B)				\
  ((__m256i) __builtin_ia32_psradi256_mask ((__v8si)(__m256i)(A),	\
    (int)(B), (__v8si)(__m256i)(W), (__mmask8)(U)))

#define _mm256_maskz_srai_epi32(U, A, B)				\
  ((__m256i) __builtin_ia32_psradi256_mask ((__v8si)(__m256i)(A),	\
    (int)(B), (__v8si)_mm256_setzero_si256 (), (__mmask8)(U)))

#define _mm_mask_srai_epi32(W, U, A, B)                                 \
  ((__m128i) __builtin_ia32_psradi128_mask ((__v4si)(__m128i)(A),       \
    (int)(B), (__v4si)(__m128i)(W), (__mmask8)(U)))

#define _mm_maskz_srai_epi32(U, A, B)                                   \
  ((__m128i) __builtin_ia32_psradi128_mask ((__v4si)(__m128i)(A),       \
    (int)(B), (__v4si)_mm_setzero_si128 (), (__mmask8)(U)))

#define _mm256_srai_epi64(A, B)						\
  ((__m256i) __builtin_ia32_psraqi256_mask ((__v4di)(__m256i)(A),	\
    (int)(B), (__v4di)_mm256_setzero_si256 (), (__mmask8)-1))

#define _mm256_mask_srai_epi64(W, U, A, B)				\
  ((__m256i) __builtin_ia32_psraqi256_mask ((__v4di)(__m256i)(A),	\
    (int)(B), (__v4di)(__m256i)(W), (__mmask8)(U)))

#define _mm256_maskz_srai_epi64(U, A, B)				\
  ((__m256i) __builtin_ia32_psraqi256_mask ((__v4di)(__m256i)(A),	\
    (int)(B), (__v4di)_mm256_setzero_si256 (), (__mmask8)(U)))

#define _mm_srai_epi64(A, B)						\
  ((__m128i) __builtin_ia32_psraqi128_mask ((__v2di)(__m128i)(A),       \
    (int)(B), (__v2di)_mm_setzero_si128 (), (__mmask8)-1))

#define _mm_mask_srai_epi64(W, U, A, B)                                 \
  ((__m128i) __builtin_ia32_psraqi128_mask ((__v2di)(__m128i)(A),       \
    (int)(B), (__v2di)(__m128i)(W), (__mmask8)(U)))

#define _mm_maskz_srai_epi64(U, A, B)                                   \
  ((__m128i) __builtin_ia32_psraqi128_mask ((__v2di)(__m128i)(A),       \
    (int)(B), (__v2di)_mm_setzero_si128 (), (__mmask8)(U)))

#define _mm256_mask_permutex_pd(W, U, A, B)                             \
  ((__m256d) __builtin_ia32_permdf256_mask ((__v4df)(__m256d)(A),       \
    (int)(B), (__v4df)(__m256d)(W), (__mmask8)(U)))

#define _mm256_maskz_permutex_pd(U, A, B)				\
  ((__m256d) __builtin_ia32_permdf256_mask ((__v4df)(__m256d)(A),       \
    (int)(B), (__v4df)(__m256d)_mm256_setzero_pd (), (__mmask8)(U)))

#define _mm256_mask_permute_pd(W, U, X, C)					    \
  ((__m256d) __builtin_ia32_vpermilpd256_mask ((__v4df)(__m256d)(X), (int)(C),	    \
					      (__v4df)(__m256d)(W),		    \
					      (__mmask8)(U)))

#define _mm256_maskz_permute_pd(U, X, C)					    \
  ((__m256d) __builtin_ia32_vpermilpd256_mask ((__v4df)(__m256d)(X), (int)(C),	    \
					      (__v4df)(__m256d)_mm256_setzero_pd (),\
					      (__mmask8)(U)))

#define _mm256_mask_permute_ps(W, U, X, C)					    \
  ((__m256) __builtin_ia32_vpermilps256_mask ((__v8sf)(__m256)(X), (int)(C),	    \
					      (__v8sf)(__m256)(W), (__mmask8)(U)))

#define _mm256_maskz_permute_ps(U, X, C)					    \
  ((__m256) __builtin_ia32_vpermilps256_mask ((__v8sf)(__m256)(X), (int)(C),	    \
					      (__v8sf)(__m256)_mm256_setzero_ps (), \
					      (__mmask8)(U)))

#define _mm_mask_permute_pd(W, U, X, C)						    \
  ((__m128d) __builtin_ia32_vpermilpd_mask ((__v2df)(__m128d)(X), (int)(C),	    \
					    (__v2df)(__m128d)(W), (__mmask8)(U)))

#define _mm_maskz_permute_pd(U, X, C)						    \
  ((__m128d) __builtin_ia32_vpermilpd_mask ((__v2df)(__m128d)(X), (int)(C),	    \
					    (__v2df)(__m128d)_mm_setzero_pd (),	    \
					    (__mmask8)(U)))

#define _mm_mask_permute_ps(W, U, X, C)						    \
  ((__m128) __builtin_ia32_vpermilps_mask ((__v4sf)(__m128)(X), (int)(C),	    \
					  (__v4sf)(__m128)(W), (__mmask8)(U)))

#define _mm_maskz_permute_ps(U, X, C)						    \
  ((__m128) __builtin_ia32_vpermilps_mask ((__v4sf)(__m128)(X), (int)(C),	    \
					  (__v4sf)(__m128)_mm_setzero_ps (),	    \
					  (__mmask8)(U)))

#define _mm256_mask_blend_pd(__U, __A, __W)			      \
  ((__m256d) __builtin_ia32_blendmpd_256_mask ((__v4df) (__A),	      \
						     (__v4df) (__W),  \
						     (__mmask8) (__U)))

#define _mm256_mask_blend_ps(__U, __A, __W)			      \
  ((__m256) __builtin_ia32_blendmps_256_mask ((__v8sf) (__A),	      \
						    (__v8sf) (__W),   \
						    (__mmask8) (__U)))

#define _mm256_mask_blend_epi64(__U, __A, __W)			      \
  ((__m256i) __builtin_ia32_blendmq_256_mask ((__v4di) (__A),	      \
						    (__v4di) (__W),   \
						    (__mmask8) (__U)))

#define _mm256_mask_blend_epi32(__U, __A, __W)			      \
  ((__m256i) __builtin_ia32_blendmd_256_mask ((__v8si) (__A),	      \
						    (__v8si) (__W),   \
						    (__mmask8) (__U)))

#define _mm_mask_blend_pd(__U, __A, __W)			      \
  ((__m128d) __builtin_ia32_blendmpd_128_mask ((__v2df) (__A),	      \
						     (__v2df) (__W),  \
						     (__mmask8) (__U)))

#define _mm_mask_blend_ps(__U, __A, __W)			      \
  ((__m128) __builtin_ia32_blendmps_128_mask ((__v4sf) (__A),	      \
						    (__v4sf) (__W),   \
						    (__mmask8) (__U)))

#define _mm_mask_blend_epi64(__U, __A, __W)			      \
  ((__m128i) __builtin_ia32_blendmq_128_mask ((__v2di) (__A),	      \
						    (__v2di) (__W),   \
						    (__mmask8) (__U)))

#define _mm_mask_blend_epi32(__U, __A, __W)			      \
  ((__m128i) __builtin_ia32_blendmd_128_mask ((__v4si) (__A),	      \
						    (__v4si) (__W),   \
						    (__mmask8) (__U)))

#define _mm256_cmp_epu32_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si)(__m256i)(X),	\
					    (__v8si)(__m256i)(Y), (int)(P),\
					    (__mmask8)-1))

#define _mm256_cmp_epi64_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpq256_mask ((__v4di)(__m256i)(X),	\
					   (__v4di)(__m256i)(Y), (int)(P),\
					   (__mmask8)-1))

#define _mm256_cmp_epi32_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpd256_mask ((__v8si)(__m256i)(X),	\
					   (__v8si)(__m256i)(Y), (int)(P),\
					   (__mmask8)-1))

#define _mm256_cmp_epu64_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di)(__m256i)(X),	\
					    (__v4di)(__m256i)(Y), (int)(P),\
					    (__mmask8)-1))

#define _mm256_cmp_pd_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmppd256_mask ((__v4df)(__m256d)(X),	\
					    (__v4df)(__m256d)(Y), (int)(P),\
					    (__mmask8)-1))

#define _mm256_cmp_ps_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpps256_mask ((__v8sf)(__m256)(X),	\
					     (__v8sf)(__m256)(Y), (int)(P),\
					     (__mmask8)-1))

#define _mm256_mask_cmp_epi64_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpq256_mask ((__v4di)(__m256i)(X),	\
					   (__v4di)(__m256i)(Y), (int)(P),\
					   (__mmask8)(M)))

#define _mm256_mask_cmp_epi32_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpd256_mask ((__v8si)(__m256i)(X),	\
					   (__v8si)(__m256i)(Y), (int)(P),\
					   (__mmask8)(M)))

#define _mm256_mask_cmp_epu64_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_ucmpq256_mask ((__v4di)(__m256i)(X),	\
					    (__v4di)(__m256i)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm256_mask_cmp_epu32_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_ucmpd256_mask ((__v8si)(__m256i)(X),	\
					    (__v8si)(__m256i)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm256_mask_cmp_pd_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmppd256_mask ((__v4df)(__m256d)(X),	\
					    (__v4df)(__m256d)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm256_mask_cmp_ps_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpps256_mask ((__v8sf)(__m256)(X),	\
					     (__v8sf)(__m256)(Y), (int)(P),\
					     (__mmask8)(M)))

#define _mm_cmp_epi64_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpq128_mask ((__v2di)(__m128i)(X),	\
					   (__v2di)(__m128i)(Y), (int)(P),\
					   (__mmask8)-1))

#define _mm_cmp_epi32_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpd128_mask ((__v4si)(__m128i)(X),	\
					   (__v4si)(__m128i)(Y), (int)(P),\
					   (__mmask8)-1))

#define _mm_cmp_epu64_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di)(__m128i)(X),	\
					    (__v2di)(__m128i)(Y), (int)(P),\
					    (__mmask8)-1))

#define _mm_cmp_epu32_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si)(__m128i)(X),	\
					    (__v4si)(__m128i)(Y), (int)(P),\
					    (__mmask8)-1))

#define _mm_cmp_pd_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmppd128_mask ((__v2df)(__m128d)(X),	\
					    (__v2df)(__m128d)(Y), (int)(P),\
					    (__mmask8)-1))

#define _mm_cmp_ps_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpps128_mask ((__v4sf)(__m128)(X),	\
					     (__v4sf)(__m128)(Y), (int)(P),\
					     (__mmask8)-1))

#define _mm_mask_cmp_epi64_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpq128_mask ((__v2di)(__m128i)(X),	\
					   (__v2di)(__m128i)(Y), (int)(P),\
					   (__mmask8)(M)))

#define _mm_mask_cmp_epi32_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpd128_mask ((__v4si)(__m128i)(X),	\
					   (__v4si)(__m128i)(Y), (int)(P),\
					   (__mmask8)(M)))

#define _mm_mask_cmp_epu64_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_ucmpq128_mask ((__v2di)(__m128i)(X),	\
					    (__v2di)(__m128i)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm_mask_cmp_epu32_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_ucmpd128_mask ((__v4si)(__m128i)(X),	\
					    (__v4si)(__m128i)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm_mask_cmp_pd_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmppd128_mask ((__v2df)(__m128d)(X),	\
					    (__v2df)(__m128d)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm_mask_cmp_ps_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpps128_mask ((__v4sf)(__m128)(X),	\
					     (__v4sf)(__m128)(Y), (int)(P),\
					     (__mmask8)(M)))

#endif

#define _mm256_permutexvar_ps(A, B)	_mm256_permutevar8x32_ps ((B), (A))

#ifdef __DISABLE_AVX512VL__
#undef __DISABLE_AVX512VL__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VL__ */

#endif /* _AVX512VLINTRIN_H_INCLUDED */
