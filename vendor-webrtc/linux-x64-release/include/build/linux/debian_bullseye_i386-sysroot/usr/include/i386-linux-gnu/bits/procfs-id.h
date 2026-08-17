/* Types of pr_uid and pr_gid in struct elf_prpsinfo.  x86 version.
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

#ifndef _SYS_PROCFS_H
# error "Never include <bits/procfs-id.h> directly; use <sys/procfs.h> instead."
#endif

#if __WORDSIZE == 32
typedef unsigned short int __pr_uid_t;
typedef unsigned short int __pr_gid_t;
#else
typedef unsigned int __pr_uid_t;
typedef unsigned int __pr_gid_t;
#endif
