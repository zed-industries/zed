// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSCONNECTIONINTERFACE_H
#define QDBUSCONNECTIONINTERFACE_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qstringlist.h>

#include <QtDBus/qdbusabstractinterface.h>
#include <QtDBus/qdbusreply.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class QDBusConnection;
class QString;
class QByteArray;

/*
 * Proxy class for interface org.freedesktop.DBus
 */
class Q_DBUS_EXPORT QDBusConnectionInterface: public QDBusAbstractInterface
{
    Q_OBJECT
    friend class QDBusConnectionPrivate;
    static inline const char *staticInterfaceName();

    explicit QDBusConnectionInterface(const QDBusConnection &connection, QObject *parent);
    ~QDBusConnectionInterface();

    Q_PROPERTY(QDBusReply<QStringList> registeredServiceNames READ registeredServiceNames)
    Q_PROPERTY(QDBusReply<QStringList> activatableServiceNames READ activatableServiceNames)

public:
    enum ServiceQueueOptions {
        DontQueueService,
        QueueService,
        ReplaceExistingService
    };
    Q_ENUM(ServiceQueueOptions)
    enum ServiceReplacementOptions {
        DontAllowReplacement,
        AllowReplacement
    };
    Q_ENUM(ServiceReplacementOptions)
    enum RegisterServiceReply {
        ServiceNotRegistered = 0,
        ServiceRegistered,
        ServiceQueued
    };
    Q_ENUM(RegisterServiceReply)

public Q_SLOTS:
    QDBusReply<QStringList> registeredServiceNames() const;
    QDBusReply<QStringList> activatableServiceNames() const;
    QDBusReply<bool> isServiceRegistered(const QString &serviceName) const;
    QDBusReply<QString> serviceOwner(const QString &name) const;
    QDBusReply<bool> unregisterService(const QString &serviceName);
    QDBusReply<QDBusConnectionInterface::RegisterServiceReply> registerService(const QString &serviceName,
                                                     ServiceQueueOptions qoption = DontQueueService,
                                                     ServiceReplacementOptions roption = DontAllowReplacement);

    QDBusReply<uint> servicePid(const QString &serviceName) const;
    QDBusReply<uint> serviceUid(const QString &serviceName) const;

    QDBusReply<void> startService(const QString &name);

Q_SIGNALS:
    void serviceRegistered(const QString &service);
    void serviceUnregistered(const QString &service);
    void serviceOwnerChanged(const QString &name, const QString &oldOwner, const QString &newOwner);
    void callWithCallbackFailed(const QDBusError &error, const QDBusMessage &call);

#ifndef Q_QDOC
    // internal signals
    // do not use
    void NameAcquired(const QString &);
    void NameLost(const QString &);
    void NameOwnerChanged(const QString &, const QString &, const QString &);
protected:
    void connectNotify(const QMetaMethod &) override;
    void disconnectNotify(const QMetaMethod &) override;
#endif
};

QT_END_NAMESPACE

Q_DECLARE_BUILTIN_METATYPE(UInt, QMetaType::UInt, QDBusConnectionInterface::RegisterServiceReply)

#endif // QT_NO_DBUS
#endif
