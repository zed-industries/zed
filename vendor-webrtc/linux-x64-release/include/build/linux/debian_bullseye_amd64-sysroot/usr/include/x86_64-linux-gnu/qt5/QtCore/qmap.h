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

#ifndef QMAP_H
#define QMAP_H

#include <QtCore/qiterator.h>
#include <QtCore/qlist.h>
#include <QtCore/qrefcount.h>
#include <QtCore/qpair.h>

#ifdef Q_MAP_DEBUG
#include <QtCore/qdebug.h>
#endif

#include <functional>
#include <initializer_list>
#include <map>
#include <new>

QT_BEGIN_NAMESPACE

/*
    QMap uses qMapLessThanKey() to compare keys. The default
    implementation uses operator<(). For pointer types,
    qMapLessThanKey() uses std::less (because operator<() on
    pointers can be used only between pointers in the same array).
*/

template <class Key> inline bool qMapLessThanKey(const Key &key1, const Key &key2)
{
    return key1 < key2;
}

template <class Ptr> inline bool qMapLessThanKey(const Ptr *key1, const Ptr *key2)
{
    return std::less<const Ptr *>()(key1, key2);
}

struct QMapDataBase;
template <class Key, class T> struct QMapData;

struct Q_CORE_EXPORT QMapNodeBase
{
    quintptr p;
    QMapNodeBase *left;
    QMapNodeBase *right;

    enum Color { Red = 0, Black = 1 };
    enum { Mask = 3 }; // reserve the second bit as well

    const QMapNodeBase *nextNode() const;
    QMapNodeBase *nextNode() { return const_cast<QMapNodeBase *>(const_cast<const QMapNodeBase *>(this)->nextNode()); }
    const QMapNodeBase *previousNode() const;
    QMapNodeBase *previousNode() { return const_cast<QMapNodeBase *>(const_cast<const QMapNodeBase *>(this)->previousNode()); }

    Color color() const { return Color(p & 1); }
    void setColor(Color c) { if (c == Black) p |= Black; else p &= ~Black; }
    QMapNodeBase *parent() const { return reinterpret_cast<QMapNodeBase *>(p & ~Mask); }
    void setParent(QMapNodeBase *pp) { p = (p & Mask) | quintptr(pp); }

    template <typename T>
    static typename std::enable_if<QTypeInfo<T>::isComplex>::type
    callDestructorIfNecessary(T &t) noexcept { Q_UNUSED(t); t.~T(); } // Q_UNUSED: silence MSVC unused 't' warning
    template <typename T>
    static typename std::enable_if<!QTypeInfo<T>::isComplex>::type
    callDestructorIfNecessary(T &) noexcept {}
};

template <class Key, class T>
struct QMapNode : public QMapNodeBase
{
    Key key;
    T value;

    inline QMapNode *leftNode() const { return static_cast<QMapNode *>(left); }
    inline QMapNode *rightNode() const { return static_cast<QMapNode *>(right); }

    inline const QMapNode *nextNode() const { return reinterpret_cast<const QMapNode *>(QMapNodeBase::nextNode()); }
    inline const QMapNode *previousNode() const { return static_cast<const QMapNode *>(QMapNodeBase::previousNode()); }
    inline QMapNode *nextNode() { return reinterpret_cast<QMapNode *>(QMapNodeBase::nextNode()); }
    inline QMapNode *previousNode() { return static_cast<QMapNode *>(QMapNodeBase::previousNode()); }

    QMapNode<Key, T> *copy(QMapData<Key, T> *d) const;

    void destroySubTree()
    {
        callDestructorIfNecessary(key);
        callDestructorIfNecessary(value);
        doDestroySubTree(std::integral_constant<bool, QTypeInfo<T>::isComplex || QTypeInfo<Key>::isComplex>());
    }

    QMapNode<Key, T> *lowerBound(const Key &key);
    QMapNode<Key, T> *upperBound(const Key &key);

private:
    void doDestroySubTree(std::false_type) {}
    void doDestroySubTree(std::true_type)
    {
        if (left)
            leftNode()->destroySubTree();
        if (right)
            rightNode()->destroySubTree();
    }

    QMapNode() = delete;
    Q_DISABLE_COPY(QMapNode)
};

template <class Key, class T>
inline QMapNode<Key, T> *QMapNode<Key, T>::lowerBound(const Key &akey)
{
    QMapNode<Key, T> *n = this;
    QMapNode<Key, T> *lastNode = nullptr;
    while (n) {
        if (!qMapLessThanKey(n->key, akey)) {
            lastNode = n;
            n = n->leftNode();
        } else {
            n = n->rightNode();
        }
    }
    return lastNode;
}

template <class Key, class T>
inline QMapNode<Key, T> *QMapNode<Key, T>::upperBound(const Key &akey)
{
    QMapNode<Key, T> *n = this;
    QMapNode<Key, T> *lastNode = nullptr;
    while (n) {
        if (qMapLessThanKey(akey, n->key)) {
            lastNode = n;
            n = n->leftNode();
        } else {
            n = n->rightNode();
        }
    }
    return lastNode;
}



struct Q_CORE_EXPORT QMapDataBase
{
    QtPrivate::RefCount ref;
    int size;
    QMapNodeBase header;
    QMapNodeBase *mostLeftNode;

    void rotateLeft(QMapNodeBase *x);
    void rotateRight(QMapNodeBase *x);
    void rebalance(QMapNodeBase *x);
    void freeNodeAndRebalance(QMapNodeBase *z);
    void recalcMostLeftNode();

