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

#ifndef QVECTOR_H
#define QVECTOR_H

#include <QtCore/qalgorithms.h>
#include <QtCore/qiterator.h>
#include <QtCore/qrefcount.h>
#include <QtCore/qarraydata.h>
#include <QtCore/qhashfunctions.h>
#include <QtCore/qcontainertools_impl.h>

#include <iterator>
#include <initializer_list>
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
#include <vector>
#endif
#include <stdlib.h>
#include <string.h>

#include <algorithm>

QT_BEGIN_NAMESPACE

template <typename T>
class QVector
{
    typedef QTypedArrayData<T> Data;
    Data *d;

public:
    inline QVector() noexcept : d(Data::sharedNull()) { }
    explicit QVector(int size);
    QVector(int size, const T &t);
    inline QVector(const QVector<T> &v);
    inline ~QVector() { if (!d->ref.deref()) freeData(d); }
    QVector<T> &operator=(const QVector<T> &v);
    QVector(QVector<T> &&other) noexcept : d(other.d) { other.d = Data::sharedNull(); }
    QVector<T> &operator=(QVector<T> &&other) noexcept
    { QVector moved(std::move(other)); swap(moved); return *this; }
    void swap(QVector<T> &other) noexcept { qSwap(d, other.d); }
    inline QVector(std::initializer_list<T> args);
    QVector<T> &operator=(std::initializer_list<T> args);
    template <typename InputIterator, QtPrivate::IfIsInputIterator<InputIterator> = true>
    inline QVector(InputIterator first, InputIterator last);
    explicit QVector(QArrayDataPointerRef<T> ref) noexcept : d(ref.ptr) {}

    bool operator==(const QVector<T> &v) const;
    inline bool operator!=(const QVector<T> &v) const { return !(*this == v); }

    inline int size() const { return d->size; }

    inline bool isEmpty() const { return d->size == 0; }

    void resize(int size);

    inline int capacity() const { return int(d->alloc); }
    void reserve(int size);
    inline void squeeze()
    {
        if (d->size < int(d->alloc)) {
            if (!d->size) {
                *this = QVector<T>();
                return;
            }
            realloc(d->size);
        }
        if (d->capacityReserved) {
            // capacity reserved in a read only memory would be useless
            // this checks avoid writing to such memory.
            d->capacityReserved = 0;
        }
    }

    inline void detach();
    inline bool isDetached() const { return !d->ref.isShared(); }
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
    inline void setSharable(bool sharable)
    {
        if (sharable == d->ref.isSharable())
            return;
        if (!sharable)
            detach();

        if (d == Data::unsharableEmpty()) {
            if (sharable)
                d = Data::sharedNull();
        } else {
            d->ref.setSharable(sharable);
        }
        Q_ASSERT(d->ref.isSharable() == sharable);
    }
#endif

    inline bool isSharedWith(const QVector<T> &other) const { return d == other.d; }

    inline T *data() { detach(); return d->begin(); }
    inline const T *data() const { return d->begin(); }
    inline const T *constData() const { return d->begin(); }
    void clear();

    const T &at(int i) const;
    T &operator[](int i);
    const T &operator[](int i) const;
    void append(const T &t);
    void append(T &&t);
    inline void append(const QVector<T> &l) { *this += l; }
    void prepend(T &&t);
    void prepend(const T &t);
    void insert(int i, T &&t);
    void insert(int i, const T &t);
    void insert(int i, int n, const T &t);
    void replace(int i, const T &t);
    void remove(int i);
    void remove(int i, int n);
    inline void removeFirst() { Q_ASSERT(!isEmpty()); erase(d->begin()); }
    inline void removeLast();
    T takeFirst() { Q_ASSERT(!isEmpty()); T r = std::move(first()); removeFirst(); return r; }
    T takeLast()  { Q_ASSERT(!isEmpty()); T r = std::move(last()); removeLast(); return r; }

    QVector<T> &fill(const T &t, int size = -1);

    int indexOf(const T &t, int from = 0) const;
    int lastIndexOf(const T &t, int from = -1) const;
    bool contains(const T &t) const;
    int count(const T &t) const;

