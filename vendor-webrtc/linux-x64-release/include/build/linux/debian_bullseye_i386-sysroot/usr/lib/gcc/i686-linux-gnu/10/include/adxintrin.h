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
# error "Never use <adxintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _ADXINTRIN_H_INCLUDED
#define _ADXINTRIN_H_INCLUDED

extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_subborrow_u32 (unsigned char __CF, unsigned int __X,
		unsigned int __Y, unsigned int *__P)
{
  return __builtin_ia32_sbb_u32 (__CF, __X, __Y, __P);
}

extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_addcarry_u32 (unsigned char __CF, unsigned int __X,
	       unsigned int __Y, unsigned int *__P)
{
  return __builtin_ia32_addcarryx_u32 (__CF, __X, __Y, __P);
}

extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_addcarryx_u32 (unsigned char __CF, unsigned int __X,
		unsigned int __Y, unsigned int *__P)
{
  return __builtin_ia32_addcarryx_u32 (__CF, __X, __Y, __P);
}

#ifdef __x86_64__
extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_subborrow_u64 (unsigned char __CF, unsigned long long __X,
		unsigned long long __Y, unsigned long long *__P)
{
  return __builtin_ia32_sbb_u64 (__CF, __X, __Y, __P);
}

extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_addcarry_u64 (unsigned char __CF, unsigned long long __X,
	       unsigned long long __Y, unsigned long long *__P)
{
  return __builtin_ia32_addcarryx_u64 (__CF, __X, __Y, __P);
}

extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_addcarryx_u64 (unsigned char __CF, unsigned long long __X,
		unsigned long long __Y, unsigned long long *__P)
{
  return __builtin_ia32_addcarryx_u64 (__CF, __X, __Y, __P);
}
#endif

#endif /* _ADXINTRIN_H_INCLUDED */
