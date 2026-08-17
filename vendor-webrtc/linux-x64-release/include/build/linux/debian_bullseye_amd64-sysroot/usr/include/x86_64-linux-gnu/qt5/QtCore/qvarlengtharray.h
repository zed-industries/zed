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

#ifndef QVARLENGTHARRAY_H
#define QVARLENGTHARRAY_H

#include <QtCore/qcontainerfwd.h>
#include <QtCore/qglobal.h>
#include <QtCore/qalgorithms.h>
#include <QtCore/qcontainertools_impl.h>
#include <QtCore/qhashfunctions.h>

#include <algorithm>
#include <initializer_list>
#include <iterator>
#include <new>
#include <string.h>
#include <stdlib.h>

QT_BEGIN_NAMESPACE


// Prealloc = 256 by default, specified in qcontainerfwd.h
template<class T, int Prealloc>
class QVarLengthArray
{
public:
    QVarLengthArray() : QVarLengthArray(0) {}

    inline explicit QVarLengthArray(int size);

    inline QVarLengthArray(const QVarLengthArray<T, Prealloc> &other)
        : a(Prealloc), s(0), ptr(reinterpret_cast<T *>(array))
    {
        append(other.constData(), other.size());
    }

    QVarLengthArray(std::initializer_list<T> args)
        : QVarLengthArray(args.begin(), args.end())
    {
    }

    template <typename InputIterator, QtPrivate::IfIsInputIterator<InputIterator> = true>
    inline QVarLengthArray(InputIterator first, InputIterator last)
        : QVarLengthArray()
    {
        QtPrivate::reserveIfForwardIterator(this, first, last);
        std::copy(first, last, std::back_inserter(*this));
    }

    inline ~QVarLengthArray() {
        if (QTypeInfo<T>::isComplex) {
            T *i = ptr + s;
            while (i-- != ptr)
                i->~T();
        }
        if (ptr != reinterpret_cast<T *>(array))
            free(ptr);
    }
    inline QVarLengthArray<T, Prealloc> &operator=(const QVarLengthArray<T, Prealloc> &other)
    {
        if (this != &other) {
            clear();
            append(other.constData(), other.size());
        }
        return *this;
    }

    QVarLengthArray<T, Prealloc> &operator=(std::initializer_list<T> list)
    {
        resize(int(list.size())); // ### q6sizetype
        std::copy(list.begin(), list.end(),
                  QT_MAKE_CHECKED_ARRAY_ITERATOR(this->begin(), this->size()));
        return *this;
    }

    inline void removeLast() {
        Q_ASSERT(s > 0);
        if (QTypeInfo<T>::isComplex)
            ptr[s - 1].~T();
        --s;
    }
    inline int size() const { return s; }
    inline int count() const { return s; }
    inline int length() const { return s; }
    inline T& first() { Q_ASSERT(!isEmpty()); return *begin(); }
    inline const T& first() const { Q_ASSERT(!isEmpty()); return *begin(); }
    T& last() { Q_ASSERT(!isEmpty()); return *(end() - 1); }
    const T& last() const { Q_ASSERT(!isEmpty()); return *(end() - 1); }
    inline bool isEmpty() const { return (s == 0); }
    inline void resize(int size);
    inline void clear() { resize(0); }
    inline void squeeze();

    inline int capacity() const { return a; }
    inline void reserve(int size);

    inline int indexOf(const T &t, int from = 0) const;
    inline int lastIndexOf(const T &t, int from = -1) const;
    inline bool contains(const T &t) const;

    inline T &operator[](int idx) {
        Q_ASSERT(idx >= 0 && idx < s);
        return ptr[idx];
    }
    inline const T &operator[](int idx) const {
        Q_ASSERT(idx >= 0 && idx < s);
        return ptr[idx];
    }
    inline const T &at(int idx) const { return operator[](idx); }

    T value(int i) const;
    T value(int i, const T &defaultValue) const;