    // QList compatibility
    void removeAt(int i) { remove(i); }
    int removeAll(const T &t)
    {
        const const_iterator ce = this->cend(), cit = std::find(this->cbegin(), ce, t);
        if (cit == ce)
            return 0;
        // next operation detaches, so ce, cit, t may become invalidated:
        const T tCopy = t;
        const int firstFoundIdx = std::distance(this->cbegin(), cit);
        const iterator e = end(), it = std::remove(begin() + firstFoundIdx, e, tCopy);
        const int result = std::distance(it, e);
        erase(it, e);
        return result;
    }
    bool removeOne(const T &t)
    {
        const int i = indexOf(t);
        if (i < 0)
            return false;
        remove(i);
        return true;
    }
    int length() const { return size(); }
    T takeAt(int i) { T t = std::move((*this)[i]); remove(i); return t; }
    void move(int from, int to)
    {
        Q_ASSERT_X(from >= 0 && from < size(), "QVector::move(int,int)", "'from' is out-of-range");
        Q_ASSERT_X(to >= 0 && to < size(), "QVector::move(int,int)", "'to' is out-of-range");
        if (from == to) // don't detach when no-op
            return;
        detach();
        T * const b = d->begin();
        if (from < to)
            std::rotate(b + from, b + from + 1, b + to + 1);
        else
            std::rotate(b + to, b + from, b + from + 1);
    }

    // STL-style
    typedef typename Data::iterator iterator;
    typedef typename Data::const_iterator const_iterator;
    typedef std::reverse_iterator<iterator> reverse_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;
#if !defined(QT_STRICT_ITERATORS) || defined(Q_CLANG_QDOC)
    inline iterator begin() { detach(); return d->begin(); }
    inline const_iterator begin() const noexcept { return d->constBegin(); }
    inline const_iterator cbegin() const noexcept { return d->constBegin(); }
    inline const_iterator constBegin() const noexcept { return d->constBegin(); }
    inline iterator end() { detach(); return d->end(); }
    inline const_iterator end() const noexcept { return d->constEnd(); }
    inline const_iterator cend() const noexcept { return d->constEnd(); }
    inline const_iterator constEnd() const noexcept { return d->constEnd(); }
#else
    inline iterator begin(iterator = iterator()) { detach(); return d->begin(); }
    inline const_iterator begin(const_iterator = const_iterator()) const noexcept { return d->constBegin(); }
    inline const_iterator cbegin(const_iterator = const_iterator()) const noexcept { return d->constBegin(); }
    inline const_iterator constBegin(const_iterator = const_iterator()) const noexcept { return d->constBegin(); }
    inline iterator end(iterator = iterator()) { detach(); return d->end(); }
    inline const_iterator end(const_iterator = const_iterator()) const noexcept { return d->constEnd(); }
    inline const_iterator cend(const_iterator = const_iterator()) const noexcept { return d->constEnd(); }
    inline const_iterator constEnd(const_iterator = const_iterator()) const noexcept { return d->constEnd(); }
#endif
    reverse_iterator rbegin() { return reverse_iterator(end()); }
    reverse_iterator rend() { return reverse_iterator(begin()); }
    const_reverse_iterator rbegin() const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator rend() const noexcept { return const_reverse_iterator(begin()); }
    const_reverse_iterator crbegin() const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator crend() const noexcept { return const_reverse_iterator(begin()); }
    iterator insert(iterator before, int n, const T &x);
    inline iterator insert(iterator before, const T &x) { return insert(before, 1, x); }
    inline iterator insert(iterator before, T &&x);
    iterator erase(iterator begin, iterator end);
    inline iterator erase(iterator pos) { return erase(pos, pos+1); }

    // more Qt
    inline int count() const { return d->size; }
    inline T& first() { Q_ASSERT(!isEmpty()); return *begin(); }
    inline const T &first() const { Q_ASSERT(!isEmpty()); return *begin(); }
    inline const T &constFirst() const { Q_ASSERT(!isEmpty()); return *begin(); }
    inline T& last() { Q_ASSERT(!isEmpty()); return *(end()-1); }
    inline const T &last() const { Q_ASSERT(!isEmpty()); return *(end()-1); }
    inline const T &constLast() const { Q_ASSERT(!isEmpty()); return *(end()-1); }
    inline bool startsWith(const T &t) const { return !isEmpty() && first() == t; }
    inline bool endsWith(const T &t) const { return !isEmpty() && last() == t; }
    QVector<T> mid(int pos, int len = -1) const;

    T value(int i) const;
    T value(int i, const T &defaultValue) const;

    void swapItemsAt(int i, int j) {
        Q_ASSERT_X(i >= 0 && i < size() && j >= 0 && j < size(),
                    "QVector<T>::swap", "index out of range");
        detach();
        qSwap(d->begin()[i], d->begin()[j]);
    }

    // STL compatibility
    typedef T value_type;
    typedef value_type* pointer;
    typedef const value_type* const_pointer;
    typedef value_type& reference;
    typedef const value_type& const_reference;
    typedef qptrdiff difference_type;
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
    typedef int size_type;
    inline void push_back(const T &t) { append(t); }
    void push_back(T &&t) { append(std::move(t)); }
    void push_front(T &&t) { prepend(std::move(t)); }
    inline void push_front(const T &t) { prepend(t); }
    void pop_back() { removeLast(); }
    void pop_front() { removeFirst(); }
    inline bool empty() const
    { return d->size == 0; }
    inline T& front() { return first(); }
    inline const_reference front() const { return first(); }
    inline reference back() { return last(); }
    inline const_reference back() const { return last(); }
    void shrink_to_fit() { squeeze(); }

