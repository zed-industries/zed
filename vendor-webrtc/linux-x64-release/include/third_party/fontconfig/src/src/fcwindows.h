/*
 * fontconfig/src/fcwindows.h
 *
 * Copyright © 2013  Google, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of the author(s) not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  The authors make no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * THE AUTHOR(S) DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 *
 * Google Author(s): Behdad Esfahbod
 */

#ifndef _FCWINDOWS_H_
#define _FCWINDOWS_H_

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#ifdef _WIN32
   /* Request Windows Vista for building.  This is required to
    * get MemoryBarrier on mingw32... */
#  if defined(_WIN32_WINNT) && _WIN32_WINNT < 0x0600
#    undef _WIN32_WINNT
#  endif
#  ifndef _WIN32_WINNT
#    define _WIN32_WINNT 0x0600
#  endif
#  define WIN32_LEAN_AND_MEAN
#  define WIN32_EXTRA_LEAN
#  define STRICT
#  include <windows.h>
#  include <io.h>

#if defined(_MSC_VER)
#include <BaseTsd.h>
typedef SSIZE_T ssize_t;
#endif

#define FC_UINT64_FORMAT	"I64u"

#define S_ISREG(m) (((m) & S_IFMT) == S_IFREG)

#ifndef S_ISDIR
#define S_ISDIR(m) (((m) & _S_IFMT) == _S_IFDIR)
#endif

#ifndef F_OK
#define F_OK 0
#endif
#ifndef X_OK
#define X_OK 0 /* no execute bit on windows */
#endif
#ifndef W_OK
#define W_OK 2
#endif
#ifndef R_OK
#define R_OK 4
#endif

/* MingW provides dirent.h / openddir(), but MSVC does not */
#ifndef HAVE_DIRENT_H

#define HAVE_STRUCT_DIRENT_D_TYPE 1

typedef struct DIR DIR;

typedef enum {
  DT_UNKNOWN = 0,
  DT_DIR,
  DT_REG,
} DIR_TYPE;

typedef struct dirent {
    const char *d_name;
    DIR_TYPE d_type;
} dirent;

#define opendir(dirname) FcCompatOpendirWin32(dirname)
#define closedir(d)      FcCompatClosedirWin32(d)
#define readdir(d)       FcCompatReaddirWin32(d)

DIR * FcCompatOpendirWin32 (const char *dirname);

struct dirent * FcCompatReaddirWin32 (DIR *dir);

int FcCompatClosedirWin32 (DIR *dir);

#endif /* HAVE_DIRENT_H */

#endif /* _WIN32 */

#endif /* _FCWINDOWS_H_ */
