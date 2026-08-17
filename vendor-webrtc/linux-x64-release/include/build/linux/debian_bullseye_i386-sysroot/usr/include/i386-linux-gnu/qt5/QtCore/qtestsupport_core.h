/****************************************************************************
**
** Copyright (C) 2018 The Qt Company Ltd.
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

#ifndef QTESTSUPPORT_CORE_H
#define QTESTSUPPORT_CORE_H

#include <QtCore/qcoreapplication.h>
#include <QtCore/qdeadlinetimer.h>

QT_BEGIN_NAMESPACE

namespace QTestPrivate {
Q_CORE_EXPORT void qSleep(int ms);
}

namespace QTest {

template <typename Functor>
Q_REQUIRED_RESULT static bool qWaitFor(Functor predicate, int timeout = 5000)
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

        remaining = deadline.remainingTime();
        if (remaining > 0)
            QTestPrivate::qSleep(qMin(10, remaining));

        if (predicate())
            return true;

        remaining = deadline.remainingTime();
    } while (remaining > 0);

    return predicate(); // Last chance
}

Q_CORE_EXPORT void qWait(int ms);

} // namespace QTest

QT_END_NAMESPACE

#endif
