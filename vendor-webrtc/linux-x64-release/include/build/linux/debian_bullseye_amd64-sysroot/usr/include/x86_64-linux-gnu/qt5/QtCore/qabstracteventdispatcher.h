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

#ifndef QABSTRACTEVENTDISPATCHER_H
#define QABSTRACTEVENTDISPATCHER_H

#include <QtCore/qobject.h>
#include <QtCore/qeventloop.h>

QT_BEGIN_NAMESPACE

class QAbstractNativeEventFilter;
class QAbstractEventDispatcherPrivate;
class QSocketNotifier;

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
class QWinEventNotifier;
#endif

class Q_CORE_EXPORT QAbstractEventDispatcher : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QAbstractEventDispatcher)

public:
    struct TimerInfo
    {
        int timerId;
        int interval;
        Qt::TimerType timerType;

        inline TimerInfo(int id, int i, Qt::TimerType t)
            : timerId(id), interval(i), timerType(t)
        { }
    };

    explicit QAbstractEventDispatcher(QObject *parent = nullptr);
    ~QAbstractEventDispatcher();

    static QAbstractEventDispatcher *instance(QThread *thread = nullptr);

    virtual bool processEvents(QEventLoop::ProcessEventsFlags flags) = 0;
    virtual bool hasPendingEvents() = 0; // ### Qt6: remove, mark final or make protected

    virtual void registerSocketNotifier(QSocketNotifier *notifier) = 0;
    virtual void unregisterSocketNotifier(QSocketNotifier *notifier) = 0;

#if QT_DEPRECATED_SINCE(5,0)
    QT_DEPRECATED inline int registerTimer(int interval, QObject *object)
    { return registerTimer(interval, Qt::CoarseTimer, object); }
    QT_DEPRECATED inline void registerTimer(int timerId, int interval, QObject *object)
    { registerTimer(timerId, interval, Qt::CoarseTimer, object); }
#endif
    // ### Qt6: change interval range to qint64 (or use QDeadlineTimer)
    int registerTimer(int interval, Qt::TimerType timerType, QObject *object);
    virtual void registerTimer(int timerId, int interval, Qt::TimerType timerType, QObject *object) = 0;
    virtual bool unregisterTimer(int timerId) = 0;
    virtual bool unregisterTimers(QObject *object) = 0;
    virtual QList<TimerInfo> registeredTimers(QObject *object) const = 0;

    virtual int remainingTime(int timerId) = 0;

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
    virtual bool registerEventNotifier(QWinEventNotifier *notifier) = 0;
    virtual void unregisterEventNotifier(QWinEventNotifier *notifier) = 0;
#endif

    virtual void wakeUp() = 0;
    virtual void interrupt() = 0;
    virtual void flush() = 0; // ### Qt6: remove, mark final or make protected

    virtual void startingUp();
    virtual void closingDown();

    void installNativeEventFilter(QAbstractNativeEventFilter *filterObj);
    void removeNativeEventFilter(QAbstractNativeEventFilter *filterObj);
#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
    bool filterNativeEvent(const QByteArray &eventType, void *message, qintptr *result);
#else
    bool filterNativeEvent(const QByteArray &eventType, void *message, long *result);
#endif
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED bool filterEvent(void *message)
    { return filterNativeEvent("", message, nullptr); }
#endif

Q_SIGNALS:
    void aboutToBlock();
    void awake();

protected:
    QAbstractEventDispatcher(QAbstractEventDispatcherPrivate &,
                             QObject *parent);
};

Q_DECLARE_TYPEINFO(QAbstractEventDispatcher::TimerInfo, (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0) ? Q_PRIMITIVE_TYPE : Q_RELOCATABLE_TYPE));

QT_END_NAMESPACE

#endif // QABSTRACTEVENTDISPATCHER_H
