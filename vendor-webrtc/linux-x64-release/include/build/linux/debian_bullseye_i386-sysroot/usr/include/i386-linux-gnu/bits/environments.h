/* Copyright (C) 1999-2020 Free Software Foundation, Inc.
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

#ifndef _UNISTD_H
# error "Never include this file directly.  Use <unistd.h> instead"
#endif

#include <bits/wordsize.h>

/* This header should define the following symbols under the described
   situations.  A value `1' means that the model is always supported,
   `-1' means it is never supported.  Undefined means it cannot be
   statically decided.

   _POSIX_V7_ILP32_OFF32   32bit int, long, pointers, and off_t type
   _POSIX_V7_ILP32_OFFBIG  32bit int, long, and pointers and larger off_t type

   _POSIX_V7_LP64_OFF32	   64bit long and pointers and 32bit off_t type
   _POSIX_V7_LPBIG_OFFBIG  64bit long and pointers and large off_t type

   The macros _POSIX_V6_ILP32_OFF32, _POSIX_V6_ILP32_OFFBIG,
   _POSIX_V6_LP64_OFF32, _POSIX_V6_LPBIG_OFFBIG, _XBS5_ILP32_OFF32,
   _XBS5_ILP32_OFFBIG, _XBS5_LP64_OFF32, and _XBS5_LPBIG_OFFBIG were
   used in previous versions of the Unix standard and are available
   only for compatibility.
*/

#if __WORDSIZE == 64

/* Environments with 32-bit wide pointers are optionally provided.
   Therefore following macros aren't defined:
   # undef _POSIX_V7_ILP32_OFF32
   # undef _POSIX_V7_ILP32_OFFBIG
   # undef _POSIX_V6_ILP32_OFF32
   # undef _POSIX_V6_ILP32_OFFBIG
   # undef _XBS5_ILP32_OFF32
   # undef _XBS5_ILP32_OFFBIG
   and users need to check at runtime.  */

/* We also have no use (for now) for an environment with bigger pointers
   and offsets.  */
# define _POSIX_V7_LPBIG_OFFBIG	-1
# define _POSIX_V6_LPBIG_OFFBIG	-1
# define _XBS5_LPBIG_OFFBIG	-1

/* By default we have 64-bit wide `long int', pointers and `off_t'.  */
# define _POSIX_V7_LP64_OFF64	1
# define _POSIX_V6_LP64_OFF64	1
# define _XBS5_LP64_OFF64	1

#else /* __WORDSIZE == 32 */

/* We have 32-bit wide `int', `long int' and pointers and all platforms
   support LFS.  -mx32 has 64-bit wide `off_t'.  */
# define _POSIX_V7_ILP32_OFFBIG	1
# define _POSIX_V6_ILP32_OFFBIG 1
# define _XBS5_ILP32_OFFBIG	1

# ifndef __x86_64__
/* -m32 has 32-bit wide `off_t'.  */
#  define _POSIX_V7_ILP32_OFF32	1
#  define _POSIX_V6_ILP32_OFF32	1
#  define _XBS5_ILP32_OFF32	1
# endif

/* We optionally provide an environment with the above size but an 64-bit
   side `off_t'.  Therefore we don't define _POSIX_V7_ILP32_OFFBIG.  */

/* Environments with 64-bit wide pointers can be provided,
   so these macros aren't defined:
   # undef _POSIX_V7_LP64_OFF64
   # undef _POSIX_V7_LPBIG_OFFBIG
   # undef _POSIX_V6_LP64_OFF64
   # undef _POSIX_V6_LPBIG_OFFBIG
   # undef _XBS5_LP64_OFF64
   # undef _XBS5_LPBIG_OFFBIG
   and sysconf tests for it at runtime.  */

#endif /* __WORDSIZE == 32 */

#define __ILP32_OFF32_CFLAGS	"-m32"
#define __ILP32_OFF32_LDFLAGS	"-m32"
#if defined __x86_64__ && defined __ILP32__
# define __ILP32_OFFBIG_CFLAGS	"-mx32"
# define __ILP32_OFFBIG_LDFLAGS	"-mx32"
#else
# define __ILP32_OFFBIG_CFLAGS	"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64"
# define __ILP32_OFFBIG_LDFLAGS	"-m32"
#endif
#define __LP64_OFF64_CFLAGS	"-m64"
#define __LP64_OFF64_LDFLAGS	"-m64"
