// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#include <QtCore/qglobal.h>

#ifndef QSTRINGFWD_H
#define QSTRINGFWD_H

QT_BEGIN_NAMESPACE

#if QT_VERSION >= QT_VERSION_CHECK(7, 0, 0)
# define QT_BEGIN_HAS_CHAR8_T_NAMESPACE inline namespace q_has_char8_t {
# define QT_BEGIN_NO_CHAR8_T_NAMESPACE namespace q_no_char8_t {
#else
# define QT_BEGIN_HAS_CHAR8_T_NAMESPACE namespace q_has_char8_t {
# define QT_BEGIN_NO_CHAR8_T_NAMESPACE inline namespace q_no_char8_t {
#endif
#define QT_END_HAS_CHAR8_T_NAMESPACE }
#define QT_END_NO_CHAR8_T_NAMESPACE }

// declare namespaces:
QT_BEGIN_HAS_CHAR8_T_NAMESPACE
QT_END_HAS_CHAR8_T_NAMESPACE
QT_BEGIN_NO_CHAR8_T_NAMESPACE
QT_END_NO_CHAR8_T_NAMESPACE

class QByteArray;
class QByteArrayView;
#if QT_VERSION >= QT_VERSION_CHECK(7, 0, 0) || defined(QT_BOOTSTRAPPED) || defined(Q_CLANG_QDOC)
class QLatin1StringView;
using QLatin1String = QLatin1StringView;
#else
class QLatin1String;
using QLatin1StringView = QLatin1String;
#endif
class QStringView;
template <bool> class QBasicUtf8StringView;
class QAnyStringView;
class QChar;
class QRegularExpression;
class QRegularExpressionMatch;

#ifndef Q_CLANG_QDOC
// ### Qt 7: remove the non-char8_t version of QUtf8StringView
QT_BEGIN_NO_CHAR8_T_NAMESPACE
using QUtf8StringView = QBasicUtf8StringView<false>;
QT_END_NO_CHAR8_T_NAMESPACE

QT_BEGIN_HAS_CHAR8_T_NAMESPACE
using QUtf8StringView = QBasicUtf8StringView<true>;
QT_END_HAS_CHAR8_T_NAMESPACE
#endif // Q_CLANG_QDOC

QT_END_NAMESPACE

#endif // QSTRINGFWD_H
