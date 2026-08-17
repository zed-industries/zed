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

#ifndef _WCTYPE_H
#define _WCTYPE_H 1

#include <features.h>
#include <bits/types.h>
#include <bits/types/wint_t.h>

/* Constant expression of type `wint_t' whose value does not correspond
   to any member of the extended character set.  */
#ifndef WEOF
# define WEOF (0xffffffffu)
#endif

/* Some definitions from this header also appear in <wchar.h> in
   Unix98 mode.  */
#include <bits/wctype-wchar.h>

/*
 * Extensible wide-character mapping functions: 7.15.3.2.
 */

__BEGIN_DECLS

/* Scalar type that can hold values which represent locale-specific
   character mappings.  */
typedef const __int32_t *wctrans_t;

/* Construct value that describes a mapping between wide characters
   identified by the string argument PROPERTY.  */
extern wctrans_t wctrans (const char *__property) __THROW;

/* Map the wide character WC using the mapping described by DESC.  */
extern wint_t towctrans (wint_t __wc, wctrans_t __desc) __THROW;

# ifdef __USE_XOPEN2K8
/* POSIX.1-2008 extended locale interface (see locale.h).  */
#  include <bits/types/locale_t.h>

/* Test for any wide character for which `iswalpha' or `iswdigit' is
   true.  */
extern int iswalnum_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character for which `iswupper' or 'iswlower' is
   true, or any wide character that is one of a locale-specific set of
   wide-characters for which none of `iswcntrl', `iswdigit',
   `iswpunct', or `iswspace' is true.  */
extern int iswalpha_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any control wide character.  */
extern int iswcntrl_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character that corresponds to a decimal-digit
   character.  */
extern int iswdigit_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character for which `iswprint' is true and
   `iswspace' is false.  */
extern int iswgraph_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character that corresponds to a lowercase letter
   or is one of a locale-specific set of wide characters for which
   none of `iswcntrl', `iswdigit', `iswpunct', or `iswspace' is true.  */
extern int iswlower_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any printing wide character.  */
extern int iswprint_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any printing wide character that is one of a
   locale-specific et of wide characters for which neither `iswspace'
   nor `iswalnum' is true.  */
extern int iswpunct_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character that corresponds to a locale-specific
   set of wide characters for which none of `iswalnum', `iswgraph', or
   `iswpunct' is true.  */
extern int iswspace_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character that corresponds to an uppercase letter
   or is one of a locale-specific set of wide character for which none
   of `iswcntrl', `iswdigit', `iswpunct', or `iswspace' is true.  */
extern int iswupper_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character that corresponds to a hexadecimal-digit
   character equivalent to that performed be the functions described
   in the previous subclause.  */
extern int iswxdigit_l (wint_t __wc, locale_t __locale) __THROW;

/* Test for any wide character that corresponds to a standard blank
   wide character or a locale-specific set of wide characters for
   which `iswalnum' is false.  */
extern int iswblank_l (wint_t __wc, locale_t __locale) __THROW;

/* Construct value that describes a class of wide characters identified
   by the string argument PROPERTY.  */
extern wctype_t wctype_l (const char *__property, locale_t __locale)
     __THROW;

/* Determine whether the wide-character WC has the property described by
   DESC.  */
extern int iswctype_l (wint_t __wc, wctype_t __desc, locale_t __locale)
     __THROW;

/*
 * Wide-character case-mapping functions.
 */

/* Converts an uppercase letter to the corresponding lowercase letter.  */
extern wint_t towlower_l (wint_t __wc, locale_t __locale) __THROW;

/* Converts an lowercase letter to the corresponding uppercase letter.  */
extern wint_t towupper_l (wint_t __wc, locale_t __locale) __THROW;

/* Construct value that describes a mapping between wide characters
   identified by the string argument PROPERTY.  */
extern wctrans_t wctrans_l (const char *__property, locale_t __locale)
     __THROW;

/* Map the wide character WC using the mapping described by DESC.  */
extern wint_t towctrans_l (wint_t __wc, wctrans_t __desc,
			   locale_t __locale) __THROW;

# endif /* Use POSIX 2008.  */

__END_DECLS

#endif /* wctype.h  */