    // comfort
    QVector<T> &operator+=(const QVector<T> &l);
    inline QVector<T> operator+(const QVector<T> &l) const
    { QVector n = *this; n += l; return n; }
    inline QVector<T> &operator+=(const T &t)
    { append(t); return *this; }
    inline QVector<T> &operator<< (const T &t)
    { append(t); return *this; }
    inline QVector<T> &operator<<(const QVector<T> &l)
    { *this += l; return *this; }
    inline QVector<T> &operator+=(T &&t)
    { append(std::move(t)); return *this; }
    inline QVector<T> &operator<<(T &&t)
    { append(std::move(t)); return *this; }

    static QVector<T> fromList(const QList<T> &list);
    QList<T> toList() const;

#if QT_DEPRECATED_SINCE(5, 14) && QT_VERSION < QT_VERSION_CHECK(6,0,0)
    QT_DEPRECATED_X("Use QVector<T>(vector.begin(), vector.end()) instead.")
    static inline QVector<T> fromStdVector(const std::vector<T> &vector)
    { return QVector<T>(vector.begin(), vector.end()); }
    QT_DEPRECATED_X("Use std::vector<T>(vector.begin(), vector.end()) instead.")
    inline std::vector<T> toStdVector() const
    { return std::vector<T>(d->begin(), d->end()); }
#endif
private:
    // ### Qt6: remove methods, they are unused
    void reallocData(const int size, const int alloc, QArrayData::AllocationOptions options = QArrayData::Default);
    void reallocData(const int sz) { reallocData(sz, d->alloc); }
    void realloc(int alloc, QArrayData::AllocationOptions options = QArrayData::Default);
    void freeData(Data *d);
    void defaultConstruct(T *from, T *to);
    void copyConstruct(const T *srcFrom, const T *srcTo, T *dstFrom);
    void destruct(T *from, T *to);
    bool isValidIterator(const iterator &i) const
    {
        const std::less<const T*> less = {};
        return !less(d->end(), i) && !less(i, d->begin());
    }
    class AlignmentDummy { Data header; T array[1]; };
};

#if defined(__cpp_deduction_guides) && __cpp_deduction_guides >= 201606
template <typename InputIterator,
          typename ValueType = typename std::iterator_traits<InputIterator>::value_type,
          QtPrivate::IfIsInputIterator<InputIterator> = true>
QVector(InputIterator, InputIterator) -> QVector<ValueType>;
#endif

#ifdef Q_CC_MSVC
// behavior change: an object of POD type constructed with an initializer of the form ()
// will be default-initialized
#   pragma warning ( push )
#   pragma warning(disable : 4127) // conditional expression is constant
#endif

template <typename T>
void QVector<T>::defaultConstruct(T *from, T *to)
{
    if (QTypeInfo<T>::isComplex) {
        while (from != to) {
            new (from++) T();
        }
    } else {
        ::memset(static_cast<void *>(from), 0, (to - from) * sizeof(T));
    }
}

template <typename T>
void QVector<T>::copyConstruct(const T *srcFrom, const T *srcTo, T *dstFrom)
{
    if (QTypeInfo<T>::isComplex) {
        while (srcFrom != srcTo)
            new (dstFrom++) T(*srcFrom++);
    } else {
        ::memcpy(static_cast<void *>(dstFrom), static_cast<const void *>(srcFrom), (srcTo - srcFrom) * sizeof(T));
    }
}

template <typename T>
void QVector<T>::destruct(T *from, T *to)
{
    if (QTypeInfo<T>::isComplex) {
        while (from != to) {
            from++->~T();
        }
    }
}

template <typename T>
inline QVector<T>::QVector(const QVector<T> &v)
{
    if (v.d->ref.ref()) {
        d = v.d;
    } else {
        if (v.d->capacityReserved) {
            d = Data::allocate(v.d->alloc);
            Q_CHECK_PTR(d);
            d->capacityReserved = true;
        } else {
            d = Data::allocate(v.d->size);
            Q_CHECK_PTR(d);
        }
        if (d->alloc) {
            copyConstruct(v.d->begin(), v.d->end(), d->begin());
            d->size = v.d->size;
        }
    }
}

#if defined(Q_CC_MSVC)
#pragma warning( pop )
#endif

template <typename T>
void QVector<T>::detach()
{
    if (!isDetached()) {
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
        if (!d->alloc)
            d = Data::unsharableEmpty();
        else
#endif
            realloc(int(d->alloc));
    }
    Q_ASSERT(isDetached());
}

