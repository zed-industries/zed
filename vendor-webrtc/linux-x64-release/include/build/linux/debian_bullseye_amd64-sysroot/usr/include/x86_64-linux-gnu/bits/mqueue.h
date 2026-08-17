/* Copyright (C) 2004-2020 Free Software Foundation, Inc.
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

#ifndef _MQUEUE_H
# error "Never use <bits/mqueue.h> directly; include <mqueue.h> instead."
#endif

#include <bits/types.h>

typedef int mqd_t;

struct mq_attr
{
  __syscall_slong_t mq_flags;	/* Message queue flags.  */
  __syscall_slong_t mq_maxmsg;	/* Maximum number of messages.  */
  __syscall_slong_t mq_msgsize;	/* Maximum message size.  */
  __syscall_slong_t mq_curmsgs;	/* Number of messages currently queued.  */
  __syscall_slong_t __pad[4];
};
