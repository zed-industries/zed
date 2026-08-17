// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSEQUENTIALITERABLE_H
#define QSEQUENTIALITERABLE_H

#include <QtCore/qiterable.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE

class Q_CORE_EXPORT QSequentialIterator : public QIterator<QMetaSequence>
{
public:
    using value_type = QVariant;
    using reference = QVariantRef<QSequentialIterator>;
    using pointer = QVariantPointer<QSequentialIterator>;

    QSequentialIterator(QIterator &&it)
        : QIterator(std::move(it))
    {}

    QVariantRef<QSequentialIterator> operator*() const;
    QVariantPointer<QSequentialIterator> operator->() const;
};

class Q_CORE_EXPORT QSequentialConstIterator : public QConstIterator<QMetaSequence>
{
public:
    using value_type = QVariant;
    using reference = const QVariant &;
    using pointer = QVariantConstPointer;

    QSequentialConstIterator(QConstIterator &&it)
        : QConstIterator(std::move(it))
    {}

    QVariant operator*() const;
    QVariantConstPointer operator->() const;
};

class Q_CORE_EXPORT QSequentialIterable : public QIterable<QMetaSequence>
{
public:
    using iterator = QTaggedIterator<QSequentialIterator, void>;
    using const_iterator = QTaggedIterator<QSequentialConstIterator, void>;

    using RandomAccessIterator = QTaggedIterator<iterator, std::random_access_iterator_tag>;
    using BidirectionalIterator = QTaggedIterator<iterator, std::bidirectional_iterator_tag>;
    using ForwardIterator = QTaggedIterator<iterator, std::forward_iterator_tag>;
    using InputIterator = QTaggedIterator<iterator, std::input_iterator_tag>;

    using RandomAccessConstIterator = QTaggedIterator<const_iterator, std::random_access_iterator_tag>;
    using BidirectionalConstIterator = QTaggedIterator<const_iterator, std::bidirectional_iterator_tag>;
    using ForwardConstIterator = QTaggedIterator<const_iterator, std::forward_iterator_tag>;
    using InputConstIterator = QTaggedIterator<const_iterator, std::input_iterator_tag>;

    template<class T>
    QSequentialIterable(const T *p)
        : QIterable(QMetaSequence::fromContainer<T>(), p)
    {
        Q_UNUSED(m_revision);
    }

    template<class T>
    QSequentialIterable(T *p)
        : QIterable(QMetaSequence::fromContainer<T>(), p)
    {
    }

    QSequentialIterable()
        : QIterable(QMetaSequence(), nullptr)
    {
    }

    template<typename Pointer>
    QSequentialIterable(const QMetaSequence &metaSequence, Pointer iterable)
        : QIterable(metaSequence, iterable)
    {
    }

    // ### Qt7: Pass QMetaType as value rather than const ref.
    QSequentialIterable(const QMetaSequence &metaSequence, const QMetaType &metaType,
                        void *iterable)
        : QIterable(metaSequence, metaType.alignOf(), iterable)
    {
    }

    // ### Qt7: Pass QMetaType as value rather than const ref.
    QSequentialIterable(const QMetaSequence &metaSequence, const QMetaType &metaType,
                        const void *iterable)
        : QIterable(metaSequence, metaType.alignOf(), iterable)
    {
    }

    QSequentialIterable(QIterable<QMetaSequence> &&other) : QIterable(std::move(other)) {}

    QSequentialIterable &operator=(QIterable<QMetaSequence> &&other)
    {
        QIterable::operator=(std::move(other));
        return *this;
    }

    const_iterator begin() const { return constBegin(); }
    const_iterator end() const { return constEnd(); }

    const_iterator constBegin() const { return const_iterator(QIterable::constBegin()); }
    const_iterator constEnd() const { return const_iterator(QIterable::constEnd()); }

    iterator mutableBegin() { return iterator(QIterable::mutableBegin()); }
    iterator mutableEnd() { return iterator(QIterable::mutableEnd()); }

    QVariant at(qsizetype idx) const;
    void set(qsizetype idx, const QVariant &value);

    enum Position { Unspecified, AtBegin, AtEnd };
    void addValue(const QVariant &value, Position position = Unspecified);
    void removeValue(Position position = Unspecified);

    QMetaType valueMetaType() const;
};

template<>
inline QVariantRef<QSequentialIterator>::operator QVariant() const
{
    if (m_pointer == nullptr)
        return QVariant();
    const QMetaType metaType(m_pointer->metaContainer().valueMetaType());
    QVariant v(metaType);
    void *dataPtr = metaType == QMetaType::fromType<QVariant>() ? &v : v.data();
    m_pointer->metaContainer().valueAtIterator(m_pointer->constIterator(), dataPtr);
    return v;
}

template<>
inline QVariantRef<QSequentialIterator> &QVariantRef<QSequentialIterator>::operator=(
        const QVariant &value)
{
    if (m_pointer == nullptr)
        return *this;

    QtPrivate::QVariantTypeCoercer coercer;
    m_pointer->metaContainer().setValueAtIterator(
                m_pointer->constIterator(),
                coercer.coerce(value, m_pointer->metaContainer().valueMetaType()));
    return *this;
}

Q_DECLARE_TYPEINFO(QSequentialIterable, Q_RELOCATABLE_TYPE);
Q_DECLARE_TYPEINFO(QSequentialIterable::iterator, Q_RELOCATABLE_TYPE);
Q_DECLARE_TYPEINFO(QSequentialIterable::const_iterator, Q_RELOCATABLE_TYPE);

QT_END_NAMESPACE

#endif // QSEQUENTIALITERABLE_H
