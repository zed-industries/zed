/* Copyright (C) 2004-2020 Free Software Foundation, Inc.

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

/* Implemented from the mm3dnow.h (of supposedly AMD origin) included with
   MSVC 7.1.  */

#ifndef _MM3DNOW_H_INCLUDED
#define _MM3DNOW_H_INCLUDED

#include <mmintrin.h>
#include <prfchwintrin.h>

#if defined __x86_64__ && !defined __SSE__ || !defined __3dNOW__
#pragma GCC push_options
#ifdef __x86_64__
#pragma GCC target("sse,3dnow")
#else
#pragma GCC target("3dnow")
#endif
#define __DISABLE_3dNOW__
#endif /* __3dNOW__ */

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_femms (void)
{
  __builtin_ia32_femms();
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pavgusb (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pavgusb ((__v8qi)__A, (__v8qi)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pf2id (__m64 __A)
{
  return (__m64)__builtin_ia32_pf2id ((__v2sf)__A);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfacc (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfacc ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfadd (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfadd ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfcmpeq (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfcmpeq ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfcmpge (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfcmpge ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfcmpgt (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfcmpgt ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfmax (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfmax ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfmin (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfmin ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfmul (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfmul ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfrcp (__m64 __A)
{
  return (__m64)__builtin_ia32_pfrcp ((__v2sf)__A);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfrcpit1 (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfrcpit1 ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfrcpit2 (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfrcpit2 ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfrsqrt (__m64 __A)
{
  return (__m64)__builtin_ia32_pfrsqrt ((__v2sf)__A);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfrsqit1 (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfrsqit1 ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfsub (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfsub ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfsubr (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfsubr ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pi2fd (__m64 __A)
{
  return (__m64)__builtin_ia32_pi2fd ((__v2si)__A);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pmulhrw (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pmulhrw ((__v4hi)__A, (__v4hi)__B);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_prefetch (void *__P)
{
  __builtin_prefetch (__P, 0, 3 /* _MM_HINT_T0 */);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_from_float (float __A)
{
  return __extension__ (__m64)(__v2sf){ __A, 0.0f };
}

extern __inline float __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_to_float (__m64 __A)
{
  union { __v2sf v; float a[2]; } __tmp;
  __tmp.v = (__v2sf)__A;
  return __tmp.a[0];
}

#ifdef __DISABLE_3dNOW__
#undef __DISABLE_3dNOW__
#pragma GCC pop_options
#endif /* __DISABLE_3dNOW__ */

#if defined __x86_64__ && !defined __SSE__ || !defined __3dNOW_A__
#pragma GCC push_options
#ifdef __x86_64__
#pragma GCC target("sse,3dnowa")
#else
#pragma GCC target("3dnowa")
#endif
#define __DISABLE_3dNOW_A__
#endif /* __3dNOW_A__ */

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pf2iw (__m64 __A)
{
  return (__m64)__builtin_ia32_pf2iw ((__v2sf)__A);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfnacc (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfnacc ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pfpnacc (__m64 __A, __m64 __B)
{
  return (__m64)__builtin_ia32_pfpnacc ((__v2sf)__A, (__v2sf)__B);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pi2fw (__m64 __A)
{
  return (__m64)__builtin_ia32_pi2fw ((__v2si)__A);
}

extern __inline __m64 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_pswapd (__m64 __A)
{
  return (__m64)__builtin_ia32_pswapdsf ((__v2sf)__A);
}

#ifdef __DISABLE_3dNOW_A__
#undef __DISABLE_3dNOW_A__
#pragma GCC pop_options
#endif /* __DISABLE_3dNOW_A__ */

#endif /* _MM3DNOW_H_INCLUDED */
