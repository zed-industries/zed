// Copyright (C) 2014 BlackBerry Limited. All rights reserved.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef Q_QNX_PLATFORMDEFS_H
#define Q_QNX_PLATFORMDEFS_H

// Get Qt defines/settings

#include "qglobal.h"

// Set any POSIX/XOPEN defines at the top of this file to turn on specific APIs

#include <unistd.h>

#define __STDC_CONSTANT_MACROS

// We are hot - unistd.h should have turned on the specific APIs we requested


#include <pthread.h>
#include <dirent.h>
#include <fcntl.h>
#include <grp.h>
#include <pwd.h>
#include <signal.h>

#include <sys/types.h>
#include <sys/ioctl.h>
#include <sys/ipc.h>
#include <sys/time.h>
// QNX doesn't have the System V <sys/shm.h> header. This is not a standard
// POSIX header, it's only documented in the Single UNIX Specification.
// The preferred POSIX compliant way to share memory is to use the functions
// in <sys/mman.h> that comply with the POSIX Real Time Interface (1003.1b).
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <netinet/in.h>

// for htonl
#include <arpa/inet.h>

#define QT_USE_XOPEN_LFS_EXTENSIONS
#if !defined(__EXT_QNX__READDIR64_R)
#define QT_NO_READDIR64
#endif
#include "../posix/qplatformdefs.h"
#if defined(__EXT_QNX__READDIR64_R)
#define QT_EXT_QNX_READDIR_R    ::_readdir64_r
#elif defined(__EXT_QNX__READDIR_R)
#define QT_EXT_QNX_READDIR_R    ::_readdir_r
#endif

#define QT_SNPRINTF ::snprintf
#define QT_VSNPRINTF ::vsnprintf

// QNX6 doesn't have getpagesize()
inline int getpagesize()
{
    return ::sysconf(_SC_PAGESIZE);
}

#include <stdlib.h>

#endif // Q_QNX_PLATFORMDEFS_H
