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

#ifndef QJSONARRAY_H
#define QJSONARRAY_H

#include <QtCore/qjsonvalue.h>
#include <QtCore/qiterator.h>
#include <QtCore/qshareddata.h>
#include <initializer_list>

QT_BEGIN_NAMESPACE

class QDebug;
class QStringList;
template <typename T> class QList;
typedef QList<QVariant> QVariantList;

class Q_CORE_EXPORT QJsonArray
{
public:
    QJsonArray();

    QJsonArray(std::initializer_list<QJsonValue> args);

    ~QJsonArray();

    QJsonArray(const QJsonArray &other);
    QJsonArray &operator =(const QJsonArray &other);

    QJsonArray(QJsonArray &&other) noexcept;

    QJsonArray &operator =(QJsonArray &&other) noexcept
    {
        swap(other);
        return *this;
    }

    static QJsonArray fromStringList(const QStringList &list);
    static QJsonArray fromVariantList(const QVariantList &list);
    QVariantList toVariantList() const;

    int size() const;
    inline int count() const { return size(); }

    bool isEmpty() const;
    QJsonValue at(int i) const;
    QJsonValue first() const;
    QJsonValue last() const;

    void prepend(const QJsonValue &value);
    void append(const QJsonValue &value);
    void removeAt(int i);
    QJsonValue takeAt(int i);
    inline void removeFirst() { removeAt(0); }
    inline void removeLast() { removeAt(size() - 1); }

    void insert(int i, const QJsonValue &value);
    void replace(int i, const QJsonValue &value);

    bool contains(const QJsonValue &element) const;
    QJsonValueRef operator[](int i);
    QJsonValue operator[](int i) const;

    bool operator==(const QJsonArray &other) const;
    bool operator!=(const QJsonArray &other) const;

    void swap(QJsonArray &other) noexcept
    {
        qSwap(a, other.a);
    }

    class const_iterator;

    class iterator {
    public:
        QJsonArray *a;
        int i;
        typedef std::random_access_iterator_tag  iterator_category;
        typedef int difference_type;
        typedef QJsonValue value_type;
        typedef QJsonValueRef reference;
        typedef QJsonValueRefPtr pointer;

        inline iterator() : a(nullptr), i(0) { }
        explicit inline iterator(QJsonArray *array, int index) : a(array), i(index) { }

        inline QJsonValueRef operator*() const { return QJsonValueRef(a, i); }
#ifdef Q_QDOC
        inline QJsonValueRef* operator->() const;
#else
        inline QJsonValueRefPtr operator->() const { return QJsonValueRefPtr(a, i); }
#endif
        inline QJsonValueRef operator[](int j) const { return QJsonValueRef(a, i + j); }

        inline bool operator==(const iterator &o) const { return i == o.i; }
        inline bool operator!=(const iterator &o) const { return i != o.i; }
        inline bool operator<(const iterator& other) const { return i < other.i; }
        inline bool operator<=(const iterator& other) const { return i <= other.i; }
        inline bool operator>(const iterator& other) const { return i > other.i; }
        inline bool operator>=(const iterator& other) const { return i >= other.i; }
        inline bool operator==(const const_iterator &o) const { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const { return i != o.i; }
        inline bool operator<(const const_iterator& other) const { return i < other.i; }
        inline bool operator<=(const const_iterator& other) const { return i <= other.i; }
        inline bool operator>(const const_iterator& other) const { return i > other.i; }
        inline bool operator>=(const const_iterator& other) const { return i >= other.i; }
        inline iterator &operator++() { ++i; return *this; }
        inline iterator operator++(int) { iterator n = *this; ++i; return n; }
        inline iterator &operator--() { i--; return *this; }
        inline iterator operator--(int) { iterator n = *this; i--; return n; }
        inline iterator &operator+=(int j) { i+=j; return *this; }
        inline iterator &operator-=(int j) { i-=j; return *this; }
        inline iterator operator+(int j) const { return iterator(a, i+j); }
        inline iterator operator-(int j) const { return iterator(a, i-j); }
        inline int operator-(iterator j) const { return i - j.i; }
    };
    friend class iterator;

    class const_iterator {
    public:
        const QJsonArray *a;
        int i;
        typedef std::random_access_iterator_tag  iterator_category;
        typedef qptrdiff difference_type;
        typedef QJsonValue value_type;
        typedef QJsonValue reference;
        typedef QJsonValuePtr pointer;

