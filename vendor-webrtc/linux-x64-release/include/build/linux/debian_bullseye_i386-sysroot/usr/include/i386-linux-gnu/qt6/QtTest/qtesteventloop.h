// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTESTEVENTLOOP_H
#define QTESTEVENTLOOP_H

#include <QtTest/qttestglobal.h>
#include <QtTest/qtestcase.h>

#include <QtCore/qcoreapplication.h>
#include <QtCore/qeventloop.h>
#include <QtCore/qobject.h>
#include <QtCore/qpointer.h>
#include <QtCore/qthread.h>

QT_BEGIN_NAMESPACE


class Q_TESTLIB_EXPORT QTestEventLoop : public QObject
{
    Q_OBJECT

public:
    QTestEventLoop(QObject *parent = nullptr)
        : QObject(parent), _timeout(false)
    {}

    inline void enterLoopMSecs(int ms);
    inline void enterLoop(int secs) { enterLoopMSecs(secs * 1000); }

    inline void changeInterval(int secs)
    { killTimer(timerId); timerId = startTimer(secs * 1000); }

    inline bool timeout() const
    { return _timeout; }

    inline static QTestEventLoop &instance()
    {
        static QPointer<QTestEventLoop> testLoop;
        if (testLoop.isNull())
            testLoop = new QTestEventLoop(QCoreApplication::instance());
        return *static_cast<QTestEventLoop *>(testLoop);
    }

public Q_SLOTS:
    inline void exitLoop();

protected:
    inline void timerEvent(QTimerEvent *e) override;

private:
    QEventLoop *loop = nullptr;
    int timerId = -1;
    uint _timeout   :1;
    Q_DECL_UNUSED_MEMBER uint reserved   :31;
};

inline void QTestEventLoop::enterLoopMSecs(int ms)
{
    Q_ASSERT(!loop);
    _timeout = false;

    if (QTest::runningTest() && QTest::currentTestFailed())
        return;

    QEventLoop l;
    // if tests want to measure sub-second precision, use a precise timer
    timerId = startTimer(ms, ms < 1000 ? Qt::PreciseTimer : Qt::CoarseTimer);

    loop = &l;
    l.exec();
    loop = nullptr;
}

inline void QTestEventLoop::exitLoop()
{
    if (thread() != QThread::currentThread())
    {
        QMetaObject::invokeMethod(this, "exitLoop", Qt::QueuedConnection);
        return;
    }

    if (timerId != -1)
        killTimer(timerId);
    timerId = -1;

    if (loop)
        loop->exit();
}

inline void QTestEventLoop::timerEvent(QTimerEvent *e)
{
    if (e->timerId() != timerId)
        return;
    _timeout = true;
    exitLoop();
}

QT_END_NAMESPACE

#endif
