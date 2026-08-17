/* Access to locale-dependent parameters.
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

#ifndef _LANGINFO_H
#define	_LANGINFO_H 1

/* Get the type definition.  */
#include <nl_types.h>

#include <bits/locale.h>	/* Define the __LC_* category names.  */


__BEGIN_DECLS

/* Construct an `nl_item' value for `nl_langinfo' from a locale category
   (LC_*) and an item index within the category.  Some code may depend on
   the item values within a category increasing monotonically with the
   indices.  */
#define _NL_ITEM(category, index)	(((category) << 16) | (index))

/* Extract the category and item index from a constructed `nl_item' value.  */
#define _NL_ITEM_CATEGORY(item)		((int) (item) >> 16)
#define _NL_ITEM_INDEX(item)		((int) (item) & 0xffff)

/* Enumeration of locale items that can be queried with `nl_langinfo'.  */
enum
{
  /* LC_TIME category: date and time formatting.  */

  /* Abbreviated days of the week. */
  ABDAY_1 = _NL_ITEM (__LC_TIME, 0), /* Sun */
#define ABDAY_1			ABDAY_1
  ABDAY_2,
#define ABDAY_2			ABDAY_2
  ABDAY_3,
#define ABDAY_3			ABDAY_3
  ABDAY_4,
#define ABDAY_4			ABDAY_4
  ABDAY_5,
#define ABDAY_5			ABDAY_5
  ABDAY_6,
#define ABDAY_6			ABDAY_6
  ABDAY_7,
#define ABDAY_7			ABDAY_7

  /* Long-named days of the week. */
  DAY_1,			/* Sunday */
#define DAY_1			DAY_1
  DAY_2,			/* Monday */
#define DAY_2			DAY_2
  DAY_3,			/* Tuesday */
#define DAY_3			DAY_3
  DAY_4,			/* Wednesday */
#define DAY_4			DAY_4
  DAY_5,			/* Thursday */
#define DAY_5			DAY_5
  DAY_6,			/* Friday */
#define DAY_6			DAY_6
  DAY_7,			/* Saturday */
#define DAY_7			DAY_7

  /* Abbreviated month names, in the grammatical form used when the month
     is a part of a complete date.  */
  ABMON_1,			/* Jan */
#define ABMON_1			ABMON_1
  ABMON_2,
#define ABMON_2			ABMON_2
  ABMON_3,
#define ABMON_3			ABMON_3
  ABMON_4,
#define ABMON_4			ABMON_4
  ABMON_5,
#define ABMON_5			ABMON_5
  ABMON_6,
#define ABMON_6			ABMON_6
  ABMON_7,
#define ABMON_7			ABMON_7
  ABMON_8,
#define ABMON_8			ABMON_8
  ABMON_9,
#define ABMON_9			ABMON_9
  ABMON_10,
#define ABMON_10		ABMON_10
  ABMON_11,
#define ABMON_11		ABMON_11
  ABMON_12,
#define ABMON_12		ABMON_12

  /* Long month names, in the grammatical form used when the month
     is a part of a complete date.  */
  MON_1,			/* January */
#define MON_1			MON_1
  MON_2,
#define MON_2			MON_2
  MON_3,
#define MON_3			MON_3
  MON_4,
#define MON_4			MON_4
  MON_5,
#define MON_5			MON_5
  MON_6,
#define MON_6			MON_6
  MON_7,
#define MON_7			MON_7
  MON_8,
#define MON_8			MON_8
  MON_9,
#define MON_9			MON_9
  MON_10,
#define MON_10			MON_10
  MON_11,
#define MON_11			MON_11
  MON_12,
#define MON_12			MON_12

  AM_STR,			/* Ante meridiem string.  */
#define AM_STR			AM_STR
  PM_STR,			/* Post meridiem string.  */
#define PM_STR			PM_STR

  D_T_FMT,			/* Date and time format for strftime.  */
#define D_T_FMT			D_T_FMT
  D_FMT,			/* Date format for strftime.  */
#define D_FMT			D_FMT
  T_FMT,			/* Time format for strftime.  */
#define T_FMT			T_FMT
  T_FMT_AMPM,			/* 12-hour time format for strftime.  */
#define T_FMT_AMPM		T_FMT_AMPM