    QMapNodeBase *createNode(int size, int alignment, QMapNodeBase *parent, bool left);
    void freeTree(QMapNodeBase *root, int alignment);

    static const QMapDataBase shared_null;

    static QMapDataBase *createData();
    static void freeData(QMapDataBase *d);
};

template <class Key, class T>
struct QMapData : public QMapDataBase
{
    typedef QMapNode<Key, T> Node;

    Node *root() const { return static_cast<Node *>(header.left); }

    // using reinterpret_cast because QMapDataBase::header is not
    // actually a QMapNode.
    const Node *end() const { return reinterpret_cast<const Node *>(&header); }
    Node *end() { return reinterpret_cast<Node *>(&header); }
    const Node *begin() const { if (root()) return static_cast<const Node*>(mostLeftNode); return end(); }
    Node *begin() { if (root()) return static_cast<Node*>(mostLeftNode); return end(); }

    void deleteNode(Node *z);
    Node *findNode(const Key &akey) const;
    void nodeRange(const Key &akey, Node **firstNode, Node **lastNode);

    Node *createNode(const Key &k, const T &v, Node *parent = nullptr, bool left = false)
    {
        Node *n = static_cast<Node *>(QMapDataBase::createNode(sizeof(Node), Q_ALIGNOF(Node),
                                      parent, left));
        QT_TRY {
            new (&n->key) Key(k);
            QT_TRY {
                new (&n->value) T(v);
            } QT_CATCH(...) {
                n->key.~Key();
                QT_RETHROW;
            }
        } QT_CATCH(...) {
            QMapDataBase::freeNodeAndRebalance(n);
            QT_RETHROW;
        }
        return n;
    }

    static QMapData *create() {
        return static_cast<QMapData *>(createData());
    }

    void destroy() {
        if (root()) {
            root()->destroySubTree();
            freeTree(header.left, Q_ALIGNOF(Node));
        }
        freeData(this);
    }
};

template <class Key, class T>
QMapNode<Key, T> *QMapNode<Key, T>::copy(QMapData<Key, T> *d) const
{
    QMapNode<Key, T> *n = d->createNode(key, value);
    n->setColor(color());
    if (left) {
        n->left = leftNode()->copy(d);
        n->left->setParent(n);
    } else {
        n->left = nullptr;
    }
    if (right) {
        n->right = rightNode()->copy(d);
        n->right->setParent(n);
    } else {
        n->right = nullptr;
    }
    return n;
}

template <class Key, class T>
void QMapData<Key, T>::deleteNode(QMapNode<Key, T> *z)
{
    QMapNodeBase::callDestructorIfNecessary(z->key);
    QMapNodeBase::callDestructorIfNecessary(z->value);
    freeNodeAndRebalance(z);
}

template <class Key, class T>
QMapNode<Key, T> *QMapData<Key, T>::findNode(const Key &akey) const
{
    if (Node *r = root()) {
        Node *lb = r->lowerBound(akey);
        if (lb && !qMapLessThanKey(akey, lb->key))
            return lb;
    }
    return nullptr;
}


template <class Key, class T>
void QMapData<Key, T>::nodeRange(const Key &akey, QMapNode<Key, T> **firstNode, QMapNode<Key, T> **lastNode)
{
    Node *n = root();
    Node *l = end();
    while (n) {
        if (qMapLessThanKey(akey, n->key)) {
            l = n;
            n = n->leftNode();
        } else if (qMapLessThanKey(n->key, akey)) {
            n = n->rightNode();
        } else {
            *firstNode = n->leftNode() ? n->leftNode()->lowerBound(akey) : nullptr;
            if (!*firstNode)
                *firstNode = n;
            *lastNode = n->rightNode() ? n->rightNode()->upperBound(akey) : nullptr;
            if (!*lastNode)
                *lastNode = l;
            return;
        }
    }
    *firstNode = *lastNode = l;
}


template <class Key, class T>
class QMap
{
    typedef QMapNode<Key, T> Node;

    QMapData<Key, T> *d;

public:
    inline QMap() noexcept : d(static_cast<QMapData<Key, T> *>(const_cast<QMapDataBase *>(&QMapDataBase::shared_null))) { }
    inline QMap(std::initializer_list<std::pair<Key,T> > list)
        : d(static_cast<QMapData<Key, T> *>(const_cast<QMapDataBase *>(&QMapDataBase::shared_null)))
    {
        for (typename std::initializer_list<std::pair<Key,T> >::const_iterator it = list.begin(); it != list.end(); ++it)
            insert(it->first, it->second);
    }
    QMap(const QMap<Key, T> &other);

    inline ~QMap() { if (!d->ref.deref()) d->destroy(); }

    QMap<Key, T> &operator=(const QMap<Key, T> &other);
    inline QMap(QMap<Key, T> &&other) noexcept
        : d(other.d)
    {
        other.d = static_cast<QMapData<Key, T> *>(
                const_cast<QMapDataBase *>(&QMapDataBase::shared_null));
    }

    inline QMap<Key, T> &operator=(QMap<Key, T> &&other) noexcept
    { QMap moved(std::move(other)); swap(moved); return *this; }
    inline void swap(QMap<Key, T> &other) noexcept { qSwap(d, other.d); }
    explicit QMap(const typename std::map<Key, T> &other);
    std::map<Key, T> toStdMap() const;

    bool operator==(const QMap<Key, T> &other) const;
    inline bool operator!=(const QMap<Key, T> &other) const { return !(*this == other); }

    inline int size() const { return d->size; }

