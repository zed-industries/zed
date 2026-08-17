/* Checking macros for wchar functions.
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

#ifndef _WCHAR_H
# error "Never include <bits/wchar2.h> directly; use <wchar.h> instead."
#endif


extern wchar_t *__wmemcpy_chk (wchar_t *__restrict __s1,
			       const wchar_t *__restrict __s2, size_t __n,
			       size_t __ns1) __THROW;
extern wchar_t *__REDIRECT_NTH (__wmemcpy_alias,
				(wchar_t *__restrict __s1,
				 const wchar_t *__restrict __s2, size_t __n),
				wmemcpy);
extern wchar_t *__REDIRECT_NTH (__wmemcpy_chk_warn,
				(wchar_t *__restrict __s1,
				 const wchar_t *__restrict __s2, size_t __n,
				 size_t __ns1), __wmemcpy_chk)
     __warnattr ("wmemcpy called with length bigger than size of destination "
		 "buffer");

__fortify_function wchar_t *
__NTH (wmemcpy (wchar_t *__restrict __s1, const wchar_t *__restrict __s2,
		size_t __n))
{
  if (__bos0 (__s1) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n))
	return __wmemcpy_chk (__s1, __s2, __n,
			      __bos0 (__s1) / sizeof (wchar_t));

      if (__n > __bos0 (__s1) / sizeof (wchar_t))
	return __wmemcpy_chk_warn (__s1, __s2, __n,
				   __bos0 (__s1) / sizeof (wchar_t));
    }
  return __wmemcpy_alias (__s1, __s2, __n);
}


extern wchar_t *__wmemmove_chk (wchar_t *__s1, const wchar_t *__s2,
				size_t __n, size_t __ns1) __THROW;
extern wchar_t *__REDIRECT_NTH (__wmemmove_alias, (wchar_t *__s1,
						   const wchar_t *__s2,
						   size_t __n), wmemmove);
extern wchar_t *__REDIRECT_NTH (__wmemmove_chk_warn,
				(wchar_t *__s1, const wchar_t *__s2,
				 size_t __n, size_t __ns1), __wmemmove_chk)
     __warnattr ("wmemmove called with length bigger than size of destination "
		 "buffer");

__fortify_function wchar_t *
__NTH (wmemmove (wchar_t *__s1, const wchar_t *__s2, size_t __n))
{
  if (__bos0 (__s1) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n))
	return __wmemmove_chk (__s1, __s2, __n,
			       __bos0 (__s1) / sizeof (wchar_t));

      if (__n > __bos0 (__s1) / sizeof (wchar_t))
	return __wmemmove_chk_warn (__s1, __s2, __n,
				    __bos0 (__s1) / sizeof (wchar_t));
    }
  return __wmemmove_alias (__s1, __s2, __n);
}


#ifdef __USE_GNU
extern wchar_t *__wmempcpy_chk (wchar_t *__restrict __s1,
				const wchar_t *__restrict __s2, size_t __n,
				size_t __ns1) __THROW;
extern wchar_t *__REDIRECT_NTH (__wmempcpy_alias,
				(wchar_t *__restrict __s1,
				 const wchar_t *__restrict __s2,
				 size_t __n), wmempcpy);
extern wchar_t *__REDIRECT_NTH (__wmempcpy_chk_warn,
				(wchar_t *__restrict __s1,
				 const wchar_t *__restrict __s2, size_t __n,
				 size_t __ns1), __wmempcpy_chk)
     __warnattr ("wmempcpy called with length bigger than size of destination "
		 "buffer");

__fortify_function wchar_t *
__NTH (wmempcpy (wchar_t *__restrict __s1, const wchar_t *__restrict __s2,
		 size_t __n))
{
  if (__bos0 (__s1) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n))
	return __wmempcpy_chk (__s1, __s2, __n,
			       __bos0 (__s1) / sizeof (wchar_t));

      if (__n > __bos0 (__s1) / sizeof (wchar_t))
	return __wmempcpy_chk_warn (__s1, __s2, __n,
				    __bos0 (__s1) / sizeof (wchar_t));
    }
  return __wmempcpy_alias (__s1, __s2, __n);
}
#endif


extern wchar_t *__wmemset_chk (wchar_t *__s, wchar_t __c, size_t __n,
			       size_t __ns) __THROW;
extern wchar_t *__REDIRECT_NTH (__wmemset_alias, (wchar_t *__s, wchar_t __c,
						  size_t __n), wmemset);
extern wchar_t *__REDIRECT_NTH (__wmemset_chk_warn,
				(wchar_t *__s, wchar_t __c, size_t __n,
				 size_t __ns), __wmemset_chk)
     __warnattr ("wmemset called with length bigger than size of destination "
		 "buffer");

__fortify_function wchar_t *
__NTH (wmemset (wchar_t *__s, wchar_t __c, size_t __n))
{
  if (__bos0 (__s) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n))
	return __wmemset_chk (__s, __c, __n, __bos0 (__s) / sizeof (wchar_t));

      if (__n > __bos0 (__s) / sizeof (wchar_t))
	return __wmemset_chk_warn (__s, __c, __n,
				   __bos0 (__s) / sizeof (wchar_t));
    }
  return __wmemset_alias (__s, __c, __n);
}


extern wchar_t *__wcscpy_chk (wchar_t *__restrict __dest,
			      const wchar_t *__restrict __src,
			      size_t __n) __THROW;
extern wchar_t *__REDIRECT_NTH (__wcscpy_alias,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src), wcscpy);

__fortify_function wchar_t *
__NTH (wcscpy (wchar_t *__restrict __dest, const wchar_t *__restrict __src))
{
  if (__bos (__dest) != (size_t) -1)
    return __wcscpy_chk (__dest, __src, __bos (__dest) / sizeof (wchar_t));
  return __wcscpy_alias (__dest, __src);
}


extern wchar_t *__wcpcpy_chk (wchar_t *__restrict __dest,
			      const wchar_t *__restrict __src,
			      size_t __destlen) __THROW;
extern wchar_t *__REDIRECT_NTH (__wcpcpy_alias,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src), wcpcpy);

__fortify_function wchar_t *
__NTH (wcpcpy (wchar_t *__restrict __dest, const wchar_t *__restrict __src))
{
  if (__bos (__dest) != (size_t) -1)
    return __wcpcpy_chk (__dest, __src, __bos (__dest) / sizeof (wchar_t));
  return __wcpcpy_alias (__dest, __src);
}


extern wchar_t *__wcsncpy_chk (wchar_t *__restrict __dest,
			       const wchar_t *__restrict __src, size_t __n,
			       size_t __destlen) __THROW;
extern wchar_t *__REDIRECT_NTH (__wcsncpy_alias,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src,
				 size_t __n), wcsncpy);
extern wchar_t *__REDIRECT_NTH (__wcsncpy_chk_warn,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src,
				 size_t __n, size_t __destlen), __wcsncpy_chk)
     __warnattr ("wcsncpy called with length bigger than size of destination "
		 "buffer");

__fortify_function wchar_t *
__NTH (wcsncpy (wchar_t *__restrict __dest, const wchar_t *__restrict __src,
		size_t __n))
{
  if (__bos (__dest) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n))
	return __wcsncpy_chk (__dest, __src, __n,
			      __bos (__dest) / sizeof (wchar_t));
      if (__n > __bos (__dest) / sizeof (wchar_t))
	return __wcsncpy_chk_warn (__dest, __src, __n,
				   __bos (__dest) / sizeof (wchar_t));
    }
  return __wcsncpy_alias (__dest, __src, __n);
}


extern wchar_t *__wcpncpy_chk (wchar_t *__restrict __dest,
			       const wchar_t *__restrict __src, size_t __n,
			       size_t __destlen) __THROW;
extern wchar_t *__REDIRECT_NTH (__wcpncpy_alias,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src,
				 size_t __n), wcpncpy);
extern wchar_t *__REDIRECT_NTH (__wcpncpy_chk_warn,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src,
				 size_t __n, size_t __destlen), __wcpncpy_chk)
     __warnattr ("wcpncpy called with length bigger than size of destination "
		 "buffer");

__fortify_function wchar_t *
__NTH (wcpncpy (wchar_t *__restrict __dest, const wchar_t *__restrict __src,
		size_t __n))
{
  if (__bos (__dest) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n))
	return __wcpncpy_chk (__dest, __src, __n,
			      __bos (__dest) / sizeof (wchar_t));
      if (__n > __bos (__dest) / sizeof (wchar_t))
	return __wcpncpy_chk_warn (__dest, __src, __n,
				   __bos (__dest) / sizeof (wchar_t));
    }
  return __wcpncpy_alias (__dest, __src, __n);
}


extern wchar_t *__wcscat_chk (wchar_t *__restrict __dest,
			      const wchar_t *__restrict __src,
			      size_t __destlen) __THROW;
extern wchar_t *__REDIRECT_NTH (__wcscat_alias,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src), wcscat);

__fortify_function wchar_t *
__NTH (wcscat (wchar_t *__restrict __dest, const wchar_t *__restrict __src))
{
  if (__bos (__dest) != (size_t) -1)
    return __wcscat_chk (__dest, __src, __bos (__dest) / sizeof (wchar_t));
  return __wcscat_alias (__dest, __src);
}


extern wchar_t *__wcsncat_chk (wchar_t *__restrict __dest,
			       const wchar_t *__restrict __src,
			       size_t __n, size_t __destlen) __THROW;
extern wchar_t *__REDIRECT_NTH (__wcsncat_alias,
				(wchar_t *__restrict __dest,
				 const wchar_t *__restrict __src,
				 size_t __n), wcsncat);

__fortify_function wchar_t *
__NTH (wcsncat (wchar_t *__restrict __dest, const wchar_t *__restrict __src,
		size_t __n))
{
  if (__bos (__dest) != (size_t) -1)
    return __wcsncat_chk (__dest, __src, __n,
			  __bos (__dest) / sizeof (wchar_t));
  return __wcsncat_alias (__dest, __src, __n);
}


extern int __swprintf_chk (wchar_t *__restrict __s, size_t __n,
			   int __flag, size_t __s_len,
			   const wchar_t *__restrict __format, ...)
     __THROW /* __attribute__ ((__format__ (__wprintf__, 5, 6))) */;

