// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBASICTIMER_H
#define QBASICTIMER_H

#include <QtCore/qglobal.h>
#include <QtCore/qnamespace.h>

QT_BEGIN_NAMESPACE


class QObject;

class Q_CORE_EXPORT QBasicTimer
{
    int id;
    Q_DISABLE_COPY(QBasicTimer)

public:
    constexpr QBasicTimer() noexcept : id{0} {}
    inline ~QBasicTimer() { if (id) stop(); }

    QBasicTimer(QBasicTimer &&other) noexcept
        : id{qExchange(other.id, 0)}
    {}

    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QBasicTimer)

    void swap(QBasicTimer &other) noexcept { std::swap(id, other.id); }

    bool isActive() const noexcept { return id != 0; }
    int timerId() const noexcept { return id; }

    void start(int msec, QObject *obj);
    void start(int msec, Qt::TimerType timerType, QObject *obj);
    void stop();
};
Q_DECLARE_TYPEINFO(QBasicTimer, Q_RELOCATABLE_TYPE);

inline void swap(QBasicTimer &lhs, QBasicTimer &rhs) noexcept { lhs.swap(rhs); }

QT_END_NAMESPACE

#endif // QBASICTIMER_H