  ERA,				/* Alternate era.  */
#define ERA			ERA
  __ERA_YEAR,			/* Year in alternate era format.  */
#ifdef __USE_GNU
# define ERA_YEAR		__ERA_YEAR
#endif
  ERA_D_FMT,			/* Date in alternate era format.  */
#define ERA_D_FMT		ERA_D_FMT
  ALT_DIGITS,			/* Alternate symbols for digits.  */
#define ALT_DIGITS		ALT_DIGITS
  ERA_D_T_FMT,			/* Date and time in alternate era format.  */
#define ERA_D_T_FMT		ERA_D_T_FMT
  ERA_T_FMT,			/* Time in alternate era format.  */
#define ERA_T_FMT		ERA_T_FMT

  _NL_TIME_ERA_NUM_ENTRIES,	/* Number entries in the era arrays.  */
  _NL_TIME_ERA_ENTRIES,		/* Structure with era entries in usable form.*/

  _NL_WABDAY_1,		/* Sun */
  _NL_WABDAY_2,
  _NL_WABDAY_3,
  _NL_WABDAY_4,
  _NL_WABDAY_5,
  _NL_WABDAY_6,
  _NL_WABDAY_7,

  /* Long-named days of the week. */
  _NL_WDAY_1,		/* Sunday */
  _NL_WDAY_2,		/* Monday */
  _NL_WDAY_3,		/* Tuesday */
  _NL_WDAY_4,		/* Wednesday */
  _NL_WDAY_5,		/* Thursday */
  _NL_WDAY_6,		/* Friday */
  _NL_WDAY_7,		/* Saturday */

  /* Abbreviated month names, in the grammatical form used when the month
     is a part of a complete date.  */
  _NL_WABMON_1,		/* Jan */
  _NL_WABMON_2,
  _NL_WABMON_3,
  _NL_WABMON_4,
  _NL_WABMON_5,
  _NL_WABMON_6,
  _NL_WABMON_7,
  _NL_WABMON_8,
  _NL_WABMON_9,
  _NL_WABMON_10,
  _NL_WABMON_11,
  _NL_WABMON_12,

  /* Long month names, in the grammatical form used when the month
     is a part of a complete date.  */
  _NL_WMON_1,		/* January */
  _NL_WMON_2,
  _NL_WMON_3,
  _NL_WMON_4,
  _NL_WMON_5,
  _NL_WMON_6,
  _NL_WMON_7,
  _NL_WMON_8,
  _NL_WMON_9,
  _NL_WMON_10,
  _NL_WMON_11,
  _NL_WMON_12,

  _NL_WAM_STR,		/* Ante meridiem string.  */
  _NL_WPM_STR,		/* Post meridiem string.  */

  _NL_WD_T_FMT,		/* Date and time format for strftime.  */
  _NL_WD_FMT,		/* Date format for strftime.  */
  _NL_WT_FMT,		/* Time format for strftime.  */
  _NL_WT_FMT_AMPM,	/* 12-hour time format for strftime.  */

  _NL_WERA_YEAR,	/* Year in alternate era format.  */
  _NL_WERA_D_FMT,	/* Date in alternate era format.  */
  _NL_WALT_DIGITS,	/* Alternate symbols for digits.  */
  _NL_WERA_D_T_FMT,	/* Date and time in alternate era format.  */
  _NL_WERA_T_FMT,	/* Time in alternate era format.  */

  _NL_TIME_WEEK_NDAYS,
  _NL_TIME_WEEK_1STDAY,
  _NL_TIME_WEEK_1STWEEK,
  _NL_TIME_FIRST_WEEKDAY,
  _NL_TIME_FIRST_WORKDAY,
  _NL_TIME_CAL_DIRECTION,
  _NL_TIME_TIMEZONE,

  _DATE_FMT,		/* strftime format for date.  */
#define _DATE_FMT	_DATE_FMT
  _NL_W_DATE_FMT,

  _NL_TIME_CODESET,

