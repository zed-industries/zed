/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the qmake spec of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QPLATFORMDEFS_H
#define QPLATFORMDEFS_H

#ifdef UNICODE
#ifndef _UNICODE
#define _UNICODE
#endif
#endif

// Get Qt defines/settings

#include "qglobal.h"

#include <tchar.h>
#include <io.h>
#include <direct.h>
#include <stdio.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/stat.h>
#include <stdlib.h>
#include <limits.h>

#if !defined(_WIN32_WINNT) || (_WIN32_WINNT-0 < 0x0500)
typedef enum {
    NameUnknown           = 0,
    NameFullyQualifiedDN  = 1,
    NameSamCompatible     = 2,
    NameDisplay           = 3,
    NameUniqueId          = 6,
    NameCanonical         = 7,
    NameUserPrincipal     = 8,
    NameCanonicalEx       = 9,
    NameServicePrincipal  = 10,
    NameDnsDomain         = 12
} EXTENDED_NAME_FORMAT, *PEXTENDED_NAME_FORMAT;
#endif

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
#endif
#define QT_FILENO               _fileno
#define QT_OPEN                 ::_open
#define QT_CLOSE                ::_close
#ifdef QT_LARGEFILE_SUPPORT
#define QT_LSEEK                ::_lseeki64
#ifndef UNICODE
#define QT_TSTAT                ::_stati64
#else
#define QT_TSTAT                ::_wstati64
#endif
#else
#define QT_LSEEK                ::_lseek
#ifndef UNICODE
#define QT_TSTAT                ::_stat
#else
#define QT_TSTAT                ::_wstat
#endif
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

#define QT_FSEEK                ::fseeko64
#define QT_FTELL                ::ftello64
#define QT_OFF_T                off64_t
#endif

#define QT_SIGNAL_ARGS          int

#define QT_VSNPRINTF            ::_vsnprintf
#define QT_SNPRINTF             ::_snprintf

# define F_OK   0
# define X_OK   1
# define W_OK   2
# define R_OK   4


#endif // QPLATFORMDEFS_H