extern int __REDIRECT_NTH_LDBL (__swprintf_alias,
				(wchar_t *__restrict __s, size_t __n,
				 const wchar_t *__restrict __fmt, ...),
				swprintf);

#ifdef __va_arg_pack
__fortify_function int
__NTH (swprintf (wchar_t *__restrict __s, size_t __n,
		 const wchar_t *__restrict __fmt, ...))
{
  if (__bos (__s) != (size_t) -1 || __USE_FORTIFY_LEVEL > 1)
    return __swprintf_chk (__s, __n, __USE_FORTIFY_LEVEL - 1,
			   __bos (__s) / sizeof (wchar_t),
			   __fmt, __va_arg_pack ());
  return __swprintf_alias (__s, __n, __fmt, __va_arg_pack ());
}
#elif !defined __cplusplus
/* XXX We might want to have support in gcc for swprintf.  */
# define swprintf(s, n, ...) \
  (__bos (s) != (size_t) -1 || __USE_FORTIFY_LEVEL > 1			      \
   ? __swprintf_chk (s, n, __USE_FORTIFY_LEVEL - 1,			      \
		     __bos (s) / sizeof (wchar_t), __VA_ARGS__)		      \
   : swprintf (s, n, __VA_ARGS__))
#endif

extern int __vswprintf_chk (wchar_t *__restrict __s, size_t __n,
			    int __flag, size_t __s_len,
			    const wchar_t *__restrict __format,
			    __gnuc_va_list __arg)
     __THROW /* __attribute__ ((__format__ (__wprintf__, 5, 0))) */;

