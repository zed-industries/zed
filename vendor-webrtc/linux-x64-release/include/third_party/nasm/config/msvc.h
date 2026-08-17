/* ----------------------------------------------------------------------- *
 *
 *   Copyright 2016 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

/*
 * config/msvc.h
 *
 * Compiler definitions for Microsoft Visual C++;
 * instead of unconfig.h.  See config.h.in for the
 * variables which can be defined here.
 *
 * MSDN seems to have information back to Visual Studio 2003, so aim
 * for compatibility that far back.
 *
 * Relevant _MSC_VER values:
 * 1310 - Visual Studio 2003
 * 1400 - Visual Studio 2005
 * 1500 - Visual Studio 2008
 * 1600 - Visual Studio 2010
 * 1700 - Visual Studio 2012
 * 1800 - Visual Studio 2013
 * 1900 - Visual Studio 2015
 * 1910 - Visual Studio 2017
 */

#ifndef NASM_CONFIG_MSVC_H
#define NASM_CONFIG_MSVC_H

/* Define to 1 if you have the <fcntl.h> header file. */
#define HAVE_FCNTL_H 1

/* Define to 1 if you have the <inttypes.h> header file. */
#if _MSC_VER >= 1800
# define HAVE_INTTYPES_H 1
#endif

/* Define to 1 if you have the <io.h> header file. */
#define HAVE_IO_H 1

/* Define to 1 if you have the <stdlib.h> header file. */
#define HAVE_STDLIB_H 1

/* Define to 1 if you have the <string.h> header file. */
#define HAVE_STRING_H 1

/* Define to 1 if you have the <sys/stat.h> header file. */
#define HAVE_SYS_STAT_H 1

/* Define to 1 if you have the <sys/types.h> header file. */
#define HAVE_SYS_TYPES_H 1

/* Define to 1 if you have the `access' function. */
#define HAVE_ACCESS 1
#if _MSC_VER < 1400
# define access _access
#endif

/* Define to 1 if you have the `fileno' function. */
#define HAVE_FILENO 1
#if _MSC_VER < 1400
# define fileno _fileno
#endif

/* Define to 1 if you have the `snprintf' function. */
#define HAVE_SNPRINTF 1
#if _MSC_VER < 1900
# define snprintf _snprintf
#endif

/* Define to 1 if you have the `_chsize' function. */
#define HAVE__CHSIZE 1

/* Define to 1 if you have the `_chsize_s' function. */
#if _MSC_VER >= 1400
# define HAVE__CHSIZE_S 1
#endif

/* Define to 1 if you have the `_filelengthi64' function. */
#define HAVE__FILELENGTHI64 1

/* Define to 1 if you have the `_fseeki64' function. */
#define HAVE__FSEEKI64 1

/* Define to 1 if you have the `_fullpath' function. */
#define HAVE__FULLPATH 1

/* Define to 1 if the system has the type `struct _stati64'. */
#define HAVE_STRUCT__STATI64

/* Define to 1 if you have the `_stati64' function. */
#define HAVE__STATI64 1

/* Define to 1 if you have the `_fstati64' function. */
#define HAVE__FSTATI64 1

/* Define to 1 if stdbool.h conforms to C99. */
#if _MSC_VER >= 1800
# define HAVE_STDBOOL_H 1
#endif

/* Define to 1 if you have the `stricmp' function. */
#define HAVE_STRICMP 1
/* Define to 1 if you have the declaration of `stricmp', and to 0 if you
   don't. */
#define HAVE_DECL_STRICMP 1
#if _MSC_VER < 1400
# define stricmp _stricmp
#endif

/* Define to 1 if you have the `strnicmp' function. */
#define HAVE_STRNICMP 1
/* Define to 1 if you have the declaration of `strnicmp', and to 0 if you
   don't. */
#define HAVE_DECL_STRNICMP 1
#if _MSC_VER < 1400
# define strnicmp _strnicmp
#endif

#if _MSC_VER >= 1400
/* Define to 1 if you have the `strnlen' function. */
# define HAVE_STRNLEN 1
/* Define to 1 if you have the declaration of `strnlen', and to 0 if you
   don't. */
# define HAVE_DECL_STRNLEN 1
#endif

/* Define to 1 if the system has the type `uintptr_t'. */
#if _MSC_VER >= 1900
# define HAVE_UINTPTR_T 1
#else
/* Define to the type of an unsigned integer type wide enough to hold a
   pointer, if such a type exists, and if the system does not define it. */
# define uintptr_t size_t
#endif

/* Define to 1 if you have the `vsnprintf' function. */
#define HAVE_VSNPRINTF 1
#if _MSC_VER < 1400
# define vsnprint _vsnprintf
#endif

/* Define to 1 if the system has the type `_Bool'. */
#if _MSC_VER >= 1900
# define HAVE__BOOL 1
#endif

/* Define to 1 if you have the ANSI C header files. */
#define STDC_HEADERS 1

/* Define to 1 if your processor stores words with the least significant byte
   first (like Intel and VAX, unlike Motorola and SPARC). */
#define WORDS_LITTLEENDIAN 1

/* Define to `__inline__' or `__inline' if that's what the C compiler
   calls it, or to nothing if 'inline' is not supported under any name.  */
#define inline __inline

/* Define to the equivalent of the C99 'restrict' keyword, or to
   nothing if this is not supported.  Do not define if restrict is
   supported directly.  */
#if _MSC_VER >= 1700
#define restrict __restrict
#else
#define restrict
#endif

#endif /* NASM_CONFIG_MSVC_H */
