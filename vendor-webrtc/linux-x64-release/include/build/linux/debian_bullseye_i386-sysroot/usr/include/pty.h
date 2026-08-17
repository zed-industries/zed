/* Functions for pseudo TTY handling.
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

#ifndef _PTY_H
#define _PTY_H	1

#include <features.h>

struct termios;
struct winsize;

#include <termios.h>
#include <sys/ioctl.h>


__BEGIN_DECLS

/* Create pseudo tty master slave pair with NAME and set terminal
   attributes according to TERMP and WINP and return handles for both
   ends in AMASTER and ASLAVE.  */
extern int openpty (int *__amaster, int *__aslave, char *__name,
		    const struct termios *__termp,
		    const struct winsize *__winp) __THROW;

/* Create child process and establish the slave pseudo terminal as the
   child's controlling terminal.  */
extern int forkpty (int *__amaster, char *__name,
		    const struct termios *__termp,
		    const struct winsize *__winp) __THROW;

__END_DECLS

#endif	/* pty.h */