    inline void append(const T &t) {
        if (s == a) {   // i.e. s != 0
            T copy(t);
            realloc(s, s<<1);
            const int idx = s++;
            if (QTypeInfo<T>::isComplex) {
                new (ptr + idx) T(std::move(copy));
            } else {
                ptr[idx] = std::move(copy);
            }
        } else {
            const int idx = s++;
            if (QTypeInfo<T>::isComplex) {
                new (ptr + idx) T(t);
            } else {
                ptr[idx] = t;
            }
        }
    }

    void append(T &&t) {
        if (s == a)
            realloc(s, s << 1);
        const int idx = s++;
        if (QTypeInfo<T>::isComplex)
            new (ptr + idx) T(std::move(t));
        else
            ptr[idx] = std::move(t);
    }

    void append(const T *buf, int size);
    inline QVarLengthArray<T, Prealloc> &operator<<(const T &t)
    { append(t); return *this; }
    inline QVarLengthArray<T, Prealloc> &operator<<(T &&t)
    { append(std::move(t)); return *this; }
    inline QVarLengthArray<T, Prealloc> &operator+=(const T &t)
    { append(t); return *this; }
    inline QVarLengthArray<T, Prealloc> &operator+=(T &&t)
    { append(std::move(t)); return *this; }

    void prepend(T &&t);
    void prepend(const T &t);
    void insert(int i, T &&t);
    void insert(int i, const T &t);
    void insert(int i, int n, const T &t);
    void replace(int i, const T &t);
    void remove(int i);
    void remove(int i, int n);


    inline T *data() { return ptr; }
    inline const T *data() const { return ptr; }
    inline const T * constData() const { return ptr; }
    typedef int size_type;
    typedef T value_type;
    typedef value_type *pointer;
    typedef const value_type *const_pointer;
    typedef value_type &reference;
    typedef const value_type &const_reference;
    typedef qptrdiff difference_type;


    typedef T* iterator;
    typedef const T* const_iterator;
    typedef std::reverse_iterator<iterator> reverse_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

    inline iterator begin() { return ptr; }
    inline const_iterator begin() const { return ptr; }
    inline const_iterator cbegin() const { return ptr; }
    inline const_iterator constBegin() const { return ptr; }
    inline iterator end() { return ptr + s; }
    inline const_iterator end() const { return ptr + s; }
    inline const_iterator cend() const { return ptr + s; }
    inline const_iterator constEnd() const { return ptr + s; }
    reverse_iterator rbegin() { return reverse_iterator(end()); }
    reverse_iterator rend() { return reverse_iterator(begin()); }
    const_reverse_iterator rbegin() const { return const_reverse_iterator(end()); }
    const_reverse_iterator rend() const { return const_reverse_iterator(begin()); }
    const_reverse_iterator crbegin() const { return const_reverse_iterator(end()); }
    const_reverse_iterator crend() const { return const_reverse_iterator(begin()); }
    iterator insert(const_iterator before, int n, const T &x);
    iterator insert(const_iterator before, T &&x);
    inline iterator insert(const_iterator before, const T &x) { return insert(before, 1, x); }
    iterator erase(const_iterator begin, const_iterator end);
    inline iterator erase(const_iterator pos) { return erase(pos, pos+1); }

    // STL compatibility:
    inline bool empty() const { return isEmpty(); }
    inline void push_back(const T &t) { append(t); }
    void push_back(T &&t) { append(std::move(t)); }
    inline void pop_back() { removeLast(); }
    inline T &front() { return first(); }
    inline const T &front() const { return first(); }
    inline T &back() { return last(); }
    inline const T &back() const { return last(); }
    void shrink_to_fit() { squeeze(); }

private:
    void realloc(int size, int alloc);

    int a;      // capacity
    int s;      // size
    T *ptr;     // data
    union {
        char array[Prealloc * sizeof(T)];
        qint64 q_for_alignment_1;
        double q_for_alignment_2;
    };

    bool isValidIterator(const const_iterator &i) const
    {
        const std::less<const T*> less = {};
        return !less(cend(), i) && !less(i, cbegin());
    }
};

#if defined(__cpp_deduction_guides) && __cpp_deduction_guides >= 201606
template <typename InputIterator,
          typename ValueType = typename std::iterator_traits<InputIterator>::value_type,
          QtPrivate::IfIsInputIterator<InputIterator> = true>
