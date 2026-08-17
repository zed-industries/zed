// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTAGGEDPOINTER_H
#define QTAGGEDPOINTER_H

#include <QtCore/qglobal.h>
#include <QtCore/qalgorithms.h>
#include <QtCore/qmath.h>
#include <QtCore/qtypeinfo.h>

QT_BEGIN_NAMESPACE

namespace QtPrivate {
    constexpr quint8 nextByteSize(quint8 bits) { return quint8((bits + 7) / 8); }

    template <typename T>
    struct TagInfo
    {
        static constexpr size_t alignment = alignof(T);
        static_assert((alignment & (alignment - 1)) == 0,
            "Alignment of template parameter must be power of two");

        static constexpr quint8 tagBits = QtPrivate::qConstexprCountTrailingZeroBits(alignment);
        static_assert(tagBits > 0,
            "Alignment of template parameter does not allow any tags");

        static constexpr size_t tagSize = QtPrivate::qConstexprNextPowerOfTwo(nextByteSize(tagBits));
        static_assert(tagSize < sizeof(quintptr),
            "Alignment of template parameter allows tags masking away pointer");

        using TagType = typename QIntegerForSize<tagSize>::Unsigned;
    };
}

template <typename T, typename Tag = typename QtPrivate::TagInfo<T>::TagType>
class QTaggedPointer
{
public:
    using Type = T;
    using TagType = Tag;

    static constexpr quintptr tagMask() { return QtPrivate::TagInfo<T>::alignment - 1; }
    static constexpr quintptr pointerMask() { return ~tagMask(); }

    constexpr QTaggedPointer() noexcept : d(0) {}
    constexpr QTaggedPointer(std::nullptr_t) noexcept : QTaggedPointer() {}

    explicit QTaggedPointer(T *pointer, Tag tag = Tag()) noexcept
        : d(quintptr(pointer) | quintptr(tag))
    {
        static_assert(sizeof(Type*) == sizeof(QTaggedPointer));

        Q_ASSERT_X((quintptr(pointer) & tagMask()) == 0, "QTaggedPointer<T, Tag>", "Pointer is not aligned");
        Q_ASSERT_X((static_cast<typename QtPrivate::TagInfo<T>::TagType>(tag) & pointerMask()) == 0,
            "QTaggedPointer<T, Tag>::setTag", "Tag is larger than allowed by number of available tag bits");
    }

    Type &operator*() const noexcept
    {
        Q_ASSERT(data());
        return *data();
    }

    Type *operator->() const noexcept
    {
        return data();
    }

    explicit operator bool() const noexcept
    {
        return !isNull();
    }

#ifdef Q_QDOC
    QTaggedPointer &operator=(T *other) noexcept;
#else
    // Disables the usage of `ptr = {}`, which would go through this operator
    // (rather than using the implicitly-generated assignment operator).
    // The operators have different semantics: the ones here leave the tag intact,
    // the implicitly-generated one overwrites it.
    template <typename U,
              std::enable_if_t<std::is_convertible_v<U *, T *>, bool> = false>
    QTaggedPointer &operator=(U *other) noexcept
    {
        T *otherT = other;
        d = reinterpret_cast<quintptr>(otherT) | (d & tagMask());
        return *this;
    }

    template <typename U,
              std::enable_if_t<std::is_null_pointer_v<U>, bool> = false>
    QTaggedPointer &operator=(U) noexcept
    {
        d = reinterpret_cast<quintptr>(static_cast<T *>(nullptr)) | (d & tagMask());
        return *this;
    }
#endif

    static constexpr Tag maximumTag() noexcept
    {
        return TagType(typename QtPrivate::TagInfo<T>::TagType(tagMask()));
    }

    void setTag(Tag tag)
    {
        Q_ASSERT_X((static_cast<typename QtPrivate::TagInfo<T>::TagType>(tag) & pointerMask()) == 0,
            "QTaggedPointer<T, Tag>::setTag", "Tag is larger than allowed by number of available tag bits");

        d = (d & pointerMask()) | static_cast<quintptr>(tag);
    }

    Tag tag() const noexcept
    {
        return TagType(typename QtPrivate::TagInfo<T>::TagType(d & tagMask()));
    }

    T* data() const noexcept
    {
        return reinterpret_cast<T*>(d & pointerMask());
    }

    bool isNull() const noexcept
    {
        return !data();
    }

    void swap(QTaggedPointer &other) noexcept
    {
        std::swap(d, other.d);
    }

    friend inline bool operator==(QTaggedPointer lhs, QTaggedPointer rhs) noexcept
    {
        return lhs.data() == rhs.data();
    }

    friend inline bool operator!=(QTaggedPointer lhs, QTaggedPointer rhs) noexcept
    {
        return lhs.data() != rhs.data();
    }

    friend inline bool operator==(QTaggedPointer lhs, std::nullptr_t) noexcept
    {
        return lhs.isNull();
    }

    friend inline bool operator==(std::nullptr_t, QTaggedPointer rhs) noexcept
    {
        return rhs.isNull();
    }

    friend inline bool operator!=(QTaggedPointer lhs, std::nullptr_t) noexcept
    {
        return !lhs.isNull();
    }

    friend inline bool operator!=(std::nullptr_t, QTaggedPointer rhs) noexcept
    {
        return !rhs.isNull();
    }

    friend inline bool operator!(QTaggedPointer ptr) noexcept
    {
        return !ptr.data();
    }

    friend inline void swap(QTaggedPointer &p1, QTaggedPointer &p2) noexcept
    {
        p1.swap(p2);
    }

protected:
    quintptr d;
};

template <typename T, typename Tag>
constexpr inline std::size_t qHash(QTaggedPointer<T, Tag> p, std::size_t seed = 0) noexcept
{ return qHash(p.data(), seed); }

template <typename T, typename Tag>
class QTypeInfo<QTaggedPointer<T, Tag>>
    : public QTypeInfoMerger<QTaggedPointer<T, Tag>, quintptr> {};

QT_END_NAMESPACE

#endif // QTAGGEDPOINTER_H