extern int __REDIRECT_NTH_LDBL (__vswprintf_alias,
				(wchar_t *__restrict __s, size_t __n,
				 const wchar_t *__restrict __fmt,
				 __gnuc_va_list __ap), vswprintf);

__fortify_function int
__NTH (vswprintf (wchar_t *__restrict __s, size_t __n,
		  const wchar_t *__restrict __fmt, __gnuc_va_list __ap))
{
  if (__bos (__s) != (size_t) -1 || __USE_FORTIFY_LEVEL > 1)
    return __vswprintf_chk (__s, __n,  __USE_FORTIFY_LEVEL - 1,
			    __bos (__s) / sizeof (wchar_t), __fmt, __ap);
  return __vswprintf_alias (__s, __n, __fmt, __ap);
}


#if __USE_FORTIFY_LEVEL > 1

extern int __fwprintf_chk (__FILE *__restrict __stream, int __flag,
			   const wchar_t *__restrict __format, ...);
extern int __wprintf_chk (int __flag, const wchar_t *__restrict __format,
			  ...);
extern int __vfwprintf_chk (__FILE *__restrict __stream, int __flag,
			    const wchar_t *__restrict __format,
			    __gnuc_va_list __ap);
extern int __vwprintf_chk (int __flag, const wchar_t *__restrict __format,
			   __gnuc_va_list __ap);

