// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPLATFORMDEFS_H
#define QPLATFORMDEFS_H

#ifdef UNICODE
#ifndef _UNICODE
#define _UNICODE
#endif
#endif

// Get Qt defines/settings

#include <QtCore/qglobal.h>

#define _POSIX_
#include <limits.h>
#undef _POSIX_

#include <tchar.h>
#include <io.h>
#include <direct.h>
#include <stdio.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/stat.h>
#include <stdlib.h>

#ifdef QT_LARGEFILE_SUPPORT
#define QT_STATBUF              struct _stati64         // non-ANSI defs
#define QT_STATBUF4TSTAT        struct _stati64         // non-ANSI defs
#define QT_STAT                 ::_stati64
#define QT_FSTAT                ::_fstati64
#else
#define QT_STATBUF              struct _stat            // non-ANSI defs
#define QT_STATBUF4TSTAT        struct _stat            // non-ANSI defs
#define QT_STAT                 ::_stat
#define QT_FSTAT                ::_fstat
#endif
#define QT_STAT_REG             _S_IFREG
#define QT_STAT_DIR             _S_IFDIR
#define QT_STAT_MASK            _S_IFMT
#if defined(_S_IFLNK)
#  define QT_STAT_LNK           _S_IFLNK
#else
#  define QT_STAT_LNK           0120000
#endif
#define QT_FILENO               _fileno
#define QT_OPEN                 ::_open
#define QT_CLOSE                ::_close
#ifdef QT_LARGEFILE_SUPPORT
#define QT_LSEEK                ::_lseeki64
#define QT_TSTAT                ::_tstati64
#else
#define QT_LSEEK                ::_lseek
#define QT_TSTAT                ::_tstat
#endif
#define QT_READ                 ::_read
#define QT_WRITE                ::_write
#define QT_ACCESS               ::_access
#define QT_GETCWD               ::_getcwd
#define QT_CHDIR                ::_chdir
#define QT_MKDIR                ::_mkdir
#define QT_RMDIR                ::_rmdir
#define QT_OPEN_LARGEFILE       0
#define QT_OPEN_RDONLY          _O_RDONLY
#define QT_OPEN_WRONLY          _O_WRONLY
#define QT_OPEN_RDWR            _O_RDWR
#define QT_OPEN_CREAT           _O_CREAT
#define QT_OPEN_TRUNC           _O_TRUNC
#define QT_OPEN_APPEND          _O_APPEND
#if defined(O_TEXT)
# define QT_OPEN_TEXT           _O_TEXT
# define QT_OPEN_BINARY         _O_BINARY
#endif

#include "../common/c89/qplatformdefs.h"

#ifdef QT_LARGEFILE_SUPPORT
#undef QT_FSEEK
#undef QT_FTELL
#undef QT_OFF_T

#define QT_FSEEK                ::_fseeki64
#define QT_FTELL                ::_ftelli64
#define QT_OFF_T                __int64
#endif

#define QT_SIGNAL_ARGS          int

#define QT_VSNPRINTF(buffer, count, format, arg) \
    vsnprintf_s(buffer, count, count-1, format, arg)

#define QT_SNPRINTF             ::_snprintf

# define F_OK   0
# define X_OK   1
# define W_OK   2
# define R_OK   4

typedef int mode_t;

#endif // QPLATFORMDEFS_H
