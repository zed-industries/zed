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

#ifndef _MWAITXINTRIN_H_INCLUDED
#define _MWAITXINTRIN_H_INCLUDED

#ifndef __MWAITX__
#pragma GCC push_options
#pragma GCC target("mwaitx")
#define __DISABLE_MWAITX__
#endif /* __MWAITX__ */

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_monitorx (void const * __P, unsigned int __E, unsigned int __H)
{
  __builtin_ia32_monitorx (__P, __E, __H);
}

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_mwaitx (unsigned int __E, unsigned int __H, unsigned int __C)
{
  __builtin_ia32_mwaitx (__E, __H, __C);
}

#ifdef __DISABLE_MWAITX__
#undef __DISABLE_MWAITX__
#pragma GCC pop_options
#endif /* __DISABLE_MWAITX__ */

#endif /* _MWAITXINTRIN_H_INCLUDED */
