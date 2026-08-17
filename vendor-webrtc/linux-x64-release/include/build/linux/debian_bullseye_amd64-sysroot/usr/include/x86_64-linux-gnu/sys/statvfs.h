/* Definitions for getting information about a filesystem.
   Copyright (C) 1998-2020 Free Software Foundation, Inc.
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

#ifndef	_SYS_STATVFS_H
#define	_SYS_STATVFS_H	1

#include <features.h>

/* Get the system-specific definition of `struct statfs'.  */
#include <bits/statvfs.h>

#ifndef __USE_FILE_OFFSET64
# ifndef __fsblkcnt_t_defined
typedef __fsblkcnt_t fsblkcnt_t; /* Type to count file system blocks.  */
#  define __fsblkcnt_t_defined
# endif
# ifndef __fsfilcnt_t_defined
typedef __fsfilcnt_t fsfilcnt_t; /* Type to count file system inodes.  */
#  define __fsfilcnt_t_defined
# endif
#else
# ifndef __fsblkcnt_t_defined
typedef __fsblkcnt64_t fsblkcnt_t; /* Type to count file system blocks.  */
#  define __fsblkcnt_t_defined
# endif
# ifndef __fsfilcnt_t_defined
typedef __fsfilcnt64_t fsfilcnt_t; /* Type to count file system inodes.  */
#  define __fsfilcnt_t_defined
# endif
#endif

__BEGIN_DECLS

/* Return information about the filesystem on which FILE resides.  */
#ifndef __USE_FILE_OFFSET64
extern int statvfs (const char *__restrict __file,
		    struct statvfs *__restrict __buf)
     __THROW __nonnull ((1, 2));
#else
# ifdef __REDIRECT_NTH
extern int __REDIRECT_NTH (statvfs,
			   (const char *__restrict __file,
			    struct statvfs *__restrict __buf), statvfs64)
     __nonnull ((1, 2));
# else
#  define statvfs statvfs64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int statvfs64 (const char *__restrict __file,
		      struct statvfs64 *__restrict __buf)
     __THROW __nonnull ((1, 2));
#endif

/* Return information about the filesystem containing the file FILDES
   refers to.  */
#ifndef __USE_FILE_OFFSET64
extern int fstatvfs (int __fildes, struct statvfs *__buf)
     __THROW __nonnull ((2));
#else
# ifdef __REDIRECT_NTH
extern int __REDIRECT_NTH (fstatvfs, (int __fildes, struct statvfs *__buf),
			   fstatvfs64) __nonnull ((2));
# else
#  define fstatvfs fstatvfs64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int fstatvfs64 (int __fildes, struct statvfs64 *__buf)
     __THROW __nonnull ((2));
#endif

__END_DECLS

#endif	/* sys/statvfs.h */
