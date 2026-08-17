/* Checking macros for fcntl functions.
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

#ifndef	_FCNTL_H
# error "Never include <bits/fcntl2.h> directly; use <fcntl.h> instead."
#endif

/* Check that calls to open and openat with O_CREAT or O_TMPFILE set have an
   appropriate third/fourth parameter.  */
#ifndef __USE_FILE_OFFSET64
extern int __open_2 (const char *__path, int __oflag) __nonnull ((1));
extern int __REDIRECT (__open_alias, (const char *__path, int __oflag, ...),
		       open) __nonnull ((1));
#else
extern int __REDIRECT (__open_2, (const char *__path, int __oflag),
		       __open64_2) __nonnull ((1));
extern int __REDIRECT (__open_alias, (const char *__path, int __oflag, ...),
		       open64) __nonnull ((1));
#endif
__errordecl (__open_too_many_args,
	     "open can be called either with 2 or 3 arguments, not more");
__errordecl (__open_missing_mode,
	     "open with O_CREAT or O_TMPFILE in second argument needs 3 arguments");

__fortify_function int
open (const char *__path, int __oflag, ...)
{
  if (__va_arg_pack_len () > 1)
    __open_too_many_args ();

  if (__builtin_constant_p (__oflag))
    {
      if (__OPEN_NEEDS_MODE (__oflag) && __va_arg_pack_len () < 1)
	{
	  __open_missing_mode ();
	  return __open_2 (__path, __oflag);
	}
      return __open_alias (__path, __oflag, __va_arg_pack ());
    }

  if (__va_arg_pack_len () < 1)
    return __open_2 (__path, __oflag);

  return __open_alias (__path, __oflag, __va_arg_pack ());
}


#ifdef __USE_LARGEFILE64
extern int __open64_2 (const char *__path, int __oflag) __nonnull ((1));
extern int __REDIRECT (__open64_alias, (const char *__path, int __oflag,
					...), open64) __nonnull ((1));
__errordecl (__open64_too_many_args,
	     "open64 can be called either with 2 or 3 arguments, not more");
__errordecl (__open64_missing_mode,
	     "open64 with O_CREAT or O_TMPFILE in second argument needs 3 arguments");

__fortify_function int
open64 (const char *__path, int __oflag, ...)
{
  if (__va_arg_pack_len () > 1)
    __open64_too_many_args ();

  if (__builtin_constant_p (__oflag))
    {
      if (__OPEN_NEEDS_MODE (__oflag) && __va_arg_pack_len () < 1)
	{
	  __open64_missing_mode ();
	  return __open64_2 (__path, __oflag);
	}
      return __open64_alias (__path, __oflag, __va_arg_pack ());
    }

  if (__va_arg_pack_len () < 1)
    return __open64_2 (__path, __oflag);

  return __open64_alias (__path, __oflag, __va_arg_pack ());
}
#endif


#ifdef __USE_ATFILE
# ifndef __USE_FILE_OFFSET64
extern int __openat_2 (int __fd, const char *__path, int __oflag)
     __nonnull ((2));
extern int __REDIRECT (__openat_alias, (int __fd, const char *__path,
					int __oflag, ...), openat)
     __nonnull ((2));
# else
extern int __REDIRECT (__openat_2, (int __fd, const char *__path,
				    int __oflag), __openat64_2)
     __nonnull ((2));
extern int __REDIRECT (__openat_alias, (int __fd, const char *__path,
					int __oflag, ...), openat64)
     __nonnull ((2));
# endif
__errordecl (__openat_too_many_args,
	     "openat can be called either with 3 or 4 arguments, not more");
__errordecl (__openat_missing_mode,
	     "openat with O_CREAT or O_TMPFILE in third argument needs 4 arguments");

__fortify_function int
openat (int __fd, const char *__path, int __oflag, ...)
{
  if (__va_arg_pack_len () > 1)
    __openat_too_many_args ();

  if (__builtin_constant_p (__oflag))
    {
      if (__OPEN_NEEDS_MODE (__oflag) && __va_arg_pack_len () < 1)
	{
	  __openat_missing_mode ();
	  return __openat_2 (__fd, __path, __oflag);
	}
      return __openat_alias (__fd, __path, __oflag, __va_arg_pack ());
    }

  if (__va_arg_pack_len () < 1)
    return __openat_2 (__fd, __path, __oflag);

  return __openat_alias (__fd, __path, __oflag, __va_arg_pack ());
}


# ifdef __USE_LARGEFILE64
extern int __openat64_2 (int __fd, const char *__path, int __oflag)
     __nonnull ((2));
extern int __REDIRECT (__openat64_alias, (int __fd, const char *__path,
					  int __oflag, ...), openat64)
     __nonnull ((2));
__errordecl (__openat64_too_many_args,
	     "openat64 can be called either with 3 or 4 arguments, not more");
__errordecl (__openat64_missing_mode,
	     "openat64 with O_CREAT or O_TMPFILE in third argument needs 4 arguments");

__fortify_function int
openat64 (int __fd, const char *__path, int __oflag, ...)
{
  if (__va_arg_pack_len () > 1)
    __openat64_too_many_args ();

  if (__builtin_constant_p (__oflag))
    {
      if (__OPEN_NEEDS_MODE (__oflag) && __va_arg_pack_len () < 1)
	{
	  __openat64_missing_mode ();
	  return __openat64_2 (__fd, __path, __oflag);
	}
      return __openat64_alias (__fd, __path, __oflag, __va_arg_pack ());
    }

  if (__va_arg_pack_len () < 1)
    return __openat64_2 (__fd, __path, __oflag);

  return __openat64_alias (__fd, __path, __oflag, __va_arg_pack ());
}
# endif
#endif
