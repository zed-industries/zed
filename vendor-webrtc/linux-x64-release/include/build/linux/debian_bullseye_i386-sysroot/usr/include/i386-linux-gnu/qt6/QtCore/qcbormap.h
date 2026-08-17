// Copyright (C) 2022 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCBORMAP_H
#define QCBORMAP_H

#include <QtCore/qcborvalue.h>
#include <QtCore/qpair.h>

#include <initializer_list>

QT_BEGIN_NAMESPACE

class QJsonObject;
class QDataStream;

namespace QJsonPrivate { class Variant; }

class QCborContainerPrivate;
class Q_CORE_EXPORT QCborMap
{
public:
    typedef QPair<QCborValue, QCborValue> value_type;
    typedef QCborValue key_type;
    typedef QCborValue mapped_type;
    typedef qsizetype size_type;

    class ConstIterator;
    class Iterator {
        QCborValueRef item {};      // points to the value
        friend class ConstIterator;
        friend class QCborMap;
        Iterator(QCborContainerPrivate *dd, qsizetype ii) : item(dd, ii) {}
    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef qsizetype difference_type;
        typedef QPair<QCborValueConstRef, QCborValueRef> value_type;
        typedef QPair<QCborValueConstRef, QCborValueRef> reference;
        typedef QPair<QCborValueConstRef, QCborValueRef> pointer;

        constexpr Iterator() = default;
        constexpr Iterator(const Iterator &) = default;
        Iterator &operator=(const Iterator &other)
        {
            // rebind the reference
            item.d = other.item.d;
            item.i = other.item.i;
            return *this;
        }

        value_type operator*() const { return { QCborValueRef{item.d, item.i - 1}, item }; }
        value_type operator[](qsizetype j) const { return *(*this + j); }
        QCborValueRef *operator->() { return &item; }
        const QCborValueConstRef *operator->() const { return &item; }
#if QT_VERSION >= QT_VERSION_CHECK(7,0,0)
        QCborValueConstRef
#else
        QCborValue
#endif
        key() const { return QCborValueRef(item.d, item.i - 1); }
        QCborValueRef value() const { return item; }

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
        Iterator &operator++() { item.i += 2; return *this; }
        Iterator operator++(int) { Iterator n = *this; item.i += 2; return n; }
        Iterator &operator--() { item.i -= 2; return *this; }
        Iterator operator--(int) { Iterator n = *this; item.i -= 2; return n; }
        Iterator &operator+=(qsizetype j) { item.i += 2 * j; return *this; }
        Iterator &operator-=(qsizetype j) { item.i -= 2 * j; return *this; }
        Iterator operator+(qsizetype j) const { return Iterator({ item.d, item.i + 2 * j }); }
        Iterator operator-(qsizetype j) const { return Iterator({ item.d, item.i - 2 * j }); }
        qsizetype operator-(Iterator j) const { return (item.i - j.item.i) / 2; }
    };

    class ConstIterator {
        QCborValueConstRef item;     // points to the value
        friend class Iterator;
        friend class QCborMap;
        friend class QCborValue;
        friend class QCborValueRef;
        constexpr ConstIterator(QCborValueConstRef it) : item{it} {}
        ConstIterator(QCborContainerPrivate *dd, qsizetype ii) : item(dd, ii) {}
    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef qsizetype difference_type;
        typedef QPair<QCborValueConstRef, QCborValueConstRef> value_type;
        typedef QPair<QCborValueConstRef, QCborValueConstRef> reference;
        typedef QPair<QCborValueConstRef, QCborValueConstRef> pointer;

        constexpr ConstIterator() = default;
        constexpr ConstIterator(const ConstIterator &) = default;
        ConstIterator &operator=(const ConstIterator &other)
        {
            // rebind the reference
            item.d = other.item.d;
            item.i = other.item.i;
            return *this;
        }