template <typename T>
void QVector<T>::reserve(int asize)
{
    if (asize > int(d->alloc))
        realloc(asize);
    if (isDetached()
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
            && d != Data::unsharableEmpty()
#endif
            )
        d->capacityReserved = 1;
    Q_ASSERT(capacity() >= asize);
}

template <typename T>
void QVector<T>::resize(int asize)
{
    if (asize == d->size)
        return detach();
    if (asize > int(d->alloc) || !isDetached()) { // there is not enough space
        QArrayData::AllocationOptions opt = asize > int(d->alloc) ? QArrayData::Grow : QArrayData::Default;
        realloc(qMax(int(d->alloc), asize), opt);
    }
    if (asize < d->size)
        destruct(begin() + asize, end());
    else
        defaultConstruct(end(), begin() + asize);
    d->size = asize;
}
template <typename T>
inline void QVector<T>::clear()
{
    if (!d->size)
        return;
    destruct(begin(), end());
    d->size = 0;
}
template <typename T>
inline const T &QVector<T>::at(int i) const
{ Q_ASSERT_X(i >= 0 && i < d->size, "QVector<T>::at", "index out of range");
  return d->begin()[i]; }
template <typename T>
inline const T &QVector<T>::operator[](int i) const
{ Q_ASSERT_X(i >= 0 && i < d->size, "QVector<T>::operator[]", "index out of range");
  return d->begin()[i]; }
template <typename T>
inline T &QVector<T>::operator[](int i)
{ Q_ASSERT_X(i >= 0 && i < d->size, "QVector<T>::operator[]", "index out of range");
  return data()[i]; }
template <typename T>
inline void QVector<T>::insert(int i, const T &t)
{ Q_ASSERT_X(i >= 0 && i <= d->size, "QVector<T>::insert", "index out of range");
  insert(begin() + i, 1, t); }
template <typename T>
inline void QVector<T>::insert(int i, int n, const T &t)
{ Q_ASSERT_X(i >= 0 && i <= d->size, "QVector<T>::insert", "index out of range");
  insert(begin() + i, n, t); }
template <typename T>
inline void QVector<T>::insert(int i, T &&t)
{ Q_ASSERT_X(i >= 0 && i <= d->size, "QVector<T>::insert", "index out of range");
  insert(begin() + i, std::move(t)); }
template <typename T>
inline void QVector<T>::remove(int i, int n)
{ Q_ASSERT_X(i >= 0 && n >= 0 && i + n <= d->size, "QVector<T>::remove", "index out of range");
  erase(d->begin() + i, d->begin() + i + n); }
template <typename T>
inline void QVector<T>::remove(int i)
{ Q_ASSERT_X(i >= 0 && i < d->size, "QVector<T>::remove", "index out of range");
  erase(d->begin() + i, d->begin() + i + 1); }
template <typename T>
inline void QVector<T>::prepend(const T &t)
{ insert(begin(), 1, t); }
template <typename T>
inline void QVector<T>::prepend(T &&t)
{ insert(begin(), std::move(t)); }

template <typename T>
inline void QVector<T>::replace(int i, const T &t)
{
    Q_ASSERT_X(i >= 0 && i < d->size, "QVector<T>::replace", "index out of range");
    const T copy(t);
    data()[i] = copy;
}

template <typename T>
QVector<T> &QVector<T>::operator=(const QVector<T> &v)
{
    if (v.d != d) {
        QVector<T> tmp(v);
        tmp.swap(*this);
    }
    return *this;
}

template <typename T>
QVector<T>::QVector(int asize)
{
    Q_ASSERT_X(asize >= 0, "QVector::QVector", "Size must be greater than or equal to 0.");
    if (Q_LIKELY(asize > 0)) {
        d = Data::allocate(asize);
        Q_CHECK_PTR(d);
        d->size = asize;
        defaultConstruct(d->begin(), d->end());
    } else {
        d = Data::sharedNull();
    }
}

template <typename T>
QVector<T>::QVector(int asize, const T &t)
{
    Q_ASSERT_X(asize >= 0, "QVector::QVector", "Size must be greater than or equal to 0.");
    if (asize > 0) {
        d = Data::allocate(asize);
        Q_CHECK_PTR(d);
        d->size = asize;
        T* i = d->end();
        while (i != d->begin())
            new (--i) T(t);
    } else {
        d = Data::sharedNull();
    }
}

#if defined(Q_CC_MSVC)
QT_WARNING_PUSH
QT_WARNING_DISABLE_MSVC(4127) // conditional expression is constant
#endif // Q_CC_MSVC

