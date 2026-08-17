// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCONTIGUOUSCACHE_H
#define QCONTIGUOUSCACHE_H

#include <QtCore/qatomic.h>
#include <limits.h>
#include <new>

QT_BEGIN_NAMESPACE

#undef QT_QCONTIGUOUSCACHE_DEBUG


struct Q_CORE_EXPORT QContiguousCacheData
{
    QBasicAtomicInt ref;
    qsizetype alloc;
    qsizetype count;
    qsizetype start;
    qsizetype offset;

    static QContiguousCacheData *allocateData(qsizetype size, qsizetype alignment);
    static void freeData(QContiguousCacheData *data);

#ifdef QT_QCONTIGUOUSCACHE_DEBUG
    void dump() const;
#endif
};

template <typename T>
struct QContiguousCacheTypedData : public QContiguousCacheData
{
    T array[1];
};

template<typename T>
class QContiguousCache {
    static_assert(std::is_nothrow_destructible_v<T>, "Types with throwing destructors are not supported in Qt containers.");

    typedef QContiguousCacheTypedData<T> Data;
    Data *d;
public:
    // STL compatibility
    typedef T value_type;
    typedef value_type* pointer;
    typedef const value_type* const_pointer;
    typedef value_type& reference;
    typedef const value_type& const_reference;
    typedef qptrdiff difference_type;
    typedef qsizetype size_type;

    explicit QContiguousCache(qsizetype capacity = 0);
    QContiguousCache(const QContiguousCache<T> &v) : d(v.d) { d->ref.ref(); }

    inline ~QContiguousCache() { if (!d) return; if (!d->ref.deref()) freeData(d); }

    inline void detach() { if (d->ref.loadRelaxed() != 1) detach_helper(); }
    inline bool isDetached() const { return d->ref.loadRelaxed() == 1; }

    QContiguousCache<T> &operator=(const QContiguousCache<T> &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QContiguousCache)
    void swap(QContiguousCache &other) noexcept { qt_ptr_swap(d, other.d); }

#ifndef Q_CLANG_QDOC
    template <typename U = T>
    QTypeTraits::compare_eq_result<U> operator==(const QContiguousCache<T> &other) const
    {
        if (other.d == d)
            return true;
        if (other.d->start != d->start
                || other.d->count != d->count
                || other.d->offset != d->offset
                || other.d->alloc != d->alloc)
            return false;
        for (qsizetype i = firstIndex(); i <= lastIndex(); ++i)
            if (!(at(i) == other.at(i)))
                return false;
        return true;
    }
    template <typename U = T>
    QTypeTraits::compare_eq_result<U> operator!=(const QContiguousCache<T> &other) const
    { return !(*this == other); }
#else
    bool operator==(const QContiguousCache &other) const;
    bool operator!=(const QContiguousCache &other) const;
#endif // Q_CLANG_QDOC

    inline qsizetype capacity() const {return d->alloc; }
    inline qsizetype count() const { return d->count; }
    inline qsizetype size() const { return d->count; }

    inline bool isEmpty() const { return d->count == 0; }
    inline bool isFull() const { return d->count == d->alloc; }
    inline qsizetype available() const { return d->alloc - d->count; }

    void clear();
    void setCapacity(qsizetype size);

    const T &at(qsizetype pos) const;
    T &operator[](qsizetype i);
    const T &operator[](qsizetype i) const;

    void append(T &&value);
    void append(const T &value);
    void prepend(T &&value);
    void prepend(const T &value);
    void insert(qsizetype pos, T &&value);
    void insert(qsizetype pos, const T &value);


    inline bool containsIndex(qsizetype pos) const { return pos >= d->offset && pos - d->offset < d->count; }
    inline qsizetype firstIndex() const { return d->offset; }
    inline qsizetype lastIndex() const { return d->offset + d->count - 1; }

    inline const T &first() const { Q_ASSERT(!isEmpty()); return d->array[d->start]; }
    inline const T &last() const { Q_ASSERT(!isEmpty()); return d->array[(d->start + d->count -1) % d->alloc]; }
    inline T &first() { Q_ASSERT(!isEmpty()); detach(); return d->array[d->start]; }
    inline T &last() { Q_ASSERT(!isEmpty()); detach(); return d->array[(d->start + d->count -1) % d->alloc]; }

