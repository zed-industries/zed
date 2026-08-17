/* Copyright (C) 2002-2020 Free Software Foundation, Inc.
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

#ifndef	_SYS_EPOLL_H
# error "Never use <bits/epoll.h> directly; include <sys/epoll.h> instead."
#endif

/* Flags to be passed to epoll_create1.  */
enum
  {
    EPOLL_CLOEXEC = 02000000
#define EPOLL_CLOEXEC EPOLL_CLOEXEC
  };

#define __EPOLL_PACKED __attribute__ ((__packed__))
