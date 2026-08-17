/* Copyright (C) 2018-2020 Free Software Foundation, Inc.

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
#error "Never use <wbnoinvdintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _WBNOINVDINTRIN_H_INCLUDED
#define _WBNOINVDINTRIN_H_INCLUDED

#ifndef __WBNOINVD__
#pragma GCC push_options
#pragma GCC target("wbnoinvd")
#define __DISABLE_WBNOINVD__
#endif /* __WBNOINVD__ */

extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_wbnoinvd (void)
{
  __builtin_ia32_wbnoinvd ();
}

#ifdef __DISABLE_WBNOINVD__
#undef __DISABLE_WBNOINVD__
#pragma GCC pop_options
#endif /* __DISABLE_WBNOINVD__ */

#endif /* _WBNOINVDINTRIN_H_INCLUDED */
