/* Copyright (C) 2008-2020 Free Software Foundation, Inc.

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
   User Guide and Reference, version 11.0.  */

#ifndef _IMMINTRIN_H_INCLUDED
# error "Never use <avxintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVXINTRIN_H_INCLUDED
#define _AVXINTRIN_H_INCLUDED

#ifndef __AVX__
#pragma GCC push_options
#pragma GCC target("avx")
#define __DISABLE_AVX__
#endif /* __AVX__ */

/* Internal data types for implementing the intrinsics.  */
typedef double __v4df __attribute__ ((__vector_size__ (32)));
typedef float __v8sf __attribute__ ((__vector_size__ (32)));
typedef long long __v4di __attribute__ ((__vector_size__ (32)));
typedef unsigned long long __v4du __attribute__ ((__vector_size__ (32)));
typedef int __v8si __attribute__ ((__vector_size__ (32)));
typedef unsigned int __v8su __attribute__ ((__vector_size__ (32)));
typedef short __v16hi __attribute__ ((__vector_size__ (32)));
typedef unsigned short __v16hu __attribute__ ((__vector_size__ (32)));
typedef char __v32qi __attribute__ ((__vector_size__ (32)));
typedef signed char __v32qs __attribute__ ((__vector_size__ (32)));
typedef unsigned char __v32qu __attribute__ ((__vector_size__ (32)));

/* The Intel API is flexible enough that we must allow aliasing with other
   vector types, and their scalar components.  */
typedef float __m256 __attribute__ ((__vector_size__ (32),
				     __may_alias__));
typedef long long __m256i __attribute__ ((__vector_size__ (32),
					  __may_alias__));
typedef double __m256d __attribute__ ((__vector_size__ (32),
				       __may_alias__));

/* Unaligned version of the same types.  */
typedef float __m256_u __attribute__ ((__vector_size__ (32),
				       __may_alias__,
				       __aligned__ (1)));
typedef long long __m256i_u __attribute__ ((__vector_size__ (32),
					    __may_alias__,
					    __aligned__ (1)));
typedef double __m256d_u __attribute__ ((__vector_size__ (32),
					 __may_alias__,
					 __aligned__ (1)));

/* Compare predicates for scalar and packed compare intrinsics.  */

