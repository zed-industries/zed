/* Definitions for getting information about a filesystem.
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

#ifndef	_SYS_STATFS_H
#define	_SYS_STATFS_H	1

#include <features.h>

/* Get the system-specific definition of `struct statfs'.  */
#include <bits/statfs.h>

__BEGIN_DECLS

/* Return information about the filesystem on which FILE resides.  */
#ifndef __USE_FILE_OFFSET64
extern int statfs (const char *__file, struct statfs *__buf)
     __THROW __nonnull ((1, 2));
#else
# ifdef __REDIRECT_NTH
extern int __REDIRECT_NTH (statfs,
			   (const char *__file, struct statfs *__buf),
			   statfs64) __nonnull ((1, 2));
# else
#  define statfs statfs64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int statfs64 (const char *__file, struct statfs64 *__buf)
     __THROW __nonnull ((1, 2));
#endif

/* Return information about the filesystem containing the file FILDES
   refers to.  */
#ifndef __USE_FILE_OFFSET64
extern int fstatfs (int __fildes, struct statfs *__buf)
     __THROW __nonnull ((2));
#else
# ifdef __REDIRECT_NTH
extern int __REDIRECT_NTH (fstatfs, (int __fildes, struct statfs *__buf),
			   fstatfs64) __nonnull ((2));
# else
#  define fstatfs fstatfs64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int fstatfs64 (int __fildes, struct statfs64 *__buf)
     __THROW __nonnull ((2));
#endif

__END_DECLS

#endif	/* sys/statfs.h */
