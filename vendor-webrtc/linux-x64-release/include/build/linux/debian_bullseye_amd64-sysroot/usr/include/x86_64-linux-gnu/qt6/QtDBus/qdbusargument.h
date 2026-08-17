// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSARGUMENT_H
#define QDBUSARGUMENT_H

#include <QtDBus/qtdbusglobal.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qhash.h>
#include <QtCore/qglobal.h>
#include <QtCore/qlist.h>
#include <QtCore/qmap.h>
#include <QtCore/qstring.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qvariant.h>
#include <QtDBus/qdbusextratypes.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class QDBusUnixFileDescriptor;

class QDBusArgumentPrivate;
class QDBusDemarshaller;
class QDBusMarshaller;
class Q_DBUS_EXPORT QDBusArgument
{
public:
    enum ElementType {
        BasicType,
        VariantType,
        ArrayType,
        StructureType,
        MapType,
        MapEntryType,
        UnknownType = -1
    };

    QDBusArgument();
    QDBusArgument(const QDBusArgument &other);
    QDBusArgument(QDBusArgument &&other) noexcept : d(other.d) { other.d = nullptr; }
    QDBusArgument &operator=(QDBusArgument &&other) noexcept { swap(other); return *this; }
    QDBusArgument &operator=(const QDBusArgument &other);
    ~QDBusArgument();

    void swap(QDBusArgument &other) noexcept { qt_ptr_swap(d, other.d); }

    // used for marshalling (Qt -> D-BUS)
    QDBusArgument &operator<<(uchar arg);
    QDBusArgument &operator<<(bool arg);
    QDBusArgument &operator<<(short arg);
    QDBusArgument &operator<<(ushort arg);
    QDBusArgument &operator<<(int arg);
    QDBusArgument &operator<<(uint arg);
    QDBusArgument &operator<<(qlonglong arg);
    QDBusArgument &operator<<(qulonglong arg);
    QDBusArgument &operator<<(double arg);
    QDBusArgument &operator<<(const QString &arg);
    QDBusArgument &operator<<(const QDBusVariant &arg);
    QDBusArgument &operator<<(const QDBusObjectPath &arg);
    QDBusArgument &operator<<(const QDBusSignature &arg);
    QDBusArgument &operator<<(const QDBusUnixFileDescriptor &arg);
    QDBusArgument &operator<<(const QStringList &arg);
    QDBusArgument &operator<<(const QByteArray &arg);

    void beginStructure();
    void endStructure();
    void beginArray(int elementMetaTypeId)
    { beginArray(QMetaType(elementMetaTypeId)); }
    void beginArray(QMetaType elementMetaType);
    void endArray();
    void beginMap(int keyMetaTypeId, int valueMetaTypeId)
    { beginMap(QMetaType(keyMetaTypeId), QMetaType(valueMetaTypeId)); }
    void beginMap(QMetaType keyMetaType, QMetaType valueMetaType);
    void endMap();
    void beginMapEntry();
    void endMapEntry();

    void appendVariant(const QVariant &v);

    // used for de-marshalling (D-BUS -> Qt)
    QString currentSignature() const;
    ElementType currentType() const;

    const QDBusArgument &operator>>(uchar &arg) const;
    const QDBusArgument &operator>>(bool &arg) const;
    const QDBusArgument &operator>>(short &arg) const;
    const QDBusArgument &operator>>(ushort &arg) const;
    const QDBusArgument &operator>>(int &arg) const;
    const QDBusArgument &operator>>(uint &arg) const;
    const QDBusArgument &operator>>(qlonglong &arg) const;
    const QDBusArgument &operator>>(qulonglong &arg) const;
    const QDBusArgument &operator>>(double &arg) const;
    const QDBusArgument &operator>>(QString &arg) const;
    const QDBusArgument &operator>>(QDBusVariant &arg) const;
    const QDBusArgument &operator>>(QDBusObjectPath &arg) const;
    const QDBusArgument &operator>>(QDBusSignature &arg) const;
    const QDBusArgument &operator>>(QDBusUnixFileDescriptor &arg) const;
    const QDBusArgument &operator>>(QStringList &arg) const;
    const QDBusArgument &operator>>(QByteArray &arg) const;

    void beginStructure() const;
    void endStructure() const;
    void beginArray() const;
    void endArray() const;
    void beginMap() const;
    void endMap() const;
    void beginMapEntry() const;
    void endMapEntry() const;
    bool atEnd() const;

    QVariant asVariant() const;

protected:
    QDBusArgument(QDBusArgumentPrivate *d);
    friend class QDBusArgumentPrivate;
    mutable QDBusArgumentPrivate *d;
};
Q_DECLARE_SHARED(QDBusArgument)

QT_END_NAMESPACE
QT_DECL_METATYPE_EXTERN(QDBusArgument, Q_DBUS_EXPORT)
QT_BEGIN_NAMESPACE

template<typename T> inline T qdbus_cast(const QDBusArgument &arg)
{
    T item;
    arg >> item;
    return item;
}

template<typename T> inline T qdbus_cast(const QVariant &v)
{
    if (v.metaType() == QMetaType::fromType<QDBusArgument>())
        return qdbus_cast<T>(qvariant_cast<QDBusArgument>(v));
    else
        return qvariant_cast<T>(v);
}

