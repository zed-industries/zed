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

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QDBusMessage call(const QString &method,
                      const QVariant &arg1,
                      const QVariant &arg2,
                      const QVariant &arg3,
                      const QVariant &arg4,
                      const QVariant &arg5,
                      const QVariant &arg6,
                      const QVariant &arg7,
                      const QVariant &arg8);

    QDBusMessage call(QDBus::CallMode mode,
                      const QString &method,
                      const QVariant &arg1,
                      const QVariant &arg2,
                      const QVariant &arg3,
                      const QVariant &arg4,
                      const QVariant &arg5,
                      const QVariant &arg6,
                      const QVariant &arg7,
                      const QVariant &arg8);
#endif // Qt 5

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

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QDBusPendingCall asyncCall(const QString &method,
                               const QVariant &arg1,
                               const QVariant &arg2,
                               const QVariant &arg3,
                               const QVariant &arg4,
                               const QVariant &arg5,
                               const QVariant &arg6,
                               const QVariant &arg7,
                               const QVariant &arg8);
#endif // Qt 5

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
