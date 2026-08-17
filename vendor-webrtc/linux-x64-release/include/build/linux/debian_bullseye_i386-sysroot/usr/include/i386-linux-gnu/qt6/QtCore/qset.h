// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSET_H
#define QSET_H

#include <QtCore/qhash.h>
#include <QtCore/qcontainertools_impl.h>

#include <initializer_list>
#include <iterator>

QT_BEGIN_NAMESPACE


template <class T>
class QSet
{
    typedef QHash<T, QHashDummyValue> Hash;

public:
    inline QSet() noexcept {}
    inline QSet(std::initializer_list<T> list)
        : QSet(list.begin(), list.end()) {}
    template <typename InputIterator, QtPrivate::IfIsInputIterator<InputIterator> = true>
    inline QSet(InputIterator first, InputIterator last)
    {
        QtPrivate::reserveIfForwardIterator(this, first, last);
        for (; first != last; ++first)
            insert(*first);
    }

    // compiler-generated copy/move ctor/assignment operators are fine!
    // compiler-generated destructor is fine!

    inline void swap(QSet<T> &other) noexcept { q_hash.swap(other.q_hash); }

#ifndef Q_CLANG_QDOC
    template <typename U = T>
    QTypeTraits::compare_eq_result_container<QSet, U> operator==(const QSet<T> &other) const
    { return q_hash == other.q_hash; }
    template <typename U = T>
    QTypeTraits::compare_eq_result_container<QSet, U> operator!=(const QSet<T> &other) const
    { return q_hash != other.q_hash; }
#else
    bool operator==(const QSet &other) const;
    bool operator!=(const QSet &other) const;
#endif

    inline qsizetype size() const { return q_hash.size(); }

    inline bool isEmpty() const { return q_hash.isEmpty(); }

    inline qsizetype capacity() const { return q_hash.capacity(); }
    inline void reserve(qsizetype size);
    inline void squeeze() { q_hash.squeeze(); }

    inline void detach() { q_hash.detach(); }
    inline bool isDetached() const { return q_hash.isDetached(); }

    inline void clear() { q_hash.clear(); }

    inline bool remove(const T &value) { return q_hash.remove(value) != 0; }

    template <typename Pred>
    inline qsizetype removeIf(Pred predicate)
    {
        return QtPrivate::qset_erase_if(*this, predicate);
    }

    inline bool contains(const T &value) const { return q_hash.contains(value); }

    bool contains(const QSet<T> &set) const;

    class const_iterator;

    class iterator
    {
        typedef QHash<T, QHashDummyValue> Hash;
        typename Hash::iterator i;
        friend class const_iterator;
        friend class QSet<T>;

    public:
        typedef std::forward_iterator_tag iterator_category;
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef const T *pointer;
        typedef const T &reference;

        inline iterator() {}
        inline iterator(typename Hash::iterator o) : i(o) {}
        inline iterator(const iterator &o) : i(o.i) {}
        inline iterator &operator=(const iterator &o) { i = o.i; return *this; }
        inline const T &operator*() const { return i.key(); }
        inline const T *operator->() const { return &i.key(); }
        inline bool operator==(const iterator &o) const { return i == o.i; }
        inline bool operator!=(const iterator &o) const { return i != o.i; }
        inline bool operator==(const const_iterator &o) const
            { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const
            { return i != o.i; }
        inline iterator &operator++() { ++i; return *this; }
        inline iterator operator++(int) { iterator r = *this; ++i; return r; }
    };

    class const_iterator
    {
        typedef QHash<T, QHashDummyValue> Hash;
        typename Hash::const_iterator i;
        friend class iterator;
        friend class QSet<T>;

    public:
        typedef std::forward_iterator_tag iterator_category;
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef const T *pointer;
        typedef const T &reference;

