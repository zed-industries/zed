// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSCONTEXT_H
#define QDBUSCONTEXT_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qstring.h>
#include <QtDBus/qdbuserror.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class QDBusConnection;
class QDBusMessage;

class QDBusContextPrivate;
class Q_DBUS_EXPORT QDBusContext
{
public:
    QDBusContext();
    ~QDBusContext();

    bool calledFromDBus() const;
    QDBusConnection connection() const;
    const QDBusMessage &message() const;

    // convenience methods
    bool isDelayedReply() const;
    // yes, they are const, so that you can use them even from const methods
    void setDelayedReply(bool enable) const;
    void sendErrorReply(const QString &name, const QString &msg = QString()) const;
    void sendErrorReply(QDBusError::ErrorType type, const QString &msg = QString()) const;

private:
    QDBusContextPrivate *d_ptr;
    friend class QDBusContextPrivate;
};

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