    inline bool isEmpty() const { return d->size == 0; }

    inline void detach() { if (d->ref.isShared()) detach_helper(); }
    inline bool isDetached() const { return !d->ref.isShared(); }
#if !defined(QT_NO_UNSHARABLE_CONTAINERS)
    inline void setSharable(bool sharable)
    {
        if (sharable == d->ref.isSharable())
            return;
        if (!sharable)
            detach();
        // Don't call on shared_null
        d->ref.setSharable(sharable);
    }
#endif
    inline bool isSharedWith(const QMap<Key, T> &other) const { return d == other.d; }

    void clear();

    int remove(const Key &key);
    T take(const Key &key);

    bool contains(const Key &key) const;
    const Key key(const T &value, const Key &defaultKey = Key()) const;
    const T value(const Key &key, const T &defaultValue = T()) const;
    T &operator[](const Key &key);
    const T operator[](const Key &key) const;

    QList<Key> keys() const;
    QList<Key> keys(const T &value) const;
    QList<T> values() const;
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_VERSION_X_5_15("Use QMultiMap for maps storing multiple values with the same key.") QList<Key> uniqueKeys() const;
    QT_DEPRECATED_VERSION_X_5_15("Use QMultiMap for maps storing multiple values with the same key.") QList<T> values(const Key &key) const;
#endif
    int count(const Key &key) const;


    inline const Key &firstKey() const { Q_ASSERT(!isEmpty()); return constBegin().key(); }
    inline const Key &lastKey() const { Q_ASSERT(!isEmpty()); return (constEnd() - 1).key(); }

    inline T &first() { Q_ASSERT(!isEmpty()); return *begin(); }
    inline const T &first() const { Q_ASSERT(!isEmpty()); return *constBegin(); }
    inline T &last() { Q_ASSERT(!isEmpty()); return *(end() - 1); }
    inline const T &last() const { Q_ASSERT(!isEmpty()); return *(constEnd() - 1); }

    class const_iterator;

    class iterator
    {
        friend class const_iterator;
        Node *i;

    public:
        typedef std::bidirectional_iterator_tag iterator_category;
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef T *pointer;
        typedef T &reference;

        inline iterator() : i(nullptr) { }
        inline iterator(Node *node) : i(node) { }

        inline const Key &key() const { return i->key; }
        inline T &value() const { return i->value; }
        inline T &operator*() const { return i->value; }
        inline T *operator->() const { return &i->value; }
        inline bool operator==(const iterator &o) const { return i == o.i; }
        inline bool operator!=(const iterator &o) const { return i != o.i; }

        inline iterator &operator++() {
            i = i->nextNode();
            return *this;
        }
        inline iterator operator++(int) {
            iterator r = *this;
            i = i->nextNode();
            return r;
        }
        inline iterator &operator--() {
            i = i->previousNode();
            return *this;
        }
        inline iterator operator--(int) {
            iterator r = *this;
            i = i->previousNode();
            return r;
        }
        inline iterator operator+(int j) const
        { iterator r = *this; if (j > 0) while (j--) ++r; else while (j++) --r; return r; }
        inline iterator operator-(int j) const { return operator+(-j); }
        inline iterator &operator+=(int j) { return *this = *this + j; }
        inline iterator &operator-=(int j) { return *this = *this - j; }
        friend inline iterator operator+(int j, iterator k) { return k + j; }

#ifndef QT_STRICT_ITERATORS
    public:
        inline bool operator==(const const_iterator &o) const
            { return i == o.i; }
        inline bool operator!=(const const_iterator &o) const
            { return i != o.i; }
#endif
        friend class QMap<Key, T>;
        friend class QMultiMap<Key, T>;
    };
    friend class iterator;

    class const_iterator
    {
        friend class iterator;
        const Node *i;

    public:
        typedef std::bidirectional_iterator_tag iterator_category;
        typedef qptrdiff difference_type;
        typedef T value_type;
        typedef const T *pointer;
        typedef const T &reference;

        Q_DECL_CONSTEXPR inline const_iterator() : i(nullptr) { }
        inline const_iterator(const Node *node) : i(node) { }
#ifdef QT_STRICT_ITERATORS
        explicit inline const_iterator(const iterator &o)
#else
        inline const_iterator(const iterator &o)
#endif
        { i = o.i; }

        inline const Key &key() const { return i->key; }
        inline const T &value() const { return i->value; }
        inline const T &operator*() const { return i->value; }
        inline const T *operator->() const { return &i->value; }
        Q_DECL_CONSTEXPR inline bool operator==(const const_iterator &o) const { return i == o.i; }
        Q_DECL_CONSTEXPR inline bool operator!=(const const_iterator &o) const { return i != o.i; }

        inline const_iterator &operator++() {
            i = i->nextNode();
            return *this;
        }
        inline const_iterator operator++(int) {
            const_iterator r = *this;
            i = i->nextNode();
            return r;
        }
        inline const_iterator &operator--() {
            i = i->previousNode();
            return *this;
        }
        inline const_iterator operator--(int) {
            const_iterator r = *this;
            i = i->previousNode();
            return r;
        }
        inline const_iterator operator+(int j) const
        { const_iterator r = *this; if (j > 0) while (j--) ++r; else while (j++) --r; return r; }
        inline const_iterator operator-(int j) const { return operator+(-j); }
        inline const_iterator &operator+=(int j) { return *this = *this + j; }
        inline const_iterator &operator-=(int j) { return *this = *this - j; }
        friend inline const_iterator operator+(int j, const_iterator k) { return k + j; }

#ifdef QT_STRICT_ITERATORS
    private:
        inline bool operator==(const iterator &o) const { return operator==(const_iterator(o)); }
        inline bool operator!=(const iterator &o) const { return operator!=(const_iterator(o)); }
#endif
        friend class QMap<Key, T>;
        friend class QMultiMap<Key, T>;
    };
    friend class const_iterator;