  /* Long month names, in the grammatical form used when the month
     is named by itself.  */
  __ALTMON_1,			/* January */
  __ALTMON_2,
  __ALTMON_3,
  __ALTMON_4,
  __ALTMON_5,
  __ALTMON_6,
  __ALTMON_7,
  __ALTMON_8,
  __ALTMON_9,
  __ALTMON_10,
  __ALTMON_11,
  __ALTMON_12,
#ifdef __USE_GNU
# define ALTMON_1		__ALTMON_1
# define ALTMON_2		__ALTMON_2
# define ALTMON_3		__ALTMON_3
# define ALTMON_4		__ALTMON_4
# define ALTMON_5		__ALTMON_5
# define ALTMON_6		__ALTMON_6
# define ALTMON_7		__ALTMON_7
# define ALTMON_8		__ALTMON_8
# define ALTMON_9		__ALTMON_9
# define ALTMON_10		__ALTMON_10
# define ALTMON_11		__ALTMON_11
# define ALTMON_12		__ALTMON_12
#endif

  /* Long month names, in the grammatical form used when the month
     is named by itself.  */
  _NL_WALTMON_1,			/* January */
  _NL_WALTMON_2,
  _NL_WALTMON_3,
  _NL_WALTMON_4,
  _NL_WALTMON_5,
  _NL_WALTMON_6,
  _NL_WALTMON_7,
  _NL_WALTMON_8,
  _NL_WALTMON_9,
  _NL_WALTMON_10,
  _NL_WALTMON_11,
  _NL_WALTMON_12,

  /* Abbreviated month names, in the grammatical form used when the month
     is named by itself.  */
  _NL_ABALTMON_1,			/* Jan */
  _NL_ABALTMON_2,
  _NL_ABALTMON_3,
  _NL_ABALTMON_4,
  _NL_ABALTMON_5,
  _NL_ABALTMON_6,
  _NL_ABALTMON_7,
  _NL_ABALTMON_8,
  _NL_ABALTMON_9,
  _NL_ABALTMON_10,
  _NL_ABALTMON_11,
  _NL_ABALTMON_12,

  /* Abbreviated month names, in the grammatical form used when the month
     is named by itself.  */
  _NL_WABALTMON_1,			/* Jan */
  _NL_WABALTMON_2,
  _NL_WABALTMON_3,
  _NL_WABALTMON_4,
  _NL_WABALTMON_5,
  _NL_WABALTMON_6,
  _NL_WABALTMON_7,
  _NL_WABALTMON_8,
  _NL_WABALTMON_9,
  _NL_WABALTMON_10,
  _NL_WABALTMON_11,
  _NL_WABALTMON_12,

  _NL_NUM_LC_TIME,	/* Number of indices in LC_TIME category.  */

  /* LC_COLLATE category: text sorting.
     This information is accessed by the strcoll and strxfrm functions.
     These `nl_langinfo' names are used only internally.  */
  _NL_COLLATE_NRULES = _NL_ITEM (__LC_COLLATE, 0),
  _NL_COLLATE_RULESETS,
  _NL_COLLATE_TABLEMB,
  _NL_COLLATE_WEIGHTMB,
  _NL_COLLATE_EXTRAMB,
  _NL_COLLATE_INDIRECTMB,
  _NL_COLLATE_GAP1,
  _NL_COLLATE_GAP2,
  _NL_COLLATE_GAP3,
  _NL_COLLATE_TABLEWC,
  _NL_COLLATE_WEIGHTWC,
  _NL_COLLATE_EXTRAWC,
  _NL_COLLATE_INDIRECTWC,
  _NL_COLLATE_SYMB_HASH_SIZEMB,
  _NL_COLLATE_SYMB_TABLEMB,
  _NL_COLLATE_SYMB_EXTRAMB,
  _NL_COLLATE_COLLSEQMB,
  _NL_COLLATE_COLLSEQWC,
  _NL_COLLATE_CODESET,
  _NL_NUM_LC_COLLATE,