template <typename T>
QVector<T>::QVector(std::initializer_list<T> args)
{
    if (args.size() > 0) {
        d = Data::allocate(args.size());
        Q_CHECK_PTR(d);
        // std::initializer_list<T>::iterator is guaranteed to be
        // const T* ([support.initlist]/1), so can be memcpy'ed away from by copyConstruct
        copyConstruct(args.begin(), args.end(), d->begin());
        d->size = int(args.size());
    } else {
        d = Data::sharedNull();
    }
}

template <typename T>
QVector<T> &QVector<T>::operator=(std::initializer_list<T> args)
{
    QVector<T> tmp(args);
    tmp.swap(*this);
    return *this;
}

#if defined(Q_CC_MSVC)
QT_WARNING_POP
#endif // Q_CC_MSVC

template <typename T>
template <typename InputIterator, QtPrivate::IfIsInputIterator<InputIterator>>
QVector<T>::QVector(InputIterator first, InputIterator last)
    : QVector()
{
    QtPrivate::reserveIfForwardIterator(this, first, last);
    std::copy(first, last, std::back_inserter(*this));
}

template <typename T>
void QVector<T>::freeData(Data *x)
{
    destruct(x->begin(), x->end());
    Data::deallocate(x);
}

#if defined(Q_CC_MSVC)
QT_WARNING_PUSH
QT_WARNING_DISABLE_MSVC(4127) // conditional expression is constant
#endif

template <typename T>
void QVector<T>::reallocData(const int asize, const int aalloc, QArrayData::AllocationOptions options)
{
    Q_ASSERT(asize >= 0 && asize <= aalloc);
    Data *x = d;

    const bool isShared = d->ref.isShared();

    if (aalloc != 0) {
        if (aalloc != int(d->alloc) || isShared) {
            QT_TRY {
                // allocate memory
                x = Data::allocate(aalloc, options);
                Q_CHECK_PTR(x);
                // aalloc is bigger then 0 so it is not [un]sharedEmpty
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
                Q_ASSERT(x->ref.isSharable() || options.testFlag(QArrayData::Unsharable));
#endif
                Q_ASSERT(!x->ref.isStatic());
                x->size = asize;

                T *srcBegin = d->begin();
                T *srcEnd = asize > d->size ? d->end() : d->begin() + asize;
                T *dst = x->begin();

                if (!QTypeInfoQuery<T>::isRelocatable || (isShared && QTypeInfo<T>::isComplex)) {
                    QT_TRY {
                        if (isShared || !std::is_nothrow_move_constructible<T>::value) {
                            // we can not move the data, we need to copy construct it
                            while (srcBegin != srcEnd)
                                new (dst++) T(*srcBegin++);
                        } else {
                            while (srcBegin != srcEnd)
                                new (dst++) T(std::move(*srcBegin++));
                        }
                    } QT_CATCH (...) {
                        // destruct already copied objects
                        destruct(x->begin(), dst);
                        QT_RETHROW;
                    }
                } else {
                    ::memcpy(static_cast<void *>(dst), static_cast<void *>(srcBegin), (srcEnd - srcBegin) * sizeof(T));
                    dst += srcEnd - srcBegin;

                    // destruct unused / not moved data
                    if (asize < d->size)
                        destruct(d->begin() + asize, d->end());
                }

                if (asize > d->size) {
                    // construct all new objects when growing
                    if (!QTypeInfo<T>::isComplex) {
                        ::memset(static_cast<void *>(dst), 0, (static_cast<T *>(x->end()) - dst) * sizeof(T));
                    } else {
                        QT_TRY {
                            while (dst != x->end())
                                new (dst++) T();
                        } QT_CATCH (...) {
                            // destruct already copied objects
                            destruct(x->begin(), dst);
                            QT_RETHROW;
                        }
                    }
                }
            } QT_CATCH (...) {
                Data::deallocate(x);
                QT_RETHROW;
            }
            x->capacityReserved = d->capacityReserved;
        } else {
            Q_ASSERT(int(d->alloc) == aalloc); // resize, without changing allocation size
            Q_ASSERT(isDetached());       // can be done only on detached d
            Q_ASSERT(x == d);             // in this case we do not need to allocate anything
            if (asize <= d->size) {
                destruct(x->begin() + asize, x->end()); // from future end to current end
            } else {
                defaultConstruct(x->end(), x->begin() + asize); // from current end to future end
            }
            x->size = asize;
        }
    } else {
        x = Data::sharedNull();
    }
    if (d != x) {
        if (!d->ref.deref()) {
            if (!QTypeInfoQuery<T>::isRelocatable || !aalloc || (isShared && QTypeInfo<T>::isComplex)) {
                // data was copy constructed, we need to call destructors
                // or if !alloc we did nothing to the old 'd'.
                freeData(d);
            } else {
                Data::deallocate(d);
            }
        }
        d = x;
    }

    Q_ASSERT(d->data());
    Q_ASSERT(uint(d->size) <= d->alloc);
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
    Q_ASSERT(d != Data::unsharableEmpty());
#endif
    Q_ASSERT(aalloc ? d != Data::sharedNull() : d == Data::sharedNull());
    Q_ASSERT(d->alloc >= uint(aalloc));
    Q_ASSERT(d->size == asize);
}

