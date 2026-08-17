// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPLATFORMDEFS_H
#define QPLATFORMDEFS_H

// Set any POSIX/XOPEN defines at the top of this file to turn on specific APIs
#define _XOPEN_SOURCE 500
#define __EXTENSIONS__

// Get Qt defines/settings

#include "qglobal.h"

#include <unistd.h>

// We are hot - unistd.h should have turned on the specific APIs we requested

#include <pthread.h>
#include <dirent.h>
#include <fcntl.h>
#include <grp.h>
#include <pwd.h>
#include <signal.h>
#include <dlfcn.h>

#include <sys/types.h>
#include <sys/ioctl.h>
#include <sys/filio.h>
#include <sys/ipc.h>
#include <sys/time.h>
#include <sys/shm.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <netinet/in.h>

#define QT_USE_XOPEN_LFS_EXTENSIONS
#include "../common/posix/qplatformdefs.h"

#undef QT_SOCKET_CONNECT
#define QT_SOCKET_CONNECT       qt_socket_connect

// Solaris redefines connect -> __xnet_connect with _XOPEN_SOURCE_EXTENDED
static inline int qt_socket_connect(int s, struct sockaddr *addr, QT_SOCKLEN_T addrlen)
{ return ::connect(s, addr, addrlen); }

// Only Solaris 7 and better support 64-bit
#define QT_SNPRINTF             ::snprintf
#define QT_VSNPRINTF            ::vsnprintf

#ifdef connect
#undef connect
#endif

#endif // QPLATFORMDEFS_H
