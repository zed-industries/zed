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

#if !defined _IMMINTRIN_H_INCLUDED
# error "Never use <xsavecintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _XSAVECINTRIN_H_INCLUDED
#define _XSAVECINTRIN_H_INCLUDED

#ifndef __XSAVEC__
#pragma GCC push_options
#pragma GCC target("xsavec")
#define __DISABLE_XSAVEC__
#endif /* __XSAVEC__ */

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xsavec (void *__P, long long __M)
{
  __builtin_ia32_xsavec (__P, __M);
}

#ifdef __x86_64__
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xsavec64 (void *__P, long long __M)
{
  __builtin_ia32_xsavec64 (__P, __M);
}
#endif

#ifdef __DISABLE_XSAVEC__
#undef __DISABLE_XSAVEC__
#pragma GCC pop_options
#endif /* __DISABLE_XSAVEC__ */

#endif /* _XSAVECINTRIN_H_INCLUDED */
