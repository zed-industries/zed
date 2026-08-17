/* Structures and definitions for the user accounting database.  GNU version.
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

#ifndef _UTMPX_H
# error "Never include <bits/utmpx.h> directly; use <utmpx.h> instead."
#endif

#include <bits/types.h>
#include <sys/time.h>
#include <bits/wordsize.h>


#ifdef __USE_GNU
# include <paths.h>
# define _PATH_UTMPX	_PATH_UTMP
# define _PATH_WTMPX	_PATH_WTMP
#endif


#define __UT_LINESIZE	32
#define __UT_NAMESIZE	32
#define __UT_HOSTSIZE	256


/* The structure describing the status of a terminated process.  This
   type is used in `struct utmpx' below.  */
struct __exit_status
  {
#ifdef __USE_GNU
    short int e_termination;	/* Process termination status.  */
    short int e_exit;		/* Process exit status.  */
#else
    short int __e_termination;	/* Process termination status.  */
    short int __e_exit;		/* Process exit status.  */
#endif
  };


/* The structure describing an entry in the user accounting database.  */
struct utmpx
{
  short int ut_type;		/* Type of login.  */
  __pid_t ut_pid;		/* Process ID of login process.  */
  char ut_line[__UT_LINESIZE]
    __attribute_nonstring__;	/* Devicename.  */
  char ut_id[4]
    __attribute_nonstring__;	/* Inittab ID.  */
  char ut_user[__UT_NAMESIZE]
    __attribute_nonstring__;	/* Username.  */
  char ut_host[__UT_HOSTSIZE]
    __attribute_nonstring__;	/* Hostname for remote login.  */
  struct __exit_status ut_exit;	/* Exit status of a process marked
				   as DEAD_PROCESS.  */

/* The fields ut_session and ut_tv must be the same size when compiled
   32- and 64-bit.  This allows files and shared memory to be shared
   between 32- and 64-bit applications.  */
#if __WORDSIZE_TIME64_COMPAT32
  __int32_t ut_session;		/* Session ID, used for windowing.  */
  struct
  {
    __int32_t tv_sec;		/* Seconds.  */
    __int32_t tv_usec;		/* Microseconds.  */
  } ut_tv;			/* Time entry was made.  */
#else
  long int ut_session;		/* Session ID, used for windowing.  */
  struct timeval ut_tv;		/* Time entry was made.  */
#endif
  __int32_t ut_addr_v6[4];	/* Internet address of remote host.  */
  char __glibc_reserved[20];		/* Reserved for future use.  */
};


/* Values for the `ut_type' field of a `struct utmpx'.  */
#define EMPTY		0	/* No valid user accounting information.  */

#ifdef __USE_GNU
# define RUN_LVL	1	/* The system's runlevel.  */
#endif
#define BOOT_TIME	2	/* Time of system boot.  */
#define NEW_TIME	3	/* Time after system clock changed.  */
#define OLD_TIME	4	/* Time when system clock changed.  */

#define INIT_PROCESS	5	/* Process spawned by the init process.  */
#define LOGIN_PROCESS	6	/* Session leader of a logged in user.  */
#define USER_PROCESS	7	/* Normal process.  */
#define DEAD_PROCESS	8	/* Terminated process.  */

#ifdef __USE_GNU
# define ACCOUNTING	9	/* System accounting.  */
#endif
