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

#ifndef QLINKEDLIST_H
#define QLINKEDLIST_H

#include <QtCore/qglobal.h>

#ifndef QT_NO_LINKED_LIST

#include <QtCore/qiterator.h>
#include <QtCore/qrefcount.h>
#include <QtCore/qcontainertools_impl.h>
#include <QtCore/qdatastream.h>
#include <QtCore/qtypeinfo.h>

#include <algorithm>
#include <initializer_list>
#include <iterator>
#include <list>


#if 0
// This is needed because of QTBUG-80347
#pragma qt_class(QLinkedList)
#pragma qt_class(QLinkedListData)
#pragma qt_class(QLinkedListNode)
#endif

#if QT_DEPRECATED_SINCE(5, 15)

QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED

QT_BEGIN_NAMESPACE

struct QT_DEPRECATED_VERSION_5_15 QLinkedListData
{
    QLinkedListData *n, *p;
    QtPrivate::RefCount ref;
    int size;
    uint sharable : 1;

    Q_CORE_EXPORT static const QLinkedListData shared_null;
};

template <typename T>
struct QT_DEPRECATED_VERSION_5_15 QLinkedListNode
{
    inline QLinkedListNode(const T &arg): t(arg) { }
    QLinkedListNode *n, *p;
    T t;
};

template <class T>
class QT_DEPRECATED_VERSION_X_5_15("Use std::list instead") QLinkedList
{
    typedef QLinkedListNode<T> Node;
    union { QLinkedListData *d; QLinkedListNode<T> *e; };

public:
    inline QLinkedList() noexcept : d(const_cast<QLinkedListData *>(&QLinkedListData::shared_null)) { }
    inline QLinkedList(const QLinkedList<T> &l) : d(l.d) { d->ref.ref(); if (!d->sharable) detach(); }
    inline QLinkedList(std::initializer_list<T> list)
        : QLinkedList(list.begin(), list.end()) {}
    template <typename InputIterator, QtPrivate::IfIsInputIterator<InputIterator> = true>
    inline QLinkedList(InputIterator first, InputIterator last)
        : QLinkedList()
    {
        std::copy(first, last, std::back_inserter(*this));
    }
    ~QLinkedList();
    QLinkedList<T> &operator=(const QLinkedList<T> &);
    QLinkedList(QLinkedList<T> &&other) noexcept
        : d(other.d) { other.d = const_cast<QLinkedListData *>(&QLinkedListData::shared_null); }
    QLinkedList<T> &operator=(QLinkedList<T> &&other) noexcept
    { QLinkedList moved(std::move(other)); swap(moved); return *this; }
    inline void swap(QLinkedList<T> &other) noexcept { qSwap(d, other.d); }
    bool operator==(const QLinkedList<T> &l) const;
    inline bool operator!=(const QLinkedList<T> &l) const { return !(*this == l); }

    inline int size() const { return d->size; }
    inline void detach()
    { if (d->ref.isShared()) detach_helper2(this->e); }
    inline bool isDetached() const { return !d->ref.isShared(); }
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
    inline void setSharable(bool sharable) { if (!sharable) detach(); if (d != &QLinkedListData::shared_null) d->sharable = sharable; }
#endif
    inline bool isSharedWith(const QLinkedList<T> &other) const { return d == other.d; }

    inline bool isEmpty() const { return d->size == 0; }

    void clear();

    void append(const T &);
    void prepend(const T &);
    T takeFirst();
    T takeLast();
    int removeAll(const T &t);
    bool removeOne(const T &t);
    bool contains(const T &t) const;
    int count(const T &t) const;

    class const_iterator;

