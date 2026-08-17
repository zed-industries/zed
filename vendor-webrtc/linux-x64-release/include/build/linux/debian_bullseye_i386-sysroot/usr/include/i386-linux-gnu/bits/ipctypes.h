/* bits/ipctypes.h -- Define some types used by SysV IPC/MSG/SHM.
   Copyright (C) 2012-2020 Free Software Foundation, Inc.
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
# error "Never use <bits/ipctypes.h> directly; include <sys/ipc.h> instead."
#endif

#ifndef _BITS_IPCTYPES_H
#define _BITS_IPCTYPES_H	1

/* Used in `struct shmid_ds'.  */
# ifdef __x86_64__
typedef int __ipc_pid_t;
# else
typedef unsigned short int __ipc_pid_t;
# endif

#endif /* bits/ipctypes.h */