  /* LC_CTYPE category: character classification.
     This information is accessed by the functions in <ctype.h>.
     These `nl_langinfo' names are used only internally.  */
  _NL_CTYPE_CLASS = _NL_ITEM (__LC_CTYPE, 0),
  _NL_CTYPE_TOUPPER,
  _NL_CTYPE_GAP1,
  _NL_CTYPE_TOLOWER,
  _NL_CTYPE_GAP2,
  _NL_CTYPE_CLASS32,
  _NL_CTYPE_GAP3,
  _NL_CTYPE_GAP4,
  _NL_CTYPE_GAP5,
  _NL_CTYPE_GAP6,
  _NL_CTYPE_CLASS_NAMES,
  _NL_CTYPE_MAP_NAMES,
  _NL_CTYPE_WIDTH,
  _NL_CTYPE_MB_CUR_MAX,
  _NL_CTYPE_CODESET_NAME,
  CODESET = _NL_CTYPE_CODESET_NAME,
#define CODESET			CODESET
  _NL_CTYPE_TOUPPER32,
  _NL_CTYPE_TOLOWER32,
  _NL_CTYPE_CLASS_OFFSET,
  _NL_CTYPE_MAP_OFFSET,
  _NL_CTYPE_INDIGITS_MB_LEN,
  _NL_CTYPE_INDIGITS0_MB,
  _NL_CTYPE_INDIGITS1_MB,
  _NL_CTYPE_INDIGITS2_MB,
  _NL_CTYPE_INDIGITS3_MB,
  _NL_CTYPE_INDIGITS4_MB,
  _NL_CTYPE_INDIGITS5_MB,
  _NL_CTYPE_INDIGITS6_MB,
  _NL_CTYPE_INDIGITS7_MB,
  _NL_CTYPE_INDIGITS8_MB,
  _NL_CTYPE_INDIGITS9_MB,
  _NL_CTYPE_INDIGITS_WC_LEN,
  _NL_CTYPE_INDIGITS0_WC,
  _NL_CTYPE_INDIGITS1_WC,
  _NL_CTYPE_INDIGITS2_WC,
  _NL_CTYPE_INDIGITS3_WC,
  _NL_CTYPE_INDIGITS4_WC,
  _NL_CTYPE_INDIGITS5_WC,
  _NL_CTYPE_INDIGITS6_WC,
  _NL_CTYPE_INDIGITS7_WC,
  _NL_CTYPE_INDIGITS8_WC,
  _NL_CTYPE_INDIGITS9_WC,
  _NL_CTYPE_OUTDIGIT0_MB,
  _NL_CTYPE_OUTDIGIT1_MB,
  _NL_CTYPE_OUTDIGIT2_MB,
  _NL_CTYPE_OUTDIGIT3_MB,
  _NL_CTYPE_OUTDIGIT4_MB,
  _NL_CTYPE_OUTDIGIT5_MB,
  _NL_CTYPE_OUTDIGIT6_MB,
  _NL_CTYPE_OUTDIGIT7_MB,
  _NL_CTYPE_OUTDIGIT8_MB,
  _NL_CTYPE_OUTDIGIT9_MB,
  _NL_CTYPE_OUTDIGIT0_WC,
  _NL_CTYPE_OUTDIGIT1_WC,
  _NL_CTYPE_OUTDIGIT2_WC,
  _NL_CTYPE_OUTDIGIT3_WC,
  _NL_CTYPE_OUTDIGIT4_WC,
  _NL_CTYPE_OUTDIGIT5_WC,
  _NL_CTYPE_OUTDIGIT6_WC,
  _NL_CTYPE_OUTDIGIT7_WC,
  _NL_CTYPE_OUTDIGIT8_WC,
  _NL_CTYPE_OUTDIGIT9_WC,
  _NL_CTYPE_TRANSLIT_TAB_SIZE,
  _NL_CTYPE_TRANSLIT_FROM_IDX,
  _NL_CTYPE_TRANSLIT_FROM_TBL,
  _NL_CTYPE_TRANSLIT_TO_IDX,
  _NL_CTYPE_TRANSLIT_TO_TBL,
  _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN,
  _NL_CTYPE_TRANSLIT_DEFAULT_MISSING,
  _NL_CTYPE_TRANSLIT_IGNORE_LEN,
  _NL_CTYPE_TRANSLIT_IGNORE,
  _NL_CTYPE_MAP_TO_NONASCII,
  _NL_CTYPE_NONASCII_CASE,
  _NL_CTYPE_EXTRA_MAP_1,
  _NL_CTYPE_EXTRA_MAP_2,
  _NL_CTYPE_EXTRA_MAP_3,
  _NL_CTYPE_EXTRA_MAP_4,
  _NL_CTYPE_EXTRA_MAP_5,
  _NL_CTYPE_EXTRA_MAP_6,
  _NL_CTYPE_EXTRA_MAP_7,
  _NL_CTYPE_EXTRA_MAP_8,
  _NL_CTYPE_EXTRA_MAP_9,
  _NL_CTYPE_EXTRA_MAP_10,
  _NL_CTYPE_EXTRA_MAP_11,
  _NL_CTYPE_EXTRA_MAP_12,
  _NL_CTYPE_EXTRA_MAP_13,
  _NL_CTYPE_EXTRA_MAP_14,
  _NL_NUM_LC_CTYPE,

