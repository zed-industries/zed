// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#include <QtCore/qglobal.h>

#ifndef QCONTAINERFWD_H
#define QCONTAINERFWD_H

// std headers can unfortunately not be forward declared
#include <utility>

QT_BEGIN_NAMESPACE

template <typename Key, typename T> class QCache;
template <typename Key, typename T> class QHash;
template <typename Key, typename T> class QMap;
template <typename Key, typename T> class QMultiHash;
template <typename Key, typename T> class QMultiMap;
template <typename T1, typename T2>
using QPair = std::pair<T1, T2>;
template <typename T> class QQueue;
template <typename T> class QSet;
template <typename T> class QStack;
template <typename T, qsizetype Prealloc = 256> class QVarLengthArray;
template <typename T> class QList;
#ifndef Q_CLANG_QDOC
template<typename T> using QVector = QList<T>;
using QStringList = QList<QString>;
using QByteArrayList = QList<QByteArray>;
#else
template<typename T> class QVector;
class QStringList;
class QByteArrayList;
#endif
class QMetaType;
class QVariant;

using QVariantList = QList<QVariant>;
using QVariantMap = QMap<QString, QVariant>;
using QVariantHash = QHash<QString, QVariant>;
using QVariantPair = QPair<QVariant, QVariant>;

QT_END_NAMESPACE

#endif // QCONTAINERFWD_H