// specialize for QVariant, allowing it to be used in place of QDBusVariant
template<> inline QVariant qdbus_cast<QVariant>(const QDBusArgument &arg)
{
    QDBusVariant item;
    arg >> item;
    return item.variant();
}
template<> inline QVariant qdbus_cast<QVariant>(const QVariant &v)
{
    return qdbus_cast<QDBusVariant>(v).variant();
}

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QVariant &v);

// QVariant types
#ifndef QDBUS_NO_SPECIALTYPES

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QDate &date);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QDate &date);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QTime &time);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QTime &time);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QDateTime &dt);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QDateTime &dt);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QRect &rect);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QRect &rect);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QRectF &rect);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QRectF &rect);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QSize &size);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QSize &size);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QSizeF &size);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QSizeF &size);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QPoint &pt);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QPoint &pt);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QPointF &pt);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QPointF &pt);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QLine &line);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QLine &line);

Q_DBUS_EXPORT const QDBusArgument &operator>>(const QDBusArgument &a, QLineF &line);
Q_DBUS_EXPORT QDBusArgument &operator<<(QDBusArgument &a, const QLineF &line);
#endif

template<template <typename> class Container, typename T,
         typename = typename Container<T>::iterator>
inline QDBusArgument &operator<<(QDBusArgument &arg, const Container<T> &list)
{
    arg.beginArray(QMetaType::fromType<T>());
    typename Container<T>::const_iterator it = list.begin();
    typename Container<T>::const_iterator end = list.end();
    for ( ; it != end; ++it)
        arg << *it;
    arg.endArray();
    return arg;
}

template<template <typename> class Container, typename T,
         typename = typename Container<T>::iterator>
inline const QDBusArgument &operator>>(const QDBusArgument &arg, Container<T> &list)
{
    arg.beginArray();
    list.clear();
    while (!arg.atEnd()) {
        T item;
        arg >> item;
        list.push_back(item);
    }

    arg.endArray();
    return arg;
}

inline QDBusArgument &operator<<(QDBusArgument &arg, const QVariantList &list)
{
    arg.beginArray(QMetaType::fromType<QDBusVariant>());
    QVariantList::ConstIterator it = list.constBegin();
    QVariantList::ConstIterator end = list.constEnd();
    for ( ; it != end; ++it)
        arg << QDBusVariant(*it);
    arg.endArray();
    return arg;
}

// Specializations for associative containers
template <template <typename, typename> class Container, typename Key, typename T,
         QtPrivate::IfAssociativeIteratorHasKeyAndValue<typename Container<Key, T>::iterator> = true>
inline QDBusArgument &operator<<(QDBusArgument &arg, const Container<Key, T> &map)
{
    arg.beginMap(QMetaType::fromType<Key>(), QMetaType::fromType<T>());
    auto it = map.begin();
    auto end = map.end();
    for ( ; it != end; ++it) {
        arg.beginMapEntry();
        arg << it.key() << it.value();
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

template <template <typename, typename> class Container, typename Key, typename T,
         QtPrivate::IfAssociativeIteratorHasFirstAndSecond<typename Container<Key, T>::iterator> = true>
inline QDBusArgument &operator<<(QDBusArgument &arg, const Container<Key, T> &map)
{
    arg.beginMap(QMetaType::fromType<Key>(), QMetaType::fromType<T>());
    auto it = map.begin();
    auto end = map.end();
    for ( ; it != end; ++it) {
        arg.beginMapEntry();
        arg << it->first << it->second;
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

template <template <typename, typename> class Container, typename Key, typename T,
         QtPrivate::IfAssociativeIteratorHasKeyAndValue<typename Container<Key, T>::iterator> = true>
inline const QDBusArgument &operator>>(const QDBusArgument &arg, Container<Key, T> &map)
{
    arg.beginMap();
    map.clear();
    while (!arg.atEnd()) {
        Key key;
        T value;
        arg.beginMapEntry();
        arg >> key >> value;
        map.insert(key, value);
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

inline QDBusArgument &operator<<(QDBusArgument &arg, const QVariantMap &map)
{
    arg.beginMap(QMetaType::fromType<QString>(), QMetaType::fromType<QDBusVariant>());
    QVariantMap::ConstIterator it = map.constBegin();
    QVariantMap::ConstIterator end = map.constEnd();
    for ( ; it != end; ++it) {
        arg.beginMapEntry();
        arg << it.key() << QDBusVariant(it.value());
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

inline QDBusArgument &operator<<(QDBusArgument &arg, const QVariantHash &map)
{
    arg.beginMap(QMetaType::fromType<QString>(), QMetaType::fromType<QDBusVariant>());
    QVariantHash::ConstIterator it = map.constBegin();
    QVariantHash::ConstIterator end = map.constEnd();
    for ( ; it != end; ++it) {
        arg.beginMapEntry();
        arg << it.key() << QDBusVariant(it.value());
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

template <typename T1, typename T2>
inline QDBusArgument &operator<<(QDBusArgument &arg, const QPair<T1, T2> &pair)
{
    arg.beginStructure();
    arg << pair.first << pair.second;
    arg.endStructure();
    return arg;
}

template <typename T1, typename T2>
inline const QDBusArgument &operator>>(const QDBusArgument &arg, QPair<T1, T2> &pair)
{
    arg.beginStructure();
    arg >> pair.first >> pair.second;
    arg.endStructure();
    return arg;
}

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