  /* LC_MONETARY category: formatting of monetary quantities.
     These items each correspond to a member of `struct lconv',
     defined in <locale.h>.  */
  __INT_CURR_SYMBOL = _NL_ITEM (__LC_MONETARY, 0),
#ifdef __USE_GNU
# define INT_CURR_SYMBOL	__INT_CURR_SYMBOL
#endif
  __CURRENCY_SYMBOL,
#ifdef __USE_GNU
# define CURRENCY_SYMBOL	__CURRENCY_SYMBOL
#endif
  __MON_DECIMAL_POINT,
#ifdef __USE_GNU
# define MON_DECIMAL_POINT	__MON_DECIMAL_POINT
#endif
  __MON_THOUSANDS_SEP,
#ifdef __USE_GNU
# define MON_THOUSANDS_SEP	__MON_THOUSANDS_SEP
#endif
  __MON_GROUPING,
#ifdef __USE_GNU
# define MON_GROUPING		__MON_GROUPING
#endif
  __POSITIVE_SIGN,
#ifdef __USE_GNU
# define POSITIVE_SIGN		__POSITIVE_SIGN
#endif
  __NEGATIVE_SIGN,
#ifdef __USE_GNU
# define NEGATIVE_SIGN		__NEGATIVE_SIGN
#endif
  __INT_FRAC_DIGITS,
#ifdef __USE_GNU
# define INT_FRAC_DIGITS	__INT_FRAC_DIGITS
#endif
  __FRAC_DIGITS,
#ifdef __USE_GNU
# define FRAC_DIGITS		__FRAC_DIGITS
#endif
  __P_CS_PRECEDES,
#ifdef __USE_GNU
# define P_CS_PRECEDES		__P_CS_PRECEDES
#endif
  __P_SEP_BY_SPACE,
#ifdef __USE_GNU
# define P_SEP_BY_SPACE		__P_SEP_BY_SPACE
#endif
  __N_CS_PRECEDES,
#ifdef __USE_GNU
# define N_CS_PRECEDES		__N_CS_PRECEDES
#endif
  __N_SEP_BY_SPACE,
#ifdef __USE_GNU
# define N_SEP_BY_SPACE		__N_SEP_BY_SPACE
#endif
  __P_SIGN_POSN,
#ifdef __USE_GNU
# define P_SIGN_POSN		__P_SIGN_POSN
#endif
  __N_SIGN_POSN,
#ifdef __USE_GNU
# define N_SIGN_POSN		__N_SIGN_POSN
#endif
  _NL_MONETARY_CRNCYSTR,
#define CRNCYSTR		_NL_MONETARY_CRNCYSTR
  __INT_P_CS_PRECEDES,
#ifdef __USE_GNU
# define INT_P_CS_PRECEDES	__INT_P_CS_PRECEDES
#endif
  __INT_P_SEP_BY_SPACE,
#ifdef __USE_GNU
# define INT_P_SEP_BY_SPACE	__INT_P_SEP_BY_SPACE
#endif
  __INT_N_CS_PRECEDES,
#ifdef __USE_GNU
# define INT_N_CS_PRECEDES	__INT_N_CS_PRECEDES
#endif
  __INT_N_SEP_BY_SPACE,
#ifdef __USE_GNU
# define INT_N_SEP_BY_SPACE	__INT_N_SEP_BY_SPACE
#endif
  __INT_P_SIGN_POSN,
#ifdef __USE_GNU
# define INT_P_SIGN_POSN	__INT_P_SIGN_POSN
#endif
  __INT_N_SIGN_POSN,
#ifdef __USE_GNU
# define INT_N_SIGN_POSN	__INT_N_SIGN_POSN
#endif
  _NL_MONETARY_DUO_INT_CURR_SYMBOL,
  _NL_MONETARY_DUO_CURRENCY_SYMBOL,
  _NL_MONETARY_DUO_INT_FRAC_DIGITS,
  _NL_MONETARY_DUO_FRAC_DIGITS,
  _NL_MONETARY_DUO_P_CS_PRECEDES,
  _NL_MONETARY_DUO_P_SEP_BY_SPACE,
  _NL_MONETARY_DUO_N_CS_PRECEDES,
  _NL_MONETARY_DUO_N_SEP_BY_SPACE,
  _NL_MONETARY_DUO_INT_P_CS_PRECEDES,
  _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE,
  _NL_MONETARY_DUO_INT_N_CS_PRECEDES,
  _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE,
  _NL_MONETARY_DUO_P_SIGN_POSN,
  _NL_MONETARY_DUO_N_SIGN_POSN,
  _NL_MONETARY_DUO_INT_P_SIGN_POSN,
  _NL_MONETARY_DUO_INT_N_SIGN_POSN,
  _NL_MONETARY_UNO_VALID_FROM,
  _NL_MONETARY_UNO_VALID_TO,
  _NL_MONETARY_DUO_VALID_FROM,
  _NL_MONETARY_DUO_VALID_TO,
  _NL_MONETARY_CONVERSION_RATE,
  _NL_MONETARY_DECIMAL_POINT_WC,
  _NL_MONETARY_THOUSANDS_SEP_WC,
  _NL_MONETARY_CODESET,
  _NL_NUM_LC_MONETARY,

