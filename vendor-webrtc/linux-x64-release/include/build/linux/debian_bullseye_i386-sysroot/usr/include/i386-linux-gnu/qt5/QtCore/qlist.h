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

#ifndef QLIST_H
#define QLIST_H

#include <QtCore/qalgorithms.h>
#include <QtCore/qiterator.h>
#include <QtCore/qrefcount.h>
#include <QtCore/qarraydata.h>
#include <QtCore/qhashfunctions.h>
#include <QtCore/qvector.h>
#include <QtCore/qcontainertools_impl.h>

#include <algorithm>
#include <initializer_list>
#include <iterator>
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
#include <list>
#endif

#include <stdlib.h>
#include <new>
#include <limits.h>
#include <string.h>

#ifdef Q_CC_MSVC
#pragma warning( push )
#pragma warning( disable : 4127 ) // "conditional expression is constant"
#endif

QT_BEGIN_NAMESPACE


template <typename T> class QVector;
template <typename T> class QSet;

template <typename T> struct QListSpecialMethods
{
protected:
    ~QListSpecialMethods() = default;
};
template <> struct QListSpecialMethods<QByteArray>;
template <> struct QListSpecialMethods<QString>;

struct Q_CORE_EXPORT QListData {
    // tags for tag-dispatching of QList implementations,
    // based on QList's three different memory layouts:
    struct NotArrayCompatibleLayout {};
    struct NotIndirectLayout {};
    struct ArrayCompatibleLayout   : NotIndirectLayout {};                           // data laid out like a C array
    struct InlineWithPaddingLayout : NotArrayCompatibleLayout, NotIndirectLayout {}; // data laid out like a C array with padding
    struct IndirectLayout          : NotArrayCompatibleLayout {};                    // data allocated on the heap

    struct Data {
        QtPrivate::RefCount ref;
        int alloc, begin, end;
        void *array[1];
    };
    enum { DataHeaderSize = sizeof(Data) - sizeof(void *) };

    Data *detach(int alloc);
    Data *detach_grow(int *i, int n);
    void realloc(int alloc);
    void realloc_grow(int growth);
    inline void dispose() { dispose(d); }
    static void dispose(Data *d);
    static const Data shared_null;
    Data *d;
    void **erase(void **xi);
    void **append(int n);
    void **append();
    void **append(const QListData &l);
    void **prepend();
    void **insert(int i);
    void remove(int i);
    void remove(int i, int n);
    void move(int from, int to);
    inline int size() const noexcept { return int(d->end - d->begin); }   // q6sizetype
    inline bool isEmpty() const noexcept { return d->end  == d->begin; }
    inline void **at(int i) const noexcept { return d->array + d->begin + i; }
    inline void **begin() const noexcept { return d->array + d->begin; }
    inline void **end() const noexcept { return d->array + d->end; }
};

namespace QtPrivate {
    template <typename V, typename U> int indexOf(const QList<V> &list, const U &u, int from);
    template <typename V, typename U> int lastIndexOf(const QList<V> &list, const U &u, int from);
}

template <typename T>
class QList
#ifndef Q_QDOC
    : public QListSpecialMethods<T>
