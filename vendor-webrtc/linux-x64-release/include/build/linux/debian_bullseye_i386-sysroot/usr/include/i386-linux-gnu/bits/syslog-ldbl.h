/* -mlong-double-64 compatibility mode for syslog functions.
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

#ifndef _SYS_SYSLOG_H
# error "Never include <bits/syslog-ldbl.h> directly; use <sys/syslog.h> instead."
#endif

__LDBL_REDIR_DECL (syslog)

#ifdef __USE_MISC
__LDBL_REDIR_DECL (vsyslog)
#endif

#if __USE_FORTIFY_LEVEL > 0 && defined __fortify_function
__LDBL_REDIR_DECL (__syslog_chk)

# ifdef __USE_MISC
__LDBL_REDIR_DECL (__vsyslog_chk)
# endif
#endif