template<typename T>
void QVector<T>::realloc(int aalloc, QArrayData::AllocationOptions options)
{
    Q_ASSERT(aalloc >= d->size);
    Data *x = d;

    const bool isShared = d->ref.isShared();

    QT_TRY {
        // allocate memory
        x = Data::allocate(aalloc, options);
        Q_CHECK_PTR(x);
        // aalloc is bigger then 0 so it is not [un]sharedEmpty
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
        Q_ASSERT(x->ref.isSharable() || options.testFlag(QArrayData::Unsharable));
#endif
        Q_ASSERT(!x->ref.isStatic());
        x->size = d->size;

        T *srcBegin = d->begin();
        T *srcEnd = d->end();
        T *dst = x->begin();

        if (!QTypeInfoQuery<T>::isRelocatable || (isShared && QTypeInfo<T>::isComplex)) {
            QT_TRY {
                if (isShared || !std::is_nothrow_move_constructible<T>::value) {
                    // we can not move the data, we need to copy construct it
                    while (srcBegin != srcEnd)
                        new (dst++) T(*srcBegin++);
                } else {
                    while (srcBegin != srcEnd)
                        new (dst++) T(std::move(*srcBegin++));
                }
            } QT_CATCH (...) {
                // destruct already copied objects
                destruct(x->begin(), dst);
                QT_RETHROW;
            }
        } else {
            ::memcpy(static_cast<void *>(dst), static_cast<void *>(srcBegin), (srcEnd - srcBegin) * sizeof(T));
            dst += srcEnd - srcBegin;
        }

    } QT_CATCH (...) {
        Data::deallocate(x);
        QT_RETHROW;
    }
    x->capacityReserved = d->capacityReserved;

    Q_ASSERT(d != x);
    if (!d->ref.deref()) {
        if (!QTypeInfoQuery<T>::isRelocatable || !aalloc || (isShared && QTypeInfo<T>::isComplex)) {
            // data was copy constructed, we need to call destructors
            // or if !alloc we did nothing to the old 'd'.
            freeData(d);
        } else {
            Data::deallocate(d);
        }
    }
    d = x;

    Q_ASSERT(d->data());
    Q_ASSERT(uint(d->size) <= d->alloc);
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
    Q_ASSERT(d != Data::unsharableEmpty());
#endif
    Q_ASSERT(d != Data::sharedNull());
    Q_ASSERT(d->alloc >= uint(aalloc));
}

#if defined(Q_CC_MSVC)
QT_WARNING_POP
#endif

template<typename T>
Q_OUTOFLINE_TEMPLATE T QVector<T>::value(int i) const
{
    if (uint(i) >= uint(d->size)) {
        return T();
    }
    return d->begin()[i];
}
template<typename T>
Q_OUTOFLINE_TEMPLATE T QVector<T>::value(int i, const T &defaultValue) const
{
    return uint(i) >= uint(d->size) ? defaultValue : d->begin()[i];
}

template <typename T>
void QVector<T>::append(const T &t)
{
    const bool isTooSmall = uint(d->size + 1) > d->alloc;
    if (!isDetached() || isTooSmall) {
        T copy(t);
        QArrayData::AllocationOptions opt(isTooSmall ? QArrayData::Grow : QArrayData::Default);
        realloc(isTooSmall ? d->size + 1 : d->alloc, opt);

        if (QTypeInfo<T>::isComplex)
            new (d->end()) T(std::move(copy));
        else
            *d->end() = std::move(copy);

    } else {
        if (QTypeInfo<T>::isComplex)
            new (d->end()) T(t);
        else
            *d->end() = t;
    }
    ++d->size;
}

template <typename T>
void QVector<T>::append(T &&t)
{
    const bool isTooSmall = uint(d->size + 1) > d->alloc;
    if (!isDetached() || isTooSmall) {
        QArrayData::AllocationOptions opt(isTooSmall ? QArrayData::Grow : QArrayData::Default);
        realloc(isTooSmall ? d->size + 1 : d->alloc, opt);
    }

    new (d->end()) T(std::move(t));

    ++d->size;
}

template <typename T>
void QVector<T>::removeLast()
{
    Q_ASSERT(!isEmpty());
    Q_ASSERT(d->alloc);

    if (d->ref.isShared())
        detach();
    --d->size;
    if (QTypeInfo<T>::isComplex)
        (d->data() + d->size)->~T();
}

