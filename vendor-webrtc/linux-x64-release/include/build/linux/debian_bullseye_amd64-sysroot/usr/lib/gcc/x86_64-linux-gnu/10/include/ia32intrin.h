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

#ifndef _X86INTRIN_H_INCLUDED
# error "Never use <ia32intrin.h> directly; include <x86intrin.h> instead."
#endif

/* 32bit bsf */
extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bsfd (int __X)
{
  return __builtin_ctz (__X);
}

/* 32bit bsr */
extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bsrd (int __X)
{
  return __builtin_ia32_bsrsi (__X);
}

/* 32bit bswap */
extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bswapd (int __X)
{
  return __builtin_bswap32 (__X);
}

#ifndef __iamcu__

#ifndef __SSE4_2__
#pragma GCC push_options
#pragma GCC target("sse4.2")
#define __DISABLE_SSE4_2__
#endif /* __SSE4_2__ */

/* 32bit accumulate CRC32 (polynomial 0x11EDC6F41) value.  */
extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__crc32b (unsigned int __C, unsigned char __V)
{
  return __builtin_ia32_crc32qi (__C, __V);
}

extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__crc32w (unsigned int __C, unsigned short __V)
{
  return __builtin_ia32_crc32hi (__C, __V);
}

extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__crc32d (unsigned int __C, unsigned int __V)
{
  return __builtin_ia32_crc32si (__C, __V);
}

#ifdef __DISABLE_SSE4_2__
#undef __DISABLE_SSE4_2__
#pragma GCC pop_options
#endif /* __DISABLE_SSE4_2__ */

#endif /* __iamcu__ */

/* 32bit popcnt */
extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__popcntd (unsigned int __X)
{
  return __builtin_popcount (__X);
}

#ifndef __iamcu__

/* rdpmc */
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rdpmc (int __S)
{
  return __builtin_ia32_rdpmc (__S);
}

#endif /* __iamcu__ */

/* rdtsc */
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rdtsc (void)
{
  return __builtin_ia32_rdtsc ();
}

#ifndef __iamcu__

/* rdtscp */
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rdtscp (unsigned int *__A)
{
  return __builtin_ia32_rdtscp (__A);
}

#endif /* __iamcu__ */

/* 8bit rol */
extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rolb (unsigned char __X, int __C)
{
  return __builtin_ia32_rolqi (__X, __C);
}

/* 16bit rol */
extern __inline unsigned short
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rolw (unsigned short __X, int __C)
{
  return __builtin_ia32_rolhi (__X, __C);
}

/* 32bit rol */
extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rold (unsigned int __X, int __C)
{
  __C &= 31;
  return (__X << __C) | (__X >> (-__C & 31));
}

/* 8bit ror */
extern __inline unsigned char
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rorb (unsigned char __X, int __C)
{
  return __builtin_ia32_rorqi (__X, __C);
}

/* 16bit ror */
extern __inline unsigned short
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rorw (unsigned short __X, int __C)
{
  return __builtin_ia32_rorhi (__X, __C);
}

/* 32bit ror */
extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rord (unsigned int __X, int __C)
{
  __C &= 31;
  return (__X >> __C) | (__X << (-__C & 31));
}

/* Pause */
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__pause (void)
{
  __builtin_ia32_pause ();
}

#ifdef __x86_64__
/* 64bit bsf */
extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bsfq (long long __X)
{
  return __builtin_ctzll (__X);
}

/* 64bit bsr */
extern __inline int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bsrq (long long __X)
{
  return __builtin_ia32_bsrdi (__X);
}

/* 64bit bswap */
extern __inline long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__bswapq (long long __X)
{
  return __builtin_bswap64 (__X);
}

#ifndef __SSE4_2__
#pragma GCC push_options
#pragma GCC target("sse4.2")
#define __DISABLE_SSE4_2__
#endif /* __SSE4_2__ */

/* 64bit accumulate CRC32 (polynomial 0x11EDC6F41) value.  */
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__crc32q (unsigned long long __C, unsigned long long __V)
{
  return __builtin_ia32_crc32di (__C, __V);
}

#ifdef __DISABLE_SSE4_2__
#undef __DISABLE_SSE4_2__
#pragma GCC pop_options
#endif /* __DISABLE_SSE4_2__ */

/* 64bit popcnt */
extern __inline long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__popcntq (unsigned long long __X)
{
  return __builtin_popcountll (__X);
}

/* 64bit rol */
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rolq (unsigned long long __X, int __C)
{
  __C &= 63;
  return (__X << __C) | (__X >> (-__C & 63));
}

/* 64bit ror */
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__rorq (unsigned long long __X, int __C)
{
  __C &= 63;
  return (__X >> __C) | (__X << (-__C & 63));
}

/* Read flags register */
extern __inline unsigned long long
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__readeflags (void)
{
  return __builtin_ia32_readeflags_u64 ();
}

/* Write flags register */
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__writeeflags (unsigned long long __X)
{
  __builtin_ia32_writeeflags_u64 (__X);
}

#define _bswap64(a)		__bswapq(a)
#define _popcnt64(a)		__popcntq(a)
#else

/* Read flags register */
extern __inline unsigned int
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__readeflags (void)
{
  return __builtin_ia32_readeflags_u32 ();
}

/* Write flags register */
extern __inline void
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
__writeeflags (unsigned int __X)
{
  __builtin_ia32_writeeflags_u32 (__X);
}

#endif

/* On LP64 systems, longs are 64-bit.  Use the appropriate rotate
 * function.  */
#ifdef __LP64__
#define _lrotl(a,b)		__rolq((a), (b))
#define _lrotr(a,b)		__rorq((a), (b))
#else
#define _lrotl(a,b)		__rold((a), (b))
#define _lrotr(a,b)		__rord((a), (b))
#endif

#define _bit_scan_forward(a)	__bsfd(a)
#define _bit_scan_reverse(a)	__bsrd(a)
#define _bswap(a)		__bswapd(a)
#define _popcnt32(a)		__popcntd(a)
#ifndef __iamcu__
#define _rdpmc(a)		__rdpmc(a)
#define _rdtscp(a)		__rdtscp(a)
#endif /* __iamcu__ */
#define _rdtsc()		__rdtsc()
#define _rotwl(a,b)		__rolw((a), (b))
#define _rotwr(a,b)		__rorw((a), (b))
#define _rotl(a,b)		__rold((a), (b))
#define _rotr(a,b)		__rord((a), (b))