/* Equal (ordered, non-signaling)  */
#define _CMP_EQ_OQ	0x00
/* Less-than (ordered, signaling)  */
#define _CMP_LT_OS	0x01
/* Less-than-or-equal (ordered, signaling)  */
#define _CMP_LE_OS	0x02
/* Unordered (non-signaling)  */
#define _CMP_UNORD_Q	0x03
/* Not-equal (unordered, non-signaling)  */
#define _CMP_NEQ_UQ	0x04
/* Not-less-than (unordered, signaling)  */
#define _CMP_NLT_US	0x05
/* Not-less-than-or-equal (unordered, signaling)  */
#define _CMP_NLE_US	0x06
/* Ordered (nonsignaling)   */
#define _CMP_ORD_Q	0x07
/* Equal (unordered, non-signaling)  */
#define _CMP_EQ_UQ	0x08
/* Not-greater-than-or-equal (unordered, signaling)  */
#define _CMP_NGE_US	0x09
/* Not-greater-than (unordered, signaling)  */
#define _CMP_NGT_US	0x0a
/* False (ordered, non-signaling)  */
#define _CMP_FALSE_OQ	0x0b
/* Not-equal (ordered, non-signaling)  */
#define _CMP_NEQ_OQ	0x0c
/* Greater-than-or-equal (ordered, signaling)  */
#define _CMP_GE_OS	0x0d
/* Greater-than (ordered, signaling)  */
#define _CMP_GT_OS	0x0e
/* True (unordered, non-signaling)  */
#define _CMP_TRUE_UQ	0x0f
/* Equal (ordered, signaling)  */
#define _CMP_EQ_OS	0x10
/* Less-than (ordered, non-signaling)  */
#define _CMP_LT_OQ	0x11
/* Less-than-or-equal (ordered, non-signaling)  */
#define _CMP_LE_OQ	0x12
/* Unordered (signaling)  */
#define _CMP_UNORD_S	0x13
/* Not-equal (unordered, signaling)  */
#define _CMP_NEQ_US	0x14
/* Not-less-than (unordered, non-signaling)  */
#define _CMP_NLT_UQ	0x15
/* Not-less-than-or-equal (unordered, non-signaling)  */
#define _CMP_NLE_UQ	0x16
/* Ordered (signaling)  */
#define _CMP_ORD_S	0x17
/* Equal (unordered, signaling)  */
#define _CMP_EQ_US	0x18
/* Not-greater-than-or-equal (unordered, non-signaling)  */
#define _CMP_NGE_UQ	0x19
/* Not-greater-than (unordered, non-signaling)  */
#define _CMP_NGT_UQ	0x1a
/* False (ordered, signaling)  */
#define _CMP_FALSE_OS	0x1b
/* Not-equal (ordered, signaling)  */
#define _CMP_NEQ_OS	0x1c
/* Greater-than-or-equal (ordered, non-signaling)  */
#define _CMP_GE_OQ	0x1d
/* Greater-than (ordered, non-signaling)  */
#define _CMP_GT_OQ	0x1e
/* True (unordered, signaling)  */
#define _CMP_TRUE_US	0x1f

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_add_pd (__m256d __A, __m256d __B)
{
  return (__m256d) ((__v4df)__A + (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_add_ps (__m256 __A, __m256 __B)
{
  return (__m256) ((__v8sf)__A + (__v8sf)__B);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_addsub_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_addsubpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_addsub_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_addsubps256 ((__v8sf)__A, (__v8sf)__B);
}


extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_and_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_andpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_and_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_andps256 ((__v8sf)__A, (__v8sf)__B);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_andnot_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_andnpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_andnot_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_andnps256 ((__v8sf)__A, (__v8sf)__B);
}

/* Double/single precision floating point blend instructions - select
   data from 2 sources using constant/variable mask.  */

#ifdef __OPTIMIZE__
extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_blend_pd (__m256d __X, __m256d __Y, const int __M)
{
  return (__m256d) __builtin_ia32_blendpd256 ((__v4df)__X,
					      (__v4df)__Y,
					      __M);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_blend_ps (__m256 __X, __m256 __Y, const int __M)
{
  return (__m256) __builtin_ia32_blendps256 ((__v8sf)__X,
					     (__v8sf)__Y,
					     __M);
}
#else
#define _mm256_blend_pd(X, Y, M)					\
  ((__m256d) __builtin_ia32_blendpd256 ((__v4df)(__m256d)(X),		\
					(__v4df)(__m256d)(Y), (int)(M)))

#define _mm256_blend_ps(X, Y, M)					\
  ((__m256) __builtin_ia32_blendps256 ((__v8sf)(__m256)(X),		\
				       (__v8sf)(__m256)(Y), (int)(M)))
#endif

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_blendv_pd (__m256d __X, __m256d __Y, __m256d __M)
{
  return (__m256d) __builtin_ia32_blendvpd256 ((__v4df)__X,
					       (__v4df)__Y,
					       (__v4df)__M);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_blendv_ps (__m256 __X, __m256 __Y, __m256 __M)
{
  return (__m256) __builtin_ia32_blendvps256 ((__v8sf)__X,
					      (__v8sf)__Y,
					      (__v8sf)__M);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_div_pd (__m256d __A, __m256d __B)
{
  return (__m256d) ((__v4df)__A / (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_div_ps (__m256 __A, __m256 __B)
{
  return (__m256) ((__v8sf)__A / (__v8sf)__B);
}

/* Dot product instructions with mask-defined summing and zeroing parts
   of result.  */

#ifdef __OPTIMIZE__
extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_dp_ps (__m256 __X, __m256 __Y, const int __M)
{
  return (__m256) __builtin_ia32_dpps256 ((__v8sf)__X,
					  (__v8sf)__Y,
					  __M);
}
#else
#define _mm256_dp_ps(X, Y, M)						\
  ((__m256) __builtin_ia32_dpps256 ((__v8sf)(__m256)(X),		\
				    (__v8sf)(__m256)(Y), (int)(M)))
#endif

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hadd_pd (__m256d __X, __m256d __Y)
{
  return (__m256d) __builtin_ia32_haddpd256 ((__v4df)__X, (__v4df)__Y);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hadd_ps (__m256 __X, __m256 __Y)
{
  return (__m256) __builtin_ia32_haddps256 ((__v8sf)__X, (__v8sf)__Y);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hsub_pd (__m256d __X, __m256d __Y)
{
  return (__m256d) __builtin_ia32_hsubpd256 ((__v4df)__X, (__v4df)__Y);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hsub_ps (__m256 __X, __m256 __Y)
{
  return (__m256) __builtin_ia32_hsubps256 ((__v8sf)__X, (__v8sf)__Y);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_maxpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_maxps256 ((__v8sf)__A, (__v8sf)__B);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_minpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_minps256 ((__v8sf)__A, (__v8sf)__B);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mul_pd (__m256d __A, __m256d __B)
{
  return (__m256d) ((__v4df)__A * (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mul_ps (__m256 __A, __m256 __B)
{
  return (__m256) ((__v8sf)__A * (__v8sf)__B);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_or_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_orpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_or_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_orps256 ((__v8sf)__A, (__v8sf)__B);
}

#ifdef __OPTIMIZE__
extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_pd (__m256d __A, __m256d __B, const int __mask)
{
  return (__m256d) __builtin_ia32_shufpd256 ((__v4df)__A, (__v4df)__B,
					     __mask);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_ps (__m256 __A, __m256 __B, const int __mask)
{
  return (__m256) __builtin_ia32_shufps256 ((__v8sf)__A, (__v8sf)__B,
					    __mask);
}
#else
#define _mm256_shuffle_pd(A, B, N)					\
  ((__m256d)__builtin_ia32_shufpd256 ((__v4df)(__m256d)(A),		\
				      (__v4df)(__m256d)(B), (int)(N)))

#define _mm256_shuffle_ps(A, B, N)					\
  ((__m256) __builtin_ia32_shufps256 ((__v8sf)(__m256)(A),		\
				      (__v8sf)(__m256)(B), (int)(N)))
#endif

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sub_pd (__m256d __A, __m256d __B)
{
  return (__m256d) ((__v4df)__A - (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sub_ps (__m256 __A, __m256 __B)
{
  return (__m256) ((__v8sf)__A - (__v8sf)__B);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_xor_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_xorpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_xor_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_xorps256 ((__v8sf)__A, (__v8sf)__B);
}

#ifdef __OPTIMIZE__
extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_pd (__m128d __X, __m128d __Y, const int __P)
{
  return (__m128d) __builtin_ia32_cmppd ((__v2df)__X, (__v2df)__Y, __P);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_ps (__m128 __X, __m128 __Y, const int __P)
{
  return (__m128) __builtin_ia32_cmpps ((__v4sf)__X, (__v4sf)__Y, __P);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_pd (__m256d __X, __m256d __Y, const int __P)
{
  return (__m256d) __builtin_ia32_cmppd256 ((__v4df)__X, (__v4df)__Y,
					    __P);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmp_ps (__m256 __X, __m256 __Y, const int __P)
{
  return (__m256) __builtin_ia32_cmpps256 ((__v8sf)__X, (__v8sf)__Y,
					   __P);
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_sd (__m128d __X, __m128d __Y, const int __P)
{
  return (__m128d) __builtin_ia32_cmpsd ((__v2df)__X, (__v2df)__Y, __P);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmp_ss (__m128 __X, __m128 __Y, const int __P)
{
  return (__m128) __builtin_ia32_cmpss ((__v4sf)__X, (__v4sf)__Y, __P);
}
#else
#define _mm_cmp_pd(X, Y, P)						\
  ((__m128d) __builtin_ia32_cmppd ((__v2df)(__m128d)(X),		\
				   (__v2df)(__m128d)(Y), (int)(P)))

#define _mm_cmp_ps(X, Y, P)						\
  ((__m128) __builtin_ia32_cmpps ((__v4sf)(__m128)(X),			\
				  (__v4sf)(__m128)(Y), (int)(P)))

#define _mm256_cmp_pd(X, Y, P)						\
  ((__m256d) __builtin_ia32_cmppd256 ((__v4df)(__m256d)(X),		\
				      (__v4df)(__m256d)(Y), (int)(P)))

#define _mm256_cmp_ps(X, Y, P)						\
  ((__m256) __builtin_ia32_cmpps256 ((__v8sf)(__m256)(X),		\
				     (__v8sf)(__m256)(Y), (int)(P)))

#define _mm_cmp_sd(X, Y, P)						\
  ((__m128d) __builtin_ia32_cmpsd ((__v2df)(__m128d)(X),		\
				   (__v2df)(__m128d)(Y), (int)(P)))

#define _mm_cmp_ss(X, Y, P)						\
  ((__m128) __builtin_ia32_cmpss ((__v4sf)(__m128)(X),			\
				  (__v4sf)(__m128)(Y), (int)(P)))
#endif

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi32_pd (__m128i __A)
{
  return (__m256d)__builtin_ia32_cvtdq2pd256 ((__v4si) __A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi32_ps (__m256i __A)
{
  return (__m256)__builtin_ia32_cvtdq2ps256 ((__v8si) __A);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtpd_ps (__m256d __A)
{
  return (__m128)__builtin_ia32_cvtpd2ps256 ((__v4df) __A);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtps_epi32 (__m256 __A)
{
  return (__m256i)__builtin_ia32_cvtps2dq256 ((__v8sf) __A);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtps_pd (__m128 __A)
{
  return (__m256d)__builtin_ia32_cvtps2pd256 ((__v4sf) __A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttpd_epi32 (__m256d __A)
{
  return (__m128i)__builtin_ia32_cvttpd2dq256 ((__v4df) __A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtpd_epi32 (__m256d __A)
{
  return (__m128i)__builtin_ia32_cvtpd2dq256 ((__v4df) __A);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvttps_epi32 (__m256 __A)
{
  return (__m256i)__builtin_ia32_cvttps2dq256 ((__v8sf) __A);
}

extern __inline double
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtsd_f64 (__m256d __A)
{
  return __A[0];
}

extern __inline float
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtss_f32 (__m256 __A)
{
  return __A[0];
}

#ifdef __OPTIMIZE__
extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extractf128_pd (__m256d __X, const int __N)
{
  return (__m128d) __builtin_ia32_vextractf128_pd256 ((__v4df)__X, __N);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extractf128_ps (__m256 __X, const int __N)
{
  return (__m128) __builtin_ia32_vextractf128_ps256 ((__v8sf)__X, __N);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extractf128_si256 (__m256i __X, const int __N)
{
  return (__m128i) __builtin_ia32_vextractf128_si256 ((__v8si)__X, __N);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extract_epi32 (__m256i __X, int const __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 2);
  return _mm_extract_epi32 (__Y, __N % 4);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extract_epi16 (__m256i __X, int const __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 3);
  return _mm_extract_epi16 (__Y, __N % 8);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extract_epi8 (__m256i __X, int const __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 4);
  return _mm_extract_epi8 (__Y, __N % 16);
}

#ifdef __x86_64__
extern __inline long long  __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extract_epi64 (__m256i __X, const int __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 1);
  return _mm_extract_epi64 (__Y, __N % 2);
}
#endif
#else
#define _mm256_extractf128_pd(X, N)					\
  ((__m128d) __builtin_ia32_vextractf128_pd256 ((__v4df)(__m256d)(X),	\
						(int)(N)))

#define _mm256_extractf128_ps(X, N)					\
  ((__m128) __builtin_ia32_vextractf128_ps256 ((__v8sf)(__m256)(X),	\
					       (int)(N)))

#define _mm256_extractf128_si256(X, N)					\
  ((__m128i) __builtin_ia32_vextractf128_si256 ((__v8si)(__m256i)(X),	\
						(int)(N)))

#define _mm256_extract_epi32(X, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 2);		\
      _mm_extract_epi32 (__Y, (N) % 4);					\
    }))

#define _mm256_extract_epi16(X, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 3);		\
      _mm_extract_epi16 (__Y, (N) % 8);					\
    }))

#define _mm256_extract_epi8(X, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 4);		\
      _mm_extract_epi8 (__Y, (N) % 16);					\
    }))

#ifdef __x86_64__
#define _mm256_extract_epi64(X, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 1);		\
      _mm_extract_epi64 (__Y, (N) % 2);					\
    }))
#endif
#endif

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_zeroall (void)
{
  __builtin_ia32_vzeroall ();
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_zeroupper (void)
{
  __builtin_ia32_vzeroupper ();
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutevar_pd (__m128d __A, __m128i __C)
{
  return (__m128d) __builtin_ia32_vpermilvarpd ((__v2df)__A,
						(__v2di)__C);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutevar_pd (__m256d __A, __m256i __C)
{
  return (__m256d) __builtin_ia32_vpermilvarpd256 ((__v4df)__A,
						   (__v4di)__C);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_permutevar_ps (__m128 __A, __m128i __C)
{
  return (__m128) __builtin_ia32_vpermilvarps ((__v4sf)__A,
					       (__v4si)__C);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutevar_ps (__m256 __A, __m256i __C)
{
  return (__m256) __builtin_ia32_vpermilvarps256 ((__v8sf)__A,
						  (__v8si)__C);
}

#ifdef __OPTIMIZE__
extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_permute_pd (__m128d __X, const int __C)
{
  return (__m128d) __builtin_ia32_vpermilpd ((__v2df)__X, __C);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute_pd (__m256d __X, const int __C)
{
  return (__m256d) __builtin_ia32_vpermilpd256 ((__v4df)__X, __C);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_permute_ps (__m128 __X, const int __C)
{
  return (__m128) __builtin_ia32_vpermilps ((__v4sf)__X, __C);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute_ps (__m256 __X, const int __C)
{
  return (__m256) __builtin_ia32_vpermilps256 ((__v8sf)__X, __C);
}
#else
#define _mm_permute_pd(X, C)						\
  ((__m128d) __builtin_ia32_vpermilpd ((__v2df)(__m128d)(X), (int)(C)))

#define _mm256_permute_pd(X, C)						\
  ((__m256d) __builtin_ia32_vpermilpd256 ((__v4df)(__m256d)(X),	(int)(C)))

#define _mm_permute_ps(X, C)						\
  ((__m128) __builtin_ia32_vpermilps ((__v4sf)(__m128)(X), (int)(C)))

#define _mm256_permute_ps(X, C)						\
  ((__m256) __builtin_ia32_vpermilps256 ((__v8sf)(__m256)(X), (int)(C)))
#endif

#ifdef __OPTIMIZE__
extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute2f128_pd (__m256d __X, __m256d __Y, const int __C)
{
  return (__m256d) __builtin_ia32_vperm2f128_pd256 ((__v4df)__X,
						    (__v4df)__Y,
						    __C);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute2f128_ps (__m256 __X, __m256 __Y, const int __C)
{
  return (__m256) __builtin_ia32_vperm2f128_ps256 ((__v8sf)__X,
						   (__v8sf)__Y,
						   __C);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute2f128_si256 (__m256i __X, __m256i __Y, const int __C)
{
  return (__m256i) __builtin_ia32_vperm2f128_si256 ((__v8si)__X,
						    (__v8si)__Y,
						    __C);
}
#else
#define _mm256_permute2f128_pd(X, Y, C)					\
  ((__m256d) __builtin_ia32_vperm2f128_pd256 ((__v4df)(__m256d)(X),	\
					      (__v4df)(__m256d)(Y),	\
					      (int)(C)))

#define _mm256_permute2f128_ps(X, Y, C)					\
  ((__m256) __builtin_ia32_vperm2f128_ps256 ((__v8sf)(__m256)(X),	\
					     (__v8sf)(__m256)(Y),	\
					     (int)(C)))

#define _mm256_permute2f128_si256(X, Y, C)				\
  ((__m256i) __builtin_ia32_vperm2f128_si256 ((__v8si)(__m256i)(X),	\
					      (__v8si)(__m256i)(Y),	\
					      (int)(C)))
#endif

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcast_ss (float const *__X)
{
  return (__m128) __builtin_ia32_vbroadcastss (__X);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_sd (double const *__X)
{
  return (__m256d) __builtin_ia32_vbroadcastsd256 (__X);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_ss (float const *__X)
{
  return (__m256) __builtin_ia32_vbroadcastss256 (__X);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_pd (__m128d const *__X)
{
  return (__m256d) __builtin_ia32_vbroadcastf128_pd256 (__X);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcast_ps (__m128 const *__X)
{
  return (__m256) __builtin_ia32_vbroadcastf128_ps256 (__X);
}

#ifdef __OPTIMIZE__
extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insertf128_pd (__m256d __X, __m128d __Y, const int __O)
{
  return (__m256d) __builtin_ia32_vinsertf128_pd256 ((__v4df)__X,
						     (__v2df)__Y,
						     __O);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insertf128_ps (__m256 __X, __m128 __Y, const int __O)
{
  return (__m256) __builtin_ia32_vinsertf128_ps256 ((__v8sf)__X,
						    (__v4sf)__Y,
						    __O);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insertf128_si256 (__m256i __X, __m128i __Y, const int __O)
{
  return (__m256i) __builtin_ia32_vinsertf128_si256 ((__v8si)__X,
						     (__v4si)__Y,
						     __O);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insert_epi32 (__m256i __X, int __D, int const __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 2);
  __Y = _mm_insert_epi32 (__Y, __D, __N % 4);
  return _mm256_insertf128_si256 (__X, __Y, __N >> 2);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insert_epi16 (__m256i __X, int __D, int const __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 3);
  __Y = _mm_insert_epi16 (__Y, __D, __N % 8);
  return _mm256_insertf128_si256 (__X, __Y, __N >> 3);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insert_epi8 (__m256i __X, int __D, int const __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 4);
  __Y = _mm_insert_epi8 (__Y, __D, __N % 16);
  return _mm256_insertf128_si256 (__X, __Y, __N >> 4);
}

#ifdef __x86_64__
extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_insert_epi64 (__m256i __X, long long __D, int const __N)
{
  __m128i __Y = _mm256_extractf128_si256 (__X, __N >> 1);
  __Y = _mm_insert_epi64 (__Y, __D, __N % 2);
  return _mm256_insertf128_si256 (__X, __Y, __N >> 1);
}
#endif
#else
#define _mm256_insertf128_pd(X, Y, O)					\
  ((__m256d) __builtin_ia32_vinsertf128_pd256 ((__v4df)(__m256d)(X),	\
					       (__v2df)(__m128d)(Y),	\
					       (int)(O)))

#define _mm256_insertf128_ps(X, Y, O)					\
  ((__m256) __builtin_ia32_vinsertf128_ps256 ((__v8sf)(__m256)(X),	\
					      (__v4sf)(__m128)(Y),  	\
					      (int)(O)))

#define _mm256_insertf128_si256(X, Y, O)				\
  ((__m256i) __builtin_ia32_vinsertf128_si256 ((__v8si)(__m256i)(X),	\
					       (__v4si)(__m128i)(Y),	\
					       (int)(O)))

#define _mm256_insert_epi32(X, D, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 2);		\
      __Y = _mm_insert_epi32 (__Y, (D), (N) % 4);			\
      _mm256_insertf128_si256 ((X), __Y, (N) >> 2);			\
    }))

#define _mm256_insert_epi16(X, D, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 3);		\
      __Y = _mm_insert_epi16 (__Y, (D), (N) % 8);			\
      _mm256_insertf128_si256 ((X), __Y, (N) >> 3);			\
    }))

#define _mm256_insert_epi8(X, D, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 4);		\
      __Y = _mm_insert_epi8 (__Y, (D), (N) % 16);			\
      _mm256_insertf128_si256 ((X), __Y, (N) >> 4);			\
    }))

#ifdef __x86_64__
#define _mm256_insert_epi64(X, D, N)					\
  (__extension__							\
   ({									\
      __m128i __Y = _mm256_extractf128_si256 ((X), (N) >> 1);		\
      __Y = _mm_insert_epi64 (__Y, (D), (N) % 2);			\
      _mm256_insertf128_si256 ((X), __Y, (N) >> 1);			\
    }))
#endif
#endif

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_load_pd (double const *__P)
{
  return *(__m256d *)__P;
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_store_pd (double *__P, __m256d __A)
{
  *(__m256d *)__P = __A;
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_load_ps (float const *__P)
{
  return *(__m256 *)__P;
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_store_ps (float *__P, __m256 __A)
{
  *(__m256 *)__P = __A;
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_loadu_pd (double const *__P)
{
  return *(__m256d_u *)__P;
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu_pd (double *__P, __m256d __A)
{
  *(__m256d_u *)__P = __A;
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_loadu_ps (float const *__P)
{
  return *(__m256_u *)__P;
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu_ps (float *__P, __m256 __A)
{
  *(__m256_u *)__P = __A;
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_load_si256 (__m256i const *__P)
{
  return *__P;
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_store_si256 (__m256i *__P, __m256i __A)
{
  *__P = __A;
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_loadu_si256 (__m256i_u const *__P)
{
  return *__P;
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu_si256 (__m256i_u *__P, __m256i __A)
{
  *__P = __A;
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskload_pd (double const *__P, __m128i __M)
{
  return (__m128d) __builtin_ia32_maskloadpd ((const __v2df *)__P,
					      (__v2di)__M);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskstore_pd (double *__P, __m128i __M, __m128d __A)
{
  __builtin_ia32_maskstorepd ((__v2df *)__P, (__v2di)__M, (__v2df)__A);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskload_pd (double const *__P, __m256i __M)
{
  return (__m256d) __builtin_ia32_maskloadpd256 ((const __v4df *)__P,
						 (__v4di)__M);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskstore_pd (double *__P, __m256i __M, __m256d __A)
{
  __builtin_ia32_maskstorepd256 ((__v4df *)__P, (__v4di)__M, (__v4df)__A);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskload_ps (float const *__P, __m128i __M)
{
  return (__m128) __builtin_ia32_maskloadps ((const __v4sf *)__P,
					     (__v4si)__M);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskstore_ps (float *__P, __m128i __M, __m128 __A)
{
  __builtin_ia32_maskstoreps ((__v4sf *)__P, (__v4si)__M, (__v4sf)__A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskload_ps (float const *__P, __m256i __M)
{
  return (__m256) __builtin_ia32_maskloadps256 ((const __v8sf *)__P,
						(__v8si)__M);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskstore_ps (float *__P, __m256i __M, __m256 __A)
{
  __builtin_ia32_maskstoreps256 ((__v8sf *)__P, (__v8si)__M, (__v8sf)__A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movehdup_ps (__m256 __X)
{
  return (__m256) __builtin_ia32_movshdup256 ((__v8sf)__X);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_moveldup_ps (__m256 __X)
{
  return (__m256) __builtin_ia32_movsldup256 ((__v8sf)__X);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movedup_pd (__m256d __X)
{
  return (__m256d) __builtin_ia32_movddup256 ((__v4df)__X);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_lddqu_si256 (__m256i const *__P)
{
  return (__m256i) __builtin_ia32_lddqu256 ((char const *)__P);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_stream_si256 (__m256i *__A, __m256i __B)
{
  __builtin_ia32_movntdq256 ((__v4di *)__A, (__v4di)__B);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_stream_pd (double *__A, __m256d __B)
{
  __builtin_ia32_movntpd256 (__A, (__v4df)__B);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_stream_ps (float *__P, __m256 __A)
{
  __builtin_ia32_movntps256 (__P, (__v8sf)__A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rcp_ps (__m256 __A)
{
  return (__m256) __builtin_ia32_rcpps256 ((__v8sf)__A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_rsqrt_ps (__m256 __A)
{
  return (__m256) __builtin_ia32_rsqrtps256 ((__v8sf)__A);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sqrt_pd (__m256d __A)
{
  return (__m256d) __builtin_ia32_sqrtpd256 ((__v4df)__A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sqrt_ps (__m256 __A)
{
  return (__m256) __builtin_ia32_sqrtps256 ((__v8sf)__A);
}

#ifdef __OPTIMIZE__
extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_round_pd (__m256d __V, const int __M)
{
  return (__m256d) __builtin_ia32_roundpd256 ((__v4df)__V, __M);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_round_ps (__m256 __V, const int __M)
{
  return (__m256) __builtin_ia32_roundps256 ((__v8sf)__V, __M);
}
#else
#define _mm256_round_pd(V, M) \
  ((__m256d) __builtin_ia32_roundpd256 ((__v4df)(__m256d)(V), (int)(M)))

#define _mm256_round_ps(V, M) \
  ((__m256) __builtin_ia32_roundps256 ((__v8sf)(__m256)(V), (int)(M)))
#endif

#define _mm256_ceil_pd(V)	_mm256_round_pd ((V), _MM_FROUND_CEIL)
#define _mm256_floor_pd(V)	_mm256_round_pd ((V), _MM_FROUND_FLOOR)
#define _mm256_ceil_ps(V)	_mm256_round_ps ((V), _MM_FROUND_CEIL)
#define _mm256_floor_ps(V)	_mm256_round_ps ((V), _MM_FROUND_FLOOR)

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpackhi_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_unpckhpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpacklo_pd (__m256d __A, __m256d __B)
{
  return (__m256d) __builtin_ia32_unpcklpd256 ((__v4df)__A, (__v4df)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpackhi_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_unpckhps256 ((__v8sf)__A, (__v8sf)__B);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpacklo_ps (__m256 __A, __m256 __B)
{
  return (__m256) __builtin_ia32_unpcklps256 ((__v8sf)__A, (__v8sf)__B);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testz_pd (__m128d __M, __m128d __V)
{
  return __builtin_ia32_vtestzpd ((__v2df)__M, (__v2df)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testc_pd (__m128d __M, __m128d __V)
{
  return __builtin_ia32_vtestcpd ((__v2df)__M, (__v2df)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testnzc_pd (__m128d __M, __m128d __V)
{
  return __builtin_ia32_vtestnzcpd ((__v2df)__M, (__v2df)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testz_ps (__m128 __M, __m128 __V)
{
  return __builtin_ia32_vtestzps ((__v4sf)__M, (__v4sf)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testc_ps (__m128 __M, __m128 __V)
{
  return __builtin_ia32_vtestcps ((__v4sf)__M, (__v4sf)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_testnzc_ps (__m128 __M, __m128 __V)
{
  return __builtin_ia32_vtestnzcps ((__v4sf)__M, (__v4sf)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testz_pd (__m256d __M, __m256d __V)
{
  return __builtin_ia32_vtestzpd256 ((__v4df)__M, (__v4df)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testc_pd (__m256d __M, __m256d __V)
{
  return __builtin_ia32_vtestcpd256 ((__v4df)__M, (__v4df)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testnzc_pd (__m256d __M, __m256d __V)
{
  return __builtin_ia32_vtestnzcpd256 ((__v4df)__M, (__v4df)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testz_ps (__m256 __M, __m256 __V)
{
  return __builtin_ia32_vtestzps256 ((__v8sf)__M, (__v8sf)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testc_ps (__m256 __M, __m256 __V)
{
  return __builtin_ia32_vtestcps256 ((__v8sf)__M, (__v8sf)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testnzc_ps (__m256 __M, __m256 __V)
{
  return __builtin_ia32_vtestnzcps256 ((__v8sf)__M, (__v8sf)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testz_si256 (__m256i __M, __m256i __V)
{
  return __builtin_ia32_ptestz256 ((__v4di)__M, (__v4di)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testc_si256 (__m256i __M, __m256i __V)
{
  return __builtin_ia32_ptestc256 ((__v4di)__M, (__v4di)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_testnzc_si256 (__m256i __M, __m256i __V)
{
  return __builtin_ia32_ptestnzc256 ((__v4di)__M, (__v4di)__V);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movemask_pd (__m256d __A)
{
  return __builtin_ia32_movmskpd256 ((__v4df)__A);
}

extern __inline int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movemask_ps (__m256 __A)
{
  return __builtin_ia32_movmskps256 ((__v8sf)__A);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_undefined_pd (void)
{
  __m256d __Y = __Y;
  return __Y;
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_undefined_ps (void)
{
  __m256 __Y = __Y;
  return __Y;
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_undefined_si256 (void)
{
  __m256i __Y = __Y;
  return __Y;
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setzero_pd (void)
{
  return __extension__ (__m256d){ 0.0, 0.0, 0.0, 0.0 };
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setzero_ps (void)
{
  return __extension__ (__m256){ 0.0, 0.0, 0.0, 0.0,
				 0.0, 0.0, 0.0, 0.0 };
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setzero_si256 (void)
{
  return __extension__ (__m256i)(__v4di){ 0, 0, 0, 0 };
}

/* Create the vector [A B C D].  */
extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_pd (double __A, double __B, double __C, double __D)
{
  return __extension__ (__m256d){ __D, __C, __B, __A };
}

/* Create the vector [A B C D E F G H].  */
extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_ps (float __A, float __B, float __C, float __D,
	       float __E, float __F, float __G, float __H)
{
  return __extension__ (__m256){ __H, __G, __F, __E,
				 __D, __C, __B, __A };
}

/* Create the vector [A B C D E F G H].  */
extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_epi32 (int __A, int __B, int __C, int __D,
		  int __E, int __F, int __G, int __H)
{
  return __extension__ (__m256i)(__v8si){ __H, __G, __F, __E,
					  __D, __C, __B, __A };
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_epi16 (short __q15, short __q14, short __q13, short __q12,
		  short __q11, short __q10, short __q09, short __q08,
		  short __q07, short __q06, short __q05, short __q04,
		  short __q03, short __q02, short __q01, short __q00)
{
  return __extension__ (__m256i)(__v16hi){
    __q00, __q01, __q02, __q03, __q04, __q05, __q06, __q07,
    __q08, __q09, __q10, __q11, __q12, __q13, __q14, __q15
  };
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_epi8  (char __q31, char __q30, char __q29, char __q28,
		  char __q27, char __q26, char __q25, char __q24,
		  char __q23, char __q22, char __q21, char __q20,
		  char __q19, char __q18, char __q17, char __q16,
		  char __q15, char __q14, char __q13, char __q12,
		  char __q11, char __q10, char __q09, char __q08,
		  char __q07, char __q06, char __q05, char __q04,
		  char __q03, char __q02, char __q01, char __q00)
{
  return __extension__ (__m256i)(__v32qi){
    __q00, __q01, __q02, __q03, __q04, __q05, __q06, __q07,
    __q08, __q09, __q10, __q11, __q12, __q13, __q14, __q15,
    __q16, __q17, __q18, __q19, __q20, __q21, __q22, __q23,
    __q24, __q25, __q26, __q27, __q28, __q29, __q30, __q31
  };
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_epi64x (long long __A, long long __B, long long __C,
		   long long __D)
{
  return __extension__ (__m256i)(__v4di){ __D, __C, __B, __A };
}

/* Create a vector with all elements equal to A.  */
extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set1_pd (double __A)
{
  return __extension__ (__m256d){ __A, __A, __A, __A };
}

/* Create a vector with all elements equal to A.  */
extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set1_ps (float __A)
{
  return __extension__ (__m256){ __A, __A, __A, __A,
				 __A, __A, __A, __A };
}

/* Create a vector with all elements equal to A.  */
extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set1_epi32 (int __A)
{
  return __extension__ (__m256i)(__v8si){ __A, __A, __A, __A,
					  __A, __A, __A, __A };
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set1_epi16 (short __A)
{
  return _mm256_set_epi16 (__A, __A, __A, __A, __A, __A, __A, __A,
			   __A, __A, __A, __A, __A, __A, __A, __A);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set1_epi8 (char __A)
{
  return _mm256_set_epi8 (__A, __A, __A, __A, __A, __A, __A, __A,
			  __A, __A, __A, __A, __A, __A, __A, __A,
			  __A, __A, __A, __A, __A, __A, __A, __A,
			  __A, __A, __A, __A, __A, __A, __A, __A);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set1_epi64x (long long __A)
{
  return __extension__ (__m256i)(__v4di){ __A, __A, __A, __A };
}

/* Create vectors of elements in the reversed order from the
   _mm256_set_XXX functions.  */

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_pd (double __A, double __B, double __C, double __D)
{
  return _mm256_set_pd (__D, __C, __B, __A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_ps (float __A, float __B, float __C, float __D,
		float __E, float __F, float __G, float __H)
{
  return _mm256_set_ps (__H, __G, __F, __E, __D, __C, __B, __A);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_epi32 (int __A, int __B, int __C, int __D,
		   int __E, int __F, int __G, int __H)
{
  return _mm256_set_epi32 (__H, __G, __F, __E, __D, __C, __B, __A);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_epi16 (short __q15, short __q14, short __q13, short __q12,
		   short __q11, short __q10, short __q09, short __q08,
		   short __q07, short __q06, short __q05, short __q04,
		   short __q03, short __q02, short __q01, short __q00)
{
  return _mm256_set_epi16 (__q00, __q01, __q02, __q03,
			   __q04, __q05, __q06, __q07,
			   __q08, __q09, __q10, __q11,
			   __q12, __q13, __q14, __q15);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_epi8  (char __q31, char __q30, char __q29, char __q28,
		   char __q27, char __q26, char __q25, char __q24,
		   char __q23, char __q22, char __q21, char __q20,
		   char __q19, char __q18, char __q17, char __q16,
		   char __q15, char __q14, char __q13, char __q12,
		   char __q11, char __q10, char __q09, char __q08,
		   char __q07, char __q06, char __q05, char __q04,
		   char __q03, char __q02, char __q01, char __q00)
{
  return _mm256_set_epi8 (__q00, __q01, __q02, __q03,
			  __q04, __q05, __q06, __q07,
			  __q08, __q09, __q10, __q11,
			  __q12, __q13, __q14, __q15,
			  __q16, __q17, __q18, __q19,
			  __q20, __q21, __q22, __q23,
			  __q24, __q25, __q26, __q27,
			  __q28, __q29, __q30, __q31);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_epi64x (long long __A, long long __B, long long __C,
		    long long __D)
{
  return _mm256_set_epi64x (__D, __C, __B, __A);
}

/* Casts between various SP, DP, INT vector types.  Note that these do no
   conversion of values, they just change the type.  */
extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castpd_ps (__m256d __A)
{
  return (__m256) __A;
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castpd_si256 (__m256d __A)
{
  return (__m256i) __A;
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castps_pd (__m256 __A)
{
  return (__m256d) __A;
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castps_si256(__m256 __A)
{
  return (__m256i) __A;
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castsi256_ps (__m256i __A)
{
  return (__m256) __A;
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castsi256_pd (__m256i __A)
{
  return (__m256d) __A;
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castpd256_pd128 (__m256d __A)
{
  return (__m128d) __builtin_ia32_pd_pd256 ((__v4df)__A);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castps256_ps128 (__m256 __A)
{
  return (__m128) __builtin_ia32_ps_ps256 ((__v8sf)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castsi256_si128 (__m256i __A)
{
  return (__m128i) __builtin_ia32_si_si256 ((__v8si)__A);
}

/* When cast is done from a 128 to 256-bit type, the low 128 bits of
   the 256-bit result contain source parameter value and the upper 128
   bits of the result are undefined.  Those intrinsics shouldn't
   generate any extra moves.  */

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castpd128_pd256 (__m128d __A)
{
  return (__m256d) __builtin_ia32_pd256_pd ((__v2df)__A);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castps128_ps256 (__m128 __A)
{
  return (__m256) __builtin_ia32_ps256_ps ((__v4sf)__A);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_castsi128_si256 (__m128i __A)
{
  return (__m256i) __builtin_ia32_si256_si ((__v4si)__A);
}

/* Similarly, but with zero extension instead of undefined values.  */

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_zextpd128_pd256 (__m128d __A)
{
  return _mm256_insertf128_pd (_mm256_setzero_pd (), __A, 0);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_zextps128_ps256 (__m128 __A)
{
  return _mm256_insertf128_ps (_mm256_setzero_ps (), __A, 0);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_zextsi128_si256 (__m128i __A)
{
  return _mm256_insertf128_si256 (_mm256_setzero_si256 (), __A, 0);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_m128 ( __m128 __H, __m128 __L)
{
  return _mm256_insertf128_ps (_mm256_castps128_ps256 (__L), __H, 1);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_m128d (__m128d __H, __m128d __L)
{
  return _mm256_insertf128_pd (_mm256_castpd128_pd256 (__L), __H, 1);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set_m128i (__m128i __H, __m128i __L)
{
  return _mm256_insertf128_si256 (_mm256_castsi128_si256 (__L), __H, 1);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_m128 (__m128 __L, __m128 __H)
{
  return _mm256_set_m128 (__H, __L);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_m128d (__m128d __L, __m128d __H)
{
  return _mm256_set_m128d (__H, __L);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setr_m128i (__m128i __L, __m128i __H)
{
  return _mm256_set_m128i (__H, __L);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_loadu2_m128 (float const *__PH, float const *__PL)
{
  return _mm256_insertf128_ps (_mm256_castps128_ps256 (_mm_loadu_ps (__PL)),
			       _mm_loadu_ps (__PH), 1);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu2_m128 (float *__PH, float *__PL, __m256 __A)
{
  _mm_storeu_ps (__PL, _mm256_castps256_ps128 (__A));
  _mm_storeu_ps (__PH, _mm256_extractf128_ps (__A, 1));
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_loadu2_m128d (double const *__PH, double const *__PL)
{
  return _mm256_insertf128_pd (_mm256_castpd128_pd256 (_mm_loadu_pd (__PL)),
			       _mm_loadu_pd (__PH), 1);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu2_m128d (double *__PH, double *__PL, __m256d __A)
{
  _mm_storeu_pd (__PL, _mm256_castpd256_pd128 (__A));
  _mm_storeu_pd (__PH, _mm256_extractf128_pd (__A, 1));
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_loadu2_m128i (__m128i_u const *__PH, __m128i_u const *__PL)
{
  return _mm256_insertf128_si256 (_mm256_castsi128_si256 (_mm_loadu_si128 (__PL)),
				  _mm_loadu_si128 (__PH), 1);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_storeu2_m128i (__m128i_u *__PH, __m128i_u *__PL, __m256i __A)
{
  _mm_storeu_si128 (__PL, _mm256_castsi256_si128 (__A));
  _mm_storeu_si128 (__PH, _mm256_extractf128_si256 (__A, 1));
}

#ifdef __DISABLE_AVX__
#undef __DISABLE_AVX__
#pragma GCC pop_options
#endif /* __DISABLE_AVX__ */

#endif /* _AVXINTRIN_H_INCLUDED */
