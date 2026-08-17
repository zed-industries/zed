/* Copyright (C) 2010-2020 Free Software Foundation, Inc.

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
# error "Never use <bmiintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _BMIINTRIN_H_INCLUDED
#define _BMIINTRIN_H_INCLUDED

#ifndef __BMI__
#pragma GCC push_options
#pragma GCC target("bmi")
#define __DISABLE_BMI__
#endif /* __BMI__ */

extern __inline unsigned short __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__tzcnt_u16 (unsigned short __X)
{
  return __builtin_ia32_tzcnt_u16 (__X);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__andn_u32 (unsigned int __X, unsigned int __Y)
{
  return ~__X & __Y;
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bextr_u32 (unsigned int __X, unsigned int __Y)
{
  return __builtin_ia32_bextr_u32 (__X, __Y);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_bextr_u32 (unsigned int __X, unsigned int __Y, unsigned __Z)
{
  return __builtin_ia32_bextr_u32 (__X, ((__Y & 0xff) | ((__Z & 0xff) << 8)));
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__blsi_u32 (unsigned int __X)
{
  return __X & -__X;
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_blsi_u32 (unsigned int __X)
{
  return __blsi_u32 (__X);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__blsmsk_u32 (unsigned int __X)
{
  return __X ^ (__X - 1);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_blsmsk_u32 (unsigned int __X)
{
  return __blsmsk_u32 (__X);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__blsr_u32 (unsigned int __X)
{
  return __X & (__X - 1);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_blsr_u32 (unsigned int __X)
{
  return __blsr_u32 (__X);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__tzcnt_u32 (unsigned int __X)
{
  return __builtin_ia32_tzcnt_u32 (__X);
}

extern __inline unsigned int __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_tzcnt_u32 (unsigned int __X)
{
  return __builtin_ia32_tzcnt_u32 (__X);
}


#ifdef  __x86_64__
extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__andn_u64 (unsigned long long __X, unsigned long long __Y)
{
  return ~__X & __Y;
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bextr_u64 (unsigned long long __X, unsigned long long __Y)
{
  return __builtin_ia32_bextr_u64 (__X, __Y);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_bextr_u64 (unsigned long long __X, unsigned int __Y, unsigned int __Z)
{
  return __builtin_ia32_bextr_u64 (__X, ((__Y & 0xff) | ((__Z & 0xff) << 8)));
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__blsi_u64 (unsigned long long __X)
{
  return __X & -__X;
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_blsi_u64 (unsigned long long __X)
{
  return __blsi_u64 (__X);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__blsmsk_u64 (unsigned long long __X)
{
  return __X ^ (__X - 1);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_blsmsk_u64 (unsigned long long __X)
{
  return __blsmsk_u64 (__X);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__blsr_u64 (unsigned long long __X)
{
  return __X & (__X - 1);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_blsr_u64 (unsigned long long __X)
{
  return __blsr_u64 (__X);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__tzcnt_u64 (unsigned long long __X)
{
  return __builtin_ia32_tzcnt_u64 (__X);
}

extern __inline unsigned long long __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_tzcnt_u64 (unsigned long long __X)
{
  return __builtin_ia32_tzcnt_u64 (__X);
}

#endif /* __x86_64__  */

#ifdef __DISABLE_BMI__
#undef __DISABLE_BMI__
#pragma GCC pop_options
#endif /* __DISABLE_BMI__ */

#endif /* _BMIINTRIN_H_INCLUDED */