  /* LC_NUMERIC category: formatting of numbers.
     These also correspond to members of `struct lconv'; see <locale.h>.  */
  __DECIMAL_POINT = _NL_ITEM (__LC_NUMERIC, 0),
#ifdef __USE_GNU
# define DECIMAL_POINT		__DECIMAL_POINT
#endif
  RADIXCHAR = __DECIMAL_POINT,
#define RADIXCHAR		RADIXCHAR
  __THOUSANDS_SEP,
#ifdef __USE_GNU
# define THOUSANDS_SEP		__THOUSANDS_SEP
#endif
  THOUSEP = __THOUSANDS_SEP,
#define THOUSEP			THOUSEP
  __GROUPING,
#ifdef __USE_GNU
# define GROUPING		__GROUPING
#endif
  _NL_NUMERIC_DECIMAL_POINT_WC,
  _NL_NUMERIC_THOUSANDS_SEP_WC,
  _NL_NUMERIC_CODESET,
  _NL_NUM_LC_NUMERIC,

  __YESEXPR = _NL_ITEM (__LC_MESSAGES, 0), /* Regex matching ``yes'' input.  */
#define YESEXPR			__YESEXPR
  __NOEXPR,			/* Regex matching ``no'' input.  */
#define NOEXPR			__NOEXPR
  __YESSTR,			/* Output string for ``yes''.  */
#if defined __USE_GNU || (defined __USE_XOPEN && !defined __USE_XOPEN2K)
# define YESSTR			__YESSTR
#endif
  __NOSTR,			/* Output string for ``no''.  */
#if defined __USE_GNU || (defined __USE_XOPEN && !defined __USE_XOPEN2K)
# define NOSTR			__NOSTR
#endif
  _NL_MESSAGES_CODESET,
  _NL_NUM_LC_MESSAGES,