    void removeFirst();
    T takeFirst();
    void removeLast();
    T takeLast();

    // Use extra parentheses around max to avoid expanding it if it is a macro.
    inline bool areIndexesValid() const
    { return d->offset >= 0 && d->offset < (std::numeric_limits<qsizetype>::max)() - d->count && (d->offset % d->alloc) == d->start; }

    inline void normalizeIndexes() { d->offset = d->start; }

#ifdef QT_QCONTIGUOUSCACHE_DEBUG
    void dump() const { d->dump(); }
#endif
private:
    void detach_helper();

    Data *allocateData(qsizetype aalloc);
    void freeData(Data *x);
};

template <typename T>
void QContiguousCache<T>::detach_helper()
{
    Data *x = allocateData(d->alloc);
    x->ref.storeRelaxed(1);
    x->count = d->count;
    x->start = d->start;
    x->offset = d->offset;
    x->alloc = d->alloc;

    T *dest = x->array + x->start;
    T *src = d->array + d->start;
    qsizetype oldcount = x->count;
    while (oldcount--) {
        new (dest) T(*src);
        dest++;
        if (dest == x->array + x->alloc)
            dest = x->array;
        src++;
        if (src == d->array + d->alloc)
            src = d->array;
    }

    if (!d->ref.deref())
        freeData(d);
    d = x;
}

template <typename T>
void QContiguousCache<T>::setCapacity(qsizetype asize)
{
    Q_ASSERT(asize >= 0);
    if (asize == d->alloc)
        return;
    detach();
    Data *x = allocateData(asize);
    x->ref.storeRelaxed(1);
    x->alloc = asize;
    x->count = qMin(d->count, asize);
    x->offset = d->offset + d->count - x->count;
    if (asize)
        x->start = x->offset % x->alloc;
    else
        x->start = 0;

    qsizetype oldcount = x->count;
    if (oldcount)
    {
        T *dest = x->array + (x->start + x->count-1) % x->alloc;
        T *src = d->array + (d->start + d->count-1) % d->alloc;
        while (oldcount--) {
            new (dest) T(*src);
            if (dest == x->array)
                dest = x->array + x->alloc;
            dest--;
            if (src == d->array)
                src = d->array + d->alloc;
            src--;
        }
    }
    /* free old */
    freeData(d);
    d = x;
}

template <typename T>
void QContiguousCache<T>::clear()
{
    if (d->ref.loadRelaxed() == 1) {
        if (QTypeInfo<T>::isComplex) {
            qsizetype oldcount = d->count;
            T * i = d->array + d->start;
            T * e = d->array + d->alloc;
            while (oldcount--) {
                i->~T();
                i++;
                if (i == e)
                    i = d->array;
            }
        }
        d->count = d->start = d->offset = 0;
    } else {
        Data *x = allocateData(d->alloc);
        x->ref.storeRelaxed(1);
        x->alloc = d->alloc;
        x->count = x->start = x->offset = 0;
        if (!d->ref.deref())
            freeData(d);
        d = x;
    }
}

template <typename T>
inline typename QContiguousCache<T>::Data *QContiguousCache<T>::allocateData(qsizetype aalloc)
{
    return static_cast<Data *>(QContiguousCacheData::allocateData(sizeof(Data) + (aalloc - 1) * sizeof(T), alignof(Data)));
}

template <typename T>
QContiguousCache<T>::QContiguousCache(qsizetype cap)
{
    Q_ASSERT(cap >= 0);
    d = allocateData(cap);
    d->ref.storeRelaxed(1);
    d->alloc = cap;
    d->count = d->start = d->offset = 0;
}

template <typename T>
QContiguousCache<T> &QContiguousCache<T>::operator=(const QContiguousCache<T> &other)
{
    other.d->ref.ref();
    if (!d->ref.deref())
        freeData(d);
    d = other.d;
    return *this;
}

