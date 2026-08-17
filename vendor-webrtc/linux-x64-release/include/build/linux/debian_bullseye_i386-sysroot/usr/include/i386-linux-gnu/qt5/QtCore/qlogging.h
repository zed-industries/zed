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

#include <QtCore/qglobal.h>

#ifndef QLOGGING_H
#define QLOGGING_H

#if 0
// header is automatically included in qglobal.h
#pragma qt_no_master_include
#endif

QT_BEGIN_NAMESPACE

/*
  Forward declarations only.

  In order to use the qDebug() stream, you must #include<QDebug>
*/
class QDebug;
class QNoDebug;

enum QtMsgType { QtDebugMsg, QtWarningMsg, QtCriticalMsg, QtFatalMsg, QtInfoMsg, QtSystemMsg = QtCriticalMsg };

class QMessageLogContext
{
    Q_DISABLE_COPY(QMessageLogContext)
public:
    Q_DECL_CONSTEXPR QMessageLogContext() noexcept = default;
    Q_DECL_CONSTEXPR QMessageLogContext(const char *fileName, int lineNumber, const char *functionName, const char *categoryName) noexcept
        : line(lineNumber), file(fileName), function(functionName), category(categoryName) {}

    int version = 2;
    int line = 0;
    const char *file = nullptr;
    const char *function = nullptr;
    const char *category = nullptr;

private:
    QMessageLogContext &copyContextFrom(const QMessageLogContext &logContext) noexcept;

    friend class QMessageLogger;
    friend class QDebug;
};

class QLoggingCategory;

class Q_CORE_EXPORT QMessageLogger
{
    Q_DISABLE_COPY(QMessageLogger)
public:
    Q_DECL_CONSTEXPR QMessageLogger() : context() {}
    Q_DECL_CONSTEXPR QMessageLogger(const char *file, int line, const char *function)
        : context(file, line, function, "default") {}
    Q_DECL_CONSTEXPR QMessageLogger(const char *file, int line, const char *function, const char *category)
        : context(file, line, function, category) {}

    void debug(const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(2, 3);
    void noDebug(const char *, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(2, 3)
    {}
    void info(const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(2, 3);
    Q_DECL_COLD_FUNCTION
    void warning(const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(2, 3);
    Q_DECL_COLD_FUNCTION
    void critical(const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(2, 3);

    typedef const QLoggingCategory &(*CategoryFunction)();

    void debug(const QLoggingCategory &cat, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);
    void debug(CategoryFunction catFunc, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);
    void info(const QLoggingCategory &cat, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);
    void info(CategoryFunction catFunc, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);
    Q_DECL_COLD_FUNCTION
    void warning(const QLoggingCategory &cat, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);
    Q_DECL_COLD_FUNCTION
    void warning(CategoryFunction catFunc, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);
    Q_DECL_COLD_FUNCTION
    void critical(const QLoggingCategory &cat, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);
    Q_DECL_COLD_FUNCTION
    void critical(CategoryFunction catFunc, const char *msg, ...) const Q_ATTRIBUTE_FORMAT_PRINTF(3, 4);

#ifndef Q_CC_MSVC
    Q_NORETURN
#endif
    Q_DECL_COLD_FUNCTION
    void fatal(const char *msg, ...) const noexcept Q_ATTRIBUTE_FORMAT_PRINTF(2, 3);

#ifndef QT_NO_DEBUG_STREAM
    QDebug debug() const;
    QDebug debug(const QLoggingCategory &cat) const;
    QDebug debug(CategoryFunction catFunc) const;
    QDebug info() const;
    QDebug info(const QLoggingCategory &cat) const;
    QDebug info(CategoryFunction catFunc) const;
    QDebug warning() const;
    QDebug warning(const QLoggingCategory &cat) const;
    QDebug warning(CategoryFunction catFunc) const;
    QDebug critical() const;
    QDebug critical(const QLoggingCategory &cat) const;
    QDebug critical(CategoryFunction catFunc) const;

    QNoDebug noDebug() const noexcept;
#endif // QT_NO_DEBUG_STREAM

private:
    QMessageLogContext context;
};

#if !defined(QT_MESSAGELOGCONTEXT) && !defined(QT_NO_MESSAGELOGCONTEXT)
#  if defined(QT_NO_DEBUG)
#    define QT_NO_MESSAGELOGCONTEXT
#  else
#    define QT_MESSAGELOGCONTEXT
#  endif
#endif

#ifdef QT_MESSAGELOGCONTEXT
  #define QT_MESSAGELOG_FILE static_cast<const char *>(__FILE__)
  #define QT_MESSAGELOG_LINE __LINE__
  #define QT_MESSAGELOG_FUNC static_cast<const char *>(Q_FUNC_INFO)
#else
  #define QT_MESSAGELOG_FILE nullptr
  #define QT_MESSAGELOG_LINE 0
  #define QT_MESSAGELOG_FUNC nullptr
#endif

#define qDebug QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC).debug
#define qInfo QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC).info
#define qWarning QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC).warning
#define qCritical QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC).critical
#define qFatal QMessageLogger(QT_MESSAGELOG_FILE, QT_MESSAGELOG_LINE, QT_MESSAGELOG_FUNC).fatal

#define QT_NO_QDEBUG_MACRO while (false) QMessageLogger().noDebug

#if defined(QT_NO_DEBUG_OUTPUT)
#  undef qDebug
#  define qDebug QT_NO_QDEBUG_MACRO
#endif
#if defined(QT_NO_INFO_OUTPUT)
#  undef qInfo
#  define qInfo QT_NO_QDEBUG_MACRO
#endif
#if defined(QT_NO_WARNING_OUTPUT)
#  undef qWarning
#  define qWarning QT_NO_QDEBUG_MACRO
#endif

Q_CORE_EXPORT void qt_message_output(QtMsgType, const QMessageLogContext &context,
                                     const QString &message);

Q_CORE_EXPORT Q_DECL_COLD_FUNCTION void qErrnoWarning(int code, const char *msg, ...);
Q_CORE_EXPORT Q_DECL_COLD_FUNCTION void qErrnoWarning(const char *msg, ...);

#if QT_DEPRECATED_SINCE(5, 0)// deprecated. Use qInstallMessageHandler instead!
typedef void (*QtMsgHandler)(QtMsgType, const char *);
Q_CORE_EXPORT QT_DEPRECATED QtMsgHandler qInstallMsgHandler(QtMsgHandler);
#endif

typedef void (*QtMessageHandler)(QtMsgType, const QMessageLogContext &, const QString &);
Q_CORE_EXPORT QtMessageHandler qInstallMessageHandler(QtMessageHandler);

Q_CORE_EXPORT void qSetMessagePattern(const QString &messagePattern);
Q_CORE_EXPORT QString qFormatLogMessage(QtMsgType type, const QMessageLogContext &context,
                                        const QString &buf);

QT_END_NAMESPACE
#endif // QLOGGING_H
