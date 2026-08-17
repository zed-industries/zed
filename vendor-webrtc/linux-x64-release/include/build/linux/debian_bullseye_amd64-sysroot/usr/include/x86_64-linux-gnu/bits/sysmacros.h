/* Definitions of macros to access `dev_t' values.
   Copyright (C) 1996-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _BITS_SYSMACROS_H
#define _BITS_SYSMACROS_H 1

#ifndef _SYS_SYSMACROS_H
# error "Never include <bits/sysmacros.h> directly; use <sys/sysmacros.h> instead."
#endif

/* dev_t in glibc is a 64-bit quantity, with 32-bit major and minor numbers.
   Our default encoding is MMMM Mmmm mmmM MMmm, where M is a hex digit of
   the major number and m is a hex digit of the minor number.  This is
   downward compatible with legacy systems where dev_t is 16 bits wide,
   encoded as MMmm.  It is also downward compatible with the Linux kernel,
   which (as of 2016) uses 32-bit dev_t, encoded as mmmM MMmm.

   Systems that use an incompatible encoding for dev_t should override this
   file in the appropriate sysdeps subdirectory.  */

#define __SYSMACROS_DECLARE_MAJOR(DECL_TEMPL)			\
  DECL_TEMPL(unsigned int, major, (__dev_t __dev))

#define __SYSMACROS_DEFINE_MAJOR(DECL_TEMPL)			\
  __SYSMACROS_DECLARE_MAJOR (DECL_TEMPL)			\
  {								\
    unsigned int __major;					\
    __major  = ((__dev & (__dev_t) 0x00000000000fff00u) >>  8); \
    __major |= ((__dev & (__dev_t) 0xfffff00000000000u) >> 32); \
    return __major;						\
  }

#define __SYSMACROS_DECLARE_MINOR(DECL_TEMPL)			\
  DECL_TEMPL(unsigned int, minor, (__dev_t __dev))

#define __SYSMACROS_DEFINE_MINOR(DECL_TEMPL)			\
  __SYSMACROS_DECLARE_MINOR (DECL_TEMPL)			\
  {								\
    unsigned int __minor;					\
    __minor  = ((__dev & (__dev_t) 0x00000000000000ffu) >>  0); \
    __minor |= ((__dev & (__dev_t) 0x00000ffffff00000u) >> 12); \
    return __minor;						\
  }

#define __SYSMACROS_DECLARE_MAKEDEV(DECL_TEMPL)			\
  DECL_TEMPL(__dev_t, makedev, (unsigned int __major, unsigned int __minor))

#define __SYSMACROS_DEFINE_MAKEDEV(DECL_TEMPL)			\
  __SYSMACROS_DECLARE_MAKEDEV (DECL_TEMPL)			\
  {								\
    __dev_t __dev;						\
    __dev  = (((__dev_t) (__major & 0x00000fffu)) <<  8);	\
    __dev |= (((__dev_t) (__major & 0xfffff000u)) << 32);	\
    __dev |= (((__dev_t) (__minor & 0x000000ffu)) <<  0);	\
    __dev |= (((__dev_t) (__minor & 0xffffff00u)) << 12);	\
    return __dev;						\
  }

#endif /* bits/sysmacros.h */
