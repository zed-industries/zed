// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef Q_POSIX_QPLATFORMDEFS_H
#define Q_POSIX_QPLATFORMDEFS_H

#include <signal.h>

#include <sys/types.h>
#ifndef QT_NO_SOCKET_H
#  include <sys/socket.h>
#endif
#include <sys/stat.h>

#if defined(QT_USE_XOPEN_LFS_EXTENSIONS) && defined(QT_LARGEFILE_SUPPORT)

#define QT_STATBUF              struct stat64
#define QT_FPOS_T               fpos64_t
#define QT_OFF_T                off64_t

#define QT_STAT                 ::stat64
#define QT_LSTAT                ::lstat64
#define QT_TRUNCATE             ::truncate64

// File I/O
#define QT_OPEN                 ::open64
#define QT_LSEEK                ::lseek64
#define QT_FSTAT                ::fstat64
#define QT_FTRUNCATE            ::ftruncate64

// Standard C89
#define QT_FOPEN                ::fopen64
#define QT_FSEEK                ::fseeko64
#define QT_FTELL                ::ftello64
#define QT_FGETPOS              ::fgetpos64
#define QT_FSETPOS              ::fsetpos64

#define QT_MMAP                 ::mmap64

#else // !defined(QT_USE_XOPEN_LFS_EXTENSIONS) || !defined(QT_LARGEFILE_SUPPORT)

#include "../c89/qplatformdefs.h"

#define QT_STATBUF              struct stat

#define QT_STAT                 ::stat
#define QT_LSTAT                ::lstat
#define QT_TRUNCATE             ::truncate

// File I/O
#define QT_OPEN                 ::open
#define QT_LSEEK                ::lseek
#define QT_FSTAT                ::fstat
#define QT_FTRUNCATE            ::ftruncate

// Posix extensions to C89
#if !defined(QT_USE_XOPEN_LFS_EXTENSIONS) && !defined(QT_NO_USE_FSEEKO)
#undef QT_OFF_T
#undef QT_FSEEK
#undef QT_FTELL

#define QT_OFF_T                off_t

#define QT_FSEEK                ::fseeko
#define QT_FTELL                ::ftello
#endif

#define QT_MMAP                 ::mmap

#endif // !defined (QT_USE_XOPEN_LFS_EXTENSIONS) || !defined(QT_LARGEFILE_SUPPORT)

#define QT_STAT_MASK            S_IFMT
#define QT_STAT_REG             S_IFREG
#define QT_STAT_DIR             S_IFDIR
#define QT_STAT_LNK             S_IFLNK

#define QT_ACCESS               ::access
#define QT_GETCWD               ::getcwd
#define QT_CHDIR                ::chdir
#define QT_MKDIR                ::mkdir
#define QT_RMDIR                ::rmdir

// File I/O
#define QT_CLOSE                ::close
#define QT_READ                 ::read
#define QT_WRITE                ::write

#define QT_OPEN_LARGEFILE       O_LARGEFILE
#define QT_OPEN_RDONLY          O_RDONLY
#define QT_OPEN_WRONLY          O_WRONLY
#define QT_OPEN_RDWR            O_RDWR
#define QT_OPEN_CREAT           O_CREAT
#define QT_OPEN_TRUNC           O_TRUNC
#define QT_OPEN_APPEND          O_APPEND
#define QT_OPEN_EXCL            O_EXCL

// Posix extensions to C89
#define QT_FILENO               fileno

// Directory iteration
#define QT_DIR                  DIR

#define QT_OPENDIR              ::opendir
#define QT_CLOSEDIR             ::closedir

#if defined(QT_LARGEFILE_SUPPORT) \
        && defined(QT_USE_XOPEN_LFS_EXTENSIONS) \
        && !defined(QT_NO_READDIR64)
#define QT_DIRENT               struct dirent64
#define QT_READDIR              ::readdir64
#define QT_READDIR_R            ::readdir64_r
#else
#define QT_DIRENT               struct dirent
#define QT_READDIR              ::readdir
#define QT_READDIR_R            ::readdir_r
#endif

#define QT_SOCKLEN_T            socklen_t

#define QT_SOCKET_CONNECT       ::connect
#define QT_SOCKET_BIND          ::bind

#define QT_SIGNAL_RETTYPE       void
#define QT_SIGNAL_ARGS          int
#define QT_SIGNAL_IGNORE        SIG_IGN

#endif // include guard
