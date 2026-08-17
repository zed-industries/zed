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
# error "Never use <xsavesintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _XSAVESINTRIN_H_INCLUDED
#define _XSAVESINTRIN_H_INCLUDED

#ifndef __XSAVES__
#pragma GCC push_options
#pragma GCC target("xsaves")
#define __DISABLE_XSAVES__
#endif /* __XSAVES__ */

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xsaves (void *__P, long long __M)
{
  __builtin_ia32_xsaves (__P, __M);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xrstors (void *__P, long long __M)
{
  __builtin_ia32_xrstors (__P, __M);
}

#ifdef __x86_64__
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xrstors64 (void *__P, long long __M)
{
  __builtin_ia32_xrstors64 (__P, __M);
}

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xsaves64 (void *__P, long long __M)
{
  __builtin_ia32_xsaves64 (__P, __M);
}
#endif

#ifdef __DISABLE_XSAVES__
#undef __DISABLE_XSAVES__
#pragma GCC pop_options
#endif /* __DISABLE_XSAVES__ */

#endif /* _XSAVESINTRIN_H_INCLUDED */
