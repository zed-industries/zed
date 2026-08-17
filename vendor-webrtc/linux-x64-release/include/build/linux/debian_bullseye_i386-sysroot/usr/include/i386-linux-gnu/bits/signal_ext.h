/* System-specific extensions of <signal.h>, Linux version.
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

#ifndef _SIGNAL_H
# error "Never include <bits/signal_ext.h> directly; use <signal.h> instead."
#endif

#ifdef __USE_GNU

/* Send SIGNAL to the thread TID in the thread group (process)
   identified by TGID.  This function behaves like kill, but also
   fails with ESRCH if the specified TID does not belong to the
   specified thread group.  */
extern int tgkill (__pid_t __tgid, __pid_t __tid, int __signal);

#endif /* __USE_GNU */
