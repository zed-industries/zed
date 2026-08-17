// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

    QWaitConditionPrivate *d;
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