        inline const_iterator() {}
        inline const_iterator(typename Hash::const_iterator o) : i(o) {}
        inline const_iterator(const const_iterator &o) : i(o.i) {}
        inline const_iterator(const iterator &o)
            : i(o.i) {}
        inline const_iterator &operator=(const const_iterator &o) { i = o.i; return *this; }
        inline const T &operator*() const { return i.key(); }
        inline const T *operator->() const { return &i.key(); }
        inline bool operator==(const const_iterator &o) const { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const { return i != o.i; }
        inline const_iterator &operator++() { ++i; return *this; }
        inline const_iterator operator++(int) { const_iterator r = *this; ++i; return r; }
    };

    // STL style
    inline iterator begin() { return q_hash.begin(); }
    inline const_iterator begin() const noexcept { return q_hash.begin(); }
    inline const_iterator cbegin() const noexcept { return q_hash.begin(); }
    inline const_iterator constBegin() const noexcept { return q_hash.constBegin(); }
    inline iterator end() { return q_hash.end(); }
    inline const_iterator end() const noexcept { return q_hash.end(); }
    inline const_iterator cend() const noexcept { return q_hash.end(); }
    inline const_iterator constEnd() const noexcept { return q_hash.constEnd(); }

    iterator erase(const_iterator i)
    {
        Q_ASSERT(i != constEnd());
        return q_hash.erase(i.i);
    }

    // more Qt
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
    inline qsizetype count() const { return q_hash.size(); }
    inline iterator insert(const T &value)
        { return q_hash.insert(value, QHashDummyValue()); }
    inline iterator insert(T &&value)
        { return q_hash.emplace(std::move(value), QHashDummyValue()); }
    iterator find(const T &value) { return q_hash.find(value); }
    const_iterator find(const T &value) const { return q_hash.find(value); }
    inline const_iterator constFind(const T &value) const { return find(value); }
    QSet<T> &unite(const QSet<T> &other);
    QSet<T> &intersect(const QSet<T> &other);
    bool intersects(const QSet<T> &other) const;
    QSet<T> &subtract(const QSet<T> &other);

    // STL compatibility
    typedef T key_type;
    typedef T value_type;
    typedef value_type *pointer;
    typedef const value_type *const_pointer;
    typedef value_type &reference;
    typedef const value_type &const_reference;
    typedef qptrdiff difference_type;
    typedef qsizetype size_type;

    inline bool empty() const { return isEmpty(); }

    iterator insert(const_iterator, const T &value) { return insert(value); }

    // comfort
    inline QSet<T> &operator<<(const T &value) { insert(value); return *this; }
    inline QSet<T> &operator|=(const QSet<T> &other) { unite(other); return *this; }
    inline QSet<T> &operator|=(const T &value) { insert(value); return *this; }
    inline QSet<T> &operator&=(const QSet<T> &other) { intersect(other); return *this; }
    inline QSet<T> &operator&=(const T &value)
        { QSet<T> result; if (contains(value)) result.insert(value); return (*this = result); }
    inline QSet<T> &operator+=(const QSet<T> &other) { unite(other); return *this; }
    inline QSet<T> &operator+=(const T &value) { insert(value); return *this; }
    inline QSet<T> &operator-=(const QSet<T> &other) { subtract(other); return *this; }
    inline QSet<T> &operator-=(const T &value) { remove(value); return *this; }
    inline QSet<T> operator|(const QSet<T> &other) const
        { QSet<T> result = *this; result |= other; return result; }
    inline QSet<T> operator&(const QSet<T> &other) const
        { QSet<T> result = *this; result &= other; return result; }
    inline QSet<T> operator+(const QSet<T> &other) const
        { QSet<T> result = *this; result += other; return result; }
    inline QSet<T> operator-(const QSet<T> &other) const
        { QSet<T> result = *this; result -= other; return result; }

