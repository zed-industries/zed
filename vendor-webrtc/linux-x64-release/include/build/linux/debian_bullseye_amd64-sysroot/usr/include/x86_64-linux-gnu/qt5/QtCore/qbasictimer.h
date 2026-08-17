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

#ifndef QBASICTIMER_H
#define QBASICTIMER_H

#include <QtCore/qglobal.h>
#include <QtCore/qnamespace.h>

QT_BEGIN_NAMESPACE


class QObject;

class Q_CORE_EXPORT QBasicTimer
{
    int id;
#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    Q_DISABLE_COPY(QBasicTimer)
#elif QT_DEPRECATED_SINCE(5, 14)
public:
    // Just here to preserve BC, we can't remove them yet
    QT_DEPRECATED_X("copy-construction is unsupported; use move-construction instead")
    QBasicTimer(const QBasicTimer &);
    QT_DEPRECATED_X("copy-assignment is unsupported; use move-assignment instead")
    QBasicTimer &operator=(const QBasicTimer &);
#endif

public:
    constexpr QBasicTimer() noexcept : id{0} {}
    inline ~QBasicTimer() { if (id) stop(); }

    QBasicTimer(QBasicTimer &&other) noexcept
        : id{qExchange(other.id, 0)}
    {}

    QBasicTimer& operator=(QBasicTimer &&other) noexcept
    {
        QBasicTimer{std::move(other)}.swap(*this);
        return *this;
    }

    void swap(QBasicTimer &other) noexcept { qSwap(id, other.id); }

    bool isActive() const noexcept { return id != 0; }
    int timerId() const noexcept { return id; }

    void start(int msec, QObject *obj);
    void start(int msec, Qt::TimerType timerType, QObject *obj);
    void stop();
};
Q_DECLARE_TYPEINFO(QBasicTimer, Q_MOVABLE_TYPE);

inline void swap(QBasicTimer &lhs, QBasicTimer &rhs) noexcept { lhs.swap(rhs); }

QT_END_NAMESPACE

#endif // QBASICTIMER_H