QVarLengthArray(InputIterator, InputIterator) -> QVarLengthArray<ValueType>;
#endif

template <class T, int Prealloc>
Q_INLINE_TEMPLATE QVarLengthArray<T, Prealloc>::QVarLengthArray(int asize)
    : s(asize) {
    Q_STATIC_ASSERT_X(Prealloc > 0, "QVarLengthArray Prealloc must be greater than 0.");
    Q_ASSERT_X(s >= 0, "QVarLengthArray::QVarLengthArray()", "Size must be greater than or equal to 0.");
    if (s > Prealloc) {
        ptr = reinterpret_cast<T *>(malloc(s * sizeof(T)));
        Q_CHECK_PTR(ptr);
        a = s;
    } else {
        ptr = reinterpret_cast<T *>(array);
        a = Prealloc;
    }
    if (QTypeInfo<T>::isComplex) {
        T *i = ptr + s;
        while (i != ptr)
            new (--i) T;
    }
}

template <class T, int Prealloc>
Q_INLINE_TEMPLATE void QVarLengthArray<T, Prealloc>::resize(int asize)
{ realloc(asize, qMax(asize, a)); }

template <class T, int Prealloc>
Q_INLINE_TEMPLATE void QVarLengthArray<T, Prealloc>::reserve(int asize)
{ if (asize > a) realloc(s, asize); }

template <class T, int Prealloc>
Q_INLINE_TEMPLATE int QVarLengthArray<T, Prealloc>::indexOf(const T &t, int from) const
{
    if (from < 0)
        from = qMax(from + s, 0);
    if (from < s) {
        T *n = ptr + from - 1;
        T *e = ptr + s;
        while (++n != e)
            if (*n == t)
                return n - ptr;
    }
    return -1;
}

template <class T, int Prealloc>
Q_INLINE_TEMPLATE int QVarLengthArray<T, Prealloc>::lastIndexOf(const T &t, int from) const
{
    if (from < 0)
        from += s;
    else if (from >= s)
        from = s - 1;
    if (from >= 0) {
        T *b = ptr;
        T *n = ptr + from + 1;
        while (n != b) {
            if (*--n == t)
                return n - b;
        }
    }
    return -1;
}

template <class T, int Prealloc>
Q_INLINE_TEMPLATE bool QVarLengthArray<T, Prealloc>::contains(const T &t) const
{
    T *b = ptr;
    T *i = ptr + s;
    while (i != b) {
        if (*--i == t)
            return true;
    }
    return false;
}

template <class T, int Prealloc>
Q_OUTOFLINE_TEMPLATE void QVarLengthArray<T, Prealloc>::append(const T *abuf, int increment)
{
    Q_ASSERT(abuf);
    if (increment <= 0)
        return;

    const int asize = s + increment;

    if (asize >= a)
        realloc(s, qMax(s*2, asize));

    if (QTypeInfo<T>::isComplex) {
        // call constructor for new objects (which can throw)
        while (s < asize)
            new (ptr+(s++)) T(*abuf++);
    } else {
        memcpy(static_cast<void *>(&ptr[s]), static_cast<const void *>(abuf), increment * sizeof(T));
        s = asize;
    }
}

template <class T, int Prealloc>
Q_INLINE_TEMPLATE void QVarLengthArray<T, Prealloc>::squeeze()
{ realloc(s, s); }