    QList<T> values() const;

private:
    Hash q_hash;
};

template <typename InputIterator,
          typename ValueType = typename std::iterator_traits<InputIterator>::value_type,
          QtPrivate::IfIsInputIterator<InputIterator> = true>
QSet(InputIterator, InputIterator) -> QSet<ValueType>;

template <typename T>
size_t qHash(const QSet<T> &key, size_t seed = 0)
noexcept(noexcept(qHashRangeCommutative(key.begin(), key.end(), seed)))
{
    return qHashRangeCommutative(key.begin(), key.end(), seed);
}

// inline function implementations

template <class T>
Q_INLINE_TEMPLATE void QSet<T>::reserve(qsizetype asize) { q_hash.reserve(asize); }

template <class T>
Q_INLINE_TEMPLATE QSet<T> &QSet<T>::unite(const QSet<T> &other)
{
    if (!q_hash.isSharedWith(other.q_hash)) {
        for (const T &e : other)
            insert(e);
    }
    return *this;
}

template <class T>
Q_INLINE_TEMPLATE QSet<T> &QSet<T>::intersect(const QSet<T> &other)
{
    QSet<T> copy1;
    QSet<T> copy2;
    if (size() <= other.size()) {
        copy1 = *this;
        copy2 = other;
    } else {
        copy1 = other;
        copy2 = *this;
        *this = copy1;
    }
    for (const auto &e : std::as_const(copy1)) {
        if (!copy2.contains(e))
            remove(e);
    }
    return *this;
}

template <class T>
Q_INLINE_TEMPLATE bool QSet<T>::intersects(const QSet<T> &other) const
{
    const bool otherIsBigger = other.size() > size();
    const QSet &smallestSet = otherIsBigger ? *this : other;
    const QSet &biggestSet = otherIsBigger ? other : *this;
    typename QSet::const_iterator i = smallestSet.cbegin();
    typename QSet::const_iterator e = smallestSet.cend();

    while (i != e) {
        if (biggestSet.contains(*i))
            return true;
        ++i;
    }

    return false;
}

template <class T>
Q_INLINE_TEMPLATE QSet<T> &QSet<T>::subtract(const QSet<T> &other)
{
    if (q_hash.isSharedWith(other.q_hash)) {
        clear();
    } else {
        for (const auto &e : other)
            remove(e);
    }
    return *this;
}

template <class T>
Q_INLINE_TEMPLATE bool QSet<T>::contains(const QSet<T> &other) const
{
    typename QSet<T>::const_iterator i = other.constBegin();
    while (i != other.constEnd()) {
        if (!contains(*i))
            return false;
        ++i;
    }
    return true;
}

template <typename T>
Q_OUTOFLINE_TEMPLATE QList<T> QSet<T>::values() const
{
    QList<T> result;
    result.reserve(size());
    typename QSet<T>::const_iterator i = constBegin();
    while (i != constEnd()) {
        result.append(*i);
        ++i;
    }
    return result;
}

Q_DECLARE_SEQUENTIAL_ITERATOR(Set)

#if !defined(QT_NO_JAVA_STYLE_ITERATORS)
template <typename T>
class QMutableSetIterator
{
    typedef typename QSet<T>::iterator iterator;
    QSet<T> *c;
    iterator i, n;
    inline bool item_exists() const { return c->constEnd() != n; }

public:
    inline QMutableSetIterator(QSet<T> &container)
        : c(&container)
    { i = c->begin(); n = c->end(); }
    inline QMutableSetIterator &operator=(QSet<T> &container)
    { c = &container; i = c->begin(); n = c->end(); return *this; }
    inline void toFront() { i = c->begin(); n = c->end(); }
    inline void toBack() { i = c->end(); n = i; }
    inline bool hasNext() const { return c->constEnd() != i; }
    inline const T &next() { n = i++; return *n; }
    inline const T &peekNext() const { return *i; }
    inline void remove()
    { if (c->constEnd() != n) { i = c->erase(n); n = c->end(); } }
    inline const T &value() const { Q_ASSERT(item_exists()); return *n; }
    inline bool findNext(const T &t)
    { while (c->constEnd() != (n = i)) if (*i++ == t) return true; return false; }
};
#endif // QT_NO_JAVA_STYLE_ITERATORS

template <typename T, typename Predicate>
qsizetype erase_if(QSet<T> &set, Predicate pred)
{
    return QtPrivate::qset_erase_if(set, pred);
}

QT_END_NAMESPACE

#endif // QSET_H
