// Copyright (C) 2018 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTESTSUPPORT_CORE_H
#define QTESTSUPPORT_CORE_H

#include <QtCore/qcoreapplication.h>
#include <QtCore/qdeadlinetimer.h>
#include <QtCore/qthread.h>

QT_BEGIN_NAMESPACE

namespace QTest {

Q_CORE_EXPORT void qSleep(int ms);

template <typename Functor>
[[nodiscard]] static bool qWaitFor(Functor predicate, int timeout = 5000)
{
    // We should not spin the event loop in case the predicate is already true,
    // otherwise we might send new events that invalidate the predicate.
    if (predicate())
        return true;

    // qWait() is expected to spin the event loop, even when called with a small
    // timeout like 1ms, so we we can't use a simple while-loop here based on
    // the deadline timer not having timed out. Use do-while instead.

    int remaining = timeout;
    QDeadlineTimer deadline(remaining, Qt::PreciseTimer);

    do {
        // We explicitly do not pass the remaining time to processEvents, as
        // that would keep spinning processEvents for the whole duration if
        // new events were posted as part of processing events, and we need
        // to return back to this function to check the predicate between
        // each pass of processEvents. Our own timer will take care of the
        // timeout.
        QCoreApplication::processEvents(QEventLoop::AllEvents);
        QCoreApplication::sendPostedEvents(nullptr, QEvent::DeferredDelete);

        if (predicate())
            return true;

        remaining = int(deadline.remainingTime());
        if (remaining > 0)
            qSleep(qMin(10, remaining));
        remaining = int(deadline.remainingTime());
    } while (remaining > 0);

    return predicate(); // Last chance
}

Q_CORE_EXPORT void qWait(int ms);

} // namespace QTest

QT_END_NAMESPACE

#endif
