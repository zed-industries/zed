// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSREPLY_H
#define QDBUSREPLY_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qvariant.h>

#include <QtDBus/qdbusmessage.h>
#include <QtDBus/qdbuserror.h>
#include <QtDBus/qdbusextratypes.h>
#include <QtDBus/qdbuspendingreply.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


Q_DBUS_EXPORT void qDBusReplyFill(const QDBusMessage &reply, QDBusError &error, QVariant &data);

template<typename T>
class QDBusReply
{
    typedef T Type;
public:
    inline QDBusReply(const QDBusMessage &reply)
    {
        *this = reply;
    }
    inline QDBusReply(const QDBusReply &) = default;
    inline QDBusReply& operator=(const QDBusMessage &reply)
    {
        QVariant data(QMetaType::fromType<Type>());
        qDBusReplyFill(reply, m_error, data);
        m_data = qvariant_cast<Type>(data);
        return *this;
    }

    inline QDBusReply(const QDBusPendingCall &pcall)
    {
        *this = pcall;
    }
    inline QDBusReply &operator=(const QDBusPendingCall &pcall)
    {
        QDBusPendingCall other(pcall);
        other.waitForFinished();
        return *this = other.reply();
    }
    inline QDBusReply(const QDBusPendingReply<T> &reply)
    {
        *this = static_cast<QDBusPendingCall>(reply);
    }

    inline QDBusReply(const QDBusError &dbusError = QDBusError())
        : m_error(dbusError), m_data(Type())
    {
    }
    inline QDBusReply& operator=(const QDBusError& dbusError)
    {
        m_error = dbusError;
        m_data = Type();
        return *this;
    }

    inline QDBusReply& operator=(const QDBusReply& other)
    {
        m_error = other.m_error;
        m_data = other.m_data;
        return *this;
    }

    inline bool isValid() const { return !m_error.isValid(); }

    inline const QDBusError& error() { return m_error; }
    inline const QDBusError& error() const { return m_error; }

    inline Type value() const
    {
        return m_data;
    }

    inline operator Type () const
    {
        return m_data;
    }

private:
    QDBusError m_error;
    Type m_data;
};

# ifndef Q_QDOC
// specialize for QVariant:
template<> inline QDBusReply<QVariant>&
QDBusReply<QVariant>::operator=(const QDBusMessage &reply)
{
    QVariant data(QMetaType::fromType<QDBusVariant>());
    qDBusReplyFill(reply, m_error, data);
    m_data = qvariant_cast<QDBusVariant>(data).variant();
    return *this;
}

// specialize for void:
template<>
class QDBusReply<void>
{
public:
    inline QDBusReply(const QDBusMessage &reply)
        : m_error(reply)
    {
    }
    inline QDBusReply& operator=(const QDBusMessage &reply)
    {
        m_error = QDBusError(reply);
        return *this;
    }
    inline QDBusReply(const QDBusError &dbusError = QDBusError())
        : m_error(dbusError)
    {
    }
    inline QDBusReply(const QDBusPendingCall &pcall)
    {
        *this = pcall;
    }
    inline QDBusReply &operator=(const QDBusPendingCall &pcall)
    {
        QDBusPendingCall other(pcall);
        other.waitForFinished();
        return *this = other.reply();
    }
    inline QDBusReply& operator=(const QDBusError& dbusError)
    {
        m_error = dbusError;
        return *this;
    }

    inline QDBusReply(const QDBusReply &) = default;

    inline QDBusReply& operator=(const QDBusReply& other)
    {
        m_error = other.m_error;
        return *this;
    }

    inline bool isValid() const { return !m_error.isValid(); }

    inline const QDBusError& error() { return m_error; }
    inline const QDBusError& error() const { return m_error; }

private:
    QDBusError m_error;
};
# endif

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
