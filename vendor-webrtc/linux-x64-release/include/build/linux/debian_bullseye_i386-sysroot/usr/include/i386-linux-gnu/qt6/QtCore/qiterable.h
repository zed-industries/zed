// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QITERABLE_H
#define QITERABLE_H

#include <QtCore/qglobal.h>
#include <QtCore/qtypeinfo.h>
#include <QtCore/qmetacontainer.h>
#include <QtCore/qtaggedpointer.h>

QT_BEGIN_NAMESPACE

namespace QtPrivate {
    template<typename Type, typename Storage = Type>
    class QConstPreservingPointer
    {
        enum Tag : bool { Const, Mutable };
        QTaggedPointer<Storage, Tag> m_pointer;

    public:
        QConstPreservingPointer(std::nullptr_t) : m_pointer(nullptr, Const) {}

        QConstPreservingPointer(const void *pointer, qsizetype alignment)
            : m_pointer(reinterpret_cast<Storage *>(const_cast<void *>(pointer)), Const)
        {
            Q_UNUSED(alignment);
            Q_ASSERT(alignment > qsizetype(alignof(Storage)));
        }

        QConstPreservingPointer(void *pointer, qsizetype alignment)
            : m_pointer(reinterpret_cast<Storage *>(pointer), Mutable)
        {
            Q_UNUSED(alignment);
            Q_ASSERT(alignment > qsizetype(alignof(Storage)));
        }

        template<typename InputType>
        QConstPreservingPointer(const InputType *pointer)
            : m_pointer(reinterpret_cast<Storage *>(const_cast<InputType *>(pointer)), Const)
        {
            static_assert(alignof(InputType) >= alignof(Storage));
        }

        template<typename InputType>
        QConstPreservingPointer(InputType *pointer)
            : m_pointer(reinterpret_cast<Storage *>(pointer), Mutable)
        {
            static_assert(alignof(InputType) >= alignof(Storage));
        }

        QConstPreservingPointer() = default;

        const Type *constPointer() const
        {
            return reinterpret_cast<const Type *>(m_pointer.data());
        }

        Type *mutablePointer() const
        {
            return m_pointer.tag() == Mutable ? reinterpret_cast<Type *>(m_pointer.data()) : nullptr;
        }
    };
}

template<class Iterator, typename IteratorCategory>
class QTaggedIterator : public Iterator
{
public:
    using iterator_category = IteratorCategory;
    QTaggedIterator(Iterator &&it) : Iterator(std::move(it))
    {
        const QMetaContainer metaContainer = this->metaContainer();
        if (std::is_base_of_v<std::random_access_iterator_tag, IteratorCategory>
                && !metaContainer.hasRandomAccessIterator()) {
            qFatal("You cannot use this iterator as a random access iterator");
            this->clearIterator();
        }

        if (std::is_base_of_v<std::bidirectional_iterator_tag, IteratorCategory>
                && !metaContainer.hasBidirectionalIterator()) {
            qFatal("You cannot use this iterator as a bidirectional iterator");
            this->clearIterator();
        }

        if (std::is_base_of_v<std::forward_iterator_tag, IteratorCategory>
                && !metaContainer.hasForwardIterator()) {
            qFatal("You cannot use this iterator as a forward iterator");
            this->clearIterator();
        }

        if (std::is_base_of_v<std::input_iterator_tag, IteratorCategory>
                && !metaContainer.hasInputIterator()) {
            qFatal("You cannot use this iterator as an input iterator");
            this->clearIterator();
        }
    }