    class iterator
    {
    public:
        typedef std::bidirectional_iterator_tag  iterator_category;
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef T *pointer;
        typedef T &reference;
        Node *i;
        inline iterator() : i(nullptr) {}
        inline iterator(Node *n) : i(n) {}
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
        iterator(const iterator &other) noexcept : i(other.i) {}
        iterator &operator=(const iterator &other) noexcept { i = other.i; return *this; }
        iterator(iterator &&other) noexcept : i(other.i) {}
        iterator &operator=(iterator &&other) noexcept { return *this = other; }
#endif
        inline T &operator*() const { return i->t; }
        inline T *operator->() const { return &i->t; }
        inline bool operator==(const iterator &o) const { return i == o.i; }
        inline bool operator!=(const iterator &o) const { return i != o.i; }
        inline bool operator==(const const_iterator &o) const
            { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const
            { return i != o.i; }
        inline iterator &operator++() { i = i->n; return *this; }
        inline iterator operator++(int) { Node *n = i; i = i->n; return n; }
        inline iterator &operator--() { i = i->p; return *this; }
        inline iterator operator--(int) { Node *n = i; i = i->p; return n; }
        inline iterator operator+(int j) const
        { Node *n = i; if (j > 0) while (j--) n = n->n; else while (j++) n = n->p; return n; }
        inline iterator operator-(int j) const { return operator+(-j); }
        inline iterator &operator+=(int j) { return *this = *this + j; }
        inline iterator &operator-=(int j) { return *this = *this - j; }
        friend inline iterator operator+(int j, iterator k) { return k + j; }
    };
    friend class iterator;

    class const_iterator
    {
    public:
        typedef std::bidirectional_iterator_tag  iterator_category;
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef const T *pointer;
        typedef const T &reference;
        Node *i;
        inline const_iterator() : i(nullptr) {}
        inline const_iterator(Node *n) : i(n) {}
        inline const_iterator(iterator ci) : i(ci.i){}
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
        const_iterator(const const_iterator &other) noexcept : i(other.i) {}
        const_iterator &operator=(const const_iterator &other) noexcept { i = other.i; return *this; }
        const_iterator(const_iterator &&other) noexcept : i(other.i) {}
        const_iterator &operator=(const_iterator &&other) noexcept { return *this = other; }
#endif
        inline const T &operator*() const { return i->t; }
        inline const T *operator->() const { return &i->t; }
        inline bool operator==(const const_iterator &o) const { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const { return i != o.i; }
        inline const_iterator &operator++() { i = i->n; return *this; }
        inline const_iterator operator++(int) { Node *n = i; i = i->n; return n; }
        inline const_iterator &operator--() { i = i->p; return *this; }
        inline const_iterator operator--(int) { Node *n = i; i = i->p; return n; }
        inline const_iterator operator+(int j) const
        { Node *n = i; if (j > 0) while (j--) n = n->n; else while (j++) n = n->p; return n; }
        inline const_iterator operator-(int j) const { return operator+(-j); }
        inline const_iterator &operator+=(int j) { return *this = *this + j; }
        inline const_iterator &operator-=(int j) { return *this = *this - j; }
        friend inline const_iterator operator+(int j, const_iterator k) { return k + j; }
    };
    friend class const_iterator;

    // stl style
    typedef std::reverse_iterator<iterator> reverse_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

    inline iterator begin() { detach(); return e->n; }
    inline const_iterator begin() const noexcept { return e->n; }
    inline const_iterator cbegin() const noexcept { return e->n; }
    inline const_iterator constBegin() const noexcept { return e->n; }
    inline iterator end() { detach(); return e; }
    inline const_iterator end() const noexcept { return e; }
    inline const_iterator cend() const noexcept { return e; }
    inline const_iterator constEnd() const noexcept { return e; }

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
    inline int count() const { return d->size; }
    inline T& first() { Q_ASSERT(!isEmpty()); return *begin(); }
    inline const T& first() const { Q_ASSERT(!isEmpty()); return *begin(); }
    T& last() { Q_ASSERT(!isEmpty()); return *(--end()); }
    const T& last() const { Q_ASSERT(!isEmpty()); return *(--end()); }
    inline void removeFirst() { Q_ASSERT(!isEmpty()); erase(begin()); }
    inline void removeLast() { Q_ASSERT(!isEmpty()); erase(--end()); }
    inline bool startsWith(const T &t) const { return !isEmpty() && first() == t; }
    inline bool endsWith(const T &t) const { return !isEmpty() && last() == t; }

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
    typedef qptrdiff difference_type;

    static inline QLinkedList<T> fromStdList(const std::list<T> &list)
    { QLinkedList<T> tmp; std::copy(list.begin(), list.end(), std::back_inserter(tmp)); return tmp; }
    inline std::list<T> toStdList() const
    { std::list<T> tmp; std::copy(constBegin(), constEnd(), std::back_inserter(tmp)); return tmp; }

