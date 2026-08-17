#ifndef _HAD_LIBZIP_COMPAT_H
#define _HAD_LIBZIP_COMPAT_H

/*
  compat.h -- compatibility defines.
  Copyright (C) 1999-2019 Dieter Baron and Thomas Klausner

  This file is part of libzip, a library to manipulate ZIP archives.
  The authors can be contacted at <libzip@nih.at>

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:
  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in
     the documentation and/or other materials provided with the
     distribution.
  3. The names of the authors may not be used to endorse or promote
     products derived from this software without specific prior
     written permission.

  THIS SOFTWARE IS PROVIDED BY THE AUTHORS ``AS IS'' AND ANY EXPRESS
  OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
  WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
  ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
  DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
  GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
  INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER
  IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
  OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN
  IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#include "zipconf.h"

#include "config.h"

/* to have *_MAX definitions for all types when compiling with g++ */
#define __STDC_LIMIT_MACROS

#ifdef _WIN32
#ifndef ZIP_EXTERN
#ifndef ZIP_STATIC
#define ZIP_EXTERN __declspec(dllexport)
#endif
#endif
/* for dup(), close(), etc. */
#include <io.h>
#endif

#ifdef HAVE_STDBOOL_H
#include <stdbool.h>
#else
typedef char bool;
#define true 1
#define false 0
#endif

#include <errno.h>

/* at least MinGW does not provide EOPNOTSUPP, see
 * http://sourceforge.net/p/mingw/bugs/263/
 */
#ifndef EOPNOTSUPP
#define EOPNOTSUPP EINVAL
#endif

/* at least MinGW does not provide EOVERFLOW, see
 * http://sourceforge.net/p/mingw/bugs/242/
 */
#ifndef EOVERFLOW
#define EOVERFLOW EFBIG
#endif

/* not supported on at least Windows */
#ifndef O_CLOEXEC
#define O_CLOEXEC 0
#endif

#ifdef _WIN32
#if defined(HAVE__CLOSE)
#define close _close
#endif
#if defined(HAVE__DUP)
#define dup _dup
#endif
/* crashes reported when using fdopen instead of _fdopen on Windows/Visual
 * Studio 10/Win64 */
#if defined(HAVE__FDOPEN)
#define fdopen _fdopen
#endif
#if !defined(HAVE_FILENO) && defined(HAVE__FILENO)
#define fileno _fileno
#endif
#if defined(HAVE__SNPRINTF)
#define snprintf _snprintf
#endif
#if defined(HAVE__STRDUP)
#if !defined(HAVE_STRDUP) || defined(_WIN32)
#undef strdup
#define strdup _strdup
#endif
#endif
#if !defined(HAVE__SETMODE) && defined(HAVE_SETMODE)
#define _setmode setmode
#endif
#if !defined(HAVE_STRTOLL) && defined(HAVE__STRTOI64)
#define strtoll _strtoi64
#endif
#if !defined(HAVE_STRTOULL) && defined(HAVE__STRTOUI64)
#define strtoull _strtoui64
#endif
#if defined(HAVE__UNLINK)
#define unlink _unlink
#endif
#endif

#ifndef HAVE_FSEEKO
#define fseeko(s, o, w) (fseek((s), (long int)(o), (w)))
#endif

#ifndef HAVE_FTELLO
#define ftello(s) ((long)ftell((s)))
#endif

#if !defined(HAVE_STRCASECMP)
#if defined(HAVE__STRICMP)
#define strcasecmp _stricmp
#elif defined(HAVE_STRICMP)
#define strcasecmp stricmp
#endif
#endif

#if SIZEOF_OFF_T == 8
#define ZIP_OFF_MAX ZIP_INT64_MAX
#define ZIP_OFF_MIN ZIP_INT64_MIN
#elif SIZEOF_OFF_T == 4
#define ZIP_OFF_MAX ZIP_INT32_MAX
#define ZIP_OFF_MIN ZIP_INT32_MIN
#elif SIZEOF_OFF_T == 2
#define ZIP_OFF_MAX ZIP_INT16_MAX
#define ZIP_OFF_MIN ZIP_INT16_MIN
#else
#error unsupported size of off_t
#endif

#if defined(HAVE_FTELLO) && defined(HAVE_FSEEKO)
#define ZIP_FSEEK_MAX ZIP_OFF_MAX
#define ZIP_FSEEK_MIN ZIP_OFF_MIN
#else
#include <limits.h>
#define ZIP_FSEEK_MAX LONG_MAX
#define ZIP_FSEEK_MIN LONG_MIN
#endif

#ifndef SIZE_MAX
#if SIZEOF_SIZE_T == 8
#define SIZE_MAX ZIP_INT64_MAX
#elif SIZEOF_SIZE_T == 4
#define SIZE_MAX ZIP_INT32_MAX
#elif SIZEOF_SIZE_T == 2
#define SIZE_MAX ZIP_INT16_MAX
#else
#error unsupported size of size_t
#endif
#endif

#ifndef PRId64
#ifdef _MSC_VER
#define PRId64 "I64d"
#else
#define PRId64 "lld"
#endif
#endif

#ifndef PRIu64
#ifdef _MSC_VER
#define PRIu64 "I64u"
#else
#define PRIu64 "llu"
#endif
#endif

#ifndef S_ISDIR
#define S_ISDIR(mode) (((mode)&S_IFMT) == S_IFDIR)
#endif

#ifndef S_ISREG
#define S_ISREG(mode) (((mode)&S_IFMT) == S_IFREG)
#endif

#endif /* compat.h */
