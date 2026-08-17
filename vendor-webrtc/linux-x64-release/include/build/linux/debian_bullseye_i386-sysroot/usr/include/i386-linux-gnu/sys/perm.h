/* Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_PERM_H

#define _SYS_PERM_H	1
#include <features.h>

__BEGIN_DECLS

/* Set port input/output permissions.  */
extern int ioperm (unsigned long int __from, unsigned long int __num,
		   int __turn_on) __THROW;


/* Change I/O privilege level.  */
extern int iopl (int __level) __THROW;

__END_DECLS

#endif	/* _SYS_PERM_H */
