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
#error "Never use <avx512vldqintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512VLDQINTRIN_H_INCLUDED
#define _AVX512VLDQINTRIN_H_INCLUDED

#if !defined(__AVX512VL__) || !defined(__AVX512DQ__)
#pragma GCC push_options
#pragma GCC target("avx512vl,avx512dq")
#define __DISABLE_AVX512VLDQ__
#endif /* __AVX512VLDQ__ */

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttpd_epi64 (__m256d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2qq256_mask ((__v4df) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttpd_epi64 (__m256i __W, __mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2qq256_mask ((__v4df) __A,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttpd_epi64 (__mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2qq256_mask ((__v4df) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttpd_epi64 (__m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2qq128_mask ((__v2df) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttpd_epi64 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2qq128_mask ((__v2df) __A,
						     (__v2di) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttpd_epi64 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2qq128_mask ((__v2df) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttpd_epu64 (__m256d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2uqq256_mask ((__v4df) __A,
						      (__v4di)
						      _mm256_setzero_si256 (),
						      (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttpd_epu64 (__m256i __W, __mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2uqq256_mask ((__v4df) __A,
						      (__v4di) __W,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttpd_epu64 (__mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2uqq256_mask ((__v4df) __A,
						      (__v4di)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttpd_epu64 (__m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2uqq128_mask ((__v2df) __A,
						      (__v2di)
						      _mm_setzero_si128 (),
						      (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttpd_epu64 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2uqq128_mask ((__v2df) __A,
						      (__v2di) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttpd_epu64 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvttpd2uqq128_mask ((__v2df) __A,
						      (__v2di)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtpd_epi64 (__m256d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2qq256_mask ((__v4df) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtpd_epi64 (__m256i __W, __mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2qq256_mask ((__v4df) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtpd_epi64 (__mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2qq256_mask ((__v4df) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtpd_epi64 (__m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2qq128_mask ((__v2df) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtpd_epi64 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2qq128_mask ((__v2df) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtpd_epi64 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2qq128_mask ((__v2df) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtpd_epu64 (__m256d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2uqq256_mask ((__v4df) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtpd_epu64 (__m256i __W, __mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2uqq256_mask ((__v4df) __A,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtpd_epu64 (__mmask8 __U, __m256d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2uqq256_mask ((__v4df) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtpd_epu64 (__m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2uqq128_mask ((__v2df) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtpd_epu64 (__m128i __W, __mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2uqq128_mask ((__v2df) __A,
						     (__v2di) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtpd_epu64 (__mmask8 __U, __m128d __A)
{
  return (__m128i) __builtin_ia32_cvtpd2uqq128_mask ((__v2df) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttps_epi64 (__m128 __A)
{
  return (__m256i) __builtin_ia32_cvttps2qq256_mask ((__v4sf) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttps_epi64 (__m256i __W, __mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvttps2qq256_mask ((__v4sf) __A,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttps_epi64 (__mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvttps2qq256_mask ((__v4sf) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttps_epi64 (__m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2qq128_mask ((__v4sf) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttps_epi64 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2qq128_mask ((__v4sf) __A,
						     (__v2di) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttps_epi64 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2qq128_mask ((__v4sf) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttps_epu64 (__m128 __A)
{
  return (__m256i) __builtin_ia32_cvttps2uqq256_mask ((__v4sf) __A,
						      (__v4di)
						      _mm256_setzero_si256 (),
						      (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvttps_epu64 (__m256i __W, __mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvttps2uqq256_mask ((__v4sf) __A,
						      (__v4di) __W,
						      (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvttps_epu64 (__mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvttps2uqq256_mask ((__v4sf) __A,
						      (__v4di)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttps_epu64 (__m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2uqq128_mask ((__v4sf) __A,
						      (__v2di)
						      _mm_setzero_si128 (),
						      (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvttps_epu64 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2uqq128_mask ((__v4sf) __A,
						      (__v2di) __W,
						      (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvttps_epu64 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvttps2uqq128_mask ((__v4sf) __A,
						      (__v2di)
						      _mm_setzero_si128 (),
						      (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_f64x2 (__m128d __A)
{
  return (__m256d) __builtin_ia32_broadcastf64x2_256_mask ((__v2df)
							   __A,
						           (__v4df)_mm256_undefined_pd(),
							   (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcast_f64x2 (__m256d __O, __mmask8 __M, __m128d __A)
{
  return (__m256d) __builtin_ia32_broadcastf64x2_256_mask ((__v2df)
							   __A,
							   (__v4df)
							   __O, __M);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcast_f64x2 (__mmask8 __M, __m128d __A)
{
  return (__m256d) __builtin_ia32_broadcastf64x2_256_mask ((__v2df)
							   __A,
							   (__v4df)
							   _mm256_setzero_ps (),
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_i64x2 (__m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti64x2_256_mask ((__v2di)
							   __A,
						           (__v4di)_mm256_undefined_si256(),
							   (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcast_i64x2 (__m256i __O, __mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti64x2_256_mask ((__v2di)
							   __A,
							   (__v4di)
							   __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcast_i64x2 (__mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti64x2_256_mask ((__v2di)
							   __A,
							   (__v4di)
							   _mm256_setzero_si256 (),
							   __M);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_f32x2 (__m128 __A)
{
  return (__m256) __builtin_ia32_broadcastf32x2_256_mask ((__v4sf) __A,
						          (__v8sf)_mm256_undefined_ps(),
							  (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcast_f32x2 (__m256 __O, __mmask8 __M, __m128 __A)
{
  return (__m256) __builtin_ia32_broadcastf32x2_256_mask ((__v4sf) __A,
							  (__v8sf) __O,
							  __M);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcast_f32x2 (__mmask8 __M, __m128 __A)
{
  return (__m256) __builtin_ia32_broadcastf32x2_256_mask ((__v4sf) __A,
							  (__v8sf)
							  _mm256_setzero_ps (),
							  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_i32x2 (__m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti32x2_256_mask ((__v4si)
							   __A,
						          (__v8si)_mm256_undefined_si256(),
							   (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_broadcast_i32x2 (__m256i __O, __mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti32x2_256_mask ((__v4si)
							   __A,
							   (__v8si)
							   __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_broadcast_i32x2 (__mmask8 __M, __m128i __A)
{
  return (__m256i) __builtin_ia32_broadcasti32x2_256_mask ((__v4si)
							   __A,
							   (__v8si)
							   _mm256_setzero_si256 (),
							   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcast_i32x2 (__m128i __A)
{
  return (__m128i) __builtin_ia32_broadcasti32x2_128_mask ((__v4si)
							   __A,
						          (__v4si)_mm_undefined_si128(),
							   (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_broadcast_i32x2 (__m128i __O, __mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_broadcasti32x2_128_mask ((__v4si)
							   __A,
							   (__v4si)
							   __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_broadcast_i32x2 (__mmask8 __M, __m128i __A)
{
  return (__m128i) __builtin_ia32_broadcasti32x2_128_mask ((__v4si)
							   __A,
							   (__v4si)
							   _mm_setzero_si128 (),
							   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mullo_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du) __A * (__v4du) __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_mullo_epi64 (__m256i __W, __mmask8 __U, __m256i __A,
			 __m256i __B)
{
  return (__m256i) __builtin_ia32_pmullq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di) __W,
						  (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_mullo_epi64 (__mmask8 __U, __m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_pmullq256_mask ((__v4di) __A,
						  (__v4di) __B,
						  (__v4di)
						  _mm256_setzero_si256 (),
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mullo_epi64 (__m128i __A, __m128i __B)
{
  return (__m128i) ((__v2du) __A * (__v2du) __B);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mullo_epi64 (__m128i __W, __mmask8 __U, __m128i __A,
		      __m128i __B)
{
  return (__m128i) __builtin_ia32_pmullq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di) __W,
						  (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mullo_epi64 (__mmask8 __U, __m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_pmullq128_mask ((__v2di) __A,
						  (__v2di) __B,
						  (__v2di)
						  _mm_setzero_si128 (),
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_andnot_pd (__m256d __W, __mmask8 __U, __m256d __A,
		       __m256d __B)
{
  return (__m256d) __builtin_ia32_andnpd256_mask ((__v4df) __A,
						  (__v4df) __B,
						  (__v4df) __W,
						  (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_andnot_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_andnpd256_mask ((__v4df) __A,
						  (__v4df) __B,
						  (__v4df)
						  _mm256_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_andnot_pd (__m128d __W, __mmask8 __U, __m128d __A,
		    __m128d __B)
{
  return (__m128d) __builtin_ia32_andnpd128_mask ((__v2df) __A,
						  (__v2df) __B,
						  (__v2df) __W,
						  (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_andnot_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_andnpd128_mask ((__v2df) __A,
						  (__v2df) __B,
						  (__v2df)
						  _mm_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_andnot_ps (__m256 __W, __mmask8 __U, __m256 __A,
		       __m256 __B)
{
  return (__m256) __builtin_ia32_andnps256_mask ((__v8sf) __A,
						 (__v8sf) __B,
						 (__v8sf) __W,
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_andnot_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_andnps256_mask ((__v8sf) __A,
						 (__v8sf) __B,
						 (__v8sf)
						 _mm256_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_andnot_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_andnps128_mask ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_andnot_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_andnps128_mask ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtps_epi64 (__m128 __A)
{
  return (__m256i) __builtin_ia32_cvtps2qq256_mask ((__v4sf) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtps_epi64 (__m256i __W, __mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvtps2qq256_mask ((__v4sf) __A,
						    (__v4di) __W,
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtps_epi64 (__mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvtps2qq256_mask ((__v4sf) __A,
						    (__v4di)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtps_epi64 (__m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2qq128_mask ((__v4sf) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtps_epi64 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2qq128_mask ((__v4sf) __A,
						    (__v2di) __W,
						    (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtps_epi64 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2qq128_mask ((__v4sf) __A,
						    (__v2di)
						    _mm_setzero_si128 (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtps_epu64 (__m128 __A)
{
  return (__m256i) __builtin_ia32_cvtps2uqq256_mask ((__v4sf) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtps_epu64 (__m256i __W, __mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvtps2uqq256_mask ((__v4sf) __A,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtps_epu64 (__mmask8 __U, __m128 __A)
{
  return (__m256i) __builtin_ia32_cvtps2uqq256_mask ((__v4sf) __A,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtps_epu64 (__m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2uqq128_mask ((__v4sf) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtps_epu64 (__m128i __W, __mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2uqq128_mask ((__v4sf) __A,
						     (__v2di) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtps_epu64 (__mmask8 __U, __m128 __A)
{
  return (__m128i) __builtin_ia32_cvtps2uqq128_mask ((__v4sf) __A,
						     (__v2di)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi64_ps (__m256i __A)
{
  return (__m128) __builtin_ia32_cvtqq2ps256_mask ((__v4di) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_ps (__m128 __W, __mmask8 __U, __m256i __A)
{
  return (__m128) __builtin_ia32_cvtqq2ps256_mask ((__v4di) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi64_ps (__mmask8 __U, __m256i __A)
{
  return (__m128) __builtin_ia32_cvtqq2ps256_mask ((__v4di) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi64_ps (__m128i __A)
{
  return (__m128) __builtin_ia32_cvtqq2ps128_mask ((__v2di) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_ps (__m128 __W, __mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtqq2ps128_mask ((__v2di) __A,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi64_ps (__mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtqq2ps128_mask ((__v2di) __A,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu64_ps (__m256i __A)
{
  return (__m128) __builtin_ia32_cvtuqq2ps256_mask ((__v4di) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu64_ps (__m128 __W, __mmask8 __U, __m256i __A)
{
  return (__m128) __builtin_ia32_cvtuqq2ps256_mask ((__v4di) __A,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu64_ps (__mmask8 __U, __m256i __A)
{
  return (__m128) __builtin_ia32_cvtuqq2ps256_mask ((__v4di) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu64_ps (__m128i __A)
{
  return (__m128) __builtin_ia32_cvtuqq2ps128_mask ((__v2di) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu64_ps (__m128 __W, __mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtuqq2ps128_mask ((__v2di) __A,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu64_ps (__mmask8 __U, __m128i __A)
{
  return (__m128) __builtin_ia32_cvtuqq2ps128_mask ((__v2di) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi64_pd (__m256i __A)
{
  return (__m256d) __builtin_ia32_cvtqq2pd256_mask ((__v4di) __A,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepi64_pd (__m256d __W, __mmask8 __U, __m256i __A)
{
  return (__m256d) __builtin_ia32_cvtqq2pd256_mask ((__v4di) __A,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepi64_pd (__mmask8 __U, __m256i __A)
{
  return (__m256d) __builtin_ia32_cvtqq2pd256_mask ((__v4di) __A,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepi64_pd (__m128i __A)
{
  return (__m128d) __builtin_ia32_cvtqq2pd128_mask ((__v2di) __A,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepi64_pd (__m128d __W, __mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtqq2pd128_mask ((__v2di) __A,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepi64_pd (__mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtqq2pd128_mask ((__v2di) __A,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu64_pd (__m256i __A)
{
  return (__m256d) __builtin_ia32_cvtuqq2pd256_mask ((__v4di) __A,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_cvtepu64_pd (__m256d __W, __mmask8 __U, __m256i __A)
{
  return (__m256d) __builtin_ia32_cvtuqq2pd256_mask ((__v4di) __A,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_cvtepu64_pd (__mmask8 __U, __m256i __A)
{
  return (__m256d) __builtin_ia32_cvtuqq2pd256_mask ((__v4di) __A,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_and_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_andpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_and_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_andpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_and_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_andpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_and_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_andpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_and_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_andps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_and_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_andps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_and_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_andps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_and_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_andps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtepu64_pd (__m128i __A)
{
  return (__m128d) __builtin_ia32_cvtuqq2pd128_mask ((__v2di) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cvtepu64_pd (__m128d __W, __mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtuqq2pd128_mask ((__v2di) __A,
						     (__v2df) __W,
						     (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_cvtepu64_pd (__mmask8 __U, __m128i __A)
{
  return (__m128d) __builtin_ia32_cvtuqq2pd128_mask ((__v2di) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_xor_pd (__m256d __W, __mmask8 __U, __m256d __A,
		    __m256d __B)
{
  return (__m256d) __builtin_ia32_xorpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df) __W,
						 (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_xor_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_xorpd256_mask ((__v4df) __A,
						 (__v4df) __B,
						 (__v4df)
						 _mm256_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_xor_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_xorpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_xor_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_xorpd128_mask ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_xor_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_xorps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf) __W,
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_xor_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_xorps256_mask ((__v8sf) __A,
						(__v8sf) __B,
						(__v8sf)
						_mm256_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_xor_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_xorps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_xor_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_xorps128_mask ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_or_pd (__m256d __W, __mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_orpd256_mask ((__v4df) __A,
						(__v4df) __B,
						(__v4df) __W,
						(__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_or_pd (__mmask8 __U, __m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_orpd256_mask ((__v4df) __A,
						(__v4df) __B,
						(__v4df)
						_mm256_setzero_pd (),
						(__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_or_pd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_orpd128_mask ((__v2df) __A,
						(__v2df) __B,
						(__v2df) __W,
						(__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_or_pd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_orpd128_mask ((__v2df) __A,
						(__v2df) __B,
						(__v2df)
						_mm_setzero_pd (),
						(__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_or_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_orps256_mask ((__v8sf) __A,
					       (__v8sf) __B,
					       (__v8sf) __W,
					       (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_or_ps (__mmask8 __U, __m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_orps256_mask ((__v8sf) __A,
					       (__v8sf) __B,
					       (__v8sf)
					       _mm256_setzero_ps (),
					       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_or_ps (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_orps128_mask ((__v4sf) __A,
					       (__v4sf) __B,
					       (__v4sf) __W,
					       (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_or_ps (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_orps128_mask ((__v4sf) __A,
					       (__v4sf) __B,
					       (__v4sf)
					       _mm_setzero_ps (),
					       (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movm_epi32 (__mmask8 __A)
{
  return (__m128i) __builtin_ia32_cvtmask2d128 (__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movm_epi32 (__mmask8 __A)
{
  return (__m256i) __builtin_ia32_cvtmask2d256 (__A);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movm_epi64 (__mmask8 __A)
{
  return (__m128i) __builtin_ia32_cvtmask2q128 (__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movm_epi64 (__mmask8 __A)
{
  return (__m256i) __builtin_ia32_cvtmask2q256 (__A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movepi32_mask (__m128i __A)
{
  return (__mmask8) __builtin_ia32_cvtd2mask128 ((__v4si) __A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movepi32_mask (__m256i __A)
{
  return (__mmask8) __builtin_ia32_cvtd2mask256 ((__v8si) __A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_movepi64_mask (__m128i __A)
{
  return (__mmask8) __builtin_ia32_cvtq2mask128 ((__v2di) __A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movepi64_mask (__m256i __A)
{
  return (__mmask8) __builtin_ia32_cvtq2mask256 ((__v4di) __A);
}

#ifdef __OPTIMIZE__
extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extractf64x2_pd (__m256d __A, const int __imm)
{
  return (__m128d) __builtin_ia32_extractf64x2_256_mask ((__v4df) __A,
							 __imm,
							 (__v2df)
							 _mm_setzero_pd (),
							 (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_extractf64x2_pd (__m128d __W, __mmask8 __U, __m256d __A,
			     const int __imm)
{
  return (__m128d) __builtin_ia32_extractf64x2_256_mask ((__v4df) __A,
							 __imm,
							 (__v2df) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_extractf64x2_pd (__mmask8 __U, __m256d __A,
			      const int __imm)
{
  return (__m128d) __builtin_ia32_extractf64x2_256_mask ((__v4df) __A,
							 __imm,
							 (__v2df)
							 _mm_setzero_pd (),
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extracti64x2_epi64 (__m256i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_extracti64x2_256_mask ((__v4di) __A,
							 __imm,
							 (__v2di)
							 _mm_setzero_si128 (),
							 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_extracti64x2_epi64 (__m128i __W, __mmask8 __U, __m256i __A,
				const int __imm)
{
  return (__m128i) __builtin_ia32_extracti64x2_256_mask ((__v4di) __A,
							 __imm,
							 (__v2di) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_extracti64x2_epi64 (__mmask8 __U, __m256i __A,
				 const int __imm)
{
  return (__m128i) __builtin_ia32_extracti64x2_256_mask ((__v4di) __A,
							 __imm,
							 (__v2di)
							 _mm_setzero_si128 (),
							 (__mmask8)
							 __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_reduce_pd (__m256d __A, int __B)
{
  return (__m256d) __builtin_ia32_reducepd256_mask ((__v4df) __A, __B,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_reduce_pd (__m256d __W, __mmask8 __U, __m256d __A, int __B)
{
  return (__m256d) __builtin_ia32_reducepd256_mask ((__v4df) __A, __B,
						    (__v4df) __W,
						    (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_reduce_pd (__mmask8 __U, __m256d __A, int __B)
{
  return (__m256d) __builtin_ia32_reducepd256_mask ((__v4df) __A, __B,
						    (__v4df)
						    _mm256_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_reduce_pd (__m128d __A, int __B)
{
  return (__m128d) __builtin_ia32_reducepd128_mask ((__v2df) __A, __B,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_reduce_pd (__m128d __W, __mmask8 __U, __m128d __A, int __B)
{
  return (__m128d) __builtin_ia32_reducepd128_mask ((__v2df) __A, __B,
						    (__v2df) __W,
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_reduce_pd (__mmask8 __U, __m128d __A, int __B)
{
  return (__m128d) __builtin_ia32_reducepd128_mask ((__v2df) __A, __B,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_reduce_ps (__m256 __A, int __B)
{
  return (__m256) __builtin_ia32_reduceps256_mask ((__v8sf) __A, __B,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_reduce_ps (__m256 __W, __mmask8 __U, __m256 __A, int __B)
{
  return (__m256) __builtin_ia32_reduceps256_mask ((__v8sf) __A, __B,
						   (__v8sf) __W,
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_reduce_ps (__mmask8 __U, __m256 __A, int __B)
{
  return (__m256) __builtin_ia32_reduceps256_mask ((__v8sf) __A, __B,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_reduce_ps (__m128 __A, int __B)
{
  return (__m128) __builtin_ia32_reduceps128_mask ((__v4sf) __A, __B,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_reduce_ps (__m128 __W, __mmask8 __U, __m128 __A, int __B)
{
  return (__m128) __builtin_ia32_reduceps128_mask ((__v4sf) __A, __B,
						   (__v4sf) __W,
						   (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_reduce_ps (__mmask8 __U, __m128 __A, int __B)
{
  return (__m128) __builtin_ia32_reduceps128_mask ((__v4sf) __A, __B,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_range_pd (__m256d __A, __m256d __B, int __C)
{
  return (__m256d) __builtin_ia32_rangepd256_mask ((__v4df) __A,
						   (__v4df) __B, __C,
						   (__v4df)
						   _mm256_setzero_pd (),
						   (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_range_pd (__m256d __W, __mmask8 __U,
		      __m256d __A, __m256d __B, int __C)
{
  return (__m256d) __builtin_ia32_rangepd256_mask ((__v4df) __A,
						   (__v4df) __B, __C,
						   (__v4df) __W,
						   (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_range_pd (__mmask8 __U, __m256d __A, __m256d __B, int __C)
{
  return (__m256d) __builtin_ia32_rangepd256_mask ((__v4df) __A,
						   (__v4df) __B, __C,
						   (__v4df)
						   _mm256_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_range_pd (__m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_rangepd128_mask ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_range_pd (__m128d __W, __mmask8 __U,
		   __m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_rangepd128_mask ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df) __W,
						   (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_range_pd (__mmask8 __U, __m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_rangepd128_mask ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_range_ps (__m256 __A, __m256 __B, int __C)
{
  return (__m256) __builtin_ia32_rangeps256_mask ((__v8sf) __A,
						  (__v8sf) __B, __C,
						  (__v8sf)
						  _mm256_setzero_ps (),
						  (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_range_ps (__m256 __W, __mmask8 __U, __m256 __A, __m256 __B,
		      int __C)
{
  return (__m256) __builtin_ia32_rangeps256_mask ((__v8sf) __A,
						  (__v8sf) __B, __C,
						  (__v8sf) __W,
						  (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_range_ps (__mmask8 __U, __m256 __A, __m256 __B, int __C)
{
  return (__m256) __builtin_ia32_rangeps256_mask ((__v8sf) __A,
						  (__v8sf) __B, __C,
						  (__v8sf)
						  _mm256_setzero_ps (),
						  (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_range_ps (__m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_rangeps128_mask ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_range_ps (__m128 __W, __mmask8 __U,
		   __m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_rangeps128_mask ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf) __W,
						  (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_range_ps (__mmask8 __U, __m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_rangeps128_mask ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fpclass_pd_mask (__mmask8 __U, __m256d __A,
			     const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasspd256_mask ((__v4df) __A,
						      __imm, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fpclass_pd_mask (__m256d __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasspd256_mask ((__v4df) __A,
						      __imm,
						      (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_fpclass_ps_mask (__mmask8 __U, __m256 __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclassps256_mask ((__v8sf) __A,
						      __imm, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fpclass_ps_mask (__m256 __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclassps256_mask ((__v8sf) __A,
						      __imm,
						      (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fpclass_pd_mask (__mmask8 __U, __m128d __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasspd128_mask ((__v2df) __A,
						      __imm, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fpclass_pd_mask (__m128d __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasspd128_mask ((__v2df) __A,
						      __imm,
						      (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fpclass_ps_mask (__mmask8 __U, __m128 __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclassps128_mask ((__v4sf) __A,
						      __imm, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fpclass_ps_mask (__m128 __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclassps128_mask ((__v4sf) __A,
						      __imm,
						      (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_inserti64x2 (__m256i __A, __m128i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_inserti64x2_256_mask ((__v4di) __A,
							(__v2di) __B,
							__imm,
							(__v4di)
							_mm256_setzero_si256 (),
							(__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_inserti64x2 (__m256i __W, __mmask8 __U, __m256i __A,
			 __m128i __B, const int __imm)
{
  return (__m256i) __builtin_ia32_inserti64x2_256_mask ((__v4di) __A,
							(__v2di) __B,
							__imm,
							(__v4di) __W,
							(__mmask8)
							__U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_inserti64x2 (__mmask8 __U, __m256i __A, __m128i __B,
			  const int __imm)
{
  return (__m256i) __builtin_ia32_inserti64x2_256_mask ((__v4di) __A,
							(__v2di) __B,
							__imm,
							(__v4di)
							_mm256_setzero_si256 (),
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insertf64x2 (__m256d __A, __m128d __B, const int __imm)
{
  return (__m256d) __builtin_ia32_insertf64x2_256_mask ((__v4df) __A,
							(__v2df) __B,
							__imm,
							(__v4df)
							_mm256_setzero_pd (),
							(__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_insertf64x2 (__m256d __W, __mmask8 __U, __m256d __A,
			 __m128d __B, const int __imm)
{
  return (__m256d) __builtin_ia32_insertf64x2_256_mask ((__v4df) __A,
							(__v2df) __B,
							__imm,
							(__v4df) __W,
							(__mmask8)
							__U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskz_insertf64x2 (__mmask8 __U, __m256d __A, __m128d __B,
			  const int __imm)
{
  return (__m256d) __builtin_ia32_insertf64x2_256_mask ((__v4df) __A,
							(__v2df) __B,
							__imm,
							(__v4df)
							_mm256_setzero_pd (),
							(__mmask8)
							__U);
}

#else
#define _mm256_insertf64x2(X, Y, C)                                     \
  ((__m256d) __builtin_ia32_insertf64x2_256_mask ((__v4df)(__m256d) (X),\
    (__v2df)(__m128d) (Y), (int) (C),					\
    (__v4df)(__m256d)_mm256_setzero_pd(),				\
    (__mmask8)-1))

#define _mm256_mask_insertf64x2(W, U, X, Y, C)                          \
  ((__m256d) __builtin_ia32_insertf64x2_256_mask ((__v4df)(__m256d) (X),\
    (__v2df)(__m128d) (Y), (int) (C),					\
    (__v4df)(__m256d)(W),						\
    (__mmask8)(U)))

#define _mm256_maskz_insertf64x2(U, X, Y, C)				\
  ((__m256d) __builtin_ia32_insertf64x2_256_mask ((__v4df)(__m256d) (X),\
    (__v2df)(__m128d) (Y), (int) (C),					\
    (__v4df)(__m256d)_mm256_setzero_pd(),				\
    (__mmask8)(U)))

#define _mm256_inserti64x2(X, Y, C)                                     \
  ((__m256i) __builtin_ia32_inserti64x2_256_mask ((__v4di)(__m256i) (X),\
    (__v2di)(__m128i) (Y), (int) (C),					\
    (__v4di)(__m256i)_mm256_setzero_si256 (),				\
    (__mmask8)-1))

#define _mm256_mask_inserti64x2(W, U, X, Y, C)                          \
  ((__m256i) __builtin_ia32_inserti64x2_256_mask ((__v4di)(__m256i) (X),\
    (__v2di)(__m128i) (Y), (int) (C),					\
    (__v4di)(__m256i)(W),						\
    (__mmask8)(U)))

#define _mm256_maskz_inserti64x2(U, X, Y, C)                            \
  ((__m256i) __builtin_ia32_inserti64x2_256_mask ((__v4di)(__m256i) (X),\
    (__v2di)(__m128i) (Y), (int) (C),					\
    (__v4di)(__m256i)_mm256_setzero_si256 (),				\
    (__mmask8)(U)))

#define _mm256_extractf64x2_pd(X, C)                                    \
  ((__m128d) __builtin_ia32_extractf64x2_256_mask ((__v4df)(__m256d) (X),\
    (int) (C), (__v2df)(__m128d) _mm_setzero_pd(), (__mmask8)-1))

#define _mm256_mask_extractf64x2_pd(W, U, X, C)                         \
  ((__m128d) __builtin_ia32_extractf64x2_256_mask ((__v4df)(__m256d) (X),\
    (int) (C), (__v2df)(__m128d) (W), (__mmask8) (U)))

#define _mm256_maskz_extractf64x2_pd(U, X, C)                           \
  ((__m128d) __builtin_ia32_extractf64x2_256_mask ((__v4df)(__m256d) (X),\
    (int) (C), (__v2df)(__m128d) _mm_setzero_pd(), (__mmask8) (U)))

#define _mm256_extracti64x2_epi64(X, C)                                 \
  ((__m128i) __builtin_ia32_extracti64x2_256_mask ((__v4di)(__m256i) (X),\
    (int) (C), (__v2di)(__m128i) _mm_setzero_si128 (), (__mmask8)-1))

#define _mm256_mask_extracti64x2_epi64(W, U, X, C)                     \
  ((__m128i) __builtin_ia32_extracti64x2_256_mask ((__v4di)(__m256i) (X),\
    (int) (C), (__v2di)(__m128i) (W), (__mmask8) (U)))

#define _mm256_maskz_extracti64x2_epi64(U, X, C)                        \
  ((__m128i) __builtin_ia32_extracti64x2_256_mask ((__v4di)(__m256i) (X),\
    (int) (C), (__v2di)(__m128i) _mm_setzero_si128 (), (__mmask8) (U)))

#define _mm256_reduce_pd(A, B)						\
  ((__m256d) __builtin_ia32_reducepd256_mask ((__v4df)(__m256d)(A),	\
    (int)(B), (__v4df)_mm256_setzero_pd(), (__mmask8)-1))

#define _mm256_mask_reduce_pd(W, U, A, B)				\
  ((__m256d) __builtin_ia32_reducepd256_mask ((__v4df)(__m256d)(A),	\
    (int)(B), (__v4df)(__m256d)(W), (__mmask8)(U)))

#define _mm256_maskz_reduce_pd(U, A, B)					\
  ((__m256d) __builtin_ia32_reducepd256_mask ((__v4df)(__m256d)(A),	\
    (int)(B), (__v4df)_mm256_setzero_pd(), (__mmask8)(U)))

#define _mm_reduce_pd(A, B)						\
  ((__m128d) __builtin_ia32_reducepd128_mask ((__v2df)(__m128d)(A),	\
    (int)(B), (__v2df)_mm_setzero_pd(), (__mmask8)-1))

#define _mm_mask_reduce_pd(W, U, A, B)					\
  ((__m128d) __builtin_ia32_reducepd128_mask ((__v2df)(__m128d)(A),	\
    (int)(B), (__v2df)(__m128d)(W), (__mmask8)(U)))

#define _mm_maskz_reduce_pd(U, A, B)					\
  ((__m128d) __builtin_ia32_reducepd128_mask ((__v2df)(__m128d)(A),	\
    (int)(B), (__v2df)_mm_setzero_pd(), (__mmask8)(U)))

#define _mm256_reduce_ps(A, B)						\
  ((__m256) __builtin_ia32_reduceps256_mask ((__v8sf)(__m256)(A),	\
    (int)(B), (__v8sf)_mm256_setzero_ps(), (__mmask8)-1))

#define _mm256_mask_reduce_ps(W, U, A, B)				\
  ((__m256) __builtin_ia32_reduceps256_mask ((__v8sf)(__m256)(A),	\
    (int)(B), (__v8sf)(__m256)(W), (__mmask8)(U)))

#define _mm256_maskz_reduce_ps(U, A, B)					\
  ((__m256) __builtin_ia32_reduceps256_mask ((__v8sf)(__m256)(A),	\
    (int)(B), (__v8sf)_mm256_setzero_ps(), (__mmask8)(U)))

#define _mm_reduce_ps(A, B)						\
  ((__m128) __builtin_ia32_reduceps128_mask ((__v4sf)(__m128)(A),	\
    (int)(B), (__v4sf)_mm_setzero_ps(), (__mmask8)-1))

#define _mm_mask_reduce_ps(W, U, A, B)					\
  ((__m128) __builtin_ia32_reduceps128_mask ((__v4sf)(__m128)(A),	\
    (int)(B), (__v4sf)(__m128)(W), (__mmask8)(U)))

#define _mm_maskz_reduce_ps(U, A, B)					\
  ((__m128) __builtin_ia32_reduceps128_mask ((__v4sf)(__m128)(A),	\
    (int)(B), (__v4sf)_mm_setzero_ps(), (__mmask8)(U)))

#define _mm256_range_pd(A, B, C)					\
  ((__m256d) __builtin_ia32_rangepd256_mask ((__v4df)(__m256d)(A),	\
    (__v4df)(__m256d)(B), (int)(C),					\
    (__v4df)_mm256_setzero_pd(), (__mmask8)-1))

#define _mm256_maskz_range_pd(U, A, B, C)				\
  ((__m256d) __builtin_ia32_rangepd256_mask ((__v4df)(__m256d)(A),	\
    (__v4df)(__m256d)(B), (int)(C),					\
    (__v4df)_mm256_setzero_pd(), (__mmask8)(U)))

#define _mm_range_pd(A, B, C)						\
  ((__m128d) __builtin_ia32_rangepd128_mask ((__v2df)(__m128d)(A),	\
    (__v2df)(__m128d)(B), (int)(C),					\
    (__v2df)_mm_setzero_pd(), (__mmask8)-1))

#define _mm256_range_ps(A, B, C)					\
  ((__m256) __builtin_ia32_rangeps256_mask ((__v8sf)(__m256)(A),	\
    (__v8sf)(__m256)(B), (int)(C),					\
    (__v8sf)_mm256_setzero_ps(), (__mmask8)-1))

#define _mm256_mask_range_ps(W, U, A, B, C)				\
  ((__m256) __builtin_ia32_rangeps256_mask ((__v8sf)(__m256)(A),	\
    (__v8sf)(__m256)(B), (int)(C),					\
    (__v8sf)(__m256)(W), (__mmask8)(U)))

#define _mm256_maskz_range_ps(U, A, B, C)				\
  ((__m256) __builtin_ia32_rangeps256_mask ((__v8sf)(__m256)(A),	\
    (__v8sf)(__m256)(B), (int)(C),					\
    (__v8sf)_mm256_setzero_ps(), (__mmask8)(U)))

#define _mm_range_ps(A, B, C)						\
  ((__m128) __builtin_ia32_rangeps128_mask ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C),					\
    (__v4sf)_mm_setzero_ps(), (__mmask8)-1))

#define _mm_mask_range_ps(W, U, A, B, C)				\
  ((__m128) __builtin_ia32_rangeps128_mask ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C),					\
    (__v4sf)(__m128)(W), (__mmask8)(U)))

#define _mm_maskz_range_ps(U, A, B, C)					\
  ((__m128) __builtin_ia32_rangeps128_mask ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C),					\
    (__v4sf)_mm_setzero_ps(), (__mmask8)(U)))

#define _mm256_mask_range_pd(W, U, A, B, C)				\
  ((__m256d) __builtin_ia32_rangepd256_mask ((__v4df)(__m256d)(A),	\
    (__v4df)(__m256d)(B), (int)(C),					\
    (__v4df)(__m256d)(W), (__mmask8)(U)))

#define _mm_mask_range_pd(W, U, A, B, C)				\
  ((__m128d) __builtin_ia32_rangepd128_mask ((__v2df)(__m128d)(A),	\
    (__v2df)(__m128d)(B), (int)(C),					\
    (__v2df)(__m128d)(W), (__mmask8)(U)))

#define _mm_maskz_range_pd(U, A, B, C)					\
  ((__m128d) __builtin_ia32_rangepd128_mask ((__v2df)(__m128d)(A),	\
    (__v2df)(__m128d)(B), (int)(C),					\
    (__v2df)_mm_setzero_pd(), (__mmask8)(U)))

#define _mm256_mask_fpclass_pd_mask(u, X, C)                            \
  ((__mmask8) __builtin_ia32_fpclasspd256_mask ((__v4df) (__m256d) (X), \
						(int) (C),(__mmask8)(u)))

#define _mm256_mask_fpclass_ps_mask(u, X, C)				\
  ((__mmask8) __builtin_ia32_fpclassps256_mask ((__v8sf) (__m256) (X),  \
						(int) (C),(__mmask8)(u)))

#define _mm_mask_fpclass_pd_mask(u, X, C)                               \
  ((__mmask8) __builtin_ia32_fpclasspd128_mask ((__v2df) (__m128d) (X), \
						(int) (C),(__mmask8)(u)))

#define _mm_mask_fpclass_ps_mask(u, X, C)                               \
  ((__mmask8) __builtin_ia32_fpclassps128_mask ((__v4sf) (__m128) (X),  \
						(int) (C),(__mmask8)(u)))

#define _mm256_fpclass_pd_mask(X, C)                                    \
  ((__mmask8) __builtin_ia32_fpclasspd256_mask ((__v4df) (__m256d) (X), \
						(int) (C),(__mmask8)-1))

#define _mm256_fpclass_ps_mask(X, C)                                    \
  ((__mmask8) __builtin_ia32_fpclassps256_mask ((__v8sf) (__m256) (X),  \
						(int) (C),(__mmask8)-1))

#define _mm_fpclass_pd_mask(X, C)                                       \
  ((__mmask8) __builtin_ia32_fpclasspd128_mask ((__v2df) (__m128d) (X), \
						(int) (C),(__mmask8)-1))

#define _mm_fpclass_ps_mask(X, C)                                       \
  ((__mmask8) __builtin_ia32_fpclassps128_mask ((__v4sf) (__m128) (X),  \
						(int) (C),(__mmask8)-1))

#endif

#ifdef __DISABLE_AVX512VLDQ__
#undef __DISABLE_AVX512VLDQ__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512VLDQ__ */

#endif /* _AVX512VLDQINTRIN_H_INCLUDED */
