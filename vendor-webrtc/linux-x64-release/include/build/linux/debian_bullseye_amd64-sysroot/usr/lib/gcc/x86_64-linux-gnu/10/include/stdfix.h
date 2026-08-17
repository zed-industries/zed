/* Copyright (C) 2007-2020 Free Software Foundation, Inc.

This file is part of GCC.

GCC is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3, or (at your option)
any later version.

GCC is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

Under Section 7 of GPL version 3, you are granted additional
permissions described in the GCC Runtime Library Exception, version
3.1, as published by the Free Software Foundation.

You should have received a copy of the GNU General Public License and
a copy of the GCC Runtime Library Exception along with this program;
see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
<http://www.gnu.org/licenses/>.  */

/* ISO/IEC JTC1 SC22 WG14 N1169
 * Date: 2006-04-04
 * ISO/IEC TR 18037
 * Programming languages - C - Extensions to support embedded processors
 */

#ifndef _STDFIX_H
#define _STDFIX_H

/* 7.18a.1 Introduction.  */

#undef fract
#undef accum
#undef sat
#define fract		_Fract
#define accum		_Accum
#define sat		_Sat

/* 7.18a.3 Precision macros.  */

#undef SFRACT_FBIT
#undef SFRACT_MIN
#undef SFRACT_MAX
#undef SFRACT_EPSILON
#define SFRACT_FBIT	__SFRACT_FBIT__
#define SFRACT_MIN	__SFRACT_MIN__
#define SFRACT_MAX	__SFRACT_MAX__
#define SFRACT_EPSILON	__SFRACT_EPSILON__

#undef USFRACT_FBIT
#undef USFRACT_MIN
#undef USFRACT_MAX
#undef USFRACT_EPSILON
#define USFRACT_FBIT	__USFRACT_FBIT__
#define USFRACT_MIN	__USFRACT_MIN__		/* GCC extension.  */
#define USFRACT_MAX	__USFRACT_MAX__
#define USFRACT_EPSILON	__USFRACT_EPSILON__

#undef FRACT_FBIT
#undef FRACT_MIN
#undef FRACT_MAX
#undef FRACT_EPSILON
#define FRACT_FBIT	__FRACT_FBIT__
#define FRACT_MIN	__FRACT_MIN__
#define FRACT_MAX	__FRACT_MAX__
#define FRACT_EPSILON	__FRACT_EPSILON__

#undef UFRACT_FBIT
#undef UFRACT_MIN
#undef UFRACT_MAX
#undef UFRACT_EPSILON
#define UFRACT_FBIT	__UFRACT_FBIT__
#define UFRACT_MIN	__UFRACT_MIN__		/* GCC extension.  */
#define UFRACT_MAX	__UFRACT_MAX__
#define UFRACT_EPSILON	__UFRACT_EPSILON__

#undef LFRACT_FBIT
#undef LFRACT_MIN
#undef LFRACT_MAX
#undef LFRACT_EPSILON
#define LFRACT_FBIT	__LFRACT_FBIT__
#define LFRACT_MIN	__LFRACT_MIN__
#define LFRACT_MAX	__LFRACT_MAX__
#define LFRACT_EPSILON	__LFRACT_EPSILON__

#undef ULFRACT_FBIT
#undef ULFRACT_MIN
#undef ULFRACT_MAX
#undef ULFRACT_EPSILON
#define ULFRACT_FBIT	__ULFRACT_FBIT__
#define ULFRACT_MIN	__ULFRACT_MIN__		/* GCC extension.  */
#define ULFRACT_MAX	__ULFRACT_MAX__
#define ULFRACT_EPSILON	__ULFRACT_EPSILON__

#undef LLFRACT_FBIT
#undef LLFRACT_MIN
#undef LLFRACT_MAX
#undef LLFRACT_EPSILON
#define LLFRACT_FBIT	__LLFRACT_FBIT__	/* GCC extension.  */
#define LLFRACT_MIN	__LLFRACT_MIN__		/* GCC extension.  */
#define LLFRACT_MAX	__LLFRACT_MAX__		/* GCC extension.  */
#define LLFRACT_EPSILON	__LLFRACT_EPSILON__	/* GCC extension.  */

#undef ULLFRACT_FBIT
#undef ULLFRACT_MIN
#undef ULLFRACT_MAX
#undef ULLFRACT_EPSILON
#define ULLFRACT_FBIT	__ULLFRACT_FBIT__	/* GCC extension.  */
#define ULLFRACT_MIN	__ULLFRACT_MIN__	/* GCC extension.  */
#define ULLFRACT_MAX	__ULLFRACT_MAX__	/* GCC extension.  */
#define ULLFRACT_EPSILON	__ULLFRACT_EPSILON__	/* GCC extension.  */

