/* Structure types for pre-termios terminal ioctls.  Linux version.
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

#ifndef _SYS_IOCTL_H
# error "Never use <bits/ioctl-types.h> directly; include <sys/ioctl.h> instead."
#endif

/* Get definition of constants for use with `ioctl'.  */
#include <asm/ioctls.h>


struct winsize
  {
    unsigned short int ws_row;
    unsigned short int ws_col;
    unsigned short int ws_xpixel;
    unsigned short int ws_ypixel;
  };

#define NCC 8
struct termio
  {
    unsigned short int c_iflag;		/* input mode flags */
    unsigned short int c_oflag;		/* output mode flags */
    unsigned short int c_cflag;		/* control mode flags */
    unsigned short int c_lflag;		/* local mode flags */
    unsigned char c_line;		/* line discipline */
    unsigned char c_cc[NCC];		/* control characters */
};

/* modem lines */
#define TIOCM_LE	0x001
#define TIOCM_DTR	0x002
#define TIOCM_RTS	0x004
#define TIOCM_ST	0x008
#define TIOCM_SR	0x010
#define TIOCM_CTS	0x020
#define TIOCM_CAR	0x040
#define TIOCM_RNG	0x080
#define TIOCM_DSR	0x100
#define TIOCM_CD	TIOCM_CAR
#define TIOCM_RI	TIOCM_RNG

/* ioctl (fd, TIOCSERGETLSR, &result) where result may be as below */

/* line disciplines */
#define N_TTY		0
#define N_SLIP		1
#define N_MOUSE		2
#define N_PPP		3
#define N_STRIP		4
#define N_AX25		5
#define N_X25		6	/* X.25 async  */
#define N_6PACK		7
#define N_MASC		8	/* Mobitex module  */
#define N_R3964		9	/* Simatic R3964 module  */
#define N_PROFIBUS_FDL	10	/* Profibus  */
#define N_IRDA		11	/* Linux IR  */
#define N_SMSBLOCK	12	/* SMS block mode  */
#define N_HDLC		13	/* synchronous HDLC  */
#define N_SYNC_PPP	14	/* synchronous PPP  */
#define	N_HCI		15	/* Bluetooth HCI UART  */
