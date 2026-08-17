// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2019 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFOREACH_H
#define QFOREACH_H

#include <QtCore/qglobal.h>

QT_BEGIN_NAMESPACE

#if 0
#pragma qt_class(QForeach)
#pragma qt_sync_stop_processing
#endif

#ifndef QT_NO_FOREACH

namespace QtPrivate {

template <typename T>
class QForeachContainer {
    Q_DISABLE_COPY(QForeachContainer)
public:
    QForeachContainer(const T &t) : c(t), i(std::as_const(c).begin()), e(std::as_const(c).end()) {}
    QForeachContainer(T &&t) : c(std::move(t)), i(std::as_const(c).begin()), e(std::as_const(c).end())  {}

    QForeachContainer(QForeachContainer &&other)
        : c(std::move(other.c)),
          i(std::as_const(c).begin()),
          e(std::as_const(c).end()),
          control(std::move(other.control))
    {
    }

    QForeachContainer &operator=(QForeachContainer &&other)
    {
        c = std::move(other.c);
        i = std::as_const(c).begin();
        e = std::as_const(c).end();
        control = std::move(other.control);
        return *this;
    }

    T c;
    typename T::const_iterator i, e;
    int control = 1;
};

// Containers that have a detach function are considered shared, and are OK in a foreach loop
template <typename T, typename = decltype(std::declval<T>().detach())>
inline void warnIfContainerIsNotShared(int) {}

#if QT_DEPRECATED_SINCE(6, 0)
// Other containers will copy themselves if used in foreach, this use is deprecated
template <typename T>
QT_DEPRECATED_VERSION_X_6_0("Do not use foreach/Q_FOREACH with containers which are not implicitly shared. "
    "Prefer using a range-based for loop with these containers: `for (const auto &it : container)`, "
    "keeping in mind that range-based for doesn't copy the container as Q_FOREACH does")
inline void warnIfContainerIsNotShared(...) {}
#endif

template<typename T>
QForeachContainer<typename std::decay<T>::type> qMakeForeachContainer(T &&t)
{
    warnIfContainerIsNotShared<typename std::decay<T>::type>(0);
    return QForeachContainer<typename std::decay<T>::type>(std::forward<T>(t));
}

}

// Use C++17 if statement with initializer. User's code ends up in a else so
// scoping of different ifs is not broken
#define Q_FOREACH_IMPL(variable, name, container)                                             \
    for (auto name = QtPrivate::qMakeForeachContainer(container); name.i != name.e; ++name.i) \
        if (variable = *name.i; false) {} else

#define Q_FOREACH_JOIN(A, B) Q_FOREACH_JOIN_IMPL(A, B)
#define Q_FOREACH_JOIN_IMPL(A, B) A ## B

#define Q_FOREACH(variable, container) \
    Q_FOREACH_IMPL(variable, Q_FOREACH_JOIN(_container_, __LINE__), container)
#endif // QT_NO_FOREACH

#define Q_FOREVER for(;;)
#ifndef QT_NO_KEYWORDS
# ifndef QT_NO_FOREACH
#  ifndef foreach
#    define foreach Q_FOREACH
#  endif
# endif // QT_NO_FOREACH
#  ifndef forever
#    define forever Q_FOREVER
#  endif
#endif

QT_END_NAMESPACE

#endif /* QFOREACH_H */
