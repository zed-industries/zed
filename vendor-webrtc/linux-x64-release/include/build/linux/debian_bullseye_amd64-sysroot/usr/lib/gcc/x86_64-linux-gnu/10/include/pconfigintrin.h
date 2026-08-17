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
#error "Never use <pconfigintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _PCONFIGINTRIN_H_INCLUDED
#define _PCONFIGINTRIN_H_INCLUDED

#ifndef __PCONFIG__
#pragma GCC push_options
#pragma GCC target("pconfig")
#define __DISABLE_PCONFIG__
#endif /* __PCONFIG__ */

#define __pconfig_b(leaf, b, retval)			\
  __asm__ __volatile__ ("pconfig\n\t"			\
	: "=a" (retval)					\
	: "a" (leaf), "b" (b)				\
	: "cc")

#define __pconfig_generic(leaf, b, c, d, retval)	\
  __asm__ __volatile__ ("pconfig\n\t"			\
	: "=a" (retval), "=b" (b), "=c" (c), "=d" (d)	\
	: "a" (leaf), "b" (b), "c" (c), "d" (d)		\
	: "cc")

extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_pconfig_u32 (const unsigned int __L, size_t __D[])
{
  enum __pconfig_type
  {
    __PCONFIG_KEY_PROGRAM = 0x01,
  };

  unsigned int __R = 0;

  if (!__builtin_constant_p (__L))
    __pconfig_generic (__L, __D[0], __D[1], __D[2], __R);
  else switch (__L)
    {
    case __PCONFIG_KEY_PROGRAM:
      __pconfig_b (__L, __D[0], __R);
      break;
    default:
      __pconfig_generic (__L, __D[0], __D[1], __D[2], __R);
    }
  return __R;
}

#ifdef __DISABLE_PCONFIG__
#undef __DISABLE_PCONFIG__
#pragma GCC pop_options
#endif /* __DISABLE_PCONFIG__ */

#endif /* _PCONFIGINTRIN_H_INCLUDED */
