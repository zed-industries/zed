/* termios baud rate selection definitions.  Linux/generic version.
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
# error "Never include <bits/termios-baud.h> directly; use <termios.h> instead."
#endif

#ifdef __USE_MISC
# define CBAUD	 000000010017 /* Baud speed mask (not in POSIX).  */
# define CBAUDEX 000000010000 /* Extra baud speed mask, included in CBAUD.
				 (not in POSIX).  */
# define CIBAUD	 002003600000 /* Input baud rate (not used).  */
# define CMSPAR  010000000000 /* Mark or space (stick) parity.  */
# define CRTSCTS 020000000000 /* Flow control.  */
#endif

/* Extra output baud rates (not in POSIX).  */
#define  B57600    0010001
#define  B115200   0010002
#define  B230400   0010003
#define  B460800   0010004
#define  B500000   0010005
#define  B576000   0010006
#define  B921600   0010007
#define  B1000000  0010010
#define  B1152000  0010011
#define  B1500000  0010012
#define  B2000000  0010013
#define  B2500000  0010014
#define  B3000000  0010015
#define  B3500000  0010016
#define  B4000000  0010017
#define __MAX_BAUD B4000000
