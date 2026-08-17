/* Copyright (C) 1991-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_VTIMES_H
#define _SYS_VTIMES_H	1

#include <features.h>

__BEGIN_DECLS

/* This interface is obsolete; use `getrusage' instead.  */

/* Granularity of the `vm_utime' and `vm_stime' fields of a `struct vtimes'.
   (This is the frequency of the machine's power supply, in Hz.)  */
#define	VTIMES_UNITS_PER_SECOND	60

struct vtimes
{
  /* User time used in units of 1/VTIMES_UNITS_PER_SECOND seconds.  */
  int vm_utime;
  /* System time used in units of 1/VTIMES_UNITS_PER_SECOND seconds.  */
  int vm_stime;

  /* Amount of data and stack memory used (kilobyte-seconds).  */
  unsigned int vm_idsrss;
  /* Amount of text memory used (kilobyte-seconds).  */
  unsigned int vm_ixrss;
  /* Maximum resident set size (text, data, and stack) (kilobytes).  */
  int vm_maxrss;

  /* Number of hard page faults (i.e. those that required I/O).  */
  int vm_majflt;
  /* Number of soft page faults (i.e. those serviced by reclaiming
     a page from the list of pages awaiting reallocation.  */
  int vm_minflt;

  /* Number of times a process was swapped out of physical memory.  */
  int vm_nswap;

  /* Number of input operations via the file system.  Note: This
     and `ru_oublock' do not include operations with the cache.  */
  int vm_inblk;
  /* Number of output operations via the file system.  */
  int vm_oublk;
};

/* If CURRENT is not NULL, write statistics for the current process into
   *CURRENT.  If CHILD is not NULL, write statistics for all terminated child
   processes into *CHILD.  Returns 0 for success, -1 for failure.  */
extern int vtimes (struct vtimes * __current, struct vtimes * __child) __THROW;

__END_DECLS

#endif /* sys/vtimes.h  */
