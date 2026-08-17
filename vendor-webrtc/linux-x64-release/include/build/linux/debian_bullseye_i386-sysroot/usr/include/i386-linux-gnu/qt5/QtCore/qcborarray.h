/****************************************************************************
**
** Copyright (C) 2018 Intel Corporation.
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

#ifndef QCBORARRAY_H
#define QCBORARRAY_H

#include <QtCore/qcborvalue.h>

#include <initializer_list>

QT_BEGIN_NAMESPACE

class QJsonArray;
class QDataStream;

namespace QJsonPrivate { class Variant; }

class QCborContainerPrivate;
class Q_CORE_EXPORT QCborArray
{
public:
    class ConstIterator;
    class Iterator {
        mutable QCborValueRef item;
        friend class ConstIterator;
        friend class QCborArray;
        Iterator(QCborContainerPrivate *dd, qsizetype ii) : item(dd, ii) {}
    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef qsizetype difference_type;
        typedef QCborValue value_type;
        typedef QCborValueRef reference;
        typedef QCborValueRef *pointer;

        Q_DECL_CONSTEXPR Iterator() = default;
        Q_DECL_CONSTEXPR Iterator(const Iterator &) = default;
        Iterator &operator=(const Iterator &other)
        {
            // rebind the reference
            item.d = other.item.d;
            item.i = other.item.i;
            return *this;
        }

        QCborValueRef operator*() const { return item; }
        QCborValueRef *operator->() const { return &item; }
        QCborValueRef operator[](qsizetype j) { return { item.d, item.i + j }; }

        bool operator==(const Iterator &o) const { return item.d == o.item.d && item.i == o.item.i; }
        bool operator!=(const Iterator &o) const { return !(*this == o); }
        bool operator<(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i < other.item.i; }
        bool operator<=(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i <= other.item.i; }
        bool operator>(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i > other.item.i; }
        bool operator>=(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i >= other.item.i; }
        bool operator==(const ConstIterator &o) const { return item.d == o.item.d && item.i == o.item.i; }
        bool operator!=(const ConstIterator &o) const { return !(*this == o); }
        bool operator<(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i < other.item.i; }
        bool operator<=(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i <= other.item.i; }
        bool operator>(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i > other.item.i; }
        bool operator>=(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i >= other.item.i; }
        Iterator &operator++() { ++item.i; return *this; }
        Iterator operator++(int) { Iterator n = *this; ++item.i; return n; }
        Iterator &operator--() { item.i--; return *this; }
        Iterator operator--(int) { Iterator n = *this; item.i--; return n; }
        Iterator &operator+=(qsizetype j) { item.i += j; return *this; }
        Iterator &operator-=(qsizetype j) { item.i -= j; return *this; }
        Iterator operator+(qsizetype j) const { return Iterator({ item.d, item.i + j }); }
        Iterator operator-(qsizetype j) const { return Iterator({ item.d, item.i - j }); }
        qsizetype operator-(Iterator j) const { return item.i - j.item.i; }
    };

    class ConstIterator {
        QCborValueRef item;
        friend class Iterator;
        friend class QCborArray;
        ConstIterator(QCborContainerPrivate *dd, qsizetype ii) : item(dd, ii) {}
    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef qsizetype difference_type;
        typedef const QCborValue value_type;
        typedef const QCborValueRef reference;
        typedef const QCborValueRef *pointer;

        Q_DECL_CONSTEXPR ConstIterator() = default;
        Q_DECL_CONSTEXPR ConstIterator(const ConstIterator &) = default;
        ConstIterator &operator=(const ConstIterator &other)
        {
            // rebind the reference
            item.d = other.item.d;
            item.i = other.item.i;
            return *this;
        }

        const QCborValueRef operator*() const { return item; }
        const QCborValueRef *operator->() const { return &item; }
        const QCborValueRef operator[](qsizetype j) { return { item.d, item.i + j }; }

        bool operator==(const Iterator &o) const { return item.d == o.item.d && item.i == o.item.i; }
        bool operator!=(const Iterator &o) const { return !(*this == o); }
        bool operator<(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i < other.item.i; }
        bool operator<=(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i <= other.item.i; }
        bool operator>(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i > other.item.i; }
        bool operator>=(const Iterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i >= other.item.i; }
        bool operator==(const ConstIterator &o) const { return item.d == o.item.d && item.i == o.item.i; }
        bool operator!=(const ConstIterator &o) const { return !(*this == o); }
        bool operator<(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i < other.item.i; }
        bool operator<=(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i <= other.item.i; }
        bool operator>(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i > other.item.i; }
        bool operator>=(const ConstIterator& other) const { Q_ASSERT(item.d == other.item.d); return item.i >= other.item.i; }
        ConstIterator &operator++() { ++item.i; return *this; }
        ConstIterator operator++(int) { ConstIterator n = *this; ++item.i; return n; }
        ConstIterator &operator--() { item.i--; return *this; }
        ConstIterator operator--(int) { ConstIterator n = *this; item.i--; return n; }
        ConstIterator &operator+=(qsizetype j) { item.i += j; return *this; }
        ConstIterator &operator-=(qsizetype j) { item.i -= j; return *this; }
        ConstIterator operator+(qsizetype j) const { return ConstIterator({ item.d, item.i + j }); }
        ConstIterator operator-(qsizetype j) const { return ConstIterator({ item.d, item.i - j }); }
        qsizetype operator-(ConstIterator j) const { return item.i - j.item.i; }
    };

    typedef qsizetype size_type;
    typedef QCborValue value_type;
    typedef value_type *pointer;
    typedef const value_type *const_pointer;
    typedef QCborValue &reference;
    typedef const QCborValue &const_reference;
    typedef qsizetype difference_type;

    QCborArray() noexcept;
    QCborArray(const QCborArray &other) noexcept;
    QCborArray &operator=(const QCborArray &other) noexcept;
    QCborArray(std::initializer_list<QCborValue> args)
        : QCborArray()
    {
        detach(qsizetype(args.size()));
        for (const QCborValue &v : args)
            append(v);
    }
    ~QCborArray();

    void swap(QCborArray &other) noexcept
    {
        qSwap(d, other.d);
    }

    QCborValue toCborValue() const { return *this; }

    qsizetype size() const noexcept;
    bool isEmpty() const { return size() == 0; }
    void clear();

    QCborValue at(qsizetype i) const;
    QCborValue first() const { return at(0); }
    QCborValue last() const { return at(size() - 1); }
    const QCborValue operator[](qsizetype i) const { return at(i); }
    QCborValueRef first() { Q_ASSERT(!isEmpty()); return begin()[0]; }
    QCborValueRef last() { Q_ASSERT(!isEmpty()); return begin()[size() - 1]; }
    QCborValueRef operator[](qsizetype i)
    {
        if (i >= size())
            insert(i, QCborValue());
        return begin()[i];
    }

    void insert(qsizetype i, const QCborValue &value);
    void insert(qsizetype i, QCborValue &&value);
    void prepend(const QCborValue &value) { insert(0, value); }
    void prepend(QCborValue &&value) { insert(0, std::move(value)); }
    void append(const QCborValue &value) { insert(-1, value); }
    void append(QCborValue &&value) { insert(-1, std::move(value)); }
    QCborValue extract(ConstIterator it) { return extract(Iterator{ it.item.d, it.item.i }); }
    QCborValue extract(Iterator it);
    void removeAt(qsizetype i);
    QCborValue takeAt(qsizetype i) { Q_ASSERT(i < size()); return extract(begin() + i); }
    void removeFirst() { removeAt(0); }
    void removeLast() { removeAt(size() - 1); }
    QCborValue takeFirst() { return takeAt(0); }
    QCborValue takeLast() { return takeAt(size() - 1); }

    bool contains(const QCborValue &value) const;

    int compare(const QCborArray &other) const noexcept Q_DECL_PURE_FUNCTION;
#if 0 && __has_include(<compare>)
    std::strong_ordering operator<=>(const QCborArray &other) const
    {
        int c = compare(other);
        if (c > 0) return std::strong_ordering::greater;
        if (c == 0) return std::strong_ordering::equivalent;
        return std::strong_ordering::less;
    }
#else
    bool operator==(const QCborArray &other) const noexcept
    { return compare(other) == 0; }
    bool operator!=(const QCborArray &other) const noexcept
    { return !(*this == other); }
    bool operator<(const QCborArray &other) const
    { return compare(other) < 0; }
#endif

    typedef Iterator iterator;
    typedef ConstIterator const_iterator;
    iterator begin() { detach(); return iterator{d.data(), 0}; }
    const_iterator constBegin() const { return const_iterator{d.data(), 0}; }
    const_iterator begin() const { return constBegin(); }
    const_iterator cbegin() const { return constBegin(); }
    iterator end() { detach(); return iterator{d.data(), size()}; }
    const_iterator constEnd() const { return const_iterator{d.data(), size()}; }
    const_iterator end() const { return constEnd(); }
    const_iterator cend() const { return constEnd(); }
    iterator insert(iterator before, const QCborValue &value)
    { insert(before.item.i, value); return iterator{d.data(), before.item.i}; }
    iterator insert(const_iterator before, const QCborValue &value)
    { insert(before.item.i, value); return iterator{d.data(), before.item.i}; }
    iterator erase(iterator it) { removeAt(it.item.i); return iterator{d.data(), it.item.i}; }
    iterator erase(const_iterator it) { removeAt(it.item.i); return iterator{d.data(), it.item.i}; }

    void push_back(const QCborValue &t) { append(t); }
    void push_front(const QCborValue &t) { prepend(t); }
    void pop_front() { removeFirst(); }
    void pop_back() { removeLast(); }
    bool empty() const { return isEmpty(); }

    // convenience
    QCborArray operator+(const QCborValue &v) const
    { QCborArray n = *this; n += v; return n; }
    QCborArray &operator+=(const QCborValue &v)
    { append(v); return *this; }
    QCborArray &operator<<(const QCborValue &v)
    { append(v); return *this; }

    static QCborArray fromStringList(const QStringList &list);
    static QCborArray fromVariantList(const QVariantList &list);
    static QCborArray fromJsonArray(const QJsonArray &array);
    QVariantList toVariantList() const;
    QJsonArray toJsonArray() const;

private:
    void detach(qsizetype reserve = 0);

    friend QCborValue;
    friend QCborValueRef;
    friend class QJsonPrivate::Variant;
    explicit QCborArray(QCborContainerPrivate &dd) noexcept;
    QExplicitlySharedDataPointer<QCborContainerPrivate> d;
};

Q_DECLARE_SHARED(QCborArray)

inline QCborValue::QCborValue(QCborArray &&a)
    : n(-1), container(a.d.take()), t(Array)
{
}

inline QCborArray QCborValueRef::toArray() const
{
    return concrete().toArray();
}

inline QCborArray QCborValueRef::toArray(const QCborArray &a) const
{
    return concrete().toArray(a);
}

Q_CORE_EXPORT uint qHash(const QCborArray &array, uint seed = 0);

#if !defined(QT_NO_DEBUG_STREAM)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QCborArray &a);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QCborArray &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QCborArray &);
#endif

QT_END_NAMESPACE

#endif // QCBORARRAY_H
