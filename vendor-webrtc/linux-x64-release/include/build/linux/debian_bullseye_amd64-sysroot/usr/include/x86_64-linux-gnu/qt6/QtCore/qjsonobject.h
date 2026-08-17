// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QJSONOBJECT_H
#define QJSONOBJECT_H

#include <QtCore/qjsonvalue.h>
#include <QtCore/qiterator.h>
#include <QtCore/qpair.h>
#include <QtCore/qshareddata.h>
#include <initializer_list>

QT_BEGIN_NAMESPACE

class QDebug;

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
        o.swap(other.o);
    }

    static QJsonObject fromVariantMap(const QVariantMap &map);
    QVariantMap toVariantMap() const;
    static QJsonObject fromVariantHash(const QVariantHash &map);
    QVariantHash toVariantHash() const;

    QStringList keys() const;
    qsizetype size() const;
    inline qsizetype count() const { return size(); }
    inline qsizetype length() const { return size(); }
    bool isEmpty() const;

    QJsonValue value(const QString &key) const;
    QJsonValue operator[] (const QString &key) const;
    QJsonValueRef operator[] (const QString &key);
    QJsonValue value(QStringView key) const;
    QJsonValue value(QLatin1StringView key) const;
    QJsonValue operator[] (QStringView key) const { return value(key); }
    QJsonValue operator[] (QLatin1StringView key) const { return value(key); }
    QJsonValueRef operator[] (QStringView key);
    QJsonValueRef operator[] (QLatin1StringView key);

    void remove(const QString &key);
    QJsonValue take(const QString &key);
    bool contains(const QString &key) const;
    void remove(QStringView key);
    void remove(QLatin1StringView key);
    QJsonValue take(QStringView key);
    QJsonValue take(QLatin1StringView key);
    bool contains(QStringView key) const;
    bool contains(QLatin1StringView key) const;

    bool operator==(const QJsonObject &other) const;
    bool operator!=(const QJsonObject &other) const;

    class const_iterator;

    class iterator
    {
        friend class const_iterator;
        friend class QJsonObject;
        QJsonValueRef item;

    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef qsizetype difference_type;
        typedef QJsonValue value_type;
        typedef QJsonValueRef reference;
        typedef QJsonValueRef *pointer;

        inline iterator() : item(static_cast<QJsonObject*>(nullptr), 0) { }
        inline iterator(QJsonObject *obj, qsizetype index) : item(obj, index) { }

        constexpr iterator(const iterator &other) = default;
        iterator &operator=(const iterator &other)
        {
            item.rebind(other.item);
            return *this;
        }

        inline QString key() const { return item.objectKey(); }
        inline QJsonValueRef value() const { return item; }
        inline QJsonValueRef operator*() const { return item; }
        inline const QJsonValueConstRef *operator->() const { return &item; }
        inline QJsonValueRef *operator->() { return &item; }
        inline QJsonValueRef operator[](qsizetype j) const { return *(*this + j); }

        inline bool operator==(const iterator &other) const
        { return item.d == other.item.d && item.index == other.item.index; }
        inline bool operator!=(const iterator &other) const { return !(*this == other); }
        bool operator<(const iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index < other.item.index; }
        bool operator<=(const iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index <= other.item.index; }
        bool operator>(const iterator& other) const { return !(*this <= other); }
        bool operator>=(const iterator& other) const { return !(*this < other); }

        inline iterator &operator++() { ++item.index; return *this; }
        inline iterator operator++(int) { iterator r = *this; ++item.index; return r; }
        inline iterator &operator--() { --item.index; return *this; }
        inline iterator operator--(int) { iterator r = *this; --item.index; return r; }
        inline iterator operator+(qsizetype j) const { iterator r = *this; return r += j; }
        inline iterator operator-(qsizetype j) const { return operator+(-j); }
        inline iterator &operator+=(qsizetype j) { item.index += quint64(j); return *this; }
        inline iterator &operator-=(qsizetype j) { item.index -= quint64(j); return *this; }
        qsizetype operator-(iterator j) const { return item.index - j.item.index; }

    public:
        inline bool operator==(const const_iterator &other) const
        { return item.d == other.item.d && item.index == other.item.index; }
        inline bool operator!=(const const_iterator &other) const { return !(*this == other); }
        bool operator<(const const_iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index < other.item.index; }
        bool operator<=(const const_iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index <= other.item.index; }
        bool operator>(const const_iterator& other) const { return !(*this <= other); }
        bool operator>=(const const_iterator& other) const { return !(*this < other); }
    };
    friend class iterator;

    class const_iterator
    {
        friend class iterator;
        QJsonValueConstRef item;

    public:
        typedef std::random_access_iterator_tag iterator_category;
        typedef qsizetype difference_type;
        typedef QJsonValue value_type;
        typedef const QJsonValueConstRef reference;
        typedef const QJsonValueConstRef *pointer;

        inline const_iterator() : item(static_cast<QJsonObject*>(nullptr), 0) { }
        inline const_iterator(const QJsonObject *obj, qsizetype index)
            : item(const_cast<QJsonObject*>(obj), index) { }
        inline const_iterator(const iterator &other)
            : item(other.item) { }

        constexpr const_iterator(const const_iterator &other) = default;
        const_iterator &operator=(const const_iterator &other)
        {
            item.rebind(other.item);
            return *this;
        }

        inline QString key() const { return item.objectKey(); }
        inline QJsonValueConstRef value() const { return item; }
        inline const QJsonValueConstRef operator*() const { return item; }
        inline const QJsonValueConstRef *operator->() const { return &item; }
        inline QJsonValueConstRef operator[](qsizetype j) const { return *(*this + j); }

        inline bool operator==(const const_iterator &other) const
        { return item.d == other.item.d && item.index == other.item.index; }
        inline bool operator!=(const const_iterator &other) const { return !(*this == other); }
        bool operator<(const const_iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index < other.item.index; }
        bool operator<=(const const_iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index <= other.item.index; }
        bool operator>(const const_iterator& other) const { return !(*this <= other); }
        bool operator>=(const const_iterator& other) const { return !(*this < other); }

        inline const_iterator &operator++() { ++item.index; return *this; }
        inline const_iterator operator++(int) { const_iterator r = *this; ++item.index; return r; }
        inline const_iterator &operator--() { --item.index; return *this; }
        inline const_iterator operator--(int) { const_iterator r = *this; --item.index; return r; }
        inline const_iterator operator+(qsizetype j) const { const_iterator r = *this; return r += j; }
        inline const_iterator operator-(qsizetype j) const { return operator+(-j); }
        inline const_iterator &operator+=(qsizetype j) { item.index += quint64(j); return *this; }
        inline const_iterator &operator-=(qsizetype j) { item.index -= quint64(j); return *this; }
        qsizetype operator-(const_iterator j) const { return item.index - j.item.index; }

        inline bool operator==(const iterator &other) const
        { return item.d == other.item.d && item.index == other.item.index; }
        inline bool operator!=(const iterator &other) const { return !(*this == other); }
        bool operator<(const iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index < other.item.index; }
        bool operator<=(const iterator& other) const
        { Q_ASSERT(item.d == other.item.d); return item.index <= other.item.index; }
        bool operator>(const iterator& other) const { return !(*this <= other); }
        bool operator>=(const iterator& other) const { return !(*this < other); }
    };
    friend class const_iterator;

    // STL style
    inline iterator begin() { detach(); return iterator(this, 0); }
    inline const_iterator begin() const { return const_iterator(this, 0); }
    inline const_iterator constBegin() const { return const_iterator(this, 0); }
    inline iterator end() { detach(); return iterator(this, size()); }
    inline const_iterator end() const { return const_iterator(this, size()); }
    inline const_iterator constEnd() const { return const_iterator(this, size()); }
    iterator erase(iterator it);

    // more Qt
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
    iterator find(const QString &key);
    const_iterator find(const QString &key) const { return constFind(key); }
    const_iterator constFind(const QString &key) const;
    iterator insert(const QString &key, const QJsonValue &value);
    iterator find(QStringView key);
    iterator find(QLatin1StringView key);
    const_iterator find(QStringView key) const { return constFind(key); }
    const_iterator find(QLatin1StringView key) const { return constFind(key); }
    const_iterator constFind(QStringView key) const;
    const_iterator constFind(QLatin1StringView key) const;
    iterator insert(QStringView key, const QJsonValue &value);
    iterator insert(QLatin1StringView key, const QJsonValue &value);

    // STL compatibility
    typedef QJsonValue mapped_type;
    typedef QString key_type;
    typedef qsizetype size_type;

    inline bool empty() const { return isEmpty(); }

