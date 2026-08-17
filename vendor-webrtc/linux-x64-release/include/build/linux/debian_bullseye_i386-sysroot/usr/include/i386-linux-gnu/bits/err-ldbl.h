/* Redirections for err.h functions for -mlong-double-64.
   Copyright (C) 2019-2020 Free Software Foundation, Inc.
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

#ifndef _ERR_H
# error "Never include <bits/err-ldbl.h> directly; use <err.h> instead."
#endif

__LDBL_REDIR_DECL (warn)
__LDBL_REDIR_DECL (vwarn)
__LDBL_REDIR_DECL (warnx)
__LDBL_REDIR_DECL (vwarnx)
__LDBL_REDIR_DECL (err)
__LDBL_REDIR_DECL (verr)
__LDBL_REDIR_DECL (errx)
__LDBL_REDIR_DECL (verrx)
