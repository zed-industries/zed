/* termios output mode definitions.  Linux/generic version.
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
# error "Never include <bits/termios-c_oflag.h> directly; use <termios.h> instead."
#endif

/* c_oflag bits */
#define OPOST	0000001  /* Post-process output.  */
#define OLCUC	0000002  /* Map lowercase characters to uppercase on output.
			    (not in POSIX).  */
#define ONLCR	0000004  /* Map NL to CR-NL on output.  */
#define OCRNL	0000010  /* Map CR to NL on output.  */
#define ONOCR	0000020  /* No CR output at column 0.  */
#define ONLRET	0000040  /* NL performs CR function.  */
#define OFILL	0000100  /* Use fill characters for delay.  */
#define OFDEL	0000200  /* Fill is DEL.  */
#if defined __USE_MISC || defined __USE_XOPEN
# define NLDLY	0000400  /* Select newline delays:  */
# define   NL0	0000000  /* Newline type 0.  */
# define   NL1	0000400  /* Newline type 1.  */
# define CRDLY	0003000  /* Select carriage-return delays:  */
# define   CR0	0000000  /* Carriage-return delay type 0.  */
# define   CR1	0001000  /* Carriage-return delay type 1.  */
# define   CR2	0002000  /* Carriage-return delay type 2.  */
# define   CR3	0003000  /* Carriage-return delay type 3.  */
# define TABDLY	0014000  /* Select horizontal-tab delays:  */
# define   TAB0	0000000  /* Horizontal-tab delay type 0.  */
# define   TAB1	0004000  /* Horizontal-tab delay type 1.  */
# define   TAB2	0010000  /* Horizontal-tab delay type 2.  */
# define   TAB3	0014000  /* Expand tabs to spaces.  */
# define BSDLY	0020000  /* Select backspace delays:  */
# define   BS0	0000000  /* Backspace-delay type 0.  */
# define   BS1	0020000  /* Backspace-delay type 1.  */
# define FFDLY	0100000  /* Select form-feed delays:  */
# define   FF0	0000000  /* Form-feed delay type 0.  */
# define   FF1	0100000  /* Form-feed delay type 1.  */
#endif

#define VTDLY	0040000  /* Select vertical-tab delays:  */
#define   VT0	0000000  /* Vertical-tab delay type 0.  */
#define   VT1	0040000  /* Vertical-tab delay type 1.  */

#ifdef __USE_MISC
# define XTABS	0014000
#endif
