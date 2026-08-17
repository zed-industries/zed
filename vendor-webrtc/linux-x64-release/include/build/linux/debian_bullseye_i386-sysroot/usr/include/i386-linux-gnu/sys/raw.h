/* Copyright (C) 1999-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_RAW_H
#define _SYS_RAW_H	1

#include <stdint.h>
#include <sys/ioctl.h>

/* The major device number for raw devices.  */
#define RAW_MAJOR	162

/* `ioctl' commands for raw devices.  */
#define RAW_SETBIND     _IO(0xac, 0)
#define RAW_GETBIND     _IO(0xac, 1)

struct raw_config_request
{
  int raw_minor;
  uint64_t block_major;
  uint64_t block_minor;
};

#endif /* sys/raw.h */
