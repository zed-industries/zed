// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPLATFORMDEFS_H
#define QPLATFORMDEFS_H

// Get Qt defines/settings

#include "qglobal.h"

// Set any POSIX/XOPEN defines at the top of this file to turn on specific APIs
#ifndef _POSIX_PTHREAD_SEMANTICS
#define _POSIX_PTHREAD_SEMANTICS
#endif

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

#undef QT_SOCKLEN_T
#undef QT_SOCKET_CONNECT
#undef QT_SOCKET_BIND

#define QT_SOCKET_CONNECT       qt_socket_connect
#define QT_SOCKET_BIND          qt_socket_bind

#if defined(_XOPEN_SOURCE) && (_XOPEN_SOURCE-0 >= 500) && (_XOPEN_VERSION-0 >= 500)
// Solaris 7 and better with specific feature test macros
#define QT_SOCKLEN_T            socklen_t
#elif defined(_XOPEN_SOURCE_EXTENDED) && defined(_XOPEN_UNIX)
// Solaris 2.6 and better with specific feature test macros
#define QT_SOCKLEN_T            size_t
#else
// always this case in practice
#define QT_SOCKLEN_T            int
#endif

// Solaris redefines connect -> __xnet_connect with _XOPEN_SOURCE_EXTENDED
static inline int qt_socket_connect(int s, struct sockaddr *addr, QT_SOCKLEN_T addrlen)
{ return  ::connect(s, addr, addrlen); }
#if defined (connect)
# undef connect
#endif

// Solaris redefines bind -> __xnet_bind with _XOPEN_SOURCE_EXTENDED
static inline int qt_socket_bind(int s, struct sockaddr *addr, QT_SOCKLEN_T addrlen)
{ return ::bind(s, addr, addrlen); }
#if defined(bind)
# undef bind
#endif

#if !defined(_XOPEN_UNIX)
// Solaris 2.5.1
// Function usleep() is defined in C library but not declared in header files
// on Solaris 2.5.1. Not really a surprise, usleep() is specified by XPG4v2
// and XPG4v2 is only supported by Solaris 2.6 and better.
// Function gethostname() is also defined in C library but not declared in
// header files on Solaris 2.5.1.
typedef unsigned int useconds_t;
extern "C" int usleep(useconds_t);
extern "C" int gethostname(char *, int);
#endif

#if defined(_XOPEN_UNIX)
// Solaris 2.6 and better
#define QT_SNPRINTF             ::snprintf
#define QT_VSNPRINTF            ::vsnprintf
#endif

#endif // QPLATFORMDEFS_H