# ifdef __va_arg_pack
__fortify_function int
wprintf (const wchar_t *__restrict __fmt, ...)
{
  return __wprintf_chk (__USE_FORTIFY_LEVEL - 1, __fmt, __va_arg_pack ());
}

__fortify_function int
fwprintf (__FILE *__restrict __stream, const wchar_t *__restrict __fmt, ...)
{
  return __fwprintf_chk (__stream, __USE_FORTIFY_LEVEL - 1, __fmt,
			 __va_arg_pack ());
}
# elif !defined __cplusplus
#  define wprintf(...) \
  __wprintf_chk (__USE_FORTIFY_LEVEL - 1, __VA_ARGS__)
#  define fwprintf(stream, ...) \
  __fwprintf_chk (stream, __USE_FORTIFY_LEVEL - 1, __VA_ARGS__)
# endif

__fortify_function int
vwprintf (const wchar_t *__restrict __fmt, __gnuc_va_list __ap)
{
  return __vwprintf_chk (__USE_FORTIFY_LEVEL - 1, __fmt, __ap);
}

__fortify_function int
vfwprintf (__FILE *__restrict __stream,
	   const wchar_t *__restrict __fmt, __gnuc_va_list __ap)
{
  return __vfwprintf_chk (__stream, __USE_FORTIFY_LEVEL - 1, __fmt, __ap);
}

#endif

extern wchar_t *__fgetws_chk (wchar_t *__restrict __s, size_t __size, int __n,
			      __FILE *__restrict __stream) __wur;
extern wchar_t *__REDIRECT (__fgetws_alias,
			    (wchar_t *__restrict __s, int __n,
			     __FILE *__restrict __stream), fgetws) __wur;
extern wchar_t *__REDIRECT (__fgetws_chk_warn,
			    (wchar_t *__restrict __s, size_t __size, int __n,
			     __FILE *__restrict __stream), __fgetws_chk)
     __wur __warnattr ("fgetws called with bigger size than length "
		       "of destination buffer");

__fortify_function __wur wchar_t *
fgetws (wchar_t *__restrict __s, int __n, __FILE *__restrict __stream)
{
  if (__bos (__s) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n) || __n <= 0)
	return __fgetws_chk (__s, __bos (__s) / sizeof (wchar_t),
			     __n, __stream);

      if ((size_t) __n > __bos (__s) / sizeof (wchar_t))
	return __fgetws_chk_warn (__s, __bos (__s) / sizeof (wchar_t),
				  __n, __stream);
    }
  return __fgetws_alias (__s, __n, __stream);
}

#ifdef __USE_GNU
extern wchar_t *__fgetws_unlocked_chk (wchar_t *__restrict __s, size_t __size,
				       int __n, __FILE *__restrict __stream)
  __wur;
extern wchar_t *__REDIRECT (__fgetws_unlocked_alias,
			    (wchar_t *__restrict __s, int __n,
			     __FILE *__restrict __stream), fgetws_unlocked)
  __wur;
extern wchar_t *__REDIRECT (__fgetws_unlocked_chk_warn,
			    (wchar_t *__restrict __s, size_t __size, int __n,
			     __FILE *__restrict __stream),
			    __fgetws_unlocked_chk)
     __wur __warnattr ("fgetws_unlocked called with bigger size than length "
		       "of destination buffer");

