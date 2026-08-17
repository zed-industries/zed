/* Copyright (C) 2007-2020 Free Software Foundation, Inc.

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

#ifndef _X86INTRIN_H_INCLUDED
# error "Never use <lwpintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _LWPINTRIN_H_INCLUDED
#define _LWPINTRIN_H_INCLUDED

#ifndef __LWP__
#pragma GCC push_options
#pragma GCC target("lwp")
#define __DISABLE_LWP__
#endif /* __LWP__ */

extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__llwpcb (void *__pcbAddress)
{
  __builtin_ia32_llwpcb (__pcbAddress);
}

extern __inline void * __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__slwpcb (void)
{
  return __builtin_ia32_slwpcb ();
}

#ifdef __OPTIMIZE__
extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__lwpval32 (unsigned int __data2, unsigned int __data1, unsigned int __flags)
{
  __builtin_ia32_lwpval32 (__data2, __data1, __flags);
}

#ifdef __x86_64__
extern __inline void __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__lwpval64 (unsigned long long __data2, unsigned int __data1,
	    unsigned int __flags)
{
  __builtin_ia32_lwpval64 (__data2, __data1, __flags);
}
#endif
#else
#define __lwpval32(D2, D1, F) \
  (__builtin_ia32_lwpval32 ((unsigned int) (D2), (unsigned int) (D1), \
			    (unsigned int) (F)))
#ifdef __x86_64__
#define __lwpval64(D2, D1, F) \
  (__builtin_ia32_lwpval64 ((unsigned long long) (D2), (unsigned int) (D1), \
			    (unsigned int) (F)))
#endif
#endif


#ifdef __OPTIMIZE__
extern __inline unsigned char __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__lwpins32 (unsigned int __data2, unsigned int __data1, unsigned int __flags)
{
  return __builtin_ia32_lwpins32 (__data2, __data1, __flags);
}

#ifdef __x86_64__
extern __inline unsigned char __attribute__((__gnu_inline__, __always_inline__, __artificial__))
__lwpins64 (unsigned long long __data2, unsigned int __data1,
	    unsigned int __flags)
{
  return __builtin_ia32_lwpins64 (__data2, __data1, __flags);
}
#endif
#else
#define __lwpins32(D2, D1, F) \
  (__builtin_ia32_lwpins32 ((unsigned int) (D2), (unsigned int) (D1), \
			    (unsigned int) (F)))
#ifdef __x86_64__
#define __lwpins64(D2, D1, F) \
  (__builtin_ia32_lwpins64 ((unsigned long long) (D2), (unsigned int) (D1), \
			    (unsigned int) (F)))
#endif
#endif

#ifdef __DISABLE_LWP__
#undef __DISABLE_LWP__
#pragma GCC pop_options
#endif /* __DISABLE_LWP__ */

#endif /* _LWPINTRIN_H_INCLUDED */
