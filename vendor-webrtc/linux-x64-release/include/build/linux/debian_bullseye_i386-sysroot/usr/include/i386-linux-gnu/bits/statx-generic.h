/* Generic statx-related definitions and declarations.
   Copyright (C) 2018-2020 Free Software Foundation, Inc.
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

/* This interface is based on <linux/stat.h> in Linux.  */

#ifndef _SYS_STAT_H
# error Never include <bits/statx-generic.h> directly, include <sys/stat.h> instead.
#endif

#include <bits/types/struct_statx_timestamp.h>
#include <bits/types/struct_statx.h>

#ifndef STATX_TYPE
# define STATX_TYPE 0x0001U
# define STATX_MODE 0x0002U
# define STATX_NLINK 0x0004U
# define STATX_UID 0x0008U
# define STATX_GID 0x0010U
# define STATX_ATIME 0x0020U
# define STATX_MTIME 0x0040U
# define STATX_CTIME 0x0080U
# define STATX_INO 0x0100U
# define STATX_SIZE 0x0200U
# define STATX_BLOCKS 0x0400U
# define STATX_BASIC_STATS 0x07ffU
# define STATX_ALL 0x0fffU
# define STATX_BTIME 0x0800U
# define STATX__RESERVED 0x80000000U

# define STATX_ATTR_COMPRESSED 0x0004
# define STATX_ATTR_IMMUTABLE 0x0010
# define STATX_ATTR_APPEND 0x0020
# define STATX_ATTR_NODUMP 0x0040
# define STATX_ATTR_ENCRYPTED 0x0800
# define STATX_ATTR_AUTOMOUNT 0x1000
#endif /* !STATX_TYPE */

__BEGIN_DECLS

/* Fill *BUF with information about PATH in DIRFD.  */
int statx (int __dirfd, const char *__restrict __path, int __flags,
           unsigned int __mask, struct statx *__restrict __buf)
  __THROW __nonnull ((2, 5));

__END_DECLS
