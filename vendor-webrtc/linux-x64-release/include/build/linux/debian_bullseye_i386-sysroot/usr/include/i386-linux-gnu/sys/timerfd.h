/* Copyright (C) 2008-2020 Free Software Foundation, Inc.
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

#ifndef	_SYS_TIMERFD_H
#define	_SYS_TIMERFD_H	1

#include <time.h>
#include <bits/types/struct_itimerspec.h>

/* Get the platform-dependent flags.  */
#include <bits/timerfd.h>


/* Bits to be set in the FLAGS parameter of `timerfd_settime'.  */
enum
  {
    TFD_TIMER_ABSTIME = 1 << 0,
#define TFD_TIMER_ABSTIME TFD_TIMER_ABSTIME
    TFD_TIMER_CANCEL_ON_SET = 1 << 1
#define TFD_TIMER_CANCEL_ON_SET TFD_TIMER_CANCEL_ON_SET
  };


__BEGIN_DECLS

/* Return file descriptor for new interval timer source.  */
extern int timerfd_create (__clockid_t __clock_id, int __flags) __THROW;

/* Set next expiration time of interval timer source UFD to UTMR.  If
   FLAGS has the TFD_TIMER_ABSTIME flag set the timeout value is
   absolute.  Optionally return the old expiration time in OTMR.  */
extern int timerfd_settime (int __ufd, int __flags,
			    const struct itimerspec *__utmr,
			    struct itimerspec *__otmr) __THROW;

/* Return the next expiration time of UFD.  */
extern int timerfd_gettime (int __ufd, struct itimerspec *__otmr) __THROW;

__END_DECLS

#endif /* sys/timerfd.h */
