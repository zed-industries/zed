/* Copyright (C) 2015-2020 Free Software Foundation, Inc.

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
# error "Never use <cetintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _CETINTRIN_H_INCLUDED
#define _CETINTRIN_H_INCLUDED

#ifndef __SHSTK__
#pragma GCC push_options
#pragma GCC target ("shstk")
#define __DISABLE_SHSTK__
#endif /* __SHSTK__ */

#ifdef __x86_64__
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_get_ssp (void)
{
  return __builtin_ia32_rdsspq ();
}
#else
extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_get_ssp (void)
{
  return __builtin_ia32_rdsspd ();
}
#endif

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_inc_ssp (unsigned int __B)
{
#ifdef __x86_64__
  __builtin_ia32_incsspq ((unsigned long long) __B);
#else
  __builtin_ia32_incsspd (__B);
#endif
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_saveprevssp (void)
{
  __builtin_ia32_saveprevssp ();
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_rstorssp (void *__B)
{
  __builtin_ia32_rstorssp (__B);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_wrssd (unsigned int __B, void *__C)
{
  __builtin_ia32_wrssd (__B, __C);
}

#ifdef __x86_64__
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_wrssq (unsigned long long __B, void *__C)
{
  __builtin_ia32_wrssq (__B, __C);
}
#endif

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_wrussd (unsigned int __B, void *__C)
{
  __builtin_ia32_wrussd (__B, __C);
}

#ifdef __x86_64__
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_wrussq (unsigned long long __B, void *__C)
{
  __builtin_ia32_wrussq (__B, __C);
}
#endif

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_setssbsy (void)
{
  __builtin_ia32_setssbsy ();
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_clrssbsy (void *__B)
{
  __builtin_ia32_clrssbsy (__B);
}

#ifdef __DISABLE_SHSTK__
#undef __DISABLE_SHSTK__
#pragma GCC pop_options
#endif /* __DISABLE_SHSTK__ */

#endif /* _CETINTRIN_H_INCLUDED.  */