__fortify_function __wur wchar_t *
fgetws_unlocked (wchar_t *__restrict __s, int __n, __FILE *__restrict __stream)
{
  if (__bos (__s) != (size_t) -1)
    {
      if (!__builtin_constant_p (__n) || __n <= 0)
	return __fgetws_unlocked_chk (__s, __bos (__s) / sizeof (wchar_t),
				      __n, __stream);

      if ((size_t) __n > __bos (__s) / sizeof (wchar_t))
	return __fgetws_unlocked_chk_warn (__s, __bos (__s) / sizeof (wchar_t),
					   __n, __stream);
    }
  return __fgetws_unlocked_alias (__s, __n, __stream);
}
#endif


extern size_t __wcrtomb_chk (char *__restrict __s, wchar_t __wchar,
			     mbstate_t *__restrict __p,
			     size_t __buflen) __THROW __wur;
extern size_t __REDIRECT_NTH (__wcrtomb_alias,
			      (char *__restrict __s, wchar_t __wchar,
			       mbstate_t *__restrict __ps), wcrtomb) __wur;

__fortify_function __wur size_t
__NTH (wcrtomb (char *__restrict __s, wchar_t __wchar,
		mbstate_t *__restrict __ps))
{
  /* We would have to include <limits.h> to get a definition of MB_LEN_MAX.
     But this would only disturb the namespace.  So we define our own
     version here.  */
#define __WCHAR_MB_LEN_MAX	16
#if defined MB_LEN_MAX && MB_LEN_MAX != __WCHAR_MB_LEN_MAX
# error "Assumed value of MB_LEN_MAX wrong"
#endif
  if (__bos (__s) != (size_t) -1 && __WCHAR_MB_LEN_MAX > __bos (__s))
    return __wcrtomb_chk (__s, __wchar, __ps, __bos (__s));
  return __wcrtomb_alias (__s, __wchar, __ps);
}


extern size_t __mbsrtowcs_chk (wchar_t *__restrict __dst,
			       const char **__restrict __src,
			       size_t __len, mbstate_t *__restrict __ps,
			       size_t __dstlen) __THROW;
extern size_t __REDIRECT_NTH (__mbsrtowcs_alias,
			      (wchar_t *__restrict __dst,
			       const char **__restrict __src,
			       size_t __len, mbstate_t *__restrict __ps),
			      mbsrtowcs);
extern size_t __REDIRECT_NTH (__mbsrtowcs_chk_warn,
			      (wchar_t *__restrict __dst,
			       const char **__restrict __src,
			       size_t __len, mbstate_t *__restrict __ps,
			       size_t __dstlen), __mbsrtowcs_chk)
     __warnattr ("mbsrtowcs called with dst buffer smaller than len "
		 "* sizeof (wchar_t)");

__fortify_function size_t
__NTH (mbsrtowcs (wchar_t *__restrict __dst, const char **__restrict __src,
		  size_t __len, mbstate_t *__restrict __ps))
{
  if (__bos (__dst) != (size_t) -1)
    {
      if (!__builtin_constant_p (__len))
	return __mbsrtowcs_chk (__dst, __src, __len, __ps,
				__bos (__dst) / sizeof (wchar_t));

      if (__len > __bos (__dst) / sizeof (wchar_t))
	return __mbsrtowcs_chk_warn (__dst, __src, __len, __ps,
				     __bos (__dst) / sizeof (wchar_t));
    }
  return __mbsrtowcs_alias (__dst, __src, __len, __ps);
}


extern size_t __wcsrtombs_chk (char *__restrict __dst,
			       const wchar_t **__restrict __src,
			       size_t __len, mbstate_t *__restrict __ps,
			       size_t __dstlen) __THROW;
extern size_t __REDIRECT_NTH (__wcsrtombs_alias,
			      (char *__restrict __dst,
			       const wchar_t **__restrict __src,
			       size_t __len, mbstate_t *__restrict __ps),
			      wcsrtombs);
