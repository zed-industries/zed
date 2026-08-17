// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPLATFORMDEFS_H
#define QPLATFORMDEFS_H

// Get Qt defines/settings

#include "qglobal.h"

// Set any POSIX/XOPEN defines at the top of this file to turn on specific APIs

#include <unistd.h>


// We are hot - unistd.h should have turned on the specific APIs we requested


#include <pthread.h>
#include <dirent.h>
#include <fcntl.h>
#include <grp.h>
#include <pwd.h>
#include <signal.h>
#define QT_NO_LIBRARY_UNLOAD

#include <sys/types.h>
#include <sys/ioctl.h>
#include <sys/ipc.h>
#include <sys/time.h>
#include <sys/shm.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/wait.h>
#define __APPLE_USE_RFC_3542
#include <netinet/in.h>

#include "../posix/qplatformdefs.h"

#undef QT_OPEN_LARGEFILE
#undef QT_SOCKLEN_T
#undef QT_SIGNAL_IGNORE

#define QT_OPEN_LARGEFILE       0

#define QT_SOCKLEN_T            socklen_t

#define QT_SIGNAL_IGNORE        (void (*)(int))1

#define QT_SNPRINTF             ::snprintf
#define QT_VSNPRINTF            ::vsnprintf

#endif // QPLATFORMDEFS_H
