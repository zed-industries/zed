/* Specializations for error functions.
   Copyright (C) 2007-2020 Free Software Foundation, Inc.
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

#ifndef	_ERROR_H
# error "Never include <bits/error.h> directly; use <error.h> instead."
#endif


extern void __REDIRECT (__error_alias, (int __status, int __errnum,
					const char *__format, ...),
			error)
  __attribute__ ((__format__ (__printf__, 3, 4)));
extern void __REDIRECT (__error_noreturn, (int __status, int __errnum,
					   const char *__format, ...),
			error)
  __attribute__ ((__noreturn__, __format__ (__printf__, 3, 4)));


/* If we know the function will never return make sure the compiler
   realizes that, too.  */
__extern_always_inline void
error (int __status, int __errnum, const char *__format, ...)
{
  if (__builtin_constant_p (__status) && __status != 0)
    __error_noreturn (__status, __errnum, __format, __va_arg_pack ());
  else
    __error_alias (__status, __errnum, __format, __va_arg_pack ());
}


extern void __REDIRECT (__error_at_line_alias, (int __status, int __errnum,
						const char *__fname,
						unsigned int __line,
						const char *__format, ...),
			error_at_line)
  __attribute__ ((__format__ (__printf__, 5, 6)));
extern void __REDIRECT (__error_at_line_noreturn, (int __status, int __errnum,
						   const char *__fname,
						   unsigned int __line,
						   const char *__format,
						   ...),
			error_at_line)
  __attribute__ ((__noreturn__, __format__ (__printf__, 5, 6)));


/* If we know the function will never return make sure the compiler
   realizes that, too.  */
__extern_always_inline void
error_at_line (int __status, int __errnum, const char *__fname,
	       unsigned int __line, const char *__format, ...)
{
  if (__builtin_constant_p (__status) && __status != 0)
    __error_at_line_noreturn (__status, __errnum, __fname, __line, __format,
			      __va_arg_pack ());
  else
    __error_at_line_alias (__status, __errnum, __fname, __line,
			   __format, __va_arg_pack ());
}