#undef SACCUM_FBIT
#undef SACCUM_IBIT
#undef SACCUM_MIN
#undef SACCUM_MAX
#undef SACCUM_EPSILON
#define SACCUM_FBIT	__SACCUM_FBIT__
#define SACCUM_IBIT	__SACCUM_IBIT__
#define SACCUM_MIN	__SACCUM_MIN__
#define SACCUM_MAX	__SACCUM_MAX__
#define SACCUM_EPSILON	__SACCUM_EPSILON__

#undef USACCUM_FBIT
#undef USACCUM_IBIT
#undef USACCUM_MIN
#undef USACCUM_MAX
#undef USACCUM_EPSILON
#define USACCUM_FBIT	__USACCUM_FBIT__
#define USACCUM_IBIT	__USACCUM_IBIT__
#define USACCUM_MIN	__USACCUM_MIN__		/* GCC extension.  */
#define USACCUM_MAX	__USACCUM_MAX__
#define USACCUM_EPSILON	__USACCUM_EPSILON__

#undef ACCUM_FBIT
#undef ACCUM_IBIT
#undef ACCUM_MIN
#undef ACCUM_MAX
#undef ACCUM_EPSILON
#define ACCUM_FBIT	__ACCUM_FBIT__
#define ACCUM_IBIT	__ACCUM_IBIT__
#define ACCUM_MIN	__ACCUM_MIN__
#define ACCUM_MAX	__ACCUM_MAX__
#define ACCUM_EPSILON	__ACCUM_EPSILON__

#undef UACCUM_FBIT
#undef UACCUM_IBIT
#undef UACCUM_MIN
#undef UACCUM_MAX
#undef UACCUM_EPSILON
#define UACCUM_FBIT	__UACCUM_FBIT__
#define UACCUM_IBIT	__UACCUM_IBIT__
#define UACCUM_MIN	__UACCUM_MIN__		/* GCC extension.  */
#define UACCUM_MAX	__UACCUM_MAX__
#define UACCUM_EPSILON	__UACCUM_EPSILON__

#undef LACCUM_FBIT
#undef LACCUM_IBIT
#undef LACCUM_MIN
#undef LACCUM_MAX
#undef LACCUM_EPSILON
#define LACCUM_FBIT	__LACCUM_FBIT__
#define LACCUM_IBIT	__LACCUM_IBIT__
#define LACCUM_MIN	__LACCUM_MIN__
#define LACCUM_MAX	__LACCUM_MAX__
#define LACCUM_EPSILON	__LACCUM_EPSILON__

#undef ULACCUM_FBIT
#undef ULACCUM_IBIT
#undef ULACCUM_MIN
#undef ULACCUM_MAX
#undef ULACCUM_EPSILON
#define ULACCUM_FBIT	__ULACCUM_FBIT__
#define ULACCUM_IBIT	__ULACCUM_IBIT__
#define ULACCUM_MIN	__ULACCUM_MIN__		/* GCC extension.  */
#define ULACCUM_MAX	__ULACCUM_MAX__
#define ULACCUM_EPSILON	__ULACCUM_EPSILON__

#undef LLACCUM_FBIT
#undef LLACCUM_IBIT
#undef LLACCUM_MIN
#undef LLACCUM_MAX
#undef LLACCUM_EPSILON
#define LLACCUM_FBIT	__LLACCUM_FBIT__	/* GCC extension.  */
#define LLACCUM_IBIT	__LLACCUM_IBIT__	/* GCC extension.  */
#define LLACCUM_MIN	__LLACCUM_MIN__		/* GCC extension.  */
#define LLACCUM_MAX	__LLACCUM_MAX__		/* GCC extension.  */
#define LLACCUM_EPSILON	__LLACCUM_EPSILON__	/* GCC extension.  */

#undef ULLACCUM_FBIT
#undef ULLACCUM_IBIT
#undef ULLACCUM_MIN
#undef ULLACCUM_MAX
#undef ULLACCUM_EPSILON
#define ULLACCUM_FBIT	__ULLACCUM_FBIT__	/* GCC extension.  */
#define ULLACCUM_IBIT	__ULLACCUM_IBIT__	/* GCC extension.  */
#define ULLACCUM_MIN	__ULLACCUM_MIN__	/* GCC extension.  */
#define ULLACCUM_MAX	__ULLACCUM_MAX__	/* GCC extension.  */
#define ULLACCUM_EPSILON	__ULLACCUM_EPSILON__	/* GCC extension.  */

#endif /* _STDFIX_H */