template <class T, int Prealloc>
Q_OUTOFLINE_TEMPLATE void QVarLengthArray<T, Prealloc>::realloc(int asize, int aalloc)
{
    Q_ASSERT(aalloc >= asize);
    T *oldPtr = ptr;
    int osize = s;

    const int copySize = qMin(asize, osize);
    Q_ASSUME(copySize >= 0);
    if (aalloc != a) {
        if (aalloc > Prealloc) {
            T* newPtr = reinterpret_cast<T *>(malloc(aalloc * sizeof(T)));
            Q_CHECK_PTR(newPtr); // could throw
            // by design: in case of QT_NO_EXCEPTIONS malloc must not fail or it crashes here
            ptr = newPtr;
            a = aalloc;
        } else {
            ptr = reinterpret_cast<T *>(array);
            a = Prealloc;
        }
        s = 0;
        if (!QTypeInfoQuery<T>::isRelocatable) {
            QT_TRY {
                // move all the old elements
                while (s < copySize) {
                    new (ptr+s) T(std::move(*(oldPtr+s)));
                    (oldPtr+s)->~T();
                    s++;
                }
            } QT_CATCH(...) {
                // clean up all the old objects and then free the old ptr
                int sClean = s;
                while (sClean < osize)
                    (oldPtr+(sClean++))->~T();
                if (oldPtr != reinterpret_cast<T *>(array) && oldPtr != ptr)
                    free(oldPtr);
                QT_RETHROW;
            }
        } else {
            memcpy(static_cast<void *>(ptr), static_cast<const void *>(oldPtr), copySize * sizeof(T));
        }
    }
    s = copySize;

    if (QTypeInfo<T>::isComplex) {
        // destroy remaining old objects
        while (osize > asize)
            (oldPtr+(--osize))->~T();
    }

    if (oldPtr != reinterpret_cast<T *>(array) && oldPtr != ptr)
        free(oldPtr);

    if (QTypeInfo<T>::isComplex) {
        // call default constructor for new objects (which can throw)
        while (s < asize)
            new (ptr+(s++)) T;
    } else {
        s = asize;
    }
}

template <class T, int Prealloc>
Q_OUTOFLINE_TEMPLATE T QVarLengthArray<T, Prealloc>::value(int i) const
{
    if (uint(i) >= uint(size())) {
        return T();
    }
    return at(i);
}
template <class T, int Prealloc>
Q_OUTOFLINE_TEMPLATE T QVarLengthArray<T, Prealloc>::value(int i, const T &defaultValue) const
{
    return (uint(i) >= uint(size())) ? defaultValue : at(i);
}

template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::insert(int i, T &&t)
{ Q_ASSERT_X(i >= 0 && i <= s, "QVarLengthArray::insert", "index out of range");
  insert(cbegin() + i, std::move(t)); }
template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::insert(int i, const T &t)
{ Q_ASSERT_X(i >= 0 && i <= s, "QVarLengthArray::insert", "index out of range");
  insert(begin() + i, 1, t); }
template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::insert(int i, int n, const T &t)
{ Q_ASSERT_X(i >= 0 && i <= s, "QVarLengthArray::insert", "index out of range");
  insert(begin() + i, n, t); }
template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::remove(int i, int n)
{ Q_ASSERT_X(i >= 0 && n >= 0 && i + n <= s, "QVarLengthArray::remove", "index out of range");
  erase(begin() + i, begin() + i + n); }
template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::remove(int i)
{ Q_ASSERT_X(i >= 0 && i < s, "QVarLengthArray::remove", "index out of range");
  erase(begin() + i, begin() + i + 1); }
template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::prepend(T &&t)
{ insert(cbegin(), std::move(t)); }
template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::prepend(const T &t)
{ insert(begin(), 1, t); }

template <class T, int Prealloc>
inline void QVarLengthArray<T, Prealloc>::replace(int i, const T &t)
{
    Q_ASSERT_X(i >= 0 && i < s, "QVarLengthArray::replace", "index out of range");
    const T copy(t);
    data()[i] = copy;
}

template <class T, int Prealloc>
Q_OUTOFLINE_TEMPLATE typename QVarLengthArray<T, Prealloc>::iterator QVarLengthArray<T, Prealloc>::insert(const_iterator before, T &&t)
{
    Q_ASSERT_X(isValidIterator(before), "QVarLengthArray::insert", "The specified const_iterator argument 'before' is invalid");

    int offset = int(before - ptr);
    reserve(s + 1);
    if (!QTypeInfo<T>::isRelocatable) {
        T *b = ptr + offset;
        T *i = ptr + s;
        T *j = i + 1;
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
        T *b = ptr + offset;
        memmove(static_cast<void *>(b + 1), static_cast<const void *>(b), (s - offset) * sizeof(T));
        new (b) T(std::move(t));
    }
    s += 1;
    return ptr + offset;
}

