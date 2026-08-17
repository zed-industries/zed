/****************************************************************************
**
** Copyright (C) 2013 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QOPENGLDEBUG_H
#define QOPENGLDEBUG_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtCore/qshareddata.h>
#include <QtCore/qflags.h>
#include <QtCore/qlist.h>
#include <QtCore/qvector.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qdebug.h>
#include <QtGui/qopenglcontext.h>

#if defined(Q_CLANG_QDOC)
#undef GLuint
typedef unsigned int GLuint;
#endif

QT_BEGIN_NAMESPACE

class QOpenGLDebugLogger;
class QOpenGLDebugLoggerPrivate;
class QOpenGLDebugMessagePrivate;

class Q_GUI_EXPORT QOpenGLDebugMessage
{
public:
    enum Source {
        InvalidSource        = 0x00000000,
        APISource            = 0x00000001,
        WindowSystemSource   = 0x00000002,
        ShaderCompilerSource = 0x00000004,
        ThirdPartySource     = 0x00000008,
        ApplicationSource    = 0x00000010,
        OtherSource          = 0x00000020,
        LastSource           = OtherSource, // private API
        AnySource            = 0xffffffff
    };
    Q_DECLARE_FLAGS(Sources, Source)

    enum Type {
        InvalidType            = 0x00000000,
        ErrorType              = 0x00000001,
        DeprecatedBehaviorType = 0x00000002,
        UndefinedBehaviorType  = 0x00000004,
        PortabilityType        = 0x00000008,
        PerformanceType        = 0x00000010,
        OtherType              = 0x00000020,
        MarkerType             = 0x00000040,
        GroupPushType          = 0x00000080,
        GroupPopType           = 0x00000100,
        LastType               = GroupPopType, // private API
        AnyType                = 0xffffffff
    };
    Q_DECLARE_FLAGS(Types, Type)

    enum Severity {
        InvalidSeverity      = 0x00000000,
        HighSeverity         = 0x00000001,
        MediumSeverity       = 0x00000002,
        LowSeverity          = 0x00000004,
        NotificationSeverity = 0x00000008,
        LastSeverity         = NotificationSeverity, // private API
        AnySeverity          = 0xffffffff
    };
    Q_DECLARE_FLAGS(Severities, Severity)

    QOpenGLDebugMessage();
    QOpenGLDebugMessage(const QOpenGLDebugMessage &debugMessage);

    QOpenGLDebugMessage &operator=(const QOpenGLDebugMessage &debugMessage);
    QOpenGLDebugMessage &operator=(QOpenGLDebugMessage &&other) noexcept { swap(other); return *this; }
    ~QOpenGLDebugMessage();

    void swap(QOpenGLDebugMessage &other) noexcept { qSwap(d, other.d); }

    Source source() const;
    Type type() const;
    Severity severity() const;
    GLuint id() const;
    QString message() const;

    static QOpenGLDebugMessage createApplicationMessage(const QString &text,
                                                        GLuint id = 0,
                                                        Severity severity = NotificationSeverity,
                                                        Type type = OtherType);
    static QOpenGLDebugMessage createThirdPartyMessage(const QString &text,
                                                       GLuint id = 0,
                                                       Severity severity = NotificationSeverity,
                                                       Type type = OtherType);

    bool operator==(const QOpenGLDebugMessage &debugMessage) const;
    inline bool operator!=(const QOpenGLDebugMessage &debugMessage) const { return !operator==(debugMessage); }

private:
    friend class QOpenGLDebugLogger;
    friend class QOpenGLDebugLoggerPrivate;
    QSharedDataPointer<QOpenGLDebugMessagePrivate> d;
};

Q_DECLARE_SHARED(QOpenGLDebugMessage)
Q_DECLARE_OPERATORS_FOR_FLAGS(QOpenGLDebugMessage::Sources)
Q_DECLARE_OPERATORS_FOR_FLAGS(QOpenGLDebugMessage::Types)
Q_DECLARE_OPERATORS_FOR_FLAGS(QOpenGLDebugMessage::Severities)

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug debug, const QOpenGLDebugMessage &message);
Q_GUI_EXPORT QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Source source);
Q_GUI_EXPORT QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Type type);
Q_GUI_EXPORT QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Severity severity);
#endif

class QOpenGLDebugLoggerPrivate;

class Q_GUI_EXPORT QOpenGLDebugLogger : public QObject
{
    Q_OBJECT
    Q_PROPERTY(LoggingMode loggingMode READ loggingMode)

public:
    enum LoggingMode {
        AsynchronousLogging,
        SynchronousLogging
    };
    Q_ENUM(LoggingMode)

    explicit QOpenGLDebugLogger(QObject *parent = nullptr);
    ~QOpenGLDebugLogger();

    bool initialize();

    bool isLogging() const;
    LoggingMode loggingMode() const;

    qint64 maximumMessageLength() const;

    void pushGroup(const QString &name,
                   GLuint id = 0,
                   QOpenGLDebugMessage::Source source = QOpenGLDebugMessage::ApplicationSource);
    void popGroup();

    void enableMessages(QOpenGLDebugMessage::Sources sources = QOpenGLDebugMessage::AnySource,
                        QOpenGLDebugMessage::Types types = QOpenGLDebugMessage::AnyType,
                        QOpenGLDebugMessage::Severities severities = QOpenGLDebugMessage::AnySeverity);

    void enableMessages(const QVector<GLuint> &ids,
                        QOpenGLDebugMessage::Sources sources = QOpenGLDebugMessage::AnySource,
                        QOpenGLDebugMessage::Types types = QOpenGLDebugMessage::AnyType);

    void disableMessages(QOpenGLDebugMessage::Sources sources = QOpenGLDebugMessage::AnySource,
                         QOpenGLDebugMessage::Types types = QOpenGLDebugMessage::AnyType,
                         QOpenGLDebugMessage::Severities severities = QOpenGLDebugMessage::AnySeverity);

    void disableMessages(const QVector<GLuint> &ids,
                         QOpenGLDebugMessage::Sources sources = QOpenGLDebugMessage::AnySource,
                         QOpenGLDebugMessage::Types types = QOpenGLDebugMessage::AnyType);

    QList<QOpenGLDebugMessage> loggedMessages() const;

public Q_SLOTS:
    void logMessage(const QOpenGLDebugMessage &debugMessage);
    void startLogging(LoggingMode loggingMode = AsynchronousLogging);
    void stopLogging();

Q_SIGNALS:
    void messageLogged(const QOpenGLDebugMessage &debugMessage);

private:
    Q_DISABLE_COPY(QOpenGLDebugLogger)
    Q_DECLARE_PRIVATE(QOpenGLDebugLogger)
    Q_PRIVATE_SLOT(d_func(), void _q_contextAboutToBeDestroyed())
};

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QOpenGLDebugMessage)

#endif // QT_NO_OPENGL

#endif // QOPENGLDEBUG_H
