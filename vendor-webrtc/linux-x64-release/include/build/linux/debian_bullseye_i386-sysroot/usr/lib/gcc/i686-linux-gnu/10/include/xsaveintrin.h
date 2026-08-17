/* Copyright (C) 2012-2020 Free Software Foundation, Inc.

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

#if !defined _IMMINTRIN_H_INCLUDED
# error "Never use <xsaveintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _XSAVEINTRIN_H_INCLUDED
#define _XSAVEINTRIN_H_INCLUDED

#ifndef __XSAVE__
#pragma GCC push_options
#pragma GCC target("xsave")
#define __DISABLE_XSAVE__
#endif /* __XSAVE__ */

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xsave (void *__P, long long __M)
{
  __builtin_ia32_xsave (__P, __M);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xrstor (void *__P, long long __M)
{
  __builtin_ia32_xrstor (__P, __M);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xsetbv (unsigned int __A, long long __V)
{
  __builtin_ia32_xsetbv (__A, __V);
}

extern __inline long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xgetbv (unsigned int __A)
{
  return __builtin_ia32_xgetbv (__A);
}

#ifdef __x86_64__
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xsave64 (void *__P, long long __M)
{
  __builtin_ia32_xsave64 (__P, __M);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xrstor64 (void *__P, long long __M)
{
  __builtin_ia32_xrstor64 (__P, __M);
}
#endif

#ifdef __DISABLE_XSAVE__
#undef __DISABLE_XSAVE__
#pragma GCC pop_options
#endif /* __DISABLE_XSAVE__ */

#endif /* _XSAVEINTRIN_H_INCLUDED */
