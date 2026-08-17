/* termios local mode definitions.  Linux/generic version.
   Copyright (C) 2019-2020 Free Software Foundation, Inc.
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
   License along with the GNU C Library.  If not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _TERMIOS_H
# error "Never include <bits/termios-c_lflag.h> directly; use <termios.h> instead."
#endif

/* c_lflag bits */
#define ISIG	0000001   /* Enable signals.  */
#define ICANON	0000002   /* Canonical input (erase and kill processing).  */
#if defined __USE_MISC || (defined __USE_XOPEN && !defined __USE_XOPEN2K)
# define XCASE	0000004
#endif
#define ECHO	0000010   /* Enable echo.  */
#define ECHOE	0000020   /* Echo erase character as error-correcting
			     backspace.  */
#define ECHOK	0000040   /* Echo KILL.  */
#define ECHONL	0000100   /* Echo NL.  */
#define NOFLSH	0000200   /* Disable flush after interrupt or quit.  */
#define TOSTOP	0000400   /* Send SIGTTOU for background output.  */
#ifdef __USE_MISC
# define ECHOCTL 0001000  /* If ECHO is also set, terminal special characters
			     other than TAB, NL, START, and STOP are echoed as
			     ^X, where X is the character with ASCII code 0x40
			     greater than the special character
			     (not in POSIX).  */
# define ECHOPRT 0002000  /* If ICANON and ECHO are also set, characters are
			     printed as they are being erased
			     (not in POSIX).  */
# define ECHOKE	 0004000  /* If ICANON is also set, KILL is echoed by erasing
			     each character on the line, as specified by ECHOE
			     and ECHOPRT (not in POSIX).  */
# define FLUSHO	 0010000  /* Output is being flushed.  This flag is toggled by
			     typing the DISCARD character (not in POSIX).  */
# define PENDIN	 0040000  /* All characters in the input queue are reprinted
			     when the next character is read
			     (not in POSIX).  */
#endif
#define IEXTEN	0100000   /* Enable implementation-defined input
			     processing.  */
#ifdef __USE_MISC
# define EXTPROC 0200000
#endif
