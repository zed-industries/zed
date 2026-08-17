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

#ifndef QWAITCONDITION_H
#define QWAITCONDITION_H

#include <QtCore/QDeadlineTimer>

QT_BEGIN_NAMESPACE

#if QT_CONFIG(thread)

class QWaitConditionPrivate;
class QMutex;
class QReadWriteLock;

class Q_CORE_EXPORT QWaitCondition
{
public:
    QWaitCondition();
    ~QWaitCondition();

    bool wait(QMutex *lockedMutex,
              QDeadlineTimer deadline = QDeadlineTimer(QDeadlineTimer::Forever));
    bool wait(QMutex *lockedMutex, unsigned long time);

    bool wait(QReadWriteLock *lockedReadWriteLock,
              QDeadlineTimer deadline = QDeadlineTimer(QDeadlineTimer::Forever));
    bool wait(QReadWriteLock *lockedReadWriteLock, unsigned long time);

    void wakeOne();
    void wakeAll();

    void notify_one() { wakeOne(); }
    void notify_all() { wakeAll(); }

private:
    Q_DISABLE_COPY(QWaitCondition)

    QWaitConditionPrivate * d;
};

#else

class QMutex;
class QReadWriteLock;

class Q_CORE_EXPORT QWaitCondition
{
public:
    QWaitCondition() {}
    ~QWaitCondition() {}

    bool wait(QMutex *, QDeadlineTimer = QDeadlineTimer(QDeadlineTimer::Forever))
    { return true; }
    bool wait(QReadWriteLock *, QDeadlineTimer = QDeadlineTimer(QDeadlineTimer::Forever))
    { return true; }
    bool wait(QMutex *, unsigned long) { return true; }
    bool wait(QReadWriteLock *, unsigned long) { return true; }

    void wakeOne() {}
    void wakeAll() {}

    void notify_one() { wakeOne(); }
    void notify_all() { wakeAll(); }
};

#endif // QT_CONFIG(thread)

QT_END_NAMESPACE

#endif // QWAITCONDITION_H
