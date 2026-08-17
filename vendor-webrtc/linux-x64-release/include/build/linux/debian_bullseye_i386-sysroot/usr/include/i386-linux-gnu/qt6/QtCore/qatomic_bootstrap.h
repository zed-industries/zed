// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2011 Thiago Macieira <thiago@kde.org>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QATOMIC_BOOTSTRAP_H
#define QATOMIC_BOOTSTRAP_H

#include <QtCore/qgenericatomic.h>

QT_BEGIN_NAMESPACE

#if 0
// silence syncqt warnings
QT_END_NAMESPACE
#pragma qt_sync_skip_header_check
#pragma qt_sync_stop_processing
#endif

#define Q_ATOMIC_INT8_IS_SUPPORTED
template<> struct QAtomicOpsSupport<1> { enum { IsSupported = 1 }; };
#define Q_ATOMIC_INT16_IS_SUPPORTED
template<> struct QAtomicOpsSupport<2> { enum { IsSupported = 1 }; };
#define Q_ATOMIC_INT32_IS_SUPPORTED
#define Q_ATOMIC_INT64_IS_SUPPORTED
template<> struct QAtomicOpsSupport<8> { enum { IsSupported = 1 }; };

template <typename T> struct QAtomicOps: QGenericAtomicOps<QAtomicOps<T> >
{
    typedef T Type;

    static bool ref(T &_q_value) noexcept
    {
        return ++_q_value != 0;
    }
    static bool deref(T &_q_value) noexcept
    {
        return --_q_value != 0;
    }

    static bool testAndSetRelaxed(T &_q_value, T expectedValue, T newValue, T *currentValue = nullptr) noexcept
    {
        if (currentValue)
            *currentValue = _q_value;
        if (_q_value == expectedValue) {
            _q_value = newValue;
            return true;
        }
        return false;
    }

    static T fetchAndStoreRelaxed(T &_q_value, T newValue) noexcept
    {
        T tmp = _q_value;
        _q_value = newValue;
        return tmp;
    }

    template <typename AdditiveType> static
    T fetchAndAddRelaxed(T &_q_value, AdditiveType valueToAdd) noexcept
    {
        T returnValue = _q_value;
        _q_value += valueToAdd;
        return returnValue;
    }
};

QT_END_NAMESPACE

#endif // QATOMIC_BOOTSTRAP_H
