/* Old compatibility names for <limits.h> and <float.h> constants.
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

/* This interface is obsolete.  New programs should use
   <limits.h> and/or <float.h> instead of <values.h>.  */

#ifndef	_VALUES_H
#define	_VALUES_H	1

#include <features.h>

#include <limits.h>

#define _TYPEBITS(type)	(sizeof (type) * CHAR_BIT)

#define CHARBITS	_TYPEBITS (char)
#define SHORTBITS	_TYPEBITS (short int)
#define INTBITS		_TYPEBITS (int)
#define LONGBITS	_TYPEBITS (long int)
#define PTRBITS		_TYPEBITS (char *)
#define DOUBLEBITS	_TYPEBITS (double)
#define FLOATBITS	_TYPEBITS (float)

#define MINSHORT	SHRT_MIN
#define	MININT		INT_MIN
#define	MINLONG		LONG_MIN

#define	MAXSHORT	SHRT_MAX
#define	MAXINT		INT_MAX
#define	MAXLONG		LONG_MAX

#define HIBITS		MINSHORT
#define HIBITL		MINLONG


#include <float.h>

#define	MAXDOUBLE	DBL_MAX
#define	MAXFLOAT	FLT_MAX
#define	MINDOUBLE	DBL_MIN
#define	MINFLOAT	FLT_MIN
#define	DMINEXP		DBL_MIN_EXP
#define	FMINEXP		FLT_MIN_EXP
#define	DMAXEXP		DBL_MAX_EXP
#define	FMAXEXP		FLT_MAX_EXP


#ifdef __USE_MISC
/* Some systems define this name instead of CHAR_BIT or CHARBITS.  */
# define BITSPERBYTE	CHAR_BIT
#endif

#endif	/* values.h */
