/* Copyright (C) 2019-2020 Free Software Foundation, Inc.

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
# error "Never use <enqcmdintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _ENQCMDINTRIN_H_INCLUDED
#define _ENQCMDINTRIN_H_INCLUDED

#ifndef __ENQCMD__
#pragma GCC push_options
#pragma GCC target ("enqcmd")
#define __DISABLE_ENQCMD__
#endif /* __ENQCMD__ */

extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_enqcmd (void * __P, const void * __Q)
{
  return __builtin_ia32_enqcmd (__P, __Q);
}

extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_enqcmds (void * __P, const void * __Q)
{
  return __builtin_ia32_enqcmds (__P, __Q);
}

#ifdef __DISABLE_ENQCMD__
#undef __DISABLE_ENQCMD__
#pragma GCC pop_options
#endif /* __DISABLE_ENQCMD__ */
#endif /* _ENQCMDINTRIN_H_INCLUDED.  */
