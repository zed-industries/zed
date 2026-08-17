/* Signal number definitions.  Linux version.
   Copyright (C) 1995-2020 Free Software Foundation, Inc.
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

#ifndef _BITS_SIGNUM_H
#define _BITS_SIGNUM_H 1

#ifndef _SIGNAL_H
#error "Never include <bits/signum.h> directly; use <signal.h> instead."
#endif

#include <bits/signum-generic.h>

/* Adjustments and additions to the signal number constants for
   most Linux systems.  */

#define	SIGSTKFLT	16	/* Stack fault (obsolete).  */
#define	SIGPWR		30	/* Power failure imminent.  */

#undef	SIGBUS
#define	SIGBUS		 7
#undef	SIGUSR1
#define	SIGUSR1		10
#undef	SIGUSR2
#define	SIGUSR2		12
#undef	SIGCHLD
#define	SIGCHLD		17
#undef	SIGCONT
#define	SIGCONT		18
#undef	SIGSTOP
#define	SIGSTOP		19
#undef	SIGTSTP
#define	SIGTSTP		20
#undef	SIGURG
#define	SIGURG		23
#undef	SIGPOLL
#define	SIGPOLL		29
#undef	SIGSYS
#define SIGSYS		31

#undef	__SIGRTMAX
#define __SIGRTMAX	64

#endif	/* <signal.h> included.  */