template <typename T>
void QContiguousCache<T>::freeData(Data *x)
{
    if (QTypeInfo<T>::isComplex) {
        qsizetype oldcount = d->count;
        T * i = d->array + d->start;
        T * e = d->array + d->alloc;
        while (oldcount--) {
            i->~T();
            i++;
            if (i == e)
                i = d->array;
        }
    }
    Data::freeData(x);
}
template <typename T>
void QContiguousCache<T>::append(T &&value)
{
    if (!d->alloc)
        return;     // zero capacity
    detach();
    if (d->count == d->alloc)
        (d->array + (d->start+d->count) % d->alloc)->~T();
    new (d->array + (d->start+d->count) % d->alloc) T(std::move(value));

    if (d->count == d->alloc) {
        d->start++;
        d->start %= d->alloc;
        d->offset++;
    } else {
        d->count++;
    }
}

template <typename T>
void QContiguousCache<T>::append(const T &value)
{
    if (!d->alloc)
        return;     // zero capacity
    detach();
    if (d->count == d->alloc)
        (d->array + (d->start+d->count) % d->alloc)->~T();
    new (d->array + (d->start+d->count) % d->alloc) T(value);

    if (d->count == d->alloc) {
        d->start++;
        d->start %= d->alloc;
        d->offset++;
    } else {
        d->count++;
    }
}

template<typename T>
void QContiguousCache<T>::prepend(T &&value)
{
    if (!d->alloc)
        return;     // zero capacity
    detach();
    if (d->start)
        d->start--;
    else
        d->start = d->alloc-1;
    d->offset--;

    if (d->count != d->alloc)
        d->count++;
    else
        (d->array + d->start)->~T();

    new (d->array + d->start) T(std::move(value));
}

template<typename T>
void QContiguousCache<T>::prepend(const T &value)
{
    if (!d->alloc)
        return;     // zero capacity
    detach();
    if (d->start)
        d->start--;
    else
        d->start = d->alloc-1;
    d->offset--;

    if (d->count != d->alloc)
        d->count++;
    else
        (d->array + d->start)->~T();

    new (d->array + d->start) T(value);
}

template<typename T>
void QContiguousCache<T>::insert(qsizetype pos, T &&value)
{
    Q_ASSERT_X(pos >= 0, "QContiguousCache<T>::insert", "index out of range");
    if (!d->alloc)
        return;     // zero capacity
    detach();
    if (containsIndex(pos)) {
        d->array[pos % d->alloc] = std::move(value);
    } else if (pos == d->offset-1)
        prepend(value);
    else if (pos == d->offset+d->count)
        append(value);
    else {
        // we don't leave gaps.
        clear();
        d->offset = pos;
        d->start = pos % d->alloc;
        d->count = 1;
        new (d->array + d->start) T(std::move(value));
    }
}

template<typename T>
void QContiguousCache<T>::insert(qsizetype pos, const T &value)
{
    return insert(pos, T(value));
}
template <typename T>
inline const T &QContiguousCache<T>::at(qsizetype pos) const
{ Q_ASSERT_X(pos >= d->offset && pos - d->offset < d->count, "QContiguousCache<T>::at", "index out of range"); return d->array[pos % d->alloc]; }
template <typename T>
inline const T &QContiguousCache<T>::operator[](qsizetype pos) const
{ return at(pos); }

template <typename T>
inline T &QContiguousCache<T>::operator[](qsizetype pos)
{
    detach();
    if (!containsIndex(pos))
        insert(pos, T());
    return d->array[pos % d->alloc];
}

template <typename T>
inline void QContiguousCache<T>::removeFirst()
{
    Q_ASSERT(d->count > 0);
    detach();
    d->count--;
    if (QTypeInfo<T>::isComplex)
        (d->array + d->start)->~T();
    d->start = (d->start + 1) % d->alloc;
    d->offset++;
}

template <typename T>
inline void QContiguousCache<T>::removeLast()
{
    Q_ASSERT(d->count > 0);
    detach();
    d->count--;
    if (QTypeInfo<T>::isComplex)
        (d->array + (d->start + d->count) % d->alloc)->~T();
}

template <typename T>
inline T QContiguousCache<T>::takeFirst()
{ T t = std::move(first()); removeFirst(); return t; }

template <typename T>
inline T QContiguousCache<T>::takeLast()
{ T t = std::move(last()); removeLast(); return t; }

QT_END_NAMESPACE

#endif
