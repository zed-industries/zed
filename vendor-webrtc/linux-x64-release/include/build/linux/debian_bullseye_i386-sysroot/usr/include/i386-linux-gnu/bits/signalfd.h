/* Copyright (C) 2007-2020 Free Software Foundation, Inc.
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

#ifndef	_SYS_SIGNALFD_H
# error "Never use <bits/signalfd.h> directly; include <sys/signalfd.h> instead."
#endif

/* Flags for signalfd.  */
enum
  {
    SFD_CLOEXEC = 02000000,
#define SFD_CLOEXEC SFD_CLOEXEC
    SFD_NONBLOCK = 00004000
#define SFD_NONBLOCK SFD_NONBLOCK
  };
