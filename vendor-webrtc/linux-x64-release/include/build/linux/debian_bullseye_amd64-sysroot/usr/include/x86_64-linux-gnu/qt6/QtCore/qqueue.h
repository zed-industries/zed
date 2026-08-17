// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QQUEUE_H
#define QQUEUE_H

#include <QtCore/qlist.h>

QT_BEGIN_NAMESPACE


template <class T>
class QQueue : public QList<T>
{
public:
    // compiler-generated special member functions are fine!
    inline void swap(QQueue<T> &other) noexcept { QList<T>::swap(other); } // prevent QList<->QQueue swaps
    inline void enqueue(const T &t) { QList<T>::append(t); }
    inline T dequeue() { return QList<T>::takeFirst(); }
    inline T &head() { return QList<T>::first(); }
    inline const T &head() const { return QList<T>::first(); }
};

QT_END_NAMESPACE

#endif // QQUEUE_H