        inline const_iterator() : a(nullptr), i(0) { }
        explicit inline const_iterator(const QJsonArray *array, int index) : a(array), i(index) { }
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
        inline const_iterator(const const_iterator &o) : a(o.a), i(o.i) {} // ### Qt 6: Removed so class can be trivially-copyable
#endif
        inline const_iterator(const iterator &o) : a(o.a), i(o.i) {}

        inline QJsonValue operator*() const { return a->at(i); }
#ifdef Q_QDOC
        inline QJsonValue* operator->() const;
#else
        inline QJsonValuePtr operator->() const { return QJsonValuePtr(a->at(i)); }
#endif
        inline QJsonValue operator[](int j) const { return a->at(i+j); }
        inline bool operator==(const const_iterator &o) const { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const { return i != o.i; }
        inline bool operator<(const const_iterator& other) const { return i < other.i; }
        inline bool operator<=(const const_iterator& other) const { return i <= other.i; }
        inline bool operator>(const const_iterator& other) const { return i > other.i; }
        inline bool operator>=(const const_iterator& other) const { return i >= other.i; }
        inline const_iterator &operator++() { ++i; return *this; }
        inline const_iterator operator++(int) { const_iterator n = *this; ++i; return n; }
        inline const_iterator &operator--() { i--; return *this; }
        inline const_iterator operator--(int) { const_iterator n = *this; i--; return n; }
        inline const_iterator &operator+=(int j) { i+=j; return *this; }
        inline const_iterator &operator-=(int j) { i-=j; return *this; }
        inline const_iterator operator+(int j) const { return const_iterator(a, i+j); }
        inline const_iterator operator-(int j) const { return const_iterator(a, i-j); }
        inline int operator-(const_iterator j) const { return i - j.i; }
    };
    friend class const_iterator;

    // stl style
    inline iterator begin() { detach2(); return iterator(this, 0); }
    inline const_iterator begin() const { return const_iterator(this, 0); }
    inline const_iterator constBegin() const { return const_iterator(this, 0); }
    inline const_iterator cbegin() const { return const_iterator(this, 0); }
    inline iterator end() { detach2(); return iterator(this, size()); }
    inline const_iterator end() const { return const_iterator(this, size()); }
    inline const_iterator constEnd() const { return const_iterator(this, size()); }
    inline const_iterator cend() const { return const_iterator(this, size()); }
    iterator insert(iterator before, const QJsonValue &value) { insert(before.i, value); return before; }
    iterator erase(iterator it) { removeAt(it.i); return it; }

    // more Qt
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;

    // convenience
    inline QJsonArray operator+(const QJsonValue &v) const
    { QJsonArray n = *this; n += v; return n; }
    inline QJsonArray &operator+=(const QJsonValue &v)
    { append(v); return *this; }
    inline QJsonArray &operator<< (const QJsonValue &v)
    { append(v); return *this; }

    // stl compatibility
    inline void push_back(const QJsonValue &t) { append(t); }
    inline void push_front(const QJsonValue &t) { prepend(t); }
    inline void pop_front() { removeFirst(); }
    inline void pop_back() { removeLast(); }
    inline bool empty() const { return isEmpty(); }
    typedef int size_type;
    typedef QJsonValue value_type;
    typedef value_type *pointer;
    typedef const value_type *const_pointer;
    typedef QJsonValueRef reference;
    typedef QJsonValue const_reference;
    typedef int difference_type;

private:
    friend class QJsonValue;
    friend class QJsonDocument;
    friend class QCborArray;
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonArray &);

    QJsonArray(QCborContainerPrivate *array);
    void initialize();
    void compact();
    // ### Qt 6: remove me and merge with detach2
    void detach(uint reserve = 0);
    bool detach2(uint reserve = 0);

    // ### Qt 6: remove
    void *dead = nullptr;
    QExplicitlySharedDataPointer<QCborContainerPrivate> a;
};

Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QJsonArray)

Q_CORE_EXPORT uint qHash(const QJsonArray &array, uint seed = 0);

#if !defined(QT_NO_DEBUG_STREAM) && !defined(QT_JSON_READONLY)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonArray &);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QJsonArray &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QJsonArray &);
#endif

QT_END_NAMESPACE

#endif // QJSONARRAY_H
