// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSABSTRACTINTERFACE_H
#define QDBUSABSTRACTINTERFACE_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qvariant.h>
#include <QtCore/qlist.h>
#include <QtCore/qobject.h>

#include <QtDBus/qdbusmessage.h>
#include <QtDBus/qdbusextratypes.h>
#include <QtDBus/qdbusconnection.h>
#include <QtDBus/qdbuspendingcall.h>

#ifdef interface
#undef interface
#endif

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class QDBusError;
class QDBusPendingCall;

class QDBusAbstractInterfacePrivate;

class Q_DBUS_EXPORT QDBusAbstractInterfaceBase: public QObject
{
public:
    int qt_metacall(QMetaObject::Call, int, void**) override;
protected:
    QDBusAbstractInterfaceBase(QDBusAbstractInterfacePrivate &dd, QObject *parent);
private:
    Q_DECLARE_PRIVATE(QDBusAbstractInterface)
};

class Q_DBUS_EXPORT QDBusAbstractInterface:
#ifdef Q_QDOC
        public QObject
#else
        public QDBusAbstractInterfaceBase
#endif
{
    Q_OBJECT

public:
    virtual ~QDBusAbstractInterface();
    bool isValid() const;

    QDBusConnection connection() const;

    QString service() const;
    QString path() const;
    QString interface() const;

    QDBusError lastError() const;

    void setTimeout(int timeout);
    int timeout() const;

    QDBusMessage call(const QString &method)
    {
        return doCall(QDBus::AutoDetect, method, nullptr, 0);
    }

    template <typename...Args>
    QDBusMessage call(const QString &method, Args &&...args)
    {
        const QVariant variants[] = { QVariant(std::forward<Args>(args))... };
        return doCall(QDBus::AutoDetect, method, variants, sizeof...(args));
    }

    QDBusMessage call(QDBus::CallMode mode, const QString &method)
    {
        return doCall(mode, method, nullptr, 0);
    }

    template <typename...Args>
    QDBusMessage call(QDBus::CallMode mode, const QString &method, Args &&...args)
    {
        const QVariant variants[] = { QVariant(std::forward<Args>(args))... };
        return doCall(mode, method, variants, sizeof...(args));
    }

    QDBusMessage callWithArgumentList(QDBus::CallMode mode,
                                      const QString &method,
                                      const QList<QVariant> &args);

    bool callWithCallback(const QString &method,
                          const QList<QVariant> &args,
                          QObject *receiver, const char *member, const char *errorSlot);
    bool callWithCallback(const QString &method,
                          const QList<QVariant> &args,
                          QObject *receiver, const char *member);

    QDBusPendingCall asyncCall(const QString &method)
    {
        return doAsyncCall(method, nullptr, 0);
    }

    template <typename...Args>
    QDBusPendingCall asyncCall(const QString &method, Args&&...args)
    {
        const QVariant variants[] = { QVariant(std::forward<Args>(args))... };
        return doAsyncCall(method, variants, sizeof...(args));
    }

    QDBusPendingCall asyncCallWithArgumentList(const QString &method,
                                               const QList<QVariant> &args);

protected:
    QDBusAbstractInterface(const QString &service, const QString &path, const char *interface,
                           const QDBusConnection &connection, QObject *parent);
    QDBusAbstractInterface(QDBusAbstractInterfacePrivate &, QObject *parent);

    void connectNotify(const QMetaMethod &signal) override;
    void disconnectNotify(const QMetaMethod &signal) override;
    QVariant internalPropGet(const char *propname) const;
    void internalPropSet(const char *propname, const QVariant &value);
    QDBusMessage internalConstCall(QDBus::CallMode mode,
                                   const QString &method,
                                   const QList<QVariant> &args = QList<QVariant>()) const;

private:
    QDBusMessage doCall(QDBus::CallMode mode, const QString &method, const QVariant *args, size_t numArgs);
    QDBusPendingCall doAsyncCall(const QString &method, const QVariant *args, size_t numArgs);

private:
    Q_DECLARE_PRIVATE(QDBusAbstractInterface)
    Q_PRIVATE_SLOT(d_func(), void _q_serviceOwnerChanged(QString,QString,QString))
};

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
