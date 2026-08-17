// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCACHE_H
#define QCACHE_H

#include <QtCore/qhash.h>

QT_BEGIN_NAMESPACE


template <class Key, class T>
class QCache
{
    struct Value
    {
        T *t = nullptr;
        qsizetype cost = 0;
        Value() noexcept = default;
        Value(T *tt, qsizetype c) noexcept
            : t(tt), cost(c)
        {}
        Value(Value &&other) noexcept
            : t(other.t),
              cost(other.cost)
        {
            other.t = nullptr;
        }
        Value &operator=(Value &&other) noexcept
        {
            qt_ptr_swap(t, other.t);
            std::swap(cost, other.cost);
            return *this;
        }
        ~Value() { delete t; }

    private:
        Q_DISABLE_COPY(Value)
    };

    struct Chain
    {
        Chain() noexcept : prev(this), next(this) { }
        Chain *prev;
        Chain *next;
    };

    struct Node : public Chain
    {
        using KeyType = Key;
        using ValueType = Value;

        Key key;
        Value value;

        Node(const Key &k, Value &&t) noexcept(std::is_nothrow_move_assignable_v<Key>)
            : Chain(),
              key(k),
              value(std::move(t))
        {
        }
        Node(Key &&k, Value &&t) noexcept(std::is_nothrow_move_assignable_v<Key>)
            : Chain(),
              key(std::move(k)),
              value(std::move(t))
        {
        }
        static void createInPlace(Node *n, const Key &k, T *o, qsizetype cost)
        {
            new (n) Node{ Key(k), Value(o, cost) };
        }
        void emplace(T *o, qsizetype cost)
        {
            value = Value(o, cost);
        }

        Node(Node &&other)
            : Chain(other),
              key(std::move(other.key)),
              value(std::move(other.value))
        {
            Q_ASSERT(this->prev);
            Q_ASSERT(this->next);
            this->prev->next = this;
            this->next->prev = this;
        }
    private:
        Q_DISABLE_COPY(Node)
    };

    using Data = QHashPrivate::Data<Node>;

    mutable Chain chain;
    Data d;
    qsizetype mx = 0;
    qsizetype total = 0;

    void unlink(Node *n) noexcept(std::is_nothrow_destructible_v<Node>)
    {
        Q_ASSERT(n->prev);
        Q_ASSERT(n->next);
        n->prev->next = n->next;
        n->next->prev = n->prev;
        total -= n->value.cost;
        auto it = d.findBucket(n->key);
        d.erase(it);
    }
    T *relink(const Key &key) const noexcept
    {
        if (isEmpty())
            return nullptr;
        Node *n = d.findNode(key);
        if (!n)
            return nullptr;

        if (chain.next != n) {
            Q_ASSERT(n->prev);
            Q_ASSERT(n->next);
            n->prev->next = n->next;
            n->next->prev = n->prev;
            n->next = chain.next;
            chain.next->prev = n;
            n->prev = &chain;
            chain.next = n;
        }
        return n->value.t;
    }

    void trim(qsizetype m) noexcept(std::is_nothrow_destructible_v<Node>)
    {
        while (chain.prev != &chain && total > m) {
            Node *n = static_cast<Node *>(chain.prev);
            unlink(n);
        }
    }


    Q_DISABLE_COPY(QCache)

public:
    inline explicit QCache(qsizetype maxCost = 100) noexcept
        : mx(maxCost)
    {
    }
    inline ~QCache()
    {
        static_assert(std::is_nothrow_destructible_v<Key>, "Types with throwing destructors are not supported in Qt containers.");
        static_assert(std::is_nothrow_destructible_v<T>, "Types with throwing destructors are not supported in Qt containers.");

        clear();
    }

    inline qsizetype maxCost() const noexcept { return mx; }
    void setMaxCost(qsizetype m) noexcept(std::is_nothrow_destructible_v<Node>)
    {
        mx = m;
        trim(mx);
    }
    inline qsizetype totalCost() const noexcept { return total; }

    inline qsizetype size() const noexcept { return qsizetype(d.size); }
    inline qsizetype count() const noexcept { return qsizetype(d.size); }
    inline bool isEmpty() const noexcept { return !d.size; }
    inline QList<Key> keys() const
    {
        QList<Key> k;
        if (size()) {
            k.reserve(size());
            for (auto it = d.begin(); it != d.end(); ++it)
                k << it.node()->key;
        }
        Q_ASSERT(k.size() == size());
        return k;
    }

    void clear() noexcept(std::is_nothrow_destructible_v<Node>)
    {
        d.clear();
        total = 0;
        chain.next = &chain;
        chain.prev = &chain;
    }

    bool insert(const Key &key, T *object, qsizetype cost = 1)
    {
        if (cost > mx) {
            remove(key);
            delete object;
            return false;
        }
        trim(mx - cost);
        auto result = d.findOrInsert(key);
        Node *n = result.it.node();
        if (result.initialized) {
            auto prevCost = n->value.cost;
            result.it.node()->emplace(object, cost);
            cost -= prevCost;
            relink(key);
        } else {
            Node::createInPlace(n, key, object, cost);
            n->prev = &chain;
            n->next = chain.next;
            chain.next->prev = n;
            chain.next = n;
        }
        total += cost;
        return true;
    }
    T *object(const Key &key) const noexcept
    {
        return relink(key);
    }
    T *operator[](const Key &key) const noexcept
    {
        return relink(key);
    }
    inline bool contains(const Key &key) const noexcept
    {
        return !isEmpty() && d.findNode(key) != nullptr;
    }

    bool remove(const Key &key) noexcept(std::is_nothrow_destructible_v<Node>)
    {
        if (isEmpty())
            return false;
        Node *n = d.findNode(key);
        if (!n) {
            return false;
        } else {
            unlink(n);
            return true;
        }
    }

    T *take(const Key &key) noexcept(std::is_nothrow_destructible_v<Key>)
    {
        if (isEmpty())
            return nullptr;
        Node *n = d.findNode(key);
        if (!n)
            return nullptr;

        T *t = n->value.t;
        n->value.t = nullptr;
        unlink(n);
        return t;
    }

};

QT_END_NAMESPACE

#endif // QCACHE_H