template <class T, int Prealloc>
Q_OUTOFLINE_TEMPLATE typename QVarLengthArray<T, Prealloc>::iterator QVarLengthArray<T, Prealloc>::insert(const_iterator before, size_type n, const T &t)
{
    Q_ASSERT_X(isValidIterator(before), "QVarLengthArray::insert", "The specified const_iterator argument 'before' is invalid");

    int offset = int(before - ptr);
    if (n != 0) {
        resize(s + n);
        const T copy(t);
        if (!QTypeInfoQuery<T>::isRelocatable) {
            T *b = ptr + offset;
            T *j = ptr + s;
            T *i = j - n;
            while (i != b)
                *--j = *--i;
            i = b + n;
            while (i != b)
                *--i = copy;
        } else {
            T *b = ptr + offset;
            T *i = b + n;
            memmove(static_cast<void *>(i), static_cast<const void *>(b), (s - offset - n) * sizeof(T));
            while (i != b)
                new (--i) T(copy);
        }
    }
    return ptr + offset;
}

template <class T, int Prealloc>
Q_OUTOFLINE_TEMPLATE typename QVarLengthArray<T, Prealloc>::iterator QVarLengthArray<T, Prealloc>::erase(const_iterator abegin, const_iterator aend)
{
    Q_ASSERT_X(isValidIterator(abegin), "QVarLengthArray::insert", "The specified const_iterator argument 'abegin' is invalid");
    Q_ASSERT_X(isValidIterator(aend), "QVarLengthArray::insert", "The specified const_iterator argument 'aend' is invalid");

    int f = int(abegin - ptr);
    int l = int(aend - ptr);
    int n = l - f;
    if (QTypeInfo<T>::isComplex) {
        std::copy(ptr + l, ptr + s, QT_MAKE_CHECKED_ARRAY_ITERATOR(ptr + f, s - f));
        T *i = ptr + s;
        T *b = ptr + s - n;
        while (i != b) {
            --i;
            i->~T();
        }
    } else {
        memmove(static_cast<void *>(ptr + f), static_cast<const void *>(ptr + l), (s - l) * sizeof(T));
    }
    s -= n;
    return ptr + f;
}

template <typename T, int Prealloc1, int Prealloc2>
bool operator==(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{
    if (l.size() != r.size())
        return false;
    const T *rb = r.begin();
    const T *b  = l.begin();
    const T *e  = l.end();
    return std::equal(b, e, QT_MAKE_CHECKED_ARRAY_ITERATOR(rb, r.size()));
}

template <typename T, int Prealloc1, int Prealloc2>
bool operator!=(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{
    return !(l == r);
}

template <typename T, int Prealloc1, int Prealloc2>
bool operator<(const QVarLengthArray<T, Prealloc1> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
    noexcept(noexcept(std::lexicographical_compare(lhs.begin(), lhs.end(),
                                                               rhs.begin(), rhs.end())))
{
    return std::lexicographical_compare(lhs.begin(), lhs.end(),
                                        rhs.begin(), rhs.end());
}

template <typename T, int Prealloc1, int Prealloc2>
inline bool operator>(const QVarLengthArray<T, Prealloc1> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return rhs < lhs;
}

template <typename T, int Prealloc1, int Prealloc2>
inline bool operator<=(const QVarLengthArray<T, Prealloc1> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return !(lhs > rhs);
}

template <typename T, int Prealloc1, int Prealloc2>
inline bool operator>=(const QVarLengthArray<T, Prealloc1> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return !(lhs < rhs);
}

template <typename T, int Prealloc>
uint qHash(const QVarLengthArray<T, Prealloc> &key, uint seed = 0)
    noexcept(noexcept(qHashRange(key.cbegin(), key.cend(), seed)))
{
    return qHashRange(key.cbegin(), key.cend(), seed);
}

QT_END_NAMESPACE

#endif // QVARLENGTHARRAY_H
