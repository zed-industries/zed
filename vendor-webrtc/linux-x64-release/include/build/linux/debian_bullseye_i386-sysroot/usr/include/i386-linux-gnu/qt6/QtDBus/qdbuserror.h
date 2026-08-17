// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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
    Q_IMPLICIT QDBusError(const QDBusMessage& msg);
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
        std::swap(code, other.code);
        msg.swap(other.msg);
        nm.swap(other.nm);
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
Q_DECLARE_SHARED(QDBusError)

#ifndef QT_NO_DEBUG_STREAM
Q_DBUS_EXPORT QDebug operator<<(QDebug, const QDBusError &);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QDBusError, Q_DBUS_EXPORT)
#else
QT_BEGIN_NAMESPACE
class Q_DBUS_EXPORT QDBusError {}; // dummy class for moc
QT_END_NAMESPACE
#endif // QT_NO_DBUS
#endif
