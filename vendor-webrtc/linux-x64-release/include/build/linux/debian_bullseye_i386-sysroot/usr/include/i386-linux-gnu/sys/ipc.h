/* Copyright (C) 1995-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_IPC_H
#define _SYS_IPC_H	1

#include <features.h>

/* Get system dependent definition of `struct ipc_perm' and more.  */
#include <bits/ipctypes.h>
#include <bits/ipc.h>

#ifndef __uid_t_defined
typedef __uid_t uid_t;
# define __uid_t_defined
#endif

#ifndef __gid_t_defined
typedef __gid_t gid_t;
# define __gid_t_defined
#endif

#ifndef __mode_t_defined
typedef __mode_t mode_t;
# define __mode_t_defined
#endif

#ifndef __key_t_defined
typedef __key_t key_t;
# define __key_t_defined
#endif

__BEGIN_DECLS

/* Generates key for System V style IPC.  */
extern key_t ftok (const char *__pathname, int __proj_id) __THROW;

__END_DECLS

#endif /* sys/ipc.h */
