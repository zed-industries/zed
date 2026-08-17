/* Copyright (C) 2001-2020 Free Software Foundation, Inc.
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

#ifndef _PROFIL_H
#define _PROFIL_H	1

#include <features.h>

#include <sys/time.h>
#include <sys/types.h>

/* This interface is intended to follow the sprofil() system calls as
   described by the sprofil(2) man page of Irix v6.5, except that:

	- there is no a priori limit on number of text sections
	- pr_scale is declared as unsigned long (instead of "unsigned int")
	- pr_size is declared as size_t (instead of "unsigned int")
	- pr_off is declared as void * (instead of "__psunsigned_t")
	- the overflow bin (pr_base==0, pr_scale==2) can appear anywhere
	  in the profp array
	- PROF_FAST has no effect  */

struct prof
  {
    void *pr_base;		/* buffer base */
    size_t pr_size;		/* buffer size */
    size_t pr_off;		/* pc offset */
    unsigned long int pr_scale;	/* pc scaling (fixed-point number) */
  };

enum
  {
    PROF_USHORT	= 0,		/* use 16-bit counters (default) */
    PROF_UINT	= 1 << 0,	/* use 32-bit counters */
    PROF_FAST   = 1 << 1	/* profile faster than usual */
  };


__BEGIN_DECLS

extern int sprofil (struct prof *__profp, int __profcnt,
		    struct timeval *__tvp, unsigned int __flags) __THROW;

__END_DECLS

#endif /* profil.h */
