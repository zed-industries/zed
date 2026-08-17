/* Copyright (C) 1995-2020 Free Software Foundation, Inc.
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

#ifndef	_BITS_TIMEX_H
#define	_BITS_TIMEX_H	1

#include <bits/types.h>
#include <bits/types/struct_timeval.h>

/* These definitions from linux/timex.h as of 3.18.  */

struct timex
{
  unsigned int modes;		/* mode selector */
  __syscall_slong_t offset;	/* time offset (usec) */
  __syscall_slong_t freq;	/* frequency offset (scaled ppm) */
  __syscall_slong_t maxerror;	/* maximum error (usec) */
  __syscall_slong_t esterror;	/* estimated error (usec) */
  int status;			/* clock command/status */
  __syscall_slong_t constant;	/* pll time constant */
  __syscall_slong_t precision;	/* clock precision (usec) (ro) */
  __syscall_slong_t tolerance;	/* clock frequency tolerance (ppm) (ro) */
  struct timeval time;		/* (read only, except for ADJ_SETOFFSET) */
  __syscall_slong_t tick;	/* (modified) usecs between clock ticks */
  __syscall_slong_t ppsfreq;	/* pps frequency (scaled ppm) (ro) */
  __syscall_slong_t jitter;	/* pps jitter (us) (ro) */
  int shift;			/* interval duration (s) (shift) (ro) */
  __syscall_slong_t stabil;	/* pps stability (scaled ppm) (ro) */
  __syscall_slong_t jitcnt;	/* jitter limit exceeded (ro) */
  __syscall_slong_t calcnt;	/* calibration intervals (ro) */
  __syscall_slong_t errcnt;	/* calibration errors (ro) */
  __syscall_slong_t stbcnt;	/* stability limit exceeded (ro) */

  int tai;			/* TAI offset (ro) */

  /* ??? */
  int  :32; int  :32; int  :32; int  :32;
  int  :32; int  :32; int  :32; int  :32;
  int  :32; int  :32; int  :32;
};

/* Mode codes (timex.mode) */
#define ADJ_OFFSET		0x0001	/* time offset */
#define ADJ_FREQUENCY		0x0002	/* frequency offset */
#define ADJ_MAXERROR		0x0004	/* maximum time error */
#define ADJ_ESTERROR		0x0008	/* estimated time error */
#define ADJ_STATUS		0x0010	/* clock status */
#define ADJ_TIMECONST		0x0020	/* pll time constant */
#define ADJ_TAI			0x0080	/* set TAI offset */
#define ADJ_SETOFFSET		0x0100	/* add 'time' to current time */
#define ADJ_MICRO		0x1000	/* select microsecond resolution */
#define ADJ_NANO		0x2000	/* select nanosecond resolution */
#define ADJ_TICK		0x4000	/* tick value */
#define ADJ_OFFSET_SINGLESHOT	0x8001	/* old-fashioned adjtime */
#define ADJ_OFFSET_SS_READ	0xa001	/* read-only adjtime */

/* xntp 3.4 compatibility names */
#define MOD_OFFSET	ADJ_OFFSET
#define MOD_FREQUENCY	ADJ_FREQUENCY
#define MOD_MAXERROR	ADJ_MAXERROR
#define MOD_ESTERROR	ADJ_ESTERROR
#define MOD_STATUS	ADJ_STATUS
#define MOD_TIMECONST	ADJ_TIMECONST
#define MOD_CLKB	ADJ_TICK
#define MOD_CLKA	ADJ_OFFSET_SINGLESHOT /* 0x8000 in original */
#define MOD_TAI		ADJ_TAI
#define MOD_MICRO	ADJ_MICRO
#define MOD_NANO	ADJ_NANO


/* Status codes (timex.status) */
#define STA_PLL		0x0001	/* enable PLL updates (rw) */
#define STA_PPSFREQ	0x0002	/* enable PPS freq discipline (rw) */
#define STA_PPSTIME	0x0004	/* enable PPS time discipline (rw) */
#define STA_FLL		0x0008	/* select frequency-lock mode (rw) */

#define STA_INS		0x0010	/* insert leap (rw) */
#define STA_DEL		0x0020	/* delete leap (rw) */
#define STA_UNSYNC	0x0040	/* clock unsynchronized (rw) */
#define STA_FREQHOLD	0x0080	/* hold frequency (rw) */

#define STA_PPSSIGNAL	0x0100	/* PPS signal present (ro) */
#define STA_PPSJITTER	0x0200	/* PPS signal jitter exceeded (ro) */
#define STA_PPSWANDER	0x0400	/* PPS signal wander exceeded (ro) */
#define STA_PPSERROR	0x0800	/* PPS signal calibration error (ro) */

#define STA_CLOCKERR	0x1000	/* clock hardware fault (ro) */
#define STA_NANO	0x2000	/* resolution (0 = us, 1 = ns) (ro) */
#define STA_MODE	0x4000	/* mode (0 = PLL, 1 = FLL) (ro) */
#define STA_CLK		0x8000	/* clock source (0 = A, 1 = B) (ro) */

/* Read-only bits */
#define STA_RONLY (STA_PPSSIGNAL | STA_PPSJITTER | STA_PPSWANDER \
    | STA_PPSERROR | STA_CLOCKERR | STA_NANO | STA_MODE | STA_CLK)

#endif /* bits/timex.h */