    bool operator==(const QTaggedIterator &o) const { return Iterator::operator==(o); }
    bool operator!=(const QTaggedIterator &o) const { return Iterator::operator!=(o); }
    QTaggedIterator &operator++() { Iterator::operator++(); return *this; }
    QTaggedIterator operator++(int x) { return QTaggedIterator(Iterator::operator++(x)); }
    QTaggedIterator &operator--() { Iterator::operator--(); return *this; }
    QTaggedIterator operator--(int x) { return QTaggedIterator(Iterator::operator--(x)); }
    QTaggedIterator &operator+=(qsizetype j) { Iterator::operator+=(j); return *this; }
    QTaggedIterator &operator-=(qsizetype j)  { Iterator::operator-=(j); return *this; }
    QTaggedIterator operator+(qsizetype j) const { return QTaggedIterator(Iterator::operator+(j)); }
    QTaggedIterator operator-(qsizetype j) const { return QTaggedIterator(Iterator::operator-(j)); }
    qsizetype operator-(const QTaggedIterator &j) const { return Iterator::operator-(j); }

    bool operator<(const QTaggedIterator &j) { return operator-(j) < 0; }
    bool operator>=(const QTaggedIterator &j) { return !operator<(j); }
    bool operator>(const QTaggedIterator &j) { return operator-(j) > 0; }
    bool operator<=(const QTaggedIterator &j) { return !operator>(j); }

    friend inline QTaggedIterator operator+(qsizetype j, const QTaggedIterator &k) { return k + j; }
};

template<class Container>
class QIterable;

template<class Container>
class QBaseIterator
{
private:
    QtPrivate::QConstPreservingPointer<QIterable<Container>> m_iterable;
    void *m_iterator = nullptr;

protected:
    QBaseIterator() = default;
    QBaseIterator(const QIterable<Container> *iterable, void *iterator)
        : m_iterable(iterable), m_iterator(iterator)
    {}

    QBaseIterator(QIterable<Container> *iterable, void *iterator)
        : m_iterable(iterable), m_iterator(iterator)
    {}

    QBaseIterator(QBaseIterator &&other)
        : m_iterable(std::move(other.m_iterable)), m_iterator(std::move(other.m_iterator))
    {
        other.m_iterator = nullptr;
    }

    QBaseIterator(const QBaseIterator &other)
        : m_iterable(other.m_iterable)
    {
        initIterator(other.m_iterator);
    }

    ~QBaseIterator() { clearIterator(); }

    QBaseIterator &operator=(QBaseIterator &&other)
    {
        if (this != &other) {
            clearIterator();
            m_iterable = std::move(other.m_iterable);
            m_iterator = std::move(other.m_iterator);
            other.m_iterator = nullptr;
        }
        return *this;
    }

    QBaseIterator &operator=(const QBaseIterator &other)
    {
        if (this != &other) {
            clearIterator();
            m_iterable = other.m_iterable;
            initIterator(other.m_iterator);
        }
        return *this;
    }

    QIterable<Container> *mutableIterable() const
    {
        return m_iterable.mutablePointer();
    }

    const QIterable<Container> *constIterable() const
    {
        return m_iterable.constPointer();
    }

    void initIterator(const void *copy)
    {
        if (!copy)
            return;
        if (auto *mutableIt = mutableIterable()) {
            m_iterator = metaContainer().begin(mutableIt->mutableIterable());
            metaContainer().copyIterator(m_iterator, copy);
        } else if (auto *constIt = constIterable()) {
            m_iterator = metaContainer().constBegin(constIt->constIterable());
            metaContainer().copyConstIterator(m_iterator, copy);
        }
    }

    void clearIterator()
    {
        if (!m_iterator)
            return;
        if (mutableIterable())
            metaContainer().destroyIterator(m_iterator);
        else
            metaContainer().destroyConstIterator(m_iterator);
    }

public:
    void *mutableIterator() { return m_iterator; }
    const void *constIterator() const { return m_iterator; }
    Container metaContainer() const { return constIterable()->m_metaContainer; }
};

template<class Container>
struct QIterator : public QBaseIterator<Container>
{
public:
    using difference_type = qsizetype;

    explicit QIterator(QIterable<Container> *iterable, void *iterator)
        : QBaseIterator<Container>(iterable, iterator)
    {
        Q_ASSERT(iterable != nullptr);
    }

    bool operator==(const QIterator &o) const
    {
        return this->metaContainer().compareIterator(this->constIterator(), o.constIterator());
    }

