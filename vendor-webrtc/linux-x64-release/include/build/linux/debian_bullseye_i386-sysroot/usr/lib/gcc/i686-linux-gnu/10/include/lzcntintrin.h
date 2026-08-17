/* Copyright (C) 2009-2020 Free Software Foundation, Inc.

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

#if !defined _X86INTRIN_H_INCLUDED && !defined _IMMINTRIN_H_INCLUDED
# error "Never use <lzcntintrin.h> directly; include <x86intrin.h> instead."
#endif


#ifndef _LZCNTINTRIN_H_INCLUDED
#define _LZCNTINTRIN_H_INCLUDED

#ifndef __LZCNT__
#pragma GCC push_options
#pragma GCC target("lzcnt")
#define __DISABLE_LZCNT__
#endif /* __LZCNT__ */

extern __inline unsigned short __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__lzcnt16 (unsigned short __X)
{
  return __builtin_ia32_lzcnt_u16 (__X);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__lzcnt32 (unsigned int __X)
{
  return __builtin_ia32_lzcnt_u32 (__X);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_lzcnt_u32 (unsigned int __X)
{
  return __builtin_ia32_lzcnt_u32 (__X);
}

#ifdef __x86_64__
extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__lzcnt64 (unsigned long long __X)
{
  return __builtin_ia32_lzcnt_u64 (__X);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_lzcnt_u64 (unsigned long long __X)
{
  return __builtin_ia32_lzcnt_u64 (__X);
}
#endif

#ifdef __DISABLE_LZCNT__
#undef __DISABLE_LZCNT__
#pragma GCC pop_options
#endif /* __DISABLE_LZCNT__ */

#endif /* _LZCNTINTRIN_H_INCLUDED */