#endif
{
public:
    struct MemoryLayout
        : std::conditional<
            // must stay isStatic until ### Qt 6 for BC reasons (don't use !isRelocatable)!
            QTypeInfo<T>::isStatic || QTypeInfo<T>::isLarge,
            QListData::IndirectLayout,
            typename std::conditional<
                sizeof(T) == sizeof(void*),
                QListData::ArrayCompatibleLayout,
                QListData::InlineWithPaddingLayout
             >::type>::type {};
private:
    template <typename V, typename U> friend int QtPrivate::indexOf(const QList<V> &list, const U &u, int from);
    template <typename V, typename U> friend int QtPrivate::lastIndexOf(const QList<V> &list, const U &u, int from);
    struct Node { void *v;
#if defined(Q_CC_BOR)
        Q_INLINE_TEMPLATE T &t();
#else
        Q_INLINE_TEMPLATE T &t()
        { return *reinterpret_cast<T*>(QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic
                                       ? v : this); }
#endif
    };

    union { QListData p; QListData::Data *d; };

public:
    inline QList() noexcept : d(const_cast<QListData::Data *>(&QListData::shared_null)) { }
    QList(const QList<T> &l);
    ~QList();
    QList<T> &operator=(const QList<T> &l);
    inline QList(QList<T> &&other) noexcept
        : d(other.d) { other.d = const_cast<QListData::Data *>(&QListData::shared_null); }
    inline QList &operator=(QList<T> &&other) noexcept
    { QList moved(std::move(other)); swap(moved); return *this; }
    inline void swap(QList<T> &other) noexcept { qSwap(d, other.d); }
    inline QList(std::initializer_list<T> args)
        : QList(args.begin(), args.end()) {}
    template <typename InputIterator, QtPrivate::IfIsInputIterator<InputIterator> = true>
    QList(InputIterator first, InputIterator last);
    bool operator==(const QList<T> &l) const;
    inline bool operator!=(const QList<T> &l) const { return !(*this == l); }

    inline int size() const noexcept { return p.size(); }

    inline void detach() { if (d->ref.isShared()) detach_helper(); }

    inline void detachShared()
    {
        // The "this->" qualification is needed for GCCE.
        if (d->ref.isShared() && this->d != &QListData::shared_null)
            detach_helper();
    }

    inline bool isDetached() const { return !d->ref.isShared(); }
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
    inline void setSharable(bool sharable)
    {
        if (sharable == d->ref.isSharable())
            return;
        if (!sharable)
            detach();
        if (d != &QListData::shared_null)
            d->ref.setSharable(sharable);
    }
#endif
    inline bool isSharedWith(const QList<T> &other) const noexcept { return d == other.d; }

    inline bool isEmpty() const noexcept { return p.isEmpty(); }

    void clear();

    const T &at(int i) const;
    const T &operator[](int i) const;
    T &operator[](int i);

    void reserve(int size);
    void append(const T &t);
    void append(const QList<T> &t);
    void prepend(const T &t);
    void insert(int i, const T &t);
    void replace(int i, const T &t);
    void removeAt(int i);
    int removeAll(const T &t);
    bool removeOne(const T &t);
    T takeAt(int i);
    T takeFirst();
    T takeLast();
    void move(int from, int to);
    void swapItemsAt(int i, int j);
#if QT_DEPRECATED_SINCE(5, 13) && QT_VERSION < QT_VERSION_CHECK(6,0,0)
    QT_DEPRECATED_VERSION_X_5_13("Use QList<T>::swapItemsAt()")
    void swap(int i, int j) { swapItemsAt(i, j); }
#endif
    int indexOf(const T &t, int from = 0) const;
    int lastIndexOf(const T &t, int from = -1) const;
    bool contains(const T &t) const;
    int count(const T &t) const;

    class const_iterator;

    class iterator {
    public:
        Node *i;
        typedef std::random_access_iterator_tag  iterator_category;
        // ### Qt6: use int
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef T *pointer;
        typedef T &reference;

        inline iterator() noexcept : i(nullptr) {}
        inline iterator(Node *n) noexcept : i(n) {}
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
        // can't remove it in Qt 5, since doing so would make the type trivial,
        // which changes the way it's passed to functions by value.
        inline iterator(const iterator &o) noexcept : i(o.i){}
        inline iterator &operator=(const iterator &o) noexcept
        { i = o.i; return *this; }
#endif
        inline T &operator*() const { return i->t(); }
        inline T *operator->() const { return &i->t(); }
        inline T &operator[](difference_type j) const { return i[j].t(); }
        inline bool operator==(const iterator &o) const noexcept { return i == o.i; }
        inline bool operator!=(const iterator &o) const noexcept { return i != o.i; }
        inline bool operator<(const iterator& other) const noexcept { return i < other.i; }
        inline bool operator<=(const iterator& other) const noexcept { return i <= other.i; }
        inline bool operator>(const iterator& other) const noexcept { return i > other.i; }
        inline bool operator>=(const iterator& other) const noexcept { return i >= other.i; }
#ifndef QT_STRICT_ITERATORS
        inline bool operator==(const const_iterator &o) const noexcept
            { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const noexcept
            { return i != o.i; }
        inline bool operator<(const const_iterator& other) const noexcept
            { return i < other.i; }
        inline bool operator<=(const const_iterator& other) const noexcept
            { return i <= other.i; }
        inline bool operator>(const const_iterator& other) const noexcept
            { return i > other.i; }
        inline bool operator>=(const const_iterator& other) const noexcept
            { return i >= other.i; }
#endif
        inline iterator &operator++() { ++i; return *this; }
        inline iterator operator++(int) { Node *n = i; ++i; return n; }
        inline iterator &operator--() { i--; return *this; }
        inline iterator operator--(int) { Node *n = i; i--; return n; }
        inline iterator &operator+=(difference_type j) { i+=j; return *this; }
        inline iterator &operator-=(difference_type j) { i-=j; return *this; }
        inline iterator operator+(difference_type j) const { return iterator(i+j); }
        inline iterator operator-(difference_type j) const { return iterator(i-j); }
        friend inline iterator operator+(difference_type j, iterator k) { return k + j; }
        inline int operator-(iterator j) const { return int(i - j.i); }
    };
    friend class iterator;

    class const_iterator {
    public:
        Node *i;
        typedef std::random_access_iterator_tag  iterator_category;
        // ### Qt6: use int
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef const T *pointer;
        typedef const T &reference;

        inline const_iterator() noexcept : i(nullptr) {}
        inline const_iterator(Node *n) noexcept : i(n) {}
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
        // can't remove it in Qt 5, since doing so would make the type trivial,
        // which changes the way it's passed to functions by value.
        inline const_iterator(const const_iterator &o) noexcept : i(o.i) {}
        inline const_iterator &operator=(const const_iterator &o) noexcept
        { i = o.i; return *this; }
#endif
#ifdef QT_STRICT_ITERATORS
        inline explicit const_iterator(const iterator &o) noexcept : i(o.i) {}
#else
        inline const_iterator(const iterator &o) noexcept : i(o.i) {}
#endif
        inline const T &operator*() const { return i->t(); }
        inline const T *operator->() const { return &i->t(); }
        inline const T &operator[](difference_type j) const { return i[j].t(); }
        inline bool operator==(const const_iterator &o) const noexcept { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const noexcept { return i != o.i; }
        inline bool operator<(const const_iterator& other) const noexcept { return i < other.i; }
        inline bool operator<=(const const_iterator& other) const noexcept { return i <= other.i; }
        inline bool operator>(const const_iterator& other) const noexcept { return i > other.i; }
        inline bool operator>=(const const_iterator& other) const noexcept { return i >= other.i; }
        inline const_iterator &operator++() { ++i; return *this; }
        inline const_iterator operator++(int) { Node *n = i; ++i; return n; }
        inline const_iterator &operator--() { i--; return *this; }
        inline const_iterator operator--(int) { Node *n = i; i--; return n; }
        inline const_iterator &operator+=(difference_type j) { i+=j; return *this; }
        inline const_iterator &operator-=(difference_type j) { i-=j; return *this; }
        inline const_iterator operator+(difference_type j) const { return const_iterator(i+j); }
        inline const_iterator operator-(difference_type j) const { return const_iterator(i-j); }
        friend inline const_iterator operator+(difference_type j, const_iterator k) { return k + j; }
        inline int operator-(const_iterator j) const { return int(i - j.i); }
    };
    friend class const_iterator;

    // stl style
    typedef std::reverse_iterator<iterator> reverse_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;
    inline iterator begin() { detach(); return reinterpret_cast<Node *>(p.begin()); }
    inline const_iterator begin() const noexcept { return reinterpret_cast<Node *>(p.begin()); }
    inline const_iterator cbegin() const noexcept { return reinterpret_cast<Node *>(p.begin()); }
    inline const_iterator constBegin() const noexcept { return reinterpret_cast<Node *>(p.begin()); }
    inline iterator end() { detach(); return reinterpret_cast<Node *>(p.end()); }
    inline const_iterator end() const noexcept { return reinterpret_cast<Node *>(p.end()); }
    inline const_iterator cend() const noexcept { return reinterpret_cast<Node *>(p.end()); }
    inline const_iterator constEnd() const noexcept { return reinterpret_cast<Node *>(p.end()); }
    reverse_iterator rbegin() { return reverse_iterator(end()); }
    reverse_iterator rend() { return reverse_iterator(begin()); }
    const_reverse_iterator rbegin() const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator rend() const noexcept { return const_reverse_iterator(begin()); }
    const_reverse_iterator crbegin() const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator crend() const noexcept { return const_reverse_iterator(begin()); }
    iterator insert(iterator before, const T &t);
    iterator erase(iterator pos);
    iterator erase(iterator first, iterator last);

    // more Qt
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
    inline int count() const { return p.size(); }
    inline int length() const { return p.size(); } // Same as count()
    inline T& first() { Q_ASSERT(!isEmpty()); return *begin(); }
    inline const T& constFirst() const { return first(); }
    inline const T& first() const { Q_ASSERT(!isEmpty()); return at(0); }
    T& last() { Q_ASSERT(!isEmpty()); return *(--end()); }
    const T& last() const { Q_ASSERT(!isEmpty()); return at(count() - 1); }
    inline const T& constLast() const { return last(); }
    inline void removeFirst() { Q_ASSERT(!isEmpty()); erase(begin()); }
    inline void removeLast() { Q_ASSERT(!isEmpty()); erase(--end()); }
    inline bool startsWith(const T &t) const { return !isEmpty() && first() == t; }
    inline bool endsWith(const T &t) const { return !isEmpty() && last() == t; }
    QList<T> mid(int pos, int length = -1) const;

    T value(int i) const;
    T value(int i, const T &defaultValue) const;

    // stl compatibility
    inline void push_back(const T &t) { append(t); }
    inline void push_front(const T &t) { prepend(t); }
    inline T& front() { return first(); }
    inline const T& front() const { return first(); }
    inline T& back() { return last(); }
    inline const T& back() const { return last(); }
    inline void pop_front() { removeFirst(); }
    inline void pop_back() { removeLast(); }
    inline bool empty() const { return isEmpty(); }
    typedef int size_type;
    typedef T value_type;
    typedef value_type *pointer;
    typedef const value_type *const_pointer;
    typedef value_type &reference;
    typedef const value_type &const_reference;
    // ### Qt6: use int
    typedef qptrdiff difference_type;

    // comfort
    QList<T> &operator+=(const QList<T> &l);
    inline QList<T> operator+(const QList<T> &l) const
    { QList n = *this; n += l; return n; }
    inline QList<T> &operator+=(const T &t)
    { append(t); return *this; }
    inline QList<T> &operator<< (const T &t)
    { append(t); return *this; }
    inline QList<T> &operator<<(const QList<T> &l)
    { *this += l; return *this; }

    static QList<T> fromVector(const QVector<T> &vector);
    QVector<T> toVector() const;

#if QT_DEPRECATED_SINCE(5, 14) && QT_VERSION < QT_VERSION_CHECK(6,0,0)
    QT_DEPRECATED_VERSION_X_5_14("Use QList<T>(set.begin(), set.end()) instead.")
    static QList<T> fromSet(const QSet<T> &set);
    QT_DEPRECATED_VERSION_X_5_14("Use QSet<T>(list.begin(), list.end()) instead.")
    QSet<T> toSet() const;

    QT_DEPRECATED_VERSION_X_5_14("Use QList<T>(list.begin(), list.end()) instead.")
    static inline QList<T> fromStdList(const std::list<T> &list)
    { return QList<T>(list.begin(), list.end()); }
    QT_DEPRECATED_VERSION_X_5_14("Use std::list<T>(list.begin(), list.end()) instead.")
    inline std::list<T> toStdList() const
    { return std::list<T>(begin(), end()); }
#endif

private:
    Node *detach_helper_grow(int i, int n);
    void detach_helper(int alloc);
    void detach_helper();
    void dealloc(QListData::Data *d);

    void node_construct(Node *n, const T &t);
    void node_destruct(Node *n);
    void node_copy(Node *from, Node *to, Node *src);
    void node_destruct(Node *from, Node *to);

    bool isValidIterator(const iterator &i) const noexcept
    {
        const std::less<const Node *> less = {};
        return !less(i.i, cbegin().i) && !less(cend().i, i.i);
    }

private:
    inline bool op_eq_impl(const QList &other, QListData::NotArrayCompatibleLayout) const;
    inline bool op_eq_impl(const QList &other, QListData::ArrayCompatibleLayout) const;
    inline bool contains_impl(const T &, QListData::NotArrayCompatibleLayout) const;
    inline bool contains_impl(const T &, QListData::ArrayCompatibleLayout) const;
    inline int count_impl(const T &, QListData::NotArrayCompatibleLayout) const;
    inline int count_impl(const T &, QListData::ArrayCompatibleLayout) const;
};

#if defined(__cpp_deduction_guides) && __cpp_deduction_guides >= 201606
template <typename InputIterator,
          typename ValueType = typename std::iterator_traits<InputIterator>::value_type,
          QtPrivate::IfIsInputIterator<InputIterator> = true>
QList(InputIterator, InputIterator) -> QList<ValueType>;
#endif

#if defined(Q_CC_BOR)
template <typename T>
Q_INLINE_TEMPLATE T &QList<T>::Node::t()
{ return QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic ? *(T*)v:*(T*)this; }
#endif

template <typename T>
Q_INLINE_TEMPLATE void QList<T>::node_construct(Node *n, const T &t)
{
    if (QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic) n->v = new T(t);
    else if (QTypeInfo<T>::isComplex) new (n) T(t);
#if (defined(__GNUC__) || defined(__INTEL_COMPILER) || defined(__IBMCPP__)) && !defined(__OPTIMIZE__)
    // This violates pointer aliasing rules, but it is known to be safe (and silent)
    // in unoptimized GCC builds (-fno-strict-aliasing). The other compilers which
    // set the same define are assumed to be safe.
    else *reinterpret_cast<T*>(n) = t;
#else
    // This is always safe, but penaltizes unoptimized builds a lot.
    else ::memcpy(n, static_cast<const void *>(&t), sizeof(T));
#endif
}

template <typename T>
Q_INLINE_TEMPLATE void QList<T>::node_destruct(Node *n)
{
    if (QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic) delete reinterpret_cast<T*>(n->v);
    else if (QTypeInfo<T>::isComplex) reinterpret_cast<T*>(n)->~T();
}

template <typename T>
Q_INLINE_TEMPLATE void QList<T>::node_copy(Node *from, Node *to, Node *src)
{
    Node *current = from;
    if (QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic) {
        QT_TRY {
            while(current != to) {
                current->v = new T(*reinterpret_cast<T*>(src->v));
                ++current;
                ++src;
            }
        } QT_CATCH(...) {
            while (current-- != from)
                delete reinterpret_cast<T*>(current->v);
            QT_RETHROW;
        }

    } else if (QTypeInfo<T>::isComplex) {
        QT_TRY {
            while(current != to) {
                new (current) T(*reinterpret_cast<T*>(src));
                ++current;
                ++src;
            }
        } QT_CATCH(...) {
            while (current-- != from)
                (reinterpret_cast<T*>(current))->~T();
            QT_RETHROW;
        }
    } else {
        if (src != from && to - from > 0)
            memcpy(from, src, (to - from) * sizeof(Node));
    }
}

template <typename T>
Q_INLINE_TEMPLATE void QList<T>::node_destruct(Node *from, Node *to)
{
    if (QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic)
        while(from != to) --to, delete reinterpret_cast<T*>(to->v);
    else if (QTypeInfo<T>::isComplex)
        while (from != to) --to, reinterpret_cast<T*>(to)->~T();
}

template <typename T>
Q_INLINE_TEMPLATE QList<T> &QList<T>::operator=(const QList<T> &l)
{
    if (d != l.d) {
        QList<T> tmp(l);
        tmp.swap(*this);
    }
    return *this;
}
template <typename T>
inline typename QList<T>::iterator QList<T>::insert(iterator before, const T &t)
{
    Q_ASSERT_X(isValidIterator(before), "QList::insert", "The specified iterator argument 'before' is invalid");

    int iBefore = int(before.i - reinterpret_cast<Node *>(p.begin()));
    Node *n = nullptr;
    if (d->ref.isShared())
        n = detach_helper_grow(iBefore, 1);
    else
        n = reinterpret_cast<Node *>(p.insert(iBefore));
    QT_TRY {
        node_construct(n, t);
    } QT_CATCH(...) {
        p.remove(iBefore);
        QT_RETHROW;
    }
    return n;
}
template <typename T>
inline typename QList<T>::iterator QList<T>::erase(iterator it)
{
    Q_ASSERT_X(isValidIterator(it), "QList::erase", "The specified iterator argument 'it' is invalid");
    if (d->ref.isShared()) {
        int offset = int(it.i - reinterpret_cast<Node *>(p.begin()));
        it = begin(); // implies detach()
        it += offset;
    }
    node_destruct(it.i);
    return reinterpret_cast<Node *>(p.erase(reinterpret_cast<void**>(it.i)));
}
template <typename T>
inline const T &QList<T>::at(int i) const
{ Q_ASSERT_X(i >= 0 && i < p.size(), "QList<T>::at", "index out of range");
 return reinterpret_cast<Node *>(p.at(i))->t(); }
template <typename T>
inline const T &QList<T>::operator[](int i) const
{ Q_ASSERT_X(i >= 0 && i < p.size(), "QList<T>::operator[]", "index out of range");
 return reinterpret_cast<Node *>(p.at(i))->t(); }
template <typename T>
inline T &QList<T>::operator[](int i)
{ Q_ASSERT_X(i >= 0 && i < p.size(), "QList<T>::operator[]", "index out of range");
  detach(); return reinterpret_cast<Node *>(p.at(i))->t(); }
template <typename T>
inline void QList<T>::removeAt(int i)
{
#if !QT_DEPRECATED_SINCE(5, 15)
    Q_ASSERT_X(i >= 0 && i < p.size(), "QList<T>::removeAt", "index out of range");
#endif
    if (i < 0 || i >= p.size()) {
#if !defined(QT_NO_DEBUG)
        qWarning("QList::removeAt(): Index out of range.");
#endif
        return;
    }
    detach();
    node_destruct(reinterpret_cast<Node *>(p.at(i))); p.remove(i);
}
template <typename T>
inline T QList<T>::takeAt(int i)
{ Q_ASSERT_X(i >= 0 && i < p.size(), "QList<T>::take", "index out of range");
 detach(); Node *n = reinterpret_cast<Node *>(p.at(i)); T t = std::move(n->t()); node_destruct(n);
 p.remove(i); return t; }
template <typename T>
inline T QList<T>::takeFirst()
{ T t = std::move(first()); removeFirst(); return t; }
template <typename T>
inline T QList<T>::takeLast()
{ T t = std::move(last()); removeLast(); return t; }

template <typename T>
Q_OUTOFLINE_TEMPLATE void QList<T>::reserve(int alloc)
{
    if (d->alloc < alloc) {
        if (d->ref.isShared())
            detach_helper(alloc);
        else
            p.realloc(alloc);
    }
}

template <typename T>
Q_OUTOFLINE_TEMPLATE void QList<T>::append(const T &t)
{
    if (d->ref.isShared()) {
        Node *n = detach_helper_grow(INT_MAX, 1);
        QT_TRY {
            node_construct(n, t);
        } QT_CATCH(...) {
            --d->end;
            QT_RETHROW;
        }
    } else {
        if (QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic) {
            Node *n = reinterpret_cast<Node *>(p.append());
            QT_TRY {
                node_construct(n, t);
            } QT_CATCH(...) {
                --d->end;
                QT_RETHROW;
            }
        } else {
            Node *n, copy;
            node_construct(&copy, t); // t might be a reference to an object in the array
            QT_TRY {
                n = reinterpret_cast<Node *>(p.append());;
            } QT_CATCH(...) {
                node_destruct(&copy);
                QT_RETHROW;
            }
            *n = copy;
        }
    }
}

template <typename T>
inline void QList<T>::prepend(const T &t)
{
    if (d->ref.isShared()) {
        Node *n = detach_helper_grow(0, 1);
        QT_TRY {
            node_construct(n, t);
        } QT_CATCH(...) {
            ++d->begin;
            QT_RETHROW;
        }
    } else {
        if (QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic) {
            Node *n = reinterpret_cast<Node *>(p.prepend());
            QT_TRY {
                node_construct(n, t);
            } QT_CATCH(...) {
                ++d->begin;
                QT_RETHROW;
            }
        } else {
            Node *n, copy;
            node_construct(&copy, t); // t might be a reference to an object in the array
            QT_TRY {
                n = reinterpret_cast<Node *>(p.prepend());;
            } QT_CATCH(...) {
                node_destruct(&copy);
                QT_RETHROW;
            }
            *n = copy;
        }
    }
}

template <typename T>
inline void QList<T>::insert(int i, const T &t)
{
#if !QT_DEPRECATED_SINCE(5, 15)
    Q_ASSERT_X(i >= 0 && i <= p.size(), "QList<T>::insert", "index out of range");
#elif !defined(QT_NO_DEBUG)
    if (i < 0 || i > p.size())
        qWarning("QList::insert(): Index out of range.");
#endif
    if (d->ref.isShared()) {
        Node *n = detach_helper_grow(i, 1);
        QT_TRY {
            node_construct(n, t);
        } QT_CATCH(...) {
            p.remove(i);
            QT_RETHROW;
        }
    } else {
        if (QTypeInfo<T>::isLarge || QTypeInfo<T>::isStatic) {
            Node *n = reinterpret_cast<Node *>(p.insert(i));
            QT_TRY {
                node_construct(n, t);
            } QT_CATCH(...) {
                p.remove(i);
                QT_RETHROW;
            }
        } else {
            Node *n, copy;
            node_construct(&copy, t); // t might be a reference to an object in the array
            QT_TRY {
                n = reinterpret_cast<Node *>(p.insert(i));;
            } QT_CATCH(...) {
                node_destruct(&copy);
                QT_RETHROW;
            }
            *n = copy;
        }
    }
}

template <typename T>
inline void QList<T>::replace(int i, const T &t)
{
    Q_ASSERT_X(i >= 0 && i < p.size(), "QList<T>::replace", "index out of range");
    detach();
    reinterpret_cast<Node *>(p.at(i))->t() = t;
}

template <typename T>
inline void QList<T>::swapItemsAt(int i, int j)
{
    Q_ASSERT_X(i >= 0 && i < p.size() && j >= 0 && j < p.size(),
                "QList<T>::swap", "index out of range");
    detach();
    qSwap(d->array[d->begin + i], d->array[d->begin + j]);
}

template <typename T>
inline void QList<T>::move(int from, int to)
{
    Q_ASSERT_X(from >= 0 && from < p.size() && to >= 0 && to < p.size(),
               "QList<T>::move", "index out of range");
    detach();
    p.move(from, to);
}

template<typename T>
Q_OUTOFLINE_TEMPLATE QList<T> QList<T>::mid(int pos, int alength) const
{
    using namespace QtPrivate;
    switch (QContainerImplHelper::mid(size(), &pos, &alength)) {
    case QContainerImplHelper::Null:
    case QContainerImplHelper::Empty:
        return QList<T>();
    case QContainerImplHelper::Full:
        return *this;
    case QContainerImplHelper::Subset:
        break;
    }

    QList<T> cpy;
    if (alength <= 0)
        return cpy;
    cpy.reserve(alength);
    cpy.d->end = alength;
    QT_TRY {
        cpy.node_copy(reinterpret_cast<Node *>(cpy.p.begin()),
                      reinterpret_cast<Node *>(cpy.p.end()),
                      reinterpret_cast<Node *>(p.begin() + pos));
    } QT_CATCH(...) {
        // restore the old end
        cpy.d->end = 0;
        QT_RETHROW;
    }
    return cpy;
}

template<typename T>
Q_OUTOFLINE_TEMPLATE T QList<T>::value(int i) const
{
    if (i < 0 || i >= p.size()) {
        return T();
    }
    return reinterpret_cast<Node *>(p.at(i))->t();
}

template<typename T>
Q_OUTOFLINE_TEMPLATE T QList<T>::value(int i, const T& defaultValue) const
{
    return ((i < 0 || i >= p.size()) ? defaultValue : reinterpret_cast<Node *>(p.at(i))->t());
}

template <typename T>
Q_OUTOFLINE_TEMPLATE typename QList<T>::Node *QList<T>::detach_helper_grow(int i, int c)
{
    Node *n = reinterpret_cast<Node *>(p.begin());
    QListData::Data *x = p.detach_grow(&i, c);
    QT_TRY {
        node_copy(reinterpret_cast<Node *>(p.begin()),
                  reinterpret_cast<Node *>(p.begin() + i), n);
    } QT_CATCH(...) {
        p.dispose();
        d = x;
        QT_RETHROW;
    }
    QT_TRY {
        node_copy(reinterpret_cast<Node *>(p.begin() + i + c),
                  reinterpret_cast<Node *>(p.end()), n + i);
    } QT_CATCH(...) {
        node_destruct(reinterpret_cast<Node *>(p.begin()),
                      reinterpret_cast<Node *>(p.begin() + i));
        p.dispose();
        d = x;
        QT_RETHROW;
    }

    if (!x->ref.deref())
        dealloc(x);

    return reinterpret_cast<Node *>(p.begin() + i);
}

template <typename T>
Q_OUTOFLINE_TEMPLATE void QList<T>::detach_helper(int alloc)
{
    Node *n = reinterpret_cast<Node *>(p.begin());
    QListData::Data *x = p.detach(alloc);
    QT_TRY {
        node_copy(reinterpret_cast<Node *>(p.begin()), reinterpret_cast<Node *>(p.end()), n);
    } QT_CATCH(...) {
        p.dispose();
        d = x;
        QT_RETHROW;
    }

    if (!x->ref.deref())
        dealloc(x);
}

template <typename T>
Q_OUTOFLINE_TEMPLATE void QList<T>::detach_helper()
{
    detach_helper(d->alloc);
}

template <typename T>
Q_OUTOFLINE_TEMPLATE QList<T>::QList(const QList<T> &l)
    : QListSpecialMethods<T>(l), d(l.d)
{
    if (!d->ref.ref()) {
        p.detach(d->alloc);

        QT_TRY {
            node_copy(reinterpret_cast<Node *>(p.begin()),
                    reinterpret_cast<Node *>(p.end()),
                    reinterpret_cast<Node *>(l.p.begin()));
        } QT_CATCH(...) {
            QListData::dispose(d);
            QT_RETHROW;
        }
    }
}

template <typename T>
Q_OUTOFLINE_TEMPLATE QList<T>::~QList()
{
    if (!d->ref.deref())
        dealloc(d);
}

template <typename T>
template <typename InputIterator, QtPrivate::IfIsInputIterator<InputIterator>>
QList<T>::QList(InputIterator first, InputIterator last)
    : QList()
{
    QtPrivate::reserveIfForwardIterator(this, first, last);
    std::copy(first, last, std::back_inserter(*this));
}

template <typename T>
Q_OUTOFLINE_TEMPLATE bool QList<T>::operator==(const QList<T> &l) const
{
    if (d == l.d)
        return true;
    if (p.size() != l.p.size())
        return false;
    return this->op_eq_impl(l, MemoryLayout());
}

template <typename T>
inline bool QList<T>::op_eq_impl(const QList &l, QListData::NotArrayCompatibleLayout) const
{
    Node *i = reinterpret_cast<Node *>(p.begin());
    Node *e = reinterpret_cast<Node *>(p.end());
    Node *li = reinterpret_cast<Node *>(l.p.begin());
    for (; i != e; ++i, ++li) {
        if (!(i->t() == li->t()))
            return false;
    }
    return true;
}

template <typename T>
inline bool QList<T>::op_eq_impl(const QList &l, QListData::ArrayCompatibleLayout) const
{
    const T *lb = reinterpret_cast<const T*>(l.p.begin());
    const T *b  = reinterpret_cast<const T*>(p.begin());
    const T *e  = reinterpret_cast<const T*>(p.end());
    return std::equal(b, e, QT_MAKE_CHECKED_ARRAY_ITERATOR(lb, l.p.size()));
}

template <typename T>
Q_OUTOFLINE_TEMPLATE void QList<T>::dealloc(QListData::Data *data)
{
    node_destruct(reinterpret_cast<Node *>(data->array + data->begin),
                  reinterpret_cast<Node *>(data->array + data->end));
    QListData::dispose(data);
}


template <typename T>
Q_OUTOFLINE_TEMPLATE void QList<T>::clear()
{
    *this = QList<T>();
}

template <typename T>
Q_OUTOFLINE_TEMPLATE int QList<T>::removeAll(const T &_t)
{
    int index = indexOf(_t);
    if (index == -1)
        return 0;

    const T t = _t;
    detach();

    Node *i = reinterpret_cast<Node *>(p.at(index));
    Node *e = reinterpret_cast<Node *>(p.end());
    Node *n = i;
    node_destruct(i);
    while (++i != e) {
        if (i->t() == t)
            node_destruct(i);
        else
            *n++ = *i;
    }

    int removedCount = int(e - n);
    d->end -= removedCount;
    return removedCount;
}

template <typename T>
Q_OUTOFLINE_TEMPLATE bool QList<T>::removeOne(const T &_t)
{
    int index = indexOf(_t);
    if (index != -1) {
        removeAt(index);
        return true;
    }
    return false;
}

template <typename T>
Q_OUTOFLINE_TEMPLATE typename QList<T>::iterator QList<T>::erase(typename QList<T>::iterator afirst,
                                                                 typename QList<T>::iterator alast)
{
    Q_ASSERT_X(isValidIterator(afirst), "QList::erase", "The specified iterator argument 'afirst' is invalid");
    Q_ASSERT_X(isValidIterator(alast), "QList::erase", "The specified iterator argument 'alast' is invalid");

    if (d->ref.isShared()) {
        // ### A block is erased and a detach is needed. We should shrink and only copy relevant items.
        int offsetfirst = int(afirst.i - reinterpret_cast<Node *>(p.begin()));
        int offsetlast = int(alast.i - reinterpret_cast<Node *>(p.begin()));
        afirst = begin(); // implies detach()
        alast = afirst;
        afirst += offsetfirst;
        alast += offsetlast;
    }

    for (Node *n = afirst.i; n < alast.i; ++n)
        node_destruct(n);
    int idx = afirst - begin();
    p.remove(idx, alast - afirst);
    return begin() + idx;
}

template <typename T>
Q_OUTOFLINE_TEMPLATE QList<T> &QList<T>::operator+=(const QList<T> &l)
{
    if (!l.isEmpty()) {
        if (d == &QListData::shared_null) {
            *this = l;
        } else {
            Node *n = (d->ref.isShared())
                      ? detach_helper_grow(INT_MAX, l.size())
                      : reinterpret_cast<Node *>(p.append(l.p));
            QT_TRY {
                node_copy(n, reinterpret_cast<Node *>(p.end()),
                          reinterpret_cast<Node *>(l.p.begin()));
            } QT_CATCH(...) {
                // restore the old end
                d->end -= int(reinterpret_cast<Node *>(p.end()) - n);
                QT_RETHROW;
            }
        }
    }
    return *this;
}

template <typename T>
inline void QList<T>::append(const QList<T> &t)
{
    *this += t;
}

template <typename T>
Q_OUTOFLINE_TEMPLATE int QList<T>::indexOf(const T &t, int from) const
{
    return QtPrivate::indexOf<T, T>(*this, t, from);
}

namespace QtPrivate
{
template <typename T, typename U>
int indexOf(const QList<T> &list, const U &u, int from)
{
    typedef typename QList<T>::Node Node;

    if (from < 0)
        from = qMax(from + list.p.size(), 0);
    if (from < list.p.size()) {
        Node *n = reinterpret_cast<Node *>(list.p.at(from -1));
        Node *e = reinterpret_cast<Node *>(list.p.end());
        while (++n != e)
            if (n->t() == u)
                return int(n - reinterpret_cast<Node *>(list.p.begin()));
    }
    return -1;
}
}

template <typename T>
Q_OUTOFLINE_TEMPLATE int QList<T>::lastIndexOf(const T &t, int from) const
{
    return QtPrivate::lastIndexOf<T, T>(*this, t, from);
}

namespace QtPrivate
{
template <typename T, typename U>
int lastIndexOf(const QList<T> &list, const U &u, int from)
{
    typedef typename QList<T>::Node Node;

    if (from < 0)
        from += list.p.size();
    else if (from >= list.p.size())
        from = list.p.size()-1;
    if (from >= 0) {
        Node *b = reinterpret_cast<Node *>(list.p.begin());
        Node *n = reinterpret_cast<Node *>(list.p.at(from + 1));
        while (n-- != b) {
            if (n->t() == u)
                return int(n - b);
        }
    }
    return -1;
}
}

template <typename T>
Q_OUTOFLINE_TEMPLATE bool QList<T>::contains(const T &t) const
{
    return contains_impl(t, MemoryLayout());
}

template <typename T>
inline bool QList<T>::contains_impl(const T &t, QListData::NotArrayCompatibleLayout) const
{
    Node *e = reinterpret_cast<Node *>(p.end());
    Node *i = reinterpret_cast<Node *>(p.begin());
    for (; i != e; ++i)
        if (i->t() == t)
            return true;
    return false;
}

template <typename T>
inline bool QList<T>::contains_impl(const T &t, QListData::ArrayCompatibleLayout) const
{
    const T *b = reinterpret_cast<const T*>(p.begin());
    const T *e = reinterpret_cast<const T*>(p.end());
    return std::find(b, e, t) != e;
}

template <typename T>
Q_OUTOFLINE_TEMPLATE int QList<T>::count(const T &t) const
{
    return this->count_impl(t, MemoryLayout());
}

template <typename T>
inline int QList<T>::count_impl(const T &t, QListData::NotArrayCompatibleLayout) const
{
    int c = 0;
    Node *e = reinterpret_cast<Node *>(p.end());
    Node *i = reinterpret_cast<Node *>(p.begin());
    for (; i != e; ++i)
        if (i->t() == t)
            ++c;
    return c;
}

template <typename T>
inline int QList<T>::count_impl(const T &t, QListData::ArrayCompatibleLayout) const
{
    return int(std::count(reinterpret_cast<const T*>(p.begin()),
                          reinterpret_cast<const T*>(p.end()),
                          t));
}

template <typename T>
Q_OUTOFLINE_TEMPLATE QVector<T> QList<T>::toVector() const
{
    return QVector<T>(begin(), end());
}

template <typename T>
QList<T> QList<T>::fromVector(const QVector<T> &vector)
{
    return vector.toList();
}

template <typename T>
Q_OUTOFLINE_TEMPLATE QList<T> QVector<T>::toList() const
{
    return QList<T>(begin(), end());
}

template <typename T>
QVector<T> QVector<T>::fromList(const QList<T> &list)
{
    return list.toVector();
}

Q_DECLARE_SEQUENTIAL_ITERATOR(List)
Q_DECLARE_MUTABLE_SEQUENTIAL_ITERATOR(List)

template <typename T>
uint qHash(const QList<T> &key, uint seed = 0)
    noexcept(noexcept(qHashRange(key.cbegin(), key.cend(), seed)))
{
    return qHashRange(key.cbegin(), key.cend(), seed);
}

template <typename T>
bool operator<(const QList<T> &lhs, const QList<T> &rhs)
    noexcept(noexcept(std::lexicographical_compare(lhs.begin(), lhs.end(),
                                                               rhs.begin(), rhs.end())))
{
    return std::lexicographical_compare(lhs.begin(), lhs.end(),
                                        rhs.begin(), rhs.end());
}

template <typename T>
inline bool operator>(const QList<T> &lhs, const QList<T> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return rhs < lhs;
}

template <typename T>
inline bool operator<=(const QList<T> &lhs, const QList<T> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return !(lhs > rhs);
}

template <typename T>
inline bool operator>=(const QList<T> &lhs, const QList<T> &rhs)
    noexcept(noexcept(lhs < rhs))
{
    return !(lhs < rhs);
}

QT_END_NAMESPACE

#include <QtCore/qbytearraylist.h>
#include <QtCore/qstringlist.h>

#ifdef Q_CC_MSVC
#pragma warning( pop )
#endif

#endif // QLIST_H
