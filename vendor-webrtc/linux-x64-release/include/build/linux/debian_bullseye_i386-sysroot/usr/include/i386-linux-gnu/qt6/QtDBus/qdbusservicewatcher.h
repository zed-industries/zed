// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSSERVICEWATCHER_H
#define QDBUSSERVICEWATCHER_H

#include <QtCore/QObject>
#include <QtCore/qcontainerfwd.h> // Q(String)List
#include <QtDBus/qtdbusglobal.h>

#if !defined(QT_NO_DBUS) && !defined(QT_NO_QOBJECT)

QT_BEGIN_NAMESPACE

class QString;
template<typename T>
class QBindable;

class QDBusConnection;

class QDBusServiceWatcherPrivate;
class Q_DBUS_EXPORT QDBusServiceWatcher: public QObject
{
    Q_OBJECT
    Q_PROPERTY(QStringList watchedServices READ watchedServices WRITE setWatchedServices
               BINDABLE bindableWatchedServices)
    Q_PROPERTY(WatchMode watchMode READ watchMode WRITE setWatchMode BINDABLE bindableWatchMode)
public:
    enum WatchModeFlag {
        WatchForRegistration = 0x01,
        WatchForUnregistration = 0x02,
        WatchForOwnerChange = 0x03
    };
    Q_DECLARE_FLAGS(WatchMode, WatchModeFlag)
    Q_FLAG(WatchMode)

    explicit QDBusServiceWatcher(QObject *parent = nullptr);
    QDBusServiceWatcher(const QString &service, const QDBusConnection &connection,
                        WatchMode watchMode = WatchForOwnerChange, QObject *parent = nullptr);
    ~QDBusServiceWatcher();

    QStringList watchedServices() const;
    void setWatchedServices(const QStringList &services);
    void addWatchedService(const QString &newService);
    bool removeWatchedService(const QString &service);
    QBindable<QStringList> bindableWatchedServices();

    WatchMode watchMode() const;
    void setWatchMode(WatchMode mode);
    QBindable<WatchMode> bindableWatchMode();

    QDBusConnection connection() const;
    void setConnection(const QDBusConnection &connection);

Q_SIGNALS:
    void serviceRegistered(const QString &service);
    void serviceUnregistered(const QString &service);
    void serviceOwnerChanged(const QString &service, const QString &oldOwner, const QString &newOwner);

private:
    Q_PRIVATE_SLOT(d_func(), void _q_serviceOwnerChanged(QString,QString,QString))
    Q_DISABLE_COPY(QDBusServiceWatcher)
    Q_DECLARE_PRIVATE(QDBusServiceWatcher)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QDBusServiceWatcher::WatchMode)

QT_END_NAMESPACE

#endif // QT_NO_DBUS || QT_NO_QOBJECT
#endif // QDBUSSERVICEWATCHER_H
