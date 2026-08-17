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

#ifndef _IMMINTRIN_H_INCLUDED
# error "Never use <rtmintrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _RTMINTRIN_H_INCLUDED
#define _RTMINTRIN_H_INCLUDED

#ifndef __RTM__
#pragma GCC push_options
#pragma GCC target("rtm")
#define __DISABLE_RTM__
#endif /* __RTM__ */

#define _XBEGIN_STARTED		(~0u)
#define _XABORT_EXPLICIT	(1 << 0)
#define _XABORT_RETRY		(1 << 1)
#define _XABORT_CONFLICT	(1 << 2)
#define _XABORT_CAPACITY	(1 << 3)
#define _XABORT_DEBUG		(1 << 4)
#define _XABORT_NESTED		(1 << 5)
#define _XABORT_CODE(x)		(((x) >> 24) & 0xFF)

/* Start an RTM code region.  Return _XBEGIN_STARTED on success and the
   abort condition otherwise.  */
extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xbegin (void)
{
  return __builtin_ia32_xbegin ();
}

/* Specify the end of an RTM code region.  If it corresponds to the
   outermost transaction, then attempts the transaction commit.  If the
   commit fails, then control is transferred to the outermost transaction
   fallback handler.  */
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xend (void)
{
  __builtin_ia32_xend ();
}

/* Force an RTM abort condition. The control is transferred to the
   outermost transaction fallback handler with the abort condition IMM.  */
#ifdef __OPTIMIZE__
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_xabort (const unsigned int __imm)
{
  __builtin_ia32_xabort (__imm);
}
#else
#define _xabort(N)  __builtin_ia32_xabort (N)
#endif /* __OPTIMIZE__ */

#ifdef __DISABLE_RTM__
#undef __DISABLE_RTM__
#pragma GCC pop_options
#endif /* __DISABLE_RTM__ */

#endif /* _RTMINTRIN_H_INCLUDED */
