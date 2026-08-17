/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2022 The NASM Authors - All Rights Reserved
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

#ifndef NASMLIB_FILE_H
#define NASMLIB_FILE_H

#include "compiler.h"
#include "nasmlib.h"
#include "error.h"

#include <errno.h>

#ifdef HAVE_FCNTL_H
# include <fcntl.h>
#endif
#ifdef HAVE_SYS_TYPES_H
#endif
#ifdef HAVE_SYS_STAT_H
# include <sys/stat.h>
#endif
#ifdef HAVE_IO_H
# include <io.h>
#endif
#ifdef HAVE_UNISTD_H
# include <unistd.h>
#endif
#ifdef HAVE_SYS_MMAN_H
# include <sys/mman.h>
#endif

#ifndef R_OK
# define R_OK 4                 /* Classic Unix constant, same on Windows */
#endif

/* Can we adjust the file size without actually writing all the bytes? */
#ifdef HAVE__CHSIZE_S
# define os_ftruncate(fd,size)	_chsize_s(fd,size)
#elif defined(HAVE__CHSIZE)
# define os_ftruncate(fd,size)	_chsize(fd,size)
#elif defined(HAVE_FTRUNCATE)
# define os_ftruncate(fd,size)	ftruncate(fd,size)
#endif

/*
 * On Windows, we want to use _wfopen(), as fopen() has a much smaller limit
 * on the path length that it supports. Furthermore, we want to prefix the
 * path name with \\?\ in order to let the Windows kernel know that
 * we are not limited to PATH_MAX characters. Thus, we wrap all the functions
 * which take filenames...
 */
#ifdef _WIN32
# include <wchar.h>
typedef wchar_t *os_filename;
typedef wchar_t  os_fopenflag;

os_filename os_mangle_filename(const char *filename);
static inline void os_free_filename(os_filename filename)
{
    nasm_free(filename);
}

# define os_fopen  _wfopen
# define os_access _waccess

/*
 * On Win32/64, we have to use the _wstati64() function. Note that
 * we can't use _wstat64() without depending on a needlessly new
 * version os MSVCRT.
 */

typedef struct _stati64 os_struct_stat;

# define os_stat  _wstati64
# define os_fstat _fstati64

/*
 * On Win32/64, freopen() and _wfreopen() fails when the mode string
 * is with the letter 'b' that represents to set binary mode. On
 * POSIX operating systems, the 'b' is ignored, without failure.
 */

#include <io.h>
#include <fcntl.h>

static inline void os_set_binary_mode(FILE *f) {
    int ret = _setmode(_fileno(f), _O_BINARY);

    if (ret == -1) {
        nasm_fatalf(ERR_NOFILE, "unable to open file: %s",
                    strerror(errno));
    }
}

#else  /* not _WIN32 */

typedef const char *os_filename;
typedef char os_fopenflag;

static inline os_filename os_mangle_filename(const char *filename)
{
    return filename;
}
static inline void os_free_filename(os_filename filename)
{
    (void)filename;             /* Nothing to do */
}

static inline void os_set_binary_mode(FILE *f) {
    (void)f;
}

# define os_fopen  fopen

#if defined(HAVE_FACCESSAT) && defined(AT_EACCESS)
static inline int os_access(os_filename pathname, int mode)
{
    return faccessat(AT_FDCWD, pathname, mode, AT_EACCESS);
}
# define os_access os_access
#elif defined(HAVE_ACCESS)
# define os_access access
#endif

#ifdef HAVE_STRUCT_STAT
typedef struct stat os_struct_stat;
# ifdef HAVE_STAT
#  define os_stat stat
# endif
# ifdef HAVE_FSTAT
#  define os_fstat fstat
# endif
#else
struct dummy_struct_stat {
    int st_mode;
    int st_size;
};
typedef struct dummy_struct_stat os_struct_stat;
#endif

#endif  /* Not _WIN32 */

#ifdef S_ISREG
/* all good */
#elif defined(HAVE_S_ISREG)
/* exists, but not a macro */
# define S_ISREG S_ISREG
#elif defined(S_IFMT) && defined(S_IFREG)
# define S_ISREG(m) (((m) & S_IFMT) == S_IFREG)
#elif defined(_S_IFMT) && defined(_S_IFREG)
# define S_ISREG(m) (((m) & _S_IFMT) == _S_IFREG)
#endif

#ifdef fileno
/* all good */
#elif defined(HAVE_FILENO)
/* exists, but not a macro */
# define fileno fileno
#elif defined(_fileno) || defined(HAVE__FILENO)
# define fileno _fileno
#endif

#ifndef S_ISREG
# undef os_stat
# undef os_fstat
#endif

/* Disable these functions if they don't support something we need */
#ifndef fileno
# undef os_fstat
# undef os_ftruncate
# undef HAVE_MMAP
#endif

/*
 * If we don't have functional versions of these functions,
 * stub them out so we don't need so many #ifndefs
 */
#ifndef os_stat
static inline int os_stat(os_filename osfname, os_struct_stat *st)
{
    (void)osfname;
    (void)st;
    return -1;
}
#endif

#ifndef os_fstat
static inline int os_fstat(int fd, os_struct_stat *st)
{
    (void)fd;
    (void)st;
    return -1;
}
#endif

#ifndef S_ISREG
static inline bool S_ISREG(int m)
{
    (void)m;
    return false;
}
#endif

#endif /* NASMLIB_FILE_H */
