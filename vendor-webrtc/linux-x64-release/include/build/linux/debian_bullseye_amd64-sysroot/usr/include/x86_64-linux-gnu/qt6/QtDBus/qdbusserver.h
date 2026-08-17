// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef QDBUSSERVER_H
#define QDBUSSERVER_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qstring.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class QDBusConnectionPrivate;
class QDBusError;
class QDBusConnection;

class Q_DBUS_EXPORT QDBusServer: public QObject
{
    Q_OBJECT
public:
    explicit QDBusServer(const QString &address, QObject *parent = nullptr);
    explicit QDBusServer(QObject *parent = nullptr);
    virtual ~QDBusServer();

    bool isConnected() const;
    QDBusError lastError() const;
    QString address() const;

    void setAnonymousAuthenticationAllowed(bool value);
    bool isAnonymousAuthenticationAllowed() const;

Q_SIGNALS:
    void newConnection(const QDBusConnection &connection);

private:
    Q_DISABLE_COPY(QDBusServer)
    Q_PRIVATE_SLOT(d, void _q_newConnection(QDBusConnectionPrivate*))
    QDBusConnectionPrivate *d;
    friend class QDBusConnectionPrivate;
};

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