extern size_t __REDIRECT_NTH (__wcsrtombs_chk_warn,
			      (char *__restrict __dst,
			       const wchar_t **__restrict __src,
			       size_t __len, mbstate_t *__restrict __ps,
			       size_t __dstlen), __wcsrtombs_chk)
    __warnattr ("wcsrtombs called with dst buffer smaller than len");

__fortify_function size_t
__NTH (wcsrtombs (char *__restrict __dst, const wchar_t **__restrict __src,
		  size_t __len, mbstate_t *__restrict __ps))
{
  if (__bos (__dst) != (size_t) -1)
    {
      if (!__builtin_constant_p (__len))
	return __wcsrtombs_chk (__dst, __src, __len, __ps, __bos (__dst));

      if (__len > __bos (__dst))
	return __wcsrtombs_chk_warn (__dst, __src, __len, __ps, __bos (__dst));
    }
  return __wcsrtombs_alias (__dst, __src, __len, __ps);
}


#ifdef __USE_GNU
extern size_t __mbsnrtowcs_chk (wchar_t *__restrict __dst,
				const char **__restrict __src, size_t __nmc,
				size_t __len, mbstate_t *__restrict __ps,
				size_t __dstlen) __THROW;
extern size_t __REDIRECT_NTH (__mbsnrtowcs_alias,
			      (wchar_t *__restrict __dst,
			       const char **__restrict __src, size_t __nmc,
			       size_t __len, mbstate_t *__restrict __ps),
			      mbsnrtowcs);
extern size_t __REDIRECT_NTH (__mbsnrtowcs_chk_warn,
			      (wchar_t *__restrict __dst,
			       const char **__restrict __src, size_t __nmc,
			       size_t __len, mbstate_t *__restrict __ps,
			       size_t __dstlen), __mbsnrtowcs_chk)
     __warnattr ("mbsnrtowcs called with dst buffer smaller than len "
		 "* sizeof (wchar_t)");

__fortify_function size_t
__NTH (mbsnrtowcs (wchar_t *__restrict __dst, const char **__restrict __src,
		   size_t __nmc, size_t __len, mbstate_t *__restrict __ps))
{
  if (__bos (__dst) != (size_t) -1)
    {
      if (!__builtin_constant_p (__len))
	return __mbsnrtowcs_chk (__dst, __src, __nmc, __len, __ps,
				 __bos (__dst) / sizeof (wchar_t));

      if (__len > __bos (__dst) / sizeof (wchar_t))
	return __mbsnrtowcs_chk_warn (__dst, __src, __nmc, __len, __ps,
				      __bos (__dst) / sizeof (wchar_t));
    }
  return __mbsnrtowcs_alias (__dst, __src, __nmc, __len, __ps);
}


extern size_t __wcsnrtombs_chk (char *__restrict __dst,
				const wchar_t **__restrict __src,
				size_t __nwc, size_t __len,
				mbstate_t *__restrict __ps, size_t __dstlen)
     __THROW;
extern size_t __REDIRECT_NTH (__wcsnrtombs_alias,
			      (char *__restrict __dst,
			       const wchar_t **__restrict __src,
			       size_t __nwc, size_t __len,
			       mbstate_t *__restrict __ps), wcsnrtombs);
extern size_t __REDIRECT_NTH (__wcsnrtombs_chk_warn,
			      (char *__restrict __dst,
			       const wchar_t **__restrict __src,
			       size_t __nwc, size_t __len,
			       mbstate_t *__restrict __ps,
			       size_t __dstlen), __wcsnrtombs_chk)
     __warnattr ("wcsnrtombs called with dst buffer smaller than len");

__fortify_function size_t
__NTH (wcsnrtombs (char *__restrict __dst, const wchar_t **__restrict __src,
		   size_t __nwc, size_t __len, mbstate_t *__restrict __ps))
{
  if (__bos (__dst) != (size_t) -1)
    {
      if (!__builtin_constant_p (__len))
	return __wcsnrtombs_chk (__dst, __src, __nwc, __len, __ps,
				 __bos (__dst));

      if (__len > __bos (__dst))
	return __wcsnrtombs_chk_warn (__dst, __src, __nwc, __len, __ps,
				      __bos (__dst));
    }
  return __wcsnrtombs_alias (__dst, __src, __nwc, __len, __ps);
}
#endif
