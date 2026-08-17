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

#ifndef _SYS_FSUID_H
#define _SYS_FSUID_H	1

#include <features.h>
#include <sys/types.h>

__BEGIN_DECLS

/* Change uid used for file access control to UID, without affecting
   other privileges (such as who can send signals at the process).  */
extern int setfsuid (__uid_t __uid) __THROW;

/* Ditto for group id. */
extern int setfsgid (__gid_t __gid) __THROW;

__END_DECLS

#endif /* fsuid.h */