    // comfort
    QLinkedList<T> &operator+=(const QLinkedList<T> &l);
    QLinkedList<T> operator+(const QLinkedList<T> &l) const;
    inline QLinkedList<T> &operator+=(const T &t) { append(t); return *this; }
    inline QLinkedList<T> &operator<< (const T &t) { append(t); return *this; }
    inline QLinkedList<T> &operator<<(const QLinkedList<T> &l) { *this += l; return *this; }

private:
    void detach_helper();
    iterator detach_helper2(iterator);
    void freeData(QLinkedListData*);
};
template <typename T>
Q_DECLARE_TYPEINFO_BODY(QLinkedList<T>, Q_MOVABLE_TYPE|Q_RELOCATABLE_TYPE);

#if defined(__cpp_deduction_guides) && __cpp_deduction_guides >= 201606
template <typename InputIterator,
          typename ValueType = typename std::iterator_traits<InputIterator>::value_type,
          QtPrivate::IfIsInputIterator<InputIterator> = true>
QLinkedList(InputIterator, InputIterator) -> QLinkedList<ValueType>;
#endif

template <typename T>
inline QLinkedList<T>::~QLinkedList()
{
    if (!d->ref.deref())
        freeData(d);
}

template <typename T>
void QLinkedList<T>::detach_helper()
{
    detach_helper2(this->e);
}

template <typename T>
typename QLinkedList<T>::iterator QLinkedList<T>::detach_helper2(iterator orgite)
{
    // detach and convert orgite to an iterator in the detached instance
    bool isEndIterator = (orgite.i == this->e);
    union { QLinkedListData *d; Node *e; } x;
    x.d = new QLinkedListData;
    x.d->ref.initializeOwned();
    x.d->size = d->size;
    x.d->sharable = true;
    Node *original = e->n;
    Node *copy = x.e;
    Node *org = orgite.i;

    while (original != org) {
        QT_TRY {
            copy->n = new Node(original->t);
            copy->n->p = copy;
            original = original->n;
            copy = copy->n;
        } QT_CATCH(...) {
            copy->n = x.e;
            Q_ASSERT(!x.d->ref.deref()); // Don't trigger assert in free
            freeData(x.d);
            QT_RETHROW;
        }
    }
    iterator r(copy);
    while (original != e) {
        QT_TRY {
            copy->n = new Node(original->t);
            copy->n->p = copy;
            original = original->n;
            copy = copy->n;
        } QT_CATCH(...) {
            copy->n = x.e;
            Q_ASSERT(!x.d->ref.deref()); // Don't trigger assert in free
            freeData(x.d);
            QT_RETHROW;
        }
    }
    copy->n = x.e;
    x.e->p = copy;
    if (!d->ref.deref())
        freeData(d);
    d = x.d;
    if (!isEndIterator)
        ++r; // since we stored the element right before the original node.
    return r;
}

template <typename T>
void QLinkedList<T>::freeData(QLinkedListData *x)
{
    Node *y = reinterpret_cast<Node*>(x);
    Node *i = y->n;
    Q_ASSERT(x->ref.atomic.loadRelaxed() == 0);
    while (i != y) {
        Node *n = i;
        i = i->n;
        delete n;
    }
    delete x;
}

template <typename T>
void QLinkedList<T>::clear()
{
    *this = QLinkedList<T>();
}

template <typename T>
QLinkedList<T> &QLinkedList<T>::operator=(const QLinkedList<T> &l)
{
    if (d != l.d) {
        QLinkedListData *o = l.d;
        o->ref.ref();
        if (!d->ref.deref())
            freeData(d);
        d = o;
        if (!d->sharable)
            detach_helper();
    }
    return *this;
}

template <typename T>
bool QLinkedList<T>::operator== (const QLinkedList<T> &l) const
{
    if (d->size != l.d->size)
        return false;
    if (e == l.e)
        return true;
    Node *i = e->n;
    Node *il = l.e->n;
    while (i != e) {
        if (! (i->t == il->t))
            return false;
        i = i->n;
        il = il->n;
    }
    return true;
}

template <typename T>
void QLinkedList<T>::append(const T &t)
{
    detach();
    Node *i = new Node(t);
    i->n = e;
    i->p = e->p;
    i->p->n = i;
    e->p = i;
    d->size++;
}

template <typename T>
void QLinkedList<T>::prepend(const T &t)
{
    detach();
    Node *i = new Node(t);
    i->n = e->n;
    i->p = e;
    i->n->p = i;
    e->n = i;
    d->size++;
}

template <typename T>
int QLinkedList<T>::removeAll(const T &_t)
{
    detach();
    const T t = _t;
    Node *i = e->n;
    int c = 0;
    while (i != e) {
        if (i->t == t) {
            Node *n = i;
            i->n->p = i->p;
            i->p->n = i->n;
            i = i->n;
            delete n;
            c++;
        } else {
            i = i->n;
        }
    }
    d->size-=c;
    return c;
}

template <typename T>
bool QLinkedList<T>::removeOne(const T &_t)
{
    detach();
    iterator it = std::find(begin(), end(), _t);
    if (it != end()) {
        erase(it);
        return true;
    }
    return false;
}

template <typename T>
inline T QLinkedList<T>::takeFirst()
{
    T t = std::move(first());
    removeFirst();
    return t;
}

template <typename T>
inline T QLinkedList<T>::takeLast()
{
    T t = std::move(last());
    removeLast();
    return t;
}

template <typename T>
bool QLinkedList<T>::contains(const T &t) const
{
    Node *i = e;
    while ((i = i->n) != e)
        if (i->t == t)
            return true;
    return false;
}

template <typename T>
int QLinkedList<T>::count(const T &t) const
{
    Node *i = e;
    int c = 0;
    while ((i = i->n) != e)
        if (i->t == t)
            c++;
    return c;
}


template <typename T>
typename QLinkedList<T>::iterator QLinkedList<T>::insert(iterator before, const T &t)
{
    if (d->ref.isShared())
        before = detach_helper2(before);

    Node *i = before.i;
    Node *m = new Node(t);
    m->n = i;
    m->p = i->p;
    m->p->n = m;
    i->p = m;
    d->size++;
    return m;
}

template <typename T>
typename QLinkedList<T>::iterator QLinkedList<T>::erase(typename QLinkedList<T>::iterator afirst,
                                                         typename QLinkedList<T>::iterator alast)
{
    while (afirst != alast)
        erase(afirst++);
    return alast;
}


template <typename T>
typename QLinkedList<T>::iterator QLinkedList<T>::erase(iterator pos)
{
    if (d->ref.isShared())
        pos = detach_helper2(pos);

    Node *i = pos.i;
    if (i != e) {
        Node *n = i;
        i->n->p = i->p;
        i->p->n = i->n;
        i = i->n;
        delete n;
        d->size--;
    }
    return i;
}

template <typename T>
QLinkedList<T> &QLinkedList<T>::operator+=(const QLinkedList<T> &l)
{
    detach();
    int n = l.d->size;
    d->size += n;
    Node *original = l.e->n;
    while (n--) {
        QT_TRY {
            Node *copy = new Node(original->t);
            original = original->n;
            copy->n = e;
            copy->p = e->p;
            copy->p->n = copy;
            e->p = copy;
        } QT_CATCH(...) {
            // restore the original list
            while (n++<d->size)
                removeLast();
            QT_RETHROW;
        }
    }
    return *this;
}

template <typename T>
QLinkedList<T> QLinkedList<T>::operator+(const QLinkedList<T> &l) const
{
    QLinkedList<T> n = *this;
    n += l;
    return n;
}

Q_DECLARE_SEQUENTIAL_ITERATOR(LinkedList)
Q_DECLARE_MUTABLE_SEQUENTIAL_ITERATOR(LinkedList)

#ifndef QT_NO_DATASTREAM
template <typename T>
inline QDataStream &operator>>(QDataStream &s, QLinkedList<T> &l)
{
    return QtPrivate::readListBasedContainer(s, l);
}

template <typename T>
inline QDataStream &operator<<(QDataStream &s, const QLinkedList<T> &l)
{
    return QtPrivate::writeSequentialContainer(s, l);
}
#endif

QT_END_NAMESPACE

Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(QLinkedList)

QT_WARNING_POP

#endif // QT_DEPRECATED_SINCE(5, 15)

#endif // QT_NO_LINKED_LIST

#endif // QLINKEDLIST_H
