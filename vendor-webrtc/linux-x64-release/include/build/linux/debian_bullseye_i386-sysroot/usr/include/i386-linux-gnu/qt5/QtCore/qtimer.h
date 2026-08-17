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

#ifndef QTIMER_H
#define QTIMER_H

#include <QtCore/qglobal.h>

#ifndef QT_NO_QOBJECT

#include <QtCore/qbasictimer.h> // conceptual inheritance
#include <QtCore/qobject.h>

#if __has_include(<chrono>)
#  include <chrono>
#endif

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QTimer : public QObject
{
    Q_OBJECT
    Q_PROPERTY(bool singleShot READ isSingleShot WRITE setSingleShot)
    Q_PROPERTY(int interval READ interval WRITE setInterval)
    Q_PROPERTY(int remainingTime READ remainingTime)
    Q_PROPERTY(Qt::TimerType timerType READ timerType WRITE setTimerType)
    Q_PROPERTY(bool active READ isActive)
public:
    explicit QTimer(QObject *parent = nullptr);
    ~QTimer();

    inline bool isActive() const { return id >= 0; }
    int timerId() const { return id; }

    void setInterval(int msec);
    int interval() const { return inter; }

    int remainingTime() const;

    void setTimerType(Qt::TimerType atype) { this->type = atype; }
    Qt::TimerType timerType() const { return Qt::TimerType(type); }

    inline void setSingleShot(bool singleShot);
    inline bool isSingleShot() const { return single; }

    static void singleShot(int msec, const QObject *receiver, const char *member);
    static void singleShot(int msec, Qt::TimerType timerType, const QObject *receiver, const char *member);

#ifdef Q_CLANG_QDOC
    template<typename PointerToMemberFunction>
    static void singleShot(int msec, const QObject *receiver, PointerToMemberFunction method);
    template<typename PointerToMemberFunction>
    static void singleShot(int msec, Qt::TimerType timerType, const QObject *receiver, PointerToMemberFunction method);
    template<typename Functor>
    static void singleShot(int msec, Functor functor);
    template<typename Functor>
    static void singleShot(int msec, Qt::TimerType timerType, Functor functor);
    template<typename Functor, int>
    static void singleShot(int msec, const QObject *context, Functor functor);
    template<typename Functor, int>
    static void singleShot(int msec, Qt::TimerType timerType, const QObject *context, Functor functor);
    template <typename Functor>
    QMetaObject::Connection callOnTimeout(Functor slot, Qt::ConnectionType connectionType = Qt::AutoConnection);
    template <typename Functor>
    QMetaObject::Connection callOnTimeout(const QObject *context, Functor slot, Qt::ConnectionType connectionType = Qt::AutoConnection);
    template <typename MemberFunction>
    QMetaObject::Connection callOnTimeout(const QObject *receiver, MemberFunction *slot, Qt::ConnectionType connectionType = Qt::AutoConnection);
#else
    // singleShot to a QObject slot
    template <typename Duration, typename Func1>
    static inline void singleShot(Duration interval, const typename QtPrivate::FunctionPointer<Func1>::Object *receiver, Func1 slot)
    {
        singleShot(interval, defaultTypeFor(interval), receiver, slot);
    }
    template <typename Duration, typename Func1>
    static inline void singleShot(Duration interval, Qt::TimerType timerType, const typename QtPrivate::FunctionPointer<Func1>::Object *receiver,
                                  Func1 slot)
    {
        typedef QtPrivate::FunctionPointer<Func1> SlotType;

        //compilation error if the slot has arguments.
        Q_STATIC_ASSERT_X(int(SlotType::ArgumentCount) == 0,
                          "The slot must not have any arguments.");

        singleShotImpl(interval, timerType, receiver,
                       new QtPrivate::QSlotObject<Func1, typename SlotType::Arguments, void>(slot));
    }
    // singleShot to a functor or function pointer (without context)
    template <typename Duration, typename Func1>
    static inline typename std::enable_if<!QtPrivate::FunctionPointer<Func1>::IsPointerToMemberFunction &&
                                          !std::is_same<const char*, Func1>::value, void>::type
            singleShot(Duration interval, Func1 slot)
    {
        singleShot(interval, defaultTypeFor(interval), nullptr, std::move(slot));
    }
    template <typename Duration, typename Func1>
    static inline typename std::enable_if<!QtPrivate::FunctionPointer<Func1>::IsPointerToMemberFunction &&
                                          !std::is_same<const char*, Func1>::value, void>::type
            singleShot(Duration interval, Qt::TimerType timerType, Func1 slot)
    {
        singleShot(interval, timerType, nullptr, std::move(slot));
    }
    // singleShot to a functor or function pointer (with context)
    template <typename Duration, typename Func1>
    static inline typename std::enable_if<!QtPrivate::FunctionPointer<Func1>::IsPointerToMemberFunction &&
                                          !std::is_same<const char*, Func1>::value, void>::type
            singleShot(Duration interval, const QObject *context, Func1 slot)
    {
        singleShot(interval, defaultTypeFor(interval), context, std::move(slot));
    }
    template <typename Duration, typename Func1>
    static inline typename std::enable_if<!QtPrivate::FunctionPointer<Func1>::IsPointerToMemberFunction &&
                                          !std::is_same<const char*, Func1>::value, void>::type
            singleShot(Duration interval, Qt::TimerType timerType, const QObject *context, Func1 slot)
    {
        //compilation error if the slot has arguments.
        typedef QtPrivate::FunctionPointer<Func1> SlotType;
        Q_STATIC_ASSERT_X(int(SlotType::ArgumentCount) <= 0,  "The slot must not have any arguments.");

        singleShotImpl(interval, timerType, context,
                       new QtPrivate::QFunctorSlotObject<Func1, 0,
                            typename QtPrivate::List_Left<void, 0>::Value, void>(std::move(slot)));
    }

    template <typename ... Args>
    QMetaObject::Connection callOnTimeout(Args && ...args)
    {
        return QObject::connect(this, &QTimer::timeout, std::forward<Args>(args)... );
    }

#endif

public Q_SLOTS:
    void start(int msec);

    void start();
    void stop();

Q_SIGNALS:
    void timeout(QPrivateSignal);

public:
#if __has_include(<chrono>) || defined(Q_QDOC)
    void setInterval(std::chrono::milliseconds value)
    {
        setInterval(int(value.count()));
    }

    std::chrono::milliseconds intervalAsDuration() const
    {
        return std::chrono::milliseconds(interval());
    }

    std::chrono::milliseconds remainingTimeAsDuration() const
    {
        return std::chrono::milliseconds(remainingTime());
    }

    static void singleShot(std::chrono::milliseconds value, const QObject *receiver, const char *member)
    {
        singleShot(int(value.count()), receiver, member);
    }

    static void singleShot(std::chrono::milliseconds value, Qt::TimerType timerType, const QObject *receiver, const char *member)
    {
        singleShot(int(value.count()), timerType, receiver, member);
    }

    void start(std::chrono::milliseconds value)
    {
        start(int(value.count()));
    }
#endif

protected:
    void timerEvent(QTimerEvent *) override;

private:
    Q_DISABLE_COPY(QTimer)

    inline int startTimer(int){ return -1;}
    inline void killTimer(int){}

    static Q_DECL_CONSTEXPR Qt::TimerType defaultTypeFor(int msecs) noexcept
    { return msecs >= 2000 ? Qt::CoarseTimer : Qt::PreciseTimer; }
    static void singleShotImpl(int msec, Qt::TimerType timerType,
                               const QObject *receiver, QtPrivate::QSlotObjectBase *slotObj);

#if __has_include(<chrono>)
    static Qt::TimerType defaultTypeFor(std::chrono::milliseconds interval)
    { return defaultTypeFor(int(interval.count())); }

    static void singleShotImpl(std::chrono::milliseconds interval, Qt::TimerType timerType,
                               const QObject *receiver, QtPrivate::QSlotObjectBase *slotObj)
    {
        singleShotImpl(int(interval.count()),
                       timerType, receiver, slotObj);
    }
#endif

    int id, inter, del;
    uint single : 1;
    uint nulltimer : 1;
    uint type : 2;
    // reserved : 28
};

inline void QTimer::setSingleShot(bool asingleShot) { single = asingleShot; }

QT_END_NAMESPACE

#endif // QT_NO_QOBJECT

#endif // QTIMER_H
