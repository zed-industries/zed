// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

enum QtMsgType {
    QtDebugMsg,
    QtWarningMsg,
    QtCriticalMsg,
    QtFatalMsg,
    QtInfoMsg,
    QtSystemMsg = QtCriticalMsg
};

class QMessageLogContext
{
    Q_DISABLE_COPY(QMessageLogContext)
public:
    constexpr QMessageLogContext() noexcept = default;
    constexpr QMessageLogContext(const char *fileName, int lineNumber, const char *functionName, const char *categoryName) noexcept
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
    constexpr QMessageLogger() : context() {}
    constexpr QMessageLogger(const char *file, int line, const char *function)
        : context(file, line, function, "default") {}
    constexpr QMessageLogger(const char *file, int line, const char *function, const char *category)
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
    Q_DECL_COLD_FUNCTION
    QDebug warning() const;
    Q_DECL_COLD_FUNCTION
    QDebug warning(const QLoggingCategory &cat) const;
    Q_DECL_COLD_FUNCTION
    QDebug warning(CategoryFunction catFunc) const;
    Q_DECL_COLD_FUNCTION
    QDebug critical() const;
    Q_DECL_COLD_FUNCTION
    QDebug critical(const QLoggingCategory &cat) const;
    Q_DECL_COLD_FUNCTION
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

typedef void (*QtMessageHandler)(QtMsgType, const QMessageLogContext &, const QString &);
Q_CORE_EXPORT QtMessageHandler qInstallMessageHandler(QtMessageHandler);

Q_CORE_EXPORT void qSetMessagePattern(const QString &messagePattern);
Q_CORE_EXPORT QString qFormatLogMessage(QtMsgType type, const QMessageLogContext &context,
                                        const QString &buf);

QT_END_NAMESPACE
#endif // QLOGGING_H