  _NL_PAPER_HEIGHT = _NL_ITEM (__LC_PAPER, 0),
  _NL_PAPER_WIDTH,
  _NL_PAPER_CODESET,
  _NL_NUM_LC_PAPER,

  _NL_NAME_NAME_FMT = _NL_ITEM (__LC_NAME, 0),
  _NL_NAME_NAME_GEN,
  _NL_NAME_NAME_MR,
  _NL_NAME_NAME_MRS,
  _NL_NAME_NAME_MISS,
  _NL_NAME_NAME_MS,
  _NL_NAME_CODESET,
  _NL_NUM_LC_NAME,

  _NL_ADDRESS_POSTAL_FMT = _NL_ITEM (__LC_ADDRESS, 0),
  _NL_ADDRESS_COUNTRY_NAME,
  _NL_ADDRESS_COUNTRY_POST,
  _NL_ADDRESS_COUNTRY_AB2,
  _NL_ADDRESS_COUNTRY_AB3,
  _NL_ADDRESS_COUNTRY_CAR,
  _NL_ADDRESS_COUNTRY_NUM,
  _NL_ADDRESS_COUNTRY_ISBN,
  _NL_ADDRESS_LANG_NAME,
  _NL_ADDRESS_LANG_AB,
  _NL_ADDRESS_LANG_TERM,
  _NL_ADDRESS_LANG_LIB,
  _NL_ADDRESS_CODESET,
  _NL_NUM_LC_ADDRESS,

  _NL_TELEPHONE_TEL_INT_FMT = _NL_ITEM (__LC_TELEPHONE, 0),
  _NL_TELEPHONE_TEL_DOM_FMT,
  _NL_TELEPHONE_INT_SELECT,
  _NL_TELEPHONE_INT_PREFIX,
  _NL_TELEPHONE_CODESET,
  _NL_NUM_LC_TELEPHONE,

  _NL_MEASUREMENT_MEASUREMENT = _NL_ITEM (__LC_MEASUREMENT, 0),
  _NL_MEASUREMENT_CODESET,
  _NL_NUM_LC_MEASUREMENT,

  _NL_IDENTIFICATION_TITLE = _NL_ITEM (__LC_IDENTIFICATION, 0),
  _NL_IDENTIFICATION_SOURCE,
  _NL_IDENTIFICATION_ADDRESS,
  _NL_IDENTIFICATION_CONTACT,
  _NL_IDENTIFICATION_EMAIL,
  _NL_IDENTIFICATION_TEL,
  _NL_IDENTIFICATION_FAX,
  _NL_IDENTIFICATION_LANGUAGE,
  _NL_IDENTIFICATION_TERRITORY,
  _NL_IDENTIFICATION_AUDIENCE,
  _NL_IDENTIFICATION_APPLICATION,
  _NL_IDENTIFICATION_ABBREVIATION,
  _NL_IDENTIFICATION_REVISION,
  _NL_IDENTIFICATION_DATE,
  _NL_IDENTIFICATION_CATEGORY,
  _NL_IDENTIFICATION_CODESET,
  _NL_NUM_LC_IDENTIFICATION,

  /* This marks the highest value used.  */
  _NL_NUM
};

/* This macro produces an item you can pass to `nl_langinfo' or
   `nl_langinfo_l' to get the name of the locale in use for CATEGORY.  */
#define _NL_LOCALE_NAME(category)	_NL_ITEM ((category),		      \
						  _NL_ITEM_INDEX (-1))
#ifdef __USE_GNU
# define NL_LOCALE_NAME(category)	_NL_LOCALE_NAME (category)
#endif


/* Return the current locale's value for ITEM.
   If ITEM is invalid, an empty string is returned.

   The string returned will not change until `setlocale' is called;
   it is usually in read-only memory and cannot be modified.  */

extern char *nl_langinfo (nl_item __item) __THROW;


#ifdef __USE_XOPEN2K8
/* POSIX.1-2008 extended locale interface (see locale.h).  */
# include <bits/types/locale_t.h>

/* Just like nl_langinfo but get the information from the locale object L.  */
extern char *nl_langinfo_l (nl_item __item, locale_t __l);
#endif

__END_DECLS

#endif	/* langinfo.h */