    bool operator!=(const QIterator &o) const
    {
        return !this->metaContainer().compareIterator(this->constIterator(), o.constIterator());
    }

    QIterator &operator++()
    {
        this->metaContainer().advanceIterator(this->mutableIterator(), 1);
        return *this;
    }

    QIterator operator++(int)
    {
        QIterable<Container> *iterable = this->mutableIterable();
        const Container metaContainer = this->metaContainer();
        QIterator result(iterable, metaContainer.begin(iterable->mutableIterable()));
        metaContainer.copyIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceIterator(this->mutableIterator(), 1);
        return result;
    }

    QIterator &operator--()
    {
        this->metaContainer().advanceIterator(this->mutableIterator(), -1);
        return *this;
    }

    QIterator operator--(int)
    {
        QIterable<Container> *iterable = this->mutableIterable();
        const Container metaContainer = this->metaContainer();
        QIterator result(iterable, metaContainer.begin(iterable->mutableIterable()));
        metaContainer.copyIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceIterator(this->mutableIterator(), -1);
        return result;
    }

    QIterator &operator+=(qsizetype j)
    {
        this->metaContainer().advanceIterator(this->mutableIterator(), j);
        return *this;
    }

    QIterator &operator-=(qsizetype j)
    {
        this->metaContainer().advanceIterator(this->mutableIterator(), -j);
        return *this;
    }

    QIterator operator+(qsizetype j) const
    {
        QIterable<Container> *iterable = this->mutableIterable();
        const Container metaContainer = this->metaContainer();
        QIterator result(iterable, metaContainer.begin(iterable->mutableIterable()));
        metaContainer.copyIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceIterator(result.mutableIterator(), j);
        return result;
    }

    QIterator operator-(qsizetype j) const
    {
        QIterable<Container> *iterable = this->mutableIterable();
        const Container metaContainer = this->metaContainer();
        QIterator result(iterable, metaContainer.begin(iterable->mutableIterable()));
        metaContainer.copyIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceIterator(result.mutableIterator(), -j);
        return result;
    }

    qsizetype operator-(const QIterator &j) const
    {
        return this->metaContainer().diffIterator(this->constIterator(), j.constIterator());
    }

    friend inline QIterator operator+(qsizetype j, const QIterator &k) { return k + j; }
};

template<class Container>
struct QConstIterator : public QBaseIterator<Container>
{
public:
    using difference_type = qsizetype;

    explicit QConstIterator(const QIterable<Container> *iterable, void *iterator)
        : QBaseIterator<Container>(iterable, iterator)
    {
    }

    bool operator==(const QConstIterator &o) const
    {
        return this->metaContainer().compareConstIterator(
                    this->constIterator(), o.constIterator());
    }

    bool operator!=(const QConstIterator &o) const
    {
        return !this->metaContainer().compareConstIterator(
                    this->constIterator(), o.constIterator());
    }

    QConstIterator &operator++()
    {
        this->metaContainer().advanceConstIterator(this->mutableIterator(), 1);
        return *this;
    }

    QConstIterator operator++(int)
    {
        const Container metaContainer = this->metaContainer();
        QConstIterator result(this->constIterable(), metaContainer.constBegin(
                                  this->constIterable()->constIterable()));
        metaContainer.copyConstIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceConstIterator(this->mutableIterator(), 1);
        return result;
    }

    QConstIterator &operator--()
    {
        this->metaContainer().advanceConstIterator(this->mutableIterator(), -1);
        return *this;
    }

    QConstIterator operator--(int)
    {
        const Container metaContainer = this->metaContainer();
        QConstIterator result(this->constIterable(), metaContainer.constBegin(
                                  this->constIterable()->constIterable()));
        metaContainer.copyConstIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceConstIterator(this->mutableIterator(), -1);
        return result;
    }

    QConstIterator &operator+=(qsizetype j)
    {
        this->metaContainer().advanceConstIterator(this->mutableIterator(), j);
        return *this;
    }

