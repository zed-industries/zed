/* Definitions for POSIX 1003.1b-1993 (aka POSIX.4) scheduling interface.
   Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

#ifndef	_SCHED_H
#define	_SCHED_H	1

#include <features.h>

/* Get type definitions.  */
#include <bits/types.h>

#define __need_size_t
#define __need_NULL
#include <stddef.h>

#include <bits/types/time_t.h>
#include <bits/types/struct_timespec.h>
#ifndef __USE_XOPEN2K
# include <time.h>
#endif

#ifndef __pid_t_defined
typedef __pid_t pid_t;
# define __pid_t_defined
#endif

/* Get system specific constant and data structure definitions.  */
#include <bits/sched.h>
#include <bits/cpu-set.h>

/* Backward compatibility.  */
#define sched_priority    sched_priority
#define __sched_priority  sched_priority


__BEGIN_DECLS

/* Set scheduling parameters for a process.  */
extern int sched_setparam (__pid_t __pid, const struct sched_param *__param)
     __THROW;

/* Retrieve scheduling parameters for a particular process.  */
extern int sched_getparam (__pid_t __pid, struct sched_param *__param) __THROW;

/* Set scheduling algorithm and/or parameters for a process.  */
extern int sched_setscheduler (__pid_t __pid, int __policy,
			       const struct sched_param *__param) __THROW;

/* Retrieve scheduling algorithm for a particular purpose.  */
extern int sched_getscheduler (__pid_t __pid) __THROW;

/* Yield the processor.  */
extern int sched_yield (void) __THROW;

/* Get maximum priority value for a scheduler.  */
extern int sched_get_priority_max (int __algorithm) __THROW;

/* Get minimum priority value for a scheduler.  */
extern int sched_get_priority_min (int __algorithm) __THROW;

/* Get the SCHED_RR interval for the named process.  */
extern int sched_rr_get_interval (__pid_t __pid, struct timespec *__t) __THROW;


#ifdef __USE_GNU
/* Access macros for `cpu_set'.  */
# define CPU_SETSIZE __CPU_SETSIZE
# define CPU_SET(cpu, cpusetp)	 __CPU_SET_S (cpu, sizeof (cpu_set_t), cpusetp)
# define CPU_CLR(cpu, cpusetp)	 __CPU_CLR_S (cpu, sizeof (cpu_set_t), cpusetp)
# define CPU_ISSET(cpu, cpusetp) __CPU_ISSET_S (cpu, sizeof (cpu_set_t), \
						cpusetp)
# define CPU_ZERO(cpusetp)	 __CPU_ZERO_S (sizeof (cpu_set_t), cpusetp)
# define CPU_COUNT(cpusetp)	 __CPU_COUNT_S (sizeof (cpu_set_t), cpusetp)

# define CPU_SET_S(cpu, setsize, cpusetp)   __CPU_SET_S (cpu, setsize, cpusetp)
# define CPU_CLR_S(cpu, setsize, cpusetp)   __CPU_CLR_S (cpu, setsize, cpusetp)
# define CPU_ISSET_S(cpu, setsize, cpusetp) __CPU_ISSET_S (cpu, setsize, \
							   cpusetp)
# define CPU_ZERO_S(setsize, cpusetp)	    __CPU_ZERO_S (setsize, cpusetp)
# define CPU_COUNT_S(setsize, cpusetp)	    __CPU_COUNT_S (setsize, cpusetp)

# define CPU_EQUAL(cpusetp1, cpusetp2) \
  __CPU_EQUAL_S (sizeof (cpu_set_t), cpusetp1, cpusetp2)
# define CPU_EQUAL_S(setsize, cpusetp1, cpusetp2) \
  __CPU_EQUAL_S (setsize, cpusetp1, cpusetp2)

# define CPU_AND(destset, srcset1, srcset2) \
  __CPU_OP_S (sizeof (cpu_set_t), destset, srcset1, srcset2, &)
# define CPU_OR(destset, srcset1, srcset2) \
  __CPU_OP_S (sizeof (cpu_set_t), destset, srcset1, srcset2, |)
# define CPU_XOR(destset, srcset1, srcset2) \
  __CPU_OP_S (sizeof (cpu_set_t), destset, srcset1, srcset2, ^)
# define CPU_AND_S(setsize, destset, srcset1, srcset2) \
  __CPU_OP_S (setsize, destset, srcset1, srcset2, &)
# define CPU_OR_S(setsize, destset, srcset1, srcset2) \
  __CPU_OP_S (setsize, destset, srcset1, srcset2, |)
# define CPU_XOR_S(setsize, destset, srcset1, srcset2) \
  __CPU_OP_S (setsize, destset, srcset1, srcset2, ^)

# define CPU_ALLOC_SIZE(count) __CPU_ALLOC_SIZE (count)
# define CPU_ALLOC(count) __CPU_ALLOC (count)
# define CPU_FREE(cpuset) __CPU_FREE (cpuset)


/* Set the CPU affinity for a task */
extern int sched_setaffinity (__pid_t __pid, size_t __cpusetsize,
			      const cpu_set_t *__cpuset) __THROW;

/* Get the CPU affinity for a task */
extern int sched_getaffinity (__pid_t __pid, size_t __cpusetsize,
			      cpu_set_t *__cpuset) __THROW;
#endif

__END_DECLS

#endif /* sched.h */
