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

/*
 *	POSIX Standard: 7.1-2 General Terminal Interface	<termios.h>
 */

#ifndef	_TERMIOS_H
#define	_TERMIOS_H	1

#include <features.h>
#if defined __USE_XOPEN_EXTENDED || defined __USE_XOPEN2K8
/* We need `pid_t'.  */
# include <bits/types.h>
# ifndef __pid_t_defined
typedef __pid_t pid_t;
#  define __pid_t_defined
# endif
#endif

__BEGIN_DECLS

/* Get the system-dependent definitions of `struct termios', `tcflag_t',
   `cc_t', `speed_t', and all the macros specifying the flag bits.  */
#include <bits/termios.h>

#ifdef __USE_MISC
/* Compare a character C to a value VAL from the `c_cc' array in a
   `struct termios'.  If VAL is _POSIX_VDISABLE, no character can match it.  */
# define CCEQ(val, c)	((c) == (val) && (val) != _POSIX_VDISABLE)
#endif

/* Return the output baud rate stored in *TERMIOS_P.  */
extern speed_t cfgetospeed (const struct termios *__termios_p) __THROW;

/* Return the input baud rate stored in *TERMIOS_P.  */
extern speed_t cfgetispeed (const struct termios *__termios_p) __THROW;

/* Set the output baud rate stored in *TERMIOS_P to SPEED.  */
extern int cfsetospeed (struct termios *__termios_p, speed_t __speed) __THROW;

/* Set the input baud rate stored in *TERMIOS_P to SPEED.  */
extern int cfsetispeed (struct termios *__termios_p, speed_t __speed) __THROW;

#ifdef	__USE_MISC
/* Set both the input and output baud rates in *TERMIOS_OP to SPEED.  */
extern int cfsetspeed (struct termios *__termios_p, speed_t __speed) __THROW;
#endif


/* Put the state of FD into *TERMIOS_P.  */
extern int tcgetattr (int __fd, struct termios *__termios_p) __THROW;

/* Set the state of FD to *TERMIOS_P.
   Values for OPTIONAL_ACTIONS (TCSA*) are in <bits/termios.h>.  */
extern int tcsetattr (int __fd, int __optional_actions,
		      const struct termios *__termios_p) __THROW;


#ifdef	__USE_MISC
/* Set *TERMIOS_P to indicate raw mode.  */
extern void cfmakeraw (struct termios *__termios_p) __THROW;
#endif

/* Send zero bits on FD.  */
extern int tcsendbreak (int __fd, int __duration) __THROW;

/* Wait for pending output to be written on FD.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
extern int tcdrain (int __fd);

/* Flush pending data on FD.
   Values for QUEUE_SELECTOR (TC{I,O,IO}FLUSH) are in <bits/termios.h>.  */
extern int tcflush (int __fd, int __queue_selector) __THROW;

/* Suspend or restart transmission on FD.
   Values for ACTION (TC[IO]{OFF,ON}) are in <bits/termios.h>.  */
extern int tcflow (int __fd, int __action) __THROW;


#if defined __USE_XOPEN_EXTENDED || defined __USE_XOPEN2K8
/* Get process group ID for session leader for controlling terminal FD.  */
extern __pid_t tcgetsid (int __fd) __THROW;
#endif


#ifdef __USE_MISC
# include <sys/ttydefaults.h>
#endif

__END_DECLS

#endif /* termios.h  */