    QConstIterator &operator-=(qsizetype j)
    {
        this->metaContainer().advanceConstIterator(this->mutableIterator(), -j);
        return *this;
    }

    QConstIterator operator+(qsizetype j) const
    {
        const Container metaContainer = this->metaContainer();
        QConstIterator result(
                    this->constIterable(),
                    metaContainer.constBegin(this->constIterable()->constIterable()));
        metaContainer.copyConstIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceConstIterator(result.mutableIterator(), j);
        return result;
    }

    QConstIterator operator-(qsizetype j) const
    {
        const Container metaContainer = this->metaContainer();
        QConstIterator result(this->constIterable(), metaContainer.constBegin(
                                  this->constIterable()->constIterable()));
        metaContainer.copyConstIterator(result.mutableIterator(), this->constIterator());
        metaContainer.advanceConstIterator(result.mutableIterator(), -j);
        return result;
    }

    qsizetype operator-(const QConstIterator &j) const
    {
        return this->metaContainer().diffIterator(this->constIterator(), j.constIterator());
    }

    friend inline QConstIterator operator+(qsizetype j, const QConstIterator &k)
    {
        return k + j;
    }
};

template<class Container>
class QIterable
{
    friend class QBaseIterator<Container>;

protected:
    uint m_revision = 0;
    QtPrivate::QConstPreservingPointer<void, quint16> m_iterable;
    Container m_metaContainer;

public:
    template<class T>
    QIterable(const Container &metaContainer, const T *p)
      : m_iterable(p), m_metaContainer(metaContainer)
    {
    }

    template<class T>
    QIterable(const Container &metaContainer, T *p)
      : m_iterable(p), m_metaContainer(metaContainer)
    {
    }

    template<typename Pointer>
    QIterable(const Container &metaContainer, Pointer iterable)
        : m_iterable(iterable), m_metaContainer(metaContainer)
    {
    }

    QIterable(const Container &metaContainer, qsizetype alignment, const void *p)
        : m_iterable(p, alignment), m_metaContainer(metaContainer)
    {
    }

    QIterable(const Container &metaContainer, qsizetype alignment, void *p)
        : m_iterable(p, alignment), m_metaContainer(metaContainer)
    {
    }

    bool canInputIterate() const
    {
        return m_metaContainer.hasInputIterator();
    }

    bool canForwardIterate() const
    {
        return m_metaContainer.hasForwardIterator();
    }

    bool canReverseIterate() const
    {
        return m_metaContainer.hasBidirectionalIterator();
    }

    bool canRandomAccessIterate() const
    {
        return m_metaContainer.hasRandomAccessIterator();
    }

    const void *constIterable() const { return m_iterable.constPointer(); }
    void *mutableIterable() { return m_iterable.mutablePointer(); }

    QConstIterator<Container> constBegin() const
    {
        return QConstIterator(this, m_metaContainer.constBegin(constIterable()));
    }

    QConstIterator<Container> constEnd() const
    {
        return QConstIterator(this, m_metaContainer.constEnd(constIterable()));
    }

    QIterator<Container> mutableBegin()
    {
        return QIterator(this, m_metaContainer.begin(mutableIterable()));
    }

    QIterator<Container> mutableEnd()
    {
        return QIterator(this, m_metaContainer.end(mutableIterable()));
    }

    qsizetype size() const
    {
        const void *container = constIterable();
        if (m_metaContainer.hasSize())
            return m_metaContainer.size(container);
        if (!m_metaContainer.hasConstIterator())
            return -1;

        const void *begin = m_metaContainer.constBegin(container);
        const void *end = m_metaContainer.constEnd(container);
        const qsizetype size = m_metaContainer.diffConstIterator(end, begin);
        m_metaContainer.destroyConstIterator(begin);
        m_metaContainer.destroyConstIterator(end);
        return size;
    }

    Container metaContainer() const
    {
        return m_metaContainer;
    }
};

QT_END_NAMESPACE

#endif // QITERABLE_H
