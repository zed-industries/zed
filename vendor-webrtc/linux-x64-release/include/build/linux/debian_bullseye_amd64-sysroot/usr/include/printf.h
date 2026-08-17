/* Copyright (C) 1991-2020 Free Software Foundation, Inc.
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

#ifndef	_PRINTF_H

#define	_PRINTF_H	1
#include <features.h>

__BEGIN_DECLS

#include <bits/types/FILE.h>

#define	__need_size_t
#define __need_wchar_t
#include <stddef.h>

#include <stdarg.h>


struct printf_info
{
  int prec;			/* Precision.  */
  int width;			/* Width.  */
  wchar_t spec;			/* Format letter.  */
  unsigned int is_long_double:1;/* L flag.  */
  unsigned int is_short:1;	/* h flag.  */
  unsigned int is_long:1;	/* l flag.  */
  unsigned int alt:1;		/* # flag.  */
  unsigned int space:1;		/* Space flag.  */
  unsigned int left:1;		/* - flag.  */
  unsigned int showsign:1;	/* + flag.  */
  unsigned int group:1;		/* ' flag.  */
  unsigned int extra:1;		/* For special use.  */
  unsigned int is_char:1;	/* hh flag.  */
  unsigned int wide:1;		/* Nonzero for wide character streams.  */
  unsigned int i18n:1;		/* I flag.  */
  unsigned int is_binary128:1;	/* Floating-point argument is ABI-compatible
				   with IEC 60559 binary128.  */
  unsigned int __pad:3;		/* Unused so far.  */
  unsigned short int user;	/* Bits for user-installed modifiers.  */
  wchar_t pad;			/* Padding character.  */
};


/* Type of a printf specifier-handler function.
   STREAM is the FILE on which to write output.
   INFO gives information about the format specification.
   ARGS is a vector of pointers to the argument data;
   the number of pointers will be the number returned
   by the associated arginfo function for the same INFO.

   The function should return the number of characters written,
   or -1 for errors.  */

typedef int printf_function (FILE *__stream,
			     const struct printf_info *__info,
			     const void *const *__args);

/* Type of a printf specifier-arginfo function.
   INFO gives information about the format specification.
   N, ARGTYPES, *SIZE has to contain the size of the parameter for
   user-defined types, and return value are as for parse_printf_format
   except that -1 should be returned if the handler cannot handle
   this case.  This allows to partially overwrite the functionality
   of existing format specifiers.  */

typedef int printf_arginfo_size_function (const struct printf_info *__info,
					  size_t __n, int *__argtypes,
					  int *__size);

/* Old version of 'printf_arginfo_function' without a SIZE parameter.  */

typedef int printf_arginfo_function (const struct printf_info *__info,
				     size_t __n, int *__argtypes);

/* Type of a function to get a value of a user-defined from the
   variable argument list.  */
typedef void printf_va_arg_function (void *__mem, va_list *__ap);


/* Register FUNC to be called to format SPEC specifiers; ARGINFO must be
   specified to determine how many arguments a SPEC conversion requires and
   what their types are.  */

extern int register_printf_specifier (int __spec, printf_function __func,
				      printf_arginfo_size_function __arginfo)
  __THROW;


/* Obsolete interface similar to register_printf_specifier.  It can only
   handle basic data types because the ARGINFO callback does not return
   information on the size of the user-defined type.  */

extern int register_printf_function (int __spec, printf_function __func,
				     printf_arginfo_function __arginfo)
  __THROW __attribute_deprecated__;


/* Register a new modifier character sequence.  If the call succeeds
   it returns a positive value representing the bit set in the USER
   field in 'struct printf_info'.  */

extern int register_printf_modifier (const wchar_t *__str) __THROW __wur;


/* Register variable argument handler for user type.  The return value
   is to be used in ARGINFO functions to signal the use of the
   type.  */
extern int register_printf_type (printf_va_arg_function __fct) __THROW __wur;


/* Parse FMT, and fill in N elements of ARGTYPES with the
   types needed for the conversions FMT specifies.  Returns
   the number of arguments required by FMT.

   The ARGINFO function registered with a user-defined format is passed a
   `struct printf_info' describing the format spec being parsed.  A width
   or precision of INT_MIN means a `*' was used to indicate that the
   width/precision will come from an arg.  The function should fill in the
   array it is passed with the types of the arguments it wants, and return
   the number of arguments it wants.  */

extern size_t parse_printf_format (const char *__restrict __fmt, size_t __n,
				   int *__restrict __argtypes) __THROW;


/* Codes returned by `parse_printf_format' for basic types.

   These values cover all the standard format specifications.
   Users can reserve new values after PA_LAST for their own types
   using 'register_printf_type'.  */

enum
{				/* C type: */
  PA_INT,			/* int */
  PA_CHAR,			/* int, cast to char */
  PA_WCHAR,			/* wide char */
  PA_STRING,			/* const char *, a '\0'-terminated string */
  PA_WSTRING,			/* const wchar_t *, wide character string */
  PA_POINTER,			/* void * */
  PA_FLOAT,			/* float */
  PA_DOUBLE,			/* double */
  PA_LAST
};

/* Flag bits that can be set in a type returned by `parse_printf_format'.  */
#define	PA_FLAG_MASK		0xff00
#define	PA_FLAG_LONG_LONG	(1 << 8)
#define	PA_FLAG_LONG_DOUBLE	PA_FLAG_LONG_LONG
#define	PA_FLAG_LONG		(1 << 9)
#define	PA_FLAG_SHORT		(1 << 10)
#define	PA_FLAG_PTR		(1 << 11)



/* Function which can be registered as `printf'-handlers.  */

/* Print floating point value using using abbreviations for the orders
   of magnitude used for numbers ('k' for kilo, 'm' for mega etc).  If
   the format specifier is a uppercase character powers of 1000 are
   used.  Otherwise powers of 1024.  */
extern int printf_size (FILE *__restrict __fp,
			const struct printf_info *__info,
			const void *const *__restrict __args) __THROW;

/* This is the appropriate argument information function for `printf_size'.  */
extern int printf_size_info (const struct printf_info *__restrict
			     __info, size_t __n, int *__restrict __argtypes)
     __THROW;

#ifdef __LDBL_COMPAT
# include <bits/printf-ldbl.h>
#endif

__END_DECLS

#endif /* printf.h  */
