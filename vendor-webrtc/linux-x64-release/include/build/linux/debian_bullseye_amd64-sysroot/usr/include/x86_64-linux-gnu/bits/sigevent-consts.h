/* sigevent constants.  Linux version.
   Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

#ifndef _BITS_SIGEVENT_CONSTS_H
#define _BITS_SIGEVENT_CONSTS_H 1

#if !defined _SIGNAL_H && !defined _AIO_H
#error "Don't include <bits/sigevent-consts.h> directly; use <signal.h> instead."
#endif

/* `sigev_notify' values.  */
enum
{
  SIGEV_SIGNAL = 0,		/* Notify via signal.  */
# define SIGEV_SIGNAL	SIGEV_SIGNAL
  SIGEV_NONE,			/* Other notification: meaningless.  */
# define SIGEV_NONE	SIGEV_NONE
  SIGEV_THREAD,			/* Deliver via thread creation.  */
# define SIGEV_THREAD	SIGEV_THREAD

  SIGEV_THREAD_ID = 4		/* Send signal to specific thread.
				   This is a Linux extension.  */
#define SIGEV_THREAD_ID	SIGEV_THREAD_ID
};

#endif
