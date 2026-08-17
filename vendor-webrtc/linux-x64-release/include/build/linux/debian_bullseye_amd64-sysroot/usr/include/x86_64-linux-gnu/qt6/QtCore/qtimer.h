// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTIMER_H
#define QTIMER_H

#include <QtCore/qglobal.h>

#ifndef QT_NO_QOBJECT

#include <QtCore/qbasictimer.h> // conceptual inheritance
#include <QtCore/qobject.h>

#include <chrono>

QT_BEGIN_NAMESPACE

class QTimerPrivate;
class Q_CORE_EXPORT QTimer : public QObject
{
    Q_OBJECT
    Q_PROPERTY(bool singleShot READ isSingleShot WRITE setSingleShot BINDABLE bindableSingleShot)
    Q_PROPERTY(int interval READ interval WRITE setInterval BINDABLE bindableInterval)
    Q_PROPERTY(int remainingTime READ remainingTime)
    Q_PROPERTY(Qt::TimerType timerType READ timerType WRITE setTimerType BINDABLE bindableTimerType)
    Q_PROPERTY(bool active READ isActive STORED false BINDABLE bindableActive)
public:
    explicit QTimer(QObject *parent = nullptr);
    ~QTimer();

    bool isActive() const;
    QBindable<bool> bindableActive();
    int timerId() const;

    void setInterval(int msec);
    int interval() const;
    QBindable<int> bindableInterval();

    int remainingTime() const;

    void setTimerType(Qt::TimerType atype);
    Qt::TimerType timerType() const;
    QBindable<Qt::TimerType> bindableTimerType();

    void setSingleShot(bool singleShot);
    bool isSingleShot() const;
    QBindable<bool> bindableSingleShot();

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
        static_assert(int(SlotType::ArgumentCount) == 0,
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
        static_assert(int(SlotType::ArgumentCount) <= 0,  "The slot must not have any arguments.");

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

protected:
    void timerEvent(QTimerEvent *) override;

private:
    Q_DISABLE_COPY(QTimer)
    Q_DECLARE_PRIVATE(QTimer)

    inline int startTimer(int){ return -1;}
    inline void killTimer(int){}

    static constexpr Qt::TimerType defaultTypeFor(int msecs) noexcept
    { return msecs >= 2000 ? Qt::CoarseTimer : Qt::PreciseTimer; }
    static void singleShotImpl(int msec, Qt::TimerType timerType,
                               const QObject *receiver, QtPrivate::QSlotObjectBase *slotObj);

    static Qt::TimerType defaultTypeFor(std::chrono::milliseconds interval)
    { return defaultTypeFor(int(interval.count())); }

    static void singleShotImpl(std::chrono::milliseconds interval, Qt::TimerType timerType,
                               const QObject *receiver, QtPrivate::QSlotObjectBase *slotObj)
    {
        singleShotImpl(int(interval.count()),
                       timerType, receiver, slotObj);
    }
};

QT_END_NAMESPACE

#endif // QT_NO_QOBJECT

#endif // QTIMER_H
