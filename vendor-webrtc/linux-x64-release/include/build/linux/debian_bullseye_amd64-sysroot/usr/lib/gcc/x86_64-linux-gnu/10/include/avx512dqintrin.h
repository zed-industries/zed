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
#error "Never use <avx512dqintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512DQINTRIN_H_INCLUDED
#define _AVX512DQINTRIN_H_INCLUDED

#ifndef __AVX512DQ__
#pragma GCC push_options
#pragma GCC target("avx512dq")
#define __DISABLE_AVX512DQ__
#endif /* __AVX512DQ__ */

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktest_mask8_u8  (__mmask8 __A,  __mmask8 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_ktestcqi (__A, __B);
  return (unsigned char) __builtin_ia32_ktestzqi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestz_mask8_u8 (__mmask8 __A, __mmask8 __B)
{
  return (unsigned char) __builtin_ia32_ktestzqi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestc_mask8_u8 (__mmask8 __A, __mmask8 __B)
{
  return (unsigned char) __builtin_ia32_ktestcqi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktest_mask16_u8  (__mmask16 __A,  __mmask16 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_ktestchi (__A, __B);
  return (unsigned char) __builtin_ia32_ktestzhi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestz_mask16_u8 (__mmask16 __A, __mmask16 __B)
{
  return (unsigned char) __builtin_ia32_ktestzhi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_ktestc_mask16_u8 (__mmask16 __A, __mmask16 __B)
{
  return (unsigned char) __builtin_ia32_ktestchi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortest_mask8_u8  (__mmask8 __A,  __mmask8 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_kortestcqi (__A, __B);
  return (unsigned char) __builtin_ia32_kortestzqi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestz_mask8_u8 (__mmask8 __A, __mmask8 __B)
{
  return (unsigned char) __builtin_ia32_kortestzqi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestc_mask8_u8 (__mmask8 __A, __mmask8 __B)
{
  return (unsigned char) __builtin_ia32_kortestcqi (__A, __B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kadd_mask8 (__mmask8 __A, __mmask8 __B)
{
  return (__mmask8) __builtin_ia32_kaddqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kadd_mask16 (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kaddhi ((__mmask16) __A, (__mmask16) __B);
}

extern __inline unsigned int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtmask8_u32 (__mmask8 __A)
{
  return (unsigned int) __builtin_ia32_kmovb ((__mmask8 ) __A);
}
	
extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtu32_mask8 (unsigned int __A)
{
  return (__mmask8) __builtin_ia32_kmovb ((__mmask8) __A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_load_mask8 (__mmask8 *__A)
{
  return (__mmask8) __builtin_ia32_kmovb (*(__mmask8 *) __A);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_store_mask8 (__mmask8 *__A, __mmask8 __B)
{
  *(__mmask8 *) __A = __builtin_ia32_kmovb (__B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_knot_mask8 (__mmask8 __A)
{
  return (__mmask8) __builtin_ia32_knotqi ((__mmask8) __A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kor_mask8 (__mmask8 __A, __mmask8 __B)
{
  return (__mmask8) __builtin_ia32_korqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kxnor_mask8 (__mmask8 __A, __mmask8 __B)
{
  return (__mmask8) __builtin_ia32_kxnorqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kxor_mask8 (__mmask8 __A, __mmask8 __B)
{
  return (__mmask8) __builtin_ia32_kxorqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kand_mask8 (__mmask8 __A, __mmask8 __B)
{
  return (__mmask8) __builtin_ia32_kandqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kandn_mask8 (__mmask8 __A, __mmask8 __B)
{
  return (__mmask8) __builtin_ia32_kandnqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_f64x2 (__m128d __A)
{
  return (__m512d)
	 __builtin_ia32_broadcastf64x2_512_mask ((__v2df) __A,
						 _mm512_undefined_pd (),
						 (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_f64x2 (__m512d __O, __mmask8 __M, __m128d __A)
{
  return (__m512d) __builtin_ia32_broadcastf64x2_512_mask ((__v2df)
							   __A,
							   (__v8df)
							   __O, __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_f64x2 (__mmask8 __M, __m128d __A)
{
  return (__m512d) __builtin_ia32_broadcastf64x2_512_mask ((__v2df)
							   __A,
							   (__v8df)
							   _mm512_setzero_ps (),
							   __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_i64x2 (__m128i __A)
{
  return (__m512i)
	 __builtin_ia32_broadcasti64x2_512_mask ((__v2di) __A,
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_i64x2 (__m512i __O, __mmask8 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_broadcasti64x2_512_mask ((__v2di)
							   __A,
							   (__v8di)
							   __O, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_i64x2 (__mmask8 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_broadcasti64x2_512_mask ((__v2di)
							   __A,
							   (__v8di)
							   _mm512_setzero_si512 (),
							   __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_f32x2 (__m128 __A)
{
  return (__m512)
	 __builtin_ia32_broadcastf32x2_512_mask ((__v4sf) __A,
						 (__v16sf)_mm512_undefined_ps (),
						 (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_f32x2 (__m512 __O, __mmask16 __M, __m128 __A)
{
  return (__m512) __builtin_ia32_broadcastf32x2_512_mask ((__v4sf) __A,
							  (__v16sf)
							  __O, __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_f32x2 (__mmask16 __M, __m128 __A)
{
  return (__m512) __builtin_ia32_broadcastf32x2_512_mask ((__v4sf) __A,
							  (__v16sf)
							  _mm512_setzero_ps (),
							  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_i32x2 (__m128i __A)
{
  return (__m512i)
	 __builtin_ia32_broadcasti32x2_512_mask ((__v4si) __A,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_i32x2 (__m512i __O, __mmask16 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_broadcasti32x2_512_mask ((__v4si)
							   __A,
							   (__v16si)
							   __O, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_i32x2 (__mmask16 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_broadcasti32x2_512_mask ((__v4si)
							   __A,
							   (__v16si)
							   _mm512_setzero_si512 (),
							   __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_f32x8 (__m256 __A)
{
  return (__m512)
	 __builtin_ia32_broadcastf32x8_512_mask ((__v8sf) __A,
						 _mm512_undefined_ps (),
						 (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_f32x8 (__m512 __O, __mmask16 __M, __m256 __A)
{
  return (__m512) __builtin_ia32_broadcastf32x8_512_mask ((__v8sf) __A,
							  (__v16sf)__O,
							  __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_f32x8 (__mmask16 __M, __m256 __A)
{
  return (__m512) __builtin_ia32_broadcastf32x8_512_mask ((__v8sf) __A,
							  (__v16sf)
							  _mm512_setzero_ps (),
							  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_i32x8 (__m256i __A)
{
  return (__m512i)
	 __builtin_ia32_broadcasti32x8_512_mask ((__v8si) __A,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_i32x8 (__m512i __O, __mmask16 __M, __m256i __A)
{
  return (__m512i) __builtin_ia32_broadcasti32x8_512_mask ((__v8si)
							   __A,
							   (__v16si)__O,
							   __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_i32x8 (__mmask16 __M, __m256i __A)
{
  return (__m512i) __builtin_ia32_broadcasti32x8_512_mask ((__v8si)
							   __A,
							   (__v16si)
							   _mm512_setzero_si512 (),
							   __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mullo_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v8du) __A * (__v8du) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mullo_epi64 (__m512i __W, __mmask8 __U, __m512i __A,
			 __m512i __B)
{
  return (__m512i) __builtin_ia32_pmullq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W,
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mullo_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmullq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_xor_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_xorpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_xor_pd (__m512d __W, __mmask8 __U, __m512d __A,
		    __m512d __B)
{
  return (__m512d) __builtin_ia32_xorpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_xor_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_xorpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_xor_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_xorps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_xor_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_xorps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_xor_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_xorps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_or_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_orpd512_mask ((__v8df) __A,
						(__v8df) __B,
						(__v8df)
						_mm512_setzero_pd (),
						(__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_or_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_orpd512_mask ((__v8df) __A,
						(__v8df) __B,
						(__v8df) __W,
						(__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_or_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_orpd512_mask ((__v8df) __A,
						(__v8df) __B,
						(__v8df)
						_mm512_setzero_pd (),
						(__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_or_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_orps512_mask ((__v16sf) __A,
					       (__v16sf) __B,
					       (__v16sf)
					       _mm512_setzero_ps (),
					       (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_or_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_orps512_mask ((__v16sf) __A,
					       (__v16sf) __B,
					       (__v16sf) __W,
					       (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_or_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_orps512_mask ((__v16sf) __A,
					       (__v16sf) __B,
					       (__v16sf)
					       _mm512_setzero_ps (),
					       (__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_and_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_andpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_and_pd (__m512d __W, __mmask8 __U, __m512d __A,
		    __m512d __B)
{
  return (__m512d) __builtin_ia32_andpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_and_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_andpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_and_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_andps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_and_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_andps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_and_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_andps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_andnot_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_andnpd512_mask ((__v8df) __A,
						  (__v8df) __B,
						  (__v8df)
						  _mm512_setzero_pd (),
						  (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_andnot_pd (__m512d __W, __mmask8 __U, __m512d __A,
		       __m512d __B)
{
  return (__m512d) __builtin_ia32_andnpd512_mask ((__v8df) __A,
						  (__v8df) __B,
						  (__v8df) __W,
						  (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_andnot_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_andnpd512_mask ((__v8df) __A,
						  (__v8df) __B,
						  (__v8df)
						  _mm512_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_andnot_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_andnps512_mask ((__v16sf) __A,
						 (__v16sf) __B,
						 (__v16sf)
						 _mm512_setzero_ps (),
						 (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_andnot_ps (__m512 __W, __mmask16 __U, __m512 __A,
		       __m512 __B)
{
  return (__m512) __builtin_ia32_andnps512_mask ((__v16sf) __A,
						 (__v16sf) __B,
						 (__v16sf) __W,
						 (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_andnot_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_andnps512_mask ((__v16sf) __A,
						 (__v16sf) __B,
						 (__v16sf)
						 _mm512_setzero_ps (),
						 (__mmask16) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movepi32_mask (__m512i __A)
{
  return (__mmask16) __builtin_ia32_cvtd2mask512 ((__v16si) __A);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movepi64_mask (__m512i __A)
{
  return (__mmask8) __builtin_ia32_cvtq2mask512 ((__v8di) __A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movm_epi32 (__mmask16 __A)
{
  return (__m512i) __builtin_ia32_cvtmask2d512 (__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movm_epi64 (__mmask8 __A)
{
  return (__m512i) __builtin_ia32_cvtmask2q512 (__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttpd_epi64 (__m512d __A)
{
  return (__m512i) __builtin_ia32_cvttpd2qq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttpd_epi64 (__m512i __W, __mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvttpd2qq512_mask ((__v8df) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttpd_epi64 (__mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvttpd2qq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttpd_epu64 (__m512d __A)
{
  return (__m512i) __builtin_ia32_cvttpd2uqq512_mask ((__v8df) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) -1,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttpd_epu64 (__m512i __W, __mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvttpd2uqq512_mask ((__v8df) __A,
						      (__v8di) __W,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttpd_epu64 (__mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvttpd2uqq512_mask ((__v8df) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttps_epi64 (__m256 __A)
{
  return (__m512i) __builtin_ia32_cvttps2qq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttps_epi64 (__m512i __W, __mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvttps2qq512_mask ((__v8sf) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttps_epi64 (__mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvttps2qq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttps_epu64 (__m256 __A)
{
  return (__m512i) __builtin_ia32_cvttps2uqq512_mask ((__v8sf) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) -1,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttps_epu64 (__m512i __W, __mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvttps2uqq512_mask ((__v8sf) __A,
						      (__v8di) __W,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttps_epu64 (__mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvttps2uqq512_mask ((__v8sf) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtpd_epi64 (__m512d __A)
{
  return (__m512i) __builtin_ia32_cvtpd2qq512_mask ((__v8df) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtpd_epi64 (__m512i __W, __mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvtpd2qq512_mask ((__v8df) __A,
						    (__v8di) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtpd_epi64 (__mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvtpd2qq512_mask ((__v8df) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtpd_epu64 (__m512d __A)
{
  return (__m512i) __builtin_ia32_cvtpd2uqq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtpd_epu64 (__m512i __W, __mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvtpd2uqq512_mask ((__v8df) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtpd_epu64 (__mmask8 __U, __m512d __A)
{
  return (__m512i) __builtin_ia32_cvtpd2uqq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtps_epi64 (__m256 __A)
{
  return (__m512i) __builtin_ia32_cvtps2qq512_mask ((__v8sf) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtps_epi64 (__m512i __W, __mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvtps2qq512_mask ((__v8sf) __A,
						    (__v8di) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtps_epi64 (__mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvtps2qq512_mask ((__v8sf) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtps_epu64 (__m256 __A)
{
  return (__m512i) __builtin_ia32_cvtps2uqq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtps_epu64 (__m512i __W, __mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvtps2uqq512_mask ((__v8sf) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtps_epu64 (__mmask8 __U, __m256 __A)
{
  return (__m512i) __builtin_ia32_cvtps2uqq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi64_ps (__m512i __A)
{
  return (__m256) __builtin_ia32_cvtqq2ps512_mask ((__v8di) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_ps (__m256 __W, __mmask8 __U, __m512i __A)
{
  return (__m256) __builtin_ia32_cvtqq2ps512_mask ((__v8di) __A,
						   (__v8sf) __W,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi64_ps (__mmask8 __U, __m512i __A)
{
  return (__m256) __builtin_ia32_cvtqq2ps512_mask ((__v8di) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu64_ps (__m512i __A)
{
  return (__m256) __builtin_ia32_cvtuqq2ps512_mask ((__v8di) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu64_ps (__m256 __W, __mmask8 __U, __m512i __A)
{
  return (__m256) __builtin_ia32_cvtuqq2ps512_mask ((__v8di) __A,
						    (__v8sf) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu64_ps (__mmask8 __U, __m512i __A)
{
  return (__m256) __builtin_ia32_cvtuqq2ps512_mask ((__v8di) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi64_pd (__m512i __A)
{
  return (__m512d) __builtin_ia32_cvtqq2pd512_mask ((__v8di) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_pd (__m512d __W, __mmask8 __U, __m512i __A)
{
  return (__m512d) __builtin_ia32_cvtqq2pd512_mask ((__v8di) __A,
						    (__v8df) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi64_pd (__mmask8 __U, __m512i __A)
{
  return (__m512d) __builtin_ia32_cvtqq2pd512_mask ((__v8di) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu64_pd (__m512i __A)
{
  return (__m512d) __builtin_ia32_cvtuqq2pd512_mask ((__v8di) __A,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu64_pd (__m512d __W, __mmask8 __U, __m512i __A)
{
  return (__m512d) __builtin_ia32_cvtuqq2pd512_mask ((__v8di) __A,
						     (__v8df) __W,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu64_pd (__mmask8 __U, __m512i __A)
{
  return (__m512d) __builtin_ia32_cvtuqq2pd512_mask ((__v8di) __A,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

#ifdef __OPTIMIZE__
extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftli_mask8 (__mmask8 __A, unsigned int __B)
{
  return (__mmask8) __builtin_ia32_kshiftliqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftri_mask8 (__mmask8 __A, unsigned int __B)
{
  return (__mmask8) __builtin_ia32_kshiftriqi ((__mmask8) __A, (__mmask8) __B);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_range_pd (__m512d __A, __m512d __B, int __C)
{
  return (__m512d) __builtin_ia32_rangepd512_mask ((__v8df) __A,
						   (__v8df) __B, __C,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_range_pd (__m512d __W, __mmask8 __U,
		      __m512d __A, __m512d __B, int __C)
{
  return (__m512d) __builtin_ia32_rangepd512_mask ((__v8df) __A,
						   (__v8df) __B, __C,
						   (__v8df) __W,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_range_pd (__mmask8 __U, __m512d __A, __m512d __B, int __C)
{
  return (__m512d) __builtin_ia32_rangepd512_mask ((__v8df) __A,
						   (__v8df) __B, __C,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_range_ps (__m512 __A, __m512 __B, int __C)
{
  return (__m512) __builtin_ia32_rangeps512_mask ((__v16sf) __A,
						  (__v16sf) __B, __C,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_range_ps (__m512 __W, __mmask16 __U,
		      __m512 __A, __m512 __B, int __C)
{
  return (__m512) __builtin_ia32_rangeps512_mask ((__v16sf) __A,
						  (__v16sf) __B, __C,
						  (__v16sf) __W,
						  (__mmask16) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_range_ps (__mmask16 __U, __m512 __A, __m512 __B, int __C)
{
  return (__m512) __builtin_ia32_rangeps512_mask ((__v16sf) __A,
						  (__v16sf) __B, __C,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_reduce_sd (__m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_reducesd_mask ((__v2df) __A,
						 (__v2df) __B, __C,
						 (__v2df) _mm_setzero_pd (),
						 (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_reduce_sd (__m128d __W,  __mmask8 __U, __m128d __A,
		    __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_reducesd_mask ((__v2df) __A,
						 (__v2df) __B, __C,
						 (__v2df) __W,
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_reduce_sd (__mmask8 __U, __m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_reducesd_mask ((__v2df) __A,
						 (__v2df) __B, __C,
						 (__v2df) _mm_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_reduce_ss (__m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_reducess_mask ((__v4sf) __A,
						(__v4sf) __B, __C,
						(__v4sf) _mm_setzero_ps (),
						(__mmask8) -1);
}


extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_reduce_ss (__m128 __W,  __mmask8 __U, __m128 __A,
		    __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_reducess_mask ((__v4sf) __A,
						(__v4sf) __B, __C,
						(__v4sf) __W,
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_reduce_ss (__mmask8 __U, __m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_reducess_mask ((__v4sf) __A,
						(__v4sf) __B, __C,
						(__v4sf) _mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_range_sd (__m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_range_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df) __W,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_range_sd (__mmask8 __U, __m128d __A, __m128d __B, int __C)
{
  return (__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_range_ss (__m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_rangess128_mask_round ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_range_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_rangess128_mask_round ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf) __W,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}


extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_range_ss (__mmask8 __U, __m128 __A, __m128 __B, int __C)
{
  return (__m128) __builtin_ia32_rangess128_mask_round ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_range_round_sd (__m128d __A, __m128d __B, int __C, const int __R)
{
  return (__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) -1, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_range_round_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			 int __C, const int __R)
{
  return (__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df) __W,
						   (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_range_round_sd (__mmask8 __U, __m128d __A, __m128d __B, int __C,
			  const int __R)
{
  return (__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df) __A,
						   (__v2df) __B, __C,
						   (__v2df)
						   _mm_setzero_pd (),
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_range_round_ss (__m128 __A, __m128 __B, int __C, const int __R)
{
  return (__m128) __builtin_ia32_rangess128_mask_round ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) -1, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_range_round_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			 int __C, const int __R)
{
  return (__m128) __builtin_ia32_rangess128_mask_round ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf) __W,
						  (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_range_round_ss (__mmask8 __U, __m128 __A, __m128 __B, int __C,
			  const int __R)
{
  return (__m128) __builtin_ia32_rangess128_mask_round ((__v4sf) __A,
						  (__v4sf) __B, __C,
						  (__v4sf)
						  _mm_setzero_ps (),
						  (__mmask8) __U, __R);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fpclass_ss_mask (__m128 __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclassss_mask ((__v4sf) __A, __imm,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fpclass_sd_mask (__m128d __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasssd_mask ((__v2df) __A, __imm,
						   (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fpclass_ss_mask (__mmask8 __U, __m128 __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclassss_mask ((__v4sf) __A, __imm, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fpclass_sd_mask (__mmask8 __U, __m128d __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasssd_mask ((__v2df) __A, __imm, __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundpd_epi64 (__m512d __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttpd2qq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundpd_epi64 (__m512i __W, __mmask8 __U, __m512d __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvttpd2qq512_mask ((__v8df) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundpd_epi64 (__mmask8 __U, __m512d __A,
				 const int __R)
{
  return (__m512i) __builtin_ia32_cvttpd2qq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundpd_epu64 (__m512d __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttpd2uqq512_mask ((__v8df) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) -1,
						      __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundpd_epu64 (__m512i __W, __mmask8 __U, __m512d __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvttpd2uqq512_mask ((__v8df) __A,
						      (__v8di) __W,
						      (__mmask8) __U,
						      __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundpd_epu64 (__mmask8 __U, __m512d __A,
				 const int __R)
{
  return (__m512i) __builtin_ia32_cvttpd2uqq512_mask ((__v8df) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) __U,
						      __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundps_epi64 (__m256 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2qq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundps_epi64 (__m512i __W, __mmask8 __U, __m256 __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2qq512_mask ((__v8sf) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundps_epi64 (__mmask8 __U, __m256 __A,
				 const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2qq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundps_epu64 (__m256 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2uqq512_mask ((__v8sf) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) -1,
						      __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundps_epu64 (__m512i __W, __mmask8 __U, __m256 __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2uqq512_mask ((__v8sf) __A,
						      (__v8di) __W,
						      (__mmask8) __U,
						      __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundps_epu64 (__mmask8 __U, __m256 __A,
				 const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2uqq512_mask ((__v8sf) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) __U,
						      __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundpd_epi64 (__m512d __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtpd2qq512_mask ((__v8df) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) -1,
						    __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundpd_epi64 (__m512i __W, __mmask8 __U, __m512d __A,
			       const int __R)
{
  return (__m512i) __builtin_ia32_cvtpd2qq512_mask ((__v8df) __A,
						    (__v8di) __W,
						    (__mmask8) __U,
						    __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundpd_epi64 (__mmask8 __U, __m512d __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvtpd2qq512_mask ((__v8df) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U,
						    __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundpd_epu64 (__m512d __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtpd2uqq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundpd_epu64 (__m512i __W, __mmask8 __U, __m512d __A,
			       const int __R)
{
  return (__m512i) __builtin_ia32_cvtpd2uqq512_mask ((__v8df) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundpd_epu64 (__mmask8 __U, __m512d __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvtpd2uqq512_mask ((__v8df) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundps_epi64 (__m256 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2qq512_mask ((__v8sf) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) -1,
						    __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundps_epi64 (__m512i __W, __mmask8 __U, __m256 __A,
			       const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2qq512_mask ((__v8sf) __A,
						    (__v8di) __W,
						    (__mmask8) __U,
						    __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundps_epi64 (__mmask8 __U, __m256 __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2qq512_mask ((__v8sf) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U,
						    __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundps_epu64 (__m256 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2uqq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) -1,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundps_epu64 (__m512i __W, __mmask8 __U, __m256 __A,
			       const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2uqq512_mask ((__v8sf) __A,
						     (__v8di) __W,
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundps_epu64 (__mmask8 __U, __m256 __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2uqq512_mask ((__v8sf) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U,
						     __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundepi64_ps (__m512i __A, const int __R)
{
  return (__m256) __builtin_ia32_cvtqq2ps512_mask ((__v8di) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) -1,
						   __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundepi64_ps (__m256 __W, __mmask8 __U, __m512i __A,
			       const int __R)
{
  return (__m256) __builtin_ia32_cvtqq2ps512_mask ((__v8di) __A,
						   (__v8sf) __W,
						   (__mmask8) __U,
						   __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundepi64_ps (__mmask8 __U, __m512i __A,
				const int __R)
{
  return (__m256) __builtin_ia32_cvtqq2ps512_mask ((__v8di) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U,
						   __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundepu64_ps (__m512i __A, const int __R)
{
  return (__m256) __builtin_ia32_cvtuqq2ps512_mask ((__v8di) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) -1,
						    __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundepu64_ps (__m256 __W, __mmask8 __U, __m512i __A,
			       const int __R)
{
  return (__m256) __builtin_ia32_cvtuqq2ps512_mask ((__v8di) __A,
						    (__v8sf) __W,
						    (__mmask8) __U,
						    __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundepu64_ps (__mmask8 __U, __m512i __A,
				const int __R)
{
  return (__m256) __builtin_ia32_cvtuqq2ps512_mask ((__v8di) __A,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U,
						    __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundepi64_pd (__m512i __A, const int __R)
{
  return (__m512d) __builtin_ia32_cvtqq2pd512_mask ((__v8di) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) -1,
						    __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundepi64_pd (__m512d __W, __mmask8 __U, __m512i __A,
			       const int __R)
{
  return (__m512d) __builtin_ia32_cvtqq2pd512_mask ((__v8di) __A,
						    (__v8df) __W,
						    (__mmask8) __U,
						    __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundepi64_pd (__mmask8 __U, __m512i __A,
				const int __R)
{
  return (__m512d) __builtin_ia32_cvtqq2pd512_mask ((__v8di) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U,
						    __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundepu64_pd (__m512i __A, const int __R)
{
  return (__m512d) __builtin_ia32_cvtuqq2pd512_mask ((__v8di) __A,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) -1,
						     __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundepu64_pd (__m512d __W, __mmask8 __U, __m512i __A,
			       const int __R)
{
  return (__m512d) __builtin_ia32_cvtuqq2pd512_mask ((__v8di) __A,
						     (__v8df) __W,
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundepu64_pd (__mmask8 __U, __m512i __A,
				const int __R)
{
  return (__m512d) __builtin_ia32_cvtuqq2pd512_mask ((__v8di) __A,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) __U,
						     __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_pd (__m512d __A, int __B)
{
  return (__m512d) __builtin_ia32_reducepd512_mask ((__v8df) __A, __B,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_pd (__m512d __W, __mmask8 __U, __m512d __A, int __B)
{
  return (__m512d) __builtin_ia32_reducepd512_mask ((__v8df) __A, __B,
						    (__v8df) __W,
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_reduce_pd (__mmask8 __U, __m512d __A, int __B)
{
  return (__m512d) __builtin_ia32_reducepd512_mask ((__v8df) __A, __B,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_ps (__m512 __A, int __B)
{
  return (__m512) __builtin_ia32_reduceps512_mask ((__v16sf) __A, __B,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_ps (__m512 __W, __mmask16 __U, __m512 __A, int __B)
{
  return (__m512) __builtin_ia32_reduceps512_mask ((__v16sf) __A, __B,
						   (__v16sf) __W,
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_reduce_ps (__mmask16 __U, __m512 __A, int __B)
{
  return (__m512) __builtin_ia32_reduceps512_mask ((__v16sf) __A, __B,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extractf32x8_ps (__m512 __A, const int __imm)
{
  return (__m256) __builtin_ia32_extractf32x8_mask ((__v16sf) __A,
						    __imm,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) -1);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extractf32x8_ps (__m256 __W, __mmask8 __U, __m512 __A,
			     const int __imm)
{
  return (__m256) __builtin_ia32_extractf32x8_mask ((__v16sf) __A,
						    __imm,
						    (__v8sf) __W,
						    (__mmask8) __U);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extractf32x8_ps (__mmask8 __U, __m512 __A,
			      const int __imm)
{
  return (__m256) __builtin_ia32_extractf32x8_mask ((__v16sf) __A,
						    __imm,
						    (__v8sf)
						    _mm256_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extractf64x2_pd (__m512d __A, const int __imm)
{
  return (__m128d) __builtin_ia32_extractf64x2_512_mask ((__v8df) __A,
							 __imm,
							 (__v2df)
							 _mm_setzero_pd (),
							 (__mmask8) -1);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extractf64x2_pd (__m128d __W, __mmask8 __U, __m512d __A,
			     const int __imm)
{
  return (__m128d) __builtin_ia32_extractf64x2_512_mask ((__v8df) __A,
							 __imm,
							 (__v2df) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extractf64x2_pd (__mmask8 __U, __m512d __A,
			      const int __imm)
{
  return (__m128d) __builtin_ia32_extractf64x2_512_mask ((__v8df) __A,
							 __imm,
							 (__v2df)
							 _mm_setzero_pd (),
							 (__mmask8)
							 __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extracti32x8_epi32 (__m512i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_extracti32x8_mask ((__v16si) __A,
						     __imm,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extracti32x8_epi32 (__m256i __W, __mmask8 __U, __m512i __A,
				const int __imm)
{
  return (__m256i) __builtin_ia32_extracti32x8_mask ((__v16si) __A,
						     __imm,
						     (__v8si) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extracti32x8_epi32 (__mmask8 __U, __m512i __A,
				 const int __imm)
{
  return (__m256i) __builtin_ia32_extracti32x8_mask ((__v16si) __A,
						     __imm,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extracti64x2_epi64 (__m512i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_extracti64x2_512_mask ((__v8di) __A,
							 __imm,
							 (__v2di)
							 _mm_setzero_si128 (),
							 (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extracti64x2_epi64 (__m128i __W, __mmask8 __U, __m512i __A,
				const int __imm)
{
  return (__m128i) __builtin_ia32_extracti64x2_512_mask ((__v8di) __A,
							 __imm,
							 (__v2di) __W,
							 (__mmask8)
							 __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extracti64x2_epi64 (__mmask8 __U, __m512i __A,
				 const int __imm)
{
  return (__m128i) __builtin_ia32_extracti64x2_512_mask ((__v8di) __A,
							 __imm,
							 (__v2di)
							 _mm_setzero_si128 (),
							 (__mmask8)
							 __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_range_round_pd (__m512d __A, __m512d __B, int __C,
		       const int __R)
{
  return (__m512d) __builtin_ia32_rangepd512_mask ((__v8df) __A,
						   (__v8df) __B, __C,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) -1,
						   __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_range_round_pd (__m512d __W, __mmask8 __U,
			    __m512d __A, __m512d __B, int __C,
			    const int __R)
{
  return (__m512d) __builtin_ia32_rangepd512_mask ((__v8df) __A,
						   (__v8df) __B, __C,
						   (__v8df) __W,
						   (__mmask8) __U,
						   __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_range_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			     int __C, const int __R)
{
  return (__m512d) __builtin_ia32_rangepd512_mask ((__v8df) __A,
						   (__v8df) __B, __C,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __U,
						   __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_range_round_ps (__m512 __A, __m512 __B, int __C, const int __R)
{
  return (__m512) __builtin_ia32_rangeps512_mask ((__v16sf) __A,
						  (__v16sf) __B, __C,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) -1,
						  __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_range_round_ps (__m512 __W, __mmask16 __U,
			    __m512 __A, __m512 __B, int __C,
			    const int __R)
{
  return (__m512) __builtin_ia32_rangeps512_mask ((__v16sf) __A,
						  (__v16sf) __B, __C,
						  (__v16sf) __W,
						  (__mmask16) __U,
						  __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_range_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
			     int __C, const int __R)
{
  return (__m512) __builtin_ia32_rangeps512_mask ((__v16sf) __A,
						  (__v16sf) __B, __C,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __U,
						  __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_inserti32x8 (__m512i __A, __m256i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti32x8_mask ((__v16si) __A,
						    (__v8si) __B,
						    __imm,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_inserti32x8 (__m512i __W, __mmask16 __U, __m512i __A,
			 __m256i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti32x8_mask ((__v16si) __A,
						    (__v8si) __B,
						    __imm,
						    (__v16si) __W,
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_inserti32x8 (__mmask16 __U, __m512i __A, __m256i __B,
			  const int __imm)
{
  return (__m512i) __builtin_ia32_inserti32x8_mask ((__v16si) __A,
						    (__v8si) __B,
						    __imm,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_insertf32x8 (__m512 __A, __m256 __B, const int __imm)
{
  return (__m512) __builtin_ia32_insertf32x8_mask ((__v16sf) __A,
						   (__v8sf) __B,
						   __imm,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_insertf32x8 (__m512 __W, __mmask16 __U, __m512 __A,
			 __m256 __B, const int __imm)
{
  return (__m512) __builtin_ia32_insertf32x8_mask ((__v16sf) __A,
						   (__v8sf) __B,
						   __imm,
						   (__v16sf) __W,
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_insertf32x8 (__mmask16 __U, __m512 __A, __m256 __B,
			  const int __imm)
{
  return (__m512) __builtin_ia32_insertf32x8_mask ((__v16sf) __A,
						   (__v8sf) __B,
						   __imm,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_inserti64x2 (__m512i __A, __m128i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti64x2_512_mask ((__v8di) __A,
							(__v2di) __B,
							__imm,
							(__v8di)
							_mm512_setzero_si512 (),
							(__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_inserti64x2 (__m512i __W, __mmask8 __U, __m512i __A,
			 __m128i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti64x2_512_mask ((__v8di) __A,
							(__v2di) __B,
							__imm,
							(__v8di) __W,
							(__mmask8)
							__U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_inserti64x2 (__mmask8 __U, __m512i __A, __m128i __B,
			  const int __imm)
{
  return (__m512i) __builtin_ia32_inserti64x2_512_mask ((__v8di) __A,
							(__v2di) __B,
							__imm,
							(__v8di)
							_mm512_setzero_si512 (),
							(__mmask8)
							__U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_insertf64x2 (__m512d __A, __m128d __B, const int __imm)
{
  return (__m512d) __builtin_ia32_insertf64x2_512_mask ((__v8df) __A,
							(__v2df) __B,
							__imm,
							(__v8df)
							_mm512_setzero_pd (),
							(__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_insertf64x2 (__m512d __W, __mmask8 __U, __m512d __A,
			 __m128d __B, const int __imm)
{
  return (__m512d) __builtin_ia32_insertf64x2_512_mask ((__v8df) __A,
							(__v2df) __B,
							__imm,
							(__v8df) __W,
							(__mmask8)
							__U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_insertf64x2 (__mmask8 __U, __m512d __A, __m128d __B,
			  const int __imm)
{
  return (__m512d) __builtin_ia32_insertf64x2_512_mask ((__v8df) __A,
							(__v2df) __B,
							__imm,
							(__v8df)
							_mm512_setzero_pd (),
							(__mmask8)
							__U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fpclass_pd_mask (__mmask8 __U, __m512d __A,
			     const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasspd512_mask ((__v8df) __A,
						      __imm, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fpclass_pd_mask (__m512d __A, const int __imm)
{
  return (__mmask8) __builtin_ia32_fpclasspd512_mask ((__v8df) __A,
						      __imm,
						      (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fpclass_ps_mask (__mmask16 __U, __m512 __A,
			     const int __imm)
{
  return (__mmask16) __builtin_ia32_fpclassps512_mask ((__v16sf) __A,
						       __imm, __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fpclass_ps_mask (__m512 __A, const int __imm)
{
  return (__mmask16) __builtin_ia32_fpclassps512_mask ((__v16sf) __A,
						       __imm,
						       (__mmask16) -1);
}

#else
#define _kshiftli_mask8(X, Y)						\
  ((__mmask8) __builtin_ia32_kshiftliqi ((__mmask8)(X), (__mmask8)(Y)))

#define _kshiftri_mask8(X, Y)						\
  ((__mmask8) __builtin_ia32_kshiftriqi ((__mmask8)(X), (__mmask8)(Y)))

#define _mm_range_sd(A, B, C)						 \
  ((__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df)(__m128d)(A), \
    (__v2df)(__m128d)(B), (int)(C), (__v2df) _mm_setzero_pd (), 	 \
    (__mmask8) -1, _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_range_sd(W, U, A, B, C)				 \
  ((__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df)(__m128d)(A), \
    (__v2df)(__m128d)(B), (int)(C), (__v2df)(__m128d)(W), 		 \
    (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_range_sd(U, A, B, C)					 \
  ((__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df)(__m128d)(A), \
    (__v2df)(__m128d)(B), (int)(C), (__v2df) _mm_setzero_pd (), 	 \
    (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_range_ss(A, B, C)						\
  ((__m128) __builtin_ia32_rangess128_mask_round ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf) _mm_setzero_ps (),		\
    (__mmask8) -1, _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_range_ss(W, U, A, B, C)				\
  ((__m128) __builtin_ia32_rangess128_mask_round ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf)(__m128)(W),			\
    (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_range_ss(U, A, B, C)					\
  ((__m128) __builtin_ia32_rangess128_mask_round ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf) _mm_setzero_ps (),		\
    (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_range_round_sd(A, B, C, R)					 \
  ((__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df)(__m128d)(A), \
    (__v2df)(__m128d)(B), (int)(C), (__v2df) _mm_setzero_pd (),		 \
    (__mmask8) -1, (R)))

#define _mm_mask_range_round_sd(W, U, A, B, C, R)			 \
  ((__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df)(__m128d)(A), \
    (__v2df)(__m128d)(B), (int)(C), (__v2df)(__m128d)(W),		 \
    (__mmask8)(U), (R)))

#define _mm_maskz_range_round_sd(U, A, B, C, R)				 \
  ((__m128d) __builtin_ia32_rangesd128_mask_round ((__v2df)(__m128d)(A), \
    (__v2df)(__m128d)(B), (int)(C), (__v2df) _mm_setzero_pd (),		 \
    (__mmask8)(U), (R)))

#define _mm_range_round_ss(A, B, C, R)					\
  ((__m128) __builtin_ia32_rangess128_mask_round ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf) _mm_setzero_ps (),		\
    (__mmask8) -1, (R)))

#define _mm_mask_range_round_ss(W, U, A, B, C, R)			\
  ((__m128) __builtin_ia32_rangess128_mask_round ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf)(__m128)(W),			\
    (__mmask8)(U), (R)))

#define _mm_maskz_range_round_ss(U, A, B, C, R)				\
  ((__m128) __builtin_ia32_rangess128_mask_round ((__v4sf)(__m128)(A),	\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf) _mm_setzero_ps (),		\
    (__mmask8)(U), (R)))

#define _mm512_cvtt_roundpd_epi64(A, B)		    \
  ((__m512i)__builtin_ia32_cvttpd2qq512_mask ((A), (__v8di)		\
					      _mm512_setzero_si512 (),	\
					      -1, (B)))

#define _mm512_mask_cvtt_roundpd_epi64(W, U, A, B)  \
    ((__m512i)__builtin_ia32_cvttpd2qq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvtt_roundpd_epi64(U, A, B)    \
    ((__m512i)__builtin_ia32_cvttpd2qq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvtt_roundpd_epu64(A, B)		    \
    ((__m512i)__builtin_ia32_cvttpd2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), -1, (B)))

#define _mm512_mask_cvtt_roundpd_epu64(W, U, A, B)  \
    ((__m512i)__builtin_ia32_cvttpd2uqq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvtt_roundpd_epu64(U, A, B)    \
    ((__m512i)__builtin_ia32_cvttpd2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvtt_roundps_epi64(A, B)		    \
    ((__m512i)__builtin_ia32_cvttps2qq512_mask ((A), (__v8di)_mm512_setzero_si512 (), -1, (B)))

#define _mm512_mask_cvtt_roundps_epi64(W, U, A, B)  \
    ((__m512i)__builtin_ia32_cvttps2qq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvtt_roundps_epi64(U, A, B)    \
    ((__m512i)__builtin_ia32_cvttps2qq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvtt_roundps_epu64(A, B)		    \
    ((__m512i)__builtin_ia32_cvttps2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), -1, (B)))

#define _mm512_mask_cvtt_roundps_epu64(W, U, A, B)  \
    ((__m512i)__builtin_ia32_cvttps2uqq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvtt_roundps_epu64(U, A, B)    \
    ((__m512i)__builtin_ia32_cvttps2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvt_roundpd_epi64(A, B)		    \
    ((__m512i)__builtin_ia32_cvtpd2qq512_mask ((A), (__v8di)_mm512_setzero_si512 (), -1, (B)))

#define _mm512_mask_cvt_roundpd_epi64(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvtpd2qq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvt_roundpd_epi64(U, A, B)     \
    ((__m512i)__builtin_ia32_cvtpd2qq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvt_roundpd_epu64(A, B)		    \
    ((__m512i)__builtin_ia32_cvtpd2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), -1, (B)))

#define _mm512_mask_cvt_roundpd_epu64(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvtpd2uqq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvt_roundpd_epu64(U, A, B)     \
    ((__m512i)__builtin_ia32_cvtpd2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvt_roundps_epi64(A, B)		    \
    ((__m512i)__builtin_ia32_cvtps2qq512_mask ((A), (__v8di)_mm512_setzero_si512 (), -1, (B)))

#define _mm512_mask_cvt_roundps_epi64(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvtps2qq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvt_roundps_epi64(U, A, B)     \
    ((__m512i)__builtin_ia32_cvtps2qq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvt_roundps_epu64(A, B)		    \
    ((__m512i)__builtin_ia32_cvtps2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), -1, (B)))

#define _mm512_mask_cvt_roundps_epu64(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvtps2uqq512_mask ((A), (__v8di)(W), (U), (B)))

#define _mm512_maskz_cvt_roundps_epu64(U, A, B)     \
    ((__m512i)__builtin_ia32_cvtps2uqq512_mask ((A), (__v8di)_mm512_setzero_si512 (), (U), (B)))

#define _mm512_cvt_roundepi64_ps(A, B)		    \
    ((__m256)__builtin_ia32_cvtqq2ps512_mask ((__v8di)(A), (__v8sf)_mm256_setzero_ps (), -1, (B)))

#define _mm512_mask_cvt_roundepi64_ps(W, U, A, B)   \
    ((__m256)__builtin_ia32_cvtqq2ps512_mask ((__v8di)(A), (W), (U), (B)))

#define _mm512_maskz_cvt_roundepi64_ps(U, A, B)     \
    ((__m256)__builtin_ia32_cvtqq2ps512_mask ((__v8di)(A), (__v8sf)_mm256_setzero_ps (), (U), (B)))

#define _mm512_cvt_roundepu64_ps(A, B)		    \
    ((__m256)__builtin_ia32_cvtuqq2ps512_mask ((__v8di)(A), (__v8sf)_mm256_setzero_ps (), -1, (B)))

#define _mm512_mask_cvt_roundepu64_ps(W, U, A, B)   \
    ((__m256)__builtin_ia32_cvtuqq2ps512_mask ((__v8di)(A), (W), (U), (B)))

#define _mm512_maskz_cvt_roundepu64_ps(U, A, B)     \
    ((__m256)__builtin_ia32_cvtuqq2ps512_mask ((__v8di)(A), (__v8sf)_mm256_setzero_ps (), (U), (B)))

#define _mm512_cvt_roundepi64_pd(A, B)		    \
    ((__m512d)__builtin_ia32_cvtqq2pd512_mask ((__v8di)(A), (__v8df)_mm512_setzero_pd (), -1, (B)))

#define _mm512_mask_cvt_roundepi64_pd(W, U, A, B)   \
    ((__m512d)__builtin_ia32_cvtqq2pd512_mask ((__v8di)(A), (W), (U), (B)))

#define _mm512_maskz_cvt_roundepi64_pd(U, A, B)     \
    ((__m512d)__builtin_ia32_cvtqq2pd512_mask ((__v8di)(A), (__v8df)_mm512_setzero_pd (), (U), (B)))

#define _mm512_cvt_roundepu64_pd(A, B)		    \
    ((__m512d)__builtin_ia32_cvtuqq2pd512_mask ((__v8di)(A), (__v8df)_mm512_setzero_pd (), -1, (B)))

#define _mm512_mask_cvt_roundepu64_pd(W, U, A, B)   \
    ((__m512d)__builtin_ia32_cvtuqq2pd512_mask ((__v8di)(A), (W), (U), (B)))

#define _mm512_maskz_cvt_roundepu64_pd(U, A, B)     \
    ((__m512d)__builtin_ia32_cvtuqq2pd512_mask ((__v8di)(A), (__v8df)_mm512_setzero_pd (), (U), (B)))

#define _mm512_reduce_pd(A, B)						\
  ((__m512d) __builtin_ia32_reducepd512_mask ((__v8df)(__m512d)(A),	\
    (int)(B), (__v8df)_mm512_setzero_pd (), (__mmask8)-1))

#define _mm512_mask_reduce_pd(W, U, A, B)				\
  ((__m512d) __builtin_ia32_reducepd512_mask ((__v8df)(__m512d)(A),	\
    (int)(B), (__v8df)(__m512d)(W), (__mmask8)(U)))

#define _mm512_maskz_reduce_pd(U, A, B)					\
  ((__m512d) __builtin_ia32_reducepd512_mask ((__v8df)(__m512d)(A),	\
    (int)(B), (__v8df)_mm512_setzero_pd (), (__mmask8)(U)))

#define _mm512_reduce_ps(A, B)						\
  ((__m512) __builtin_ia32_reduceps512_mask ((__v16sf)(__m512)(A),	\
    (int)(B), (__v16sf)_mm512_setzero_ps (), (__mmask16)-1))

#define _mm512_mask_reduce_ps(W, U, A, B)				\
  ((__m512) __builtin_ia32_reduceps512_mask ((__v16sf)(__m512)(A),	\
    (int)(B), (__v16sf)(__m512)(W), (__mmask16)(U)))

#define _mm512_maskz_reduce_ps(U, A, B)					\
  ((__m512) __builtin_ia32_reduceps512_mask ((__v16sf)(__m512)(A),	\
    (int)(B), (__v16sf)_mm512_setzero_ps (), (__mmask16)(U)))

#define _mm512_extractf32x8_ps(X, C)                                    \
  ((__m256) __builtin_ia32_extractf32x8_mask ((__v16sf)(__m512) (X),    \
    (int) (C), (__v8sf)(__m256) _mm256_setzero_ps (), (__mmask8)-1))

#define _mm512_mask_extractf32x8_ps(W, U, X, C)                         \
  ((__m256) __builtin_ia32_extractf32x8_mask ((__v16sf)(__m512) (X),    \
    (int) (C), (__v8sf)(__m256) (W), (__mmask8) (U)))

#define _mm512_maskz_extractf32x8_ps(U, X, C)                           \
  ((__m256) __builtin_ia32_extractf32x8_mask ((__v16sf)(__m512) (X),    \
    (int) (C), (__v8sf)(__m256) _mm256_setzero_ps (), (__mmask8) (U)))

#define _mm512_extractf64x2_pd(X, C)                                    \
  ((__m128d) __builtin_ia32_extractf64x2_512_mask ((__v8df)(__m512d) (X),\
    (int) (C), (__v2df)(__m128d) _mm_setzero_pd (), (__mmask8)-1))

#define _mm512_mask_extractf64x2_pd(W, U, X, C)                         \
  ((__m128d) __builtin_ia32_extractf64x2_512_mask ((__v8df)(__m512d) (X),\
    (int) (C), (__v2df)(__m128d) (W), (__mmask8) (U)))

#define _mm512_maskz_extractf64x2_pd(U, X, C)                           \
  ((__m128d) __builtin_ia32_extractf64x2_512_mask ((__v8df)(__m512d) (X),\
    (int) (C), (__v2df)(__m128d) _mm_setzero_pd (), (__mmask8) (U)))

#define _mm512_extracti32x8_epi32(X, C)                                 \
  ((__m256i) __builtin_ia32_extracti32x8_mask ((__v16si)(__m512i) (X),  \
    (int) (C), (__v8si)(__m256i) _mm256_setzero_si256 (), (__mmask8)-1))

#define _mm512_mask_extracti32x8_epi32(W, U, X, C)                      \
  ((__m256i) __builtin_ia32_extracti32x8_mask ((__v16si)(__m512i) (X),  \
    (int) (C), (__v8si)(__m256i) (W), (__mmask8) (U)))

#define _mm512_maskz_extracti32x8_epi32(U, X, C)                        \
  ((__m256i) __builtin_ia32_extracti32x8_mask ((__v16si)(__m512i) (X),  \
    (int) (C), (__v8si)(__m256i) _mm256_setzero_si256 (), (__mmask8) (U)))

#define _mm512_extracti64x2_epi64(X, C)                                 \
  ((__m128i) __builtin_ia32_extracti64x2_512_mask ((__v8di)(__m512i) (X),\
    (int) (C), (__v2di)(__m128i) _mm_setzero_si128 (), (__mmask8)-1))

#define _mm512_mask_extracti64x2_epi64(W, U, X, C)                      \
  ((__m128i) __builtin_ia32_extracti64x2_512_mask ((__v8di)(__m512i) (X),\
    (int) (C), (__v2di)(__m128i) (W), (__mmask8) (U)))

#define _mm512_maskz_extracti64x2_epi64(U, X, C)                        \
  ((__m128i) __builtin_ia32_extracti64x2_512_mask ((__v8di)(__m512i) (X),\
    (int) (C), (__v2di)(__m128i) _mm_setzero_si128 (), (__mmask8) (U)))

#define _mm512_range_pd(A, B, C)					\
  ((__m512d) __builtin_ia32_rangepd512_mask ((__v8df)(__m512d)(A),	\
    (__v8df)(__m512d)(B), (int)(C),					\
    (__v8df)_mm512_setzero_pd (), (__mmask8)-1, _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_range_pd(W, U, A, B, C)				\
  ((__m512d) __builtin_ia32_rangepd512_mask ((__v8df)(__m512d)(A),	\
    (__v8df)(__m512d)(B), (int)(C),					\
    (__v8df)(__m512d)(W), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_range_pd(U, A, B, C)				\
  ((__m512d) __builtin_ia32_rangepd512_mask ((__v8df)(__m512d)(A),	\
    (__v8df)(__m512d)(B), (int)(C),					\
    (__v8df)_mm512_setzero_pd (), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_range_ps(A, B, C)					\
  ((__m512) __builtin_ia32_rangeps512_mask ((__v16sf)(__m512)(A),	\
    (__v16sf)(__m512)(B), (int)(C),					\
    (__v16sf)_mm512_setzero_ps (), (__mmask16)-1, _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_range_ps(W, U, A, B, C)				\
  ((__m512) __builtin_ia32_rangeps512_mask ((__v16sf)(__m512)(A),	\
    (__v16sf)(__m512)(B), (int)(C),					\
    (__v16sf)(__m512)(W), (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_range_ps(U, A, B, C)				\
  ((__m512) __builtin_ia32_rangeps512_mask ((__v16sf)(__m512)(A),	\
    (__v16sf)(__m512)(B), (int)(C),					\
    (__v16sf)_mm512_setzero_ps (), (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_range_round_pd(A, B, C, R)					\
  ((__m512d) __builtin_ia32_rangepd512_mask ((__v8df)(__m512d)(A),	\
    (__v8df)(__m512d)(B), (int)(C),					\
    (__v8df)_mm512_setzero_pd (), (__mmask8)-1, (R)))

#define _mm512_mask_range_round_pd(W, U, A, B, C, R)				\
  ((__m512d) __builtin_ia32_rangepd512_mask ((__v8df)(__m512d)(A),	\
    (__v8df)(__m512d)(B), (int)(C),					\
    (__v8df)(__m512d)(W), (__mmask8)(U), (R)))

#define _mm512_maskz_range_round_pd(U, A, B, C, R)				\
  ((__m512d) __builtin_ia32_rangepd512_mask ((__v8df)(__m512d)(A),	\
    (__v8df)(__m512d)(B), (int)(C),					\
    (__v8df)_mm512_setzero_pd (), (__mmask8)(U), (R)))

#define _mm512_range_round_ps(A, B, C, R)					\
  ((__m512) __builtin_ia32_rangeps512_mask ((__v16sf)(__m512)(A),	\
    (__v16sf)(__m512)(B), (int)(C),					\
    (__v16sf)_mm512_setzero_ps (), (__mmask16)-1, (R)))

#define _mm512_mask_range_round_ps(W, U, A, B, C, R)				\
  ((__m512) __builtin_ia32_rangeps512_mask ((__v16sf)(__m512)(A),	\
    (__v16sf)(__m512)(B), (int)(C),					\
    (__v16sf)(__m512)(W), (__mmask16)(U), (R)))

#define _mm512_maskz_range_round_ps(U, A, B, C, R)				\
  ((__m512) __builtin_ia32_rangeps512_mask ((__v16sf)(__m512)(A),	\
    (__v16sf)(__m512)(B), (int)(C),					\
    (__v16sf)_mm512_setzero_ps (), (__mmask16)(U), (R)))

#define _mm512_insertf64x2(X, Y, C)                                     \
  ((__m512d) __builtin_ia32_insertf64x2_512_mask ((__v8df)(__m512d) (X),\
    (__v2df)(__m128d) (Y), (int) (C), (__v8df)(__m512d) (X),            \
    (__mmask8)-1))

#define _mm512_mask_insertf64x2(W, U, X, Y, C)                          \
  ((__m512d) __builtin_ia32_insertf64x2_512_mask ((__v8df)(__m512d) (X),\
    (__v2df)(__m128d) (Y), (int) (C), (__v8df)(__m512d) (W),            \
    (__mmask8) (U)))

#define _mm512_maskz_insertf64x2(U, X, Y, C)                            \
  ((__m512d) __builtin_ia32_insertf64x2_512_mask ((__v8df)(__m512d) (X),\
    (__v2df)(__m128d) (Y), (int) (C),                                   \
    (__v8df)(__m512d) _mm512_setzero_pd (), (__mmask8) (U)))

#define _mm512_inserti64x2(X, Y, C)                                     \
  ((__m512i) __builtin_ia32_inserti64x2_512_mask ((__v8di)(__m512i) (X),\
    (__v2di)(__m128i) (Y), (int) (C), (__v8di)(__m512i) (X), (__mmask8)-1))

#define _mm512_mask_inserti64x2(W, U, X, Y, C)                          \
  ((__m512i) __builtin_ia32_inserti64x2_512_mask ((__v8di)(__m512i) (X),\
    (__v2di)(__m128i) (Y), (int) (C), (__v8di)(__m512i) (W),            \
    (__mmask8) (U)))

#define _mm512_maskz_inserti64x2(U, X, Y, C)                            \
  ((__m512i) __builtin_ia32_inserti64x2_512_mask ((__v8di)(__m512i) (X),\
    (__v2di)(__m128i) (Y), (int) (C),                                   \
    (__v8di)(__m512i) _mm512_setzero_si512 (), (__mmask8) (U)))

#define _mm512_insertf32x8(X, Y, C)                                     \
  ((__m512) __builtin_ia32_insertf32x8_mask ((__v16sf)(__m512) (X),     \
    (__v8sf)(__m256) (Y), (int) (C),\
    (__v16sf)(__m512)_mm512_setzero_ps (),\
    (__mmask16)-1))

#define _mm512_mask_insertf32x8(W, U, X, Y, C)                          \
  ((__m512) __builtin_ia32_insertf32x8_mask ((__v16sf)(__m512) (X),     \
    (__v8sf)(__m256) (Y), (int) (C),\
    (__v16sf)(__m512)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_insertf32x8(U, X, Y, C)                            \
  ((__m512) __builtin_ia32_insertf32x8_mask ((__v16sf)(__m512) (X),     \
    (__v8sf)(__m256) (Y), (int) (C),\
    (__v16sf)(__m512)_mm512_setzero_ps (),\
    (__mmask16)(U)))

#define _mm512_inserti32x8(X, Y, C)                                     \
  ((__m512i) __builtin_ia32_inserti32x8_mask ((__v16si)(__m512i) (X),   \
    (__v8si)(__m256i) (Y), (int) (C),\
    (__v16si)(__m512i)_mm512_setzero_si512 (),\
    (__mmask16)-1))

#define _mm512_mask_inserti32x8(W, U, X, Y, C)                          \
  ((__m512i) __builtin_ia32_inserti32x8_mask ((__v16si)(__m512i) (X),   \
    (__v8si)(__m256i) (Y), (int) (C),\
    (__v16si)(__m512i)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_inserti32x8(U, X, Y, C)                            \
  ((__m512i) __builtin_ia32_inserti32x8_mask ((__v16si)(__m512i) (X),   \
    (__v8si)(__m256i) (Y), (int) (C),\
    (__v16si)(__m512i)_mm512_setzero_si512 (),\
    (__mmask16)(U)))

#define _mm_fpclass_ss_mask(X, C)					\
  ((__mmask8) __builtin_ia32_fpclassss_mask ((__v4sf) (__m128) (X),	\
					     (int) (C), (__mmask8) (-1))) \

#define _mm_fpclass_sd_mask(X, C)					\
  ((__mmask8) __builtin_ia32_fpclasssd_mask ((__v2df) (__m128d) (X),	\
					     (int) (C), (__mmask8) (-1))) \

#define _mm_mask_fpclass_ss_mask(X, C, U)				\
  ((__mmask8) __builtin_ia32_fpclassss_mask ((__v4sf) (__m128) (X),	\
					     (int) (C), (__mmask8) (U)))

#define _mm_mask_fpclass_sd_mask(X, C, U)				\
  ((__mmask8) __builtin_ia32_fpclasssd_mask ((__v2df) (__m128d) (X),	\
					     (int) (C), (__mmask8) (U)))

#define _mm512_mask_fpclass_pd_mask(u, X, C)                            \
  ((__mmask8) __builtin_ia32_fpclasspd512_mask ((__v8df) (__m512d) (X), \
						(int) (C), (__mmask8)(u)))

#define _mm512_mask_fpclass_ps_mask(u, x, c)				\
  ((__mmask16) __builtin_ia32_fpclassps512_mask ((__v16sf) (__m512) (x),\
						 (int) (c),(__mmask8)(u)))

#define _mm512_fpclass_pd_mask(X, C)                                    \
  ((__mmask8) __builtin_ia32_fpclasspd512_mask ((__v8df) (__m512d) (X), \
						(int) (C), (__mmask8)-1))

#define _mm512_fpclass_ps_mask(x, c)                                    \
  ((__mmask16) __builtin_ia32_fpclassps512_mask ((__v16sf) (__m512) (x),\
						 (int) (c),(__mmask8)-1))

#define _mm_reduce_sd(A, B, C)						\
  ((__m128d) __builtin_ia32_reducesd_mask ((__v2df)(__m128d)(A),	\
    (__v2df)(__m128d)(B), (int)(C), (__v2df) _mm_setzero_pd (),		\
    (__mmask8)-1))

#define _mm_mask_reduce_sd(W, U, A, B, C)				\
  ((__m128d) __builtin_ia32_reducesd_mask ((__v2df)(__m128d)(A),	\
    (__v2df)(__m128d)(B), (int)(C), (__v2df)(__m128d)(W), (__mmask8)(U)))

#define _mm_maskz_reduce_sd(U, A, B, C)					\
  ((__m128d) __builtin_ia32_reducesd_mask ((__v2df)(__m128d)(A),	\
    (__v2df)(__m128d)(B), (int)(C), (__v2df) _mm_setzero_pd (),		\
    (__mmask8)(U)))

#define _mm_reduce_ss(A, B, C)						\
  ((__m128) __builtin_ia32_reducess_mask ((__v4sf)(__m128)(A),		\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf) _mm_setzero_ps (),		\
    (__mmask8)-1))

#define _mm_mask_reduce_ss(W, U, A, B, C)				\
  ((__m128) __builtin_ia32_reducess_mask ((__v4sf)(__m128)(A),		\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf)(__m128)(W), (__mmask8)(U)))

#define _mm_maskz_reduce_ss(U, A, B, C)					\
  ((__m128) __builtin_ia32_reducess_mask ((__v4sf)(__m128)(A),		\
    (__v4sf)(__m128)(B), (int)(C), (__v4sf) _mm_setzero_ps (),		\
    (__mmask8)(U)))



#endif

#ifdef __DISABLE_AVX512DQ__
#undef __DISABLE_AVX512DQ__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512DQ__ */

#endif /* _AVX512DQINTRIN_H_INCLUDED */
