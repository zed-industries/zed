// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QVECTOR_H
#define QVECTOR_H

#include <QtCore/qlist.h>
#include <QtCore/qcontainerfwd.h>

#if 0
#pragma qt_class(QVector)
#pragma qt_class(QMutableVectorIterator)
#pragma qt_class(QVectorIterator)
#endif

QT_BEGIN_NAMESPACE

#if !defined(QT_NO_JAVA_STYLE_ITERATORS)
template<typename T>
using QMutableVectorIterator = QMutableListIterator<T>;
template<typename T>
using QVectorIterator = QListIterator<T>;
#endif

QT_END_NAMESPACE

#endif // QVECTOR_H
