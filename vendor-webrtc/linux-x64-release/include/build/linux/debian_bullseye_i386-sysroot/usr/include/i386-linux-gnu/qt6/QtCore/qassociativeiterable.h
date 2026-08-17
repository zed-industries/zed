// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QASSOCIATIVEITERABLE_H
#define QASSOCIATIVEITERABLE_H

#include <QtCore/qiterable.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE

class Q_CORE_EXPORT QAssociativeIterator : public QIterator<QMetaAssociation>
{
public:
    using key_type = QVariant;
    using mapped_type = QVariant;
    using reference = QVariantRef<QAssociativeIterator>;
    using pointer = QVariantPointer<QAssociativeIterator>;

    QAssociativeIterator(QIterator &&it)
        : QIterator(std::move(it))
    {}

    QVariant key() const;
    QVariantRef<QAssociativeIterator> value() const;

    QVariantRef<QAssociativeIterator> operator*() const;
    QVariantPointer<QAssociativeIterator> operator->() const;
};

class Q_CORE_EXPORT QAssociativeConstIterator : public QConstIterator<QMetaAssociation>
{
public:
    using key_type = QVariant;
    using mapped_type = QVariant;
    using reference = const QVariant &;
    using pointer = QVariantConstPointer;

    QAssociativeConstIterator(QConstIterator &&it)
        : QConstIterator(std::move(it))
    {}

    QVariant key() const;
    QVariant value() const;

    QVariant operator*() const;
    QVariantConstPointer operator->() const;
};

class Q_CORE_EXPORT QAssociativeIterable : public QIterable<QMetaAssociation>
{
public:
    using iterator = QTaggedIterator<QAssociativeIterator, void>;
    using const_iterator = QTaggedIterator<QAssociativeConstIterator, void>;

    using RandomAccessIterator = QTaggedIterator<iterator, std::random_access_iterator_tag>;
    using BidirectionalIterator = QTaggedIterator<iterator, std::bidirectional_iterator_tag>;
    using ForwardIterator = QTaggedIterator<iterator, std::forward_iterator_tag>;
    using InputIterator = QTaggedIterator<iterator, std::input_iterator_tag>;

    using RandomAccessConstIterator = QTaggedIterator<const_iterator, std::random_access_iterator_tag>;
    using BidirectionalConstIterator = QTaggedIterator<const_iterator, std::bidirectional_iterator_tag>;
    using ForwardConstIterator = QTaggedIterator<const_iterator, std::forward_iterator_tag>;
    using InputConstIterator = QTaggedIterator<const_iterator, std::input_iterator_tag>;

    template<class T>
    QAssociativeIterable(const T *p)
        : QIterable(QMetaAssociation::fromContainer<T>(), p)
    {
    }

    template<class T>
    QAssociativeIterable(T *p)
        : QIterable(QMetaAssociation::fromContainer<T>(), p)
    {
    }

    QAssociativeIterable()
        : QIterable(QMetaAssociation(), nullptr)
    {
    }

    template<typename Pointer>
    QAssociativeIterable(const QMetaAssociation &metaAssociation, Pointer iterable)
        : QIterable(metaAssociation, iterable)
    {
    }

    // ### Qt7: Pass QMetaType as value rather than const ref.
    QAssociativeIterable(const QMetaAssociation &metaAssociation, const QMetaType &metaType,
                         void *iterable)
        : QIterable(metaAssociation, metaType.alignOf(), iterable)
    {
    }

    // ### Qt7: Pass QMetaType as value rather than const ref.
    QAssociativeIterable(const QMetaAssociation &metaAssociation, const QMetaType &metaType,
                         const void *iterable)
        : QIterable(metaAssociation, metaType.alignOf(), iterable)
    {
    }

    QAssociativeIterable(QIterable<QMetaAssociation> &&other) : QIterable(std::move(other)) {}

    QAssociativeIterable &operator=(QIterable<QMetaAssociation> &&other)
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

    const_iterator find(const QVariant &key) const;
    const_iterator constFind(const QVariant &key) const { return find(key); }
    iterator mutableFind(const QVariant &key);

    bool containsKey(const QVariant &key);
    void insertKey(const QVariant &key);
    void removeKey(const QVariant &key);

    QVariant value(const QVariant &key) const;
    void setValue(const QVariant &key, const QVariant &mapped);
};

template<>
inline QVariantRef<QAssociativeIterator>::operator QVariant() const
{
    if (m_pointer == nullptr)
        return QVariant();

    const auto metaAssociation = m_pointer->metaContainer();
    const QMetaType metaType(metaAssociation.mappedMetaType());
    if (!metaType.isValid())
        return m_pointer->key();

    QVariant v(metaType);
    metaAssociation.mappedAtIterator(m_pointer->constIterator(),
                                     metaType == QMetaType::fromType<QVariant>() ? &v : v.data());
    return v;
}

template<>
inline QVariantRef<QAssociativeIterator> &QVariantRef<QAssociativeIterator>::operator=(
        const QVariant &value)
{
    if (m_pointer == nullptr)
        return *this;

    const auto metaAssociation = m_pointer->metaContainer();
    const QMetaType metaType(metaAssociation.mappedMetaType());
    if (metaType.isValid()) {
        QtPrivate::QVariantTypeCoercer coercer;
        metaAssociation.setMappedAtIterator(m_pointer->constIterator(),
                                            coercer.coerce(value, metaType));
    }

    return *this;
}

Q_DECLARE_TYPEINFO(QAssociativeIterable, Q_RELOCATABLE_TYPE);
Q_DECLARE_TYPEINFO(QAssociativeIterable::iterator, Q_RELOCATABLE_TYPE);
Q_DECLARE_TYPEINFO(QAssociativeIterable::const_iterator, Q_RELOCATABLE_TYPE);

QT_END_NAMESPACE

#endif // QASSOCIATIVEITERABLE_H
