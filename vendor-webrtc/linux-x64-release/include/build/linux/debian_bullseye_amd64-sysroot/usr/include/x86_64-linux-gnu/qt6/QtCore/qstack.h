// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTACK_H
#define QSTACK_H

#include <QtCore/qlist.h>

QT_BEGIN_NAMESPACE

template<class T>
class QStack : public QList<T>
{
public:
    // compiler-generated special member functions are fine!
    void swap(QStack<T> &other) noexcept { QList<T>::swap(other); } // prevent QList<->QStack swaps
    void push(const T &t) { QList<T>::append(t); }
    T pop() { return QList<T>::takeLast(); }
    T &top() { return QList<T>::last(); }
    const T &top() const { return QList<T>::last(); }
};

QT_END_NAMESPACE

#endif // QSTACK_H
