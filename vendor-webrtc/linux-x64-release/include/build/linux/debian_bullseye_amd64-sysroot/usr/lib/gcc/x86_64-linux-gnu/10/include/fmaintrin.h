/* Copyright (C) 2011-2020 Free Software Foundation, Inc.

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
# error "Never use <fmaintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _FMAINTRIN_H_INCLUDED
#define _FMAINTRIN_H_INCLUDED

#ifndef __FMA__
#pragma GCC push_options
#pragma GCC target("fma")
#define __DISABLE_FMA__
#endif /* __FMA__ */

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmadd_pd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfmaddpd ((__v2df)__A, (__v2df)__B,
                                           (__v2df)__C);
}

extern __inline __m256d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmadd_pd (__m256d __A, __m256d __B, __m256d __C)
{
  return (__m256d)__builtin_ia32_vfmaddpd256 ((__v4df)__A, (__v4df)__B,
                                              (__v4df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmadd_ps (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfmaddps ((__v4sf)__A, (__v4sf)__B,
                                          (__v4sf)__C);
}

extern __inline __m256
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmadd_ps (__m256 __A, __m256 __B, __m256 __C)
{
  return (__m256)__builtin_ia32_vfmaddps256 ((__v8sf)__A, (__v8sf)__B,
                                             (__v8sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmadd_sd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d) __builtin_ia32_vfmaddsd3 ((__v2df)__A, (__v2df)__B,
                                             (__v2df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmadd_ss (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128) __builtin_ia32_vfmaddss3 ((__v4sf)__A, (__v4sf)__B,
                                            (__v4sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsub_pd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfmsubpd ((__v2df)__A, (__v2df)__B,
                                           (__v2df)__C);
}

extern __inline __m256d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmsub_pd (__m256d __A, __m256d __B, __m256d __C)
{
  return (__m256d)__builtin_ia32_vfmsubpd256 ((__v4df)__A, (__v4df)__B,
                                              (__v4df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsub_ps (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfmsubps ((__v4sf)__A, (__v4sf)__B,
                                          (__v4sf)__C);
}

extern __inline __m256
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmsub_ps (__m256 __A, __m256 __B, __m256 __C)
{
  return (__m256)__builtin_ia32_vfmsubps256 ((__v8sf)__A, (__v8sf)__B,
                                             (__v8sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsub_sd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfmsubsd3 ((__v2df)__A, (__v2df)__B,
                                            (__v2df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsub_ss (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfmsubss3 ((__v4sf)__A, (__v4sf)__B,
                                           (__v4sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmadd_pd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfnmaddpd ((__v2df)__A, (__v2df)__B,
					    (__v2df)__C);
}

extern __inline __m256d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fnmadd_pd (__m256d __A, __m256d __B, __m256d __C)
{
  return (__m256d)__builtin_ia32_vfnmaddpd256 ((__v4df)__A, (__v4df)__B,
					       (__v4df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmadd_ps (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfnmaddps ((__v4sf)__A, (__v4sf)__B,
					   (__v4sf)__C);
}

extern __inline __m256
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fnmadd_ps (__m256 __A, __m256 __B, __m256 __C)
{
  return (__m256)__builtin_ia32_vfnmaddps256 ((__v8sf)__A, (__v8sf)__B,
					      (__v8sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmadd_sd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfnmaddsd3 ((__v2df)__A, (__v2df)__B,
					     (__v2df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmadd_ss (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfnmaddss3 ((__v4sf)__A, (__v4sf)__B,
					    (__v4sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmsub_pd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfnmsubpd ((__v2df)__A, (__v2df)__B,
					    (__v2df)__C);
}

extern __inline __m256d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fnmsub_pd (__m256d __A, __m256d __B, __m256d __C)
{
  return (__m256d)__builtin_ia32_vfnmsubpd256 ((__v4df)__A, (__v4df)__B,
					       (__v4df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmsub_ps (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfnmsubps ((__v4sf)__A, (__v4sf)__B,
					   (__v4sf)__C);
}

extern __inline __m256
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fnmsub_ps (__m256 __A, __m256 __B, __m256 __C)
{
  return (__m256)__builtin_ia32_vfnmsubps256 ((__v8sf)__A, (__v8sf)__B,
					      (__v8sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmsub_sd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfnmsubsd3 ((__v2df)__A, (__v2df)__B,
					     (__v2df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fnmsub_ss (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfnmsubss3 ((__v4sf)__A, (__v4sf)__B,
					    (__v4sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmaddsub_pd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfmaddsubpd ((__v2df)__A, (__v2df)__B,
                                              (__v2df)__C);
}

extern __inline __m256d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmaddsub_pd (__m256d __A, __m256d __B, __m256d __C)
{
  return (__m256d)__builtin_ia32_vfmaddsubpd256 ((__v4df)__A,
                                                 (__v4df)__B,
                                                 (__v4df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmaddsub_ps (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfmaddsubps ((__v4sf)__A, (__v4sf)__B,
                                             (__v4sf)__C);
}

extern __inline __m256
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmaddsub_ps (__m256 __A, __m256 __B, __m256 __C)
{
  return (__m256)__builtin_ia32_vfmaddsubps256 ((__v8sf)__A,
                                                (__v8sf)__B,
                                                (__v8sf)__C);
}

extern __inline __m128d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsubadd_pd (__m128d __A, __m128d __B, __m128d __C)
{
  return (__m128d)__builtin_ia32_vfmaddsubpd ((__v2df)__A, (__v2df)__B,
                                              -(__v2df)__C);
}

extern __inline __m256d
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmsubadd_pd (__m256d __A, __m256d __B, __m256d __C)
{
  return (__m256d)__builtin_ia32_vfmaddsubpd256 ((__v4df)__A,
                                                 (__v4df)__B,
                                                 -(__v4df)__C);
}

extern __inline __m128
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_fmsubadd_ps (__m128 __A, __m128 __B, __m128 __C)
{
  return (__m128)__builtin_ia32_vfmaddsubps ((__v4sf)__A, (__v4sf)__B,
                                             -(__v4sf)__C);
}

extern __inline __m256
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmsubadd_ps (__m256 __A, __m256 __B, __m256 __C)
{
  return (__m256)__builtin_ia32_vfmaddsubps256 ((__v8sf)__A,
                                                (__v8sf)__B,
                                                -(__v8sf)__C);
}

#ifdef __DISABLE_FMA__
#undef __DISABLE_FMA__
#pragma GCC pop_options
#endif /* __DISABLE_FMA__ */

#endif
