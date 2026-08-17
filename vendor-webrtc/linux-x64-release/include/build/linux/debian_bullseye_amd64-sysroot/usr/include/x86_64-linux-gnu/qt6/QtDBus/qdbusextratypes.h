// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSEXTRATYPES_H
#define QDBUSEXTRATYPES_H

// define some useful types for D-BUS

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qvariant.h>
#include <QtCore/qstring.h>
#include <QtCore/qhashfunctions.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE

class Q_DBUS_EXPORT QDBusObjectPath
{
    QString m_path;
public:
    QDBusObjectPath() noexcept : m_path() {}
    // compiler-generated copy/move constructor/assignment operators are ok!
    // compiler-generated destructor is ok!

    inline explicit QDBusObjectPath(const char *path);
    inline explicit QDBusObjectPath(QLatin1StringView path);
    inline explicit QDBusObjectPath(const QString &path);
    explicit QDBusObjectPath(QString &&p) : m_path(std::move(p)) { doCheck(); }

    void swap(QDBusObjectPath &other) noexcept { m_path.swap(other.m_path); }

    inline void setPath(const QString &path);

    inline QString path() const
    { return m_path; }

    operator QVariant() const;

private:
    void doCheck();
};
Q_DECLARE_SHARED(QDBusObjectPath)

inline QDBusObjectPath::QDBusObjectPath(const char *objectPath)
    : m_path(QString::fromLatin1(objectPath))
{ doCheck(); }

inline QDBusObjectPath::QDBusObjectPath(QLatin1StringView objectPath)
    : m_path(objectPath)
{ doCheck(); }

inline QDBusObjectPath::QDBusObjectPath(const QString &objectPath)
    : m_path(objectPath)
{ doCheck(); }

inline void QDBusObjectPath::setPath(const QString &objectPath)
{ m_path = objectPath; doCheck(); }

inline bool operator==(const QDBusObjectPath &lhs, const QDBusObjectPath &rhs)
{ return lhs.path() == rhs.path(); }

inline bool operator!=(const QDBusObjectPath &lhs, const QDBusObjectPath &rhs)
{ return lhs.path() != rhs.path(); }

inline bool operator<(const QDBusObjectPath &lhs, const QDBusObjectPath &rhs)
{ return lhs.path() < rhs.path(); }

inline size_t qHash(const QDBusObjectPath &objectPath, size_t seed = 0)
{ return qHash(objectPath.path(), seed); }


class Q_DBUS_EXPORT QDBusSignature
{
    QString m_signature;
public:
    QDBusSignature() noexcept : m_signature() {}
    // compiler-generated copy/move constructor/assignment operators are ok!
    // compiler-generated destructor is ok!

    inline explicit QDBusSignature(const char *signature);
    inline explicit QDBusSignature(QLatin1StringView signature);
    inline explicit QDBusSignature(const QString &signature);
    explicit QDBusSignature(QString &&sig) : m_signature(std::move(sig)) { doCheck(); }

    void swap(QDBusSignature &other) noexcept { m_signature.swap(other.m_signature); }

    inline void setSignature(const QString &signature);

    inline QString signature() const
    { return m_signature; }

private:
    void doCheck();
};
Q_DECLARE_SHARED(QDBusSignature)

inline QDBusSignature::QDBusSignature(const char *dBusSignature)
    : m_signature(QString::fromLatin1(dBusSignature))
{ doCheck(); }

inline QDBusSignature::QDBusSignature(QLatin1StringView dBusSignature)
    : m_signature(dBusSignature)
{ doCheck(); }

inline QDBusSignature::QDBusSignature(const QString &dBusSignature)
    : m_signature(dBusSignature)
{ doCheck(); }

inline void QDBusSignature::setSignature(const QString &dBusSignature)
{ m_signature = dBusSignature; doCheck(); }

inline bool operator==(const QDBusSignature &lhs, const QDBusSignature &rhs)
{ return lhs.signature() == rhs.signature(); }

inline bool operator!=(const QDBusSignature &lhs, const QDBusSignature &rhs)
{ return lhs.signature() != rhs.signature(); }

inline bool operator<(const QDBusSignature &lhs, const QDBusSignature &rhs)
{ return lhs.signature() < rhs.signature(); }

inline size_t qHash(const QDBusSignature &signature, size_t seed = 0)
{ return qHash(signature.signature(), seed); }

class QDBusVariant
{
    QVariant m_variant;
public:
    QDBusVariant() noexcept : m_variant() {}
    // compiler-generated copy/move constructor/assignment operators are ok!
    // compiler-generated destructor is ok!

    inline explicit QDBusVariant(const QVariant &variant);
    explicit QDBusVariant(QVariant &&v) noexcept : m_variant(std::move(v)) {}

    void swap(QDBusVariant &other) noexcept { m_variant.swap(other.m_variant); }

    inline void setVariant(const QVariant &variant);

    inline QVariant variant() const
    { return m_variant; }
};
Q_DECLARE_SHARED(QDBusVariant)

inline  QDBusVariant::QDBusVariant(const QVariant &dBusVariant)
    : m_variant(dBusVariant) { }

inline void QDBusVariant::setVariant(const QVariant &dBusVariant)
{ m_variant = dBusVariant; }

inline bool operator==(const QDBusVariant &v1, const QDBusVariant &v2)
{ return v1.variant() == v2.variant(); }

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QDBusVariant, Q_DBUS_EXPORT)
QT_DECL_METATYPE_EXTERN(QDBusObjectPath, Q_DBUS_EXPORT)
QT_DECL_METATYPE_EXTERN(QDBusSignature, Q_DBUS_EXPORT)

#endif // QT_NO_DBUS
#endif
