/* Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_STATVFS_H
# error "Never include <bits/statvfs.h> directly; use <sys/statvfs.h> instead."
#endif

#include <bits/types.h>  /* For __fsblkcnt_t and __fsfilcnt_t.  */

#if (__WORDSIZE == 32 \
     && (!defined __SYSCALL_WORDSIZE || __SYSCALL_WORDSIZE == 32))
#define _STATVFSBUF_F_UNUSED
#endif

struct statvfs
  {
    unsigned long int f_bsize;
    unsigned long int f_frsize;
#ifndef __USE_FILE_OFFSET64
    __fsblkcnt_t f_blocks;
    __fsblkcnt_t f_bfree;
    __fsblkcnt_t f_bavail;
    __fsfilcnt_t f_files;
    __fsfilcnt_t f_ffree;
    __fsfilcnt_t f_favail;
#else
    __fsblkcnt64_t f_blocks;
    __fsblkcnt64_t f_bfree;
    __fsblkcnt64_t f_bavail;
    __fsfilcnt64_t f_files;
    __fsfilcnt64_t f_ffree;
    __fsfilcnt64_t f_favail;
#endif
    unsigned long int f_fsid;
#ifdef _STATVFSBUF_F_UNUSED
    int __f_unused;
#endif
    unsigned long int f_flag;
    unsigned long int f_namemax;
    int __f_spare[6];
  };

#ifdef __USE_LARGEFILE64
struct statvfs64
  {
    unsigned long int f_bsize;
    unsigned long int f_frsize;
    __fsblkcnt64_t f_blocks;
    __fsblkcnt64_t f_bfree;
    __fsblkcnt64_t f_bavail;
    __fsfilcnt64_t f_files;
    __fsfilcnt64_t f_ffree;
    __fsfilcnt64_t f_favail;
    unsigned long int f_fsid;
#ifdef _STATVFSBUF_F_UNUSED
    int __f_unused;
#endif
    unsigned long int f_flag;
    unsigned long int f_namemax;
    int __f_spare[6];
  };
#endif

/* Definitions for the flag in `f_flag'.  These definitions should be
   kept in sync with the definitions in <sys/mount.h>.  */
enum
{
  ST_RDONLY = 1,		/* Mount read-only.  */
#define ST_RDONLY	ST_RDONLY
  ST_NOSUID = 2			/* Ignore suid and sgid bits.  */
#define ST_NOSUID	ST_NOSUID
#ifdef __USE_GNU
  ,
  ST_NODEV = 4,			/* Disallow access to device special files.  */
# define ST_NODEV	ST_NODEV
  ST_NOEXEC = 8,		/* Disallow program execution.  */
# define ST_NOEXEC	ST_NOEXEC
  ST_SYNCHRONOUS = 16,		/* Writes are synced at once.  */
# define ST_SYNCHRONOUS	ST_SYNCHRONOUS
  ST_MANDLOCK = 64,		/* Allow mandatory locks on an FS.  */
# define ST_MANDLOCK	ST_MANDLOCK
  ST_WRITE = 128,		/* Write on file/directory/symlink.  */
# define ST_WRITE	ST_WRITE
  ST_APPEND = 256,		/* Append-only file.  */
# define ST_APPEND	ST_APPEND
  ST_IMMUTABLE = 512,		/* Immutable file.  */
# define ST_IMMUTABLE	ST_IMMUTABLE
  ST_NOATIME = 1024,		/* Do not update access times.  */
# define ST_NOATIME	ST_NOATIME
  ST_NODIRATIME = 2048,		/* Do not update directory access times.  */
# define ST_NODIRATIME	ST_NODIRATIME
  ST_RELATIME = 4096		/* Update atime relative to mtime/ctime.  */
# define ST_RELATIME	ST_RELATIME
#endif	/* Use GNU.  */
};
