/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtDBus module of the Qt Toolkit.
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

#ifndef QDBUSERROR_H
#define QDBUSERROR_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qobjectdefs.h>
#include <QtCore/qstring.h>

#ifndef QT_NO_DBUS

struct DBusError;

QT_BEGIN_NAMESPACE


class QDBusMessage;

class Q_DBUS_EXPORT QDBusError
{
    Q_GADGET
public:
    enum ErrorType {
        NoError = 0,
        Other = 1,
        Failed,
        NoMemory,
        ServiceUnknown,
        NoReply,
        BadAddress,
        NotSupported,
        LimitsExceeded,
        AccessDenied,
        NoServer,
        Timeout,
        NoNetwork,
        AddressInUse,
        Disconnected,
        InvalidArgs,
        UnknownMethod,
        TimedOut,
        InvalidSignature,
        UnknownInterface,
        UnknownObject,
        UnknownProperty,
        PropertyReadOnly,
        InternalError,
        InvalidService,
        InvalidObjectPath,
        InvalidInterface,
        InvalidMember,

#ifndef Q_QDOC
        // don't use this one!
        LastErrorType = InvalidMember
#endif
    };
    Q_ENUM(ErrorType)

    QDBusError();
#ifndef QT_BOOTSTRAPPED
    explicit QDBusError(const DBusError *error);
    /*implicit*/ QDBusError(const QDBusMessage& msg);
#endif
    QDBusError(ErrorType error, const QString &message);
    QDBusError(const QDBusError &other);
    QDBusError(QDBusError &&other) noexcept
        : code(other.code), msg(std::move(other.msg)), nm(std::move(other.nm))
    {}
    QDBusError &operator=(QDBusError &&other) noexcept { swap(other); return *this; }
    QDBusError &operator=(const QDBusError &other);
#ifndef QT_BOOTSTRAPPED
    QDBusError &operator=(const QDBusMessage &msg);
#endif

    void swap(QDBusError &other) noexcept
    {
        qSwap(code,   other.code);
        qSwap(msg,    other.msg);
        qSwap(nm,     other.nm);
    }

    ErrorType type() const;
    QString name() const;
    QString message() const;
    bool isValid() const;

    static QString errorString(ErrorType error);

private:
    ErrorType code;
    QString msg;
    QString nm;
    // ### This class has an implicit (therefore inline) destructor
    // so the following field cannot be used:
    void *unused;
};
Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QDBusError)

#ifndef QT_NO_DEBUG_STREAM
Q_DBUS_EXPORT QDebug operator<<(QDebug, const QDBusError &);
#endif

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QDBusError)

#endif // QT_NO_DBUS
#endif
