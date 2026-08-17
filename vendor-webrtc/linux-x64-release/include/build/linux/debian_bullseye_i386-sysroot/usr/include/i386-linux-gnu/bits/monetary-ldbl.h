/* -mlong-double-64 compatibility mode for monetary functions.
   Copyright (C) 2006-2020 Free Software Foundation, Inc.
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

#ifndef _MONETARY_H
# error "Never include <bits/monetary-ldbl.h> directly; use <monetary.h> instead."
#endif

__LDBL_REDIR_DECL (strfmon)

#ifdef __USE_GNU
__LDBL_REDIR_DECL (strfmon_l)
#endif
