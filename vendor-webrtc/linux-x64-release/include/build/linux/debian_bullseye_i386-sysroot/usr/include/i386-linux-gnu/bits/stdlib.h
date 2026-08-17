/* Checking macros for stdlib functions.
   Copyright (C) 2005-2020 Free Software Foundation, Inc.
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

#ifndef _STDLIB_H
# error "Never include <bits/stdlib.h> directly; use <stdlib.h> instead."
#endif

extern char *__realpath_chk (const char *__restrict __name,
			     char *__restrict __resolved,
			     size_t __resolvedlen) __THROW __wur;
extern char *__REDIRECT_NTH (__realpath_alias,
			     (const char *__restrict __name,
			      char *__restrict __resolved), realpath) __wur;
extern char *__REDIRECT_NTH (__realpath_chk_warn,
			     (const char *__restrict __name,
			      char *__restrict __resolved,
			      size_t __resolvedlen), __realpath_chk) __wur
     __warnattr ("second argument of realpath must be either NULL or at "
		 "least PATH_MAX bytes long buffer");

__fortify_function __wur char *
__NTH (realpath (const char *__restrict __name, char *__restrict __resolved))
{
  if (__bos (__resolved) != (size_t) -1)
    {
#if defined _LIBC_LIMITS_H_ && defined PATH_MAX
      if (__bos (__resolved) < PATH_MAX)
	return __realpath_chk_warn (__name, __resolved, __bos (__resolved));
#endif
      return __realpath_chk (__name, __resolved, __bos (__resolved));
    }

  return __realpath_alias (__name, __resolved);
}


extern int __ptsname_r_chk (int __fd, char *__buf, size_t __buflen,
			    size_t __nreal) __THROW __nonnull ((2));
extern int __REDIRECT_NTH (__ptsname_r_alias, (int __fd, char *__buf,
					       size_t __buflen), ptsname_r)
     __nonnull ((2));
extern int __REDIRECT_NTH (__ptsname_r_chk_warn,
			   (int __fd, char *__buf, size_t __buflen,
			    size_t __nreal), __ptsname_r_chk)
     __nonnull ((2)) __warnattr ("ptsname_r called with buflen bigger than "
				 "size of buf");

__fortify_function int
__NTH (ptsname_r (int __fd, char *__buf, size_t __buflen))
{
  if (__bos (__buf) != (size_t) -1)
    {
      if (!__builtin_constant_p (__buflen))
	return __ptsname_r_chk (__fd, __buf, __buflen, __bos (__buf));
      if (__buflen > __bos (__buf))
	return __ptsname_r_chk_warn (__fd, __buf, __buflen, __bos (__buf));
    }
  return __ptsname_r_alias (__fd, __buf, __buflen);
}


extern int __wctomb_chk (char *__s, wchar_t __wchar, size_t __buflen)
  __THROW __wur;
extern int __REDIRECT_NTH (__wctomb_alias, (char *__s, wchar_t __wchar),
			   wctomb) __wur;

__fortify_function __wur int
__NTH (wctomb (char *__s, wchar_t __wchar))
{
  /* We would have to include <limits.h> to get a definition of MB_LEN_MAX.
     But this would only disturb the namespace.  So we define our own
     version here.  */
#define __STDLIB_MB_LEN_MAX	16
#if defined MB_LEN_MAX && MB_LEN_MAX != __STDLIB_MB_LEN_MAX
# error "Assumed value of MB_LEN_MAX wrong"
#endif
  if (__bos (__s) != (size_t) -1 && __STDLIB_MB_LEN_MAX > __bos (__s))
    return __wctomb_chk (__s, __wchar, __bos (__s));
  return __wctomb_alias (__s, __wchar);
}


extern size_t __mbstowcs_chk (wchar_t *__restrict __dst,
			      const char *__restrict __src,
			      size_t __len, size_t __dstlen) __THROW;
extern size_t __REDIRECT_NTH (__mbstowcs_alias,
			      (wchar_t *__restrict __dst,
			       const char *__restrict __src,
			       size_t __len), mbstowcs);
extern size_t __REDIRECT_NTH (__mbstowcs_chk_warn,
			      (wchar_t *__restrict __dst,
			       const char *__restrict __src,
			       size_t __len, size_t __dstlen), __mbstowcs_chk)
     __warnattr ("mbstowcs called with dst buffer smaller than len "
		 "* sizeof (wchar_t)");

__fortify_function size_t
__NTH (mbstowcs (wchar_t *__restrict __dst, const char *__restrict __src,
		 size_t __len))
{
  if (__bos (__dst) != (size_t) -1)
    {
      if (!__builtin_constant_p (__len))
	return __mbstowcs_chk (__dst, __src, __len,
			       __bos (__dst) / sizeof (wchar_t));

      if (__len > __bos (__dst) / sizeof (wchar_t))
	return __mbstowcs_chk_warn (__dst, __src, __len,
				     __bos (__dst) / sizeof (wchar_t));
    }
  return __mbstowcs_alias (__dst, __src, __len);
}


extern size_t __wcstombs_chk (char *__restrict __dst,
			      const wchar_t *__restrict __src,
			      size_t __len, size_t __dstlen) __THROW;
extern size_t __REDIRECT_NTH (__wcstombs_alias,
			      (char *__restrict __dst,
			       const wchar_t *__restrict __src,
			       size_t __len), wcstombs);
extern size_t __REDIRECT_NTH (__wcstombs_chk_warn,
			      (char *__restrict __dst,
			       const wchar_t *__restrict __src,
			       size_t __len, size_t __dstlen), __wcstombs_chk)
     __warnattr ("wcstombs called with dst buffer smaller than len");

__fortify_function size_t
__NTH (wcstombs (char *__restrict __dst, const wchar_t *__restrict __src,
		 size_t __len))
{
  if (__bos (__dst) != (size_t) -1)
    {
      if (!__builtin_constant_p (__len))
	return __wcstombs_chk (__dst, __src, __len, __bos (__dst));
      if (__len > __bos (__dst))
	return __wcstombs_chk_warn (__dst, __src, __len, __bos (__dst));
    }
  return __wcstombs_alias (__dst, __src, __len);
}
