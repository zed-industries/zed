/* Copyright (C) 1996-2020 Free Software Foundation, Inc.
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
 *	ISO C99 Standard: 7.25
 *	Wide character classification and mapping utilities  <wctype.h>
 */

#ifndef _BITS_WCTYPE_WCHAR_H
#define _BITS_WCTYPE_WCHAR_H 1

#if !defined _WCTYPE_H && !defined _WCHAR_H
#error "Never include <bits/wctype-wchar.h> directly; include <wctype.h> or <wchar.h> instead."
#endif

#include <bits/types.h>
#include <bits/types/wint_t.h>

/* The definitions in this header are specified to appear in <wctype.h>
   in ISO C99, but in <wchar.h> in Unix98.  _GNU_SOURCE follows C99.  */

/* Scalar type that can hold values which represent locale-specific
   character classifications.  */
typedef unsigned long int wctype_t;

# ifndef _ISwbit
/* The characteristics are stored always in network byte order (big
   endian).  We define the bit value interpretations here dependent on the
   machine's byte order.  */

#  include <bits/endian.h>
#  if __BYTE_ORDER == __BIG_ENDIAN
#   define _ISwbit(bit)	(1 << (bit))
#  else /* __BYTE_ORDER == __LITTLE_ENDIAN */
#   define _ISwbit(bit)	\
	((bit) < 8 ? (int) ((1UL << (bit)) << 24)			      \
	 : ((bit) < 16 ? (int) ((1UL << (bit)) << 8)			      \
	    : ((bit) < 24 ? (int) ((1UL << (bit)) >> 8)			      \
	       : (int) ((1UL << (bit)) >> 24))))
#  endif

enum
{
  __ISwupper = 0,			/* UPPERCASE.  */
  __ISwlower = 1,			/* lowercase.  */
  __ISwalpha = 2,			/* Alphabetic.  */
  __ISwdigit = 3,			/* Numeric.  */
  __ISwxdigit = 4,			/* Hexadecimal numeric.  */
  __ISwspace = 5,			/* Whitespace.  */
  __ISwprint = 6,			/* Printing.  */
  __ISwgraph = 7,			/* Graphical.  */
  __ISwblank = 8,			/* Blank (usually SPC and TAB).  */
  __ISwcntrl = 9,			/* Control character.  */
  __ISwpunct = 10,			/* Punctuation.  */
  __ISwalnum = 11,			/* Alphanumeric.  */

  _ISwupper = _ISwbit (__ISwupper),	/* UPPERCASE.  */
  _ISwlower = _ISwbit (__ISwlower),	/* lowercase.  */
  _ISwalpha = _ISwbit (__ISwalpha),	/* Alphabetic.  */
  _ISwdigit = _ISwbit (__ISwdigit),	/* Numeric.  */
  _ISwxdigit = _ISwbit (__ISwxdigit),	/* Hexadecimal numeric.  */
  _ISwspace = _ISwbit (__ISwspace),	/* Whitespace.  */
  _ISwprint = _ISwbit (__ISwprint),	/* Printing.  */
  _ISwgraph = _ISwbit (__ISwgraph),	/* Graphical.  */
  _ISwblank = _ISwbit (__ISwblank),	/* Blank (usually SPC and TAB).  */
  _ISwcntrl = _ISwbit (__ISwcntrl),	/* Control character.  */
  _ISwpunct = _ISwbit (__ISwpunct),	/* Punctuation.  */
  _ISwalnum = _ISwbit (__ISwalnum)	/* Alphanumeric.  */
};
# endif /* Not _ISwbit  */


__BEGIN_DECLS

/*
 * Wide-character classification functions: 7.15.2.1.
 */

/* Test for any wide character for which `iswalpha' or `iswdigit' is
   true.  */
extern int iswalnum (wint_t __wc) __THROW;

/* Test for any wide character for which `iswupper' or 'iswlower' is
   true, or any wide character that is one of a locale-specific set of
   wide-characters for which none of `iswcntrl', `iswdigit',
   `iswpunct', or `iswspace' is true.  */
extern int iswalpha (wint_t __wc) __THROW;

/* Test for any control wide character.  */
extern int iswcntrl (wint_t __wc) __THROW;

/* Test for any wide character that corresponds to a decimal-digit
   character.  */
extern int iswdigit (wint_t __wc) __THROW;

/* Test for any wide character for which `iswprint' is true and
   `iswspace' is false.  */
extern int iswgraph (wint_t __wc) __THROW;

/* Test for any wide character that corresponds to a lowercase letter
   or is one of a locale-specific set of wide characters for which
   none of `iswcntrl', `iswdigit', `iswpunct', or `iswspace' is true.  */
extern int iswlower (wint_t __wc) __THROW;

/* Test for any printing wide character.  */
extern int iswprint (wint_t __wc) __THROW;

/* Test for any printing wide character that is one of a
   locale-specific et of wide characters for which neither `iswspace'
   nor `iswalnum' is true.  */
extern int iswpunct (wint_t __wc) __THROW;

/* Test for any wide character that corresponds to a locale-specific
   set of wide characters for which none of `iswalnum', `iswgraph', or
   `iswpunct' is true.  */
extern int iswspace (wint_t __wc) __THROW;

/* Test for any wide character that corresponds to an uppercase letter
   or is one of a locale-specific set of wide character for which none
   of `iswcntrl', `iswdigit', `iswpunct', or `iswspace' is true.  */
extern int iswupper (wint_t __wc) __THROW;

/* Test for any wide character that corresponds to a hexadecimal-digit
   character equivalent to that performed be the functions described
   in the previous subclause.  */
extern int iswxdigit (wint_t __wc) __THROW;

/* Test for any wide character that corresponds to a standard blank
   wide character or a locale-specific set of wide characters for
   which `iswalnum' is false.  */
# ifdef __USE_ISOC99
extern int iswblank (wint_t __wc) __THROW;
# endif

/*
 * Extensible wide-character classification functions: 7.15.2.2.
 */

/* Construct value that describes a class of wide characters identified
   by the string argument PROPERTY.  */
extern wctype_t wctype (const char *__property) __THROW;

/* Determine whether the wide-character WC has the property described by
   DESC.  */
extern int iswctype (wint_t __wc, wctype_t __desc) __THROW;

/*
 * Wide-character case-mapping functions: 7.15.3.1.
 */

/* Converts an uppercase letter to the corresponding lowercase letter.  */
extern wint_t towlower (wint_t __wc) __THROW;

/* Converts an lowercase letter to the corresponding uppercase letter.  */
extern wint_t towupper (wint_t __wc) __THROW;

__END_DECLS

#endif /* bits/wctype-wchar.h.  */