        value_type operator*() const { return { QCborValueRef(item.d, item.i - 1), item }; }
        value_type operator[](qsizetype j) const { return *(*this + j); }
        const QCborValueConstRef *operator->() const { return &item; }
#if QT_VERSION >= QT_VERSION_CHECK(7,0,0)
        QCborValueConstRef
#else
        QCborValue
#endif
        key() const { return QCborValueRef(item.d, item.i - 1); }
        QCborValueConstRef value() const { return item; }

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
        ConstIterator &operator++() { item.i += 2; return *this; }
        ConstIterator operator++(int) { ConstIterator n = *this; item.i += 2; return n; }
        ConstIterator &operator--() { item.i -= 2; return *this; }
        ConstIterator operator--(int) { ConstIterator n = *this; item.i -= 2; return n; }
        ConstIterator &operator+=(qsizetype j) { item.i += 2 * j; return *this; }
        ConstIterator &operator-=(qsizetype j) { item.i -= 2 * j; return *this; }
        ConstIterator operator+(qsizetype j) const { return ConstIterator{ item.d, item.i + 2 * j }; }
        ConstIterator operator-(qsizetype j) const { return ConstIterator{ item.d, item.i - 2 * j }; }
        qsizetype operator-(ConstIterator j) const { return (item.i - j.item.i) / 2; }
    };

    QCborMap()  noexcept;
    QCborMap(const QCborMap &other) noexcept;
    QCborMap &operator=(const QCborMap &other) noexcept;
    QCborMap(std::initializer_list<value_type> args)
        : QCborMap()
    {
        detach(args.size());
        for (const auto &pair : args)
           insert(pair.first, pair.second);
    }
    ~QCborMap();

    void swap(QCborMap &other) noexcept
    {
        d.swap(other.d);
    }

    QCborValue toCborValue() const { return *this; }

    qsizetype size() const noexcept Q_DECL_PURE_FUNCTION;
    bool isEmpty() const { return size() == 0; }
    void clear();
    QList<QCborValue> keys() const;

