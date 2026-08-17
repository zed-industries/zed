/*
   This file is part of Valgrind, a dynamic binary instrumentation framework.

   Copyright (C) 2019 Bart Van Assche <bvanassche@acm.org>

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.
*/

#ifndef __VKI_SCNUMS_32BIT_LINUX_H
#define __VKI_SCNUMS_32BIT_LINUX_H

// Derived from the __BITS_PER_LONG == 32 sections in
// linux-5.2/include/uapi/asm-generic/unistd.h

#define __NR_clock_gettime64	403
#define __NR_clock_settime64	404
#define __NR_clock_adjtime64	405
#define __NR_clock_getres_time64	406
#define __NR_clock_nanosleep_time64	407
#define __NR_timer_gettime64	408
#define __NR_timer_settime64	409
#define __NR_timerfd_gettime64	410
#define __NR_timerfd_settime64	411
#define __NR_utimensat_time64	412
#define __NR_pselect6_time64	413
#define __NR_ppoll_time64	414
#define __NR_io_pgetevents_time64	416
#define __NR_recvmmsg_time64	417
#define __NR_mq_timedsend_time64	418
#define __NR_mq_timedreceive_time64	419
#define __NR_semtimedop_time64	420
#define __NR_rt_sigtimedwait_time64	421
#define __NR_futex_time64	422
#define __NR_sched_rr_get_interval_time64	423

#endif