template <typename T>
typename QVector<T>::iterator QVector<T>::insert(iterator before, size_type n, const T &t)
{
    Q_ASSERT_X(isValidIterator(before),  "QVector::insert", "The specified iterator argument 'before' is invalid");

    const auto offset = std::distance(d->begin(), before);
    if (n != 0) {
        const T copy(t);
        if (!isDetached() || d->size + n > int(d->alloc))
            realloc(d->size + n, QArrayData::Grow);
        if (!QTypeInfoQuery<T>::isRelocatable) {
            T *b = d->end();
            T *i = d->end() + n;
            while (i != b)
                new (--i) T;
            i = d->end();
            T *j = i + n;
            b = d->begin() + offset;
            while (i != b)
                *--j = *--i;
            i = b+n;
            while (i != b)
                *--i = copy;
        } else {
            T *b = d->begin() + offset;
            T *i = b + n;
            memmove(static_cast<void *>(i), static_cast<const void *>(b), (d->size - offset) * sizeof(T));
            while (i != b)
                new (--i) T(copy);
        }
        d->size += n;
    }
    return d->begin() + offset;
}

template <typename T>
typename QVector<T>::iterator QVector<T>::insert(iterator before, T &&t)
{
    Q_ASSERT_X(isValidIterator(before),  "QVector::insert", "The specified iterator argument 'before' is invalid");

    const auto offset = std::distance(d->begin(), before);
    if (!isDetached() || d->size + 1 > int(d->alloc))
        realloc(d->size + 1, QArrayData::Grow);
    if (!QTypeInfoQuery<T>::isRelocatable) {
        T *i = d->end();
        T *j = i + 1;
        T *b = d->begin() + offset;
        // The new end-element needs to be constructed, the rest must be move assigned
        if (i != b) {
            new (--j) T(std::move(*--i));
            while (i != b)
                *--j = std::move(*--i);
            *b = std::move(t);
        } else {
            new (b) T(std::move(t));
        }
    } else {
        T *b = d->begin() + offset;
        memmove(static_cast<void *>(b + 1), static_cast<const void *>(b), (d->size - offset) * sizeof(T));
        new (b) T(std::move(t));
    }
    d->size += 1;
    return d->begin() + offset;
}

template <typename T>
typename QVector<T>::iterator QVector<T>::erase(iterator abegin, iterator aend)
{
    Q_ASSERT_X(isValidIterator(abegin), "QVector::erase", "The specified iterator argument 'abegin' is invalid");
    Q_ASSERT_X(isValidIterator(aend), "QVector::erase", "The specified iterator argument 'aend' is invalid");

    const auto itemsToErase = aend - abegin;

    if (!itemsToErase)
        return abegin;

    Q_ASSERT(abegin >= d->begin());
    Q_ASSERT(aend <= d->end());
    Q_ASSERT(abegin <= aend);

    const auto itemsUntouched = abegin - d->begin();

    // FIXME we could do a proper realloc, which copy constructs only needed data.
    // FIXME we are about to delete data - maybe it is good time to shrink?
    // FIXME the shrink is also an issue in removeLast, that is just a copy + reduce of this.
    if (d->alloc) {
        detach();
        abegin = d->begin() + itemsUntouched;
        aend = abegin + itemsToErase;
        if (!QTypeInfoQuery<T>::isRelocatable) {
            iterator moveBegin = abegin + itemsToErase;
            iterator moveEnd = d->end();
            while (moveBegin != moveEnd) {
                if (QTypeInfo<T>::isComplex)
                    static_cast<T *>(abegin)->~T();
                new (abegin++) T(*moveBegin++);
            }
            if (abegin < d->end()) {
                // destroy rest of instances
                destruct(abegin, d->end());
            }
        } else {
            destruct(abegin, aend);
            // QTBUG-53605: static_cast<void *> masks clang errors of the form
            // error: destination for this 'memmove' call is a pointer to class containing a dynamic class
            // FIXME maybe use std::is_polymorphic (as soon as allowed) to avoid the memmove
            memmove(static_cast<void *>(abegin), static_cast<void *>(aend),
                    (d->size - itemsToErase - itemsUntouched) * sizeof(T));
        }
        d->size -= int(itemsToErase);
    }
    return d->begin() + itemsUntouched;
}

template <typename T>
bool QVector<T>::operator==(const QVector<T> &v) const
{
    if (d == v.d)
        return true;
    if (d->size != v.d->size)
        return false;
    const T *vb = v.d->begin();
    const T *b  = d->begin();
    const T *e  = d->end();
    return std::equal(b, e, QT_MAKE_CHECKED_ARRAY_ITERATOR(vb, v.d->size));
}

template <typename T>
QVector<T> &QVector<T>::fill(const T &from, int asize)
{
    const T copy(from);
    resize(asize < 0 ? d->size : asize);
    if (d->size) {
        T *i = d->end();
        T *b = d->begin();
        while (i != b)
            *--i = copy;
    }
    return *this;
}