private:
    friend class QJsonValue;
    friend class QJsonDocument;
    friend class QJsonPrivate::Value;
    friend class QJsonValueConstRef;
    friend class QJsonValueRef;
    friend class QCborMap;
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonObject &);

    QJsonObject(QCborContainerPrivate *object);
    bool detach(qsizetype reserve = 0);

    template <typename T> QJsonValue valueImpl(T key) const;
    template <typename T> QJsonValueRef atImpl(T key);
    template <typename T> void removeImpl(T key);
    template <typename T> QJsonValue takeImpl(T key);
    template <typename T> bool containsImpl(T key) const;
    template <typename T> iterator findImpl(T key);
    template <typename T> const_iterator constFindImpl(T key) const;
    template <typename T> iterator insertImpl(T key, const QJsonValue &value);

#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0) && !defined(QT_BOOTSTRAPPED)
    QString keyAt(qsizetype i) const;
    QJsonValue valueAt(qsizetype i) const;
    void setValueAt(qsizetype i, const QJsonValue &val);
#endif
    void removeAt(qsizetype i);
    template <typename T> iterator insertAt(qsizetype i, T key, const QJsonValue &val, bool exists);

    QExplicitlySharedDataPointer<QCborContainerPrivate> o;
};

Q_DECLARE_SHARED(QJsonObject)

#if QT_VERSION >= QT_VERSION_CHECK(7, 0, 0) || defined(QT_BOOTSTRAPPED)
inline QJsonValueConstRef::QJsonValueConstRef(QJsonObject *o, qsizetype idx)
    : d(o ? o->o.data() : nullptr), is_object(true), index(idx)
{}
#endif

Q_CORE_EXPORT size_t qHash(const QJsonObject &object, size_t seed = 0);

#if !defined(QT_NO_DEBUG_STREAM) && !defined(QT_JSON_READONLY)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonObject &);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QJsonObject &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QJsonObject &);
#endif

QT_END_NAMESPACE

#endif // QJSONOBJECT_H
