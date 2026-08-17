/* Compatibility header for old-style Unix parameters and limits.
   Copyright (C) 1995-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_PARAM_H
#define _SYS_PARAM_H    1

#define __need_NULL
#include <stddef.h>

#include <sys/types.h>
#include <limits.h>
#include <endian.h>                     /* Define BYTE_ORDER et al.  */
#include <signal.h>                     /* Define NSIG.  */

/* This file defines some things in system-specific ways.  */
#include <bits/param.h>


/* BSD names for some <limits.h> values.  */

#define NBBY		CHAR_BIT

#if !defined NGROUPS && defined NGROUPS_MAX
# define NGROUPS	NGROUPS_MAX
#endif
#if !defined MAXSYMLINKS && defined SYMLOOP_MAX
# define MAXSYMLINKS	SYMLOOP_MAX
#endif
#if !defined CANBSIZ && defined MAX_CANON
# define CANBSIZ	MAX_CANON
#endif
#if !defined MAXPATHLEN && defined PATH_MAX
# define MAXPATHLEN	PATH_MAX
#endif
#if !defined NOFILE && defined OPEN_MAX
# define NOFILE		OPEN_MAX
#endif
#if !defined MAXHOSTNAMELEN && defined HOST_NAME_MAX
# define MAXHOSTNAMELEN	HOST_NAME_MAX
#endif
#ifndef NCARGS
# ifdef ARG_MAX
#  define NCARGS	ARG_MAX
# else
/* ARG_MAX is unlimited, but we define NCARGS for BSD programs that want to
   compare against some fixed limit.  */
# define NCARGS		INT_MAX
# endif
#endif


/* Magical constants.  */
#ifndef NOGROUP
# define NOGROUP	65535     /* Marker for empty group set member.  */
#endif
#ifndef NODEV
# define NODEV		((dev_t) -1)    /* Non-existent device.  */
#endif


/* Unit of `st_blocks'.  */
#ifndef DEV_BSIZE
# define DEV_BSIZE	512
#endif


/* Bit map related macros.  */
#define setbit(a,i)     ((a)[(i)/NBBY] |= 1<<((i)%NBBY))
#define clrbit(a,i)     ((a)[(i)/NBBY] &= ~(1<<((i)%NBBY)))
#define isset(a,i)      ((a)[(i)/NBBY] & (1<<((i)%NBBY)))
#define isclr(a,i)      (((a)[(i)/NBBY] & (1<<((i)%NBBY))) == 0)

/* Macros for counting and rounding.  */
#ifndef howmany
# define howmany(x, y)  (((x) + ((y) - 1)) / (y))
#endif
#ifdef __GNUC__
# define roundup(x, y)  (__builtin_constant_p (y) && powerof2 (y)             \
                         ? (((x) + (y) - 1) & ~((y) - 1))                     \
                         : ((((x) + ((y) - 1)) / (y)) * (y)))
#else
# define roundup(x, y)  ((((x) + ((y) - 1)) / (y)) * (y))
#endif
#define powerof2(x)     ((((x) - 1) & (x)) == 0)

/* Macros for min/max.  */
#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))


#endif  /* sys/param.h */
