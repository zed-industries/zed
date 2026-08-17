// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#define _POSIX_TIMERS

#include "qglobal.h"

// extra disabling.
#ifdef __native_client__
#define QT_NO_FSFILEENGINE
#endif

#define QT_NO_SOCKET_H

#define DIR void *
#define PATH_MAX 256

#include "qfunctions_nacl.h"
#include <pthread.h>