    class key_iterator
    {
        const_iterator i;

    public:
        typedef typename const_iterator::iterator_category iterator_category;
        typedef typename const_iterator::difference_type difference_type;
        typedef Key value_type;
        typedef const Key *pointer;
        typedef const Key &reference;

        key_iterator() = default;
        explicit key_iterator(const_iterator o) : i(o) { }

        const Key &operator*() const { return i.key(); }
        const Key *operator->() const { return &i.key(); }
        bool operator==(key_iterator o) const { return i == o.i; }
        bool operator!=(key_iterator o) const { return i != o.i; }

        inline key_iterator &operator++() { ++i; return *this; }
        inline key_iterator operator++(int) { return key_iterator(i++);}
        inline key_iterator &operator--() { --i; return *this; }
        inline key_iterator operator--(int) { return key_iterator(i--); }
        const_iterator base() const { return i; }
    };

    typedef QKeyValueIterator<const Key&, const T&, const_iterator> const_key_value_iterator;
    typedef QKeyValueIterator<const Key&, T&, iterator> key_value_iterator;

    // STL style
    inline iterator begin() { detach(); return iterator(d->begin()); }
    inline const_iterator begin() const { return const_iterator(d->begin()); }
    inline const_iterator constBegin() const { return const_iterator(d->begin()); }
    inline const_iterator cbegin() const { return const_iterator(d->begin()); }
    inline iterator end() { detach(); return iterator(d->end()); }
    inline const_iterator end() const { return const_iterator(d->end()); }
    inline const_iterator constEnd() const { return const_iterator(d->end()); }
    inline const_iterator cend() const { return const_iterator(d->end()); }
    inline key_iterator keyBegin() const { return key_iterator(begin()); }
    inline key_iterator keyEnd() const { return key_iterator(end()); }
    inline key_value_iterator keyValueBegin() { return key_value_iterator(begin()); }
    inline key_value_iterator keyValueEnd() { return key_value_iterator(end()); }
    inline const_key_value_iterator keyValueBegin() const { return const_key_value_iterator(begin()); }
    inline const_key_value_iterator constKeyValueBegin() const { return const_key_value_iterator(begin()); }
    inline const_key_value_iterator keyValueEnd() const { return const_key_value_iterator(end()); }
    inline const_key_value_iterator constKeyValueEnd() const { return const_key_value_iterator(end()); }
    iterator erase(iterator it);

    // more Qt
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
    inline int count() const { return d->size; }
    iterator find(const Key &key);
    const_iterator find(const Key &key) const;
    const_iterator constFind(const Key &key) const;
    iterator lowerBound(const Key &key);
    const_iterator lowerBound(const Key &key) const;
    iterator upperBound(const Key &key);
    const_iterator upperBound(const Key &key) const;
    iterator insert(const Key &key, const T &value);
    iterator insert(const_iterator pos, const Key &key, const T &value);
    void insert(const QMap<Key, T> &map);
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_VERSION_X_5_15("Use QMultiMap for maps storing multiple values with the same key.") iterator insertMulti(const Key &key, const T &value);
    QT_DEPRECATED_VERSION_X_5_15("Use QMultiMap for maps storing multiple values with the same key.") iterator insertMulti(const_iterator pos, const Key &akey, const T &avalue);
    QT_DEPRECATED_VERSION_X_5_15("Use QMultiMap for maps storing multiple values with the same key.") QMap<Key, T> &unite(const QMap<Key, T> &other);
#endif

    // STL compatibility
    typedef Key key_type;
    typedef T mapped_type;
    typedef qptrdiff difference_type;
    typedef int size_type;
    inline bool empty() const { return isEmpty(); }
    QPair<iterator, iterator> equal_range(const Key &akey);
    QPair<const_iterator, const_iterator> equal_range(const Key &akey) const;

#ifdef Q_MAP_DEBUG
    void dump() const;
#endif

private:
    void detach_helper();
    bool isValidIterator(const const_iterator &ci) const
    {
#if defined(QT_DEBUG) && !defined(Q_MAP_NO_ITERATOR_DEBUG)
        const QMapNodeBase *n = ci.i;
        while (n->parent())
            n = n->parent();
        return n->left == d->root();
#else
        Q_UNUSED(ci);
        return true;
#endif
    }

    friend class QMultiMap<Key, T>;
};

template <class Key, class T>
inline QMap<Key, T>::QMap(const QMap<Key, T> &other)
{
    if (other.d->ref.ref()) {
        d = other.d;
    } else {
        d = QMapData<Key, T>::create();
        if (other.d->header.left) {
            d->header.left = static_cast<Node *>(other.d->header.left)->copy(d);
            d->header.left->setParent(&d->header);
            d->recalcMostLeftNode();
        }
    }
}

template <class Key, class T>
Q_INLINE_TEMPLATE QMap<Key, T> &QMap<Key, T>::operator=(const QMap<Key, T> &other)
{
    if (d != other.d) {
        QMap<Key, T> tmp(other);
        tmp.swap(*this);
    }
    return *this;
}

template <class Key, class T>
Q_INLINE_TEMPLATE void QMap<Key, T>::clear()
{
    *this = QMap<Key, T>();
}

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wreturn-stack-address")

