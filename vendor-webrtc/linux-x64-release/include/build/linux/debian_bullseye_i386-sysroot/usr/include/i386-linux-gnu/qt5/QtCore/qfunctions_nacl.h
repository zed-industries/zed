/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QFUNCTIONS_NACL_H
#define QFUNCTIONS_NACL_H

#include <QtCore/qglobal.h>

#ifdef Q_OS_NACL

#include <sys/types.h>

// pthread
#include <pthread.h>
#define PTHREAD_CANCEL_DISABLE 1
#define PTHREAD_CANCEL_ENABLE 2
#define PTHREAD_INHERIT_SCHED 3

QT_BEGIN_NAMESPACE


extern "C" {

void pthread_cleanup_push(void (*handler)(void *), void *arg);
void pthread_cleanup_pop(int execute);

int pthread_setcancelstate(int state, int *oldstate);
int pthread_setcanceltype(int type, int *oldtype);
void pthread_testcancel(void);
int pthread_cancel(pthread_t thread);

int pthread_attr_setinheritsched(pthread_attr_t *attr,
    int inheritsched);
int pthread_attr_getinheritsched(const pthread_attr_t *attr,
    int *inheritsched);

// event dispatcher, select
//struct fd_set;
//struct timeval;
int fcntl(int fildes, int cmd, ...);
int sigaction(int sig, const struct sigaction * act, struct sigaction * oact);

typedef long off64_t;
off64_t ftello64(void *stream);
off64_t lseek64(int fildes, off_t offset, int whence);
int open64(const char *path, int oflag, ...);

}

int select(int nfds, fd_set * readfds, fd_set * writefds, fd_set * errorfds, struct timeval * timeout);

QT_END_NAMESPACE

#endif //Q_OS_NACL

#endif //QFUNCTIONS_NACL_H
