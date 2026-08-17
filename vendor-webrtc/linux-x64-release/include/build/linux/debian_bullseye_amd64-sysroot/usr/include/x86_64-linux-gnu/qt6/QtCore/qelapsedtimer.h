// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QELAPSEDTIMER_H
#define QELAPSEDTIMER_H

#include <QtCore/qglobal.h>

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QElapsedTimer
{
public:
    enum ClockType {
        SystemTime,
        MonotonicClock,
        TickCounter Q_DECL_ENUMERATOR_DEPRECATED_X(
                "Not supported anymore. Use PerformanceCounter instead."),
        MachAbsoluteTime,
        PerformanceCounter
    };

    constexpr QElapsedTimer()
        : t1(Q_INT64_C(0x8000000000000000)),
          t2(Q_INT64_C(0x8000000000000000))
    {
    }

    static ClockType clockType() noexcept;
    static bool isMonotonic() noexcept;

    void start() noexcept;
    qint64 restart() noexcept;
    void invalidate() noexcept;
    bool isValid() const noexcept;

    qint64 nsecsElapsed() const noexcept;
    qint64 elapsed() const noexcept;
    bool hasExpired(qint64 timeout) const noexcept;

    qint64 msecsSinceReference() const noexcept;
    qint64 msecsTo(const QElapsedTimer &other) const noexcept;
    qint64 secsTo(const QElapsedTimer &other) const noexcept;

    friend bool operator==(const QElapsedTimer &lhs, const QElapsedTimer &rhs) noexcept
    { return lhs.t1 == rhs.t1 && lhs.t2 == rhs.t2; }
    friend bool operator!=(const QElapsedTimer &lhs, const QElapsedTimer &rhs) noexcept
    { return !(lhs == rhs); }

    friend bool Q_CORE_EXPORT operator<(const QElapsedTimer &lhs, const QElapsedTimer &rhs) noexcept;

private:
    qint64 t1;
    qint64 t2;
};

QT_END_NAMESPACE

#endif // QELAPSEDTIMER_H
