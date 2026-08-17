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

#if !defined _IMMINTRIN_H_INCLUDED && !defined _MM3DNOW_H_INCLUDED
# error "Never use <prfchwintrin.h> directly; include <immintrin.h> or <mm3dnow.h> instead."
#endif

#ifndef _PRFCHWINTRIN_H_INCLUDED
#define _PRFCHWINTRIN_H_INCLUDED

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_m_prefetchw (void *__P)
{
  __builtin_prefetch (__P, 1, 3 /* _MM_HINT_T0 */);
}

#endif /* _PRFCHWINTRIN_H_INCLUDED */
