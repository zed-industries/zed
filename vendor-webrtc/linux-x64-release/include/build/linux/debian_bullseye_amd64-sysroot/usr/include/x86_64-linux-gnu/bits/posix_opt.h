/* Define POSIX options for Linux.
   Copyright (C) 1996-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; see the file COPYING.LIB.  If
   not, see <https://www.gnu.org/licenses/>.  */

#ifndef	_BITS_POSIX_OPT_H
#define	_BITS_POSIX_OPT_H	1

/* Job control is supported.  */
#define	_POSIX_JOB_CONTROL	1

/* Processes have a saved set-user-ID and a saved set-group-ID.  */
#define	_POSIX_SAVED_IDS	1

/* Priority scheduling is not supported with the correct semantics,
   but GNU/Linux applications expect that the corresponding interfaces
   are available, even though the semantics do not meet the POSIX
   requirements.  See glibc bug 14829.  */
#define	_POSIX_PRIORITY_SCHEDULING	200809L

/* Synchronizing file data is supported.  */
#define	_POSIX_SYNCHRONIZED_IO	200809L

/* The fsync function is present.  */
#define	_POSIX_FSYNC	200809L

/* Mapping of files to memory is supported.  */
#define	_POSIX_MAPPED_FILES	200809L

/* Locking of all memory is supported.  */
#define	_POSIX_MEMLOCK	200809L

/* Locking of ranges of memory is supported.  */
#define	_POSIX_MEMLOCK_RANGE	200809L

/* Setting of memory protections is supported.  */
#define	_POSIX_MEMORY_PROTECTION	200809L

/* Some filesystems allow all users to change file ownership.  */
#define	_POSIX_CHOWN_RESTRICTED	0

/* `c_cc' member of 'struct termios' structure can be disabled by
   using the value _POSIX_VDISABLE.  */
#define	_POSIX_VDISABLE	'\0'

/* Filenames are not silently truncated.  */
#define	_POSIX_NO_TRUNC	1

/* X/Open realtime support is available.  */
#define _XOPEN_REALTIME	1

/* X/Open thread realtime support is available.  */
#define _XOPEN_REALTIME_THREADS	1

/* XPG4.2 shared memory is supported.  */
#define	_XOPEN_SHM	1

/* Tell we have POSIX threads.  */
#define _POSIX_THREADS	200809L

/* We have the reentrant functions described in POSIX.  */
#define _POSIX_REENTRANT_FUNCTIONS      1
#define _POSIX_THREAD_SAFE_FUNCTIONS	200809L

/* We provide priority scheduling for threads.  */
#define _POSIX_THREAD_PRIORITY_SCHEDULING	200809L

/* We support user-defined stack sizes.  */
#define _POSIX_THREAD_ATTR_STACKSIZE	200809L

/* We support user-defined stacks.  */
#define _POSIX_THREAD_ATTR_STACKADDR	200809L

/* We support priority inheritence.  */
#define _POSIX_THREAD_PRIO_INHERIT	200809L

/* We support priority protection, though only for non-robust
   mutexes.  */
#define _POSIX_THREAD_PRIO_PROTECT	200809L

#ifdef __USE_XOPEN2K8
/* We support priority inheritence for robust mutexes.  */
# define _POSIX_THREAD_ROBUST_PRIO_INHERIT	200809L

/* We do not support priority protection for robust mutexes.  */
# define _POSIX_THREAD_ROBUST_PRIO_PROTECT	-1
#endif

/* We support POSIX.1b semaphores.  */
#define _POSIX_SEMAPHORES	200809L

/* Real-time signals are supported.  */
#define _POSIX_REALTIME_SIGNALS	200809L

/* We support asynchronous I/O.  */
#define _POSIX_ASYNCHRONOUS_IO	200809L
#define _POSIX_ASYNC_IO		1
/* Alternative name for Unix98.  */
#define _LFS_ASYNCHRONOUS_IO	1
/* Support for prioritization is also available.  */
#define _POSIX_PRIORITIZED_IO	200809L

/* The LFS support in asynchronous I/O is also available.  */
#define _LFS64_ASYNCHRONOUS_IO	1

/* The rest of the LFS is also available.  */
#define _LFS_LARGEFILE		1
#define _LFS64_LARGEFILE	1
#define _LFS64_STDIO		1

/* POSIX shared memory objects are implemented.  */
#define _POSIX_SHARED_MEMORY_OBJECTS	200809L

/* CPU-time clocks support needs to be checked at runtime.  */
#define _POSIX_CPUTIME	0

/* Clock support in threads must be also checked at runtime.  */
#define _POSIX_THREAD_CPUTIME	0

/* GNU libc provides regular expression handling.  */
#define _POSIX_REGEXP	1

/* Reader/Writer locks are available.  */
#define _POSIX_READER_WRITER_LOCKS	200809L

/* We have a POSIX shell.  */
#define _POSIX_SHELL	1

/* We support the Timeouts option.  */
#define _POSIX_TIMEOUTS	200809L

/* We support spinlocks.  */
#define _POSIX_SPIN_LOCKS	200809L

/* The `spawn' function family is supported.  */
#define _POSIX_SPAWN	200809L

/* We have POSIX timers.  */
#define _POSIX_TIMERS	200809L

/* The barrier functions are available.  */
#define _POSIX_BARRIERS	200809L

/* POSIX message queues are available.  */
#define	_POSIX_MESSAGE_PASSING	200809L

/* Thread process-shared synchronization is supported.  */
#define _POSIX_THREAD_PROCESS_SHARED	200809L

/* The monotonic clock might be available.  */
#define _POSIX_MONOTONIC_CLOCK	0

/* The clock selection interfaces are available.  */
#define _POSIX_CLOCK_SELECTION	200809L

/* Advisory information interfaces are available.  */
#define _POSIX_ADVISORY_INFO	200809L

/* IPv6 support is available.  */
#define _POSIX_IPV6	200809L

/* Raw socket support is available.  */
#define _POSIX_RAW_SOCKETS	200809L

/* We have at least one terminal.  */
#define _POSIX2_CHAR_TERM	200809L

/* Neither process nor thread sporadic server interfaces is available.  */
#define _POSIX_SPORADIC_SERVER	-1
#define _POSIX_THREAD_SPORADIC_SERVER	-1

/* trace.h is not available.  */
#define _POSIX_TRACE	-1
#define _POSIX_TRACE_EVENT_FILTER	-1
#define _POSIX_TRACE_INHERIT	-1
#define _POSIX_TRACE_LOG	-1

/* Typed memory objects are not available.  */
#define _POSIX_TYPED_MEMORY_OBJECTS	-1

#endif /* bits/posix_opt.h */
