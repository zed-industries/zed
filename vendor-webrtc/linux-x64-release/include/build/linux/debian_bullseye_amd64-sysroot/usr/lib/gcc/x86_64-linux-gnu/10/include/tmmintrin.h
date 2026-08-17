/* Copyright (C) 2006-2020 Free Software Foundation, Inc.

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
   User Guide and Reference, version 9.1.  */

#ifndef _TMMINTRIN_H_INCLUDED
#define _TMMINTRIN_H_INCLUDED

/* We need definitions from the SSE3, SSE2 and SSE header files*/
#include <pmmintrin.h>

#ifndef __SSSE3__
#pragma GCC push_options
#pragma GCC target("ssse3")
#define __DISABLE_SSSE3__
#endif /* __SSSE3__ */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hadd_epi16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_phaddw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hadd_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_phaddd128 ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hadds_epi16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_phaddsw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hadd_pi16 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_phaddw ((__v4hi)__X, (__v4hi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hadd_pi32 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_phaddd ((__v2si)__X, (__v2si)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hadds_pi16 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_phaddsw ((__v4hi)__X, (__v4hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsub_epi16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_phsubw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsub_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_phsubd128 ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsubs_epi16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_phsubsw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsub_pi16 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_phsubw ((__v4hi)__X, (__v4hi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsub_pi32 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_phsubd ((__v2si)__X, (__v2si)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsubs_pi16 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_phsubsw ((__v4hi)__X, (__v4hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maddubs_epi16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmaddubsw128 ((__v16qi)__X, (__v16qi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maddubs_pi16 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_pmaddubsw ((__v8qi)__X, (__v8qi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mulhrs_epi16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pmulhrsw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mulhrs_pi16 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_pmulhrsw ((__v4hi)__X, (__v4hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shuffle_epi8 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_pshufb128 ((__v16qi)__X, (__v16qi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shuffle_pi8 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_pshufb ((__v8qi)__X, (__v8qi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sign_epi8 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psignb128 ((__v16qi)__X, (__v16qi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sign_epi16 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psignw128 ((__v8hi)__X, (__v8hi)__Y);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sign_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psignd128 ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sign_pi8 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_psignb ((__v8qi)__X, (__v8qi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sign_pi16 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_psignw ((__v4hi)__X, (__v4hi)__Y);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sign_pi32 (__m64 __X, __m64 __Y)
{
  return (__m64) __builtin_ia32_psignd ((__v2si)__X, (__v2si)__Y);
}

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_alignr_epi8(__m128i __X, __m128i __Y, const int __N)
{
  return (__m128i) __builtin_ia32_palignr128 ((__v2di)__X,
					      (__v2di)__Y, __N * 8);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_alignr_pi8(__m64 __X, __m64 __Y, const int __N)
{
  return (__m64) __builtin_ia32_palignr ((__v1di)__X,
					 (__v1di)__Y, __N * 8);
}
#else
#define _mm_alignr_epi8(X, Y, N)					\
  ((__m128i) __builtin_ia32_palignr128 ((__v2di)(__m128i)(X),		\
					(__v2di)(__m128i)(Y),		\
					(int)(N) * 8))
#define _mm_alignr_pi8(X, Y, N)						\
  ((__m64) __builtin_ia32_palignr ((__v1di)(__m64)(X),			\
				   (__v1di)(__m64)(Y),			\
				   (int)(N) * 8))
#endif

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_abs_epi8 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pabsb128 ((__v16qi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_abs_epi16 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pabsw128 ((__v8hi)__X);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_abs_epi32 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pabsd128 ((__v4si)__X);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_abs_pi8 (__m64 __X)
{
  return (__m64) __builtin_ia32_pabsb ((__v8qi)__X);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_abs_pi16 (__m64 __X)
{
  return (__m64) __builtin_ia32_pabsw ((__v4hi)__X);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_abs_pi32 (__m64 __X)
{
  return (__m64) __builtin_ia32_pabsd ((__v2si)__X);
}

#ifdef __DISABLE_SSSE3__
#undef __DISABLE_SSSE3__
#pragma GCC pop_options
#endif /* __DISABLE_SSSE3__ */

#endif /* _TMMINTRIN_H_INCLUDED */
