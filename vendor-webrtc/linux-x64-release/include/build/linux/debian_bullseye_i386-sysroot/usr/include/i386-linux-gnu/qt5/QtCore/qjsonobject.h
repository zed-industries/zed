/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QJSONOBJECT_H
#define QJSONOBJECT_H

#include <QtCore/qjsonvalue.h>
#include <QtCore/qiterator.h>
#include <QtCore/qpair.h>
#include <QtCore/qshareddata.h>
#include <initializer_list>

QT_BEGIN_NAMESPACE

class QDebug;
template <class Key, class T> class QMap;
typedef QMap<QString, QVariant> QVariantMap;
template <class Key, class T> class QHash;
typedef QHash<QString, QVariant> QVariantHash;

class QCborContainerPrivate;

class Q_CORE_EXPORT QJsonObject
{
public:
    QJsonObject();

    QJsonObject(std::initializer_list<QPair<QString, QJsonValue> > args);

    ~QJsonObject();

    QJsonObject(const QJsonObject &other);
    QJsonObject &operator =(const QJsonObject &other);

    QJsonObject(QJsonObject &&other) noexcept;

    QJsonObject &operator =(QJsonObject &&other) noexcept
    {
        swap(other);
        return *this;
    }

    void swap(QJsonObject &other) noexcept
    {
        qSwap(o, other.o);
    }

    static QJsonObject fromVariantMap(const QVariantMap &map);
    QVariantMap toVariantMap() const;
    static QJsonObject fromVariantHash(const QVariantHash &map);
    QVariantHash toVariantHash() const;

    QStringList keys() const;
    int size() const;
    inline int count() const { return size(); }
    inline int length() const { return size(); }
    bool isEmpty() const;

#if QT_STRINGVIEW_LEVEL < 2
    QJsonValue value(const QString &key) const;
    QJsonValue operator[] (const QString &key) const;
    QJsonValueRef operator[] (const QString &key);
#endif
    QJsonValue value(QStringView key) const;
    QJsonValue value(QLatin1String key) const;
    QJsonValue operator[] (QStringView key) const { return value(key); }
    QJsonValue operator[] (QLatin1String key) const { return value(key); }
    QJsonValueRef operator[] (QStringView key);
    QJsonValueRef operator[] (QLatin1String key);

#if QT_STRINGVIEW_LEVEL < 2
    void remove(const QString &key);
    QJsonValue take(const QString &key);
    bool contains(const QString &key) const;
#endif
    void remove(QStringView key);
    void remove(QLatin1String key);
    QJsonValue take(QStringView key);
    QJsonValue take(QLatin1String key);
    bool contains(QStringView key) const;
    bool contains(QLatin1String key) const;

    bool operator==(const QJsonObject &other) const;
    bool operator!=(const QJsonObject &other) const;

    class const_iterator;

    class iterator
    {
        friend class const_iterator;
        friend class QJsonObject;
        QJsonObject *o;
        int i;

    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef int difference_type;
        typedef QJsonValue value_type;
        typedef QJsonValueRef reference;
        typedef QJsonValuePtr pointer;

        Q_DECL_CONSTEXPR inline iterator() : o(nullptr), i(0) {}
        Q_DECL_CONSTEXPR inline iterator(QJsonObject *obj, int index) : o(obj), i(index) {}

        inline QString key() const { return o->keyAt(i); }
        inline QJsonValueRef value() const { return QJsonValueRef(o, i); }
        inline QJsonValueRef operator*() const { return QJsonValueRef(o, i); }
#ifdef Q_QDOC
        inline QJsonValueRef* operator->() const;
#else
        inline QJsonValueRefPtr operator->() const { return QJsonValueRefPtr(o, i); }
#endif
        const QJsonValueRef operator[](int j) { return QJsonValueRef(o, i + j); }

        inline bool operator==(const iterator &other) const { return i == other.i; }
        inline bool operator!=(const iterator &other) const { return i != other.i; }
        bool operator<(const iterator& other) const { return i < other.i; }
        bool operator<=(const iterator& other) const { return i <= other.i; }
        bool operator>(const iterator& other) const { return i > other.i; }
        bool operator>=(const iterator& other) const { return i >= other.i; }

        inline iterator &operator++() { ++i; return *this; }
        inline iterator operator++(int) { iterator r = *this; ++i; return r; }
        inline iterator &operator--() { --i; return *this; }
        inline iterator operator--(int) { iterator r = *this; --i; return r; }
        inline iterator operator+(int j) const
        { iterator r = *this; r.i += j; return r; }
        inline iterator operator-(int j) const { return operator+(-j); }
        inline iterator &operator+=(int j) { i += j; return *this; }
        inline iterator &operator-=(int j) { i -= j; return *this; }
        int operator-(iterator j) const { return i - j.i; }

    public:
        inline bool operator==(const const_iterator &other) const { return i == other.i; }
        inline bool operator!=(const const_iterator &other) const { return i != other.i; }
        bool operator<(const const_iterator& other) const { return i < other.i; }
        bool operator<=(const const_iterator& other) const { return i <= other.i; }
        bool operator>(const const_iterator& other) const { return i > other.i; }
        bool operator>=(const const_iterator& other) const { return i >= other.i; }
    };
    friend class iterator;

