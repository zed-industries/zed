/* Declarations for getopt.
   Copyright (C) 1989-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Unlike the bulk of the getopt implementation, this file is NOT part
   of gnulib; gnulib also has a getopt.h but it is different.

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

#ifndef _GETOPT_H
#define _GETOPT_H 1

#include <features.h>

/* The type of the 'argv' argument to getopt_long and getopt_long_only
   is properly 'char **', since both functions may write to the array
   (in order to move all the options to the beginning).  However, for
   compatibility with old versions of LSB, glibc has to use 'char *const *'
   instead.  */
#ifndef __getopt_argv_const
# define __getopt_argv_const const
#endif

#include <bits/getopt_core.h>
#include <bits/getopt_ext.h>

#endif /* getopt.h */
