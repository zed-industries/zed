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
#error "Never use <avx512fintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX512FINTRIN_H_INCLUDED
#define _AVX512FINTRIN_H_INCLUDED

#ifndef __AVX512F__
#pragma GCC push_options
#pragma GCC target("avx512f")
#define __DISABLE_AVX512F__
#endif /* __AVX512F__ */

/* Internal data types for implementing the intrinsics.  */
typedef double __v8df __attribute__ ((__vector_size__ (64)));
typedef float __v16sf __attribute__ ((__vector_size__ (64)));
typedef long long __v8di __attribute__ ((__vector_size__ (64)));
typedef unsigned long long __v8du __attribute__ ((__vector_size__ (64)));
typedef int __v16si __attribute__ ((__vector_size__ (64)));
typedef unsigned int __v16su __attribute__ ((__vector_size__ (64)));
typedef short __v32hi __attribute__ ((__vector_size__ (64)));
typedef unsigned short __v32hu __attribute__ ((__vector_size__ (64)));
typedef char __v64qi __attribute__ ((__vector_size__ (64)));
typedef unsigned char __v64qu __attribute__ ((__vector_size__ (64)));

/* The Intel API is flexible enough that we must allow aliasing with other
   vector types, and their scalar components.  */
typedef float __m512 __attribute__ ((__vector_size__ (64), __may_alias__));
typedef long long __m512i __attribute__ ((__vector_size__ (64), __may_alias__));
typedef double __m512d __attribute__ ((__vector_size__ (64), __may_alias__));

/* Unaligned version of the same type.  */
typedef float __m512_u __attribute__ ((__vector_size__ (64), __may_alias__, __aligned__ (1)));
typedef long long __m512i_u __attribute__ ((__vector_size__ (64), __may_alias__, __aligned__ (1)));
typedef double __m512d_u __attribute__ ((__vector_size__ (64), __may_alias__, __aligned__ (1)));

typedef unsigned char  __mmask8;
typedef unsigned short __mmask16;

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_int2mask (int __M)
{
  return (__mmask16) __M;
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask2int (__mmask16 __M)
{
  return (int) __M;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set_epi64 (long long __A, long long __B, long long __C,
		  long long __D, long long __E, long long __F,
		  long long __G, long long __H)
{
  return __extension__ (__m512i) (__v8di)
	 { __H, __G, __F, __E, __D, __C, __B, __A };
}

/* Create the vector [A B C D E F G H I J K L M N O P].  */
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set_epi32 (int __A, int __B, int __C, int __D,
		  int __E, int __F, int __G, int __H,
		  int __I, int __J, int __K, int __L,
		  int __M, int __N, int __O, int __P)
{
  return __extension__ (__m512i)(__v16si)
	 { __P, __O, __N, __M, __L, __K, __J, __I,
	   __H, __G, __F, __E, __D, __C, __B, __A };
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set_epi16 (short __q31, short __q30, short __q29, short __q28,
		  short __q27, short __q26, short __q25, short __q24,
		  short __q23, short __q22, short __q21, short __q20,
		  short __q19, short __q18, short __q17, short __q16,
		  short __q15, short __q14, short __q13, short __q12,
		  short __q11, short __q10, short __q09, short __q08,
		  short __q07, short __q06, short __q05, short __q04,
		  short __q03, short __q02, short __q01, short __q00)
{
  return __extension__ (__m512i)(__v32hi){
    __q00, __q01, __q02, __q03, __q04, __q05, __q06, __q07,
    __q08, __q09, __q10, __q11, __q12, __q13, __q14, __q15,
    __q16, __q17, __q18, __q19, __q20, __q21, __q22, __q23,
    __q24, __q25, __q26, __q27, __q28, __q29, __q30, __q31
  };
}

extern __inline __m512i
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set_epi8 (char __q63, char __q62, char __q61, char __q60,
		 char __q59, char __q58, char __q57, char __q56,
		 char __q55, char __q54, char __q53, char __q52,
		 char __q51, char __q50, char __q49, char __q48,
		 char __q47, char __q46, char __q45, char __q44,
		 char __q43, char __q42, char __q41, char __q40,
		 char __q39, char __q38, char __q37, char __q36,
		 char __q35, char __q34, char __q33, char __q32,
		 char __q31, char __q30, char __q29, char __q28,
		 char __q27, char __q26, char __q25, char __q24,
		 char __q23, char __q22, char __q21, char __q20,
		 char __q19, char __q18, char __q17, char __q16,
		 char __q15, char __q14, char __q13, char __q12,
		 char __q11, char __q10, char __q09, char __q08,
		 char __q07, char __q06, char __q05, char __q04,
		 char __q03, char __q02, char __q01, char __q00)
{
  return __extension__ (__m512i)(__v64qi){
    __q00, __q01, __q02, __q03, __q04, __q05, __q06, __q07,
    __q08, __q09, __q10, __q11, __q12, __q13, __q14, __q15,
    __q16, __q17, __q18, __q19, __q20, __q21, __q22, __q23,
    __q24, __q25, __q26, __q27, __q28, __q29, __q30, __q31,
    __q32, __q33, __q34, __q35, __q36, __q37, __q38, __q39,
    __q40, __q41, __q42, __q43, __q44, __q45, __q46, __q47,
    __q48, __q49, __q50, __q51, __q52, __q53, __q54, __q55,
    __q56, __q57, __q58, __q59, __q60, __q61, __q62, __q63
  };
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set_pd (double __A, double __B, double __C, double __D,
	       double __E, double __F, double __G, double __H)
{
  return __extension__ (__m512d)
	 { __H, __G, __F, __E, __D, __C, __B, __A };
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set_ps (float __A, float __B, float __C, float __D,
	       float __E, float __F, float __G, float __H,
	       float __I, float __J, float __K, float __L,
	       float __M, float __N, float __O, float __P)
{
  return __extension__ (__m512)
	 { __P, __O, __N, __M, __L, __K, __J, __I,
	   __H, __G, __F, __E, __D, __C, __B, __A };
}

#define _mm512_setr_epi64(e0,e1,e2,e3,e4,e5,e6,e7)			      \
  _mm512_set_epi64(e7,e6,e5,e4,e3,e2,e1,e0)

#define _mm512_setr_epi32(e0,e1,e2,e3,e4,e5,e6,e7,			      \
			  e8,e9,e10,e11,e12,e13,e14,e15)		      \
  _mm512_set_epi32(e15,e14,e13,e12,e11,e10,e9,e8,e7,e6,e5,e4,e3,e2,e1,e0)

#define _mm512_setr_pd(e0,e1,e2,e3,e4,e5,e6,e7)				      \
  _mm512_set_pd(e7,e6,e5,e4,e3,e2,e1,e0)

#define _mm512_setr_ps(e0,e1,e2,e3,e4,e5,e6,e7,e8,e9,e10,e11,e12,e13,e14,e15) \
  _mm512_set_ps(e15,e14,e13,e12,e11,e10,e9,e8,e7,e6,e5,e4,e3,e2,e1,e0)

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_undefined_ps (void)
{
  __m512 __Y = __Y;
  return __Y;
}

#define _mm512_undefined _mm512_undefined_ps

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_undefined_pd (void)
{
  __m512d __Y = __Y;
  return __Y;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_undefined_epi32 (void)
{
  __m512i __Y = __Y;
  return __Y;
}

#define _mm512_undefined_si512 _mm512_undefined_epi32

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set1_epi8 (char __A)
{
  return __extension__ (__m512i)(__v64qi)
	 { __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A };
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set1_epi16 (short __A)
{
  return __extension__ (__m512i)(__v32hi)
	 { __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A,
	   __A, __A, __A, __A, __A, __A, __A, __A };
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set1_pd (double __A)
{
  return (__m512d) __builtin_ia32_broadcastsd512 (__extension__
						  (__v2df) { __A, },
						  (__v8df)
						  _mm512_undefined_pd (),
						  (__mmask8) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set1_ps (float __A)
{
  return (__m512) __builtin_ia32_broadcastss512 (__extension__
						 (__v4sf) { __A, },
						 (__v16sf)
						 _mm512_undefined_ps (),
						 (__mmask16) -1);
}

/* Create the vector [A B C D A B C D A B C D A B C D].  */
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set4_epi32 (int __A, int __B, int __C, int __D)
{
  return __extension__ (__m512i)(__v16si)
	 { __D, __C, __B, __A, __D, __C, __B, __A,
	   __D, __C, __B, __A, __D, __C, __B, __A };
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set4_epi64 (long long __A, long long __B, long long __C,
		   long long __D)
{
  return __extension__ (__m512i) (__v8di)
	 { __D, __C, __B, __A, __D, __C, __B, __A };
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set4_pd (double __A, double __B, double __C, double __D)
{
  return __extension__ (__m512d)
	 { __D, __C, __B, __A, __D, __C, __B, __A };
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set4_ps (float __A, float __B, float __C, float __D)
{
  return __extension__ (__m512)
	 { __D, __C, __B, __A, __D, __C, __B, __A,
	   __D, __C, __B, __A, __D, __C, __B, __A };
}

#define _mm512_setr4_epi64(e0,e1,e2,e3)					      \
  _mm512_set4_epi64(e3,e2,e1,e0)

#define _mm512_setr4_epi32(e0,e1,e2,e3)					      \
  _mm512_set4_epi32(e3,e2,e1,e0)

#define _mm512_setr4_pd(e0,e1,e2,e3)					      \
  _mm512_set4_pd(e3,e2,e1,e0)

#define _mm512_setr4_ps(e0,e1,e2,e3)					      \
  _mm512_set4_ps(e3,e2,e1,e0)

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_setzero_ps (void)
{
  return __extension__ (__m512){ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
				 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 };
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_setzero (void)
{
  return _mm512_setzero_ps ();
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_setzero_pd (void)
{
  return __extension__ (__m512d) { 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 };
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_setzero_epi32 (void)
{
  return __extension__ (__m512i)(__v8di){ 0, 0, 0, 0, 0, 0, 0, 0 };
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_setzero_si512 (void)
{
  return __extension__ (__m512i)(__v8di){ 0, 0, 0, 0, 0, 0, 0, 0 };
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mov_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_movapd512_mask ((__v8df) __A,
						  (__v8df) __W,
						  (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mov_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_movapd512_mask ((__v8df) __A,
						  (__v8df)
						  _mm512_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mov_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_movaps512_mask ((__v16sf) __A,
						 (__v16sf) __W,
						 (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mov_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_movaps512_mask ((__v16sf) __A,
						 (__v16sf)
						 _mm512_setzero_ps (),
						 (__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_load_pd (void const *__P)
{
  return *(__m512d *) __P;
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_load_pd (__m512d __W, __mmask8 __U, void const *__P)
{
  return (__m512d) __builtin_ia32_loadapd512_mask ((const __v8df *) __P,
						   (__v8df) __W,
						   (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_load_pd (__mmask8 __U, void const *__P)
{
  return (__m512d) __builtin_ia32_loadapd512_mask ((const __v8df *) __P,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_store_pd (void *__P, __m512d __A)
{
  *(__m512d *) __P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_store_pd (void *__P, __mmask8 __U, __m512d __A)
{
  __builtin_ia32_storeapd512_mask ((__v8df *) __P, (__v8df) __A,
				   (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_load_ps (void const *__P)
{
  return *(__m512 *) __P;
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_load_ps (__m512 __W, __mmask16 __U, void const *__P)
{
  return (__m512) __builtin_ia32_loadaps512_mask ((const __v16sf *) __P,
						  (__v16sf) __W,
						  (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_load_ps (__mmask16 __U, void const *__P)
{
  return (__m512) __builtin_ia32_loadaps512_mask ((const __v16sf *) __P,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_store_ps (void *__P, __m512 __A)
{
  *(__m512 *) __P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_store_ps (void *__P, __mmask16 __U, __m512 __A)
{
  __builtin_ia32_storeaps512_mask ((__v16sf *) __P, (__v16sf) __A,
				   (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mov_epi64 (__m512i __W, __mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdqa64_512_mask ((__v8di) __A,
						     (__v8di) __W,
						     (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mov_epi64 (__mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdqa64_512_mask ((__v8di) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_load_epi64 (void const *__P)
{
  return *(__m512i *) __P;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_load_epi64 (__m512i __W, __mmask8 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_movdqa64load512_mask ((const __v8di *) __P,
							(__v8di) __W,
							(__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_load_epi64 (__mmask8 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_movdqa64load512_mask ((const __v8di *) __P,
							(__v8di)
							_mm512_setzero_si512 (),
							(__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_store_epi64 (void *__P, __m512i __A)
{
  *(__m512i *) __P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_store_epi64 (void *__P, __mmask8 __U, __m512i __A)
{
  __builtin_ia32_movdqa64store512_mask ((__v8di *) __P, (__v8di) __A,
					(__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mov_epi32 (__m512i __W, __mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdqa32_512_mask ((__v16si) __A,
						     (__v16si) __W,
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mov_epi32 (__mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_movdqa32_512_mask ((__v16si) __A,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_load_si512 (void const *__P)
{
  return *(__m512i *) __P;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_load_epi32 (void const *__P)
{
  return *(__m512i *) __P;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_load_epi32 (__m512i __W, __mmask16 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_movdqa32load512_mask ((const __v16si *) __P,
							(__v16si) __W,
							(__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_load_epi32 (__mmask16 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_movdqa32load512_mask ((const __v16si *) __P,
							(__v16si)
							_mm512_setzero_si512 (),
							(__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_store_si512 (void *__P, __m512i __A)
{
  *(__m512i *) __P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_store_epi32 (void *__P, __m512i __A)
{
  *(__m512i *) __P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_store_epi32 (void *__P, __mmask16 __U, __m512i __A)
{
  __builtin_ia32_movdqa32store512_mask ((__v16si *) __P, (__v16si) __A,
					(__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mullo_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A * (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mullo_epi32 (__mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulld512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mullo_epi32 (__m512i __W, __mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmulld512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mullox_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v8du) __A * (__v8du) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mullox_epi64 (__m512i __W, __mmask8 __M, __m512i __A, __m512i __B)
{
  return _mm512_mask_mov_epi64 (__W, __M, _mm512_mullox_epi64 (__A, __B));
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sllv_epi32 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psllv16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sllv_epi32 (__m512i __W, __mmask16 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psllv16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sllv_epi32 (__mmask16 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psllv16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srav_epi32 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrav16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srav_epi32 (__m512i __W, __mmask16 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrav16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srav_epi32 (__mmask16 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrav16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srlv_epi32 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrlv16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srlv_epi32 (__m512i __W, __mmask16 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrlv16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srlv_epi32 (__mmask16 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrlv16si_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v8du) __A + (__v8du) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v8du) __A - (__v8du) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sllv_epi64 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psllv8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di)
						 _mm512_undefined_pd (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sllv_epi64 (__m512i __W, __mmask8 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psllv8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sllv_epi64 (__mmask8 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psllv8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srav_epi64 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrav8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srav_epi64 (__m512i __W, __mmask8 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrav8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srav_epi64 (__mmask8 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrav8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srlv_epi64 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrlv8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srlv_epi64 (__m512i __W, __mmask8 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrlv8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srlv_epi64 (__mmask8 __U, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_psrlv8di_mask ((__v8di) __X,
						 (__v8di) __Y,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A + (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddd512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_paddd512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mul_epi32 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmuldq512_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mul_epi32 (__m512i __W, __mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmuldq512_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v8di) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mul_epi32 (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmuldq512_mask ((__v16si) __X,
						  (__v16si) __Y,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A - (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubd512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_psubd512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mul_epu32 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmuludq512_mask ((__v16si) __X,
						   (__v16si) __Y,
						   (__v8di)
						   _mm512_undefined_epi32 (),
						   (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mul_epu32 (__m512i __W, __mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmuludq512_mask ((__v16si) __X,
						   (__v16si) __Y,
						   (__v8di) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mul_epu32 (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_pmuludq512_mask ((__v16si) __X,
						   (__v16si) __Y,
						   (__v8di)
						   _mm512_setzero_si512 (),
						   __M);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_slli_epi64 (__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psllqi512_mask ((__v8di) __A, __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_slli_epi64 (__m512i __W, __mmask8 __U, __m512i __A,
			unsigned int __B)
{
  return (__m512i) __builtin_ia32_psllqi512_mask ((__v8di) __A, __B,
						  (__v8di) __W,
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_slli_epi64 (__mmask8 __U, __m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psllqi512_mask ((__v8di) __A, __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __U);
}
#else
#define _mm512_slli_epi64(X, C)						   \
  ((__m512i) __builtin_ia32_psllqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask8)-1))

#define _mm512_mask_slli_epi64(W, U, X, C)				   \
  ((__m512i) __builtin_ia32_psllqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_slli_epi64(U, X, C)                                   \
  ((__m512i) __builtin_ia32_psllqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)_mm512_setzero_si512 (),\
    (__mmask8)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sll_epi64 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psllq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sll_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psllq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sll_epi64 (__mmask8 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psllq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srli_epi64 (__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psrlqi512_mask ((__v8di) __A, __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srli_epi64 (__m512i __W, __mmask8 __U,
			__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psrlqi512_mask ((__v8di) __A, __B,
						  (__v8di) __W,
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srli_epi64 (__mmask8 __U, __m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psrlqi512_mask ((__v8di) __A, __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __U);
}
#else
#define _mm512_srli_epi64(X, C)						   \
  ((__m512i) __builtin_ia32_psrlqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask8)-1))

#define _mm512_mask_srli_epi64(W, U, X, C)				   \
  ((__m512i) __builtin_ia32_psrlqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_srli_epi64(U, X, C)                                   \
  ((__m512i) __builtin_ia32_psrlqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)_mm512_setzero_si512 (),\
    (__mmask8)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srl_epi64 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrlq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srl_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrlq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srl_epi64 (__mmask8 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrlq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srai_epi64 (__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psraqi512_mask ((__v8di) __A, __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srai_epi64 (__m512i __W, __mmask8 __U, __m512i __A,
			unsigned int __B)
{
  return (__m512i) __builtin_ia32_psraqi512_mask ((__v8di) __A, __B,
						  (__v8di) __W,
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srai_epi64 (__mmask8 __U, __m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psraqi512_mask ((__v8di) __A, __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __U);
}
#else
#define _mm512_srai_epi64(X, C)						   \
  ((__m512i) __builtin_ia32_psraqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask8)-1))

#define _mm512_mask_srai_epi64(W, U, X, C)				   \
  ((__m512i) __builtin_ia32_psraqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_srai_epi64(U, X, C)				   \
  ((__m512i) __builtin_ia32_psraqi512_mask ((__v8di)(__m512i)(X), (int)(C),\
    (__v8di)(__m512i)_mm512_setzero_si512 (),\
    (__mmask8)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sra_epi64 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psraq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sra_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psraq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sra_epi64 (__mmask8 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psraq512_mask ((__v8di) __A,
						 (__v2di) __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_slli_epi32 (__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_pslldi512_mask ((__v16si) __A, __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_slli_epi32 (__m512i __W, __mmask16 __U, __m512i __A,
			unsigned int __B)
{
  return (__m512i) __builtin_ia32_pslldi512_mask ((__v16si) __A, __B,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_slli_epi32 (__mmask16 __U, __m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_pslldi512_mask ((__v16si) __A, __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}
#else
#define _mm512_slli_epi32(X, C)						    \
  ((__m512i) __builtin_ia32_pslldi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask16)-1))

#define _mm512_mask_slli_epi32(W, U, X, C)                                  \
  ((__m512i) __builtin_ia32_pslldi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_slli_epi32(U, X, C)                                    \
  ((__m512i) __builtin_ia32_pslldi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_setzero_si512 (),\
    (__mmask16)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sll_epi32 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_pslld512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sll_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_pslld512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sll_epi32 (__mmask16 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_pslld512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srli_epi32 (__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psrldi512_mask ((__v16si) __A, __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srli_epi32 (__m512i __W, __mmask16 __U,
			__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psrldi512_mask ((__v16si) __A, __B,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srli_epi32 (__mmask16 __U, __m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psrldi512_mask ((__v16si) __A, __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}
#else
#define _mm512_srli_epi32(X, C)						    \
  ((__m512i) __builtin_ia32_psrldi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask16)-1))

#define _mm512_mask_srli_epi32(W, U, X, C)                                  \
  ((__m512i) __builtin_ia32_psrldi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_srli_epi32(U, X, C)				    \
  ((__m512i) __builtin_ia32_psrldi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_setzero_si512 (),\
    (__mmask16)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srl_epi32 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrld512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srl_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrld512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srl_epi32 (__mmask16 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrld512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_srai_epi32 (__m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psradi512_mask ((__v16si) __A, __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_srai_epi32 (__m512i __W, __mmask16 __U, __m512i __A,
			unsigned int __B)
{
  return (__m512i) __builtin_ia32_psradi512_mask ((__v16si) __A, __B,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_srai_epi32 (__mmask16 __U, __m512i __A, unsigned int __B)
{
  return (__m512i) __builtin_ia32_psradi512_mask ((__v16si) __A, __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}
#else
#define _mm512_srai_epi32(X, C)						    \
  ((__m512i) __builtin_ia32_psradi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask16)-1))

#define _mm512_mask_srai_epi32(W, U, X, C)				    \
  ((__m512i) __builtin_ia32_psradi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_srai_epi32(U, X, C)				    \
  ((__m512i) __builtin_ia32_psradi512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_setzero_si512 (),\
    (__mmask16)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sra_epi32 (__m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrad512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sra_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrad512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sra_epi32 (__mmask16 __U, __m512i __A, __m128i __B)
{
  return (__m512i) __builtin_ia32_psrad512_mask ((__v16si) __A,
						 (__v4si) __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_add_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_addsd_round ((__v2df) __A,
					       (__v2df) __B,
					       __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_addsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_addsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_add_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_addss_round ((__v4sf) __A,
					      (__v4sf) __B,
					      __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_addss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			   const int __R)
{
  return (__m128) __builtin_ia32_addss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sub_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_subsd_round ((__v2df) __A,
					       (__v2df) __B,
					       __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_subsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_subsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sub_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_subss_round ((__v4sf) __A,
					      (__v4sf) __B,
					      __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_subss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			   const int __R)
{
  return (__m128) __builtin_ia32_subss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U, __R);
}

#else
#define _mm_add_round_sd(A, B, C)            \
    (__m128d)__builtin_ia32_addsd_round(A, B, C)

#define _mm_mask_add_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_addsd_mask_round(A, B, W, U, C)

#define _mm_maskz_add_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_addsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U, C)

#define _mm_add_round_ss(A, B, C)            \
    (__m128)__builtin_ia32_addss_round(A, B, C)

#define _mm_mask_add_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_addss_mask_round(A, B, W, U, C)

#define _mm_maskz_add_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_addss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U, C)

#define _mm_sub_round_sd(A, B, C)            \
    (__m128d)__builtin_ia32_subsd_round(A, B, C)

#define _mm_mask_sub_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_subsd_mask_round(A, B, W, U, C)

#define _mm_maskz_sub_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_subsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U, C)

#define _mm_sub_round_ss(A, B, C)            \
    (__m128)__builtin_ia32_subss_round(A, B, C)

#define _mm_mask_sub_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_subss_mask_round(A, B, W, U, C)

#define _mm_maskz_sub_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_subss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U, C)

#endif

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_ternarylogic_epi64 (__m512i __A, __m512i __B, __m512i __C,
			   const int __imm)
{
  return (__m512i) __builtin_ia32_pternlogq512_mask ((__v8di) __A,
						     (__v8di) __B,
						     (__v8di) __C, __imm,
						     (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_ternarylogic_epi64 (__m512i __A, __mmask8 __U, __m512i __B,
				__m512i __C, const int __imm)
{
  return (__m512i) __builtin_ia32_pternlogq512_mask ((__v8di) __A,
						     (__v8di) __B,
						     (__v8di) __C, __imm,
						     (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_ternarylogic_epi64 (__mmask8 __U, __m512i __A, __m512i __B,
				 __m512i __C, const int __imm)
{
  return (__m512i) __builtin_ia32_pternlogq512_maskz ((__v8di) __A,
						      (__v8di) __B,
						      (__v8di) __C,
						      __imm, (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_ternarylogic_epi32 (__m512i __A, __m512i __B, __m512i __C,
			   const int __imm)
{
  return (__m512i) __builtin_ia32_pternlogd512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si) __C,
						     __imm, (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_ternarylogic_epi32 (__m512i __A, __mmask16 __U, __m512i __B,
				__m512i __C, const int __imm)
{
  return (__m512i) __builtin_ia32_pternlogd512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si) __C,
						     __imm, (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_ternarylogic_epi32 (__mmask16 __U, __m512i __A, __m512i __B,
				 __m512i __C, const int __imm)
{
  return (__m512i) __builtin_ia32_pternlogd512_maskz ((__v16si) __A,
						      (__v16si) __B,
						      (__v16si) __C,
						      __imm, (__mmask16) __U);
}
#else
#define _mm512_ternarylogic_epi64(A, B, C, I)				\
  ((__m512i) __builtin_ia32_pternlogq512_mask ((__v8di)(__m512i)(A),	\
    (__v8di)(__m512i)(B), (__v8di)(__m512i)(C), (int)(I), (__mmask8)-1))
#define _mm512_mask_ternarylogic_epi64(A, U, B, C, I)			\
  ((__m512i) __builtin_ia32_pternlogq512_mask ((__v8di)(__m512i)(A),	\
    (__v8di)(__m512i)(B), (__v8di)(__m512i)(C), (int)(I), (__mmask8)(U)))
#define _mm512_maskz_ternarylogic_epi64(U, A, B, C, I)			\
  ((__m512i) __builtin_ia32_pternlogq512_maskz ((__v8di)(__m512i)(A),	\
    (__v8di)(__m512i)(B), (__v8di)(__m512i)(C), (int)(I), (__mmask8)(U)))
#define _mm512_ternarylogic_epi32(A, B, C, I)				\
  ((__m512i) __builtin_ia32_pternlogd512_mask ((__v16si)(__m512i)(A),	\
    (__v16si)(__m512i)(B), (__v16si)(__m512i)(C), (int)(I),		\
    (__mmask16)-1))
#define _mm512_mask_ternarylogic_epi32(A, U, B, C, I)			\
  ((__m512i) __builtin_ia32_pternlogd512_mask ((__v16si)(__m512i)(A),	\
    (__v16si)(__m512i)(B), (__v16si)(__m512i)(C), (int)(I),		\
    (__mmask16)(U)))
#define _mm512_maskz_ternarylogic_epi32(U, A, B, C, I)			\
  ((__m512i) __builtin_ia32_pternlogd512_maskz ((__v16si)(__m512i)(A),	\
    (__v16si)(__m512i)(B), (__v16si)(__m512i)(C), (int)(I),		\
    (__mmask16)(U)))
#endif

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rcp14_pd (__m512d __A)
{
  return (__m512d) __builtin_ia32_rcp14pd512_mask ((__v8df) __A,
						   (__v8df)
						   _mm512_undefined_pd (),
						   (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rcp14_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_rcp14pd512_mask ((__v8df) __A,
						   (__v8df) __W,
						   (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rcp14_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_rcp14pd512_mask ((__v8df) __A,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rcp14_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_rcp14ps512_mask ((__v16sf) __A,
						  (__v16sf)
						  _mm512_undefined_ps (),
						  (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rcp14_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_rcp14ps512_mask ((__v16sf) __A,
						  (__v16sf) __W,
						  (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rcp14_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_rcp14ps512_mask ((__v16sf) __A,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rcp14_sd (__m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_rcp14sd ((__v2df) __B,
					   (__v2df) __A);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rcp14_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_rcp14sd_mask ((__v2df) __B,
						(__v2df) __A,
						(__v2df) __W,
						(__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rcp14_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_rcp14sd_mask ((__v2df) __B,
						(__v2df) __A,
						(__v2df) _mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rcp14_ss (__m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_rcp14ss ((__v4sf) __B,
					  (__v4sf) __A);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rcp14_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_rcp14ss_mask ((__v4sf) __B,
						(__v4sf) __A,
						(__v4sf) __W,
						(__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rcp14_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_rcp14ss_mask ((__v4sf) __B,
						(__v4sf) __A,
						(__v4sf) _mm_setzero_ps (),
						(__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rsqrt14_pd (__m512d __A)
{
  return (__m512d) __builtin_ia32_rsqrt14pd512_mask ((__v8df) __A,
						     (__v8df)
						     _mm512_undefined_pd (),
						     (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rsqrt14_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_rsqrt14pd512_mask ((__v8df) __A,
						     (__v8df) __W,
						     (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rsqrt14_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_rsqrt14pd512_mask ((__v8df) __A,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rsqrt14_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_rsqrt14ps512_mask ((__v16sf) __A,
						    (__v16sf)
						    _mm512_undefined_ps (),
						    (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rsqrt14_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_rsqrt14ps512_mask ((__v16sf) __A,
						    (__v16sf) __W,
						    (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rsqrt14_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_rsqrt14ps512_mask ((__v16sf) __A,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rsqrt14_sd (__m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_rsqrt14sd ((__v2df) __B,
					     (__v2df) __A);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rsqrt14_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_rsqrt14sd_mask ((__v2df) __B,
						 (__v2df) __A,
						 (__v2df) __W,
						 (__mmask8) __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rsqrt14_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_rsqrt14sd_mask ((__v2df) __B,
						 (__v2df) __A,
						 (__v2df) _mm_setzero_pd (),
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_rsqrt14_ss (__m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_rsqrt14ss ((__v4sf) __B,
					    (__v4sf) __A);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_rsqrt14_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_rsqrt14ss_mask ((__v4sf) __B,
						 (__v4sf) __A,
						 (__v4sf) __W,
						 (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_rsqrt14_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_rsqrt14ss_mask ((__v4sf) __B,
						(__v4sf) __A,
						(__v4sf) _mm_setzero_ps (),
						(__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sqrt_round_pd (__m512d __A, const int __R)
{
  return (__m512d) __builtin_ia32_sqrtpd512_mask ((__v8df) __A,
						  (__v8df)
						  _mm512_undefined_pd (),
						  (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sqrt_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			   const int __R)
{
  return (__m512d) __builtin_ia32_sqrtpd512_mask ((__v8df) __A,
						  (__v8df) __W,
						  (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sqrt_round_pd (__mmask8 __U, __m512d __A, const int __R)
{
  return (__m512d) __builtin_ia32_sqrtpd512_mask ((__v8df) __A,
						  (__v8df)
						  _mm512_setzero_pd (),
						  (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sqrt_round_ps (__m512 __A, const int __R)
{
  return (__m512) __builtin_ia32_sqrtps512_mask ((__v16sf) __A,
						 (__v16sf)
						 _mm512_undefined_ps (),
						 (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sqrt_round_ps (__m512 __W, __mmask16 __U, __m512 __A, const int __R)
{
  return (__m512) __builtin_ia32_sqrtps512_mask ((__v16sf) __A,
						 (__v16sf) __W,
						 (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sqrt_round_ps (__mmask16 __U, __m512 __A, const int __R)
{
  return (__m512) __builtin_ia32_sqrtps512_mask ((__v16sf) __A,
						 (__v16sf)
						 _mm512_setzero_ps (),
						 (__mmask16) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sqrt_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_sqrtsd_mask_round ((__v2df) __B,
						     (__v2df) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) -1, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sqrt_round_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			const int __R)
{
  return (__m128d) __builtin_ia32_sqrtsd_mask_round ((__v2df) __B,
						     (__v2df) __A,
						     (__v2df) __W,
						     (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sqrt_round_sd (__mmask8 __U, __m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_sqrtsd_mask_round ((__v2df) __B,
						     (__v2df) __A,
						     (__v2df)
						     _mm_setzero_pd (),
						     (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sqrt_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_sqrtss_mask_round ((__v4sf) __B,
						    (__v4sf) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) -1, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sqrt_round_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			const int __R)
{
  return (__m128) __builtin_ia32_sqrtss_mask_round ((__v4sf) __B,
						    (__v4sf) __A,
						    (__v4sf) __W,
						    (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sqrt_round_ss (__mmask8 __U, __m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_sqrtss_mask_round ((__v4sf) __B,
						    (__v4sf) __A,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U, __R);
}
#else
#define _mm512_sqrt_round_pd(A, C)            \
    (__m512d)__builtin_ia32_sqrtpd512_mask(A, (__v8df)_mm512_undefined_pd(), -1, C)

#define _mm512_mask_sqrt_round_pd(W, U, A, C) \
    (__m512d)__builtin_ia32_sqrtpd512_mask(A, W, U, C)

#define _mm512_maskz_sqrt_round_pd(U, A, C)   \
    (__m512d)__builtin_ia32_sqrtpd512_mask(A, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_sqrt_round_ps(A, C)            \
    (__m512)__builtin_ia32_sqrtps512_mask(A, (__v16sf)_mm512_undefined_ps(), -1, C)

#define _mm512_mask_sqrt_round_ps(W, U, A, C) \
    (__m512)__builtin_ia32_sqrtps512_mask(A, W, U, C)

#define _mm512_maskz_sqrt_round_ps(U, A, C)   \
    (__m512)__builtin_ia32_sqrtps512_mask(A, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm_sqrt_round_sd(A, B, C)	      \
    (__m128d)__builtin_ia32_sqrtsd_mask_round (B, A, \
	(__v2df) _mm_setzero_pd (), -1, C)

#define _mm_mask_sqrt_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_sqrtsd_mask_round (B, A, W, U, C)

#define _mm_maskz_sqrt_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_sqrtsd_mask_round (B, A, \
	(__v2df) _mm_setzero_pd (), U, C)

#define _mm_sqrt_round_ss(A, B, C)	      \
    (__m128)__builtin_ia32_sqrtss_mask_round (B, A, \
	(__v4sf) _mm_setzero_ps (), -1, C)

#define _mm_mask_sqrt_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_sqrtss_mask_round (B, A, W, U, C)

#define _mm_maskz_sqrt_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_sqrtss_mask_round (B, A, \
	(__v4sf) _mm_setzero_ps (), U, C)
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi8_epi32 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbd512_mask ((__v16qi) __A,
						    (__v16si)
						    _mm512_undefined_epi32 (),
						    (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi8_epi32 (__m512i __W, __mmask16 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbd512_mask ((__v16qi) __A,
						    (__v16si) __W,
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi8_epi32 (__mmask16 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbd512_mask ((__v16qi) __A,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi8_epi64 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbq512_mask ((__v16qi) __A,
						    (__v8di)
						    _mm512_undefined_epi32 (),
						    (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi8_epi64 (__m512i __W, __mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbq512_mask ((__v16qi) __A,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi8_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxbq512_mask ((__v16qi) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi16_epi32 (__m256i __A)
{
  return (__m512i) __builtin_ia32_pmovsxwd512_mask ((__v16hi) __A,
						    (__v16si)
						    _mm512_undefined_epi32 (),
						    (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi16_epi32 (__m512i __W, __mmask16 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovsxwd512_mask ((__v16hi) __A,
						    (__v16si) __W,
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi16_epi32 (__mmask16 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovsxwd512_mask ((__v16hi) __A,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi16_epi64 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxwq512_mask ((__v8hi) __A,
						    (__v8di)
						    _mm512_undefined_epi32 (),
						    (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi16_epi64 (__m512i __W, __mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxwq512_mask ((__v8hi) __A,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi16_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovsxwq512_mask ((__v8hi) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi32_epi64 (__m256i __X)
{
  return (__m512i) __builtin_ia32_pmovsxdq512_mask ((__v8si) __X,
						    (__v8di)
						    _mm512_undefined_epi32 (),
						    (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi32_epi64 (__m512i __W, __mmask8 __U, __m256i __X)
{
  return (__m512i) __builtin_ia32_pmovsxdq512_mask ((__v8si) __X,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi32_epi64 (__mmask8 __U, __m256i __X)
{
  return (__m512i) __builtin_ia32_pmovsxdq512_mask ((__v8si) __X,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu8_epi32 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbd512_mask ((__v16qi) __A,
						    (__v16si)
						    _mm512_undefined_epi32 (),
						    (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu8_epi32 (__m512i __W, __mmask16 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbd512_mask ((__v16qi) __A,
						    (__v16si) __W,
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu8_epi32 (__mmask16 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbd512_mask ((__v16qi) __A,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu8_epi64 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbq512_mask ((__v16qi) __A,
						    (__v8di)
						    _mm512_undefined_epi32 (),
						    (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu8_epi64 (__m512i __W, __mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbq512_mask ((__v16qi) __A,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu8_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxbq512_mask ((__v16qi) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu16_epi32 (__m256i __A)
{
  return (__m512i) __builtin_ia32_pmovzxwd512_mask ((__v16hi) __A,
						    (__v16si)
						    _mm512_undefined_epi32 (),
						    (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu16_epi32 (__m512i __W, __mmask16 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovzxwd512_mask ((__v16hi) __A,
						    (__v16si) __W,
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu16_epi32 (__mmask16 __U, __m256i __A)
{
  return (__m512i) __builtin_ia32_pmovzxwd512_mask ((__v16hi) __A,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu16_epi64 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxwq512_mask ((__v8hi) __A,
						    (__v8di)
						    _mm512_undefined_epi32 (),
						    (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu16_epi64 (__m512i __W, __mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxwq512_mask ((__v8hi) __A,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu16_epi64 (__mmask8 __U, __m128i __A)
{
  return (__m512i) __builtin_ia32_pmovzxwq512_mask ((__v8hi) __A,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu32_epi64 (__m256i __X)
{
  return (__m512i) __builtin_ia32_pmovzxdq512_mask ((__v8si) __X,
						    (__v8di)
						    _mm512_undefined_epi32 (),
						    (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu32_epi64 (__m512i __W, __mmask8 __U, __m256i __X)
{
  return (__m512i) __builtin_ia32_pmovzxdq512_mask ((__v8si) __X,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu32_epi64 (__mmask8 __U, __m256i __X)
{
  return (__m512i) __builtin_ia32_pmovzxdq512_mask ((__v8si) __X,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_round_pd (__m512d __A, __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_addpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			  __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_addpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			   const int __R)
{
  return (__m512d) __builtin_ia32_addpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_round_ps (__m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_addps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			  __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_addps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_round_ps (__mmask16 __U, __m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_addps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_round_pd (__m512d __A, __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_subpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			  __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_subpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			   const int __R)
{
  return (__m512d) __builtin_ia32_subpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_round_ps (__m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_subps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			  __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_subps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_round_ps (__mmask16 __U, __m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_subps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U, __R);
}
#else
#define _mm512_add_round_pd(A, B, C)            \
    (__m512d)__builtin_ia32_addpd512_mask(A, B, (__v8df)_mm512_undefined_pd(), -1, C)

#define _mm512_mask_add_round_pd(W, U, A, B, C) \
    (__m512d)__builtin_ia32_addpd512_mask(A, B, W, U, C)

#define _mm512_maskz_add_round_pd(U, A, B, C)   \
    (__m512d)__builtin_ia32_addpd512_mask(A, B, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_add_round_ps(A, B, C)            \
    (__m512)__builtin_ia32_addps512_mask(A, B, (__v16sf)_mm512_undefined_ps(), -1, C)

#define _mm512_mask_add_round_ps(W, U, A, B, C) \
    (__m512)__builtin_ia32_addps512_mask(A, B, W, U, C)

#define _mm512_maskz_add_round_ps(U, A, B, C)   \
    (__m512)__builtin_ia32_addps512_mask(A, B, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm512_sub_round_pd(A, B, C)            \
    (__m512d)__builtin_ia32_subpd512_mask(A, B, (__v8df)_mm512_undefined_pd(), -1, C)

#define _mm512_mask_sub_round_pd(W, U, A, B, C) \
    (__m512d)__builtin_ia32_subpd512_mask(A, B, W, U, C)

#define _mm512_maskz_sub_round_pd(U, A, B, C)   \
    (__m512d)__builtin_ia32_subpd512_mask(A, B, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_sub_round_ps(A, B, C)            \
    (__m512)__builtin_ia32_subps512_mask(A, B, (__v16sf)_mm512_undefined_ps(), -1, C)

#define _mm512_mask_sub_round_ps(W, U, A, B, C) \
    (__m512)__builtin_ia32_subps512_mask(A, B, W, U, C)

#define _mm512_maskz_sub_round_ps(U, A, B, C)   \
    (__m512)__builtin_ia32_subps512_mask(A, B, (__v16sf)_mm512_setzero_ps(), U, C)
#endif

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mul_round_pd (__m512d __A, __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_mulpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mul_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			  __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_mulpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mul_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			   const int __R)
{
  return (__m512d) __builtin_ia32_mulpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mul_round_ps (__m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_mulps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mul_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			  __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_mulps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mul_round_ps (__mmask16 __U, __m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_mulps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_div_round_pd (__m512d __M, __m512d __V, const int __R)
{
  return (__m512d) __builtin_ia32_divpd512_mask ((__v8df) __M,
						 (__v8df) __V,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_div_round_pd (__m512d __W, __mmask8 __U, __m512d __M,
			  __m512d __V, const int __R)
{
  return (__m512d) __builtin_ia32_divpd512_mask ((__v8df) __M,
						 (__v8df) __V,
						 (__v8df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_div_round_pd (__mmask8 __U, __m512d __M, __m512d __V,
			   const int __R)
{
  return (__m512d) __builtin_ia32_divpd512_mask ((__v8df) __M,
						 (__v8df) __V,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_div_round_ps (__m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_divps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_div_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			  __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_divps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_div_round_ps (__mmask16 __U, __m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_divps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mul_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_mulsd_round ((__v2df) __A,
					       (__v2df) __B,
					       __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_mulsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_mulsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mul_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_mulss_round ((__v4sf) __A,
					      (__v4sf) __B,
					      __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_mulss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			   const int __R)
{
  return (__m128) __builtin_ia32_mulss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_div_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_divsd_round ((__v2df) __A,
					       (__v2df) __B,
					       __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_div_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_divsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_div_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_divsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_div_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_divss_round ((__v4sf) __A,
					      (__v4sf) __B,
					      __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_div_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_divss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_div_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			   const int __R)
{
  return (__m128) __builtin_ia32_divss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U, __R);
}

#else
#define _mm512_mul_round_pd(A, B, C)            \
    (__m512d)__builtin_ia32_mulpd512_mask(A, B, (__v8df)_mm512_undefined_pd(), -1, C)

#define _mm512_mask_mul_round_pd(W, U, A, B, C) \
    (__m512d)__builtin_ia32_mulpd512_mask(A, B, W, U, C)

#define _mm512_maskz_mul_round_pd(U, A, B, C)   \
    (__m512d)__builtin_ia32_mulpd512_mask(A, B, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_mul_round_ps(A, B, C)            \
    (__m512)__builtin_ia32_mulps512_mask(A, B, (__v16sf)_mm512_undefined_ps(), -1, C)

#define _mm512_mask_mul_round_ps(W, U, A, B, C) \
    (__m512)__builtin_ia32_mulps512_mask(A, B, W, U, C)

#define _mm512_maskz_mul_round_ps(U, A, B, C)   \
    (__m512)__builtin_ia32_mulps512_mask(A, B, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm512_div_round_pd(A, B, C)            \
    (__m512d)__builtin_ia32_divpd512_mask(A, B, (__v8df)_mm512_undefined_pd(), -1, C)

#define _mm512_mask_div_round_pd(W, U, A, B, C) \
    (__m512d)__builtin_ia32_divpd512_mask(A, B, W, U, C)

#define _mm512_maskz_div_round_pd(U, A, B, C)   \
    (__m512d)__builtin_ia32_divpd512_mask(A, B, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_div_round_ps(A, B, C)            \
    (__m512)__builtin_ia32_divps512_mask(A, B, (__v16sf)_mm512_undefined_ps(), -1, C)

#define _mm512_mask_div_round_ps(W, U, A, B, C) \
    (__m512)__builtin_ia32_divps512_mask(A, B, W, U, C)

#define _mm512_maskz_div_round_ps(U, A, B, C)   \
    (__m512)__builtin_ia32_divps512_mask(A, B, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm_mul_round_sd(A, B, C)            \
    (__m128d)__builtin_ia32_mulsd_round(A, B, C)

#define _mm_mask_mul_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_mulsd_mask_round(A, B, W, U, C)

#define _mm_maskz_mul_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_mulsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U, C)

#define _mm_mul_round_ss(A, B, C)            \
    (__m128)__builtin_ia32_mulss_round(A, B, C)

#define _mm_mask_mul_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_mulss_mask_round(A, B, W, U, C)

#define _mm_maskz_mul_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_mulss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U, C)

#define _mm_div_round_sd(A, B, C)            \
    (__m128d)__builtin_ia32_divsd_round(A, B, C)

#define _mm_mask_div_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_divsd_mask_round(A, B, W, U, C)

#define _mm_maskz_div_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_divsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U, C)

#define _mm_div_round_ss(A, B, C)            \
    (__m128)__builtin_ia32_divss_round(A, B, C)

#define _mm_mask_div_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_divss_mask_round(A, B, W, U, C)

#define _mm_maskz_div_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_divss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U, C)

#endif

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_round_pd (__m512d __A, __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_maxpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			  __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_maxpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			   const int __R)
{
  return (__m512d) __builtin_ia32_maxpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_round_ps (__m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_maxps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			  __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_maxps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_round_ps (__mmask16 __U, __m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_maxps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_round_pd (__m512d __A, __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_minpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			  __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_minpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			   const int __R)
{
  return (__m512d) __builtin_ia32_minpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_round_ps (__m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_minps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			  __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_minps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_round_ps (__mmask16 __U, __m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_minps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U, __R);
}
#else
#define _mm512_max_round_pd(A, B,  R) \
    (__m512d)__builtin_ia32_maxpd512_mask(A, B, (__v8df)_mm512_undefined_pd(), -1, R)

#define _mm512_mask_max_round_pd(W, U,  A, B, R) \
    (__m512d)__builtin_ia32_maxpd512_mask(A, B, W, U, R)

#define _mm512_maskz_max_round_pd(U, A,  B, R) \
    (__m512d)__builtin_ia32_maxpd512_mask(A, B, (__v8df)_mm512_setzero_pd(), U, R)

#define _mm512_max_round_ps(A, B,  R) \
    (__m512)__builtin_ia32_maxps512_mask(A, B, (__v16sf)_mm512_undefined_pd(), -1, R)

#define _mm512_mask_max_round_ps(W, U,  A, B, R) \
    (__m512)__builtin_ia32_maxps512_mask(A, B, W, U, R)

#define _mm512_maskz_max_round_ps(U, A,  B, R) \
    (__m512)__builtin_ia32_maxps512_mask(A, B, (__v16sf)_mm512_setzero_ps(), U, R)

#define _mm512_min_round_pd(A, B,  R) \
    (__m512d)__builtin_ia32_minpd512_mask(A, B, (__v8df)_mm512_undefined_pd(), -1, R)

#define _mm512_mask_min_round_pd(W, U,  A, B, R) \
    (__m512d)__builtin_ia32_minpd512_mask(A, B, W, U, R)

#define _mm512_maskz_min_round_pd(U, A,  B, R) \
    (__m512d)__builtin_ia32_minpd512_mask(A, B, (__v8df)_mm512_setzero_pd(), U, R)

#define _mm512_min_round_ps(A, B, R) \
    (__m512)__builtin_ia32_minps512_mask(A, B, (__v16sf)_mm512_undefined_ps(), -1, R)

#define _mm512_mask_min_round_ps(W, U,  A, B, R) \
    (__m512)__builtin_ia32_minps512_mask(A, B, W, U, R)

#define _mm512_maskz_min_round_ps(U, A,  B, R) \
    (__m512)__builtin_ia32_minps512_mask(A, B, (__v16sf)_mm512_setzero_ps(), U, R)
#endif

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_scalef_round_pd (__m512d __A, __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_scalefpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_scalef_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			     __m512d __B, const int __R)
{
  return (__m512d) __builtin_ia32_scalefpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __W,
						    (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_scalef_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			      const int __R)
{
  return (__m512d) __builtin_ia32_scalefpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_scalef_round_ps (__m512 __A, __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_scalefps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_scalef_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			     __m512 __B, const int __R)
{
  return (__m512) __builtin_ia32_scalefps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __W,
						   (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_scalef_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
			      const int __R)
{
  return (__m512) __builtin_ia32_scalefps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_scalef_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_scalefsd_mask_round ((__v2df) __A,
						       (__v2df) __B,
						       (__v2df)
						       _mm_setzero_pd (),
						       (__mmask8) -1, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_scalef_round_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			  const int __R)
{
  return (__m128d) __builtin_ia32_scalefsd_mask_round ((__v2df) __A,
						       (__v2df) __B,
						       (__v2df) __W,
						       (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_scalef_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_scalefsd_mask_round ((__v2df) __A,
						       (__v2df) __B,
						       (__v2df)
						       _mm_setzero_pd (),
						       (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_scalef_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_scalefss_mask_round ((__v4sf) __A,
						      (__v4sf) __B,
						      (__v4sf)
						      _mm_setzero_ps (),
						      (__mmask8) -1, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_scalef_round_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			 const int __R)
{
  return (__m128) __builtin_ia32_scalefss_mask_round ((__v4sf) __A,
						      (__v4sf) __B,
						      (__v4sf) __W,
						      (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_scalef_round_ss (__mmask8 __U, __m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_scalefss_mask_round ((__v4sf) __A,
						      (__v4sf) __B,
						      (__v4sf)
						      _mm_setzero_ps (),
						      (__mmask8) __U, __R);
}
#else
#define _mm512_scalef_round_pd(A, B, C)            \
    (__m512d)__builtin_ia32_scalefpd512_mask(A, B, (__v8df)_mm512_undefined_pd(), -1, C)

#define _mm512_mask_scalef_round_pd(W, U, A, B, C) \
    (__m512d)__builtin_ia32_scalefpd512_mask(A, B, W, U, C)

#define _mm512_maskz_scalef_round_pd(U, A, B, C)   \
    (__m512d)__builtin_ia32_scalefpd512_mask(A, B, (__v8df)_mm512_setzero_pd(), U, C)

#define _mm512_scalef_round_ps(A, B, C)            \
    (__m512)__builtin_ia32_scalefps512_mask(A, B, (__v16sf)_mm512_undefined_ps(), -1, C)

#define _mm512_mask_scalef_round_ps(W, U, A, B, C) \
    (__m512)__builtin_ia32_scalefps512_mask(A, B, W, U, C)

#define _mm512_maskz_scalef_round_ps(U, A, B, C)   \
    (__m512)__builtin_ia32_scalefps512_mask(A, B, (__v16sf)_mm512_setzero_ps(), U, C)

#define _mm_scalef_round_sd(A, B, C)            \
    (__m128d)__builtin_ia32_scalefsd_mask_round (A, B, \
	(__v2df)_mm_setzero_pd (), -1, C)

#define _mm_scalef_round_ss(A, B, C)            \
    (__m128)__builtin_ia32_scalefss_mask_round (A, B, \
	(__v4sf)_mm_setzero_ps (), -1, C)
#endif

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmadd_round_pd (__m512d __A, __m512d __B, __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmadd_round_pd (__m512d __A, __mmask8 __U, __m512d __B,
			    __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmadd_round_pd (__m512d __A, __m512d __B, __m512d __C,
			     __mmask8 __U, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_mask3 ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmadd_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			     __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_maskz ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmadd_round_ps (__m512 __A, __m512 __B, __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmadd_round_ps (__m512 __A, __mmask16 __U, __m512 __B,
			    __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmadd_round_ps (__m512 __A, __m512 __B, __m512 __C,
			     __mmask16 __U, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddps512_mask3 ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmadd_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
			     __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddps512_maskz ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsub_round_pd (__m512d __A, __m512d __B, __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsub_round_pd (__m512d __A, __mmask8 __U, __m512d __B,
			    __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsub_round_pd (__m512d __A, __m512d __B, __m512d __C,
			     __mmask8 __U, const int __R)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_mask3 ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsub_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			     __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_maskz ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsub_round_ps (__m512 __A, __m512 __B, __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmsubps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsub_round_ps (__m512 __A, __mmask16 __U, __m512 __B,
			    __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmsubps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsub_round_ps (__m512 __A, __m512 __B, __m512 __C,
			     __mmask16 __U, const int __R)
{
  return (__m512) __builtin_ia32_vfmsubps512_mask3 ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsub_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
			     __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmsubps512_maskz ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmaddsub_round_pd (__m512d __A, __m512d __B, __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       (__v8df) __C,
						       (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmaddsub_round_pd (__m512d __A, __mmask8 __U, __m512d __B,
			       __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       (__v8df) __C,
						       (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmaddsub_round_pd (__m512d __A, __m512d __B, __m512d __C,
				__mmask8 __U, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask3 ((__v8df) __A,
							(__v8df) __B,
							(__v8df) __C,
							(__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmaddsub_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
				__m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_maskz ((__v8df) __A,
							(__v8df) __B,
							(__v8df) __C,
							(__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmaddsub_round_ps (__m512 __A, __m512 __B, __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      (__v16sf) __C,
						      (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmaddsub_round_ps (__m512 __A, __mmask16 __U, __m512 __B,
			       __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      (__v16sf) __C,
						      (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmaddsub_round_ps (__m512 __A, __m512 __B, __m512 __C,
				__mmask16 __U, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask3 ((__v16sf) __A,
						       (__v16sf) __B,
						       (__v16sf) __C,
						       (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmaddsub_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
				__m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_maskz ((__v16sf) __A,
						       (__v16sf) __B,
						       (__v16sf) __C,
						       (__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsubadd_round_pd (__m512d __A, __m512d __B, __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       -(__v8df) __C,
						       (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsubadd_round_pd (__m512d __A, __mmask8 __U, __m512d __B,
			       __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       -(__v8df) __C,
						       (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsubadd_round_pd (__m512d __A, __m512d __B, __m512d __C,
				__mmask8 __U, const int __R)
{
  return (__m512d) __builtin_ia32_vfmsubaddpd512_mask3 ((__v8df) __A,
							(__v8df) __B,
							(__v8df) __C,
							(__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsubadd_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
				__m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_maskz ((__v8df) __A,
							(__v8df) __B,
							-(__v8df) __C,
							(__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsubadd_round_ps (__m512 __A, __m512 __B, __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      -(__v16sf) __C,
						      (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsubadd_round_ps (__m512 __A, __mmask16 __U, __m512 __B,
			       __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      -(__v16sf) __C,
						      (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsubadd_round_ps (__m512 __A, __m512 __B, __m512 __C,
				__mmask16 __U, const int __R)
{
  return (__m512) __builtin_ia32_vfmsubaddps512_mask3 ((__v16sf) __A,
						       (__v16sf) __B,
						       (__v16sf) __C,
						       (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsubadd_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
				__m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_maskz ((__v16sf) __A,
						       (__v16sf) __B,
						       -(__v16sf) __C,
						       (__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmadd_round_pd (__m512d __A, __m512d __B, __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmadd_round_pd (__m512d __A, __mmask8 __U, __m512d __B,
			     __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmadd_round_pd (__m512d __A, __m512d __B, __m512d __C,
			      __mmask8 __U, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_mask3 ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmadd_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			      __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_maskz ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmadd_round_ps (__m512 __A, __m512 __B, __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfnmaddps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmadd_round_ps (__m512 __A, __mmask16 __U, __m512 __B,
			     __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfnmaddps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmadd_round_ps (__m512 __A, __m512 __B, __m512 __C,
			      __mmask16 __U, const int __R)
{
  return (__m512) __builtin_ia32_vfnmaddps512_mask3 ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmadd_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
			      __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfnmaddps512_maskz ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmsub_round_pd (__m512d __A, __m512d __B, __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmsub_round_pd (__m512d __A, __mmask8 __U, __m512d __B,
			     __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmsub_round_pd (__m512d __A, __m512d __B, __m512d __C,
			      __mmask8 __U, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_mask3 ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmsub_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
			      __m512d __C, const int __R)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_maskz ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmsub_round_ps (__m512 __A, __m512 __B, __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfnmsubps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmsub_round_ps (__m512 __A, __mmask16 __U, __m512 __B,
			     __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfnmsubps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmsub_round_ps (__m512 __A, __m512 __B, __m512 __C,
			      __mmask16 __U, const int __R)
{
  return (__m512) __builtin_ia32_vfnmsubps512_mask3 ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmsub_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
			      __m512 __C, const int __R)
{
  return (__m512) __builtin_ia32_vfnmsubps512_maskz ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U, __R);
}
#else
#define _mm512_fmadd_round_pd(A, B, C, R)            \
    (__m512d)__builtin_ia32_vfmaddpd512_mask(A, B, C, -1, R)

#define _mm512_mask_fmadd_round_pd(A, U, B, C, R)    \
    (__m512d)__builtin_ia32_vfmaddpd512_mask(A, B, C, U, R)

#define _mm512_mask3_fmadd_round_pd(A, B, C, U, R)   \
    (__m512d)__builtin_ia32_vfmaddpd512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmadd_round_pd(U, A, B, C, R)   \
    (__m512d)__builtin_ia32_vfmaddpd512_maskz(A, B, C, U, R)

#define _mm512_fmadd_round_ps(A, B, C, R)            \
    (__m512)__builtin_ia32_vfmaddps512_mask(A, B, C, -1, R)

#define _mm512_mask_fmadd_round_ps(A, U, B, C, R)    \
    (__m512)__builtin_ia32_vfmaddps512_mask(A, B, C, U, R)

#define _mm512_mask3_fmadd_round_ps(A, B, C, U, R)   \
    (__m512)__builtin_ia32_vfmaddps512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmadd_round_ps(U, A, B, C, R)   \
    (__m512)__builtin_ia32_vfmaddps512_maskz(A, B, C, U, R)

#define _mm512_fmsub_round_pd(A, B, C, R)            \
    (__m512d)__builtin_ia32_vfmsubpd512_mask(A, B, C, -1, R)

#define _mm512_mask_fmsub_round_pd(A, U, B, C, R)    \
    (__m512d)__builtin_ia32_vfmsubpd512_mask(A, B, C, U, R)

#define _mm512_mask3_fmsub_round_pd(A, B, C, U, R)   \
    (__m512d)__builtin_ia32_vfmsubpd512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmsub_round_pd(U, A, B, C, R)   \
    (__m512d)__builtin_ia32_vfmsubpd512_maskz(A, B, C, U, R)

#define _mm512_fmsub_round_ps(A, B, C, R)            \
    (__m512)__builtin_ia32_vfmsubps512_mask(A, B, C, -1, R)

#define _mm512_mask_fmsub_round_ps(A, U, B, C, R)    \
    (__m512)__builtin_ia32_vfmsubps512_mask(A, B, C, U, R)

#define _mm512_mask3_fmsub_round_ps(A, B, C, U, R)   \
    (__m512)__builtin_ia32_vfmsubps512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmsub_round_ps(U, A, B, C, R)   \
    (__m512)__builtin_ia32_vfmsubps512_maskz(A, B, C, U, R)

#define _mm512_fmaddsub_round_pd(A, B, C, R)            \
    (__m512d)__builtin_ia32_vfmaddsubpd512_mask(A, B, C, -1, R)

#define _mm512_mask_fmaddsub_round_pd(A, U, B, C, R)    \
    (__m512d)__builtin_ia32_vfmaddsubpd512_mask(A, B, C, U, R)

#define _mm512_mask3_fmaddsub_round_pd(A, B, C, U, R)   \
    (__m512d)__builtin_ia32_vfmaddsubpd512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmaddsub_round_pd(U, A, B, C, R)   \
    (__m512d)__builtin_ia32_vfmaddsubpd512_maskz(A, B, C, U, R)

#define _mm512_fmaddsub_round_ps(A, B, C, R)            \
    (__m512)__builtin_ia32_vfmaddsubps512_mask(A, B, C, -1, R)

#define _mm512_mask_fmaddsub_round_ps(A, U, B, C, R)    \
    (__m512)__builtin_ia32_vfmaddsubps512_mask(A, B, C, U, R)

#define _mm512_mask3_fmaddsub_round_ps(A, B, C, U, R)   \
    (__m512)__builtin_ia32_vfmaddsubps512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmaddsub_round_ps(U, A, B, C, R)   \
    (__m512)__builtin_ia32_vfmaddsubps512_maskz(A, B, C, U, R)

#define _mm512_fmsubadd_round_pd(A, B, C, R)            \
    (__m512d)__builtin_ia32_vfmaddsubpd512_mask(A, B, -(C), -1, R)

#define _mm512_mask_fmsubadd_round_pd(A, U, B, C, R)    \
    (__m512d)__builtin_ia32_vfmaddsubpd512_mask(A, B, -(C), U, R)

#define _mm512_mask3_fmsubadd_round_pd(A, B, C, U, R)   \
    (__m512d)__builtin_ia32_vfmsubaddpd512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmsubadd_round_pd(U, A, B, C, R)   \
    (__m512d)__builtin_ia32_vfmaddsubpd512_maskz(A, B, -(C), U, R)

#define _mm512_fmsubadd_round_ps(A, B, C, R)            \
    (__m512)__builtin_ia32_vfmaddsubps512_mask(A, B, -(C), -1, R)

#define _mm512_mask_fmsubadd_round_ps(A, U, B, C, R)    \
    (__m512)__builtin_ia32_vfmaddsubps512_mask(A, B, -(C), U, R)

#define _mm512_mask3_fmsubadd_round_ps(A, B, C, U, R)   \
    (__m512)__builtin_ia32_vfmsubaddps512_mask3(A, B, C, U, R)

#define _mm512_maskz_fmsubadd_round_ps(U, A, B, C, R)   \
    (__m512)__builtin_ia32_vfmaddsubps512_maskz(A, B, -(C), U, R)

#define _mm512_fnmadd_round_pd(A, B, C, R)            \
    (__m512d)__builtin_ia32_vfnmaddpd512_mask(A, B, C, -1, R)

#define _mm512_mask_fnmadd_round_pd(A, U, B, C, R)    \
    (__m512d)__builtin_ia32_vfnmaddpd512_mask(A, B, C, U, R)

#define _mm512_mask3_fnmadd_round_pd(A, B, C, U, R)   \
    (__m512d)__builtin_ia32_vfnmaddpd512_mask3(A, B, C, U, R)

#define _mm512_maskz_fnmadd_round_pd(U, A, B, C, R)   \
    (__m512d)__builtin_ia32_vfnmaddpd512_maskz(A, B, C, U, R)

#define _mm512_fnmadd_round_ps(A, B, C, R)            \
    (__m512)__builtin_ia32_vfnmaddps512_mask(A, B, C, -1, R)

#define _mm512_mask_fnmadd_round_ps(A, U, B, C, R)    \
    (__m512)__builtin_ia32_vfnmaddps512_mask(A, B, C, U, R)

#define _mm512_mask3_fnmadd_round_ps(A, B, C, U, R)   \
    (__m512)__builtin_ia32_vfnmaddps512_mask3(A, B, C, U, R)

#define _mm512_maskz_fnmadd_round_ps(U, A, B, C, R)   \
    (__m512)__builtin_ia32_vfnmaddps512_maskz(A, B, C, U, R)

#define _mm512_fnmsub_round_pd(A, B, C, R)            \
    (__m512d)__builtin_ia32_vfnmsubpd512_mask(A, B, C, -1, R)

#define _mm512_mask_fnmsub_round_pd(A, U, B, C, R)    \
    (__m512d)__builtin_ia32_vfnmsubpd512_mask(A, B, C, U, R)

#define _mm512_mask3_fnmsub_round_pd(A, B, C, U, R)   \
    (__m512d)__builtin_ia32_vfnmsubpd512_mask3(A, B, C, U, R)

#define _mm512_maskz_fnmsub_round_pd(U, A, B, C, R)   \
    (__m512d)__builtin_ia32_vfnmsubpd512_maskz(A, B, C, U, R)

#define _mm512_fnmsub_round_ps(A, B, C, R)            \
    (__m512)__builtin_ia32_vfnmsubps512_mask(A, B, C, -1, R)

#define _mm512_mask_fnmsub_round_ps(A, U, B, C, R)    \
    (__m512)__builtin_ia32_vfnmsubps512_mask(A, B, C, U, R)

#define _mm512_mask3_fnmsub_round_ps(A, B, C, U, R)   \
    (__m512)__builtin_ia32_vfnmsubps512_mask3(A, B, C, U, R)

#define _mm512_maskz_fnmsub_round_ps(U, A, B, C, R)   \
    (__m512)__builtin_ia32_vfnmsubps512_maskz(A, B, C, U, R)
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_abs_epi64 (__m512i __A)
{
  return (__m512i) __builtin_ia32_pabsq512_mask ((__v8di) __A,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_abs_epi64 (__m512i __W, __mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsq512_mask ((__v8di) __A,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_abs_epi64 (__mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsq512_mask ((__v8di) __A,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_abs_epi32 (__m512i __A)
{
  return (__m512i) __builtin_ia32_pabsd512_mask ((__v16si) __A,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_abs_epi32 (__m512i __W, __mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsd512_mask ((__v16si) __A,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_abs_epi32 (__mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_pabsd512_mask ((__v16si) __A,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastss_ps (__m128 __A)
{
  return (__m512) __builtin_ia32_broadcastss512 ((__v4sf) __A,
						 (__v16sf)
						 _mm512_undefined_ps (),
						 (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcastss_ps (__m512 __O, __mmask16 __M, __m128 __A)
{
  return (__m512) __builtin_ia32_broadcastss512 ((__v4sf) __A,
						 (__v16sf) __O, __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcastss_ps (__mmask16 __M, __m128 __A)
{
  return (__m512) __builtin_ia32_broadcastss512 ((__v4sf) __A,
						 (__v16sf)
						 _mm512_setzero_ps (),
						 __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastsd_pd (__m128d __A)
{
  return (__m512d) __builtin_ia32_broadcastsd512 ((__v2df) __A,
						  (__v8df)
						  _mm512_undefined_pd (),
						  (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcastsd_pd (__m512d __O, __mmask8 __M, __m128d __A)
{
  return (__m512d) __builtin_ia32_broadcastsd512 ((__v2df) __A,
						  (__v8df) __O, __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcastsd_pd (__mmask8 __M, __m128d __A)
{
  return (__m512d) __builtin_ia32_broadcastsd512 ((__v2df) __A,
						  (__v8df)
						  _mm512_setzero_pd (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastd_epi32 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastd512 ((__v4si) __A,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcastd_epi32 (__m512i __O, __mmask16 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastd512 ((__v4si) __A,
						  (__v16si) __O, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcastd_epi32 (__mmask16 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastd512 ((__v4si) __A,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set1_epi32 (int __A)
{
  return (__m512i) __builtin_ia32_pbroadcastd512_gpr_mask (__A,
							   (__v16si)
							   _mm512_undefined_epi32 (),
							   (__mmask16)(-1));
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_set1_epi32 (__m512i __O, __mmask16 __M, int __A)
{
  return (__m512i) __builtin_ia32_pbroadcastd512_gpr_mask (__A, (__v16si) __O,
							   __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_set1_epi32 (__mmask16 __M, int __A)
{
  return (__m512i)
	 __builtin_ia32_pbroadcastd512_gpr_mask (__A,
						 (__v16si) _mm512_setzero_si512 (),
						 __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcastq_epi64 (__m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastq512 ((__v2di) __A,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcastq_epi64 (__m512i __O, __mmask8 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastq512 ((__v2di) __A,
						  (__v8di) __O, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcastq_epi64 (__mmask8 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_pbroadcastq512 ((__v2di) __A,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_set1_epi64 (long long __A)
{
  return (__m512i) __builtin_ia32_pbroadcastq512_gpr_mask (__A,
							   (__v8di)
							   _mm512_undefined_epi32 (),
							   (__mmask8)(-1));
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_set1_epi64 (__m512i __O, __mmask8 __M, long long __A)
{
  return (__m512i) __builtin_ia32_pbroadcastq512_gpr_mask (__A, (__v8di) __O,
							   __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_set1_epi64 (__mmask8 __M, long long __A)
{
  return (__m512i)
	 __builtin_ia32_pbroadcastq512_gpr_mask (__A,
						 (__v8di) _mm512_setzero_si512 (),
						 __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_f32x4 (__m128 __A)
{
  return (__m512) __builtin_ia32_broadcastf32x4_512 ((__v4sf) __A,
						     (__v16sf)
						     _mm512_undefined_ps (),
						     (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_f32x4 (__m512 __O, __mmask16 __M, __m128 __A)
{
  return (__m512) __builtin_ia32_broadcastf32x4_512 ((__v4sf) __A,
						     (__v16sf) __O,
						     __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_f32x4 (__mmask16 __M, __m128 __A)
{
  return (__m512) __builtin_ia32_broadcastf32x4_512 ((__v4sf) __A,
						     (__v16sf)
						     _mm512_setzero_ps (),
						     __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_i32x4 (__m128i __A)
{
  return (__m512i) __builtin_ia32_broadcasti32x4_512 ((__v4si) __A,
						      (__v16si)
						      _mm512_undefined_epi32 (),
						      (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_i32x4 (__m512i __O, __mmask16 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_broadcasti32x4_512 ((__v4si) __A,
						      (__v16si) __O,
						      __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_i32x4 (__mmask16 __M, __m128i __A)
{
  return (__m512i) __builtin_ia32_broadcasti32x4_512 ((__v4si) __A,
						      (__v16si)
						      _mm512_setzero_si512 (),
						      __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_f64x4 (__m256d __A)
{
  return (__m512d) __builtin_ia32_broadcastf64x4_512 ((__v4df) __A,
						      (__v8df)
						      _mm512_undefined_pd (),
						      (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_f64x4 (__m512d __O, __mmask8 __M, __m256d __A)
{
  return (__m512d) __builtin_ia32_broadcastf64x4_512 ((__v4df) __A,
						      (__v8df) __O,
						      __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_f64x4 (__mmask8 __M, __m256d __A)
{
  return (__m512d) __builtin_ia32_broadcastf64x4_512 ((__v4df) __A,
						      (__v8df)
						      _mm512_setzero_pd (),
						      __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_broadcast_i64x4 (__m256i __A)
{
  return (__m512i) __builtin_ia32_broadcasti64x4_512 ((__v4di) __A,
						      (__v8di)
						      _mm512_undefined_epi32 (),
						      (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_broadcast_i64x4 (__m512i __O, __mmask8 __M, __m256i __A)
{
  return (__m512i) __builtin_ia32_broadcasti64x4_512 ((__v4di) __A,
						      (__v8di) __O,
						      __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_broadcast_i64x4 (__mmask8 __M, __m256i __A)
{
  return (__m512i) __builtin_ia32_broadcasti64x4_512 ((__v4di) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      __M);
}

typedef enum
{
  _MM_PERM_AAAA = 0x00, _MM_PERM_AAAB = 0x01, _MM_PERM_AAAC = 0x02,
  _MM_PERM_AAAD = 0x03, _MM_PERM_AABA = 0x04, _MM_PERM_AABB = 0x05,
  _MM_PERM_AABC = 0x06, _MM_PERM_AABD = 0x07, _MM_PERM_AACA = 0x08,
  _MM_PERM_AACB = 0x09, _MM_PERM_AACC = 0x0A, _MM_PERM_AACD = 0x0B,
  _MM_PERM_AADA = 0x0C, _MM_PERM_AADB = 0x0D, _MM_PERM_AADC = 0x0E,
  _MM_PERM_AADD = 0x0F, _MM_PERM_ABAA = 0x10, _MM_PERM_ABAB = 0x11,
  _MM_PERM_ABAC = 0x12, _MM_PERM_ABAD = 0x13, _MM_PERM_ABBA = 0x14,
  _MM_PERM_ABBB = 0x15, _MM_PERM_ABBC = 0x16, _MM_PERM_ABBD = 0x17,
  _MM_PERM_ABCA = 0x18, _MM_PERM_ABCB = 0x19, _MM_PERM_ABCC = 0x1A,
  _MM_PERM_ABCD = 0x1B, _MM_PERM_ABDA = 0x1C, _MM_PERM_ABDB = 0x1D,
  _MM_PERM_ABDC = 0x1E, _MM_PERM_ABDD = 0x1F, _MM_PERM_ACAA = 0x20,
  _MM_PERM_ACAB = 0x21, _MM_PERM_ACAC = 0x22, _MM_PERM_ACAD = 0x23,
  _MM_PERM_ACBA = 0x24, _MM_PERM_ACBB = 0x25, _MM_PERM_ACBC = 0x26,
  _MM_PERM_ACBD = 0x27, _MM_PERM_ACCA = 0x28, _MM_PERM_ACCB = 0x29,
  _MM_PERM_ACCC = 0x2A, _MM_PERM_ACCD = 0x2B, _MM_PERM_ACDA = 0x2C,
  _MM_PERM_ACDB = 0x2D, _MM_PERM_ACDC = 0x2E, _MM_PERM_ACDD = 0x2F,
  _MM_PERM_ADAA = 0x30, _MM_PERM_ADAB = 0x31, _MM_PERM_ADAC = 0x32,
  _MM_PERM_ADAD = 0x33, _MM_PERM_ADBA = 0x34, _MM_PERM_ADBB = 0x35,
  _MM_PERM_ADBC = 0x36, _MM_PERM_ADBD = 0x37, _MM_PERM_ADCA = 0x38,
  _MM_PERM_ADCB = 0x39, _MM_PERM_ADCC = 0x3A, _MM_PERM_ADCD = 0x3B,
  _MM_PERM_ADDA = 0x3C, _MM_PERM_ADDB = 0x3D, _MM_PERM_ADDC = 0x3E,
  _MM_PERM_ADDD = 0x3F, _MM_PERM_BAAA = 0x40, _MM_PERM_BAAB = 0x41,
  _MM_PERM_BAAC = 0x42, _MM_PERM_BAAD = 0x43, _MM_PERM_BABA = 0x44,
  _MM_PERM_BABB = 0x45, _MM_PERM_BABC = 0x46, _MM_PERM_BABD = 0x47,
  _MM_PERM_BACA = 0x48, _MM_PERM_BACB = 0x49, _MM_PERM_BACC = 0x4A,
  _MM_PERM_BACD = 0x4B, _MM_PERM_BADA = 0x4C, _MM_PERM_BADB = 0x4D,
  _MM_PERM_BADC = 0x4E, _MM_PERM_BADD = 0x4F, _MM_PERM_BBAA = 0x50,
  _MM_PERM_BBAB = 0x51, _MM_PERM_BBAC = 0x52, _MM_PERM_BBAD = 0x53,
  _MM_PERM_BBBA = 0x54, _MM_PERM_BBBB = 0x55, _MM_PERM_BBBC = 0x56,
  _MM_PERM_BBBD = 0x57, _MM_PERM_BBCA = 0x58, _MM_PERM_BBCB = 0x59,
  _MM_PERM_BBCC = 0x5A, _MM_PERM_BBCD = 0x5B, _MM_PERM_BBDA = 0x5C,
  _MM_PERM_BBDB = 0x5D, _MM_PERM_BBDC = 0x5E, _MM_PERM_BBDD = 0x5F,
  _MM_PERM_BCAA = 0x60, _MM_PERM_BCAB = 0x61, _MM_PERM_BCAC = 0x62,
  _MM_PERM_BCAD = 0x63, _MM_PERM_BCBA = 0x64, _MM_PERM_BCBB = 0x65,
  _MM_PERM_BCBC = 0x66, _MM_PERM_BCBD = 0x67, _MM_PERM_BCCA = 0x68,
  _MM_PERM_BCCB = 0x69, _MM_PERM_BCCC = 0x6A, _MM_PERM_BCCD = 0x6B,
  _MM_PERM_BCDA = 0x6C, _MM_PERM_BCDB = 0x6D, _MM_PERM_BCDC = 0x6E,
  _MM_PERM_BCDD = 0x6F, _MM_PERM_BDAA = 0x70, _MM_PERM_BDAB = 0x71,
  _MM_PERM_BDAC = 0x72, _MM_PERM_BDAD = 0x73, _MM_PERM_BDBA = 0x74,
  _MM_PERM_BDBB = 0x75, _MM_PERM_BDBC = 0x76, _MM_PERM_BDBD = 0x77,
  _MM_PERM_BDCA = 0x78, _MM_PERM_BDCB = 0x79, _MM_PERM_BDCC = 0x7A,
  _MM_PERM_BDCD = 0x7B, _MM_PERM_BDDA = 0x7C, _MM_PERM_BDDB = 0x7D,
  _MM_PERM_BDDC = 0x7E, _MM_PERM_BDDD = 0x7F, _MM_PERM_CAAA = 0x80,
  _MM_PERM_CAAB = 0x81, _MM_PERM_CAAC = 0x82, _MM_PERM_CAAD = 0x83,
  _MM_PERM_CABA = 0x84, _MM_PERM_CABB = 0x85, _MM_PERM_CABC = 0x86,
  _MM_PERM_CABD = 0x87, _MM_PERM_CACA = 0x88, _MM_PERM_CACB = 0x89,
  _MM_PERM_CACC = 0x8A, _MM_PERM_CACD = 0x8B, _MM_PERM_CADA = 0x8C,
  _MM_PERM_CADB = 0x8D, _MM_PERM_CADC = 0x8E, _MM_PERM_CADD = 0x8F,
  _MM_PERM_CBAA = 0x90, _MM_PERM_CBAB = 0x91, _MM_PERM_CBAC = 0x92,
  _MM_PERM_CBAD = 0x93, _MM_PERM_CBBA = 0x94, _MM_PERM_CBBB = 0x95,
  _MM_PERM_CBBC = 0x96, _MM_PERM_CBBD = 0x97, _MM_PERM_CBCA = 0x98,
  _MM_PERM_CBCB = 0x99, _MM_PERM_CBCC = 0x9A, _MM_PERM_CBCD = 0x9B,
  _MM_PERM_CBDA = 0x9C, _MM_PERM_CBDB = 0x9D, _MM_PERM_CBDC = 0x9E,
  _MM_PERM_CBDD = 0x9F, _MM_PERM_CCAA = 0xA0, _MM_PERM_CCAB = 0xA1,
  _MM_PERM_CCAC = 0xA2, _MM_PERM_CCAD = 0xA3, _MM_PERM_CCBA = 0xA4,
  _MM_PERM_CCBB = 0xA5, _MM_PERM_CCBC = 0xA6, _MM_PERM_CCBD = 0xA7,
  _MM_PERM_CCCA = 0xA8, _MM_PERM_CCCB = 0xA9, _MM_PERM_CCCC = 0xAA,
  _MM_PERM_CCCD = 0xAB, _MM_PERM_CCDA = 0xAC, _MM_PERM_CCDB = 0xAD,
  _MM_PERM_CCDC = 0xAE, _MM_PERM_CCDD = 0xAF, _MM_PERM_CDAA = 0xB0,
  _MM_PERM_CDAB = 0xB1, _MM_PERM_CDAC = 0xB2, _MM_PERM_CDAD = 0xB3,
  _MM_PERM_CDBA = 0xB4, _MM_PERM_CDBB = 0xB5, _MM_PERM_CDBC = 0xB6,
  _MM_PERM_CDBD = 0xB7, _MM_PERM_CDCA = 0xB8, _MM_PERM_CDCB = 0xB9,
  _MM_PERM_CDCC = 0xBA, _MM_PERM_CDCD = 0xBB, _MM_PERM_CDDA = 0xBC,
  _MM_PERM_CDDB = 0xBD, _MM_PERM_CDDC = 0xBE, _MM_PERM_CDDD = 0xBF,
  _MM_PERM_DAAA = 0xC0, _MM_PERM_DAAB = 0xC1, _MM_PERM_DAAC = 0xC2,
  _MM_PERM_DAAD = 0xC3, _MM_PERM_DABA = 0xC4, _MM_PERM_DABB = 0xC5,
  _MM_PERM_DABC = 0xC6, _MM_PERM_DABD = 0xC7, _MM_PERM_DACA = 0xC8,
  _MM_PERM_DACB = 0xC9, _MM_PERM_DACC = 0xCA, _MM_PERM_DACD = 0xCB,
  _MM_PERM_DADA = 0xCC, _MM_PERM_DADB = 0xCD, _MM_PERM_DADC = 0xCE,
  _MM_PERM_DADD = 0xCF, _MM_PERM_DBAA = 0xD0, _MM_PERM_DBAB = 0xD1,
  _MM_PERM_DBAC = 0xD2, _MM_PERM_DBAD = 0xD3, _MM_PERM_DBBA = 0xD4,
  _MM_PERM_DBBB = 0xD5, _MM_PERM_DBBC = 0xD6, _MM_PERM_DBBD = 0xD7,
  _MM_PERM_DBCA = 0xD8, _MM_PERM_DBCB = 0xD9, _MM_PERM_DBCC = 0xDA,
  _MM_PERM_DBCD = 0xDB, _MM_PERM_DBDA = 0xDC, _MM_PERM_DBDB = 0xDD,
  _MM_PERM_DBDC = 0xDE, _MM_PERM_DBDD = 0xDF, _MM_PERM_DCAA = 0xE0,
  _MM_PERM_DCAB = 0xE1, _MM_PERM_DCAC = 0xE2, _MM_PERM_DCAD = 0xE3,
  _MM_PERM_DCBA = 0xE4, _MM_PERM_DCBB = 0xE5, _MM_PERM_DCBC = 0xE6,
  _MM_PERM_DCBD = 0xE7, _MM_PERM_DCCA = 0xE8, _MM_PERM_DCCB = 0xE9,
  _MM_PERM_DCCC = 0xEA, _MM_PERM_DCCD = 0xEB, _MM_PERM_DCDA = 0xEC,
  _MM_PERM_DCDB = 0xED, _MM_PERM_DCDC = 0xEE, _MM_PERM_DCDD = 0xEF,
  _MM_PERM_DDAA = 0xF0, _MM_PERM_DDAB = 0xF1, _MM_PERM_DDAC = 0xF2,
  _MM_PERM_DDAD = 0xF3, _MM_PERM_DDBA = 0xF4, _MM_PERM_DDBB = 0xF5,
  _MM_PERM_DDBC = 0xF6, _MM_PERM_DDBD = 0xF7, _MM_PERM_DDCA = 0xF8,
  _MM_PERM_DDCB = 0xF9, _MM_PERM_DDCC = 0xFA, _MM_PERM_DDCD = 0xFB,
  _MM_PERM_DDDA = 0xFC, _MM_PERM_DDDB = 0xFD, _MM_PERM_DDDC = 0xFE,
  _MM_PERM_DDDD = 0xFF
} _MM_PERM_ENUM;

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_epi32 (__m512i __A, _MM_PERM_ENUM __mask)
{
  return (__m512i) __builtin_ia32_pshufd512_mask ((__v16si) __A,
						  __mask,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_epi32 (__m512i __W, __mmask16 __U, __m512i __A,
			   _MM_PERM_ENUM __mask)
{
  return (__m512i) __builtin_ia32_pshufd512_mask ((__v16si) __A,
						  __mask,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_epi32 (__mmask16 __U, __m512i __A, _MM_PERM_ENUM __mask)
{
  return (__m512i) __builtin_ia32_pshufd512_mask ((__v16si) __A,
						  __mask,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_i64x2 (__m512i __A, __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_shuf_i64x2_mask ((__v8di) __A,
						   (__v8di) __B, __imm,
						   (__v8di)
						   _mm512_undefined_epi32 (),
						   (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_i64x2 (__m512i __W, __mmask8 __U, __m512i __A,
			   __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_shuf_i64x2_mask ((__v8di) __A,
						   (__v8di) __B, __imm,
						   (__v8di) __W,
						   (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_i64x2 (__mmask8 __U, __m512i __A, __m512i __B,
			    const int __imm)
{
  return (__m512i) __builtin_ia32_shuf_i64x2_mask ((__v8di) __A,
						   (__v8di) __B, __imm,
						   (__v8di)
						   _mm512_setzero_si512 (),
						   (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_i32x4 (__m512i __A, __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_shuf_i32x4_mask ((__v16si) __A,
						   (__v16si) __B,
						   __imm,
						   (__v16si)
						   _mm512_undefined_epi32 (),
						   (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_i32x4 (__m512i __W, __mmask16 __U, __m512i __A,
			   __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_shuf_i32x4_mask ((__v16si) __A,
						   (__v16si) __B,
						   __imm,
						   (__v16si) __W,
						   (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_i32x4 (__mmask16 __U, __m512i __A, __m512i __B,
			    const int __imm)
{
  return (__m512i) __builtin_ia32_shuf_i32x4_mask ((__v16si) __A,
						   (__v16si) __B,
						   __imm,
						   (__v16si)
						   _mm512_setzero_si512 (),
						   (__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_f64x2 (__m512d __A, __m512d __B, const int __imm)
{
  return (__m512d) __builtin_ia32_shuf_f64x2_mask ((__v8df) __A,
						   (__v8df) __B, __imm,
						   (__v8df)
						   _mm512_undefined_pd (),
						   (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_f64x2 (__m512d __W, __mmask8 __U, __m512d __A,
			   __m512d __B, const int __imm)
{
  return (__m512d) __builtin_ia32_shuf_f64x2_mask ((__v8df) __A,
						   (__v8df) __B, __imm,
						   (__v8df) __W,
						   (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_f64x2 (__mmask8 __U, __m512d __A, __m512d __B,
			    const int __imm)
{
  return (__m512d) __builtin_ia32_shuf_f64x2_mask ((__v8df) __A,
						   (__v8df) __B, __imm,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_f32x4 (__m512 __A, __m512 __B, const int __imm)
{
  return (__m512) __builtin_ia32_shuf_f32x4_mask ((__v16sf) __A,
						  (__v16sf) __B, __imm,
						  (__v16sf)
						  _mm512_undefined_ps (),
						  (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_f32x4 (__m512 __W, __mmask16 __U, __m512 __A,
			   __m512 __B, const int __imm)
{
  return (__m512) __builtin_ia32_shuf_f32x4_mask ((__v16sf) __A,
						  (__v16sf) __B, __imm,
						  (__v16sf) __W,
						  (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_f32x4 (__mmask16 __U, __m512 __A, __m512 __B,
			    const int __imm)
{
  return (__m512) __builtin_ia32_shuf_f32x4_mask ((__v16sf) __A,
						  (__v16sf) __B, __imm,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __U);
}

#else
#define _mm512_shuffle_epi32(X, C)                                      \
  ((__m512i)  __builtin_ia32_pshufd512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask16)-1))

#define _mm512_mask_shuffle_epi32(W, U, X, C)                           \
  ((__m512i)  __builtin_ia32_pshufd512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_shuffle_epi32(U, X, C)                             \
  ((__m512i)  __builtin_ia32_pshufd512_mask ((__v16si)(__m512i)(X), (int)(C),\
    (__v16si)(__m512i)_mm512_setzero_si512 (),\
    (__mmask16)(U)))

#define _mm512_shuffle_i64x2(X, Y, C)                                   \
  ((__m512i)  __builtin_ia32_shuf_i64x2_mask ((__v8di)(__m512i)(X),     \
      (__v8di)(__m512i)(Y), (int)(C),\
    (__v8di)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask8)-1))

#define _mm512_mask_shuffle_i64x2(W, U, X, Y, C)                        \
  ((__m512i)  __builtin_ia32_shuf_i64x2_mask ((__v8di)(__m512i)(X),     \
      (__v8di)(__m512i)(Y), (int)(C),\
    (__v8di)(__m512i)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_shuffle_i64x2(U, X, Y, C)                          \
  ((__m512i)  __builtin_ia32_shuf_i64x2_mask ((__v8di)(__m512i)(X),     \
      (__v8di)(__m512i)(Y), (int)(C),\
    (__v8di)(__m512i)_mm512_setzero_si512 (),\
    (__mmask8)(U)))

#define _mm512_shuffle_i32x4(X, Y, C)                                   \
  ((__m512i)  __builtin_ia32_shuf_i32x4_mask ((__v16si)(__m512i)(X),    \
      (__v16si)(__m512i)(Y), (int)(C),\
    (__v16si)(__m512i)_mm512_undefined_epi32 (),\
    (__mmask16)-1))

#define _mm512_mask_shuffle_i32x4(W, U, X, Y, C)                        \
  ((__m512i)  __builtin_ia32_shuf_i32x4_mask ((__v16si)(__m512i)(X),    \
      (__v16si)(__m512i)(Y), (int)(C),\
    (__v16si)(__m512i)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_shuffle_i32x4(U, X, Y, C)                          \
  ((__m512i)  __builtin_ia32_shuf_i32x4_mask ((__v16si)(__m512i)(X),    \
      (__v16si)(__m512i)(Y), (int)(C),\
    (__v16si)(__m512i)_mm512_setzero_si512 (),\
    (__mmask16)(U)))

#define _mm512_shuffle_f64x2(X, Y, C)                                   \
  ((__m512d)  __builtin_ia32_shuf_f64x2_mask ((__v8df)(__m512d)(X),     \
      (__v8df)(__m512d)(Y), (int)(C),\
    (__v8df)(__m512d)_mm512_undefined_pd(),\
    (__mmask8)-1))

#define _mm512_mask_shuffle_f64x2(W, U, X, Y, C)                        \
  ((__m512d)  __builtin_ia32_shuf_f64x2_mask ((__v8df)(__m512d)(X),     \
      (__v8df)(__m512d)(Y), (int)(C),\
    (__v8df)(__m512d)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_shuffle_f64x2(U, X, Y, C)                         \
  ((__m512d)  __builtin_ia32_shuf_f64x2_mask ((__v8df)(__m512d)(X),    \
      (__v8df)(__m512d)(Y), (int)(C),\
    (__v8df)(__m512d)_mm512_setzero_pd(),\
    (__mmask8)(U)))

#define _mm512_shuffle_f32x4(X, Y, C)                                  \
  ((__m512)  __builtin_ia32_shuf_f32x4_mask ((__v16sf)(__m512)(X),     \
      (__v16sf)(__m512)(Y), (int)(C),\
    (__v16sf)(__m512)_mm512_undefined_ps(),\
    (__mmask16)-1))

#define _mm512_mask_shuffle_f32x4(W, U, X, Y, C)                       \
  ((__m512)  __builtin_ia32_shuf_f32x4_mask ((__v16sf)(__m512)(X),     \
      (__v16sf)(__m512)(Y), (int)(C),\
    (__v16sf)(__m512)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_shuffle_f32x4(U, X, Y, C)                         \
  ((__m512)  __builtin_ia32_shuf_f32x4_mask ((__v16sf)(__m512)(X),     \
      (__v16sf)(__m512)(Y), (int)(C),\
    (__v16sf)(__m512)_mm512_setzero_ps(),\
    (__mmask16)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rolv_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prolvd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rolv_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prolvd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rolv_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prolvd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rorv_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prorvd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rorv_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prorvd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rorv_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prorvd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rolv_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prolvq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rolv_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prolvq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W,
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rolv_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prolvq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rorv_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prorvq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rorv_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prorvq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W,
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rorv_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_prorvq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundpd_epi32 (__m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvttpd2dq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_undefined_si256 (),
						     (__mmask8) -1, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundpd_epi32 (__m256i __W, __mmask8 __U, __m512d __A,
				const int __R)
{
  return (__m256i) __builtin_ia32_cvttpd2dq512_mask ((__v8df) __A,
						     (__v8si) __W,
						     (__mmask8) __U, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundpd_epi32 (__mmask8 __U, __m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvttpd2dq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundpd_epu32 (__m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvttpd2udq512_mask ((__v8df) __A,
						      (__v8si)
						      _mm256_undefined_si256 (),
						      (__mmask8) -1, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundpd_epu32 (__m256i __W, __mmask8 __U, __m512d __A,
				const int __R)
{
  return (__m256i) __builtin_ia32_cvttpd2udq512_mask ((__v8df) __A,
						      (__v8si) __W,
						      (__mmask8) __U, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundpd_epu32 (__mmask8 __U, __m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvttpd2udq512_mask ((__v8df) __A,
						      (__v8si)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U, __R);
}
#else
#define _mm512_cvtt_roundpd_epi32(A, B)		     \
    ((__m256i)__builtin_ia32_cvttpd2dq512_mask(A, (__v8si)_mm256_undefined_si256(), -1, B))

#define _mm512_mask_cvtt_roundpd_epi32(W, U, A, B)   \
    ((__m256i)__builtin_ia32_cvttpd2dq512_mask(A, (__v8si)(W), U, B))

#define _mm512_maskz_cvtt_roundpd_epi32(U, A, B)     \
    ((__m256i)__builtin_ia32_cvttpd2dq512_mask(A, (__v8si)_mm256_setzero_si256(), U, B))

#define _mm512_cvtt_roundpd_epu32(A, B)		     \
    ((__m256i)__builtin_ia32_cvttpd2udq512_mask(A, (__v8si)_mm256_undefined_si256(), -1, B))

#define _mm512_mask_cvtt_roundpd_epu32(W, U, A, B)   \
    ((__m256i)__builtin_ia32_cvttpd2udq512_mask(A, (__v8si)(W), U, B))

#define _mm512_maskz_cvtt_roundpd_epu32(U, A, B)     \
    ((__m256i)__builtin_ia32_cvttpd2udq512_mask(A, (__v8si)_mm256_setzero_si256(), U, B))
#endif

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundpd_epi32 (__m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvtpd2dq512_mask ((__v8df) __A,
						    (__v8si)
						    _mm256_undefined_si256 (),
						    (__mmask8) -1, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundpd_epi32 (__m256i __W, __mmask8 __U, __m512d __A,
			       const int __R)
{
  return (__m256i) __builtin_ia32_cvtpd2dq512_mask ((__v8df) __A,
						    (__v8si) __W,
						    (__mmask8) __U, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundpd_epi32 (__mmask8 __U, __m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvtpd2dq512_mask ((__v8df) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundpd_epu32 (__m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvtpd2udq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_undefined_si256 (),
						     (__mmask8) -1, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundpd_epu32 (__m256i __W, __mmask8 __U, __m512d __A,
			       const int __R)
{
  return (__m256i) __builtin_ia32_cvtpd2udq512_mask ((__v8df) __A,
						     (__v8si) __W,
						     (__mmask8) __U, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundpd_epu32 (__mmask8 __U, __m512d __A, const int __R)
{
  return (__m256i) __builtin_ia32_cvtpd2udq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U, __R);
}
#else
#define _mm512_cvt_roundpd_epi32(A, B)		    \
    ((__m256i)__builtin_ia32_cvtpd2dq512_mask(A, (__v8si)_mm256_undefined_si256(), -1, B))

#define _mm512_mask_cvt_roundpd_epi32(W, U, A, B)   \
    ((__m256i)__builtin_ia32_cvtpd2dq512_mask(A, (__v8si)(W), U, B))

#define _mm512_maskz_cvt_roundpd_epi32(U, A, B)     \
    ((__m256i)__builtin_ia32_cvtpd2dq512_mask(A, (__v8si)_mm256_setzero_si256(), U, B))

#define _mm512_cvt_roundpd_epu32(A, B)		    \
    ((__m256i)__builtin_ia32_cvtpd2udq512_mask(A, (__v8si)_mm256_undefined_si256(), -1, B))

#define _mm512_mask_cvt_roundpd_epu32(W, U, A, B)   \
    ((__m256i)__builtin_ia32_cvtpd2udq512_mask(A, (__v8si)(W), U, B))

#define _mm512_maskz_cvt_roundpd_epu32(U, A, B)     \
    ((__m256i)__builtin_ia32_cvtpd2udq512_mask(A, (__v8si)_mm256_setzero_si256(), U, B))
#endif

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundps_epi32 (__m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2dq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_undefined_epi32 (),
						     (__mmask16) -1, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundps_epi32 (__m512i __W, __mmask16 __U, __m512 __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2dq512_mask ((__v16sf) __A,
						     (__v16si) __W,
						     (__mmask16) __U, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundps_epi32 (__mmask16 __U, __m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2dq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtt_roundps_epu32 (__m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2udq512_mask ((__v16sf) __A,
						      (__v16si)
						      _mm512_undefined_epi32 (),
						      (__mmask16) -1, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtt_roundps_epu32 (__m512i __W, __mmask16 __U, __m512 __A,
				const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2udq512_mask ((__v16sf) __A,
						      (__v16si) __W,
						      (__mmask16) __U, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtt_roundps_epu32 (__mmask16 __U, __m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvttps2udq512_mask ((__v16sf) __A,
						      (__v16si)
						      _mm512_setzero_si512 (),
						      (__mmask16) __U, __R);
}
#else
#define _mm512_cvtt_roundps_epi32(A, B)		     \
    ((__m512i)__builtin_ia32_cvttps2dq512_mask(A, (__v16si)_mm512_undefined_epi32 (), -1, B))

#define _mm512_mask_cvtt_roundps_epi32(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvttps2dq512_mask(A, (__v16si)(W), U, B))

#define _mm512_maskz_cvtt_roundps_epi32(U, A, B)     \
    ((__m512i)__builtin_ia32_cvttps2dq512_mask(A, (__v16si)_mm512_setzero_si512 (), U, B))

#define _mm512_cvtt_roundps_epu32(A, B)		     \
    ((__m512i)__builtin_ia32_cvttps2udq512_mask(A, (__v16si)_mm512_undefined_epi32 (), -1, B))

#define _mm512_mask_cvtt_roundps_epu32(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvttps2udq512_mask(A, (__v16si)(W), U, B))

#define _mm512_maskz_cvtt_roundps_epu32(U, A, B)     \
    ((__m512i)__builtin_ia32_cvttps2udq512_mask(A, (__v16si)_mm512_setzero_si512 (), U, B))
#endif

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundps_epi32 (__m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2dq512_mask ((__v16sf) __A,
						    (__v16si)
						    _mm512_undefined_epi32 (),
						    (__mmask16) -1, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundps_epi32 (__m512i __W, __mmask16 __U, __m512 __A,
			       const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2dq512_mask ((__v16sf) __A,
						    (__v16si) __W,
						    (__mmask16) __U, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundps_epi32 (__mmask16 __U, __m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2dq512_mask ((__v16sf) __A,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) __U, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundps_epu32 (__m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2udq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_undefined_epi32 (),
						     (__mmask16) -1, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundps_epu32 (__m512i __W, __mmask16 __U, __m512 __A,
			       const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2udq512_mask ((__v16sf) __A,
						     (__v16si) __W,
						     (__mmask16) __U, __R);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundps_epu32 (__mmask16 __U, __m512 __A, const int __R)
{
  return (__m512i) __builtin_ia32_cvtps2udq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U, __R);
}
#else
#define _mm512_cvt_roundps_epi32(A, B)		    \
    ((__m512i)__builtin_ia32_cvtps2dq512_mask(A, (__v16si)_mm512_undefined_epi32 (), -1, B))

#define _mm512_mask_cvt_roundps_epi32(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvtps2dq512_mask(A, (__v16si)(W), U, B))

#define _mm512_maskz_cvt_roundps_epi32(U, A, B)     \
    ((__m512i)__builtin_ia32_cvtps2dq512_mask(A, (__v16si)_mm512_setzero_si512 (), U, B))

#define _mm512_cvt_roundps_epu32(A, B)		    \
    ((__m512i)__builtin_ia32_cvtps2udq512_mask(A, (__v16si)_mm512_undefined_epi32 (), -1, B))

#define _mm512_mask_cvt_roundps_epu32(W, U, A, B)   \
    ((__m512i)__builtin_ia32_cvtps2udq512_mask(A, (__v16si)(W), U, B))

#define _mm512_maskz_cvt_roundps_epu32(U, A, B)     \
    ((__m512i)__builtin_ia32_cvtps2udq512_mask(A, (__v16si)_mm512_setzero_si512 (), U, B))
#endif

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtu32_sd (__m128d __A, unsigned __B)
{
  return (__m128d) __builtin_ia32_cvtusi2sd32 ((__v2df) __A, __B);
}

#ifdef __x86_64__
#ifdef __OPTIMIZE__
extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundu64_sd (__m128d __A, unsigned long long __B, const int __R)
{
  return (__m128d) __builtin_ia32_cvtusi2sd64 ((__v2df) __A, __B, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundi64_sd (__m128d __A, long long __B, const int __R)
{
  return (__m128d) __builtin_ia32_cvtsi2sd64 ((__v2df) __A, __B, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsi64_sd (__m128d __A, long long __B, const int __R)
{
  return (__m128d) __builtin_ia32_cvtsi2sd64 ((__v2df) __A, __B, __R);
}
#else
#define _mm_cvt_roundu64_sd(A, B, C)   \
    (__m128d)__builtin_ia32_cvtusi2sd64(A, B, C)

#define _mm_cvt_roundi64_sd(A, B, C)   \
    (__m128d)__builtin_ia32_cvtsi2sd64(A, B, C)

#define _mm_cvt_roundsi64_sd(A, B, C)   \
    (__m128d)__builtin_ia32_cvtsi2sd64(A, B, C)
#endif

#endif

#ifdef __OPTIMIZE__
extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundu32_ss (__m128 __A, unsigned __B, const int __R)
{
  return (__m128) __builtin_ia32_cvtusi2ss32 ((__v4sf) __A, __B, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsi32_ss (__m128 __A, int __B, const int __R)
{
  return (__m128) __builtin_ia32_cvtsi2ss32 ((__v4sf) __A, __B, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundi32_ss (__m128 __A, int __B, const int __R)
{
  return (__m128) __builtin_ia32_cvtsi2ss32 ((__v4sf) __A, __B, __R);
}
#else
#define _mm_cvt_roundu32_ss(A, B, C)   \
    (__m128)__builtin_ia32_cvtusi2ss32(A, B, C)

#define _mm_cvt_roundi32_ss(A, B, C)   \
    (__m128)__builtin_ia32_cvtsi2ss32(A, B, C)

#define _mm_cvt_roundsi32_ss(A, B, C)   \
    (__m128)__builtin_ia32_cvtsi2ss32(A, B, C)
#endif

#ifdef __x86_64__
#ifdef __OPTIMIZE__
extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundu64_ss (__m128 __A, unsigned long long __B, const int __R)
{
  return (__m128) __builtin_ia32_cvtusi2ss64 ((__v4sf) __A, __B, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsi64_ss (__m128 __A, long long __B, const int __R)
{
  return (__m128) __builtin_ia32_cvtsi2ss64 ((__v4sf) __A, __B, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundi64_ss (__m128 __A, long long __B, const int __R)
{
  return (__m128) __builtin_ia32_cvtsi2ss64 ((__v4sf) __A, __B, __R);
}
#else
#define _mm_cvt_roundu64_ss(A, B, C)   \
    (__m128)__builtin_ia32_cvtusi2ss64(A, B, C)

#define _mm_cvt_roundi64_ss(A, B, C)   \
    (__m128)__builtin_ia32_cvtsi2ss64(A, B, C)

#define _mm_cvt_roundsi64_ss(A, B, C)   \
    (__m128)__builtin_ia32_cvtsi2ss64(A, B, C)
#endif

#endif

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi32_epi8 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovdb512_mask ((__v16si) __A,
						  (__v16qi)
						  _mm_undefined_si128 (),
						  (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi32_storeu_epi8 (void * __P, __mmask16 __M, __m512i __A)
{
  __builtin_ia32_pmovdb512mem_mask ((__v16qi *) __P, (__v16si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi32_epi8 (__m128i __O, __mmask16 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovdb512_mask ((__v16si) __A,
						  (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi32_epi8 (__mmask16 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovdb512_mask ((__v16si) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtsepi32_epi8 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb512_mask ((__v16si) __A,
						   (__v16qi)
						   _mm_undefined_si128 (),
						   (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi32_storeu_epi8 (void * __P, __mmask16 __M, __m512i __A)
{
  __builtin_ia32_pmovsdb512mem_mask ((__v16qi *) __P, (__v16si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi32_epi8 (__m128i __O, __mmask16 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb512_mask ((__v16si) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtsepi32_epi8 (__mmask16 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsdb512_mask ((__v16si) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtusepi32_epi8 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb512_mask ((__v16si) __A,
						    (__v16qi)
						    _mm_undefined_si128 (),
						    (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi32_storeu_epi8 (void * __P, __mmask16 __M, __m512i __A)
{
  __builtin_ia32_pmovusdb512mem_mask ((__v16qi *) __P, (__v16si) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi32_epi8 (__m128i __O, __mmask16 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb512_mask ((__v16si) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtusepi32_epi8 (__mmask16 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusdb512_mask ((__v16si) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi32_epi16 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovdw512_mask ((__v16si) __A,
						  (__v16hi)
						  _mm256_undefined_si256 (),
						  (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi32_storeu_epi16 (void * __P, __mmask16 __M, __m512i __A)
{
  __builtin_ia32_pmovdw512mem_mask ((__v16hi *) __P, (__v16si) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi32_epi16 (__m256i __O, __mmask16 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovdw512_mask ((__v16si) __A,
						  (__v16hi) __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi32_epi16 (__mmask16 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovdw512_mask ((__v16si) __A,
						  (__v16hi)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtsepi32_epi16 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovsdw512_mask ((__v16si) __A,
						   (__v16hi)
						   _mm256_undefined_si256 (),
						   (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi32_storeu_epi16 (void *__P, __mmask16 __M, __m512i __A)
{
  __builtin_ia32_pmovsdw512mem_mask ((__v16hi*) __P, (__v16si) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi32_epi16 (__m256i __O, __mmask16 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovsdw512_mask ((__v16si) __A,
						   (__v16hi) __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtsepi32_epi16 (__mmask16 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovsdw512_mask ((__v16si) __A,
						   (__v16hi)
						   _mm256_setzero_si256 (),
						   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtusepi32_epi16 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovusdw512_mask ((__v16si) __A,
						    (__v16hi)
						    _mm256_undefined_si256 (),
						    (__mmask16) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi32_storeu_epi16 (void *__P, __mmask16 __M, __m512i __A)
{
  __builtin_ia32_pmovusdw512mem_mask ((__v16hi*) __P, (__v16si) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi32_epi16 (__m256i __O, __mmask16 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovusdw512_mask ((__v16si) __A,
						    (__v16hi) __O,
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtusepi32_epi16 (__mmask16 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovusdw512_mask ((__v16si) __A,
						    (__v16hi)
						    _mm256_setzero_si256 (),
						    __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi64_epi32 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovqd512_mask ((__v8di) __A,
						  (__v8si)
						  _mm256_undefined_si256 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_storeu_epi32 (void* __P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovqd512mem_mask ((__v8si *) __P, (__v8di) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_epi32 (__m256i __O, __mmask8 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovqd512_mask ((__v8di) __A,
						  (__v8si) __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi64_epi32 (__mmask8 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovqd512_mask ((__v8di) __A,
						  (__v8si)
						  _mm256_setzero_si256 (),
						  __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtsepi64_epi32 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovsqd512_mask ((__v8di) __A,
						   (__v8si)
						   _mm256_undefined_si256 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi64_storeu_epi32 (void *__P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovsqd512mem_mask ((__v8si *) __P, (__v8di) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi64_epi32 (__m256i __O, __mmask8 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovsqd512_mask ((__v8di) __A,
						   (__v8si) __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtsepi64_epi32 (__mmask8 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovsqd512_mask ((__v8di) __A,
						   (__v8si)
						   _mm256_setzero_si256 (),
						   __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtusepi64_epi32 (__m512i __A)
{
  return (__m256i) __builtin_ia32_pmovusqd512_mask ((__v8di) __A,
						    (__v8si)
						    _mm256_undefined_si256 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi64_storeu_epi32 (void* __P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovusqd512mem_mask ((__v8si*) __P, (__v8di) __A, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi64_epi32 (__m256i __O, __mmask8 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovusqd512_mask ((__v8di) __A,
						    (__v8si) __O, __M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtusepi64_epi32 (__mmask8 __M, __m512i __A)
{
  return (__m256i) __builtin_ia32_pmovusqd512_mask ((__v8di) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi64_epi16 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovqw512_mask ((__v8di) __A,
						  (__v8hi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_storeu_epi16 (void *__P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovqw512mem_mask ((__v8hi *) __P, (__v8di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_epi16 (__m128i __O, __mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovqw512_mask ((__v8di) __A,
						  (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi64_epi16 (__mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovqw512_mask ((__v8di) __A,
						  (__v8hi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtsepi64_epi16 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw512_mask ((__v8di) __A,
						   (__v8hi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi64_storeu_epi16 (void * __P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovsqw512mem_mask ((__v8hi *) __P, (__v8di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi64_epi16 (__m128i __O, __mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw512_mask ((__v8di) __A,
						   (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtsepi64_epi16 (__mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsqw512_mask ((__v8di) __A,
						   (__v8hi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtusepi64_epi16 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw512_mask ((__v8di) __A,
						    (__v8hi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi64_storeu_epi16 (void *__P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovusqw512mem_mask ((__v8hi*) __P, (__v8di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi64_epi16 (__m128i __O, __mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw512_mask ((__v8di) __A,
						    (__v8hi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtusepi64_epi16 (__mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusqw512_mask ((__v8di) __A,
						    (__v8hi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi64_epi8 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovqb512_mask ((__v8di) __A,
						  (__v16qi)
						  _mm_undefined_si128 (),
						  (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_storeu_epi8 (void * __P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovqb512mem_mask ((__v16qi *) __P, (__v8di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi64_epi8 (__m128i __O, __mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovqb512_mask ((__v8di) __A,
						  (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi64_epi8 (__mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovqb512_mask ((__v8di) __A,
						  (__v16qi)
						  _mm_setzero_si128 (),
						  __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtsepi64_epi8 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb512_mask ((__v8di) __A,
						   (__v16qi)
						   _mm_undefined_si128 (),
						   (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi64_storeu_epi8 (void * __P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovsqb512mem_mask ((__v16qi *) __P, (__v8di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtsepi64_epi8 (__m128i __O, __mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb512_mask ((__v8di) __A,
						   (__v16qi) __O, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtsepi64_epi8 (__mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovsqb512_mask ((__v8di) __A,
						   (__v16qi)
						   _mm_setzero_si128 (),
						   __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtusepi64_epi8 (__m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb512_mask ((__v8di) __A,
						    (__v16qi)
						    _mm_undefined_si128 (),
						    (__mmask8) -1);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi64_storeu_epi8 (void * __P, __mmask8 __M, __m512i __A)
{
  __builtin_ia32_pmovusqb512mem_mask ((__v16qi *) __P, (__v8di) __A, __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtusepi64_epi8 (__m128i __O, __mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb512_mask ((__v8di) __A,
						    (__v16qi) __O,
						    __M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtusepi64_epi8 (__mmask8 __M, __m512i __A)
{
  return (__m128i) __builtin_ia32_pmovusqb512_mask ((__v8di) __A,
						    (__v16qi)
						    _mm_setzero_si128 (),
						    __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi32_pd (__m256i __A)
{
  return (__m512d) __builtin_ia32_cvtdq2pd512_mask ((__v8si) __A,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi32_pd (__m512d __W, __mmask8 __U, __m256i __A)
{
  return (__m512d) __builtin_ia32_cvtdq2pd512_mask ((__v8si) __A,
						    (__v8df) __W,
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi32_pd (__mmask8 __U, __m256i __A)
{
  return (__m512d) __builtin_ia32_cvtdq2pd512_mask ((__v8si) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu32_pd (__m256i __A)
{
  return (__m512d) __builtin_ia32_cvtudq2pd512_mask ((__v8si) __A,
						     (__v8df)
						     _mm512_undefined_pd (),
						     (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu32_pd (__m512d __W, __mmask8 __U, __m256i __A)
{
  return (__m512d) __builtin_ia32_cvtudq2pd512_mask ((__v8si) __A,
						     (__v8df) __W,
						     (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu32_pd (__mmask8 __U, __m256i __A)
{
  return (__m512d) __builtin_ia32_cvtudq2pd512_mask ((__v8si) __A,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundepi32_ps (__m512i __A, const int __R)
{
  return (__m512) __builtin_ia32_cvtdq2ps512_mask ((__v16si) __A,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundepi32_ps (__m512 __W, __mmask16 __U, __m512i __A,
			       const int __R)
{
  return (__m512) __builtin_ia32_cvtdq2ps512_mask ((__v16si) __A,
						   (__v16sf) __W,
						   (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundepi32_ps (__mmask16 __U, __m512i __A, const int __R)
{
  return (__m512) __builtin_ia32_cvtdq2ps512_mask ((__v16si) __A,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundepu32_ps (__m512i __A, const int __R)
{
  return (__m512) __builtin_ia32_cvtudq2ps512_mask ((__v16si) __A,
						    (__v16sf)
						    _mm512_undefined_ps (),
						    (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundepu32_ps (__m512 __W, __mmask16 __U, __m512i __A,
			       const int __R)
{
  return (__m512) __builtin_ia32_cvtudq2ps512_mask ((__v16si) __A,
						    (__v16sf) __W,
						    (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundepu32_ps (__mmask16 __U, __m512i __A, const int __R)
{
  return (__m512) __builtin_ia32_cvtudq2ps512_mask ((__v16si) __A,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U, __R);
}

#else
#define _mm512_cvt_roundepi32_ps(A, B)        \
    (__m512)__builtin_ia32_cvtdq2ps512_mask((__v16si)(A), (__v16sf)_mm512_undefined_ps(), -1, B)

#define _mm512_mask_cvt_roundepi32_ps(W, U, A, B)   \
    (__m512)__builtin_ia32_cvtdq2ps512_mask((__v16si)(A), W, U, B)

#define _mm512_maskz_cvt_roundepi32_ps(U, A, B)      \
    (__m512)__builtin_ia32_cvtdq2ps512_mask((__v16si)(A), (__v16sf)_mm512_setzero_ps(), U, B)

#define _mm512_cvt_roundepu32_ps(A, B)        \
    (__m512)__builtin_ia32_cvtudq2ps512_mask((__v16si)(A), (__v16sf)_mm512_undefined_ps(), -1, B)

#define _mm512_mask_cvt_roundepu32_ps(W, U, A, B)   \
    (__m512)__builtin_ia32_cvtudq2ps512_mask((__v16si)(A), W, U, B)

#define _mm512_maskz_cvt_roundepu32_ps(U, A, B)      \
    (__m512)__builtin_ia32_cvtudq2ps512_mask((__v16si)(A), (__v16sf)_mm512_setzero_ps(), U, B)
#endif

#ifdef __OPTIMIZE__
extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extractf64x4_pd (__m512d __A, const int __imm)
{
  return (__m256d) __builtin_ia32_extractf64x4_mask ((__v8df) __A,
						     __imm,
						     (__v4df)
						     _mm256_undefined_pd (),
						     (__mmask8) -1);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extractf64x4_pd (__m256d __W, __mmask8 __U, __m512d __A,
			     const int __imm)
{
  return (__m256d) __builtin_ia32_extractf64x4_mask ((__v8df) __A,
						     __imm,
						     (__v4df) __W,
						     (__mmask8) __U);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extractf64x4_pd (__mmask8 __U, __m512d __A, const int __imm)
{
  return (__m256d) __builtin_ia32_extractf64x4_mask ((__v8df) __A,
						     __imm,
						     (__v4df)
						     _mm256_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extractf32x4_ps (__m512 __A, const int __imm)
{
  return (__m128) __builtin_ia32_extractf32x4_mask ((__v16sf) __A,
						    __imm,
						    (__v4sf)
						    _mm_undefined_ps (),
						    (__mmask8) -1);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extractf32x4_ps (__m128 __W, __mmask8 __U, __m512 __A,
			     const int __imm)
{
  return (__m128) __builtin_ia32_extractf32x4_mask ((__v16sf) __A,
						    __imm,
						    (__v4sf) __W,
						    (__mmask8) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extractf32x4_ps (__mmask8 __U, __m512 __A, const int __imm)
{
  return (__m128) __builtin_ia32_extractf32x4_mask ((__v16sf) __A,
						    __imm,
						    (__v4sf)
						    _mm_setzero_ps (),
						    (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extracti64x4_epi64 (__m512i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_extracti64x4_mask ((__v8di) __A,
						     __imm,
						     (__v4di)
						     _mm256_undefined_si256 (),
						     (__mmask8) -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extracti64x4_epi64 (__m256i __W, __mmask8 __U, __m512i __A,
				const int __imm)
{
  return (__m256i) __builtin_ia32_extracti64x4_mask ((__v8di) __A,
						     __imm,
						     (__v4di) __W,
						     (__mmask8) __U);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extracti64x4_epi64 (__mmask8 __U, __m512i __A, const int __imm)
{
  return (__m256i) __builtin_ia32_extracti64x4_mask ((__v8di) __A,
						     __imm,
						     (__v4di)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_extracti32x4_epi32 (__m512i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_extracti32x4_mask ((__v16si) __A,
						     __imm,
						     (__v4si)
						     _mm_undefined_si128 (),
						     (__mmask8) -1);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_extracti32x4_epi32 (__m128i __W, __mmask8 __U, __m512i __A,
				const int __imm)
{
  return (__m128i) __builtin_ia32_extracti32x4_mask ((__v16si) __A,
						     __imm,
						     (__v4si) __W,
						     (__mmask8) __U);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_extracti32x4_epi32 (__mmask8 __U, __m512i __A, const int __imm)
{
  return (__m128i) __builtin_ia32_extracti32x4_mask ((__v16si) __A,
						     __imm,
						     (__v4si)
						     _mm_setzero_si128 (),
						     (__mmask8) __U);
}
#else

#define _mm512_extractf64x4_pd(X, C)                                    \
  ((__m256d) __builtin_ia32_extractf64x4_mask ((__v8df)(__m512d) (X),   \
    (int) (C),\
    (__v4df)(__m256d)_mm256_undefined_pd(),\
    (__mmask8)-1))

#define _mm512_mask_extractf64x4_pd(W, U, X, C)                         \
  ((__m256d) __builtin_ia32_extractf64x4_mask ((__v8df)(__m512d) (X),   \
    (int) (C),\
    (__v4df)(__m256d)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_extractf64x4_pd(U, X, C)                           \
  ((__m256d) __builtin_ia32_extractf64x4_mask ((__v8df)(__m512d) (X),   \
    (int) (C),\
    (__v4df)(__m256d)_mm256_setzero_pd(),\
    (__mmask8)(U)))

#define _mm512_extractf32x4_ps(X, C)                                    \
  ((__m128) __builtin_ia32_extractf32x4_mask ((__v16sf)(__m512) (X),    \
    (int) (C),\
    (__v4sf)(__m128)_mm_undefined_ps(),\
    (__mmask8)-1))

#define _mm512_mask_extractf32x4_ps(W, U, X, C)                         \
  ((__m128) __builtin_ia32_extractf32x4_mask ((__v16sf)(__m512) (X),    \
    (int) (C),\
    (__v4sf)(__m128)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_extractf32x4_ps(U, X, C)                           \
  ((__m128) __builtin_ia32_extractf32x4_mask ((__v16sf)(__m512) (X),    \
    (int) (C),\
    (__v4sf)(__m128)_mm_setzero_ps(),\
    (__mmask8)(U)))

#define _mm512_extracti64x4_epi64(X, C)                                 \
  ((__m256i) __builtin_ia32_extracti64x4_mask ((__v8di)(__m512i) (X),   \
    (int) (C),\
    (__v4di)(__m256i)_mm256_undefined_si256 (),\
    (__mmask8)-1))

#define _mm512_mask_extracti64x4_epi64(W, U, X, C)                      \
  ((__m256i) __builtin_ia32_extracti64x4_mask ((__v8di)(__m512i) (X),   \
    (int) (C),\
    (__v4di)(__m256i)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_extracti64x4_epi64(U, X, C)                        \
  ((__m256i) __builtin_ia32_extracti64x4_mask ((__v8di)(__m512i) (X),   \
    (int) (C),\
    (__v4di)(__m256i)_mm256_setzero_si256 (),\
    (__mmask8)(U)))

#define _mm512_extracti32x4_epi32(X, C)                                 \
  ((__m128i) __builtin_ia32_extracti32x4_mask ((__v16si)(__m512i) (X),  \
    (int) (C),\
    (__v4si)(__m128i)_mm_undefined_si128 (),\
    (__mmask8)-1))

#define _mm512_mask_extracti32x4_epi32(W, U, X, C)                      \
  ((__m128i) __builtin_ia32_extracti32x4_mask ((__v16si)(__m512i) (X),  \
    (int) (C),\
    (__v4si)(__m128i)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_extracti32x4_epi32(U, X, C)                        \
  ((__m128i) __builtin_ia32_extracti32x4_mask ((__v16si)(__m512i) (X),  \
    (int) (C),\
    (__v4si)(__m128i)_mm_setzero_si128 (),\
    (__mmask8)(U)))
#endif

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_inserti32x4 (__m512i __A, __m128i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti32x4_mask ((__v16si) __A,
						    (__v4si) __B,
						    __imm,
						    (__v16si) __A, -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_insertf32x4 (__m512 __A, __m128 __B, const int __imm)
{
  return (__m512) __builtin_ia32_insertf32x4_mask ((__v16sf) __A,
						   (__v4sf) __B,
						   __imm,
						   (__v16sf) __A, -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_inserti64x4 (__m512i __A, __m256i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti64x4_mask ((__v8di) __A,
						    (__v4di) __B,
						    __imm,
						    (__v8di)
						    _mm512_undefined_epi32 (),
						    (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_inserti64x4 (__m512i __W, __mmask8 __U, __m512i __A,
			 __m256i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti64x4_mask ((__v8di) __A,
						    (__v4di) __B,
						    __imm,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_inserti64x4 (__mmask8 __U, __m512i __A, __m256i __B,
			  const int __imm)
{
  return (__m512i) __builtin_ia32_inserti64x4_mask ((__v8di) __A,
						    (__v4di) __B,
						    __imm,
						    (__v8di)
						    _mm512_setzero_si512 (),
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_insertf64x4 (__m512d __A, __m256d __B, const int __imm)
{
  return (__m512d) __builtin_ia32_insertf64x4_mask ((__v8df) __A,
						    (__v4df) __B,
						    __imm,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_insertf64x4 (__m512d __W, __mmask8 __U, __m512d __A,
			 __m256d __B, const int __imm)
{
  return (__m512d) __builtin_ia32_insertf64x4_mask ((__v8df) __A,
						    (__v4df) __B,
						    __imm,
						    (__v8df) __W,
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_insertf64x4 (__mmask8 __U, __m512d __A, __m256d __B,
			  const int __imm)
{
  return (__m512d) __builtin_ia32_insertf64x4_mask ((__v8df) __A,
						    (__v4df) __B,
						    __imm,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U);
}
#else
#define _mm512_insertf32x4(X, Y, C)                                     \
  ((__m512) __builtin_ia32_insertf32x4_mask ((__v16sf)(__m512) (X),     \
    (__v4sf)(__m128) (Y), (int) (C), (__v16sf)(__m512) (X), (__mmask16)(-1)))

#define _mm512_inserti32x4(X, Y, C)                                     \
  ((__m512i) __builtin_ia32_inserti32x4_mask ((__v16si)(__m512i) (X),   \
    (__v4si)(__m128i) (Y), (int) (C), (__v16si)(__m512i) (X), (__mmask16)(-1)))

#define _mm512_insertf64x4(X, Y, C)                                     \
  ((__m512d) __builtin_ia32_insertf64x4_mask ((__v8df)(__m512d) (X),    \
    (__v4df)(__m256d) (Y), (int) (C),					\
    (__v8df)(__m512d)_mm512_undefined_pd(),				\
    (__mmask8)-1))

#define _mm512_mask_insertf64x4(W, U, X, Y, C)                          \
  ((__m512d) __builtin_ia32_insertf64x4_mask ((__v8df)(__m512d) (X),    \
    (__v4df)(__m256d) (Y), (int) (C),					\
    (__v8df)(__m512d)(W),						\
    (__mmask8)(U)))

#define _mm512_maskz_insertf64x4(U, X, Y, C)                            \
  ((__m512d) __builtin_ia32_insertf64x4_mask ((__v8df)(__m512d) (X),    \
    (__v4df)(__m256d) (Y), (int) (C),					\
    (__v8df)(__m512d)_mm512_setzero_pd(),				\
    (__mmask8)(U)))

#define _mm512_inserti64x4(X, Y, C)                                     \
  ((__m512i) __builtin_ia32_inserti64x4_mask ((__v8di)(__m512i) (X),    \
    (__v4di)(__m256i) (Y), (int) (C),					\
    (__v8di)(__m512i)_mm512_undefined_epi32 (),				\
    (__mmask8)-1))

#define _mm512_mask_inserti64x4(W, U, X, Y, C)                          \
  ((__m512i) __builtin_ia32_inserti64x4_mask ((__v8di)(__m512i) (X),    \
    (__v4di)(__m256i) (Y), (int) (C),\
    (__v8di)(__m512i)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_inserti64x4(U, X, Y, C)                            \
  ((__m512i) __builtin_ia32_inserti64x4_mask ((__v8di)(__m512i) (X),    \
    (__v4di)(__m256i) (Y), (int) (C),					\
    (__v8di)(__m512i)_mm512_setzero_si512 (),				\
    (__mmask8)(U)))
#endif

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_loadu_pd (void const *__P)
{
  return *(__m512d_u *)__P;
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_loadu_pd (__m512d __W, __mmask8 __U, void const *__P)
{
  return (__m512d) __builtin_ia32_loadupd512_mask ((const double *) __P,
						   (__v8df) __W,
						   (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_loadu_pd (__mmask8 __U, void const *__P)
{
  return (__m512d) __builtin_ia32_loadupd512_mask ((const double *) __P,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_storeu_pd (void *__P, __m512d __A)
{
  *(__m512d_u *)__P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_storeu_pd (void *__P, __mmask8 __U, __m512d __A)
{
  __builtin_ia32_storeupd512_mask ((double *) __P, (__v8df) __A,
				   (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_loadu_ps (void const *__P)
{
  return *(__m512_u *)__P;
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_loadu_ps (__m512 __W, __mmask16 __U, void const *__P)
{
  return (__m512) __builtin_ia32_loadups512_mask ((const float *) __P,
						  (__v16sf) __W,
						  (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_loadu_ps (__mmask16 __U, void const *__P)
{
  return (__m512) __builtin_ia32_loadups512_mask ((const float *) __P,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_storeu_ps (void *__P, __m512 __A)
{
  *(__m512_u *)__P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_storeu_ps (void *__P, __mmask16 __U, __m512 __A)
{
  __builtin_ia32_storeups512_mask ((float *) __P, (__v16sf) __A,
				   (__mmask16) __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_load_ss (__m128 __W, __mmask8 __U, const float *__P)
{
  return (__m128) __builtin_ia32_loadss_mask (__P, (__v4sf) __W, __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_load_ss (__mmask8 __U, const float *__P)
{
  return (__m128) __builtin_ia32_loadss_mask (__P, (__v4sf) _mm_setzero_ps (),
					      __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_load_sd (__m128d __W, __mmask8 __U, const double *__P)
{
  return (__m128d) __builtin_ia32_loadsd_mask (__P, (__v2df) __W, __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_load_sd (__mmask8 __U, const double *__P)
{
  return (__m128d) __builtin_ia32_loadsd_mask (__P, (__v2df) _mm_setzero_pd (),
					       __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_move_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_movess_mask ((__v4sf) __A, (__v4sf) __B,
					      (__v4sf) __W, __U);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_move_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_movess_mask ((__v4sf) __A, (__v4sf) __B,
					      (__v4sf) _mm_setzero_ps (), __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_move_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_movesd_mask ((__v2df) __A, (__v2df) __B,
					       (__v2df) __W, __U);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_move_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_movesd_mask ((__v2df) __A, (__v2df) __B,
					       (__v2df) _mm_setzero_pd (),
					       __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_store_ss (float *__P, __mmask8 __U, __m128 __A)
{
  __builtin_ia32_storess_mask (__P, (__v4sf) __A, (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_store_sd (double *__P, __mmask8 __U, __m128d __A)
{
  __builtin_ia32_storesd_mask (__P, (__v2df) __A, (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_loadu_epi64 (void const *__P)
{
  return *(__m512i_u *) __P;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_loadu_epi64 (__m512i __W, __mmask8 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddqudi512_mask ((const long long *) __P,
						     (__v8di) __W,
						     (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_loadu_epi64 (__mmask8 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddqudi512_mask ((const long long *) __P,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_storeu_epi64 (void *__P, __m512i __A)
{
  *(__m512i_u *) __P = (__m512i_u) __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_storeu_epi64 (void *__P, __mmask8 __U, __m512i __A)
{
  __builtin_ia32_storedqudi512_mask ((long long *) __P, (__v8di) __A,
				     (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_loadu_si512 (void const *__P)
{
  return *(__m512i_u *)__P;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_loadu_epi32 (void const *__P)
{
  return *(__m512i_u *) __P;
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_loadu_epi32 (__m512i __W, __mmask16 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddqusi512_mask ((const int *) __P,
						     (__v16si) __W,
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_loadu_epi32 (__mmask16 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_loaddqusi512_mask ((const int *) __P,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_storeu_si512 (void *__P, __m512i __A)
{
  *(__m512i_u *)__P = __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_storeu_epi32 (void *__P, __m512i __A)
{
  *(__m512i_u *) __P = (__m512i_u) __A;
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_storeu_epi32 (void *__P, __mmask16 __U, __m512i __A)
{
  __builtin_ia32_storedqusi512_mask ((int *) __P, (__v16si) __A,
				     (__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutevar_pd (__m512d __A, __m512i __C)
{
  return (__m512d) __builtin_ia32_vpermilvarpd512_mask ((__v8df) __A,
							(__v8di) __C,
							(__v8df)
							_mm512_undefined_pd (),
							(__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutevar_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512i __C)
{
  return (__m512d) __builtin_ia32_vpermilvarpd512_mask ((__v8df) __A,
							(__v8di) __C,
							(__v8df) __W,
							(__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutevar_pd (__mmask8 __U, __m512d __A, __m512i __C)
{
  return (__m512d) __builtin_ia32_vpermilvarpd512_mask ((__v8df) __A,
							(__v8di) __C,
							(__v8df)
							_mm512_setzero_pd (),
							(__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutevar_ps (__m512 __A, __m512i __C)
{
  return (__m512) __builtin_ia32_vpermilvarps512_mask ((__v16sf) __A,
						       (__v16si) __C,
						       (__v16sf)
						       _mm512_undefined_ps (),
						       (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutevar_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512i __C)
{
  return (__m512) __builtin_ia32_vpermilvarps512_mask ((__v16sf) __A,
						       (__v16si) __C,
						       (__v16sf) __W,
						       (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutevar_ps (__mmask16 __U, __m512 __A, __m512i __C)
{
  return (__m512) __builtin_ia32_vpermilvarps512_mask ((__v16sf) __A,
						       (__v16si) __C,
						       (__v16sf)
						       _mm512_setzero_ps (),
						       (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex2var_epi64 (__m512i __A, __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varq512_mask ((__v8di) __I
						       /* idx */ ,
						       (__v8di) __A,
						       (__v8di) __B,
						       (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex2var_epi64 (__m512i __A, __mmask8 __U, __m512i __I,
				__m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varq512_mask ((__v8di) __I
						       /* idx */ ,
						       (__v8di) __A,
						       (__v8di) __B,
						       (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask2_permutex2var_epi64 (__m512i __A, __m512i __I,
				 __mmask8 __U, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermi2varq512_mask ((__v8di) __A,
						       (__v8di) __I
						       /* idx */ ,
						       (__v8di) __B,
						       (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex2var_epi64 (__mmask8 __U, __m512i __A,
				 __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2varq512_maskz ((__v8di) __I
							/* idx */ ,
							(__v8di) __A,
							(__v8di) __B,
							(__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex2var_epi32 (__m512i __A, __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2vard512_mask ((__v16si) __I
						       /* idx */ ,
						       (__v16si) __A,
						       (__v16si) __B,
						       (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex2var_epi32 (__m512i __A, __mmask16 __U,
				__m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2vard512_mask ((__v16si) __I
						       /* idx */ ,
						       (__v16si) __A,
						       (__v16si) __B,
						       (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask2_permutex2var_epi32 (__m512i __A, __m512i __I,
				 __mmask16 __U, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermi2vard512_mask ((__v16si) __A,
						       (__v16si) __I
						       /* idx */ ,
						       (__v16si) __B,
						       (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex2var_epi32 (__mmask16 __U, __m512i __A,
				 __m512i __I, __m512i __B)
{
  return (__m512i) __builtin_ia32_vpermt2vard512_maskz ((__v16si) __I
							/* idx */ ,
							(__v16si) __A,
							(__v16si) __B,
							(__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex2var_pd (__m512d __A, __m512i __I, __m512d __B)
{
  return (__m512d) __builtin_ia32_vpermt2varpd512_mask ((__v8di) __I
							/* idx */ ,
							(__v8df) __A,
							(__v8df) __B,
							(__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex2var_pd (__m512d __A, __mmask8 __U, __m512i __I,
			     __m512d __B)
{
  return (__m512d) __builtin_ia32_vpermt2varpd512_mask ((__v8di) __I
							/* idx */ ,
							(__v8df) __A,
							(__v8df) __B,
							(__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask2_permutex2var_pd (__m512d __A, __m512i __I, __mmask8 __U,
			      __m512d __B)
{
  return (__m512d) __builtin_ia32_vpermi2varpd512_mask ((__v8df) __A,
							(__v8di) __I
							/* idx */ ,
							(__v8df) __B,
							(__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex2var_pd (__mmask8 __U, __m512d __A, __m512i __I,
			      __m512d __B)
{
  return (__m512d) __builtin_ia32_vpermt2varpd512_maskz ((__v8di) __I
							 /* idx */ ,
							 (__v8df) __A,
							 (__v8df) __B,
							 (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex2var_ps (__m512 __A, __m512i __I, __m512 __B)
{
  return (__m512) __builtin_ia32_vpermt2varps512_mask ((__v16si) __I
						       /* idx */ ,
						       (__v16sf) __A,
						       (__v16sf) __B,
						       (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex2var_ps (__m512 __A, __mmask16 __U, __m512i __I, __m512 __B)
{
  return (__m512) __builtin_ia32_vpermt2varps512_mask ((__v16si) __I
						       /* idx */ ,
						       (__v16sf) __A,
						       (__v16sf) __B,
						       (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask2_permutex2var_ps (__m512 __A, __m512i __I, __mmask16 __U,
			      __m512 __B)
{
  return (__m512) __builtin_ia32_vpermi2varps512_mask ((__v16sf) __A,
						       (__v16si) __I
						       /* idx */ ,
						       (__v16sf) __B,
						       (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex2var_ps (__mmask16 __U, __m512 __A, __m512i __I,
			      __m512 __B)
{
  return (__m512) __builtin_ia32_vpermt2varps512_maskz ((__v16si) __I
							/* idx */ ,
							(__v16sf) __A,
							(__v16sf) __B,
							(__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permute_pd (__m512d __X, const int __C)
{
  return (__m512d) __builtin_ia32_vpermilpd512_mask ((__v8df) __X, __C,
						     (__v8df)
						     _mm512_undefined_pd (),
						     (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permute_pd (__m512d __W, __mmask8 __U, __m512d __X, const int __C)
{
  return (__m512d) __builtin_ia32_vpermilpd512_mask ((__v8df) __X, __C,
						     (__v8df) __W,
						     (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permute_pd (__mmask8 __U, __m512d __X, const int __C)
{
  return (__m512d) __builtin_ia32_vpermilpd512_mask ((__v8df) __X, __C,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permute_ps (__m512 __X, const int __C)
{
  return (__m512) __builtin_ia32_vpermilps512_mask ((__v16sf) __X, __C,
						    (__v16sf)
						    _mm512_undefined_ps (),
						    (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permute_ps (__m512 __W, __mmask16 __U, __m512 __X, const int __C)
{
  return (__m512) __builtin_ia32_vpermilps512_mask ((__v16sf) __X, __C,
						    (__v16sf) __W,
						    (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permute_ps (__mmask16 __U, __m512 __X, const int __C)
{
  return (__m512) __builtin_ia32_vpermilps512_mask ((__v16sf) __X, __C,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U);
}
#else
#define _mm512_permute_pd(X, C)							    \
  ((__m512d) __builtin_ia32_vpermilpd512_mask ((__v8df)(__m512d)(X), (int)(C),	    \
					      (__v8df)(__m512d)_mm512_undefined_pd(),\
					      (__mmask8)(-1)))

#define _mm512_mask_permute_pd(W, U, X, C)					    \
  ((__m512d) __builtin_ia32_vpermilpd512_mask ((__v8df)(__m512d)(X), (int)(C),	    \
					      (__v8df)(__m512d)(W),		    \
					      (__mmask8)(U)))

#define _mm512_maskz_permute_pd(U, X, C)					    \
  ((__m512d) __builtin_ia32_vpermilpd512_mask ((__v8df)(__m512d)(X), (int)(C),	    \
					      (__v8df)(__m512d)_mm512_setzero_pd(), \
					      (__mmask8)(U)))

#define _mm512_permute_ps(X, C)							    \
  ((__m512) __builtin_ia32_vpermilps512_mask ((__v16sf)(__m512)(X), (int)(C),	    \
					      (__v16sf)(__m512)_mm512_undefined_ps(),\
					      (__mmask16)(-1)))

#define _mm512_mask_permute_ps(W, U, X, C)					    \
  ((__m512) __builtin_ia32_vpermilps512_mask ((__v16sf)(__m512)(X), (int)(C),	    \
					      (__v16sf)(__m512)(W),		    \
					      (__mmask16)(U)))

#define _mm512_maskz_permute_ps(U, X, C)					    \
  ((__m512) __builtin_ia32_vpermilps512_mask ((__v16sf)(__m512)(X), (int)(C),	    \
					      (__v16sf)(__m512)_mm512_setzero_ps(), \
					      (__mmask16)(U)))
#endif

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex_epi64 (__m512i __X, const int __I)
{
  return (__m512i) __builtin_ia32_permdi512_mask ((__v8di) __X, __I,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) (-1));
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex_epi64 (__m512i __W, __mmask8 __M,
			    __m512i __X, const int __I)
{
  return (__m512i) __builtin_ia32_permdi512_mask ((__v8di) __X, __I,
						  (__v8di) __W,
						  (__mmask8) __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex_epi64 (__mmask8 __M, __m512i __X, const int __I)
{
  return (__m512i) __builtin_ia32_permdi512_mask ((__v8di) __X, __I,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutex_pd (__m512d __X, const int __M)
{
  return (__m512d) __builtin_ia32_permdf512_mask ((__v8df) __X, __M,
						  (__v8df)
						  _mm512_undefined_pd (),
						  (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutex_pd (__m512d __W, __mmask8 __U, __m512d __X, const int __M)
{
  return (__m512d) __builtin_ia32_permdf512_mask ((__v8df) __X, __M,
						  (__v8df) __W,
						  (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutex_pd (__mmask8 __U, __m512d __X, const int __M)
{
  return (__m512d) __builtin_ia32_permdf512_mask ((__v8df) __X, __M,
						  (__v8df)
						  _mm512_setzero_pd (),
						  (__mmask8) __U);
}
#else
#define _mm512_permutex_pd(X, M)						\
  ((__m512d) __builtin_ia32_permdf512_mask ((__v8df)(__m512d)(X), (int)(M),	\
					    (__v8df)(__m512d)_mm512_undefined_pd(),\
					    (__mmask8)-1))

#define _mm512_mask_permutex_pd(W, U, X, M)					\
  ((__m512d) __builtin_ia32_permdf512_mask ((__v8df)(__m512d)(X), (int)(M),	\
					    (__v8df)(__m512d)(W), (__mmask8)(U)))

#define _mm512_maskz_permutex_pd(U, X, M)					\
  ((__m512d) __builtin_ia32_permdf512_mask ((__v8df)(__m512d)(X), (int)(M),	\
					    (__v8df)(__m512d)_mm512_setzero_pd(),\
					    (__mmask8)(U)))

#define _mm512_permutex_epi64(X, I)			          \
  ((__m512i) __builtin_ia32_permdi512_mask ((__v8di)(__m512i)(X), \
					    (int)(I),             \
					    (__v8di)(__m512i)	  \
					    (_mm512_undefined_epi32 ()),\
					    (__mmask8)(-1)))

#define _mm512_maskz_permutex_epi64(M, X, I)                 \
  ((__m512i) __builtin_ia32_permdi512_mask ((__v8di)(__m512i)(X), \
					    (int)(I),             \
					    (__v8di)(__m512i)     \
					    (_mm512_setzero_si512 ()),\
					    (__mmask8)(M)))

#define _mm512_mask_permutex_epi64(W, M, X, I)               \
  ((__m512i) __builtin_ia32_permdi512_mask ((__v8di)(__m512i)(X), \
					    (int)(I),             \
					    (__v8di)(__m512i)(W), \
					    (__mmask8)(M)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutexvar_epi64 (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_permvardi512_mask ((__v8di) __Y,
						     (__v8di) __X,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutexvar_epi64 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_permvardi512_mask ((__v8di) __Y,
						     (__v8di) __X,
						     (__v8di)
						     _mm512_undefined_epi32 (),
						     (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutexvar_epi64 (__m512i __W, __mmask8 __M, __m512i __X,
			       __m512i __Y)
{
  return (__m512i) __builtin_ia32_permvardi512_mask ((__v8di) __Y,
						     (__v8di) __X,
						     (__v8di) __W,
						     __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutexvar_epi32 (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_permvarsi512_mask ((__v16si) __Y,
						     (__v16si) __X,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutexvar_epi32 (__m512i __X, __m512i __Y)
{
  return (__m512i) __builtin_ia32_permvarsi512_mask ((__v16si) __Y,
						     (__v16si) __X,
						     (__v16si)
						     _mm512_undefined_epi32 (),
						     (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutexvar_epi32 (__m512i __W, __mmask16 __M, __m512i __X,
			       __m512i __Y)
{
  return (__m512i) __builtin_ia32_permvarsi512_mask ((__v16si) __Y,
						     (__v16si) __X,
						     (__v16si) __W,
						     __M);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutexvar_pd (__m512i __X, __m512d __Y)
{
  return (__m512d) __builtin_ia32_permvardf512_mask ((__v8df) __Y,
						     (__v8di) __X,
						     (__v8df)
						     _mm512_undefined_pd (),
						     (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutexvar_pd (__m512d __W, __mmask8 __U, __m512i __X, __m512d __Y)
{
  return (__m512d) __builtin_ia32_permvardf512_mask ((__v8df) __Y,
						     (__v8di) __X,
						     (__v8df) __W,
						     (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutexvar_pd (__mmask8 __U, __m512i __X, __m512d __Y)
{
  return (__m512d) __builtin_ia32_permvardf512_mask ((__v8df) __Y,
						     (__v8di) __X,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_permutexvar_ps (__m512i __X, __m512 __Y)
{
  return (__m512) __builtin_ia32_permvarsf512_mask ((__v16sf) __Y,
						    (__v16si) __X,
						    (__v16sf)
						    _mm512_undefined_ps (),
						    (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_permutexvar_ps (__m512 __W, __mmask16 __U, __m512i __X, __m512 __Y)
{
  return (__m512) __builtin_ia32_permvarsf512_mask ((__v16sf) __Y,
						    (__v16si) __X,
						    (__v16sf) __W,
						    (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_permutexvar_ps (__mmask16 __U, __m512i __X, __m512 __Y)
{
  return (__m512) __builtin_ia32_permvarsf512_mask ((__v16sf) __Y,
						    (__v16si) __X,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_ps (__m512 __M, __m512 __V, const int __imm)
{
  return (__m512) __builtin_ia32_shufps512_mask ((__v16sf) __M,
						 (__v16sf) __V, __imm,
						 (__v16sf)
						 _mm512_undefined_ps (),
						 (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_ps (__m512 __W, __mmask16 __U, __m512 __M,
			__m512 __V, const int __imm)
{
  return (__m512) __builtin_ia32_shufps512_mask ((__v16sf) __M,
						 (__v16sf) __V, __imm,
						 (__v16sf) __W,
						 (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_ps (__mmask16 __U, __m512 __M, __m512 __V, const int __imm)
{
  return (__m512) __builtin_ia32_shufps512_mask ((__v16sf) __M,
						 (__v16sf) __V, __imm,
						 (__v16sf)
						 _mm512_setzero_ps (),
						 (__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_shuffle_pd (__m512d __M, __m512d __V, const int __imm)
{
  return (__m512d) __builtin_ia32_shufpd512_mask ((__v8df) __M,
						  (__v8df) __V, __imm,
						  (__v8df)
						  _mm512_undefined_pd (),
						  (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_shuffle_pd (__m512d __W, __mmask8 __U, __m512d __M,
			__m512d __V, const int __imm)
{
  return (__m512d) __builtin_ia32_shufpd512_mask ((__v8df) __M,
						  (__v8df) __V, __imm,
						  (__v8df) __W,
						  (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_shuffle_pd (__mmask8 __U, __m512d __M, __m512d __V,
			 const int __imm)
{
  return (__m512d) __builtin_ia32_shufpd512_mask ((__v8df) __M,
						  (__v8df) __V, __imm,
						  (__v8df)
						  _mm512_setzero_pd (),
						  (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fixupimm_round_pd (__m512d __A, __m512d __B, __m512i __C,
			  const int __imm, const int __R)
{
  return (__m512d) __builtin_ia32_fixupimmpd512_mask ((__v8df) __A,
						      (__v8df) __B,
						      (__v8di) __C,
						      __imm,
						      (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fixupimm_round_pd (__m512d __A, __mmask8 __U, __m512d __B,
			       __m512i __C, const int __imm, const int __R)
{
  return (__m512d) __builtin_ia32_fixupimmpd512_mask ((__v8df) __A,
						      (__v8df) __B,
						      (__v8di) __C,
						      __imm,
						      (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fixupimm_round_pd (__mmask8 __U, __m512d __A, __m512d __B,
				__m512i __C, const int __imm, const int __R)
{
  return (__m512d) __builtin_ia32_fixupimmpd512_maskz ((__v8df) __A,
						       (__v8df) __B,
						       (__v8di) __C,
						       __imm,
						       (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fixupimm_round_ps (__m512 __A, __m512 __B, __m512i __C,
			  const int __imm, const int __R)
{
  return (__m512) __builtin_ia32_fixupimmps512_mask ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16si) __C,
						     __imm,
						     (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fixupimm_round_ps (__m512 __A, __mmask16 __U, __m512 __B,
			       __m512i __C, const int __imm, const int __R)
{
  return (__m512) __builtin_ia32_fixupimmps512_mask ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16si) __C,
						     __imm,
						     (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fixupimm_round_ps (__mmask16 __U, __m512 __A, __m512 __B,
				__m512i __C, const int __imm, const int __R)
{
  return (__m512) __builtin_ia32_fixupimmps512_maskz ((__v16sf) __A,
						      (__v16sf) __B,
						      (__v16si) __C,
						      __imm,
						      (__mmask16) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fixupimm_round_sd (__m128d __A, __m128d __B, __m128i __C,
		       const int __imm, const int __R)
{
  return (__m128d) __builtin_ia32_fixupimmsd_mask ((__v2df) __A,
						   (__v2df) __B,
						   (__v2di) __C, __imm,
						   (__mmask8) -1, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fixupimm_round_sd (__m128d __A, __mmask8 __U, __m128d __B,
			    __m128i __C, const int __imm, const int __R)
{
  return (__m128d) __builtin_ia32_fixupimmsd_mask ((__v2df) __A,
						   (__v2df) __B,
						   (__v2di) __C, __imm,
						   (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fixupimm_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			     __m128i __C, const int __imm, const int __R)
{
  return (__m128d) __builtin_ia32_fixupimmsd_maskz ((__v2df) __A,
						    (__v2df) __B,
						    (__v2di) __C,
						    __imm,
						    (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fixupimm_round_ss (__m128 __A, __m128 __B, __m128i __C,
		       const int __imm, const int __R)
{
  return (__m128) __builtin_ia32_fixupimmss_mask ((__v4sf) __A,
						  (__v4sf) __B,
						  (__v4si) __C, __imm,
						  (__mmask8) -1, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fixupimm_round_ss (__m128 __A, __mmask8 __U, __m128 __B,
			    __m128i __C, const int __imm, const int __R)
{
  return (__m128) __builtin_ia32_fixupimmss_mask ((__v4sf) __A,
						  (__v4sf) __B,
						  (__v4si) __C, __imm,
						  (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fixupimm_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			     __m128i __C, const int __imm, const int __R)
{
  return (__m128) __builtin_ia32_fixupimmss_maskz ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4si) __C, __imm,
						   (__mmask8) __U, __R);
}

#else
#define _mm512_shuffle_pd(X, Y, C)                                      \
    ((__m512d)__builtin_ia32_shufpd512_mask ((__v8df)(__m512d)(X),           \
        (__v8df)(__m512d)(Y), (int)(C),\
    (__v8df)(__m512d)_mm512_undefined_pd(),\
    (__mmask8)-1))

#define _mm512_mask_shuffle_pd(W, U, X, Y, C)                           \
    ((__m512d)__builtin_ia32_shufpd512_mask ((__v8df)(__m512d)(X),           \
        (__v8df)(__m512d)(Y), (int)(C),\
    (__v8df)(__m512d)(W),\
    (__mmask8)(U)))

#define _mm512_maskz_shuffle_pd(U, X, Y, C)                             \
    ((__m512d)__builtin_ia32_shufpd512_mask ((__v8df)(__m512d)(X),           \
        (__v8df)(__m512d)(Y), (int)(C),\
    (__v8df)(__m512d)_mm512_setzero_pd(),\
    (__mmask8)(U)))

#define _mm512_shuffle_ps(X, Y, C)                                      \
    ((__m512)__builtin_ia32_shufps512_mask ((__v16sf)(__m512)(X),            \
        (__v16sf)(__m512)(Y), (int)(C),\
    (__v16sf)(__m512)_mm512_undefined_ps(),\
    (__mmask16)-1))

#define _mm512_mask_shuffle_ps(W, U, X, Y, C)                           \
    ((__m512)__builtin_ia32_shufps512_mask ((__v16sf)(__m512)(X),            \
        (__v16sf)(__m512)(Y), (int)(C),\
    (__v16sf)(__m512)(W),\
    (__mmask16)(U)))

#define _mm512_maskz_shuffle_ps(U, X, Y, C)                             \
    ((__m512)__builtin_ia32_shufps512_mask ((__v16sf)(__m512)(X),            \
        (__v16sf)(__m512)(Y), (int)(C),\
    (__v16sf)(__m512)_mm512_setzero_ps(),\
    (__mmask16)(U)))

#define _mm512_fixupimm_round_pd(X, Y, Z, C, R)					\
  ((__m512d)__builtin_ia32_fixupimmpd512_mask ((__v8df)(__m512d)(X),	\
      (__v8df)(__m512d)(Y), (__v8di)(__m512i)(Z), (int)(C),		\
      (__mmask8)(-1), (R)))

#define _mm512_mask_fixupimm_round_pd(X, U, Y, Z, C, R)                          \
  ((__m512d)__builtin_ia32_fixupimmpd512_mask ((__v8df)(__m512d)(X),    \
      (__v8df)(__m512d)(Y), (__v8di)(__m512i)(Z), (int)(C),             \
      (__mmask8)(U), (R)))

#define _mm512_maskz_fixupimm_round_pd(U, X, Y, Z, C, R)                         \
  ((__m512d)__builtin_ia32_fixupimmpd512_maskz ((__v8df)(__m512d)(X),   \
      (__v8df)(__m512d)(Y), (__v8di)(__m512i)(Z), (int)(C),             \
      (__mmask8)(U), (R)))

#define _mm512_fixupimm_round_ps(X, Y, Z, C, R)					\
  ((__m512)__builtin_ia32_fixupimmps512_mask ((__v16sf)(__m512)(X),	\
    (__v16sf)(__m512)(Y), (__v16si)(__m512i)(Z), (int)(C),		\
    (__mmask16)(-1), (R)))

#define _mm512_mask_fixupimm_round_ps(X, U, Y, Z, C, R)                          \
  ((__m512)__builtin_ia32_fixupimmps512_mask ((__v16sf)(__m512)(X),     \
    (__v16sf)(__m512)(Y), (__v16si)(__m512i)(Z), (int)(C),              \
    (__mmask16)(U), (R)))

#define _mm512_maskz_fixupimm_round_ps(U, X, Y, Z, C, R)                         \
  ((__m512)__builtin_ia32_fixupimmps512_maskz ((__v16sf)(__m512)(X),    \
    (__v16sf)(__m512)(Y), (__v16si)(__m512i)(Z), (int)(C),              \
    (__mmask16)(U), (R)))

#define _mm_fixupimm_round_sd(X, Y, Z, C, R)					\
    ((__m128d)__builtin_ia32_fixupimmsd_mask ((__v2df)(__m128d)(X),	\
      (__v2df)(__m128d)(Y), (__v2di)(__m128i)(Z), (int)(C),		\
      (__mmask8)(-1), (R)))

#define _mm_mask_fixupimm_round_sd(X, U, Y, Z, C, R)				\
    ((__m128d)__builtin_ia32_fixupimmsd_mask ((__v2df)(__m128d)(X),	\
      (__v2df)(__m128d)(Y), (__v2di)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), (R)))

#define _mm_maskz_fixupimm_round_sd(U, X, Y, Z, C, R)				\
    ((__m128d)__builtin_ia32_fixupimmsd_maskz ((__v2df)(__m128d)(X),	\
      (__v2df)(__m128d)(Y), (__v2di)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), (R)))

#define _mm_fixupimm_round_ss(X, Y, Z, C, R)					\
    ((__m128)__builtin_ia32_fixupimmss_mask ((__v4sf)(__m128)(X),	\
      (__v4sf)(__m128)(Y), (__v4si)(__m128i)(Z), (int)(C),		\
      (__mmask8)(-1), (R)))

#define _mm_mask_fixupimm_round_ss(X, U, Y, Z, C, R)				\
    ((__m128)__builtin_ia32_fixupimmss_mask ((__v4sf)(__m128)(X),	\
      (__v4sf)(__m128)(Y), (__v4si)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), (R)))

#define _mm_maskz_fixupimm_round_ss(U, X, Y, Z, C, R)				\
    ((__m128)__builtin_ia32_fixupimmss_maskz ((__v4sf)(__m128)(X),	\
      (__v4sf)(__m128)(Y), (__v4si)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), (R)))
#endif

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movehdup_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_movshdup512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_movehdup_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_movshdup512_mask ((__v16sf) __A,
						   (__v16sf) __W,
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_movehdup_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_movshdup512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_moveldup_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_movsldup512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_moveldup_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_movsldup512_mask ((__v16sf) __A,
						   (__v16sf) __W,
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_moveldup_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_movsldup512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_or_si512 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A | (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_or_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A | (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_or_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pord512_mask ((__v16si) __A,
						(__v16si) __B,
						(__v16si) __W,
						(__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_or_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pord512_mask ((__v16si) __A,
						(__v16si) __B,
						(__v16si)
						_mm512_setzero_si512 (),
						(__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_or_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v8du) __A | (__v8du) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_or_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_porq512_mask ((__v8di) __A,
						(__v8di) __B,
						(__v8di) __W,
						(__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_or_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_porq512_mask ((__v8di) __A,
						(__v8di) __B,
						(__v8di)
						_mm512_setzero_si512 (),
						(__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_xor_si512 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A ^ (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_xor_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A ^ (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_xor_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pxord512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_xor_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pxord512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_xor_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v8du) __A ^ (__v8du) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_xor_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pxorq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_xor_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pxorq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rol_epi32 (__m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_prold512_mask ((__v16si) __A, __B,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rol_epi32 (__m512i __W, __mmask16 __U, __m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_prold512_mask ((__v16si) __A, __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rol_epi32 (__mmask16 __U, __m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_prold512_mask ((__v16si) __A, __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_ror_epi32 (__m512i __A, int __B)
{
  return (__m512i) __builtin_ia32_prord512_mask ((__v16si) __A, __B,
						 (__v16si)
						 _mm512_undefined_epi32 (),
						 (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_ror_epi32 (__m512i __W, __mmask16 __U, __m512i __A, int __B)
{
  return (__m512i) __builtin_ia32_prord512_mask ((__v16si) __A, __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_ror_epi32 (__mmask16 __U, __m512i __A, int __B)
{
  return (__m512i) __builtin_ia32_prord512_mask ((__v16si) __A, __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_rol_epi64 (__m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_prolq512_mask ((__v8di) __A, __B,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_rol_epi64 (__m512i __W, __mmask8 __U, __m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_prolq512_mask ((__v8di) __A, __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_rol_epi64 (__mmask8 __U, __m512i __A, const int __B)
{
  return (__m512i) __builtin_ia32_prolq512_mask ((__v8di) __A, __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_ror_epi64 (__m512i __A, int __B)
{
  return (__m512i) __builtin_ia32_prorq512_mask ((__v8di) __A, __B,
						 (__v8di)
						 _mm512_undefined_epi32 (),
						 (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_ror_epi64 (__m512i __W, __mmask8 __U, __m512i __A, int __B)
{
  return (__m512i) __builtin_ia32_prorq512_mask ((__v8di) __A, __B,
						 (__v8di) __W,
						 (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_ror_epi64 (__mmask8 __U, __m512i __A, int __B)
{
  return (__m512i) __builtin_ia32_prorq512_mask ((__v8di) __A, __B,
						 (__v8di)
						 _mm512_setzero_si512 (),
						 (__mmask8) __U);
}

#else
#define _mm512_rol_epi32(A, B)						  \
    ((__m512i)__builtin_ia32_prold512_mask ((__v16si)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v16si)_mm512_undefined_epi32 (), \
					    (__mmask16)(-1)))
#define _mm512_mask_rol_epi32(W, U, A, B)				  \
    ((__m512i)__builtin_ia32_prold512_mask ((__v16si)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v16si)(__m512i)(W),	  \
					    (__mmask16)(U)))
#define _mm512_maskz_rol_epi32(U, A, B)					  \
    ((__m512i)__builtin_ia32_prold512_mask ((__v16si)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v16si)_mm512_setzero_si512 (), \
					    (__mmask16)(U)))
#define _mm512_ror_epi32(A, B)						  \
    ((__m512i)__builtin_ia32_prord512_mask ((__v16si)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v16si)_mm512_undefined_epi32 (), \
					    (__mmask16)(-1)))
#define _mm512_mask_ror_epi32(W, U, A, B)				  \
    ((__m512i)__builtin_ia32_prord512_mask ((__v16si)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v16si)(__m512i)(W),	  \
					    (__mmask16)(U)))
#define _mm512_maskz_ror_epi32(U, A, B)					  \
    ((__m512i)__builtin_ia32_prord512_mask ((__v16si)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v16si)_mm512_setzero_si512 (), \
					    (__mmask16)(U)))
#define _mm512_rol_epi64(A, B)						  \
    ((__m512i)__builtin_ia32_prolq512_mask ((__v8di)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v8di)_mm512_undefined_epi32 (),  \
					    (__mmask8)(-1)))
#define _mm512_mask_rol_epi64(W, U, A, B)				  \
    ((__m512i)__builtin_ia32_prolq512_mask ((__v8di)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v8di)(__m512i)(W),	  \
					    (__mmask8)(U)))
#define _mm512_maskz_rol_epi64(U, A, B)					  \
    ((__m512i)__builtin_ia32_prolq512_mask ((__v8di)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v8di)_mm512_setzero_si512 (),  \
					    (__mmask8)(U)))

#define _mm512_ror_epi64(A, B)						  \
    ((__m512i)__builtin_ia32_prorq512_mask ((__v8di)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v8di)_mm512_undefined_epi32 (),  \
					    (__mmask8)(-1)))
#define _mm512_mask_ror_epi64(W, U, A, B)				  \
    ((__m512i)__builtin_ia32_prorq512_mask ((__v8di)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v8di)(__m512i)(W),	  \
					    (__mmask8)(U)))
#define _mm512_maskz_ror_epi64(U, A, B)					  \
    ((__m512i)__builtin_ia32_prorq512_mask ((__v8di)(__m512i)(A),	  \
					    (int)(B),			  \
					    (__v8di)_mm512_setzero_si512 (),  \
					    (__mmask8)(U)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_and_si512 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A & (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_and_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v16su) __A & (__v16su) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_and_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandd512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si) __W,
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_and_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandd512_mask ((__v16si) __A,
						 (__v16si) __B,
						 (__v16si)
						 _mm512_setzero_si512 (),
						 (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_and_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) ((__v8du) __A & (__v8du) __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_and_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di) __W, __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_and_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandq512_mask ((__v8di) __A,
						 (__v8di) __B,
						 (__v8di)
						 _mm512_setzero_pd (),
						 __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_andnot_si512 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandnd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_andnot_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandnd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_andnot_epi32 (__m512i __W, __mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandnd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_andnot_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandnd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_andnot_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandnq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_andnot_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandnq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W, __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_andnot_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pandnq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_pd (),
						  __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_test_epi32_mask (__m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ptestmd512 ((__v16si) __A,
						(__v16si) __B,
						(__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_test_epi32_mask (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ptestmd512 ((__v16si) __A,
						(__v16si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_test_epi64_mask (__m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ptestmq512 ((__v8di) __A,
					       (__v8di) __B,
					       (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_test_epi64_mask (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ptestmq512 ((__v8di) __A, (__v8di) __B, __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_testn_epi32_mask (__m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ptestnmd512 ((__v16si) __A,
						 (__v16si) __B,
						 (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_testn_epi32_mask (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ptestnmd512 ((__v16si) __A,
						 (__v16si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_testn_epi64_mask (__m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmq512 ((__v8di) __A,
						(__v8di) __B,
						(__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_testn_epi64_mask (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ptestnmq512 ((__v8di) __A,
						(__v8di) __B, __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_abs_ps (__m512 __A)
{
  return (__m512) _mm512_and_epi32 ((__m512i) __A,
				    _mm512_set1_epi32 (0x7fffffff));
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_abs_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) _mm512_mask_and_epi32 ((__m512i) __W, __U, (__m512i) __A,
					 _mm512_set1_epi32 (0x7fffffff));
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_abs_pd (__m512d __A)
{
  return (__m512d) _mm512_and_epi64 ((__m512i) __A,
				     _mm512_set1_epi64 (0x7fffffffffffffffLL));
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_abs_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d)
	 _mm512_mask_and_epi64 ((__m512i) __W, __U, (__m512i) __A,
				_mm512_set1_epi64 (0x7fffffffffffffffLL));
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpackhi_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhdq512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si)
						     _mm512_undefined_epi32 (),
						     (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpackhi_epi32 (__m512i __W, __mmask16 __U, __m512i __A,
			    __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhdq512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si) __W,
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpackhi_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhdq512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpackhi_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhqdq512_mask ((__v8di) __A,
						      (__v8di) __B,
						      (__v8di)
						      _mm512_undefined_epi32 (),
						      (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpackhi_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhqdq512_mask ((__v8di) __A,
						      (__v8di) __B,
						      (__v8di) __W,
						      (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpackhi_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckhqdq512_mask ((__v8di) __A,
						      (__v8di) __B,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpacklo_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckldq512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si)
						     _mm512_undefined_epi32 (),
						     (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpacklo_epi32 (__m512i __W, __mmask16 __U, __m512i __A,
			    __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckldq512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si) __W,
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpacklo_epi32 (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpckldq512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpacklo_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklqdq512_mask ((__v8di) __A,
						      (__v8di) __B,
						      (__v8di)
						      _mm512_undefined_epi32 (),
						      (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpacklo_epi64 (__m512i __W, __mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklqdq512_mask ((__v8di) __A,
						      (__v8di) __B,
						      (__v8di) __W,
						      (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpacklo_epi64 (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_punpcklqdq512_mask ((__v8di) __A,
						      (__v8di) __B,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) __U);
}

#ifdef __x86_64__
#ifdef __OPTIMIZE__
extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundss_u64 (__m128 __A, const int __R)
{
  return (unsigned long long) __builtin_ia32_vcvtss2usi64 ((__v4sf) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundss_si64 (__m128 __A, const int __R)
{
  return (long long) __builtin_ia32_vcvtss2si64 ((__v4sf) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundss_i64 (__m128 __A, const int __R)
{
  return (long long) __builtin_ia32_vcvtss2si64 ((__v4sf) __A, __R);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundss_u64 (__m128 __A, const int __R)
{
  return (unsigned long long) __builtin_ia32_vcvttss2usi64 ((__v4sf) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundss_i64 (__m128 __A, const int __R)
{
  return (long long) __builtin_ia32_vcvttss2si64 ((__v4sf) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundss_si64 (__m128 __A, const int __R)
{
  return (long long) __builtin_ia32_vcvttss2si64 ((__v4sf) __A, __R);
}
#else
#define _mm_cvt_roundss_u64(A, B)   \
    ((unsigned long long)__builtin_ia32_vcvtss2usi64(A, B))

#define _mm_cvt_roundss_si64(A, B)   \
    ((long long)__builtin_ia32_vcvtss2si64(A, B))

#define _mm_cvt_roundss_i64(A, B)   \
    ((long long)__builtin_ia32_vcvtss2si64(A, B))

#define _mm_cvtt_roundss_u64(A, B)  \
    ((unsigned long long)__builtin_ia32_vcvttss2usi64(A, B))

#define _mm_cvtt_roundss_i64(A, B)  \
    ((long long)__builtin_ia32_vcvttss2si64(A, B))

#define _mm_cvtt_roundss_si64(A, B)  \
    ((long long)__builtin_ia32_vcvttss2si64(A, B))
#endif
#endif

#ifdef __OPTIMIZE__
extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundss_u32 (__m128 __A, const int __R)
{
  return (unsigned) __builtin_ia32_vcvtss2usi32 ((__v4sf) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundss_si32 (__m128 __A, const int __R)
{
  return (int) __builtin_ia32_vcvtss2si32 ((__v4sf) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundss_i32 (__m128 __A, const int __R)
{
  return (int) __builtin_ia32_vcvtss2si32 ((__v4sf) __A, __R);
}

extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundss_u32 (__m128 __A, const int __R)
{
  return (unsigned) __builtin_ia32_vcvttss2usi32 ((__v4sf) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundss_i32 (__m128 __A, const int __R)
{
  return (int) __builtin_ia32_vcvttss2si32 ((__v4sf) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundss_si32 (__m128 __A, const int __R)
{
  return (int) __builtin_ia32_vcvttss2si32 ((__v4sf) __A, __R);
}
#else
#define _mm_cvt_roundss_u32(A, B)   \
    ((unsigned)__builtin_ia32_vcvtss2usi32(A, B))

#define _mm_cvt_roundss_si32(A, B)   \
    ((int)__builtin_ia32_vcvtss2si32(A, B))

#define _mm_cvt_roundss_i32(A, B)   \
    ((int)__builtin_ia32_vcvtss2si32(A, B))

#define _mm_cvtt_roundss_u32(A, B)  \
    ((unsigned)__builtin_ia32_vcvttss2usi32(A, B))

#define _mm_cvtt_roundss_si32(A, B)  \
    ((int)__builtin_ia32_vcvttss2si32(A, B))

#define _mm_cvtt_roundss_i32(A, B)  \
    ((int)__builtin_ia32_vcvttss2si32(A, B))
#endif

#ifdef __x86_64__
#ifdef __OPTIMIZE__
extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsd_u64 (__m128d __A, const int __R)
{
  return (unsigned long long) __builtin_ia32_vcvtsd2usi64 ((__v2df) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsd_si64 (__m128d __A, const int __R)
{
  return (long long) __builtin_ia32_vcvtsd2si64 ((__v2df) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsd_i64 (__m128d __A, const int __R)
{
  return (long long) __builtin_ia32_vcvtsd2si64 ((__v2df) __A, __R);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundsd_u64 (__m128d __A, const int __R)
{
  return (unsigned long long) __builtin_ia32_vcvttsd2usi64 ((__v2df) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundsd_si64 (__m128d __A, const int __R)
{
  return (long long) __builtin_ia32_vcvttsd2si64 ((__v2df) __A, __R);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundsd_i64 (__m128d __A, const int __R)
{
  return (long long) __builtin_ia32_vcvttsd2si64 ((__v2df) __A, __R);
}
#else
#define _mm_cvt_roundsd_u64(A, B)   \
    ((unsigned long long)__builtin_ia32_vcvtsd2usi64(A, B))

#define _mm_cvt_roundsd_si64(A, B)   \
    ((long long)__builtin_ia32_vcvtsd2si64(A, B))

#define _mm_cvt_roundsd_i64(A, B)   \
    ((long long)__builtin_ia32_vcvtsd2si64(A, B))

#define _mm_cvtt_roundsd_u64(A, B)   \
    ((unsigned long long)__builtin_ia32_vcvttsd2usi64(A, B))

#define _mm_cvtt_roundsd_si64(A, B)   \
    ((long long)__builtin_ia32_vcvttsd2si64(A, B))

#define _mm_cvtt_roundsd_i64(A, B)   \
    ((long long)__builtin_ia32_vcvttsd2si64(A, B))
#endif
#endif

#ifdef __OPTIMIZE__
extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsd_u32 (__m128d __A, const int __R)
{
  return (unsigned) __builtin_ia32_vcvtsd2usi32 ((__v2df) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsd_si32 (__m128d __A, const int __R)
{
  return (int) __builtin_ia32_vcvtsd2si32 ((__v2df) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsd_i32 (__m128d __A, const int __R)
{
  return (int) __builtin_ia32_vcvtsd2si32 ((__v2df) __A, __R);
}

extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundsd_u32 (__m128d __A, const int __R)
{
  return (unsigned) __builtin_ia32_vcvttsd2usi32 ((__v2df) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundsd_i32 (__m128d __A, const int __R)
{
  return (int) __builtin_ia32_vcvttsd2si32 ((__v2df) __A, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtt_roundsd_si32 (__m128d __A, const int __R)
{
  return (int) __builtin_ia32_vcvttsd2si32 ((__v2df) __A, __R);
}
#else
#define _mm_cvt_roundsd_u32(A, B)   \
    ((unsigned)__builtin_ia32_vcvtsd2usi32(A, B))

#define _mm_cvt_roundsd_si32(A, B)   \
    ((int)__builtin_ia32_vcvtsd2si32(A, B))

#define _mm_cvt_roundsd_i32(A, B)   \
    ((int)__builtin_ia32_vcvtsd2si32(A, B))

#define _mm_cvtt_roundsd_u32(A, B)   \
    ((unsigned)__builtin_ia32_vcvttsd2usi32(A, B))

#define _mm_cvtt_roundsd_si32(A, B)   \
    ((int)__builtin_ia32_vcvttsd2si32(A, B))

#define _mm_cvtt_roundsd_i32(A, B)   \
    ((int)__builtin_ia32_vcvttsd2si32(A, B))
#endif

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_movedup_pd (__m512d __A)
{
  return (__m512d) __builtin_ia32_movddup512_mask ((__v8df) __A,
						   (__v8df)
						   _mm512_undefined_pd (),
						   (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_movedup_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_movddup512_mask ((__v8df) __A,
						   (__v8df) __W,
						   (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_movedup_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_movddup512_mask ((__v8df) __A,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpacklo_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_unpcklpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpacklo_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_unpcklpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __W,
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpacklo_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_unpcklpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpackhi_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_unpckhpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpackhi_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_unpckhpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __W,
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpackhi_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_unpckhpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpackhi_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_unpckhps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpackhi_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_unpckhps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __W,
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpackhi_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_unpckhps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundps_pd (__m256 __A, const int __R)
{
  return (__m512d) __builtin_ia32_cvtps2pd512_mask ((__v8sf) __A,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundps_pd (__m512d __W, __mmask8 __U, __m256 __A,
			    const int __R)
{
  return (__m512d) __builtin_ia32_cvtps2pd512_mask ((__v8sf) __A,
						    (__v8df) __W,
						    (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundps_pd (__mmask8 __U, __m256 __A, const int __R)
{
  return (__m512d) __builtin_ia32_cvtps2pd512_mask ((__v8sf) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundph_ps (__m256i __A, const int __R)
{
  return (__m512) __builtin_ia32_vcvtph2ps512_mask ((__v16hi) __A,
						    (__v16sf)
						    _mm512_undefined_ps (),
						    (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundph_ps (__m512 __W, __mmask16 __U, __m256i __A,
			    const int __R)
{
  return (__m512) __builtin_ia32_vcvtph2ps512_mask ((__v16hi) __A,
						    (__v16sf) __W,
						    (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundph_ps (__mmask16 __U, __m256i __A, const int __R)
{
  return (__m512) __builtin_ia32_vcvtph2ps512_mask ((__v16hi) __A,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U, __R);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundps_ph (__m512 __A, const int __I)
{
  return (__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf) __A,
						     __I,
						     (__v16hi)
						     _mm256_undefined_si256 (),
						     -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtps_ph (__m512 __A, const int __I)
{
  return (__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf) __A,
						     __I,
						     (__v16hi)
						     _mm256_undefined_si256 (),
						     -1);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundps_ph (__m256i __U, __mmask16 __W, __m512 __A,
			    const int __I)
{
  return (__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf) __A,
						     __I,
						     (__v16hi) __U,
						     (__mmask16) __W);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtps_ph (__m256i __U, __mmask16 __W, __m512 __A, const int __I)
{
  return (__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf) __A,
						     __I,
						     (__v16hi) __U,
						     (__mmask16) __W);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundps_ph (__mmask16 __W, __m512 __A, const int __I)
{
  return (__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf) __A,
						     __I,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) __W);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtps_ph (__mmask16 __W, __m512 __A, const int __I)
{
  return (__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf) __A,
						     __I,
						     (__v16hi)
						     _mm256_setzero_si256 (),
						     (__mmask16) __W);
}
#else
#define _mm512_cvt_roundps_pd(A, B)		 \
    (__m512d)__builtin_ia32_cvtps2pd512_mask(A, (__v8df)_mm512_undefined_pd(), -1, B)

#define _mm512_mask_cvt_roundps_pd(W, U, A, B)   \
    (__m512d)__builtin_ia32_cvtps2pd512_mask(A, (__v8df)(W), U, B)

#define _mm512_maskz_cvt_roundps_pd(U, A, B)     \
    (__m512d)__builtin_ia32_cvtps2pd512_mask(A, (__v8df)_mm512_setzero_pd(), U, B)

#define _mm512_cvt_roundph_ps(A, B)		 \
    (__m512)__builtin_ia32_vcvtph2ps512_mask((__v16hi)(A), (__v16sf)_mm512_undefined_ps(), -1, B)

#define _mm512_mask_cvt_roundph_ps(W, U, A, B)   \
    (__m512)__builtin_ia32_vcvtph2ps512_mask((__v16hi)(A), (__v16sf)(W), U, B)

#define _mm512_maskz_cvt_roundph_ps(U, A, B)     \
    (__m512)__builtin_ia32_vcvtph2ps512_mask((__v16hi)(A), (__v16sf)_mm512_setzero_ps(), U, B)

#define _mm512_cvt_roundps_ph(A, I)						 \
  ((__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf)(__m512) (A), (int) (I),\
    (__v16hi)_mm256_undefined_si256 (), -1))
#define _mm512_cvtps_ph(A, I)						 \
  ((__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf)(__m512) (A), (int) (I),\
    (__v16hi)_mm256_undefined_si256 (), -1))
#define _mm512_mask_cvt_roundps_ph(U, W, A, I)				 \
  ((__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf)(__m512) (A), (int) (I),\
    (__v16hi)(__m256i)(U), (__mmask16) (W)))
#define _mm512_mask_cvtps_ph(U, W, A, I)				 \
  ((__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf)(__m512) (A), (int) (I),\
    (__v16hi)(__m256i)(U), (__mmask16) (W)))
#define _mm512_maskz_cvt_roundps_ph(W, A, I)					 \
  ((__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf)(__m512) (A), (int) (I),\
    (__v16hi)_mm256_setzero_si256 (), (__mmask16) (W)))
#define _mm512_maskz_cvtps_ph(W, A, I)					 \
  ((__m256i) __builtin_ia32_vcvtps2ph512_mask ((__v16sf)(__m512) (A), (int) (I),\
    (__v16hi)_mm256_setzero_si256 (), (__mmask16) (W)))
#endif

#ifdef __OPTIMIZE__
extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvt_roundpd_ps (__m512d __A, const int __R)
{
  return (__m256) __builtin_ia32_cvtpd2ps512_mask ((__v8df) __A,
						   (__v8sf)
						   _mm256_undefined_ps (),
						   (__mmask8) -1, __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvt_roundpd_ps (__m256 __W, __mmask8 __U, __m512d __A,
			    const int __R)
{
  return (__m256) __builtin_ia32_cvtpd2ps512_mask ((__v8df) __A,
						   (__v8sf) __W,
						   (__mmask8) __U, __R);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvt_roundpd_ps (__mmask8 __U, __m512d __A, const int __R)
{
  return (__m256) __builtin_ia32_cvtpd2ps512_mask ((__v8df) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundsd_ss (__m128 __A, __m128d __B, const int __R)
{
  return (__m128) __builtin_ia32_cvtsd2ss_round ((__v4sf) __A,
						 (__v2df) __B,
						 __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvt_roundss_sd (__m128d __A, __m128 __B, const int __R)
{
  return (__m128d) __builtin_ia32_cvtss2sd_round ((__v2df) __A,
						  (__v4sf) __B,
						  __R);
}
#else
#define _mm512_cvt_roundpd_ps(A, B)		 \
    (__m256)__builtin_ia32_cvtpd2ps512_mask(A, (__v8sf)_mm256_undefined_ps(), -1, B)

#define _mm512_mask_cvt_roundpd_ps(W, U, A, B)   \
    (__m256)__builtin_ia32_cvtpd2ps512_mask(A, (__v8sf)(W), U, B)

#define _mm512_maskz_cvt_roundpd_ps(U, A, B)     \
    (__m256)__builtin_ia32_cvtpd2ps512_mask(A, (__v8sf)_mm256_setzero_ps(), U, B)

#define _mm_cvt_roundsd_ss(A, B, C)		 \
    (__m128)__builtin_ia32_cvtsd2ss_round(A, B, C)

#define _mm_cvt_roundss_sd(A, B, C)		 \
    (__m128d)__builtin_ia32_cvtss2sd_round(A, B, C)
#endif

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_stream_si512 (__m512i * __P, __m512i __A)
{
  __builtin_ia32_movntdq512 ((__v8di *) __P, (__v8di) __A);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_stream_ps (float *__P, __m512 __A)
{
  __builtin_ia32_movntps512 (__P, (__v16sf) __A);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_stream_pd (double *__P, __m512d __A)
{
  __builtin_ia32_movntpd512 (__P, (__v8df) __A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_stream_load_si512 (void *__P)
{
  return __builtin_ia32_movntdqa512 ((__v8di *)__P);
}

/* Constants for mantissa extraction */
typedef enum
{
  _MM_MANT_NORM_1_2,		/* interval [1, 2)      */
  _MM_MANT_NORM_p5_2,		/* interval [0.5, 2)    */
  _MM_MANT_NORM_p5_1,		/* interval [0.5, 1)    */
  _MM_MANT_NORM_p75_1p5		/* interval [0.75, 1.5) */
} _MM_MANTISSA_NORM_ENUM;

typedef enum
{
  _MM_MANT_SIGN_src,		/* sign = sign(SRC)     */
  _MM_MANT_SIGN_zero,		/* sign = 0             */
  _MM_MANT_SIGN_nan		/* DEST = NaN if sign(SRC) = 1 */
} _MM_MANTISSA_SIGN_ENUM;

#ifdef __OPTIMIZE__
extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getexp_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_getexpss128_round ((__v4sf) __A,
						    (__v4sf) __B,
						    __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getexp_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_getexpss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getexp_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			   const int __R)
{
  return (__m128) __builtin_ia32_getexpss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getexp_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_getexpsd128_round ((__v2df) __A,
						     (__v2df) __B,
						     __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getexp_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_getexpsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getexp_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_getexpsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getexp_round_ps (__m512 __A, const int __R)
{
  return (__m512) __builtin_ia32_getexpps512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getexp_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			     const int __R)
{
  return (__m512) __builtin_ia32_getexpps512_mask ((__v16sf) __A,
						   (__v16sf) __W,
						   (__mmask16) __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getexp_round_ps (__mmask16 __U, __m512 __A, const int __R)
{
  return (__m512) __builtin_ia32_getexpps512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getexp_round_pd (__m512d __A, const int __R)
{
  return (__m512d) __builtin_ia32_getexppd512_mask ((__v8df) __A,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getexp_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			     const int __R)
{
  return (__m512d) __builtin_ia32_getexppd512_mask ((__v8df) __A,
						    (__v8df) __W,
						    (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getexp_round_pd (__mmask8 __U, __m512d __A, const int __R)
{
  return (__m512d) __builtin_ia32_getexppd512_mask ((__v8df) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getmant_round_pd (__m512d __A, _MM_MANTISSA_NORM_ENUM __B,
			 _MM_MANTISSA_SIGN_ENUM __C, const int __R)
{
  return (__m512d) __builtin_ia32_getmantpd512_mask ((__v8df) __A,
						     (__C << 2) | __B,
						     _mm512_undefined_pd (),
						     (__mmask8) -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getmant_round_pd (__m512d __W, __mmask8 __U, __m512d __A,
			      _MM_MANTISSA_NORM_ENUM __B,
			      _MM_MANTISSA_SIGN_ENUM __C, const int __R)
{
  return (__m512d) __builtin_ia32_getmantpd512_mask ((__v8df) __A,
						     (__C << 2) | __B,
						     (__v8df) __W, __U,
						     __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getmant_round_pd (__mmask8 __U, __m512d __A,
			       _MM_MANTISSA_NORM_ENUM __B,
			       _MM_MANTISSA_SIGN_ENUM __C, const int __R)
{
  return (__m512d) __builtin_ia32_getmantpd512_mask ((__v8df) __A,
						     (__C << 2) | __B,
						     (__v8df)
						     _mm512_setzero_pd (),
						     __U, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getmant_round_ps (__m512 __A, _MM_MANTISSA_NORM_ENUM __B,
			 _MM_MANTISSA_SIGN_ENUM __C, const int __R)
{
  return (__m512) __builtin_ia32_getmantps512_mask ((__v16sf) __A,
						    (__C << 2) | __B,
						    _mm512_undefined_ps (),
						    (__mmask16) -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getmant_round_ps (__m512 __W, __mmask16 __U, __m512 __A,
			      _MM_MANTISSA_NORM_ENUM __B,
			      _MM_MANTISSA_SIGN_ENUM __C, const int __R)
{
  return (__m512) __builtin_ia32_getmantps512_mask ((__v16sf) __A,
						    (__C << 2) | __B,
						    (__v16sf) __W, __U,
						    __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getmant_round_ps (__mmask16 __U, __m512 __A,
			       _MM_MANTISSA_NORM_ENUM __B,
			       _MM_MANTISSA_SIGN_ENUM __C, const int __R)
{
  return (__m512) __builtin_ia32_getmantps512_mask ((__v16sf) __A,
						    (__C << 2) | __B,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getmant_round_sd (__m128d __A, __m128d __B,
		      _MM_MANTISSA_NORM_ENUM __C,
		      _MM_MANTISSA_SIGN_ENUM __D, const int __R)
{
  return (__m128d) __builtin_ia32_getmantsd_round ((__v2df) __A,
						  (__v2df) __B,
						  (__D << 2) | __C,
						   __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getmant_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			      __m128d __B, _MM_MANTISSA_NORM_ENUM __C,
			      _MM_MANTISSA_SIGN_ENUM __D, const int __R)
{
  return (__m128d) __builtin_ia32_getmantsd_mask_round ((__v2df) __A,
						    (__v2df) __B,
						    (__D << 2) | __C,
                                                    (__v2df) __W,
						     __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getmant_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			       _MM_MANTISSA_NORM_ENUM __C,
			       _MM_MANTISSA_SIGN_ENUM __D, const int __R)
{
  return (__m128d) __builtin_ia32_getmantsd_mask_round ((__v2df) __A,
							(__v2df) __B,
						        (__D << 2) | __C,
                                                        (__v2df)
                                                        _mm_setzero_pd(),
						        __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getmant_round_ss (__m128 __A, __m128 __B,
		      _MM_MANTISSA_NORM_ENUM __C,
		      _MM_MANTISSA_SIGN_ENUM __D, const int __R)
{
  return (__m128) __builtin_ia32_getmantss_round ((__v4sf) __A,
						  (__v4sf) __B,
						  (__D << 2) | __C,
						  __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getmant_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			      __m128 __B, _MM_MANTISSA_NORM_ENUM __C,
			      _MM_MANTISSA_SIGN_ENUM __D, const int __R)
{
  return (__m128) __builtin_ia32_getmantss_mask_round ((__v4sf) __A,
						    (__v4sf) __B,
						    (__D << 2) | __C,
                                                    (__v4sf) __W,
						     __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getmant_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			       _MM_MANTISSA_NORM_ENUM __C,
			       _MM_MANTISSA_SIGN_ENUM __D, const int __R)
{
  return (__m128) __builtin_ia32_getmantss_mask_round ((__v4sf) __A,
							(__v4sf) __B,
						        (__D << 2) | __C,
                                                        (__v4sf)
                                                        _mm_setzero_ps(),
						        __U, __R);
}

#else
#define _mm512_getmant_round_pd(X, B, C, R)                                                  \
  ((__m512d)__builtin_ia32_getmantpd512_mask ((__v8df)(__m512d)(X),                 \
                                              (int)(((C)<<2) | (B)),                \
                                              (__v8df)(__m512d)_mm512_undefined_pd(), \
                                              (__mmask8)-1,\
					      (R)))

#define _mm512_mask_getmant_round_pd(W, U, X, B, C, R)                                       \
  ((__m512d)__builtin_ia32_getmantpd512_mask ((__v8df)(__m512d)(X),                 \
                                              (int)(((C)<<2) | (B)),                \
                                              (__v8df)(__m512d)(W),                 \
                                              (__mmask8)(U),\
					      (R)))

#define _mm512_maskz_getmant_round_pd(U, X, B, C, R)                                         \
  ((__m512d)__builtin_ia32_getmantpd512_mask ((__v8df)(__m512d)(X),                 \
                                              (int)(((C)<<2) | (B)),                \
                                              (__v8df)(__m512d)_mm512_setzero_pd(), \
                                              (__mmask8)(U),\
					      (R)))
#define _mm512_getmant_round_ps(X, B, C, R)                                                  \
  ((__m512)__builtin_ia32_getmantps512_mask ((__v16sf)(__m512)(X),                  \
                                             (int)(((C)<<2) | (B)),                 \
                                             (__v16sf)(__m512)_mm512_undefined_ps(), \
                                             (__mmask16)-1,\
					     (R)))

#define _mm512_mask_getmant_round_ps(W, U, X, B, C, R)                                       \
  ((__m512)__builtin_ia32_getmantps512_mask ((__v16sf)(__m512)(X),                  \
                                             (int)(((C)<<2) | (B)),                 \
                                             (__v16sf)(__m512)(W),                  \
                                             (__mmask16)(U),\
					     (R)))

#define _mm512_maskz_getmant_round_ps(U, X, B, C, R)                                         \
  ((__m512)__builtin_ia32_getmantps512_mask ((__v16sf)(__m512)(X),                  \
                                             (int)(((C)<<2) | (B)),                 \
                                             (__v16sf)(__m512)_mm512_setzero_ps(),  \
                                             (__mmask16)(U),\
					     (R)))
#define _mm_getmant_round_sd(X, Y, C, D, R)                                                  \
  ((__m128d)__builtin_ia32_getmantsd_round ((__v2df)(__m128d)(X),                    \
					    (__v2df)(__m128d)(Y),	\
					    (int)(((D)<<2) | (C)),	\
					    (R)))

#define _mm_mask_getmant_round_sd(W, U, X, Y, C, D, R)                                       \
  ((__m128d)__builtin_ia32_getmantsd_mask_round ((__v2df)(__m128d)(X),                  \
					     (__v2df)(__m128d)(Y),                  \
                                             (int)(((D)<<2) | (C)),                 \
                                             (__v2df)(__m128d)(W),                   \
                                             (__mmask8)(U),\
					     (R)))

#define _mm_maskz_getmant_round_sd(U, X, Y, C, D, R)                                         \
  ((__m128d)__builtin_ia32_getmantsd_mask_round ((__v2df)(__m128d)(X),                  \
                                                 (__v2df)(__m128d)(Y),                  \
                                             (int)(((D)<<2) | (C)),              \
                                             (__v2df)(__m128d)_mm_setzero_pd(),  \
                                             (__mmask8)(U),\
					     (R)))

#define _mm_getmant_round_ss(X, Y, C, D, R)                                                  \
  ((__m128)__builtin_ia32_getmantss_round ((__v4sf)(__m128)(X),                      \
					   (__v4sf)(__m128)(Y),		\
					   (int)(((D)<<2) | (C)),	\
					   (R)))

#define _mm_mask_getmant_round_ss(W, U, X, Y, C, D, R)                                       \
  ((__m128)__builtin_ia32_getmantss_mask_round ((__v4sf)(__m128)(X),                  \
					     (__v4sf)(__m128)(Y),                  \
                                             (int)(((D)<<2) | (C)),                 \
                                             (__v4sf)(__m128)(W),                   \
                                             (__mmask8)(U),\
					     (R)))

#define _mm_maskz_getmant_round_ss(U, X, Y, C, D, R)                                         \
  ((__m128)__builtin_ia32_getmantss_mask_round ((__v4sf)(__m128)(X),                  \
                                                 (__v4sf)(__m128)(Y),                  \
                                             (int)(((D)<<2) | (C)),              \
                                             (__v4sf)(__m128)_mm_setzero_ps(),  \
                                             (__mmask8)(U),\
					     (R)))

#define _mm_getexp_round_ss(A, B, R)						      \
  ((__m128)__builtin_ia32_getexpss128_round((__v4sf)(__m128)(A), (__v4sf)(__m128)(B), R))

#define _mm_mask_getexp_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_getexpss_mask_round(A, B, W, U, C)

#define _mm_maskz_getexp_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_getexpss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U, C)

#define _mm_getexp_round_sd(A, B, R)						       \
  ((__m128d)__builtin_ia32_getexpsd128_round((__v2df)(__m128d)(A), (__v2df)(__m128d)(B), R))

#define _mm_mask_getexp_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_getexpsd_mask_round(A, B, W, U, C)

#define _mm_maskz_getexp_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_getexpsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U, C)


#define _mm512_getexp_round_ps(A, R)						\
  ((__m512)__builtin_ia32_getexpps512_mask((__v16sf)(__m512)(A),		\
  (__v16sf)_mm512_undefined_ps(), (__mmask16)-1, R))

#define _mm512_mask_getexp_round_ps(W, U, A, R)					\
  ((__m512)__builtin_ia32_getexpps512_mask((__v16sf)(__m512)(A),		\
  (__v16sf)(__m512)(W), (__mmask16)(U), R))

#define _mm512_maskz_getexp_round_ps(U, A, R)					\
  ((__m512)__builtin_ia32_getexpps512_mask((__v16sf)(__m512)(A),		\
  (__v16sf)_mm512_setzero_ps(), (__mmask16)(U), R))

#define _mm512_getexp_round_pd(A, R)						\
  ((__m512d)__builtin_ia32_getexppd512_mask((__v8df)(__m512d)(A),		\
  (__v8df)_mm512_undefined_pd(), (__mmask8)-1, R))

#define _mm512_mask_getexp_round_pd(W, U, A, R)					\
  ((__m512d)__builtin_ia32_getexppd512_mask((__v8df)(__m512d)(A),		\
  (__v8df)(__m512d)(W), (__mmask8)(U), R))

#define _mm512_maskz_getexp_round_pd(U, A, R)					\
  ((__m512d)__builtin_ia32_getexppd512_mask((__v8df)(__m512d)(A),		\
  (__v8df)_mm512_setzero_pd(), (__mmask8)(U), R))
#endif

#ifdef __OPTIMIZE__
extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_roundscale_round_ps (__m512 __A, const int __imm, const int __R)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __A, __imm,
						  (__v16sf)
						  _mm512_undefined_ps (),
						  -1, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_roundscale_round_ps (__m512 __A, __mmask16 __B, __m512 __C,
				 const int __imm, const int __R)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __C, __imm,
						  (__v16sf) __A,
						  (__mmask16) __B, __R);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_roundscale_round_ps (__mmask16 __A, __m512 __B,
				  const int __imm, const int __R)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __B,
						  __imm,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __A, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_roundscale_round_pd (__m512d __A, const int __imm, const int __R)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __A, __imm,
						   (__v8df)
						   _mm512_undefined_pd (),
						   -1, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_roundscale_round_pd (__m512d __A, __mmask8 __B,
				 __m512d __C, const int __imm, const int __R)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __C, __imm,
						   (__v8df) __A,
						   (__mmask8) __B, __R);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_roundscale_round_pd (__mmask8 __A, __m512d __B,
				  const int __imm, const int __R)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __B,
						   __imm,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __A, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_roundscale_round_ss (__m128 __A, __m128 __B, const int __imm,
			 const int __R)
{
  return (__m128)
    __builtin_ia32_rndscaless_mask_round ((__v4sf) __A,
					  (__v4sf) __B, __imm,
					  (__v4sf)
					  _mm_setzero_ps (),
					  (__mmask8) -1,
					  __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_roundscale_round_ss (__m128 __A, __mmask8 __B, __m128 __C,
			      __m128 __D, const int __imm, const int __R)
{
  return (__m128)
    __builtin_ia32_rndscaless_mask_round ((__v4sf) __C,
					  (__v4sf) __D, __imm,
					  (__v4sf) __A,
					  (__mmask8) __B,
					  __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_roundscale_round_ss (__mmask8 __A, __m128 __B, __m128 __C,
			       const int __imm, const int __R)
{
  return (__m128)
    __builtin_ia32_rndscaless_mask_round ((__v4sf) __B,
					  (__v4sf) __C, __imm,
					  (__v4sf)
					  _mm_setzero_ps (),
					  (__mmask8) __A,
					  __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_roundscale_round_sd (__m128d __A, __m128d __B, const int __imm,
			 const int __R)
{
  return (__m128d)
    __builtin_ia32_rndscalesd_mask_round ((__v2df) __A,
					  (__v2df) __B, __imm,
					  (__v2df)
					  _mm_setzero_pd (),
					  (__mmask8) -1,
					  __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_roundscale_round_sd (__m128d __A, __mmask8 __B, __m128d __C,
			      __m128d __D, const int __imm, const int __R)
{
  return (__m128d)
    __builtin_ia32_rndscalesd_mask_round ((__v2df) __C,
					  (__v2df) __D, __imm,
					  (__v2df) __A,
					  (__mmask8) __B,
					  __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_roundscale_round_sd (__mmask8 __A, __m128d __B, __m128d __C,
			       const int __imm, const int __R)
{
  return (__m128d)
    __builtin_ia32_rndscalesd_mask_round ((__v2df) __B,
					  (__v2df) __C, __imm,
					  (__v2df)
					  _mm_setzero_pd (),
					  (__mmask8) __A,
					  __R);
}

#else
#define _mm512_roundscale_round_ps(A, B, R) \
  ((__m512) __builtin_ia32_rndscaleps_mask ((__v16sf)(__m512)(A), (int)(B),\
    (__v16sf)_mm512_undefined_ps(), (__mmask16)(-1), R))
#define _mm512_mask_roundscale_round_ps(A, B, C, D, R)				\
  ((__m512) __builtin_ia32_rndscaleps_mask ((__v16sf)(__m512)(C),	\
					    (int)(D),			\
					    (__v16sf)(__m512)(A),	\
					    (__mmask16)(B), R))
#define _mm512_maskz_roundscale_round_ps(A, B, C, R)				\
  ((__m512) __builtin_ia32_rndscaleps_mask ((__v16sf)(__m512)(B),	\
					    (int)(C),			\
					    (__v16sf)_mm512_setzero_ps(),\
					    (__mmask16)(A), R))
#define _mm512_roundscale_round_pd(A, B, R) \
  ((__m512d) __builtin_ia32_rndscalepd_mask ((__v8df)(__m512d)(A), (int)(B),\
    (__v8df)_mm512_undefined_pd(), (__mmask8)(-1), R))
#define _mm512_mask_roundscale_round_pd(A, B, C, D, R)				\
  ((__m512d) __builtin_ia32_rndscalepd_mask ((__v8df)(__m512d)(C),	\
					     (int)(D),			\
					     (__v8df)(__m512d)(A),	\
					     (__mmask8)(B), R))
#define _mm512_maskz_roundscale_round_pd(A, B, C, R)				\
  ((__m512d) __builtin_ia32_rndscalepd_mask ((__v8df)(__m512d)(B),	\
					     (int)(C),			\
					     (__v8df)_mm512_setzero_pd(),\
					     (__mmask8)(A), R))
#define _mm_roundscale_round_ss(A, B, I, R)				\
  ((__m128)								\
   __builtin_ia32_rndscaless_mask_round ((__v4sf) (__m128) (A),		\
					 (__v4sf) (__m128) (B),		\
					 (int) (I),			\
					 (__v4sf) _mm_setzero_ps (),	\
					 (__mmask8) (-1),		\
					 (int) (R)))
#define _mm_mask_roundscale_round_ss(A, U, B, C, I, R)		\
  ((__m128)							\
   __builtin_ia32_rndscaless_mask_round ((__v4sf) (__m128) (B),	\
					 (__v4sf) (__m128) (C),	\
					 (int) (I),		\
					 (__v4sf) (__m128) (A),	\
					 (__mmask8) (U),	\
					 (int) (R)))
#define _mm_maskz_roundscale_round_ss(U, A, B, I, R)			\
  ((__m128)								\
   __builtin_ia32_rndscaless_mask_round ((__v4sf) (__m128) (A),		\
					 (__v4sf) (__m128) (B),		\
					 (int) (I),			\
					 (__v4sf) _mm_setzero_ps (),	\
					 (__mmask8) (U),		\
					 (int) (R)))
#define _mm_roundscale_round_sd(A, B, I, R)				\
  ((__m128d)								\
   __builtin_ia32_rndscalesd_mask_round ((__v2df) (__m128d) (A),	\
					 (__v2df) (__m128d) (B),	\
					 (int) (I),			\
					 (__v2df) _mm_setzero_pd (),	\
					 (__mmask8) (-1),		\
					 (int) (R)))
#define _mm_mask_roundscale_round_sd(A, U, B, C, I, R)			\
  ((__m128d)								\
   __builtin_ia32_rndscalesd_mask_round ((__v2df) (__m128d) (B),	\
					 (__v2df) (__m128d) (C),	\
					 (int) (I),			\
					 (__v2df) (__m128d) (A),	\
					 (__mmask8) (U),		\
					 (int) (R)))
#define _mm_maskz_roundscale_round_sd(U, A, B, I, R)			\
  ((__m128d)								\
   __builtin_ia32_rndscalesd_mask_round ((__v2df) (__m128d) (A),	\
					 (__v2df) (__m128d) (B),	\
					 (int) (I),			\
					 (__v2df) _mm_setzero_pd (),	\
					 (__mmask8) (U),		\
					 (int) (R)))
#endif

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_floor_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __A,
						  _MM_FROUND_FLOOR,
						  (__v16sf) __A, -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_floor_pd (__m512d __A)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __A,
						   _MM_FROUND_FLOOR,
						   (__v8df) __A, -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_ceil_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __A,
						  _MM_FROUND_CEIL,
						  (__v16sf) __A, -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_ceil_pd (__m512d __A)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __A,
						   _MM_FROUND_CEIL,
						   (__v8df) __A, -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_floor_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __A,
						  _MM_FROUND_FLOOR,
						  (__v16sf) __W, __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_floor_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __A,
						   _MM_FROUND_FLOOR,
						   (__v8df) __W, __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_ceil_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __A,
						  _MM_FROUND_CEIL,
						  (__v16sf) __W, __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_ceil_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __A,
						   _MM_FROUND_CEIL,
						   (__v8df) __W, __U,
						   _MM_FROUND_CUR_DIRECTION);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_alignr_epi32 (__m512i __A, __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_alignd512_mask ((__v16si) __A,
						  (__v16si) __B, __imm,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_alignr_epi32 (__m512i __W, __mmask16 __U, __m512i __A,
			  __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_alignd512_mask ((__v16si) __A,
						  (__v16si) __B, __imm,
						  (__v16si) __W,
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_alignr_epi32 (__mmask16 __U, __m512i __A, __m512i __B,
			   const int __imm)
{
  return (__m512i) __builtin_ia32_alignd512_mask ((__v16si) __A,
						  (__v16si) __B, __imm,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_alignr_epi64 (__m512i __A, __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_alignq512_mask ((__v8di) __A,
						  (__v8di) __B, __imm,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_alignr_epi64 (__m512i __W, __mmask8 __U, __m512i __A,
			  __m512i __B, const int __imm)
{
  return (__m512i) __builtin_ia32_alignq512_mask ((__v8di) __A,
						  (__v8di) __B, __imm,
						  (__v8di) __W,
						  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_alignr_epi64 (__mmask8 __U, __m512i __A, __m512i __B,
			   const int __imm)
{
  return (__m512i) __builtin_ia32_alignq512_mask ((__v8di) __A,
						  (__v8di) __B, __imm,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  (__mmask8) __U);
}
#else
#define _mm512_alignr_epi32(X, Y, C)                                        \
    ((__m512i)__builtin_ia32_alignd512_mask ((__v16si)(__m512i)(X),         \
        (__v16si)(__m512i)(Y), (int)(C), (__v16si)_mm512_undefined_epi32 (),\
        (__mmask16)-1))

#define _mm512_mask_alignr_epi32(W, U, X, Y, C)                             \
    ((__m512i)__builtin_ia32_alignd512_mask ((__v16si)(__m512i)(X),         \
        (__v16si)(__m512i)(Y), (int)(C), (__v16si)(__m512i)(W),             \
        (__mmask16)(U)))

#define _mm512_maskz_alignr_epi32(U, X, Y, C)                               \
    ((__m512i)__builtin_ia32_alignd512_mask ((__v16si)(__m512i)(X),         \
        (__v16si)(__m512i)(Y), (int)(C), (__v16si)_mm512_setzero_si512 (),\
        (__mmask16)(U)))

#define _mm512_alignr_epi64(X, Y, C)                                        \
    ((__m512i)__builtin_ia32_alignq512_mask ((__v8di)(__m512i)(X),          \
        (__v8di)(__m512i)(Y), (int)(C), (__v8di)_mm512_undefined_epi32 (),  \
	(__mmask8)-1))

#define _mm512_mask_alignr_epi64(W, U, X, Y, C)                             \
    ((__m512i)__builtin_ia32_alignq512_mask ((__v8di)(__m512i)(X),          \
        (__v8di)(__m512i)(Y), (int)(C), (__v8di)(__m512i)(W), (__mmask8)(U)))

#define _mm512_maskz_alignr_epi64(U, X, Y, C)                               \
    ((__m512i)__builtin_ia32_alignq512_mask ((__v8di)(__m512i)(X),          \
        (__v8di)(__m512i)(Y), (int)(C), (__v8di)_mm512_setzero_si512 (),\
        (__mmask8)(U)))
#endif

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epi32_mask (__m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_pcmpeqd512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epi32_mask (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_pcmpeqd512_mask ((__v16si) __A,
						     (__v16si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epi64_mask (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqq512_mask ((__v8di) __A,
						    (__v8di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epi64_mask (__m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_pcmpeqq512_mask ((__v8di) __A,
						    (__v8di) __B,
						    (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epi32_mask (__m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_pcmpgtd512_mask ((__v16si) __A,
						     (__v16si) __B,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epi32_mask (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_pcmpgtd512_mask ((__v16si) __A,
						     (__v16si) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epi64_mask (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtq512_mask ((__v8di) __A,
						    (__v8di) __B, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epi64_mask (__m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_pcmpgtq512_mask ((__v8di) __A,
						    (__v8di) __B,
						    (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epi32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 5,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epi32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 5,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epu32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 5,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epu32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 5,
						    (__mmask16) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epi64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 5,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epi64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 5,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpge_epu64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 5,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpge_epu64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 5,
						    (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epi32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 2,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epi32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 2,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epu32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 2,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epu32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 2,
						    (__mmask16) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epi64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 2,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epi64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 2,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_epu64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 2,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_epu64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 2,
						    (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epi32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 1,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epi32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 1,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epu32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 1,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epu32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 1,
						    (__mmask16) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epi64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 1,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epi64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 1,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_epu64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 1,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_epu64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 1,
						    (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epi32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 4,
						    (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epi32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 4,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epu32_mask (__mmask16 __M, __m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 4,
						    (__mmask16) __M);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epu32_mask (__m512i __X, __m512i __Y)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						    (__v16si) __Y, 4,
						    (__mmask16) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epi64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 4,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epi64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 4,
						    (__mmask8) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_epu64_mask (__mmask8 __M, __m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 4,
						    (__mmask8) __M);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_epu64_mask (__m512i __X, __m512i __Y)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						    (__v8di) __Y, 4,
						    (__mmask8) -1);
}

#define _MM_CMPINT_EQ	    0x0
#define _MM_CMPINT_LT	    0x1
#define _MM_CMPINT_LE	    0x2
#define _MM_CMPINT_UNUSED   0x3
#define _MM_CMPINT_NE	    0x4
#define _MM_CMPINT_NLT	    0x5
#define _MM_CMPINT_GE	    0x5
#define _MM_CMPINT_NLE	    0x6
#define _MM_CMPINT_GT	    0x6

#ifdef __OPTIMIZE__
extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftli_mask16 (__mmask16 __A, unsigned int __B)
{
  return (__mmask16) __builtin_ia32_kshiftlihi ((__mmask16) __A,
						(__mmask8) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kshiftri_mask16 (__mmask16 __A, unsigned int __B)
{
  return (__mmask16) __builtin_ia32_kshiftrihi ((__mmask16) __A,
						(__mmask8) __B);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epi64_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						 (__v8di) __Y, __P,
						 (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epi32_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						  (__v16si) __Y, __P,
						  (__mmask16) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epu64_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						  (__v8di) __Y, __P,
						  (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_epu32_mask (__m512i __X, __m512i __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						   (__v16si) __Y, __P,
						   (__mmask16) -1);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_round_pd_mask (__m512d __X, __m512d __Y, const int __P,
			  const int __R)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, __P,
						  (__mmask8) -1, __R);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_round_ps_mask (__m512 __X, __m512 __Y, const int __P, const int __R)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, __P,
						   (__mmask16) -1, __R);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epi64_mask (__mmask8 __U, __m512i __X, __m512i __Y,
			    const int __P)
{
  return (__mmask8) __builtin_ia32_cmpq512_mask ((__v8di) __X,
						 (__v8di) __Y, __P,
						 (__mmask8) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epi32_mask (__mmask16 __U, __m512i __X, __m512i __Y,
			    const int __P)
{
  return (__mmask16) __builtin_ia32_cmpd512_mask ((__v16si) __X,
						  (__v16si) __Y, __P,
						  (__mmask16) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epu64_mask (__mmask8 __U, __m512i __X, __m512i __Y,
			    const int __P)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __X,
						  (__v8di) __Y, __P,
						  (__mmask8) __U);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_epu32_mask (__mmask16 __U, __m512i __X, __m512i __Y,
			    const int __P)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __X,
						   (__v16si) __Y, __P,
						   (__mmask16) __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_round_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y,
			       const int __P, const int __R)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, __P,
						  (__mmask8) __U, __R);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_round_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y,
			       const int __P, const int __R)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, __P,
						   (__mmask16) __U, __R);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_round_sd_mask (__m128d __X, __m128d __Y, const int __P, const int __R)
{
  return (__mmask8) __builtin_ia32_cmpsd_mask ((__v2df) __X,
					       (__v2df) __Y, __P,
					       (__mmask8) -1, __R);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_round_sd_mask (__mmask8 __M, __m128d __X, __m128d __Y,
			    const int __P, const int __R)
{
  return (__mmask8) __builtin_ia32_cmpsd_mask ((__v2df) __X,
					       (__v2df) __Y, __P,
					       (__mmask8) __M, __R);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_round_ss_mask (__m128 __X, __m128 __Y, const int __P, const int __R)
{
  return (__mmask8) __builtin_ia32_cmpss_mask ((__v4sf) __X,
					       (__v4sf) __Y, __P,
					       (__mmask8) -1, __R);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_round_ss_mask (__mmask8 __M, __m128 __X, __m128 __Y,
			    const int __P, const int __R)
{
  return (__mmask8) __builtin_ia32_cmpss_mask ((__v4sf) __X,
					       (__v4sf) __Y, __P,
					       (__mmask8) __M, __R);
}

#else
#define _kshiftli_mask16(X, Y)						\
  ((__mmask16) __builtin_ia32_kshiftlihi ((__mmask16)(X), (__mmask8)(Y)))

#define _kshiftri_mask16(X, Y)						\
  ((__mmask16) __builtin_ia32_kshiftrihi ((__mmask16)(X), (__mmask8)(Y)))

#define _mm512_cmp_epi64_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpq512_mask ((__v8di)(__m512i)(X),	\
					   (__v8di)(__m512i)(Y), (int)(P),\
					   (__mmask8)-1))

#define _mm512_cmp_epi32_mask(X, Y, P)					\
  ((__mmask16) __builtin_ia32_cmpd512_mask ((__v16si)(__m512i)(X),	\
					    (__v16si)(__m512i)(Y), (int)(P), \
					    (__mmask16)-1))

#define _mm512_cmp_epu64_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di)(__m512i)(X),	\
					    (__v8di)(__m512i)(Y), (int)(P),\
					    (__mmask8)-1))

#define _mm512_cmp_epu32_mask(X, Y, P)					\
  ((__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si)(__m512i)(X),	\
					     (__v16si)(__m512i)(Y), (int)(P), \
					     (__mmask16)-1))

#define _mm512_cmp_round_pd_mask(X, Y, P, R)				\
  ((__mmask8) __builtin_ia32_cmppd512_mask ((__v8df)(__m512d)(X),	\
					    (__v8df)(__m512d)(Y), (int)(P),\
					    (__mmask8)-1, R))

#define _mm512_cmp_round_ps_mask(X, Y, P, R)				\
  ((__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf)(__m512)(X),	\
					     (__v16sf)(__m512)(Y), (int)(P),\
					     (__mmask16)-1, R))

#define _mm512_mask_cmp_epi64_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_cmpq512_mask ((__v8di)(__m512i)(X),	\
					   (__v8di)(__m512i)(Y), (int)(P),\
					   (__mmask8)(M)))

#define _mm512_mask_cmp_epi32_mask(M, X, Y, P)				\
  ((__mmask16) __builtin_ia32_cmpd512_mask ((__v16si)(__m512i)(X),	\
					    (__v16si)(__m512i)(Y), (int)(P), \
					    (__mmask16)(M)))

#define _mm512_mask_cmp_epu64_mask(M, X, Y, P)				\
  ((__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di)(__m512i)(X),	\
					    (__v8di)(__m512i)(Y), (int)(P),\
					    (__mmask8)(M)))

#define _mm512_mask_cmp_epu32_mask(M, X, Y, P)				\
  ((__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si)(__m512i)(X),	\
					     (__v16si)(__m512i)(Y), (int)(P), \
					     (__mmask16)(M)))

#define _mm512_mask_cmp_round_pd_mask(M, X, Y, P, R)			\
  ((__mmask8) __builtin_ia32_cmppd512_mask ((__v8df)(__m512d)(X),	\
					    (__v8df)(__m512d)(Y), (int)(P),\
					    (__mmask8)(M), R))

#define _mm512_mask_cmp_round_ps_mask(M, X, Y, P, R)			\
  ((__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf)(__m512)(X),	\
					     (__v16sf)(__m512)(Y), (int)(P),\
					     (__mmask16)(M), R))

#define _mm_cmp_round_sd_mask(X, Y, P, R)				\
  ((__mmask8) __builtin_ia32_cmpsd_mask ((__v2df)(__m128d)(X),		\
					 (__v2df)(__m128d)(Y), (int)(P),\
					 (__mmask8)-1, R))

#define _mm_mask_cmp_round_sd_mask(M, X, Y, P, R)			\
  ((__mmask8) __builtin_ia32_cmpsd_mask ((__v2df)(__m128d)(X),		\
					 (__v2df)(__m128d)(Y), (int)(P),\
					 (M), R))

#define _mm_cmp_round_ss_mask(X, Y, P, R)				\
  ((__mmask8) __builtin_ia32_cmpss_mask ((__v4sf)(__m128)(X),		\
					 (__v4sf)(__m128)(Y), (int)(P), \
					 (__mmask8)-1, R))

#define _mm_mask_cmp_round_ss_mask(M, X, Y, P, R)			\
  ((__mmask8) __builtin_ia32_cmpss_mask ((__v4sf)(__m128)(X),		\
					 (__v4sf)(__m128)(Y), (int)(P), \
					 (M), R))
#endif

#ifdef __OPTIMIZE__
extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32gather_ps (__m512i __index, void const *__addr, int __scale)
{
  __m512 __v1_old = _mm512_undefined_ps ();
  __mmask16 __mask = 0xFFFF;

  return (__m512) __builtin_ia32_gathersiv16sf ((__v16sf) __v1_old,
						__addr,
						(__v16si) __index,
						__mask, __scale);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32gather_ps (__m512 __v1_old, __mmask16 __mask,
			  __m512i __index, void const *__addr, int __scale)
{
  return (__m512) __builtin_ia32_gathersiv16sf ((__v16sf) __v1_old,
						__addr,
						(__v16si) __index,
						__mask, __scale);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32gather_pd (__m256i __index, void const *__addr, int __scale)
{
  __m512d __v1_old = _mm512_undefined_pd ();
  __mmask8 __mask = 0xFF;

  return (__m512d) __builtin_ia32_gathersiv8df ((__v8df) __v1_old,
						__addr,
						(__v8si) __index, __mask,
						__scale);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32gather_pd (__m512d __v1_old, __mmask8 __mask,
			  __m256i __index, void const *__addr, int __scale)
{
  return (__m512d) __builtin_ia32_gathersiv8df ((__v8df) __v1_old,
						__addr,
						(__v8si) __index,
						__mask, __scale);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64gather_ps (__m512i __index, void const *__addr, int __scale)
{
  __m256 __v1_old = _mm256_undefined_ps ();
  __mmask8 __mask = 0xFF;

  return (__m256) __builtin_ia32_gatherdiv16sf ((__v8sf) __v1_old,
						__addr,
						(__v8di) __index, __mask,
						__scale);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64gather_ps (__m256 __v1_old, __mmask8 __mask,
			  __m512i __index, void const *__addr, int __scale)
{
  return (__m256) __builtin_ia32_gatherdiv16sf ((__v8sf) __v1_old,
						__addr,
						(__v8di) __index,
						__mask, __scale);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64gather_pd (__m512i __index, void const *__addr, int __scale)
{
  __m512d __v1_old = _mm512_undefined_pd ();
  __mmask8 __mask = 0xFF;

  return (__m512d) __builtin_ia32_gatherdiv8df ((__v8df) __v1_old,
						__addr,
						(__v8di) __index, __mask,
						__scale);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64gather_pd (__m512d __v1_old, __mmask8 __mask,
			  __m512i __index, void const *__addr, int __scale)
{
  return (__m512d) __builtin_ia32_gatherdiv8df ((__v8df) __v1_old,
						__addr,
						(__v8di) __index,
						__mask, __scale);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32gather_epi32 (__m512i __index, void const *__addr, int __scale)
{
  __m512i __v1_old = _mm512_undefined_epi32 ();
  __mmask16 __mask = 0xFFFF;

  return (__m512i) __builtin_ia32_gathersiv16si ((__v16si) __v1_old,
						 __addr,
						 (__v16si) __index,
						 __mask, __scale);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32gather_epi32 (__m512i __v1_old, __mmask16 __mask,
			     __m512i __index, void const *__addr, int __scale)
{
  return (__m512i) __builtin_ia32_gathersiv16si ((__v16si) __v1_old,
						 __addr,
						 (__v16si) __index,
						 __mask, __scale);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32gather_epi64 (__m256i __index, void const *__addr, int __scale)
{
  __m512i __v1_old = _mm512_undefined_epi32 ();
  __mmask8 __mask = 0xFF;

  return (__m512i) __builtin_ia32_gathersiv8di ((__v8di) __v1_old,
						__addr,
						(__v8si) __index, __mask,
						__scale);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32gather_epi64 (__m512i __v1_old, __mmask8 __mask,
			     __m256i __index, void const *__addr,
			     int __scale)
{
  return (__m512i) __builtin_ia32_gathersiv8di ((__v8di) __v1_old,
						__addr,
						(__v8si) __index,
						__mask, __scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64gather_epi32 (__m512i __index, void const *__addr, int __scale)
{
  __m256i __v1_old = _mm256_undefined_si256 ();
  __mmask8 __mask = 0xFF;

  return (__m256i) __builtin_ia32_gatherdiv16si ((__v8si) __v1_old,
						 __addr,
						 (__v8di) __index,
						 __mask, __scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64gather_epi32 (__m256i __v1_old, __mmask8 __mask,
			     __m512i __index, void const *__addr, int __scale)
{
  return (__m256i) __builtin_ia32_gatherdiv16si ((__v8si) __v1_old,
						 __addr,
						 (__v8di) __index,
						 __mask, __scale);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64gather_epi64 (__m512i __index, void const *__addr, int __scale)
{
  __m512i __v1_old = _mm512_undefined_epi32 ();
  __mmask8 __mask = 0xFF;

  return (__m512i) __builtin_ia32_gatherdiv8di ((__v8di) __v1_old,
						__addr,
						(__v8di) __index, __mask,
						__scale);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64gather_epi64 (__m512i __v1_old, __mmask8 __mask,
			     __m512i __index, void const *__addr,
			     int __scale)
{
  return (__m512i) __builtin_ia32_gatherdiv8di ((__v8di) __v1_old,
						__addr,
						(__v8di) __index,
						__mask, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32scatter_ps (void *__addr, __m512i __index, __m512 __v1, int __scale)
{
  __builtin_ia32_scattersiv16sf (__addr, (__mmask16) 0xFFFF,
				 (__v16si) __index, (__v16sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32scatter_ps (void *__addr, __mmask16 __mask,
			   __m512i __index, __m512 __v1, int __scale)
{
  __builtin_ia32_scattersiv16sf (__addr, __mask, (__v16si) __index,
				 (__v16sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32scatter_pd (void *__addr, __m256i __index, __m512d __v1,
		      int __scale)
{
  __builtin_ia32_scattersiv8df (__addr, (__mmask8) 0xFF,
				(__v8si) __index, (__v8df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32scatter_pd (void *__addr, __mmask8 __mask,
			   __m256i __index, __m512d __v1, int __scale)
{
  __builtin_ia32_scattersiv8df (__addr, __mask, (__v8si) __index,
				(__v8df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64scatter_ps (void *__addr, __m512i __index, __m256 __v1, int __scale)
{
  __builtin_ia32_scatterdiv16sf (__addr, (__mmask8) 0xFF,
				 (__v8di) __index, (__v8sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64scatter_ps (void *__addr, __mmask8 __mask,
			   __m512i __index, __m256 __v1, int __scale)
{
  __builtin_ia32_scatterdiv16sf (__addr, __mask, (__v8di) __index,
				 (__v8sf) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64scatter_pd (void *__addr, __m512i __index, __m512d __v1,
		      int __scale)
{
  __builtin_ia32_scatterdiv8df (__addr, (__mmask8) 0xFF,
				(__v8di) __index, (__v8df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64scatter_pd (void *__addr, __mmask8 __mask,
			   __m512i __index, __m512d __v1, int __scale)
{
  __builtin_ia32_scatterdiv8df (__addr, __mask, (__v8di) __index,
				(__v8df) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32scatter_epi32 (void *__addr, __m512i __index,
			 __m512i __v1, int __scale)
{
  __builtin_ia32_scattersiv16si (__addr, (__mmask16) 0xFFFF,
				 (__v16si) __index, (__v16si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32scatter_epi32 (void *__addr, __mmask16 __mask,
			      __m512i __index, __m512i __v1, int __scale)
{
  __builtin_ia32_scattersiv16si (__addr, __mask, (__v16si) __index,
				 (__v16si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i32scatter_epi64 (void *__addr, __m256i __index,
			 __m512i __v1, int __scale)
{
  __builtin_ia32_scattersiv8di (__addr, (__mmask8) 0xFF,
				(__v8si) __index, (__v8di) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i32scatter_epi64 (void *__addr, __mmask8 __mask,
			      __m256i __index, __m512i __v1, int __scale)
{
  __builtin_ia32_scattersiv8di (__addr, __mask, (__v8si) __index,
				(__v8di) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64scatter_epi32 (void *__addr, __m512i __index,
			 __m256i __v1, int __scale)
{
  __builtin_ia32_scatterdiv16si (__addr, (__mmask8) 0xFF,
				 (__v8di) __index, (__v8si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64scatter_epi32 (void *__addr, __mmask8 __mask,
			      __m512i __index, __m256i __v1, int __scale)
{
  __builtin_ia32_scatterdiv16si (__addr, __mask, (__v8di) __index,
				 (__v8si) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_i64scatter_epi64 (void *__addr, __m512i __index,
			 __m512i __v1, int __scale)
{
  __builtin_ia32_scatterdiv8di (__addr, (__mmask8) 0xFF,
				(__v8di) __index, (__v8di) __v1, __scale);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_i64scatter_epi64 (void *__addr, __mmask8 __mask,
			      __m512i __index, __m512i __v1, int __scale)
{
  __builtin_ia32_scatterdiv8di (__addr, __mask, (__v8di) __index,
				(__v8di) __v1, __scale);
}
#else
#define _mm512_i32gather_ps(INDEX, ADDR, SCALE)				\
  (__m512) __builtin_ia32_gathersiv16sf ((__v16sf)_mm512_undefined_ps(),\
					 (void const *) (ADDR),		\
					 (__v16si)(__m512i) (INDEX),	\
					 (__mmask16)0xFFFF,		\
					 (int) (SCALE))

#define _mm512_mask_i32gather_ps(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m512) __builtin_ia32_gathersiv16sf ((__v16sf)(__m512) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v16si)(__m512i) (INDEX),	\
					 (__mmask16) (MASK),		\
					 (int) (SCALE))

#define _mm512_i32gather_pd(INDEX, ADDR, SCALE)				\
  (__m512d) __builtin_ia32_gathersiv8df ((__v8df)_mm512_undefined_pd(),	\
					 (void const *) (ADDR),		\
					 (__v8si)(__m256i) (INDEX),	\
					 (__mmask8)0xFF, (int) (SCALE))

#define _mm512_mask_i32gather_pd(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m512d) __builtin_ia32_gathersiv8df ((__v8df)(__m512d) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v8si)(__m256i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm512_i64gather_ps(INDEX, ADDR, SCALE)				\
  (__m256) __builtin_ia32_gatherdiv16sf ((__v8sf)_mm256_undefined_ps(),	\
					 (void const *) (ADDR),		\
					 (__v8di)(__m512i) (INDEX),	\
					 (__mmask8)0xFF, (int) (SCALE))

#define _mm512_mask_i64gather_ps(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256) __builtin_ia32_gatherdiv16sf ((__v8sf)(__m256) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v8di)(__m512i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm512_i64gather_pd(INDEX, ADDR, SCALE)				\
  (__m512d) __builtin_ia32_gatherdiv8df ((__v8df)_mm512_undefined_pd(),	\
					 (void const *) (ADDR),		\
					 (__v8di)(__m512i) (INDEX),	\
					 (__mmask8)0xFF, (int) (SCALE))

#define _mm512_mask_i64gather_pd(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m512d) __builtin_ia32_gatherdiv8df ((__v8df)(__m512d) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v8di)(__m512i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm512_i32gather_epi32(INDEX, ADDR, SCALE)			\
  (__m512i) __builtin_ia32_gathersiv16si ((__v16si)_mm512_undefined_epi32 (),\
					  (void const *) (ADDR),	\
					  (__v16si)(__m512i) (INDEX),	\
					  (__mmask16)0xFFFF,		\
					  (int) (SCALE))

#define _mm512_mask_i32gather_epi32(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m512i) __builtin_ia32_gathersiv16si ((__v16si)(__m512i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v16si)(__m512i) (INDEX),	\
					  (__mmask16) (MASK),		\
					  (int) (SCALE))

#define _mm512_i32gather_epi64(INDEX, ADDR, SCALE)			\
  (__m512i) __builtin_ia32_gathersiv8di ((__v8di)_mm512_undefined_epi32 (),\
					 (void const *) (ADDR),		\
					 (__v8si)(__m256i) (INDEX),	\
					 (__mmask8)0xFF, (int) (SCALE))

#define _mm512_mask_i32gather_epi64(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m512i) __builtin_ia32_gathersiv8di ((__v8di)(__m512i) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v8si)(__m256i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm512_i64gather_epi32(INDEX, ADDR, SCALE)			   \
  (__m256i) __builtin_ia32_gatherdiv16si ((__v8si)_mm256_undefined_si256(),\
					  (void const *) (ADDR),	   \
					  (__v8di)(__m512i) (INDEX),	   \
					  (__mmask8)0xFF, (int) (SCALE))

#define _mm512_mask_i64gather_epi32(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m256i) __builtin_ia32_gatherdiv16si ((__v8si)(__m256i) (V1OLD),	\
					  (void const *) (ADDR),	\
					  (__v8di)(__m512i) (INDEX),	\
					  (__mmask8) (MASK),		\
					  (int) (SCALE))

#define _mm512_i64gather_epi64(INDEX, ADDR, SCALE)			\
  (__m512i) __builtin_ia32_gatherdiv8di ((__v8di)_mm512_undefined_epi32 (),\
					 (void const *) (ADDR),		\
					 (__v8di)(__m512i) (INDEX),	\
					 (__mmask8)0xFF, (int) (SCALE))

#define _mm512_mask_i64gather_epi64(V1OLD, MASK, INDEX, ADDR, SCALE)	\
  (__m512i) __builtin_ia32_gatherdiv8di ((__v8di)(__m512i) (V1OLD),	\
					 (void const *) (ADDR),		\
					 (__v8di)(__m512i) (INDEX),	\
					 (__mmask8) (MASK),		\
					 (int) (SCALE))

#define _mm512_i32scatter_ps(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv16sf ((void *) (ADDR), (__mmask16)0xFFFF,	\
				 (__v16si)(__m512i) (INDEX),		\
				 (__v16sf)(__m512) (V1), (int) (SCALE))

#define _mm512_mask_i32scatter_ps(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv16sf ((void *) (ADDR), (__mmask16) (MASK),	\
				 (__v16si)(__m512i) (INDEX),		\
				 (__v16sf)(__m512) (V1), (int) (SCALE))

#define _mm512_i32scatter_pd(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv8df ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8df)(__m512d) (V1), (int) (SCALE))

#define _mm512_mask_i32scatter_pd(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scattersiv8df ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8df)(__m512d) (V1), (int) (SCALE))

#define _mm512_i64scatter_ps(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv16sf ((void *) (ADDR), (__mmask8)0xFF,	\
				 (__v8di)(__m512i) (INDEX),		\
				 (__v8sf)(__m256) (V1), (int) (SCALE))

#define _mm512_mask_i64scatter_ps(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv16sf ((void *) (ADDR), (__mmask16) (MASK),	\
				 (__v8di)(__m512i) (INDEX),		\
				 (__v8sf)(__m256) (V1), (int) (SCALE))

#define _mm512_i64scatter_pd(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv8df ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v8di)(__m512i) (INDEX),		\
				(__v8df)(__m512d) (V1), (int) (SCALE))

#define _mm512_mask_i64scatter_pd(ADDR, MASK, INDEX, V1, SCALE)		\
  __builtin_ia32_scatterdiv8df ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v8di)(__m512i) (INDEX),		\
				(__v8df)(__m512d) (V1), (int) (SCALE))

#define _mm512_i32scatter_epi32(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv16si ((void *) (ADDR), (__mmask16)0xFFFF,	\
				 (__v16si)(__m512i) (INDEX),		\
				 (__v16si)(__m512i) (V1), (int) (SCALE))

#define _mm512_mask_i32scatter_epi32(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scattersiv16si ((void *) (ADDR), (__mmask16) (MASK),	\
				 (__v16si)(__m512i) (INDEX),		\
				 (__v16si)(__m512i) (V1), (int) (SCALE))

#define _mm512_i32scatter_epi64(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scattersiv8di ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8di)(__m512i) (V1), (int) (SCALE))

#define _mm512_mask_i32scatter_epi64(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scattersiv8di ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v8si)(__m256i) (INDEX),		\
				(__v8di)(__m512i) (V1), (int) (SCALE))

#define _mm512_i64scatter_epi32(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv16si ((void *) (ADDR), (__mmask8)0xFF,	\
				 (__v8di)(__m512i) (INDEX),		\
				 (__v8si)(__m256i) (V1), (int) (SCALE))

#define _mm512_mask_i64scatter_epi32(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scatterdiv16si ((void *) (ADDR), (__mmask8) (MASK),	\
				 (__v8di)(__m512i) (INDEX),		\
				 (__v8si)(__m256i) (V1), (int) (SCALE))

#define _mm512_i64scatter_epi64(ADDR, INDEX, V1, SCALE)			\
  __builtin_ia32_scatterdiv8di ((void *) (ADDR), (__mmask8)0xFF,	\
				(__v8di)(__m512i) (INDEX),		\
				(__v8di)(__m512i) (V1), (int) (SCALE))

#define _mm512_mask_i64scatter_epi64(ADDR, MASK, INDEX, V1, SCALE)	\
  __builtin_ia32_scatterdiv8di ((void *) (ADDR), (__mmask8) (MASK),	\
				(__v8di)(__m512i) (INDEX),		\
				(__v8di)(__m512i) (V1), (int) (SCALE))
#endif

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compress_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_compressdf512_mask ((__v8df) __A,
						      (__v8df) __W,
						      (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_compress_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_compressdf512_mask ((__v8df) __A,
						      (__v8df)
						      _mm512_setzero_pd (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compressstoreu_pd (void *__P, __mmask8 __U, __m512d __A)
{
  __builtin_ia32_compressstoredf512_mask ((__v8df *) __P, (__v8df) __A,
					  (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compress_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_compresssf512_mask ((__v16sf) __A,
						     (__v16sf) __W,
						     (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_compress_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_compresssf512_mask ((__v16sf) __A,
						     (__v16sf)
						     _mm512_setzero_ps (),
						     (__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compressstoreu_ps (void *__P, __mmask16 __U, __m512 __A)
{
  __builtin_ia32_compressstoresf512_mask ((__v16sf *) __P, (__v16sf) __A,
					  (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compress_epi64 (__m512i __W, __mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_compressdi512_mask ((__v8di) __A,
						      (__v8di) __W,
						      (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_compress_epi64 (__mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_compressdi512_mask ((__v8di) __A,
						      (__v8di)
						      _mm512_setzero_si512 (),
						      (__mmask8) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compressstoreu_epi64 (void *__P, __mmask8 __U, __m512i __A)
{
  __builtin_ia32_compressstoredi512_mask ((__v8di *) __P, (__v8di) __A,
					  (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compress_epi32 (__m512i __W, __mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_compresssi512_mask ((__v16si) __A,
						      (__v16si) __W,
						      (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_compress_epi32 (__mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_compresssi512_mask ((__v16si) __A,
						      (__v16si)
						      _mm512_setzero_si512 (),
						      (__mmask16) __U);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_compressstoreu_epi32 (void *__P, __mmask16 __U, __m512i __A)
{
  __builtin_ia32_compressstoresi512_mask ((__v16si *) __P, (__v16si) __A,
					  (__mmask16) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expand_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_expanddf512_mask ((__v8df) __A,
						    (__v8df) __W,
						    (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expand_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_expanddf512_maskz ((__v8df) __A,
						     (__v8df)
						     _mm512_setzero_pd (),
						     (__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expandloadu_pd (__m512d __W, __mmask8 __U, void const *__P)
{
  return (__m512d) __builtin_ia32_expandloaddf512_mask ((const __v8df *) __P,
							(__v8df) __W,
							(__mmask8) __U);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expandloadu_pd (__mmask8 __U, void const *__P)
{
  return (__m512d) __builtin_ia32_expandloaddf512_maskz ((const __v8df *) __P,
							 (__v8df)
							 _mm512_setzero_pd (),
							 (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expand_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_expandsf512_mask ((__v16sf) __A,
						   (__v16sf) __W,
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expand_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_expandsf512_maskz ((__v16sf) __A,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expandloadu_ps (__m512 __W, __mmask16 __U, void const *__P)
{
  return (__m512) __builtin_ia32_expandloadsf512_mask ((const __v16sf *) __P,
						       (__v16sf) __W,
						       (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expandloadu_ps (__mmask16 __U, void const *__P)
{
  return (__m512) __builtin_ia32_expandloadsf512_maskz ((const __v16sf *) __P,
							(__v16sf)
							_mm512_setzero_ps (),
							(__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expand_epi64 (__m512i __W, __mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_expanddi512_mask ((__v8di) __A,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expand_epi64 (__mmask8 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_expanddi512_maskz ((__v8di) __A,
						     (__v8di)
						     _mm512_setzero_si512 (),
						     (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expandloadu_epi64 (__m512i __W, __mmask8 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_expandloaddi512_mask ((const __v8di *) __P,
							(__v8di) __W,
							(__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expandloadu_epi64 (__mmask8 __U, void const *__P)
{
  return (__m512i)
	 __builtin_ia32_expandloaddi512_maskz ((const __v8di *) __P,
					       (__v8di)
					       _mm512_setzero_si512 (),
					       (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expand_epi32 (__m512i __W, __mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_expandsi512_mask ((__v16si) __A,
						    (__v16si) __W,
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expand_epi32 (__mmask16 __U, __m512i __A)
{
  return (__m512i) __builtin_ia32_expandsi512_maskz ((__v16si) __A,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_expandloadu_epi32 (__m512i __W, __mmask16 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_expandloadsi512_mask ((const __v16si *) __P,
							(__v16si) __W,
							(__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_expandloadu_epi32 (__mmask16 __U, void const *__P)
{
  return (__m512i) __builtin_ia32_expandloadsi512_maskz ((const __v16si *) __P,
							 (__v16si)
							 _mm512_setzero_si512
							 (), (__mmask16) __U);
}

/* Mask arithmetic operations */
#define _kand_mask16 _mm512_kand
#define _kandn_mask16 _mm512_kandn
#define _knot_mask16 _mm512_knot
#define _kor_mask16 _mm512_kor
#define _kxnor_mask16 _mm512_kxnor
#define _kxor_mask16 _mm512_kxor

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortest_mask16_u8  (__mmask16 __A,  __mmask16 __B, unsigned char *__CF)
{
  *__CF = (unsigned char) __builtin_ia32_kortestchi (__A, __B);
  return (unsigned char) __builtin_ia32_kortestzhi (__A, __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestz_mask16_u8 (__mmask16 __A, __mmask16 __B)
{
  return (unsigned char) __builtin_ia32_kortestzhi ((__mmask16) __A,
						    (__mmask16) __B);
}

extern __inline unsigned char
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kortestc_mask16_u8 (__mmask16 __A, __mmask16 __B)
{
  return (unsigned char) __builtin_ia32_kortestchi ((__mmask16) __A,
						    (__mmask16) __B);
}

extern __inline unsigned int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtmask16_u32 (__mmask16 __A)
{
  return (unsigned int) __builtin_ia32_kmovw ((__mmask16 ) __A);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_cvtu32_mask16 (unsigned int __A)
{
  return (__mmask16) __builtin_ia32_kmovw ((__mmask16 ) __A);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_load_mask16 (__mmask16 *__A)
{
  return (__mmask16) __builtin_ia32_kmovw (*(__mmask16 *) __A);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_store_mask16 (__mmask16 *__A, __mmask16 __B)
{
  *(__mmask16 *) __A = __builtin_ia32_kmovw (__B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kand (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kandhi ((__mmask16) __A, (__mmask16) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kandn (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kandnhi ((__mmask16) __A,
					     (__mmask16) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kor (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_korhi ((__mmask16) __A, (__mmask16) __B);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kortestz (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kortestzhi ((__mmask16) __A,
						(__mmask16) __B);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kortestc (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kortestchi ((__mmask16) __A,
						(__mmask16) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kxnor (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kxnorhi ((__mmask16) __A, (__mmask16) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kxor (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kxorhi ((__mmask16) __A, (__mmask16) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_knot (__mmask16 __A)
{
  return (__mmask16) __builtin_ia32_knothi ((__mmask16) __A);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kunpackb (__mmask16 __A, __mmask16 __B)
{
  return (__mmask16) __builtin_ia32_kunpckhi ((__mmask16) __A, (__mmask16) __B);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_kunpackb_mask16 (__mmask8 __A, __mmask8 __B)
{
  return (__mmask16) __builtin_ia32_kunpckhi ((__mmask16) __A, (__mmask16) __B);
}

#ifdef __OPTIMIZE__
extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_inserti32x4 (__mmask16 __B, __m512i __C, __m128i __D,
			  const int __imm)
{
  return (__m512i) __builtin_ia32_inserti32x4_mask ((__v16si) __C,
						    (__v4si) __D,
						    __imm,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    __B);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_insertf32x4 (__mmask16 __B, __m512 __C, __m128 __D,
			  const int __imm)
{
  return (__m512) __builtin_ia32_insertf32x4_mask ((__v16sf) __C,
						   (__v4sf) __D,
						   __imm,
						   (__v16sf)
						   _mm512_setzero_ps (), __B);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_inserti32x4 (__m512i __A, __mmask16 __B, __m512i __C,
			 __m128i __D, const int __imm)
{
  return (__m512i) __builtin_ia32_inserti32x4_mask ((__v16si) __C,
						    (__v4si) __D,
						    __imm,
						    (__v16si) __A,
						    __B);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_insertf32x4 (__m512 __A, __mmask16 __B, __m512 __C,
			 __m128 __D, const int __imm)
{
  return (__m512) __builtin_ia32_insertf32x4_mask ((__v16sf) __C,
						   (__v4sf) __D,
						   __imm,
						   (__v16sf) __A, __B);
}
#else
#define _mm512_maskz_insertf32x4(A, X, Y, C)                            \
  ((__m512) __builtin_ia32_insertf32x4_mask ((__v16sf)(__m512) (X),     \
    (__v4sf)(__m128) (Y), (int) (C), (__v16sf)_mm512_setzero_ps(),      \
    (__mmask16)(A)))

#define _mm512_maskz_inserti32x4(A, X, Y, C)                            \
  ((__m512i) __builtin_ia32_inserti32x4_mask ((__v16si)(__m512i) (X),   \
    (__v4si)(__m128i) (Y), (int) (C), (__v16si)_mm512_setzero_si512 (),     \
    (__mmask16)(A)))

#define _mm512_mask_insertf32x4(A, B, X, Y, C)                          \
  ((__m512) __builtin_ia32_insertf32x4_mask ((__v16sf)(__m512) (X),     \
    (__v4sf)(__m128) (Y), (int) (C), (__v16sf)(__m512) (A),             \
					     (__mmask16)(B)))

#define _mm512_mask_inserti32x4(A, B, X, Y, C)                          \
  ((__m512i) __builtin_ia32_inserti32x4_mask ((__v16si)(__m512i) (X),   \
    (__v4si)(__m128i) (Y), (int) (C), (__v16si)(__m512i) (A),           \
					      (__mmask16)(B)))
#endif

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epi64 (__mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epi64 (__m512i __W, __mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epi64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epi64 (__m512i __W, __mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epi64 (__mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epu64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxuq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epu64 (__mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxuq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epu64 (__m512i __W, __mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxuq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epu64 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminuq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_undefined_epi32 (),
						  (__mmask8) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epu64 (__m512i __W, __mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminuq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epu64 (__mmask8 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminuq512_mask ((__v8di) __A,
						  (__v8di) __B,
						  (__v8di)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epi32 (__mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epi32 (__m512i __W, __mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxsd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epi32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epi32 (__mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epi32 (__m512i __W, __mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminsd512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_epu32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxud512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_epu32 (__mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxud512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_epu32 (__m512i __W, __mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pmaxud512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W, __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_epu32 (__m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminud512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_undefined_epi32 (),
						  (__mmask16) -1);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_epu32 (__mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminud512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si)
						  _mm512_setzero_si512 (),
						  __M);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_epu32 (__m512i __W, __mmask16 __M, __m512i __A, __m512i __B)
{
  return (__m512i) __builtin_ia32_pminud512_mask ((__v16si) __A,
						  (__v16si) __B,
						  (__v16si) __W, __M);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_unpacklo_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_unpcklps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_unpacklo_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_unpcklps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __W,
						   (__mmask16) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_unpacklo_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_unpcklps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_maxsd_round ((__v2df) __A,
					       (__v2df) __B,
					       __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_maxsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_maxsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_max_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_maxss_round ((__v4sf) __A,
					      (__v4sf) __B,
					      __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_maxss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			   const int __R)
{
  return (__m128) __builtin_ia32_maxss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_round_sd (__m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_minsd_round ((__v2df) __A,
					       (__v2df) __B,
					       __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_round_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_minsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_round_sd (__mmask8 __U, __m128d __A, __m128d __B,
			   const int __R)
{
  return (__m128d) __builtin_ia32_minsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_min_round_ss (__m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_minss_round ((__v4sf) __A,
					      (__v4sf) __B,
					      __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_round_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_minss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_round_ss (__mmask8 __U, __m128 __A, __m128 __B,
			   const int __R)
{
  return (__m128) __builtin_ia32_minss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U, __R);
}

#else
#define _mm_max_round_sd(A, B, C)            \
    (__m128d)__builtin_ia32_maxsd_round(A, B, C)

#define _mm_mask_max_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_maxsd_mask_round(A, B, W, U, C)

#define _mm_maskz_max_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_maxsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U, C)

#define _mm_max_round_ss(A, B, C)            \
    (__m128)__builtin_ia32_maxss_round(A, B, C)

#define _mm_mask_max_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_maxss_mask_round(A, B, W, U, C)

#define _mm_maskz_max_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_maxss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U, C)

#define _mm_min_round_sd(A, B, C)            \
    (__m128d)__builtin_ia32_minsd_round(A, B, C)

#define _mm_mask_min_round_sd(W, U, A, B, C) \
    (__m128d)__builtin_ia32_minsd_mask_round(A, B, W, U, C)

#define _mm_maskz_min_round_sd(U, A, B, C)   \
    (__m128d)__builtin_ia32_minsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U, C)

#define _mm_min_round_ss(A, B, C)            \
    (__m128)__builtin_ia32_minss_round(A, B, C)

#define _mm_mask_min_round_ss(W, U, A, B, C) \
    (__m128)__builtin_ia32_minss_mask_round(A, B, W, U, C)

#define _mm_maskz_min_round_ss(U, A, B, C)   \
    (__m128)__builtin_ia32_minss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U, C)

#endif

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_blend_pd (__mmask8 __U, __m512d __A, __m512d __W)
{
  return (__m512d) __builtin_ia32_blendmpd_512_mask ((__v8df) __A,
						     (__v8df) __W,
						     (__mmask8) __U);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_blend_ps (__mmask16 __U, __m512 __A, __m512 __W)
{
  return (__m512) __builtin_ia32_blendmps_512_mask ((__v16sf) __A,
						    (__v16sf) __W,
						    (__mmask16) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_blend_epi64 (__mmask8 __U, __m512i __A, __m512i __W)
{
  return (__m512i) __builtin_ia32_blendmq_512_mask ((__v8di) __A,
						    (__v8di) __W,
						    (__mmask8) __U);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_blend_epi32 (__mmask16 __U, __m512i __A, __m512i __W)
{
  return (__m512i) __builtin_ia32_blendmd_512_mask ((__v16si) __A,
						    (__v16si) __W,
						    (__mmask16) __U);
}

#ifdef __OPTIMIZE__
extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmadd_round_sd (__m128d __W, __m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_round ((__v2df) __W,
						   (__v2df) __A,
						   (__v2df) __B,
						   __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmadd_round_ss (__m128 __W, __m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_round ((__v4sf) __W,
						  (__v4sf) __A,
						  (__v4sf) __B,
						  __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsub_round_sd (__m128d __W, __m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_round ((__v2df) __W,
						   (__v2df) __A,
						   -(__v2df) __B,
						   __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsub_round_ss (__m128 __W, __m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_round ((__v4sf) __W,
						  (__v4sf) __A,
						  -(__v4sf) __B,
						  __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmadd_round_sd (__m128d __W, __m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_round ((__v2df) __W,
						   -(__v2df) __A,
						   (__v2df) __B,
						   __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmadd_round_ss (__m128 __W, __m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_round ((__v4sf) __W,
						  -(__v4sf) __A,
						  (__v4sf) __B,
						  __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmsub_round_sd (__m128d __W, __m128d __A, __m128d __B, const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_round ((__v2df) __W,
						   -(__v2df) __A,
						   -(__v2df) __B,
						   __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmsub_round_ss (__m128 __W, __m128 __A, __m128 __B, const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_round ((__v4sf) __W,
						  -(__v4sf) __A,
						  -(__v4sf) __B,
						  __R);
}
#else
#define _mm_fmadd_round_sd(A, B, C, R)            \
    (__m128d)__builtin_ia32_vfmaddsd3_round(A, B, C, R)

#define _mm_fmadd_round_ss(A, B, C, R)            \
    (__m128)__builtin_ia32_vfmaddss3_round(A, B, C, R)

#define _mm_fmsub_round_sd(A, B, C, R)            \
    (__m128d)__builtin_ia32_vfmaddsd3_round(A, B, -(C), R)

#define _mm_fmsub_round_ss(A, B, C, R)            \
    (__m128)__builtin_ia32_vfmaddss3_round(A, B, -(C), R)

#define _mm_fnmadd_round_sd(A, B, C, R)            \
    (__m128d)__builtin_ia32_vfmaddsd3_round(A, -(B), C, R)

#define _mm_fnmadd_round_ss(A, B, C, R)            \
   (__m128)__builtin_ia32_vfmaddss3_round(A, -(B), C, R)

#define _mm_fnmsub_round_sd(A, B, C, R)            \
    (__m128d)__builtin_ia32_vfmaddsd3_round(A, -(B), -(C), R)

#define _mm_fnmsub_round_ss(A, B, C, R)            \
    (__m128)__builtin_ia32_vfmaddss3_round(A, -(B), -(C), R)
#endif

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmadd_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  (__v2df) __A,
						  (__v2df) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmadd_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 (__v4sf) __A,
						 (__v4sf) __B,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmadd_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask3 ((__v2df) __W,
						   (__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmadd_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask3 ((__v4sf) __W,
						  (__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmadd_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   (__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmadd_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  (__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsub_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  (__v2df) __A,
						  -(__v2df) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsub_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 (__v4sf) __A,
						 -(__v4sf) __B,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsub_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmsubsd3_mask3 ((__v2df) __W,
						   (__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsub_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmsubss3_mask3 ((__v4sf) __W,
						  (__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsub_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   (__v2df) __A,
						   -(__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsub_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  (__v4sf) __A,
						  -(__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmadd_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  -(__v2df) __A,
						  (__v2df) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmadd_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 -(__v4sf) __A,
						 (__v4sf) __B,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmadd_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask3 ((__v2df) __W,
						   -(__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmadd_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask3 ((__v4sf) __W,
						  -(__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmadd_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   -(__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmadd_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  -(__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmsub_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  -(__v2df) __A,
						  -(__v2df) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmsub_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 -(__v4sf) __A,
						 -(__v4sf) __B,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmsub_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U)
{
  return (__m128d) __builtin_ia32_vfmsubsd3_mask3 ((__v2df) __W,
						   -(__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmsub_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U)
{
  return (__m128) __builtin_ia32_vfmsubss3_mask3 ((__v4sf) __W,
						  -(__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmsub_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   -(__v2df) __A,
						   -(__v2df) __B,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmsub_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  -(__v4sf) __A,
						  -(__v4sf) __B,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

#ifdef __OPTIMIZE__
extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmadd_round_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			 const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  (__v2df) __A,
						  (__v2df) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmadd_round_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			 const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 (__v4sf) __A,
						 (__v4sf) __B,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmadd_round_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask3 ((__v2df) __W,
						   (__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmadd_round_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask3 ((__v4sf) __W,
						  (__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmadd_round_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   (__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmadd_round_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  (__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsub_round_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			 const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  (__v2df) __A,
						  -(__v2df) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fmsub_round_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			 const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 (__v4sf) __A,
						 -(__v4sf) __B,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsub_round_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmsubsd3_mask3 ((__v2df) __W,
						   (__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fmsub_round_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmsubss3_mask3 ((__v4sf) __W,
						  (__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsub_round_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   (__v2df) __A,
						   -(__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fmsub_round_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  (__v4sf) __A,
						  -(__v4sf) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmadd_round_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			 const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  -(__v2df) __A,
						  (__v2df) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmadd_round_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			 const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 -(__v4sf) __A,
						 (__v4sf) __B,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmadd_round_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask3 ((__v2df) __W,
						   -(__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmadd_round_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask3 ((__v4sf) __W,
						  -(__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmadd_round_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   -(__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmadd_round_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  -(__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmsub_round_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			 const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_mask ((__v2df) __W,
						  -(__v2df) __A,
						  -(__v2df) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fnmsub_round_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			 const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_mask ((__v4sf) __W,
						 -(__v4sf) __A,
						 -(__v4sf) __B,
						 (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmsub_round_sd (__m128d __W, __m128d __A, __m128d __B, __mmask8 __U,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmsubsd3_mask3 ((__v2df) __W,
						   -(__v2df) __A,
						   (__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask3_fnmsub_round_ss (__m128 __W, __m128 __A, __m128 __B, __mmask8 __U,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmsubss3_mask3 ((__v4sf) __W,
						  -(__v4sf) __A,
						  (__v4sf) __B,
						  (__mmask8) __U, __R);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmsub_round_sd (__mmask8 __U, __m128d __W, __m128d __A, __m128d __B,
			  const int __R)
{
  return (__m128d) __builtin_ia32_vfmaddsd3_maskz ((__v2df) __W,
						   -(__v2df) __A,
						   -(__v2df) __B,
						   (__mmask8) __U, __R);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fnmsub_round_ss (__mmask8 __U, __m128 __W, __m128 __A, __m128 __B,
			  const int __R)
{
  return (__m128) __builtin_ia32_vfmaddss3_maskz ((__v4sf) __W,
						  -(__v4sf) __A,
						  -(__v4sf) __B,
						  (__mmask8) __U, __R);
}
#else
#define _mm_mask_fmadd_round_sd(A, U, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_mask (A, B, C, U, R)

#define _mm_mask_fmadd_round_ss(A, U, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_mask (A, B, C, U, R)

#define _mm_mask3_fmadd_round_sd(A, B, C, U, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_mask3 (A, B, C, U, R)

#define _mm_mask3_fmadd_round_ss(A, B, C, U, R)            \
    (__m128) __builtin_ia32_vfmaddss3_mask3 (A, B, C, U, R)

#define _mm_maskz_fmadd_round_sd(U, A, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_maskz (A, B, C, U, R)

#define _mm_maskz_fmadd_round_ss(U, A, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_maskz (A, B, C, U, R)

#define _mm_mask_fmsub_round_sd(A, U, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_mask (A, B, -(C), U, R)

#define _mm_mask_fmsub_round_ss(A, U, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_mask (A, B, -(C), U, R)

#define _mm_mask3_fmsub_round_sd(A, B, C, U, R)            \
    (__m128d) __builtin_ia32_vfmsubsd3_mask3 (A, B, C, U, R)

#define _mm_mask3_fmsub_round_ss(A, B, C, U, R)            \
    (__m128) __builtin_ia32_vfmsubss3_mask3 (A, B, C, U, R)

#define _mm_maskz_fmsub_round_sd(U, A, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_maskz (A, B, -(C), U, R)

#define _mm_maskz_fmsub_round_ss(U, A, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_maskz (A, B, -(C), U, R)

#define _mm_mask_fnmadd_round_sd(A, U, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_mask (A, -(B), C, U, R)

#define _mm_mask_fnmadd_round_ss(A, U, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_mask (A, -(B), C, U, R)

#define _mm_mask3_fnmadd_round_sd(A, B, C, U, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_mask3 (A, -(B), C, U, R)

#define _mm_mask3_fnmadd_round_ss(A, B, C, U, R)            \
    (__m128) __builtin_ia32_vfmaddss3_mask3 (A, -(B), C, U, R)

#define _mm_maskz_fnmadd_round_sd(U, A, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_maskz (A, -(B), C, U, R)

#define _mm_maskz_fnmadd_round_ss(U, A, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_maskz (A, -(B), C, U, R)

#define _mm_mask_fnmsub_round_sd(A, U, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_mask (A, -(B), -(C), U, R)

#define _mm_mask_fnmsub_round_ss(A, U, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_mask (A, -(B), -(C), U, R)

#define _mm_mask3_fnmsub_round_sd(A, B, C, U, R)            \
    (__m128d) __builtin_ia32_vfmsubsd3_mask3 (A, -(B), C, U, R)

#define _mm_mask3_fnmsub_round_ss(A, B, C, U, R)            \
    (__m128) __builtin_ia32_vfmsubss3_mask3 (A, -(B), C, U, R)

#define _mm_maskz_fnmsub_round_sd(U, A, B, C, R)            \
    (__m128d) __builtin_ia32_vfmaddsd3_maskz (A, -(B), -(C), U, R)

#define _mm_maskz_fnmsub_round_ss(U, A, B, C, R)            \
    (__m128) __builtin_ia32_vfmaddss3_maskz (A, -(B), -(C), U, R)
#endif

#ifdef __OPTIMIZE__
extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_comi_round_ss (__m128 __A, __m128 __B, const int __P, const int __R)
{
  return __builtin_ia32_vcomiss ((__v4sf) __A, (__v4sf) __B, __P, __R);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_comi_round_sd (__m128d __A, __m128d __B, const int __P, const int __R)
{
  return __builtin_ia32_vcomisd ((__v2df) __A, (__v2df) __B, __P, __R);
}
#else
#define _mm_comi_round_ss(A, B, C, D)\
__builtin_ia32_vcomiss(A, B, C, D)
#define _mm_comi_round_sd(A, B, C, D)\
__builtin_ia32_vcomisd(A, B, C, D)
#endif

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sqrt_pd (__m512d __A)
{
  return (__m512d) __builtin_ia32_sqrtpd512_mask ((__v8df) __A,
						  (__v8df)
						  _mm512_undefined_pd (),
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sqrt_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_sqrtpd512_mask ((__v8df) __A,
						  (__v8df) __W,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sqrt_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_sqrtpd512_mask ((__v8df) __A,
						  (__v8df)
						  _mm512_setzero_pd (),
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sqrt_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_sqrtps512_mask ((__v16sf) __A,
						 (__v16sf)
						 _mm512_undefined_ps (),
						 (__mmask16) -1,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sqrt_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_sqrtps512_mask ((__v16sf) __A,
						 (__v16sf) __W,
						 (__mmask16) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sqrt_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_sqrtps512_mask ((__v16sf) __A,
						 (__v16sf)
						 _mm512_setzero_ps (),
						 (__mmask16) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_pd (__m512d __A, __m512d __B)
{
  return (__m512d) ((__v8df)__A + (__v8df)__B);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_addpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_addpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_add_ps (__m512 __A, __m512 __B)
{
  return (__m512) ((__v16sf)__A + (__v16sf)__B);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_add_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_addps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_add_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_addps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_addsd_mask_round ((__v2df) __A,
						(__v2df) __B,
						(__v2df) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_addsd_mask_round ((__v2df) __A,
						(__v2df) __B,
						(__v2df)
						_mm_setzero_pd (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_add_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_addss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_add_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_addss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_pd (__m512d __A, __m512d __B)
{
  return (__m512d) ((__v8df)__A - (__v8df)__B);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_subpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_subpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_sub_ps (__m512 __A, __m512 __B)
{
  return (__m512) ((__v16sf)__A - (__v16sf)__B);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_sub_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_subps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_sub_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_subps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_subsd_mask_round ((__v2df) __A,
						(__v2df) __B,
						(__v2df) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_subsd_mask_round ((__v2df) __A,
						(__v2df) __B,
						(__v2df)
						_mm_setzero_pd (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_sub_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_subss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_sub_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_subss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mul_pd (__m512d __A, __m512d __B)
{
  return (__m512d) ((__v8df)__A * (__v8df)__B);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mul_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_mulpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mul_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_mulpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mul_ps (__m512 __A, __m512 __B)
{
  return (__m512) ((__v16sf)__A * (__v16sf)__B);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_mul_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_mulps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_mul_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_mulps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B)
{
  return (__m128d) __builtin_ia32_mulsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_mulsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_mul_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B)
{
  return (__m128) __builtin_ia32_mulss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_mul_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_mulss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_div_pd (__m512d __M, __m512d __V)
{
  return (__m512d) ((__v8df)__M / (__v8df)__V);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_div_pd (__m512d __W, __mmask8 __U, __m512d __M, __m512d __V)
{
  return (__m512d) __builtin_ia32_divpd512_mask ((__v8df) __M,
						 (__v8df) __V,
						 (__v8df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_div_pd (__mmask8 __U, __m512d __M, __m512d __V)
{
  return (__m512d) __builtin_ia32_divpd512_mask ((__v8df) __M,
						 (__v8df) __V,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_div_ps (__m512 __A, __m512 __B)
{
  return (__m512) ((__v16sf)__A / (__v16sf)__B);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_div_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_divps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_div_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_divps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_div_sd (__m128d __W, __mmask8 __U, __m128d __A,
			  __m128d __B)
{
  return (__m128d) __builtin_ia32_divsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_div_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_divsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_div_ss (__m128 __W, __mmask8 __U, __m128 __A,
			  __m128 __B)
{
  return (__m128) __builtin_ia32_divss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf) __W,
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_div_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_divss_mask_round ((__v4sf) __A,
						 (__v4sf) __B,
						 (__v4sf)
						 _mm_setzero_ps (),
						 (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_maxpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_maxpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_maxpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_max_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_maxps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_max_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_maxps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_max_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_maxps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_maxsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_maxsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_max_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_maxss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_max_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_maxss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_minpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_undefined_pd (),
						 (__mmask8) -1,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_minpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_minpd512_mask ((__v8df) __A,
						 (__v8df) __B,
						 (__v8df)
						 _mm512_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_min_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_minps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_undefined_ps (),
						(__mmask16) -1,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_min_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_minps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf) __W,
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_min_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_minps512_mask ((__v16sf) __A,
						(__v16sf) __B,
						(__v16sf)
						_mm512_setzero_ps (),
						(__mmask16) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_minsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df) __W,
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_minsd_mask_round ((__v2df) __A,
						 (__v2df) __B,
						 (__v2df)
						 _mm_setzero_pd (),
						 (__mmask8) __U,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_min_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_minss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_min_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_minss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_scalef_pd (__m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_scalefpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_scalef_pd (__m512d __W, __mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_scalefpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_scalef_pd (__mmask8 __U, __m512d __A, __m512d __B)
{
  return (__m512d) __builtin_ia32_scalefpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_scalef_ps (__m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_scalefps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_scalef_ps (__m512 __W, __mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_scalefps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __W,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_scalef_ps (__mmask16 __U, __m512 __A, __m512 __B)
{
  return (__m512) __builtin_ia32_scalefps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_scalef_sd (__m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_scalefsd_mask_round ((__v2df) __A,
						    (__v2df) __B,
						    (__v2df)
						    _mm_setzero_pd (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_scalef_ss (__m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_scalefss_mask_round ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4sf)
						   _mm_setzero_ps (),
						   (__mmask8) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmadd_pd (__m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmadd_pd (__m512d __A, __mmask8 __U, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmadd_pd (__m512d __A, __m512d __B, __m512d __C, __mmask8 __U)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_mask3 ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmadd_pd (__mmask8 __U, __m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddpd512_maskz ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmadd_ps (__m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmadd_ps (__m512 __A, __mmask16 __U, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmadd_ps (__m512 __A, __m512 __B, __m512 __C, __mmask16 __U)
{
  return (__m512) __builtin_ia32_vfmaddps512_mask3 ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmadd_ps (__mmask16 __U, __m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddps512_maskz ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsub_pd (__m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsub_pd (__m512d __A, __mmask8 __U, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_mask ((__v8df) __A,
						    (__v8df) __B,
						    (__v8df) __C,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsub_pd (__m512d __A, __m512d __B, __m512d __C, __mmask8 __U)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_mask3 ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsub_pd (__mmask8 __U, __m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmsubpd512_maskz ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsub_ps (__m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmsubps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsub_ps (__m512 __A, __mmask16 __U, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmsubps512_mask ((__v16sf) __A,
						   (__v16sf) __B,
						   (__v16sf) __C,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsub_ps (__m512 __A, __m512 __B, __m512 __C, __mmask16 __U)
{
  return (__m512) __builtin_ia32_vfmsubps512_mask3 ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsub_ps (__mmask16 __U, __m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmsubps512_maskz ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmaddsub_pd (__m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       (__v8df) __C,
						       (__mmask8) -1,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmaddsub_pd (__m512d __A, __mmask8 __U, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       (__v8df) __C,
						       (__mmask8) __U,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmaddsub_pd (__m512d __A, __m512d __B, __m512d __C, __mmask8 __U)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask3 ((__v8df) __A,
							(__v8df) __B,
							(__v8df) __C,
							(__mmask8) __U,
							_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmaddsub_pd (__mmask8 __U, __m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_maskz ((__v8df) __A,
							(__v8df) __B,
							(__v8df) __C,
							(__mmask8) __U,
							_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmaddsub_ps (__m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      (__v16sf) __C,
						      (__mmask16) -1,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmaddsub_ps (__m512 __A, __mmask16 __U, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      (__v16sf) __C,
						      (__mmask16) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmaddsub_ps (__m512 __A, __m512 __B, __m512 __C, __mmask16 __U)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask3 ((__v16sf) __A,
						       (__v16sf) __B,
						       (__v16sf) __C,
						       (__mmask16) __U,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmaddsub_ps (__mmask16 __U, __m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_maskz ((__v16sf) __A,
						       (__v16sf) __B,
						       (__v16sf) __C,
						       (__mmask16) __U,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsubadd_pd (__m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       -(__v8df) __C,
						       (__mmask8) -1,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsubadd_pd (__m512d __A, __mmask8 __U, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_mask ((__v8df) __A,
						       (__v8df) __B,
						       -(__v8df) __C,
						       (__mmask8) __U,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsubadd_pd (__m512d __A, __m512d __B, __m512d __C, __mmask8 __U)
{
  return (__m512d) __builtin_ia32_vfmsubaddpd512_mask3 ((__v8df) __A,
							(__v8df) __B,
							(__v8df) __C,
							(__mmask8) __U,
							_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsubadd_pd (__mmask8 __U, __m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfmaddsubpd512_maskz ((__v8df) __A,
							(__v8df) __B,
							-(__v8df) __C,
							(__mmask8) __U,
							_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fmsubadd_ps (__m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      -(__v16sf) __C,
						      (__mmask16) -1,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fmsubadd_ps (__m512 __A, __mmask16 __U, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_mask ((__v16sf) __A,
						      (__v16sf) __B,
						      -(__v16sf) __C,
						      (__mmask16) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fmsubadd_ps (__m512 __A, __m512 __B, __m512 __C, __mmask16 __U)
{
  return (__m512) __builtin_ia32_vfmsubaddps512_mask3 ((__v16sf) __A,
						       (__v16sf) __B,
						       (__v16sf) __C,
						       (__mmask16) __U,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fmsubadd_ps (__mmask16 __U, __m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfmaddsubps512_maskz ((__v16sf) __A,
						       (__v16sf) __B,
						       -(__v16sf) __C,
						       (__mmask16) __U,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmadd_pd (__m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmadd_pd (__m512d __A, __mmask8 __U, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmadd_pd (__m512d __A, __m512d __B, __m512d __C, __mmask8 __U)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_mask3 ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmadd_pd (__mmask8 __U, __m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfnmaddpd512_maskz ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmadd_ps (__m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfnmaddps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmadd_ps (__m512 __A, __mmask16 __U, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfnmaddps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmadd_ps (__m512 __A, __m512 __B, __m512 __C, __mmask16 __U)
{
  return (__m512) __builtin_ia32_vfnmaddps512_mask3 ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmadd_ps (__mmask16 __U, __m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfnmaddps512_maskz ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmsub_pd (__m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmsub_pd (__m512d __A, __mmask8 __U, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_mask ((__v8df) __A,
						     (__v8df) __B,
						     (__v8df) __C,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmsub_pd (__m512d __A, __m512d __B, __m512d __C, __mmask8 __U)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_mask3 ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmsub_pd (__mmask8 __U, __m512d __A, __m512d __B, __m512d __C)
{
  return (__m512d) __builtin_ia32_vfnmsubpd512_maskz ((__v8df) __A,
						      (__v8df) __B,
						      (__v8df) __C,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fnmsub_ps (__m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfnmsubps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fnmsub_ps (__m512 __A, __mmask16 __U, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfnmsubps512_mask ((__v16sf) __A,
						    (__v16sf) __B,
						    (__v16sf) __C,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask3_fnmsub_ps (__m512 __A, __m512 __B, __m512 __C, __mmask16 __U)
{
  return (__m512) __builtin_ia32_vfnmsubps512_mask3 ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fnmsub_ps (__mmask16 __U, __m512 __A, __m512 __B, __m512 __C)
{
  return (__m512) __builtin_ia32_vfnmsubps512_maskz ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16sf) __C,
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttpd_epi32 (__m512d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2dq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_undefined_si256 (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttpd_epi32 (__m256i __W, __mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2dq512_mask ((__v8df) __A,
						     (__v8si) __W,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttpd_epi32 (__mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2dq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttpd_epu32 (__m512d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2udq512_mask ((__v8df) __A,
						      (__v8si)
						      _mm256_undefined_si256 (),
						      (__mmask8) -1,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttpd_epu32 (__m256i __W, __mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2udq512_mask ((__v8df) __A,
						      (__v8si) __W,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttpd_epu32 (__mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvttpd2udq512_mask ((__v8df) __A,
						      (__v8si)
						      _mm256_setzero_si256 (),
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtpd_epi32 (__m512d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2dq512_mask ((__v8df) __A,
						    (__v8si)
						    _mm256_undefined_si256 (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtpd_epi32 (__m256i __W, __mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2dq512_mask ((__v8df) __A,
						    (__v8si) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtpd_epi32 (__mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2dq512_mask ((__v8df) __A,
						    (__v8si)
						    _mm256_setzero_si256 (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtpd_epu32 (__m512d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2udq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_undefined_si256 (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtpd_epu32 (__m256i __W, __mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2udq512_mask ((__v8df) __A,
						     (__v8si) __W,
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtpd_epu32 (__mmask8 __U, __m512d __A)
{
  return (__m256i) __builtin_ia32_cvtpd2udq512_mask ((__v8df) __A,
						     (__v8si)
						     _mm256_setzero_si256 (),
						     (__mmask8) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttps_epi32 (__m512 __A)
{
  return (__m512i) __builtin_ia32_cvttps2dq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_undefined_epi32 (),
						     (__mmask16) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttps_epi32 (__m512i __W, __mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvttps2dq512_mask ((__v16sf) __A,
						     (__v16si) __W,
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttps_epi32 (__mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvttps2dq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvttps_epu32 (__m512 __A)
{
  return (__m512i) __builtin_ia32_cvttps2udq512_mask ((__v16sf) __A,
						      (__v16si)
						      _mm512_undefined_epi32 (),
						      (__mmask16) -1,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvttps_epu32 (__m512i __W, __mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvttps2udq512_mask ((__v16sf) __A,
						      (__v16si) __W,
						      (__mmask16) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvttps_epu32 (__mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvttps2udq512_mask ((__v16sf) __A,
						      (__v16si)
						      _mm512_setzero_si512 (),
						      (__mmask16) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtps_epi32 (__m512 __A)
{
  return (__m512i) __builtin_ia32_cvtps2dq512_mask ((__v16sf) __A,
						    (__v16si)
						    _mm512_undefined_epi32 (),
						    (__mmask16) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtps_epi32 (__m512i __W, __mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvtps2dq512_mask ((__v16sf) __A,
						    (__v16si) __W,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtps_epi32 (__mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvtps2dq512_mask ((__v16sf) __A,
						    (__v16si)
						    _mm512_setzero_si512 (),
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtps_epu32 (__m512 __A)
{
  return (__m512i) __builtin_ia32_cvtps2udq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_undefined_epi32 (),
						     (__mmask16) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtps_epu32 (__m512i __W, __mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvtps2udq512_mask ((__v16sf) __A,
						     (__v16si) __W,
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtps_epu32 (__mmask16 __U, __m512 __A)
{
  return (__m512i) __builtin_ia32_cvtps2udq512_mask ((__v16sf) __A,
						     (__v16si)
						     _mm512_setzero_si512 (),
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtsd_f64 (__m512d __A)
{
  return __A[0];
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtss_f32 (__m512 __A)
{
  return __A[0];
}

#ifdef __x86_64__
extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtu64_ss (__m128 __A, unsigned long long __B)
{
  return (__m128) __builtin_ia32_cvtusi2ss64 ((__v4sf) __A, __B,
					      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtu64_sd (__m128d __A, unsigned long long __B)
{
  return (__m128d) __builtin_ia32_cvtusi2sd64 ((__v2df) __A, __B,
					       _MM_FROUND_CUR_DIRECTION);
}
#endif

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtu32_ss (__m128 __A, unsigned __B)
{
  return (__m128) __builtin_ia32_cvtusi2ss32 ((__v4sf) __A, __B,
					      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepi32_ps (__m512i __A)
{
  return (__m512) __builtin_ia32_cvtdq2ps512_mask ((__v16si) __A,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepi32_ps (__m512 __W, __mmask16 __U, __m512i __A)
{
  return (__m512) __builtin_ia32_cvtdq2ps512_mask ((__v16si) __A,
						   (__v16sf) __W,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepi32_ps (__mmask16 __U, __m512i __A)
{
  return (__m512) __builtin_ia32_cvtdq2ps512_mask ((__v16si) __A,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtepu32_ps (__m512i __A)
{
  return (__m512) __builtin_ia32_cvtudq2ps512_mask ((__v16si) __A,
						    (__v16sf)
						    _mm512_undefined_ps (),
						    (__mmask16) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtepu32_ps (__m512 __W, __mmask16 __U, __m512i __A)
{
  return (__m512) __builtin_ia32_cvtudq2ps512_mask ((__v16si) __A,
						    (__v16sf) __W,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtepu32_ps (__mmask16 __U, __m512i __A)
{
  return (__m512) __builtin_ia32_cvtudq2ps512_mask ((__v16si) __A,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

#ifdef __OPTIMIZE__
extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fixupimm_pd (__m512d __A, __m512d __B, __m512i __C, const int __imm)
{
  return (__m512d) __builtin_ia32_fixupimmpd512_mask ((__v8df) __A,
						      (__v8df) __B,
						      (__v8di) __C,
						      __imm,
						      (__mmask8) -1,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fixupimm_pd (__m512d __A, __mmask8 __U, __m512d __B,
			 __m512i __C, const int __imm)
{
  return (__m512d) __builtin_ia32_fixupimmpd512_mask ((__v8df) __A,
						      (__v8df) __B,
						      (__v8di) __C,
						      __imm,
						      (__mmask8) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fixupimm_pd (__mmask8 __U, __m512d __A, __m512d __B,
			  __m512i __C, const int __imm)
{
  return (__m512d) __builtin_ia32_fixupimmpd512_maskz ((__v8df) __A,
						       (__v8df) __B,
						       (__v8di) __C,
						       __imm,
						       (__mmask8) __U,
						       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_fixupimm_ps (__m512 __A, __m512 __B, __m512i __C, const int __imm)
{
  return (__m512) __builtin_ia32_fixupimmps512_mask ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16si) __C,
						     __imm,
						     (__mmask16) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_fixupimm_ps (__m512 __A, __mmask16 __U, __m512 __B,
			 __m512i __C, const int __imm)
{
  return (__m512) __builtin_ia32_fixupimmps512_mask ((__v16sf) __A,
						     (__v16sf) __B,
						     (__v16si) __C,
						     __imm,
						     (__mmask16) __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_fixupimm_ps (__mmask16 __U, __m512 __A, __m512 __B,
			  __m512i __C, const int __imm)
{
  return (__m512) __builtin_ia32_fixupimmps512_maskz ((__v16sf) __A,
						      (__v16sf) __B,
						      (__v16si) __C,
						      __imm,
						      (__mmask16) __U,
						      _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fixupimm_sd (__m128d __A, __m128d __B, __m128i __C, const int __imm)
{
  return (__m128d) __builtin_ia32_fixupimmsd_mask ((__v2df) __A,
						   (__v2df) __B,
						   (__v2di) __C, __imm,
						   (__mmask8) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fixupimm_sd (__m128d __A, __mmask8 __U, __m128d __B,
		      __m128i __C, const int __imm)
{
  return (__m128d) __builtin_ia32_fixupimmsd_mask ((__v2df) __A,
						   (__v2df) __B,
						   (__v2di) __C, __imm,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fixupimm_sd (__mmask8 __U, __m128d __A, __m128d __B,
		       __m128i __C, const int __imm)
{
  return (__m128d) __builtin_ia32_fixupimmsd_maskz ((__v2df) __A,
						    (__v2df) __B,
						    (__v2di) __C,
						    __imm,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_fixupimm_ss (__m128 __A, __m128 __B, __m128i __C, const int __imm)
{
  return (__m128) __builtin_ia32_fixupimmss_mask ((__v4sf) __A,
						  (__v4sf) __B,
						  (__v4si) __C, __imm,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_fixupimm_ss (__m128 __A, __mmask8 __U, __m128 __B,
		      __m128i __C, const int __imm)
{
  return (__m128) __builtin_ia32_fixupimmss_mask ((__v4sf) __A,
						  (__v4sf) __B,
						  (__v4si) __C, __imm,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_fixupimm_ss (__mmask8 __U, __m128 __A, __m128 __B,
		       __m128i __C, const int __imm)
{
  return (__m128) __builtin_ia32_fixupimmss_maskz ((__v4sf) __A,
						   (__v4sf) __B,
						   (__v4si) __C, __imm,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}
#else
#define _mm512_fixupimm_pd(X, Y, Z, C)					\
  ((__m512d)__builtin_ia32_fixupimmpd512_mask ((__v8df)(__m512d)(X),	\
      (__v8df)(__m512d)(Y), (__v8di)(__m512i)(Z), (int)(C),		\
      (__mmask8)(-1), _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_fixupimm_pd(X, U, Y, Z, C)                          \
  ((__m512d)__builtin_ia32_fixupimmpd512_mask ((__v8df)(__m512d)(X),    \
      (__v8df)(__m512d)(Y), (__v8di)(__m512i)(Z), (int)(C),             \
      (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_fixupimm_pd(U, X, Y, Z, C)                         \
  ((__m512d)__builtin_ia32_fixupimmpd512_maskz ((__v8df)(__m512d)(X),   \
      (__v8df)(__m512d)(Y), (__v8di)(__m512i)(Z), (int)(C),             \
      (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_fixupimm_ps(X, Y, Z, C)					\
  ((__m512)__builtin_ia32_fixupimmps512_mask ((__v16sf)(__m512)(X),	\
    (__v16sf)(__m512)(Y), (__v16si)(__m512i)(Z), (int)(C),		\
    (__mmask16)(-1), _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_fixupimm_ps(X, U, Y, Z, C)                          \
  ((__m512)__builtin_ia32_fixupimmps512_mask ((__v16sf)(__m512)(X),     \
    (__v16sf)(__m512)(Y), (__v16si)(__m512i)(Z), (int)(C),              \
    (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_fixupimm_ps(U, X, Y, Z, C)                         \
  ((__m512)__builtin_ia32_fixupimmps512_maskz ((__v16sf)(__m512)(X),    \
    (__v16sf)(__m512)(Y), (__v16si)(__m512i)(Z), (int)(C),              \
    (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_fixupimm_sd(X, Y, Z, C)					\
    ((__m128d)__builtin_ia32_fixupimmsd_mask ((__v2df)(__m128d)(X),	\
      (__v2df)(__m128d)(Y), (__v2di)(__m128i)(Z), (int)(C),		\
      (__mmask8)(-1), _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_fixupimm_sd(X, U, Y, Z, C)				\
    ((__m128d)__builtin_ia32_fixupimmsd_mask ((__v2df)(__m128d)(X),	\
      (__v2df)(__m128d)(Y), (__v2di)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_fixupimm_sd(U, X, Y, Z, C)				\
    ((__m128d)__builtin_ia32_fixupimmsd_maskz ((__v2df)(__m128d)(X),	\
      (__v2df)(__m128d)(Y), (__v2di)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_fixupimm_ss(X, Y, Z, C)					\
    ((__m128)__builtin_ia32_fixupimmss_mask ((__v4sf)(__m128)(X),	\
      (__v4sf)(__m128)(Y), (__v4si)(__m128i)(Z), (int)(C),		\
      (__mmask8)(-1), _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_fixupimm_ss(X, U, Y, Z, C)				\
    ((__m128)__builtin_ia32_fixupimmss_mask ((__v4sf)(__m128)(X),	\
      (__v4sf)(__m128)(Y), (__v4si)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_fixupimm_ss(U, X, Y, Z, C)				\
    ((__m128)__builtin_ia32_fixupimmss_maskz ((__v4sf)(__m128)(X),	\
      (__v4sf)(__m128)(Y), (__v4si)(__m128i)(Z), (int)(C),		\
      (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))
#endif

#ifdef __x86_64__
extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtss_u64 (__m128 __A)
{
  return (unsigned long long) __builtin_ia32_vcvtss2usi64 ((__v4sf)
							   __A,
							   _MM_FROUND_CUR_DIRECTION);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttss_u64 (__m128 __A)
{
  return (unsigned long long) __builtin_ia32_vcvttss2usi64 ((__v4sf)
							    __A,
							    _MM_FROUND_CUR_DIRECTION);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttss_i64 (__m128 __A)
{
  return (long long) __builtin_ia32_vcvttss2si64 ((__v4sf) __A,
						  _MM_FROUND_CUR_DIRECTION);
}
#endif /* __x86_64__ */

extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtss_u32 (__m128 __A)
{
  return (unsigned) __builtin_ia32_vcvtss2usi32 ((__v4sf) __A,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttss_u32 (__m128 __A)
{
  return (unsigned) __builtin_ia32_vcvttss2usi32 ((__v4sf) __A,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttss_i32 (__m128 __A)
{
  return (int) __builtin_ia32_vcvttss2si32 ((__v4sf) __A,
					    _MM_FROUND_CUR_DIRECTION);
}

#ifdef __x86_64__
extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsd_u64 (__m128d __A)
{
  return (unsigned long long) __builtin_ia32_vcvtsd2usi64 ((__v2df)
							   __A,
							   _MM_FROUND_CUR_DIRECTION);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttsd_u64 (__m128d __A)
{
  return (unsigned long long) __builtin_ia32_vcvttsd2usi64 ((__v2df)
							    __A,
							    _MM_FROUND_CUR_DIRECTION);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttsd_i64 (__m128d __A)
{
  return (long long) __builtin_ia32_vcvttsd2si64 ((__v2df) __A,
						  _MM_FROUND_CUR_DIRECTION);
}
#endif /* __x86_64__ */

extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvtsd_u32 (__m128d __A)
{
  return (unsigned) __builtin_ia32_vcvtsd2usi32 ((__v2df) __A,
						 _MM_FROUND_CUR_DIRECTION);
}

extern __inline unsigned
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttsd_u32 (__m128d __A)
{
  return (unsigned) __builtin_ia32_vcvttsd2usi32 ((__v2df) __A,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cvttsd_i32 (__m128d __A)
{
  return (int) __builtin_ia32_vcvttsd2si32 ((__v2df) __A,
					    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtps_pd (__m256 __A)
{
  return (__m512d) __builtin_ia32_cvtps2pd512_mask ((__v8sf) __A,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtps_pd (__m512d __W, __mmask8 __U, __m256 __A)
{
  return (__m512d) __builtin_ia32_cvtps2pd512_mask ((__v8sf) __A,
						    (__v8df) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtps_pd (__mmask8 __U, __m256 __A)
{
  return (__m512d) __builtin_ia32_cvtps2pd512_mask ((__v8sf) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtph_ps (__m256i __A)
{
  return (__m512) __builtin_ia32_vcvtph2ps512_mask ((__v16hi) __A,
						    (__v16sf)
						    _mm512_undefined_ps (),
						    (__mmask16) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtph_ps (__m512 __W, __mmask16 __U, __m256i __A)
{
  return (__m512) __builtin_ia32_vcvtph2ps512_mask ((__v16hi) __A,
						    (__v16sf) __W,
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtph_ps (__mmask16 __U, __m256i __A)
{
  return (__m512) __builtin_ia32_vcvtph2ps512_mask ((__v16hi) __A,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    (__mmask16) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cvtpd_ps (__m512d __A)
{
  return (__m256) __builtin_ia32_cvtpd2ps512_mask ((__v8df) __A,
						   (__v8sf)
						   _mm256_undefined_ps (),
						   (__mmask8) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cvtpd_ps (__m256 __W, __mmask8 __U, __m512d __A)
{
  return (__m256) __builtin_ia32_cvtpd2ps512_mask ((__v8df) __A,
						   (__v8sf) __W,
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_cvtpd_ps (__mmask8 __U, __m512d __A)
{
  return (__m256) __builtin_ia32_cvtpd2ps512_mask ((__v8df) __A,
						   (__v8sf)
						   _mm256_setzero_ps (),
						   (__mmask8) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

#ifdef __OPTIMIZE__
extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getexp_ps (__m512 __A)
{
  return (__m512) __builtin_ia32_getexpps512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_undefined_ps (),
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getexp_ps (__m512 __W, __mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_getexpps512_mask ((__v16sf) __A,
						   (__v16sf) __W,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getexp_ps (__mmask16 __U, __m512 __A)
{
  return (__m512) __builtin_ia32_getexpps512_mask ((__v16sf) __A,
						   (__v16sf)
						   _mm512_setzero_ps (),
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getexp_pd (__m512d __A)
{
  return (__m512d) __builtin_ia32_getexppd512_mask ((__v8df) __A,
						    (__v8df)
						    _mm512_undefined_pd (),
						    (__mmask8) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getexp_pd (__m512d __W, __mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_getexppd512_mask ((__v8df) __A,
						    (__v8df) __W,
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getexp_pd (__mmask8 __U, __m512d __A)
{
  return (__m512d) __builtin_ia32_getexppd512_mask ((__v8df) __A,
						    (__v8df)
						    _mm512_setzero_pd (),
						    (__mmask8) __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getexp_ss (__m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_getexpss128_round ((__v4sf) __A,
						    (__v4sf) __B,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getexp_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_getexpss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getexp_ss (__mmask8 __U, __m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_getexpss_mask_round ((__v4sf) __A,
						(__v4sf) __B,
						(__v4sf)
						_mm_setzero_ps (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getexp_sd (__m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_getexpsd128_round ((__v2df) __A,
						     (__v2df) __B,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getexp_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_getexpsd_mask_round ((__v2df) __A,
						(__v2df) __B,
						(__v2df) __W,
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getexp_sd (__mmask8 __U, __m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_getexpsd_mask_round ((__v2df) __A,
						(__v2df) __B,
						(__v2df)
						_mm_setzero_pd (),
						(__mmask8) __U,
						_MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getmant_pd (__m512d __A, _MM_MANTISSA_NORM_ENUM __B,
		   _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m512d) __builtin_ia32_getmantpd512_mask ((__v8df) __A,
						     (__C << 2) | __B,
						     _mm512_undefined_pd (),
						     (__mmask8) -1,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getmant_pd (__m512d __W, __mmask8 __U, __m512d __A,
			_MM_MANTISSA_NORM_ENUM __B, _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m512d) __builtin_ia32_getmantpd512_mask ((__v8df) __A,
						     (__C << 2) | __B,
						     (__v8df) __W, __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getmant_pd (__mmask8 __U, __m512d __A,
			 _MM_MANTISSA_NORM_ENUM __B, _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m512d) __builtin_ia32_getmantpd512_mask ((__v8df) __A,
						     (__C << 2) | __B,
						     (__v8df)
						     _mm512_setzero_pd (),
						     __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_getmant_ps (__m512 __A, _MM_MANTISSA_NORM_ENUM __B,
		   _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m512) __builtin_ia32_getmantps512_mask ((__v16sf) __A,
						    (__C << 2) | __B,
						    _mm512_undefined_ps (),
						    (__mmask16) -1,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_getmant_ps (__m512 __W, __mmask16 __U, __m512 __A,
			_MM_MANTISSA_NORM_ENUM __B, _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m512) __builtin_ia32_getmantps512_mask ((__v16sf) __A,
						    (__C << 2) | __B,
						    (__v16sf) __W, __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_getmant_ps (__mmask16 __U, __m512 __A,
			 _MM_MANTISSA_NORM_ENUM __B, _MM_MANTISSA_SIGN_ENUM __C)
{
  return (__m512) __builtin_ia32_getmantps512_mask ((__v16sf) __A,
						    (__C << 2) | __B,
						    (__v16sf)
						    _mm512_setzero_ps (),
						    __U,
						    _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getmant_sd (__m128d __A, __m128d __B, _MM_MANTISSA_NORM_ENUM __C,
		_MM_MANTISSA_SIGN_ENUM __D)
{
  return (__m128d) __builtin_ia32_getmantsd_round ((__v2df) __A,
						   (__v2df) __B,
						   (__D << 2) | __C,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getmant_sd (__m128d __W, __mmask8 __U, __m128d __A, __m128d __B,
			_MM_MANTISSA_NORM_ENUM __C, _MM_MANTISSA_SIGN_ENUM __D)
{
  return (__m128d) __builtin_ia32_getmantsd_mask_round ((__v2df) __A,
							(__v2df) __B,
						        (__D << 2) | __C,
                                                        (__v2df) __W,
						       __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getmant_sd (__mmask8 __U, __m128d __A, __m128d __B,
			 _MM_MANTISSA_NORM_ENUM __C, _MM_MANTISSA_SIGN_ENUM __D)
{
  return (__m128d) __builtin_ia32_getmantsd_mask_round ((__v2df) __A,
                                                        (__v2df) __B,
						        (__D << 2) | __C,
                                                        (__v2df)
							_mm_setzero_pd(),
						        __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_getmant_ss (__m128 __A, __m128 __B, _MM_MANTISSA_NORM_ENUM __C,
		_MM_MANTISSA_SIGN_ENUM __D)
{
  return (__m128) __builtin_ia32_getmantss_round ((__v4sf) __A,
						  (__v4sf) __B,
						  (__D << 2) | __C,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_getmant_ss (__m128 __W, __mmask8 __U, __m128 __A, __m128 __B,
			_MM_MANTISSA_NORM_ENUM __C, _MM_MANTISSA_SIGN_ENUM __D)
{
  return (__m128) __builtin_ia32_getmantss_mask_round ((__v4sf) __A,
							(__v4sf) __B,
						        (__D << 2) | __C,
                                                        (__v4sf) __W,
						       __U,
						     _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_getmant_ss (__mmask8 __U, __m128 __A, __m128 __B,
			 _MM_MANTISSA_NORM_ENUM __C, _MM_MANTISSA_SIGN_ENUM __D)
{
  return (__m128) __builtin_ia32_getmantss_mask_round ((__v4sf) __A,
                                                        (__v4sf) __B,
						        (__D << 2) | __C,
                                                        (__v4sf)
							_mm_setzero_ps(),
						        __U,
						     _MM_FROUND_CUR_DIRECTION);
}

#else
#define _mm512_getmant_pd(X, B, C)                                                  \
  ((__m512d)__builtin_ia32_getmantpd512_mask ((__v8df)(__m512d)(X),                 \
                                              (int)(((C)<<2) | (B)),                \
                                              (__v8df)_mm512_undefined_pd(),        \
                                              (__mmask8)-1,\
					      _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_getmant_pd(W, U, X, B, C)                                       \
  ((__m512d)__builtin_ia32_getmantpd512_mask ((__v8df)(__m512d)(X),                 \
                                              (int)(((C)<<2) | (B)),                \
                                              (__v8df)(__m512d)(W),                 \
                                              (__mmask8)(U),\
					      _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_getmant_pd(U, X, B, C)                                         \
  ((__m512d)__builtin_ia32_getmantpd512_mask ((__v8df)(__m512d)(X),                 \
                                              (int)(((C)<<2) | (B)),                \
                                              (__v8df)_mm512_setzero_pd(),          \
                                              (__mmask8)(U),\
					      _MM_FROUND_CUR_DIRECTION))
#define _mm512_getmant_ps(X, B, C)                                                  \
  ((__m512)__builtin_ia32_getmantps512_mask ((__v16sf)(__m512)(X),                  \
                                             (int)(((C)<<2) | (B)),                 \
                                             (__v16sf)_mm512_undefined_ps(),        \
                                             (__mmask16)-1,\
					     _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_getmant_ps(W, U, X, B, C)                                       \
  ((__m512)__builtin_ia32_getmantps512_mask ((__v16sf)(__m512)(X),                  \
                                             (int)(((C)<<2) | (B)),                 \
                                             (__v16sf)(__m512)(W),                  \
                                             (__mmask16)(U),\
					     _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_getmant_ps(U, X, B, C)                                         \
  ((__m512)__builtin_ia32_getmantps512_mask ((__v16sf)(__m512)(X),                  \
                                             (int)(((C)<<2) | (B)),                 \
                                             (__v16sf)_mm512_setzero_ps(),          \
                                             (__mmask16)(U),\
					     _MM_FROUND_CUR_DIRECTION))
#define _mm_getmant_sd(X, Y, C, D)                                                  \
  ((__m128d)__builtin_ia32_getmantsd_round ((__v2df)(__m128d)(X),                    \
                                           (__v2df)(__m128d)(Y),                    \
                                           (int)(((D)<<2) | (C)),                   \
					   _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_getmant_sd(W, U, X, Y, C, D)                                       \
  ((__m128d)__builtin_ia32_getmantsd_mask_round ((__v2df)(__m128d)(X),                 \
                                                 (__v2df)(__m128d)(Y),                 \
                                                 (int)(((D)<<2) | (C)),                \
                                                (__v2df)(__m128d)(W),                 \
                                              (__mmask8)(U),\
					      _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_getmant_sd(U, X, Y, C, D)                                         \
  ((__m128d)__builtin_ia32_getmantsd_mask_round ((__v2df)(__m128d)(X),                 \
                                           (__v2df)(__m128d)(Y),                     \
                                              (int)(((D)<<2) | (C)),                \
                                           (__v2df)_mm_setzero_pd(),             \
                                              (__mmask8)(U),\
					      _MM_FROUND_CUR_DIRECTION))

#define _mm_getmant_ss(X, Y, C, D)                                                  \
  ((__m128)__builtin_ia32_getmantss_round ((__v4sf)(__m128)(X),                      \
                                          (__v4sf)(__m128)(Y),                      \
                                          (int)(((D)<<2) | (C)),                    \
					  _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_getmant_ss(W, U, X, Y, C, D)                                       \
  ((__m128)__builtin_ia32_getmantss_mask_round ((__v4sf)(__m128)(X),                 \
                                                 (__v4sf)(__m128)(Y),                 \
                                                 (int)(((D)<<2) | (C)),                \
                                                (__v4sf)(__m128)(W),                 \
                                              (__mmask8)(U),\
					      _MM_FROUND_CUR_DIRECTION))

#define _mm_maskz_getmant_ss(U, X, Y, C, D)                                         \
  ((__m128)__builtin_ia32_getmantss_mask_round ((__v4sf)(__m128)(X),                 \
                                           (__v4sf)(__m128)(Y),                     \
                                              (int)(((D)<<2) | (C)),                \
                                           (__v4sf)_mm_setzero_ps(),             \
                                              (__mmask8)(U),\
					      _MM_FROUND_CUR_DIRECTION))

#define _mm_getexp_ss(A, B)						      \
  ((__m128)__builtin_ia32_getexpss128_round((__v4sf)(__m128)(A), (__v4sf)(__m128)(B),  \
					   _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_getexp_ss(W, U, A, B) \
    (__m128)__builtin_ia32_getexpss_mask_round(A, B, W, U,\
                                             _MM_FROUND_CUR_DIRECTION)

#define _mm_maskz_getexp_ss(U, A, B)   \
    (__m128)__builtin_ia32_getexpss_mask_round(A, B, (__v4sf)_mm_setzero_ps(), U,\
					      _MM_FROUND_CUR_DIRECTION)

#define _mm_getexp_sd(A, B)						       \
  ((__m128d)__builtin_ia32_getexpsd128_round((__v2df)(__m128d)(A), (__v2df)(__m128d)(B),\
					    _MM_FROUND_CUR_DIRECTION))

#define _mm_mask_getexp_sd(W, U, A, B) \
    (__m128d)__builtin_ia32_getexpsd_mask_round(A, B, W, U,\
                                             _MM_FROUND_CUR_DIRECTION)

#define _mm_maskz_getexp_sd(U, A, B)   \
    (__m128d)__builtin_ia32_getexpsd_mask_round(A, B, (__v2df)_mm_setzero_pd(), U,\
					      _MM_FROUND_CUR_DIRECTION)

#define _mm512_getexp_ps(A)						\
  ((__m512)__builtin_ia32_getexpps512_mask((__v16sf)(__m512)(A),		\
  (__v16sf)_mm512_undefined_ps(), (__mmask16)-1, _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_getexp_ps(W, U, A)					\
  ((__m512)__builtin_ia32_getexpps512_mask((__v16sf)(__m512)(A),		\
  (__v16sf)(__m512)(W), (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_getexp_ps(U, A)					\
  ((__m512)__builtin_ia32_getexpps512_mask((__v16sf)(__m512)(A),		\
  (__v16sf)_mm512_setzero_ps(), (__mmask16)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_getexp_pd(A)						\
  ((__m512d)__builtin_ia32_getexppd512_mask((__v8df)(__m512d)(A),		\
  (__v8df)_mm512_undefined_pd(), (__mmask8)-1, _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_getexp_pd(W, U, A)					\
  ((__m512d)__builtin_ia32_getexppd512_mask((__v8df)(__m512d)(A),		\
  (__v8df)(__m512d)(W), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))

#define _mm512_maskz_getexp_pd(U, A)					\
  ((__m512d)__builtin_ia32_getexppd512_mask((__v8df)(__m512d)(A),		\
  (__v8df)_mm512_setzero_pd(), (__mmask8)(U), _MM_FROUND_CUR_DIRECTION))
#endif

#ifdef __OPTIMIZE__
extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_roundscale_ps (__m512 __A, const int __imm)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __A, __imm,
						  (__v16sf)
						  _mm512_undefined_ps (),
						  -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_roundscale_ps (__m512 __A, __mmask16 __B, __m512 __C,
			   const int __imm)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __C, __imm,
						  (__v16sf) __A,
						  (__mmask16) __B,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_roundscale_ps (__mmask16 __A, __m512 __B, const int __imm)
{
  return (__m512) __builtin_ia32_rndscaleps_mask ((__v16sf) __B,
						  __imm,
						  (__v16sf)
						  _mm512_setzero_ps (),
						  (__mmask16) __A,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_roundscale_pd (__m512d __A, const int __imm)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __A, __imm,
						   (__v8df)
						   _mm512_undefined_pd (),
						   -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_roundscale_pd (__m512d __A, __mmask8 __B, __m512d __C,
			   const int __imm)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __C, __imm,
						   (__v8df) __A,
						   (__mmask8) __B,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_maskz_roundscale_pd (__mmask8 __A, __m512d __B, const int __imm)
{
  return (__m512d) __builtin_ia32_rndscalepd_mask ((__v8df) __B,
						   __imm,
						   (__v8df)
						   _mm512_setzero_pd (),
						   (__mmask8) __A,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_roundscale_ss (__m128 __A, __m128 __B, const int __imm)
{
  return (__m128)
    __builtin_ia32_rndscaless_mask_round ((__v4sf) __A,
					  (__v4sf) __B, __imm,
					  (__v4sf)
					  _mm_setzero_ps (),
					  (__mmask8) -1,
					  _MM_FROUND_CUR_DIRECTION);
}


extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_roundscale_ss (__m128 __A, __mmask8 __B, __m128 __C, __m128 __D,
			const int __imm)
{
  return (__m128)
    __builtin_ia32_rndscaless_mask_round ((__v4sf) __C,
					  (__v4sf) __D, __imm,
					  (__v4sf) __A,
					  (__mmask8) __B,
					  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_roundscale_ss (__mmask8 __A, __m128 __B, __m128 __C,
			 const int __imm)
{
  return (__m128)
    __builtin_ia32_rndscaless_mask_round ((__v4sf) __B,
					  (__v4sf) __C, __imm,
					  (__v4sf)
					  _mm_setzero_ps (),
					  (__mmask8) __A,
					  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_roundscale_sd (__m128d __A, __m128d __B, const int __imm)
{
  return (__m128d)
    __builtin_ia32_rndscalesd_mask_round ((__v2df) __A,
					  (__v2df) __B, __imm,
					  (__v2df)
					  _mm_setzero_pd (),
					  (__mmask8) -1,
					  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_roundscale_sd (__m128d __A, __mmask8 __B, __m128d __C, __m128d __D,
			const int __imm)
{
  return (__m128d)
    __builtin_ia32_rndscalesd_mask_round ((__v2df) __C,
					  (__v2df) __D, __imm,
					  (__v2df) __A,
					  (__mmask8) __B,
					  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskz_roundscale_sd (__mmask8 __A, __m128d __B, __m128d __C,
			 const int __imm)
{
  return (__m128d)
    __builtin_ia32_rndscalesd_mask_round ((__v2df) __B,
					  (__v2df) __C, __imm,
					  (__v2df)
					  _mm_setzero_pd (),
					  (__mmask8) __A,
					  _MM_FROUND_CUR_DIRECTION);
}

#else
#define _mm512_roundscale_ps(A, B) \
  ((__m512) __builtin_ia32_rndscaleps_mask ((__v16sf)(__m512)(A), (int)(B),\
    (__v16sf)_mm512_undefined_ps(), (__mmask16)(-1), _MM_FROUND_CUR_DIRECTION))
#define _mm512_mask_roundscale_ps(A, B, C, D)				\
  ((__m512) __builtin_ia32_rndscaleps_mask ((__v16sf)(__m512)(C),	\
					    (int)(D),			\
					    (__v16sf)(__m512)(A),	\
					    (__mmask16)(B), _MM_FROUND_CUR_DIRECTION))
#define _mm512_maskz_roundscale_ps(A, B, C)				\
  ((__m512) __builtin_ia32_rndscaleps_mask ((__v16sf)(__m512)(B),	\
					    (int)(C),			\
					    (__v16sf)_mm512_setzero_ps(),\
					    (__mmask16)(A), _MM_FROUND_CUR_DIRECTION))
#define _mm512_roundscale_pd(A, B) \
  ((__m512d) __builtin_ia32_rndscalepd_mask ((__v8df)(__m512d)(A), (int)(B),\
    (__v8df)_mm512_undefined_pd(), (__mmask8)(-1), _MM_FROUND_CUR_DIRECTION))
#define _mm512_mask_roundscale_pd(A, B, C, D)				\
  ((__m512d) __builtin_ia32_rndscalepd_mask ((__v8df)(__m512d)(C),	\
					     (int)(D),			\
					     (__v8df)(__m512d)(A),	\
					     (__mmask8)(B), _MM_FROUND_CUR_DIRECTION))
#define _mm512_maskz_roundscale_pd(A, B, C)				\
  ((__m512d) __builtin_ia32_rndscalepd_mask ((__v8df)(__m512d)(B),	\
					     (int)(C),			\
					     (__v8df)_mm512_setzero_pd(),\
					     (__mmask8)(A), _MM_FROUND_CUR_DIRECTION))
#define _mm_roundscale_ss(A, B, I)					\
  ((__m128)								\
   __builtin_ia32_rndscaless_mask_round ((__v4sf) (__m128) (A),		\
					 (__v4sf) (__m128) (B),		\
					 (int) (I),			\
					 (__v4sf) _mm_setzero_ps (),	\
					 (__mmask8) (-1),		\
					 _MM_FROUND_CUR_DIRECTION))
#define _mm_mask_roundscale_ss(A, U, B, C, I)				\
  ((__m128)								\
   __builtin_ia32_rndscaless_mask_round ((__v4sf) (__m128) (B),		\
					 (__v4sf) (__m128) (C),		\
					 (int) (I),			\
					 (__v4sf) (__m128) (A),		\
					 (__mmask8) (U),		\
					 _MM_FROUND_CUR_DIRECTION))
#define _mm_maskz_roundscale_ss(U, A, B, I)				\
  ((__m128)								\
   __builtin_ia32_rndscaless_mask_round ((__v4sf) (__m128) (A),		\
					 (__v4sf) (__m128) (B),		\
					 (int) (I),			\
					 (__v4sf) _mm_setzero_ps (),	\
					 (__mmask8) (U),		\
					 _MM_FROUND_CUR_DIRECTION))
#define _mm_roundscale_sd(A, B, I)					\
  ((__m128d)								\
   __builtin_ia32_rndscalesd_mask_round ((__v2df) (__m128d) (A),	\
					 (__v2df) (__m128d) (B),	\
					 (int) (I),			\
					 (__v2df) _mm_setzero_pd (),	\
					 (__mmask8) (-1),		\
					 _MM_FROUND_CUR_DIRECTION))
#define _mm_mask_roundscale_sd(A, U, B, C, I)				\
  ((__m128d)								\
   __builtin_ia32_rndscalesd_mask_round ((__v2df) (__m128d) (B),	\
					 (__v2df) (__m128d) (C),	\
					 (int) (I),			\
					 (__v2df) (__m128d) (A),	\
					 (__mmask8) (U),		\
					 _MM_FROUND_CUR_DIRECTION))
#define _mm_maskz_roundscale_sd(U, A, B, I)				\
  ((__m128d)								\
   __builtin_ia32_rndscalesd_mask_round ((__v2df) (__m128d) (A),	\
					 (__v2df) (__m128d) (B),	\
					 (int) (I),			\
					 (__v2df) _mm_setzero_pd (),	\
					 (__mmask8) (U),		\
					 _MM_FROUND_CUR_DIRECTION))
#endif

#ifdef __OPTIMIZE__
extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_pd_mask (__m512d __X, __m512d __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, __P,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmp_ps_mask (__m512 __X, __m512 __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, __P,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y, const int __P)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, __P,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmp_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, __P,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_sd_mask (__m128d __X, __m128d __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpsd_mask ((__v2df) __X,
					       (__v2df) __Y, __P,
					       (__mmask8) -1,
					       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_sd_mask (__mmask8 __M, __m128d __X, __m128d __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpsd_mask ((__v2df) __X,
					       (__v2df) __Y, __P,
					       (__mmask8) __M,
					       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_ss_mask (__m128 __X, __m128 __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpss_mask ((__v4sf) __X,
					       (__v4sf) __Y, __P,
					       (__mmask8) -1,
					       _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_cmp_ss_mask (__mmask8 __M, __m128 __X, __m128 __Y, const int __P)
{
  return (__mmask8) __builtin_ia32_cmpss_mask ((__v4sf) __X,
					       (__v4sf) __Y, __P,
					       (__mmask8) __M,
					       _MM_FROUND_CUR_DIRECTION);
}

#else
#define _mm512_cmp_pd_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmppd512_mask ((__v8df)(__m512d)(X),	\
					    (__v8df)(__m512d)(Y), (int)(P),\
					    (__mmask8)-1,_MM_FROUND_CUR_DIRECTION))

#define _mm512_cmp_ps_mask(X, Y, P)					\
  ((__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf)(__m512)(X),	\
					     (__v16sf)(__m512)(Y), (int)(P),\
					     (__mmask16)-1,_MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_cmp_pd_mask(M, X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmppd512_mask ((__v8df)(__m512d)(X),	\
					    (__v8df)(__m512d)(Y), (int)(P),\
					    (__mmask8)(M), _MM_FROUND_CUR_DIRECTION))

#define _mm512_mask_cmp_ps_mask(M, X, Y, P)					\
  ((__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf)(__m512)(X),	\
					     (__v16sf)(__m512)(Y), (int)(P),\
					     (__mmask16)(M),_MM_FROUND_CUR_DIRECTION))

#define _mm_cmp_sd_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpsd_mask ((__v2df)(__m128d)(X),		\
					 (__v2df)(__m128d)(Y), (int)(P),\
					 (__mmask8)-1,_MM_FROUND_CUR_DIRECTION))

#define _mm_mask_cmp_sd_mask(M, X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpsd_mask ((__v2df)(__m128d)(X),		\
					 (__v2df)(__m128d)(Y), (int)(P),\
					 M,_MM_FROUND_CUR_DIRECTION))

#define _mm_cmp_ss_mask(X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpss_mask ((__v4sf)(__m128)(X),		\
					 (__v4sf)(__m128)(Y), (int)(P), \
					 (__mmask8)-1,_MM_FROUND_CUR_DIRECTION))

#define _mm_mask_cmp_ss_mask(M, X, Y, P)					\
  ((__mmask8) __builtin_ia32_cmpss_mask ((__v4sf)(__m128)(X),		\
					 (__v4sf)(__m128)(Y), (int)(P), \
					 M,_MM_FROUND_CUR_DIRECTION))
#endif

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_EQ_OQ,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_EQ_OQ,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_LT_OS,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_LT_OS,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_LE_OS,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_LE_OS,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpunord_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_UNORD_Q,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpunord_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_UNORD_Q,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_NEQ_UQ,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_NEQ_UQ,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpnlt_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_NLT_US,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpnlt_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_NLT_US,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpnle_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_NLE_US,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpnle_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_NLE_US,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpord_pd_mask (__m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_ORD_Q,
						  (__mmask8) -1,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpord_pd_mask (__mmask8 __U, __m512d __X, __m512d __Y)
{
  return (__mmask8) __builtin_ia32_cmppd512_mask ((__v8df) __X,
						  (__v8df) __Y, _CMP_ORD_Q,
						  (__mmask8) __U,
						  _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_EQ_OQ,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_EQ_OQ,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmplt_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_LT_OS,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmplt_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_LT_OS,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmple_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_LE_OS,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmple_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_LE_OS,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpunord_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_UNORD_Q,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpunord_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_UNORD_Q,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpneq_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_NEQ_UQ,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpneq_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_NEQ_UQ,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpnlt_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_NLT_US,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpnlt_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_NLT_US,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpnle_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_NLE_US,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpnle_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_NLE_US,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpord_ps_mask (__m512 __X, __m512 __Y)
{
  return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_ORD_Q,
						   (__mmask16) -1,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpord_ps_mask (__mmask16 __U, __m512 __X, __m512 __Y)
{
   return (__mmask16) __builtin_ia32_cmpps512_mask ((__v16sf) __X,
						   (__v16sf) __Y, _CMP_ORD_Q,
						   (__mmask16) __U,
						   _MM_FROUND_CUR_DIRECTION);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_kmov (__mmask16 __A)
{
  return __builtin_ia32_kmovw (__A);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castpd_ps (__m512d __A)
{
  return (__m512) (__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castpd_si512 (__m512d __A)
{
  return (__m512i) (__A);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castps_pd (__m512 __A)
{
  return (__m512d) (__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castps_si512 (__m512 __A)
{
  return (__m512i) (__A);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castsi512_ps (__m512i __A)
{
  return (__m512) (__A);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castsi512_pd (__m512i __A)
{
  return (__m512d) (__A);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castpd512_pd128 (__m512d __A)
{
  return (__m128d)_mm512_extractf32x4_ps((__m512)__A, 0);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castps512_ps128 (__m512 __A)
{
  return _mm512_extractf32x4_ps(__A, 0);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castsi512_si128 (__m512i __A)
{
  return (__m128i)_mm512_extracti32x4_epi32((__m512i)__A, 0);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castpd512_pd256 (__m512d __A)
{
  return _mm512_extractf64x4_pd(__A, 0);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castps512_ps256 (__m512 __A)
{
  return (__m256)_mm512_extractf64x4_pd((__m512d)__A, 0);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castsi512_si256 (__m512i __A)
{
  return (__m256i)_mm512_extractf64x4_pd((__m512d)__A, 0);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castpd128_pd512 (__m128d __A)
{
  return (__m512d) __builtin_ia32_pd512_pd((__m128d)__A);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castps128_ps512 (__m128 __A)
{
  return (__m512) __builtin_ia32_ps512_ps((__m128)__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castsi128_si512 (__m128i __A)
{
  return (__m512i) __builtin_ia32_si512_si((__v4si)__A);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castpd256_pd512 (__m256d __A)
{
  return __builtin_ia32_pd512_256pd (__A);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castps256_ps512 (__m256 __A)
{
  return __builtin_ia32_ps512_256ps (__A);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_castsi256_si512 (__m256i __A)
{
  return (__m512i)__builtin_ia32_si512_256si ((__v8si)__A);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_zextpd128_pd512 (__m128d __A)
{
  return (__m512d) _mm512_insertf32x4 (_mm512_setzero_ps (), (__m128) __A, 0);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_zextps128_ps512 (__m128 __A)
{
  return _mm512_insertf32x4 (_mm512_setzero_ps (), __A, 0);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_zextsi128_si512 (__m128i __A)
{
  return _mm512_inserti32x4 (_mm512_setzero_si512 (), __A, 0);
}

extern __inline __m512d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_zextpd256_pd512 (__m256d __A)
{
  return _mm512_insertf64x4 (_mm512_setzero_pd (), __A, 0);
}

extern __inline __m512
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_zextps256_ps512 (__m256 __A)
{
  return (__m512) _mm512_insertf64x4 (_mm512_setzero_pd (), (__m256d) __A, 0);
}

extern __inline __m512i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_zextsi256_si512 (__m256i __A)
{
  return _mm512_inserti64x4 (_mm512_setzero_si512 (), __A, 0);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epu32_mask (__m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __A,
						     (__v16si) __B, 0,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epu32_mask (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __A,
						     (__v16si) __B, 0, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpeq_epu64_mask (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __A,
						    (__v8di) __B, 0, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpeq_epu64_mask (__m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __A,
						    (__v8di) __B, 0,
						    (__mmask8) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epu32_mask (__m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __A,
						     (__v16si) __B, 6,
						     (__mmask16) -1);
}

extern __inline __mmask16
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epu32_mask (__mmask16 __U, __m512i __A, __m512i __B)
{
  return (__mmask16) __builtin_ia32_ucmpd512_mask ((__v16si) __A,
						     (__v16si) __B, 6,  __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_cmpgt_epu64_mask (__mmask8 __U, __m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __A,
						    (__v8di) __B, 6, __U);
}

extern __inline __mmask8
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_cmpgt_epu64_mask (__m512i __A, __m512i __B)
{
  return (__mmask8) __builtin_ia32_ucmpq512_mask ((__v8di) __A,
						    (__v8di) __B, 6,
						    (__mmask8) -1);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __v8si __T1 = (__v8si) _mm512_extracti64x4_epi64 (__A, 1);		\
  __v8si __T2 = (__v8si) _mm512_extracti64x4_epi64 (__A, 0);		\
  __m256i __T3 = (__m256i) (__T1 op __T2);				\
  __v4si __T4 = (__v4si) _mm256_extracti128_si256 (__T3, 1);		\
  __v4si __T5 = (__v4si) _mm256_extracti128_si256 (__T3, 0);		\
  __v4si __T6 = __T4 op __T5;						\
  __v4si __T7 = __builtin_shuffle (__T6, (__v4si) { 2, 3, 0, 1 });	\
  __v4si __T8 = __T6 op __T7;						\
  return __T8[0] op __T8[1]

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_add_epi32 (__m512i __A)
{
  __MM512_REDUCE_OP (+);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_mul_epi32 (__m512i __A)
{
  __MM512_REDUCE_OP (*);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_and_epi32 (__m512i __A)
{
  __MM512_REDUCE_OP (&);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_or_epi32 (__m512i __A)
{
  __MM512_REDUCE_OP (|);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_add_epi32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_maskz_mov_epi32 (__U, __A);
  __MM512_REDUCE_OP (+);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_mul_epi32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi32 (_mm512_set1_epi32 (1), __U, __A);
  __MM512_REDUCE_OP (*);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_and_epi32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi32 (_mm512_set1_epi32 (~0), __U, __A);
  __MM512_REDUCE_OP (&);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_or_epi32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_maskz_mov_epi32 (__U, __A);
  __MM512_REDUCE_OP (|);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __m256i __T1 = (__m256i) _mm512_extracti64x4_epi64 (__A, 1);		\
  __m256i __T2 = (__m256i) _mm512_extracti64x4_epi64 (__A, 0);		\
  __m256i __T3 = _mm256_##op (__T1, __T2);				\
  __m128i __T4 = (__m128i) _mm256_extracti128_si256 (__T3, 1);		\
  __m128i __T5 = (__m128i) _mm256_extracti128_si256 (__T3, 0);		\
  __m128i __T6 = _mm_##op (__T4, __T5);					\
  __m128i __T7 = (__m128i) __builtin_shuffle ((__v4si) __T6,		\
					      (__v4si) { 2, 3, 0, 1 });	\
  __m128i __T8 = _mm_##op (__T6, __T7);					\
  __m128i __T9 = (__m128i) __builtin_shuffle ((__v4si) __T8,		\
					      (__v4si) { 1, 0, 1, 0 });	\
  __v4si __T10 = (__v4si) _mm_##op (__T8, __T9);			\
  return __T10[0]

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_min_epi32 (__m512i __A)
{
  __MM512_REDUCE_OP (min_epi32);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_max_epi32 (__m512i __A)
{
  __MM512_REDUCE_OP (max_epi32);
}

extern __inline unsigned int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_min_epu32 (__m512i __A)
{
  __MM512_REDUCE_OP (min_epu32);
}

extern __inline unsigned int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_max_epu32 (__m512i __A)
{
  __MM512_REDUCE_OP (max_epu32);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_min_epi32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi32 (_mm512_set1_epi32 (__INT_MAX__), __U, __A);
  __MM512_REDUCE_OP (min_epi32);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_max_epi32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi32 (_mm512_set1_epi32 (-__INT_MAX__ - 1), __U, __A);
  __MM512_REDUCE_OP (max_epi32);
}

extern __inline unsigned int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_min_epu32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi32 (_mm512_set1_epi32 (~0), __U, __A);
  __MM512_REDUCE_OP (min_epu32);
}

extern __inline unsigned int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_max_epu32 (__mmask16 __U, __m512i __A)
{
  __A = _mm512_maskz_mov_epi32 (__U, __A);
  __MM512_REDUCE_OP (max_epu32);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __m256 __T1 = (__m256) _mm512_extractf64x4_pd ((__m512d) __A, 1);	\
  __m256 __T2 = (__m256) _mm512_extractf64x4_pd ((__m512d) __A, 0);	\
  __m256 __T3 = __T1 op __T2;						\
  __m128 __T4 = _mm256_extractf128_ps (__T3, 1);			\
  __m128 __T5 = _mm256_extractf128_ps (__T3, 0);			\
  __m128 __T6 = __T4 op __T5;						\
  __m128 __T7 = __builtin_shuffle (__T6, (__v4si) { 2, 3, 0, 1 });	\
  __m128 __T8 = __T6 op __T7;						\
  return __T8[0] op __T8[1]

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_add_ps (__m512 __A)
{
  __MM512_REDUCE_OP (+);
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_mul_ps (__m512 __A)
{
  __MM512_REDUCE_OP (*);
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_add_ps (__mmask16 __U, __m512 __A)
{
  __A = _mm512_maskz_mov_ps (__U, __A);
  __MM512_REDUCE_OP (+);
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_mul_ps (__mmask16 __U, __m512 __A)
{
  __A = _mm512_mask_mov_ps (_mm512_set1_ps (1.0f), __U, __A);
  __MM512_REDUCE_OP (*);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __m256 __T1 = (__m256) _mm512_extractf64x4_pd ((__m512d) __A, 1);	\
  __m256 __T2 = (__m256) _mm512_extractf64x4_pd ((__m512d) __A, 0);	\
  __m256 __T3 = _mm256_##op (__T1, __T2);				\
  __m128 __T4 = _mm256_extractf128_ps (__T3, 1);			\
  __m128 __T5 = _mm256_extractf128_ps (__T3, 0);			\
  __m128 __T6 = _mm_##op (__T4, __T5);					\
  __m128 __T7 = __builtin_shuffle (__T6, (__v4si) { 2, 3, 0, 1 });	\
  __m128 __T8 = _mm_##op (__T6, __T7);					\
  __m128 __T9 = __builtin_shuffle (__T8, (__v4si) { 1, 0, 1, 0 });	\
  __m128 __T10 = _mm_##op (__T8, __T9);					\
  return __T10[0]

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_min_ps (__m512 __A)
{
  __MM512_REDUCE_OP (min_ps);
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_max_ps (__m512 __A)
{
  __MM512_REDUCE_OP (max_ps);
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_min_ps (__mmask16 __U, __m512 __A)
{
  __A = _mm512_mask_mov_ps (_mm512_set1_ps (__builtin_inff ()), __U, __A);
  __MM512_REDUCE_OP (min_ps);
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_max_ps (__mmask16 __U, __m512 __A)
{
  __A = _mm512_mask_mov_ps (_mm512_set1_ps (-__builtin_inff ()), __U, __A);
  __MM512_REDUCE_OP (max_ps);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __v4di __T1 = (__v4di) _mm512_extracti64x4_epi64 (__A, 1);		\
  __v4di __T2 = (__v4di) _mm512_extracti64x4_epi64 (__A, 0);		\
  __m256i __T3 = (__m256i) (__T1 op __T2);				\
  __v2di __T4 = (__v2di) _mm256_extracti128_si256 (__T3, 1);		\
  __v2di __T5 = (__v2di) _mm256_extracti128_si256 (__T3, 0);		\
  __v2di __T6 = __T4 op __T5;						\
  return __T6[0] op __T6[1]

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_add_epi64 (__m512i __A)
{
  __MM512_REDUCE_OP (+);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_mul_epi64 (__m512i __A)
{
  __MM512_REDUCE_OP (*);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_and_epi64 (__m512i __A)
{
  __MM512_REDUCE_OP (&);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_or_epi64 (__m512i __A)
{
  __MM512_REDUCE_OP (|);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_add_epi64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_maskz_mov_epi64 (__U, __A);
  __MM512_REDUCE_OP (+);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_mul_epi64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi64 (_mm512_set1_epi64 (1LL), __U, __A);
  __MM512_REDUCE_OP (*);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_and_epi64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi64 (_mm512_set1_epi64 (~0LL), __U, __A);
  __MM512_REDUCE_OP (&);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_or_epi64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_maskz_mov_epi64 (__U, __A);
  __MM512_REDUCE_OP (|);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __m512i __T1 = _mm512_shuffle_i64x2 (__A, __A, 0x4e);			\
  __m512i __T2 = _mm512_##op (__A, __T1);				\
  __m512i __T3								\
    = (__m512i) __builtin_shuffle ((__v8di) __T2,			\
				   (__v8di) { 2, 3, 0, 1, 6, 7, 4, 5 });\
  __m512i __T4 = _mm512_##op (__T2, __T3);				\
  __m512i __T5								\
    = (__m512i) __builtin_shuffle ((__v8di) __T4,			\
				   (__v8di) { 1, 0, 3, 2, 5, 4, 7, 6 });\
  __v8di __T6 = (__v8di) _mm512_##op (__T4, __T5);			\
  return __T6[0]

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_min_epi64 (__m512i __A)
{
  __MM512_REDUCE_OP (min_epi64);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_max_epi64 (__m512i __A)
{
  __MM512_REDUCE_OP (max_epi64);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_min_epi64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi64 (_mm512_set1_epi64 (__LONG_LONG_MAX__),
			       __U, __A);
  __MM512_REDUCE_OP (min_epi64);
}

extern __inline long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_max_epi64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi64 (_mm512_set1_epi64 (-__LONG_LONG_MAX__ - 1),
			       __U, __A);
  __MM512_REDUCE_OP (max_epi64);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_min_epu64 (__m512i __A)
{
  __MM512_REDUCE_OP (min_epu64);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_max_epu64 (__m512i __A)
{
  __MM512_REDUCE_OP (max_epu64);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_min_epu64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_mask_mov_epi64 (_mm512_set1_epi64 (~0LL), __U, __A);
  __MM512_REDUCE_OP (min_epu64);
}

extern __inline unsigned long long
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_max_epu64 (__mmask8 __U, __m512i __A)
{
  __A = _mm512_maskz_mov_epi64 (__U, __A);
  __MM512_REDUCE_OP (max_epu64);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __m256d __T1 = (__m256d) _mm512_extractf64x4_pd (__A, 1);		\
  __m256d __T2 = (__m256d) _mm512_extractf64x4_pd (__A, 0);		\
  __m256d __T3 = __T1 op __T2;						\
  __m128d __T4 = _mm256_extractf128_pd (__T3, 1);			\
  __m128d __T5 = _mm256_extractf128_pd (__T3, 0);			\
  __m128d __T6 = __T4 op __T5;						\
  return __T6[0] op __T6[1]

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_add_pd (__m512d __A)
{
  __MM512_REDUCE_OP (+);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_mul_pd (__m512d __A)
{
  __MM512_REDUCE_OP (*);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_add_pd (__mmask8 __U, __m512d __A)
{
  __A = _mm512_maskz_mov_pd (__U, __A);
  __MM512_REDUCE_OP (+);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_mul_pd (__mmask8 __U, __m512d __A)
{
  __A = _mm512_mask_mov_pd (_mm512_set1_pd (1.0), __U, __A);
  __MM512_REDUCE_OP (*);
}

#undef __MM512_REDUCE_OP
#define __MM512_REDUCE_OP(op) \
  __m256d __T1 = (__m256d) _mm512_extractf64x4_pd (__A, 1);		\
  __m256d __T2 = (__m256d) _mm512_extractf64x4_pd (__A, 0);		\
  __m256d __T3 = _mm256_##op (__T1, __T2);				\
  __m128d __T4 = _mm256_extractf128_pd (__T3, 1);			\
  __m128d __T5 = _mm256_extractf128_pd (__T3, 0);			\
  __m128d __T6 = _mm_##op (__T4, __T5);					\
  __m128d __T7 = (__m128d) __builtin_shuffle (__T6, (__v2di) { 1, 0 });	\
  __m128d __T8 = _mm_##op (__T6, __T7);					\
  return __T8[0]

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_min_pd (__m512d __A)
{
  __MM512_REDUCE_OP (min_pd);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_reduce_max_pd (__m512d __A)
{
  __MM512_REDUCE_OP (max_pd);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_min_pd (__mmask8 __U, __m512d __A)
{
  __A = _mm512_mask_mov_pd (_mm512_set1_pd (__builtin_inf ()), __U, __A);
  __MM512_REDUCE_OP (min_pd);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm512_mask_reduce_max_pd (__mmask8 __U, __m512d __A)
{
  __A = _mm512_mask_mov_pd (_mm512_set1_pd (-__builtin_inf ()), __U, __A);
  __MM512_REDUCE_OP (max_pd);
}

#undef __MM512_REDUCE_OP

#ifdef __DISABLE_AVX512F__
#undef __DISABLE_AVX512F__
#pragma GCC pop_options
#endif /* __DISABLE_AVX512F__ */

#endif /* _AVX512FINTRIN_H_INCLUDED */
