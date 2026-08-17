/* Access to the auxiliary vector.
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

#ifndef _SYS_AUXV_H
#define _SYS_AUXV_H 1

#include <elf.h>
#include <bits/auxv.h>
#include <sys/cdefs.h>
#include <bits/hwcap.h>

__BEGIN_DECLS

/* Return the value associated with an Elf*_auxv_t type from the auxv list
   passed to the program on startup.  If TYPE was not present in the auxv
   list, returns zero and sets errno to ENOENT.  */
extern unsigned long int getauxval (unsigned long int __type)
  __THROW;

__END_DECLS

#endif /* sys/auxv.h */
