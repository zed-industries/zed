/* Copyright (C) 1992-2020 Free Software Foundation, Inc.
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

#ifndef	_SYS_RESOURCE_H
#define	_SYS_RESOURCE_H	1

#include <features.h>

/* Get the system-dependent definitions of structures and bit values.  */
#include <bits/resource.h>

#ifndef __id_t_defined
typedef __id_t id_t;
# define __id_t_defined
#endif

__BEGIN_DECLS

/* The X/Open standard defines that all the functions below must use
   `int' as the type for the first argument.  When we are compiling with
   GNU extensions we change this slightly to provide better error
   checking.  */
#if defined __USE_GNU && !defined __cplusplus
typedef enum __rlimit_resource __rlimit_resource_t;
typedef enum __rusage_who __rusage_who_t;
typedef enum __priority_which __priority_which_t;
#else
typedef int __rlimit_resource_t;
typedef int __rusage_who_t;
typedef int __priority_which_t;
#endif

/* Put the soft and hard limits for RESOURCE in *RLIMITS.
   Returns 0 if successful, -1 if not (and sets errno).  */
#ifndef __USE_FILE_OFFSET64
extern int getrlimit (__rlimit_resource_t __resource,
		      struct rlimit *__rlimits) __THROW;
#else
# ifdef __REDIRECT_NTH
extern int __REDIRECT_NTH (getrlimit, (__rlimit_resource_t __resource,
				       struct rlimit *__rlimits), getrlimit64);
# else
#  define getrlimit getrlimit64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int getrlimit64 (__rlimit_resource_t __resource,
			struct rlimit64 *__rlimits) __THROW;
#endif

/* Set the soft and hard limits for RESOURCE to *RLIMITS.
   Only the super-user can increase hard limits.
   Return 0 if successful, -1 if not (and sets errno).  */
#ifndef __USE_FILE_OFFSET64
extern int setrlimit (__rlimit_resource_t __resource,
		      const struct rlimit *__rlimits) __THROW;
#else
# ifdef __REDIRECT_NTH
extern int __REDIRECT_NTH (setrlimit, (__rlimit_resource_t __resource,
				       const struct rlimit *__rlimits),
			   setrlimit64);
# else
#  define setrlimit setrlimit64
# endif
#endif
#ifdef __USE_LARGEFILE64
extern int setrlimit64 (__rlimit_resource_t __resource,
			const struct rlimit64 *__rlimits) __THROW;
#endif

/* Return resource usage information on process indicated by WHO
   and put it in *USAGE.  Returns 0 for success, -1 for failure.  */
extern int getrusage (__rusage_who_t __who, struct rusage *__usage) __THROW;

/* Return the highest priority of any process specified by WHICH and WHO
   (see above); if WHO is zero, the current process, process group, or user
   (as specified by WHO) is used.  A lower priority number means higher
   priority.  Priorities range from PRIO_MIN to PRIO_MAX (above).  */
extern int getpriority (__priority_which_t __which, id_t __who) __THROW;

/* Set the priority of all processes specified by WHICH and WHO (see above)
   to PRIO.  Returns 0 on success, -1 on errors.  */
extern int setpriority (__priority_which_t __which, id_t __who, int __prio)
     __THROW;

__END_DECLS

#endif	/* sys/resource.h  */