template <typename T>
QVector<T> &QVector<T>::operator+=(const QVector &l)
{
    if (d->size == 0) {
        *this = l;
    } else {
        uint newSize = d->size + l.d->size;
        const bool isTooSmall = newSize > d->alloc;
        if (!isDetached() || isTooSmall) {
            QArrayData::AllocationOptions opt(isTooSmall ? QArrayData::Grow : QArrayData::Default);
            realloc(isTooSmall ? newSize : d->alloc, opt);
        }

        if (d->alloc) {
            T *w = d->begin() + newSize;
            T *i = l.d->end();
            T *b = l.d->begin();
            while (i != b) {
                if (QTypeInfo<T>::isComplex)
                    new (--w) T(*--i);
                else
                    *--w = *--i;
            }
            d->size = newSize;
        }
    }
    return *this;
}

template <typename T>
int QVector<T>::indexOf(const T &t, int from) const
{
    if (from < 0)
        from = qMax(from + d->size, 0);
    if (from < d->size) {
        T* n = d->begin() + from - 1;
        T* e = d->end();
        while (++n != e)
            if (*n == t)
                return n - d->begin();
    }
    return -1;
}

template <typename T>
int QVector<T>::lastIndexOf(const T &t, int from) const
{
    if (from < 0)
        from += d->size;
    else if (from >= d->size)
        from = d->size-1;
    if (from >= 0) {
        T* b = d->begin();
        T* n = d->begin() + from + 1;
        while (n != b) {
            if (*--n == t)
                return n - b;
        }
    }
    return -1;
}

template <typename T>
bool QVector<T>::contains(const T &t) const
{
    const T *b = d->begin();
    const T *e = d->end();
    return std::find(b, e, t) != e;
}

template <typename T>
int QVector<T>::count(const T &t) const
{
    const T *b = d->begin();
    const T *e = d->end();
    return int(std::count(b, e, t));
}

template <typename T>
Q_OUTOFLINE_TEMPLATE QVector<T> QVector<T>::mid(int pos, int len) const
{
    using namespace QtPrivate;
    switch (QContainerImplHelper::mid(d->size, &pos, &len)) {
    case QContainerImplHelper::Null:
    case QContainerImplHelper::Empty:
        return QVector<T>();
    case QContainerImplHelper::Full:
        return *this;
    case QContainerImplHelper::Subset:
        break;
    }

    QVector<T> midResult;
    midResult.realloc(len);
    T *srcFrom = d->begin() + pos;
    T *srcTo = d->begin() + pos + len;
    midResult.copyConstruct(srcFrom, srcTo, midResult.data());
    midResult.d->size = len;
    return midResult;
}

Q_DECLARE_SEQUENTIAL_ITERATOR(Vector)
Q_DECLARE_MUTABLE_SEQUENTIAL_ITERATOR(Vector)

template <typename T>
uint qHash(const QVector<T> &key, uint seed = 0)
    noexcept(noexcept(qHashRange(key.cbegin(), key.cend(), seed)))
{
    return qHashRange(key.cbegin(), key.cend(), seed);
}

template <typename T>
bool operator<(const QVector<T> &lhs, const QVector<T> &rhs)
    noexcept(noexcept(std::lexicographical_compare(lhs.begin(), lhs.end(),
                                                               rhs.begin(), rhs.end())))
{
    return std::lexicographical_compare(lhs.begin(), lhs.end(),
                                        rhs.begin(), rhs.end());
}

template <typename T>
inline bool operator>(const QVector<T> &lhs, const QVector<T> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return rhs < lhs;
}

template <typename T>
inline bool operator<=(const QVector<T> &lhs, const QVector<T> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return !(lhs > rhs);
}

template <typename T>
inline bool operator>=(const QVector<T> &lhs, const QVector<T> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return !(lhs < rhs);
}

/*
   ### Qt 5:
   ### This needs to be removed for next releases of Qt. It is a workaround for vc++ because
   ### Qt exports QPolygon and QPolygonF that inherit QVector<QPoint> and
   ### QVector<QPointF> respectively.
*/

#if defined(Q_CC_MSVC) && !defined(QT_BUILD_CORE_LIB)
QT_BEGIN_INCLUDE_NAMESPACE
#include <QtCore/qpoint.h>
QT_END_INCLUDE_NAMESPACE
extern template class Q_CORE_EXPORT QVector<QPointF>;
extern template class Q_CORE_EXPORT QVector<QPoint>;
#endif

QVector<uint> QStringView::toUcs4() const { return QtPrivate::convertToUcs4(*this); }

QT_END_NAMESPACE

#endif // QVECTOR_H
