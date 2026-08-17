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

    void swap(QDBusArgument &other) noexcept { qSwap(d, other.d); }

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
    void beginArray(int elementMetaTypeId);
    void endArray();
    void beginMap(int keyMetaTypeId, int valueMetaTypeId);
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
Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QDBusArgument)

QT_END_NAMESPACE
Q_DECLARE_METATYPE(QDBusArgument)
QT_BEGIN_NAMESPACE

// ### Qt6: remove the defaulted T * = nullptr from these two (MSVC6 work-around):
template<typename T> inline T qdbus_cast(const QDBusArgument &arg, T * = nullptr)
{
    T item;
    arg >> item;
    return item;
}

template<typename T> inline T qdbus_cast(const QVariant &v, T * = nullptr)
{
    int id = v.userType();
    if (id == qMetaTypeId<QDBusArgument>())
        return qdbus_cast<T>(qvariant_cast<QDBusArgument>(v));
    else
        return qvariant_cast<T>(v);
}

// specialize for QVariant, allowing it to be used in place of QDBusVariant
template<> inline QVariant qdbus_cast<QVariant>(const QDBusArgument &arg, QVariant *)
{
    QDBusVariant item;
    arg >> item;
    return item.variant();
}
template<> inline QVariant qdbus_cast<QVariant>(const QVariant &v, QVariant *)
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

template<template <typename> class Container, typename T>
inline QDBusArgument &operator<<(QDBusArgument &arg, const Container<T> &list)
{
    int id = qMetaTypeId<T>();
    arg.beginArray(id);
    typename Container<T>::const_iterator it = list.begin();
    typename Container<T>::const_iterator end = list.end();
    for ( ; it != end; ++it)
        arg << *it;
    arg.endArray();
    return arg;
}

template<template <typename> class Container, typename T>
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

// QList specializations
template<typename T>
inline QDBusArgument &operator<<(QDBusArgument &arg, const QList<T> &list)
{
    int id = qMetaTypeId<T>();
    arg.beginArray(id);
    typename QList<T>::ConstIterator it = list.constBegin();
    typename QList<T>::ConstIterator end = list.constEnd();
    for ( ; it != end; ++it)
        arg << *it;
    arg.endArray();
    return arg;
}

template<typename T>
inline const QDBusArgument &operator>>(const QDBusArgument &arg, QList<T> &list)
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
    int id = qMetaTypeId<QDBusVariant>();
    arg.beginArray(id);
    QVariantList::ConstIterator it = list.constBegin();
    QVariantList::ConstIterator end = list.constEnd();
    for ( ; it != end; ++it)
        arg << QDBusVariant(*it);
    arg.endArray();
    return arg;
}

// QMap specializations
template<typename Key, typename T>
inline QDBusArgument &operator<<(QDBusArgument &arg, const QMap<Key, T> &map)
{
    int kid = qMetaTypeId<Key>();
    int vid = qMetaTypeId<T>();
    arg.beginMap(kid, vid);
    typename QMap<Key, T>::ConstIterator it = map.constBegin();
    typename QMap<Key, T>::ConstIterator end = map.constEnd();
    for ( ; it != end; ++it) {
        arg.beginMapEntry();
        arg << it.key() << it.value();
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

template<typename Key, typename T>
inline const QDBusArgument &operator>>(const QDBusArgument &arg, QMap<Key, T> &map)
{
    arg.beginMap();
    map.clear();
    while (!arg.atEnd()) {
        Key key;
        T value;
        arg.beginMapEntry();
        arg >> key >> value;
        static_cast<QMultiMap<Key, T> &>(map).insert(key, value);
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

inline QDBusArgument &operator<<(QDBusArgument &arg, const QVariantMap &map)
{
    arg.beginMap(QMetaType::QString, qMetaTypeId<QDBusVariant>());
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

// QHash specializations
template<typename Key, typename T>
inline QDBusArgument &operator<<(QDBusArgument &arg, const QHash<Key, T> &map)
{
    int kid = qMetaTypeId<Key>();
    int vid = qMetaTypeId<T>();
    arg.beginMap(kid, vid);
    typename QHash<Key, T>::ConstIterator it = map.constBegin();
    typename QHash<Key, T>::ConstIterator end = map.constEnd();
    for ( ; it != end; ++it) {
        arg.beginMapEntry();
        arg << it.key() << it.value();
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

template<typename Key, typename T>
inline const QDBusArgument &operator>>(const QDBusArgument &arg, QHash<Key, T> &map)
{
    arg.beginMap();
    map.clear();
    while (!arg.atEnd()) {
        Key key;
        T value;
        arg.beginMapEntry();
        arg >> key >> value;
        static_cast<QMultiHash<Key, T> &>(map).insert(key, value);
        arg.endMapEntry();
    }
    arg.endMap();
    return arg;
}

inline QDBusArgument &operator<<(QDBusArgument &arg, const QVariantHash &map)
{
    arg.beginMap(QMetaType::QString, qMetaTypeId<QDBusVariant>());
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