    class const_iterator
    {
        friend class iterator;
        const QJsonObject *o;
        int i;

    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef int difference_type;
        typedef QJsonValue value_type;
        typedef QJsonValue reference;
        typedef QJsonValuePtr pointer;

        Q_DECL_CONSTEXPR inline const_iterator() : o(nullptr), i(0) {}
        Q_DECL_CONSTEXPR inline const_iterator(const QJsonObject *obj, int index)
            : o(obj), i(index) {}
        inline const_iterator(const iterator &other)
            : o(other.o), i(other.i) {}

        inline QString key() const { return o->keyAt(i); }
        inline QJsonValue value() const { return o->valueAt(i); }
        inline QJsonValue operator*() const { return o->valueAt(i); }
#ifdef Q_QDOC
        inline QJsonValue* operator->() const;
#else
        inline QJsonValuePtr operator->() const { return QJsonValuePtr(o->valueAt(i)); }
#endif
        const QJsonValue operator[](int j) { return o->valueAt(i + j); }

        inline bool operator==(const const_iterator &other) const { return i == other.i; }
        inline bool operator!=(const const_iterator &other) const { return i != other.i; }
        bool operator<(const const_iterator& other) const { return i < other.i; }
        bool operator<=(const const_iterator& other) const { return i <= other.i; }
        bool operator>(const const_iterator& other) const { return i > other.i; }
        bool operator>=(const const_iterator& other) const { return i >= other.i; }

        inline const_iterator &operator++() { ++i; return *this; }
        inline const_iterator operator++(int) { const_iterator r = *this; ++i; return r; }
        inline const_iterator &operator--() { --i; return *this; }
        inline const_iterator operator--(int) { const_iterator r = *this; --i; return r; }
        inline const_iterator operator+(int j) const
        { const_iterator r = *this; r.i += j; return r; }
        inline const_iterator operator-(int j) const { return operator+(-j); }
        inline const_iterator &operator+=(int j) { i += j; return *this; }
        inline const_iterator &operator-=(int j) { i -= j; return *this; }
        int operator-(const_iterator j) const { return i - j.i; }

        inline bool operator==(const iterator &other) const { return i == other.i; }
        inline bool operator!=(const iterator &other) const { return i != other.i; }
        bool operator<(const iterator& other) const { return i < other.i; }
        bool operator<=(const iterator& other) const { return i <= other.i; }
        bool operator>(const iterator& other) const { return i > other.i; }
        bool operator>=(const iterator& other) const { return i >= other.i; }
    };
    friend class const_iterator;

    // STL style
    inline iterator begin() { detach2(); return iterator(this, 0); }
    inline const_iterator begin() const { return const_iterator(this, 0); }
    inline const_iterator constBegin() const { return const_iterator(this, 0); }
    inline iterator end() { detach2(); return iterator(this, size()); }
    inline const_iterator end() const { return const_iterator(this, size()); }
    inline const_iterator constEnd() const { return const_iterator(this, size()); }
    iterator erase(iterator it);

    // more Qt
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
#if QT_STRINGVIEW_LEVEL < 2
    iterator find(const QString &key);
    const_iterator find(const QString &key) const { return constFind(key); }
    const_iterator constFind(const QString &key) const;
    iterator insert(const QString &key, const QJsonValue &value);
#endif
    iterator find(QStringView key);
    iterator find(QLatin1String key);
    const_iterator find(QStringView key) const { return constFind(key); }
    const_iterator find(QLatin1String key) const { return constFind(key); }
    const_iterator constFind(QStringView key) const;
    const_iterator constFind(QLatin1String key) const;
    iterator insert(QStringView key, const QJsonValue &value);
    iterator insert(QLatin1String key, const QJsonValue &value);

    // STL compatibility
    typedef QJsonValue mapped_type;
    typedef QString key_type;
    typedef int size_type;

    inline bool empty() const { return isEmpty(); }

private:
    friend class QJsonValue;
    friend class QJsonDocument;
    friend class QJsonValueRef;
    friend class QCborMap;
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonObject &);

    QJsonObject(QCborContainerPrivate *object);
    void initialize();
    // ### Qt 6: remove me and merge with detach2
    void detach(uint reserve = 0);
    bool detach2(uint reserve = 0);
    void compact();

    template <typename T> QJsonValue valueImpl(T key) const;
    template <typename T> QJsonValueRef atImpl(T key);
    template <typename T> void removeImpl(T key);
    template <typename T> QJsonValue takeImpl(T key);
    template <typename T> bool containsImpl(T key) const;
    template <typename T> iterator findImpl(T key);
    template <typename T> const_iterator constFindImpl(T key) const;
    template <typename T> iterator insertImpl(T key, const QJsonValue &value);

    QString keyAt(int i) const;
    QJsonValue valueAt(int i) const;
    void setValueAt(int i, const QJsonValue &val);
    void removeAt(int i);
    template <typename T> iterator insertAt(int i, T key, const QJsonValue &val, bool exists);

    // ### Qt 6: remove
    void *dead = nullptr;
    QExplicitlySharedDataPointer<QCborContainerPrivate> o;
};

Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QJsonObject)

Q_CORE_EXPORT uint qHash(const QJsonObject &object, uint seed = 0);

#if !defined(QT_NO_DEBUG_STREAM) && !defined(QT_JSON_READONLY)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonObject &);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QJsonObject &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QJsonObject &);
#endif

QT_END_NAMESPACE

#endif // QJSONOBJECT_H
