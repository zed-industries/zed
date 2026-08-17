// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSCOPEDVALUEROLLBACK_H
#define QSCOPEDVALUEROLLBACK_H

#include <QtCore/qglobal.h>

QT_BEGIN_NAMESPACE

template <typename T>
class [[nodiscard]] QScopedValueRollback
{
public:
    explicit constexpr QScopedValueRollback(T &var)
        : varRef(var), oldValue(var)
    {
    }

    explicit constexpr QScopedValueRollback(T &var, T value)
        : varRef(var), oldValue(qExchange(var, std::move(value)))
    {
    }

#if __cpp_constexpr >= 201907L
    constexpr
#endif
    ~QScopedValueRollback()
    {
        varRef = std::move(oldValue);
    }

    constexpr void commit()
    {
        oldValue = varRef;
    }

private:
    T &varRef;
    T oldValue;

    Q_DISABLE_COPY_MOVE(QScopedValueRollback)
};

QT_END_NAMESPACE

#endif // QSCOPEDVALUEROLLBACK_H