template <class Key, class T>
Q_INLINE_TEMPLATE const T QMap<Key, T>::value(const Key &akey, const T &adefaultValue) const
{
    Node *n = d->findNode(akey);
    return n ? n->value : adefaultValue;
}

QT_WARNING_POP

template <class Key, class T>
Q_INLINE_TEMPLATE const T QMap<Key, T>::operator[](const Key &akey) const
{
    return value(akey);
}

template <class Key, class T>
Q_INLINE_TEMPLATE T &QMap<Key, T>::operator[](const Key &akey)
{
    detach();
    Node *n = d->findNode(akey);
    if (!n)
        return *insert(akey, T());
    return n->value;
}

template <class Key, class T>
Q_INLINE_TEMPLATE int QMap<Key, T>::count(const Key &akey) const
{
    Node *firstNode;
    Node *lastNode;
    d->nodeRange(akey, &firstNode, &lastNode);

    const_iterator ci_first(firstNode);
    const const_iterator ci_last(lastNode);
    int cnt = 0;
    while (ci_first != ci_last) {
        ++cnt;
        ++ci_first;
    }
    return cnt;
}

template <class Key, class T>
Q_INLINE_TEMPLATE bool QMap<Key, T>::contains(const Key &akey) const
{
    return d->findNode(akey) != nullptr;
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::iterator QMap<Key, T>::insert(const Key &akey, const T &avalue)
{
    detach();
    Node *n = d->root();
    Node *y = d->end();
    Node *lastNode = nullptr;
    bool  left = true;
    while (n) {
        y = n;
        if (!qMapLessThanKey(n->key, akey)) {
            lastNode = n;
            left = true;
            n = n->leftNode();
        } else {
            left = false;
            n = n->rightNode();
        }
    }
    if (lastNode && !qMapLessThanKey(akey, lastNode->key)) {
        lastNode->value = avalue;
        return iterator(lastNode);
    }
    Node *z = d->createNode(akey, avalue, y, left);
    return iterator(z);
}

template <class Key, class T>
typename QMap<Key, T>::iterator QMap<Key, T>::insert(const_iterator pos, const Key &akey, const T &avalue)
{
    if (d->ref.isShared())
        return this->insert(akey, avalue);

    Q_ASSERT_X(isValidIterator(pos), "QMap::insert", "The specified const_iterator argument 'it' is invalid");

    if (pos == constEnd()) {
        // Hint is that the Node is larger than (or equal to) the largest value.
        Node *n = static_cast<Node *>(pos.i->left);
        if (n) {
            while (n->right)
                n = static_cast<Node *>(n->right);

            if (!qMapLessThanKey(n->key, akey))
                return this->insert(akey, avalue); // ignore hint
            // This can be optimized by checking equal too.
            // we can overwrite if previous node key is strictly smaller
            // (or there is no previous node)

            Node *z = d->createNode(akey, avalue, n, false); // insert right most
            return iterator(z);
        }
        return this->insert(akey, avalue);
    } else {
        // Hint indicates that the node should be less (or equal to) the hint given
        // but larger than the previous value.
        Node *next = const_cast<Node*>(pos.i);
        if (qMapLessThanKey(next->key, akey))
            return this->insert(akey, avalue); // ignore hint

        if (pos == constBegin()) {
            // There is no previous value
            // Maybe overwrite left most value
            if (!qMapLessThanKey(akey, next->key)) {
                next->value = avalue; // overwrite current iterator
                return iterator(next);
            }
            // insert left most.
            Node *z = d->createNode(akey, avalue, begin().i, true);
            return iterator(z);
        } else {
            Node *prev = const_cast<Node*>(pos.i->previousNode());
            if (!qMapLessThanKey(prev->key, akey)) {
                return this->insert(akey, avalue); // ignore hint
            }
            // Hint is ok
            if (!qMapLessThanKey(akey, next->key)) {
                next->value = avalue; // overwrite current iterator
                return iterator(next);
            }

            // we need to insert (not overwrite)
            if (prev->right == nullptr) {
                Node *z = d->createNode(akey, avalue, prev, false);
                return iterator(z);
            }
            if (next->left == nullptr) {
                Node *z = d->createNode(akey, avalue, next, true);
                return iterator(z);
            }
            Q_ASSERT(false); // We should have prev->right == nullptr or next->left == nullptr.
            return this->insert(akey, avalue);
        }
    }
}

template <class Key, class T>
Q_INLINE_TEMPLATE void QMap<Key, T>::insert(const QMap<Key, T> &map)
{
    if (d == map.d)
        return;

    detach();

    Node *n = d->root();
    auto it = map.cbegin();
    const auto e = map.cend();
    while (it != e) {
        // Insertion here is based on insert(Key, T)
        auto parent = d->end();
        bool left = true;
        Node *lastNode = nullptr;
        while (n) {
            parent = n;
            if (!qMapLessThanKey(n->key, it.key())) {
                lastNode = n;
                n = n->leftNode();
                left = true;
            } else {
                n = n->rightNode();
                left = false;
            }
        }
        if (lastNode && !qMapLessThanKey(it.key(), lastNode->key)) {
            lastNode->value = it.value();
            n = lastNode;
        } else {
            n = d->createNode(it.key(), it.value(), parent, left);
        }
        ++it;
        if (it != e) {
            // Move back up the tree until we find the next branch or node which is
            // relevant for the next key.
            while (n != d->root() && qMapLessThanKey(n->key, it.key()))
                n = static_cast<Node *>(n->parent());
        }
    }
}


template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::const_iterator QMap<Key, T>::constFind(const Key &akey) const
{
    Node *n = d->findNode(akey);
    return const_iterator(n ? n : d->end());
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::const_iterator QMap<Key, T>::find(const Key &akey) const
{
    return constFind(akey);
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::iterator QMap<Key, T>::find(const Key &akey)
{
    detach();
    Node *n = d->findNode(akey);
    return iterator(n ? n : d->end());
}

template <class Key, class T>
QPair<typename QMap<Key, T>::iterator, typename QMap<Key, T>::iterator> QMap<Key, T>::equal_range(const Key &akey)
{
    detach();
    Node *firstNode, *lastNode;
    d->nodeRange(akey, &firstNode, &lastNode);
    return QPair<iterator, iterator>(iterator(firstNode), iterator(lastNode));
}

template <class Key, class T>
QPair<typename QMap<Key, T>::const_iterator, typename QMap<Key, T>::const_iterator>
QMap<Key, T>::equal_range(const Key &akey) const
{
    Node *firstNode, *lastNode;
    d->nodeRange(akey, &firstNode, &lastNode);
    return qMakePair(const_iterator(firstNode), const_iterator(lastNode));
}

#ifdef Q_MAP_DEBUG
template <class Key, class T>
void QMap<Key, T>::dump() const
{
    const_iterator it = begin();
    qDebug("map dump:");
    while (it != end()) {
        const QMapNodeBase *n = it.i;
        int depth = 0;
        while (n && n != d->root()) {
            ++depth;
            n = n->parent();
        }
        QByteArray space(4*depth, ' ');
        qDebug() << space << (it.i->color() == Node::Red ? "Red  " : "Black") << it.i << it.i->left << it.i->right
                 << it.key() << it.value();
        ++it;
    }
    qDebug("---------");
}
#endif

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE int QMap<Key, T>::remove(const Key &akey)
{
    detach();
    int n = 0;
    while (Node *node = d->findNode(akey)) {
        d->deleteNode(node);
        ++n;
    }
    return n;
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE T QMap<Key, T>::take(const Key &akey)
{
    detach();

    Node *node = d->findNode(akey);
    if (node) {
        T t = std::move(node->value);
        d->deleteNode(node);
        return t;
    }
    return T();
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE typename QMap<Key, T>::iterator QMap<Key, T>::erase(iterator it)
{
    if (it == iterator(d->end()))
        return it;

    Q_ASSERT_X(isValidIterator(const_iterator(it)), "QMap::erase", "The specified iterator argument 'it' is invalid");

    if (d->ref.isShared()) {
        const_iterator oldBegin = constBegin();
        const_iterator old = const_iterator(it);
        int backStepsWithSameKey = 0;

        while (old != oldBegin) {
            --old;
            if (qMapLessThanKey(old.key(), it.key()))
                break;
            ++backStepsWithSameKey;
        }

        it = find(old.key()); // ensures detach
        Q_ASSERT_X(it != iterator(d->end()), "QMap::erase", "Unable to locate same key in erase after detach.");

        while (backStepsWithSameKey > 0) {
            ++it;
            --backStepsWithSameKey;
        }
    }

    Node *n = it.i;
    ++it;
    d->deleteNode(n);
    return it;
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE void QMap<Key, T>::detach_helper()
{
    QMapData<Key, T> *x = QMapData<Key, T>::create();
    if (d->header.left) {
        x->header.left = static_cast<Node *>(d->header.left)->copy(x);
        x->header.left->setParent(&x->header);
    }
    if (!d->ref.deref())
        d->destroy();
    d = x;
    d->recalcMostLeftNode();
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE QList<Key> QMap<Key, T>::keys() const
{
    QList<Key> res;
    res.reserve(size());
    const_iterator i = begin();
    while (i != end()) {
        res.append(i.key());
        ++i;
    }
    return res;
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE QList<Key> QMap<Key, T>::keys(const T &avalue) const
{
    QList<Key> res;
    const_iterator i = begin();
    while (i != end()) {
        if (i.value() == avalue)
            res.append(i.key());
        ++i;
    }
    return res;
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE const Key QMap<Key, T>::key(const T &avalue, const Key &defaultKey) const
{
    const_iterator i = begin();
    while (i != end()) {
        if (i.value() == avalue)
            return i.key();
        ++i;
    }

    return defaultKey;
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE QList<T> QMap<Key, T>::values() const
{
    QList<T> res;
    res.reserve(size());
    const_iterator i = begin();
    while (i != end()) {
        res.append(i.value());
        ++i;
    }
    return res;
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::const_iterator QMap<Key, T>::lowerBound(const Key &akey) const
{
    Node *lb = d->root() ? d->root()->lowerBound(akey) : nullptr;
    if (!lb)
        lb = d->end();
    return const_iterator(lb);
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::iterator QMap<Key, T>::lowerBound(const Key &akey)
{
    detach();
    Node *lb = d->root() ? d->root()->lowerBound(akey) : nullptr;
    if (!lb)
        lb = d->end();
    return iterator(lb);
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::const_iterator
QMap<Key, T>::upperBound(const Key &akey) const
{
    Node *ub = d->root() ? d->root()->upperBound(akey) : nullptr;
    if (!ub)
        ub = d->end();
    return const_iterator(ub);
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::iterator QMap<Key, T>::upperBound(const Key &akey)
{
    detach();
    Node *ub = d->root() ? d->root()->upperBound(akey) : nullptr;
    if (!ub)
        ub = d->end();
    return iterator(ub);
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE bool QMap<Key, T>::operator==(const QMap<Key, T> &other) const
{
    if (size() != other.size())
        return false;
    if (d == other.d)
        return true;

    const_iterator it1 = begin();
    const_iterator it2 = other.begin();

    while (it1 != end()) {
        if (!(it1.value() == it2.value()) || qMapLessThanKey(it1.key(), it2.key()) || qMapLessThanKey(it2.key(), it1.key()))
            return false;
        ++it2;
        ++it1;
    }
    return true;
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE QMap<Key, T>::QMap(const std::map<Key, T> &other)
{
    d = QMapData<Key, T>::create();
    typename std::map<Key,T>::const_iterator it = other.end();
    while (it != other.begin()) {
        --it;
        d->createNode((*it).first, (*it).second, d->begin(), true); // insert on most left node.
    }
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE std::map<Key, T> QMap<Key, T>::toStdMap() const
{
    std::map<Key, T> map;
    const_iterator it = end();
    while (it != begin()) {
        --it;
        map.insert(map.begin(), std::pair<Key, T>(it.key(), it.value()));
    }
    return map;
}

template <class Key, class T>
class QMultiMap : public QMap<Key, T>
{
public:
    QMultiMap() noexcept {}
    inline QMultiMap(std::initializer_list<std::pair<Key,T> > list)
    {
        for (typename std::initializer_list<std::pair<Key,T> >::const_iterator it = list.begin(); it != list.end(); ++it)
            insert(it->first, it->second);
    }
    QMultiMap(const QMap<Key, T> &other) : QMap<Key, T>(other) {}
    QMultiMap(QMap<Key, T> &&other) noexcept : QMap<Key, T>(std::move(other)) {}
    void swap(QMultiMap<Key, T> &other) noexcept { QMap<Key, T>::swap(other); }

    QList<Key> uniqueKeys() const;
    QList<T> values(const Key &key) const;

    inline typename QMap<Key, T>::iterator replace(const Key &key, const T &value)
    { return QMap<Key, T>::insert(key, value); }
    typename QMap<Key, T>::iterator insert(const Key &key, const T &value);
    //! [qmultimap-insert-pos]
    typename QMap<Key, T>::iterator insert(typename QMap<Key, T>::const_iterator pos,
                                           const Key &key, const T &value);

    //! [qmultimap-unite]
    QMultiMap &unite(const QMultiMap &other);
    inline QMultiMap &operator+=(const QMultiMap &other)
    { return unite(other); }
    inline QMultiMap operator+(const QMultiMap &other) const
    { QMultiMap result = *this; result += other; return result; }

    using QMap<Key, T>::contains;
    using QMap<Key, T>::remove;
    using QMap<Key, T>::count;
    using QMap<Key, T>::find;
    using QMap<Key, T>::constFind;
    using QMap<Key, T>::values;
    using QMap<Key, T>::size;
    using QMap<Key, T>::detach;
    using QMap<Key, T>::erase;
    using QMap<Key, T>::isValidIterator;
    using typename QMap<Key, T>::Node;

    bool contains(const Key &key, const T &value) const;

    int remove(const Key &key, const T &value);

    int count(const Key &key, const T &value) const;

    typename QMap<Key, T>::iterator find(const Key &key, const T &value) {
        typename QMap<Key, T>::iterator i(find(key));
        typename QMap<Key, T>::iterator end(this->end());
        while (i != end && !qMapLessThanKey<Key>(key, i.key())) {
            if (i.value() == value)
                return i;
            ++i;
        }
        return end;
    }
    typename QMap<Key, T>::const_iterator find(const Key &key, const T &value) const {
        typename QMap<Key, T>::const_iterator i(constFind(key));
        typename QMap<Key, T>::const_iterator end(QMap<Key, T>::constEnd());
        while (i != end && !qMapLessThanKey<Key>(key, i.key())) {
            if (i.value() == value)
                return i;
            ++i;
        }
        return end;
    }
    typename QMap<Key, T>::const_iterator constFind(const Key &key, const T &value) const
        { return find(key, value); }
private:
    T &operator[](const Key &key);
    const T operator[](const Key &key) const;
};

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE QList<Key> QMultiMap<Key, T>::uniqueKeys() const
{
    QList<Key> res;
    res.reserve(size()); // May be too much, but assume short lifetime
    typename QMap<Key, T>::const_iterator i = this->begin();
    if (i != this->end()) {
        for (;;) {
            const Key &aKey = i.key();
            res.append(aKey);
            do {
                if (++i == this->end())
                    goto break_out_of_outer_loop;
            } while (!qMapLessThanKey(aKey, i.key()));   // loop while (key == i.key())
        }
    }
break_out_of_outer_loop:
    return res;
}

template <class Key, class T>
Q_OUTOFLINE_TEMPLATE QList<T> QMultiMap<Key, T>::values(const Key &akey) const
{
    QList<T> res;
    Node *n = this->d->findNode(akey);
    if (n) {
        typename QMap<Key, T>::const_iterator it(n);
        do {
            res.append(*it);
            ++it;
        } while (it != this->constEnd() && !qMapLessThanKey<Key>(akey, it.key()));
    }
    return res;
}

template <class Key, class T>
Q_INLINE_TEMPLATE typename QMap<Key, T>::iterator QMultiMap<Key, T>::insert(const Key &akey,
                                                                            const T &avalue)
{
    detach();
    Node* y = this->d->end();
    Node* x = static_cast<Node *>(this->d->root());
    bool left = true;
    while (x != nullptr) {
        left = !qMapLessThanKey(x->key, akey);
        y = x;
        x = left ? x->leftNode() : x->rightNode();
    }
    Node *z = this->d->createNode(akey, avalue, y, left);
    return typename QMap<Key, T>::iterator(z);
}

#ifndef Q_CLANG_QDOC
template <class Key, class T>
typename QMap<Key, T>::iterator QMultiMap<Key, T>::insert(typename QMap<Key, T>::const_iterator pos,
                                                          const Key &akey, const T &avalue)
{
    if (this->d->ref.isShared())
        return insert(akey, avalue);

    Q_ASSERT_X(isValidIterator(pos), "QMap::insert", "The specified const_iterator argument 'pos' is invalid");

    if (pos == this->constEnd()) {
        // Hint is that the Node is larger than (or equal to) the largest value.
        Node *n = static_cast<Node *>(pos.i->left);
        if (n) {
            while (n->right)
                n = static_cast<Node *>(n->right);

            if (!qMapLessThanKey(n->key, akey))
                return insert(akey, avalue); // ignore hint
            Node *z = this->d->createNode(akey, avalue, n, false); // insert right most
            return typename QMap<Key, T>::iterator(z);
        }
        return insert(akey, avalue);
    } else {
        // Hint indicates that the node should be less (or equal to) the hint given
        // but larger than the previous value.
        Node *next = const_cast<Node*>(pos.i);
        if (qMapLessThanKey(next->key, akey))
            return insert(akey, avalue); // ignore hint

        if (pos == this->constBegin()) {
            // There is no previous value (insert left most)
            Node *z = this->d->createNode(akey, avalue, this->begin().i, true);
            return typename QMap<Key, T>::iterator(z);
        } else {
            Node *prev = const_cast<Node*>(pos.i->previousNode());
            if (!qMapLessThanKey(prev->key, akey))
                return insert(akey, avalue); // ignore hint

            // Hint is ok - do insert
            if (prev->right == nullptr) {
                Node *z = this->d->createNode(akey, avalue, prev, false);
                return typename QMap<Key, T>::iterator(z);
            }
            if (next->left == nullptr) {
                Node *z = this->d->createNode(akey, avalue, next, true);
                return typename QMap<Key, T>::iterator(z);
            }
            Q_ASSERT(false); // We should have prev->right == nullptr or next->left == nullptr.
            return insert(akey, avalue);
        }
    }
}

template <class Key, class T>
Q_INLINE_TEMPLATE QMultiMap<Key, T> &QMultiMap<Key, T>::unite(const QMultiMap<Key, T> &other)
{
    QMultiMap<Key, T> copy(other);
    typename QMap<Key, T>::const_iterator it = copy.constEnd();
    const typename QMap<Key, T>::const_iterator b = copy.constBegin();
    while (it != b) {
        --it;
        insert(it.key(), it.value());
    }
    return *this;
}
#endif // Q_CLANG_QDOC

template <class Key, class T>
Q_INLINE_TEMPLATE bool QMultiMap<Key, T>::contains(const Key &key, const T &value) const
{
    return constFind(key, value) != QMap<Key, T>::constEnd();
}

template <class Key, class T>
Q_INLINE_TEMPLATE int QMultiMap<Key, T>::remove(const Key &key, const T &value)
{
    int n = 0;
    typename QMap<Key, T>::iterator i(find(key));
    typename QMap<Key, T>::iterator end(QMap<Key, T>::end());
    while (i != end && !qMapLessThanKey<Key>(key, i.key())) {
        if (i.value() == value) {
            i = erase(i);
            ++n;
        } else {
            ++i;
        }
    }
    return n;
}

template <class Key, class T>
Q_INLINE_TEMPLATE int QMultiMap<Key, T>::count(const Key &key, const T &value) const
{
    int n = 0;
    typename QMap<Key, T>::const_iterator i(constFind(key));
    typename QMap<Key, T>::const_iterator end(QMap<Key, T>::constEnd());
    while (i != end && !qMapLessThanKey<Key>(key, i.key())) {
        if (i.value() == value)
            ++n;
        ++i;
    }
    return n;
}

#if QT_DEPRECATED_SINCE(5, 15)
template<class Key, class T>
QList<Key> QMap<Key, T>::uniqueKeys() const
{
    return static_cast<const QMultiMap<Key, T> *>(this)->uniqueKeys();
}

template<class Key, class T>
QList<T> QMap<Key, T>::values(const Key &key) const
{
    return static_cast<const QMultiMap<Key, T> *>(this)->values(key);
}

template<class Key, class T>
typename QMap<Key, T>::iterator QMap<Key, T>::insertMulti(const Key &key, const T &value)
{
    return static_cast<QMultiMap<Key, T> *>(this)->insert(key, value);
}

template<class Key, class T>
typename QMap<Key, T>::iterator QMap<Key, T>::insertMulti(const_iterator pos, const Key &akey, const T &avalue)
{
    return static_cast<QMultiMap<Key, T> *>(this)->insert(pos, akey, avalue);
}

template<class Key, class T>
QMap<Key, T> &QMap<Key, T>::unite(const QMap<Key, T> &other)
{
    return static_cast<QMultiMap<Key, T> *>(this)->unite(other);
}
#endif

Q_DECLARE_ASSOCIATIVE_ITERATOR(Map)
Q_DECLARE_MUTABLE_ASSOCIATIVE_ITERATOR(Map)

QT_END_NAMESPACE

#endif // QMAP_H
