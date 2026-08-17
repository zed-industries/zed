// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_COMPILERTEST_H
#define QTCONCURRENT_COMPILERTEST_H

#include <QtConcurrent/qtconcurrent_global.h>

#ifndef QT_NO_CONCURRENT

QT_BEGIN_NAMESPACE

namespace QtPrivate {

    template <class T, typename = void>
    struct IsIterable : std::false_type {};
    template <class T>
    struct IsIterable<T, std::void_t<decltype(std::declval<T>().begin()),
        decltype(std::declval<T>().end())>>
        : std::true_type
    { };

    template <class T>
    inline constexpr bool IsIterableValue = IsIterable<T>::value;

    template <class T, typename = void>
    struct IsDereferenceable : std::false_type {};
    template <class T>
    struct IsDereferenceable<T, std::void_t<decltype(*std::declval<T>())>>
        : std::true_type
    { };

    template <class T>
    inline constexpr bool IsDereferenceableValue = IsDereferenceable<T>::value;
}

QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