    QCborValue value(qint64 key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
    QCborValue value(QLatin1StringView key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
    QCborValue value(const QString & key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
    QCborValue value(const QCborValue &key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
#if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
    template<size_t N> QT_ASCII_CAST_WARN const QCborValue value(const char (&key)[N]) const
    { return value(QString::fromUtf8(key, N - 1)); }
#endif
    const QCborValue operator[](qint64 key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
    const QCborValue operator[](QLatin1StringView key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
    const QCborValue operator[](const QString & key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
    const QCborValue operator[](const QCborValue &key) const
    { const_iterator it = find(key); return it == end() ? QCborValue() : it.value(); }
#if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
    template<size_t N> QT_ASCII_CAST_WARN const QCborValue operator[](const char (&key)[N]) const
    { return operator[](QString::fromUtf8(key, N - 1)); }
#endif
    QCborValueRef operator[](qint64 key);
    QCborValueRef operator[](QLatin1StringView key);
    QCborValueRef operator[](const QString & key);
    QCborValueRef operator[](const QCborValue &key);

    QCborValue take(qint64 key)
    { const_iterator it = constFind(key); if (it != constEnd()) return extract(it); return QCborValue(); }
    QCborValue take(QLatin1StringView key)
    { const_iterator it = constFind(key); if (it != constEnd()) return extract(it); return QCborValue(); }
    QCborValue take(const QString &key)
    { const_iterator it = constFind(key); if (it != constEnd()) return extract(it); return QCborValue(); }
    QCborValue take(const QCborValue &key)
    { const_iterator it = constFind(key); if (it != constEnd()) return extract(it); return QCborValue(); }
    void remove(qint64 key)
    { const_iterator it = constFind(key); if (it != constEnd()) erase(it); }
    void remove(QLatin1StringView key)
    { const_iterator it = constFind(key); if (it != constEnd()) erase(it); }
    void remove(const QString & key)
    { const_iterator it = constFind(key); if (it != constEnd()) erase(it); }
    void remove(const QCborValue &key)
    { const_iterator it = constFind(key); if (it != constEnd()) erase(it); }
    bool contains(qint64 key) const
    { const_iterator it = find(key); return it != end(); }
    bool contains(QLatin1StringView key) const
    { const_iterator it = find(key); return it != end(); }
    bool contains(const QString & key) const
    { const_iterator it = find(key); return it != end(); }
    bool contains(const QCborValue &key) const
    { const_iterator it = find(key); return it != end(); }

    int compare(const QCborMap &other) const noexcept Q_DECL_PURE_FUNCTION;
#if 0 && __has_include(<compare>)
    std::strong_ordering operator<=>(const QCborMap &other) const
    {
        int c = compare(other);
        if (c > 0) return std::strong_ordering::greater;
        if (c == 0) return std::strong_ordering::equivalent;
        return std::strong_ordering::less;
    }
#else
    bool operator==(const QCborMap &other) const noexcept
    { return compare(other) == 0; }
    bool operator!=(const QCborMap &other) const noexcept
    { return !(*this == other); }
    bool operator<(const QCborMap &other) const
    { return compare(other) < 0; }
#endif

    typedef Iterator iterator;
    typedef ConstIterator const_iterator;
    iterator begin() { detach(); return iterator{d.data(), 1}; }
    const_iterator constBegin() const { return const_iterator{d.data(), 1}; }
    const_iterator begin() const { return constBegin(); }
    const_iterator cbegin() const { return constBegin(); }
    iterator end() { detach(); return iterator{d.data(), 2 * size() + 1}; }
    const_iterator constEnd() const { return const_iterator{d.data(), 2 * size() + 1}; }
    const_iterator end() const { return constEnd(); }
    const_iterator cend() const { return constEnd(); }
    iterator erase(iterator it);
    iterator erase(const_iterator it) { return erase(iterator{ it.item.d, it.item.i }); }
    QCborValue extract(iterator it);
    QCborValue extract(const_iterator it) { return extract(iterator{ it.item.d, it.item.i }); }
    bool empty() const { return isEmpty(); }

    iterator find(qint64 key);
    iterator find(QLatin1StringView key);
    iterator find(const QString & key);
    iterator find(const QCborValue &key);
    const_iterator constFind(qint64 key) const;
    const_iterator constFind(QLatin1StringView key) const;
    const_iterator constFind(const QString & key) const;
    const_iterator constFind(const QCborValue &key) const;
    const_iterator find(qint64 key) const { return constFind(key); }
    const_iterator find(QLatin1StringView key) const { return constFind(key); }
    const_iterator find(const QString & key) const { return constFind(key); }
    const_iterator find(const QCborValue &key) const { return constFind(key); }

    iterator insert(qint64 key, const QCborValue &value_)
    {
        QCborValueRef v = operator[](key);  // detaches
        v = value_;
        return { d.data(), v.i };
    }
    iterator insert(QLatin1StringView key, const QCborValue &value_)
    {
        QCborValueRef v = operator[](key);  // detaches
        v = value_;
        return { d.data(), v.i };
    }
    iterator insert(const QString &key, const QCborValue &value_)
    {
        QCborValueRef v = operator[](key);  // detaches
        v = value_;
        return { d.data(), v.i };
    }
    iterator insert(const QCborValue &key, const QCborValue &value_)
    {
        QCborValueRef v = operator[](key);  // detaches
        v = value_;
        return { d.data(), v.i };
    }
    iterator insert(value_type v) { return insert(v.first, v.second); }

    static QCborMap fromVariantMap(const QVariantMap &map);
    static QCborMap fromVariantHash(const QVariantHash &hash);
    static QCborMap fromJsonObject(const QJsonObject &o);
    static QCborMap fromJsonObject(QJsonObject &&o) noexcept;
    QVariantMap toVariantMap() const;
    QVariantHash toVariantHash() const;
    QJsonObject toJsonObject() const;

private:
    friend class QCborContainerPrivate;
    friend class QCborValue;
    friend class QCborValueRef;
    friend class QJsonPrivate::Variant;
    void detach(qsizetype reserve = 0);

    explicit QCborMap(QCborContainerPrivate &dd) noexcept;
    QExplicitlySharedDataPointer<QCborContainerPrivate> d;
};

Q_DECLARE_SHARED(QCborMap)

inline QCborValue::QCborValue(QCborMap &&m)
    : n(-1), container(m.d.take()), t(Map)
{
}

#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0) && !defined(QT_BOOTSTRAPPED)
inline QCborMap QCborValueRef::toMap() const
{
    return concrete().toMap();
}

inline QCborMap QCborValueRef::toMap(const QCborMap &m) const
{
    return concrete().toMap(m);
}
#endif

inline QCborMap QCborValueConstRef::toMap() const
{
    return concrete().toMap();
}

inline QCborMap QCborValueConstRef::toMap(const QCborMap &m) const
{
    return concrete().toMap(m);
}

Q_CORE_EXPORT size_t qHash(const QCborMap &map, size_t seed = 0);

#if !defined(QT_NO_DEBUG_STREAM)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QCborMap &m);
#endif

#ifndef QT_NO_DATASTREAM
#if QT_CONFIG(cborstreamwriter)
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QCborMap &);
#endif
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QCborMap &);
#endif


QT_END_NAMESPACE

#endif // QCBORMAP_H
