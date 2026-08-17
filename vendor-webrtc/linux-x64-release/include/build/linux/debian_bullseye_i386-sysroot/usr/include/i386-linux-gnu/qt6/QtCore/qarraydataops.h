// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QARRAYDATAOPS_H
#define QARRAYDATAOPS_H

#include <QtCore/qarraydata.h>
#include <QtCore/qcontainertools_impl.h>

#include <memory>
#include <new>
#include <string.h>
#include <utility>
#include <iterator>
#include <tuple>
#include <type_traits>

QT_BEGIN_NAMESPACE

template <class T> struct QArrayDataPointer;

namespace QtPrivate {

template <class T>
struct QPodArrayOps
        : public QArrayDataPointer<T>
{
    static_assert (std::is_nothrow_destructible_v<T>, "Types with throwing destructors are not supported in Qt containers.");

protected:
    typedef QTypedArrayData<T> Data;
    using DataPointer = QArrayDataPointer<T>;

public:
    typedef typename QArrayDataPointer<T>::parameter_type parameter_type;

    void appendInitialize(qsizetype newSize) noexcept
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(!this->isShared());
        Q_ASSERT(newSize > this->size);
        Q_ASSERT(newSize - this->size <= this->freeSpaceAtEnd());

        T *where = this->end();
        this->size = newSize;
        const T *e = this->end();
        while (where != e)
            *where++ = T();
    }

    void copyAppend(const T *b, const T *e) noexcept
    {
        Q_ASSERT(this->isMutable() || b == e);
        Q_ASSERT(!this->isShared() || b == e);
        Q_ASSERT(b <= e);
        Q_ASSERT((e - b) <= this->freeSpaceAtEnd());

        if (b == e)
            return;

        ::memcpy(static_cast<void *>(this->end()), static_cast<const void *>(b), (e - b) * sizeof(T));
        this->size += (e - b);
    }

    void copyAppend(qsizetype n, parameter_type t) noexcept
    {
        Q_ASSERT(!this->isShared() || n == 0);
        Q_ASSERT(this->freeSpaceAtEnd() >= n);
        if (!n)
            return;

        T *where = this->end();
        this->size += qsizetype(n);
        while (n--)
            *where++ = t;
    }

    void moveAppend(T *b, T *e) noexcept
    {
        copyAppend(b, e);
    }

    void truncate(size_t newSize) noexcept
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(!this->isShared());
        Q_ASSERT(newSize < size_t(this->size));

        this->size = qsizetype(newSize);
    }

    void destroyAll() noexcept // Call from destructors, ONLY!
    {
        Q_ASSERT(this->d);
        Q_ASSERT(this->d->ref_.loadRelaxed() == 0);

        // As this is to be called only from destructor, it doesn't need to be
        // exception safe; size not updated.
    }

    T *createHole(QArrayData::GrowthPosition pos, qsizetype where, qsizetype n)
    {
        Q_ASSERT((pos == QArrayData::GrowsAtBeginning && n <= this->freeSpaceAtBegin()) ||
                 (pos == QArrayData::GrowsAtEnd && n <= this->freeSpaceAtEnd()));

        T *insertionPoint = this->ptr + where;
        if (pos == QArrayData::GrowsAtEnd) {
            if (where < this->size)
                ::memmove(static_cast<void *>(insertionPoint + n), static_cast<void *>(insertionPoint), (this->size - where) * sizeof(T));
        } else {
            Q_ASSERT(where == 0);
            this->ptr -= n;
            insertionPoint -= n;
        }
        this->size += n;
        return insertionPoint;
    }

    void insert(qsizetype i, const T *data, qsizetype n)
    {
        typename Data::GrowthPosition pos = Data::GrowsAtEnd;
        if (this->size != 0 && i == 0)
            pos = Data::GrowsAtBeginning;

        DataPointer oldData;
        this->detachAndGrow(pos, n, &data, &oldData);
        Q_ASSERT((pos == Data::GrowsAtBeginning && this->freeSpaceAtBegin() >= n) ||
                 (pos == Data::GrowsAtEnd && this->freeSpaceAtEnd() >= n));

        T *where = createHole(pos, i, n);
        ::memcpy(static_cast<void *>(where), static_cast<const void *>(data), n * sizeof(T));
    }

    void insert(qsizetype i, qsizetype n, parameter_type t)
    {
        T copy(t);

        typename Data::GrowthPosition pos = Data::GrowsAtEnd;
        if (this->size != 0 && i == 0)
            pos = Data::GrowsAtBeginning;

        this->detachAndGrow(pos, n, nullptr, nullptr);
        Q_ASSERT((pos == Data::GrowsAtBeginning && this->freeSpaceAtBegin() >= n) ||
                 (pos == Data::GrowsAtEnd && this->freeSpaceAtEnd() >= n));

        T *where = createHole(pos, i, n);
        while (n--)
            *where++ = copy;
    }

    template<typename... Args>
    void emplace(qsizetype i, Args &&... args)
    {
        bool detach = this->needsDetach();
        if (!detach) {
            if (i == this->size && this->freeSpaceAtEnd()) {
                new (this->end()) T(std::forward<Args>(args)...);
                ++this->size;
                return;
            }
            if (i == 0 && this->freeSpaceAtBegin()) {
                new (this->begin() - 1) T(std::forward<Args>(args)...);
                --this->ptr;
                ++this->size;
                return;
            }
        }
        T tmp(std::forward<Args>(args)...);
        typename QArrayData::GrowthPosition pos = QArrayData::GrowsAtEnd;
        if (this->size != 0 && i == 0)
            pos = QArrayData::GrowsAtBeginning;

        this->detachAndGrow(pos, 1, nullptr, nullptr);

        T *where = createHole(pos, i, 1);
        new (where) T(std::move(tmp));
    }

    void erase(T *b, qsizetype n)
    {
        T *e = b + n;
        Q_ASSERT(this->isMutable());
        Q_ASSERT(b < e);
        Q_ASSERT(b >= this->begin() && b < this->end());
        Q_ASSERT(e > this->begin() && e <= this->end());

        // Comply with std::vector::erase(): erased elements and all after them
        // are invalidated. However, erasing from the beginning effectively
        // means that all iterators are invalidated. We can use this freedom to
        // erase by moving towards the end.
        if (b == this->begin() && e != this->end()) {
            this->ptr = e;
        } else if (e != this->end()) {
            ::memmove(static_cast<void *>(b), static_cast<void *>(e),
                      (static_cast<T *>(this->end()) - e) * sizeof(T));
        }
        this->size -= n;
    }

    void eraseFirst() noexcept
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(this->size);
        ++this->ptr;
        --this->size;
    }

    void eraseLast() noexcept
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(this->size);
        --this->size;
    }

    void assign(T *b, T *e, parameter_type t) noexcept
    {
        Q_ASSERT(b <= e);
        Q_ASSERT(b >= this->begin() && e <= this->end());

        while (b != e)
            ::memcpy(static_cast<void *>(b++), static_cast<const void *>(&t), sizeof(T));
    }

    bool compare(const T *begin1, const T *begin2, size_t n) const
    {
        // only use memcmp for fundamental types or pointers.
        // Other types could have padding in the data structure or custom comparison
        // operators that would break the comparison using memcmp
        if constexpr (QArrayDataPointer<T>::pass_parameter_by_value) {
            return ::memcmp(begin1, begin2, n * sizeof(T)) == 0;
        } else {
            const T *end1 = begin1 + n;
            while (begin1 != end1) {
                if (*begin1 == *begin2) {
                    ++begin1;
                    ++begin2;
                } else {
                    return false;
                }
            }
            return true;
        }
    }

    void reallocate(qsizetype alloc, QArrayData::AllocationOption option)
    {
        auto pair = Data::reallocateUnaligned(this->d, this->ptr, alloc, option);
        Q_CHECK_PTR(pair.second);
        Q_ASSERT(pair.first != nullptr);
        this->d = pair.first;
        this->ptr = pair.second;
    }
};

template <class T>
struct QGenericArrayOps
        : public QArrayDataPointer<T>
{
    static_assert (std::is_nothrow_destructible_v<T>, "Types with throwing destructors are not supported in Qt containers.");

protected:
    typedef QTypedArrayData<T> Data;
    using DataPointer = QArrayDataPointer<T>;

public:
    typedef typename QArrayDataPointer<T>::parameter_type parameter_type;

    void appendInitialize(qsizetype newSize)
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(!this->isShared());
        Q_ASSERT(newSize > this->size);
        Q_ASSERT(newSize - this->size <= this->freeSpaceAtEnd());

        T *const b = this->begin();
        do {
            new (b + this->size) T;
        } while (++this->size != newSize);
    }

    void copyAppend(const T *b, const T *e)
    {
        Q_ASSERT(this->isMutable() || b == e);
        Q_ASSERT(!this->isShared() || b == e);
        Q_ASSERT(b <= e);
        Q_ASSERT((e - b) <= this->freeSpaceAtEnd());

        if (b == e) // short-cut and handling the case b and e == nullptr
            return;

        T *data = this->begin();
        while (b < e) {
            new (data + this->size) T(*b);
            ++b;
            ++this->size;
        }
    }

    void copyAppend(qsizetype n, parameter_type t)
    {
        Q_ASSERT(!this->isShared() || n == 0);
        Q_ASSERT(this->freeSpaceAtEnd() >= n);
        if (!n)
            return;

        T *data = this->begin();
        while (n--) {
            new (data + this->size) T(t);
            ++this->size;
        }
    }

    void moveAppend(T *b, T *e)
    {
        Q_ASSERT(this->isMutable() || b == e);
        Q_ASSERT(!this->isShared() || b == e);
        Q_ASSERT(b <= e);
        Q_ASSERT((e - b) <= this->freeSpaceAtEnd());

        if (b == e)
            return;

        T *data = this->begin();
        while (b < e) {
            new (data + this->size) T(std::move(*b));
            ++b;
            ++this->size;
        }
    }

    void truncate(size_t newSize)
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(!this->isShared());
        Q_ASSERT(newSize < size_t(this->size));

        std::destroy(this->begin() + newSize, this->end());
        this->size = newSize;
    }

    void destroyAll() // Call from destructors, ONLY
    {
        Q_ASSERT(this->d);
        // As this is to be called only from destructor, it doesn't need to be
        // exception safe; size not updated.

        Q_ASSERT(this->d->ref_.loadRelaxed() == 0);

        std::destroy(this->begin(), this->end());
    }

    struct Inserter
    {
        QArrayDataPointer<T> *data;
        T *begin;
        qsizetype size;

        qsizetype sourceCopyConstruct = 0, nSource = 0, move = 0, sourceCopyAssign = 0;
        T *end = nullptr, *last = nullptr, *where = nullptr;

        Inserter(QArrayDataPointer<T> *d) : data(d)
        {
            begin = d->ptr;
            size = d->size;
        }
        ~Inserter() {
            data->ptr = begin;
            data->size = size;
        }
        Q_DISABLE_COPY(Inserter)

        void setup(qsizetype pos, qsizetype n)
        {
            end = begin + size;
            last = end - 1;
            where = begin + pos;
            qsizetype dist = size - pos;
            sourceCopyConstruct = 0;
            nSource = n;
            move = n - dist; // smaller 0
            sourceCopyAssign = n;
            if (n > dist) {
                sourceCopyConstruct = n - dist;
                move = 0;
                sourceCopyAssign -= sourceCopyConstruct;
            }
        }

        void insert(qsizetype pos, const T *source, qsizetype n)
        {
            qsizetype oldSize = size;
            Q_UNUSED(oldSize);

            setup(pos, n);

            // first create new elements at the end, by copying from elements
            // to be inserted (if they extend past the current end of the array)
            for (qsizetype i = 0; i != sourceCopyConstruct; ++i) {
                new (end + i) T(source[nSource - sourceCopyConstruct + i]);
                ++size;
            }
            Q_ASSERT(size <= oldSize + n);

            // now move construct new elements at the end from existing elements inside
            // the array.
            for (qsizetype i = sourceCopyConstruct; i != nSource; ++i) {
                new (end + i) T(std::move(*(end + i - nSource)));
                ++size;
            }
            // array has the new size now!
            Q_ASSERT(size == oldSize + n);

            // now move assign existing elements towards the end
            for (qsizetype i = 0; i != move; --i)
                last[i] = std::move(last[i - nSource]);

            // finally copy the remaining elements from source over
            for (qsizetype i = 0; i != sourceCopyAssign; ++i)
                where[i] = source[i];
        }

        void insert(qsizetype pos, const T &t, qsizetype n)
        {
            const qsizetype oldSize = size;
            Q_UNUSED(oldSize);

            setup(pos, n);

            // first create new elements at the end, by copying from elements
            // to be inserted (if they extend past the current end of the array)
            for (qsizetype i = 0; i != sourceCopyConstruct; ++i) {
                new (end + i) T(t);
                ++size;
            }
            Q_ASSERT(size <= oldSize + n);

            // now move construct new elements at the end from existing elements inside
            // the array.
            for (qsizetype i = sourceCopyConstruct; i != nSource; ++i) {
                new (end + i) T(std::move(*(end + i - nSource)));
                ++size;
            }
            // array has the new size now!
            Q_ASSERT(size == oldSize + n);

            // now move assign existing elements towards the end
            for (qsizetype i = 0; i != move; --i)
                last[i] = std::move(last[i - nSource]);

            // finally copy the remaining elements from source over
            for (qsizetype i = 0; i != sourceCopyAssign; ++i)
                where[i] = t;
        }

        void insertOne(qsizetype pos, T &&t)
        {
            setup(pos, 1);

            if (sourceCopyConstruct) {
                Q_ASSERT(sourceCopyConstruct == 1);
                new (end) T(std::move(t));
                ++size;
            } else {
                // create a new element at the end by move constructing one existing element
                // inside the array.
                new (end) T(std::move(*(end - 1)));
                ++size;

                // now move assign existing elements towards the end
                for (qsizetype i = 0; i != move; --i)
                    last[i] = std::move(last[i - 1]);

                // and move the new item into place
                *where = std::move(t);
            }
        }
    };

    void insert(qsizetype i, const T *data, qsizetype n)
    {
        const bool growsAtBegin = this->size != 0 && i == 0;
        const auto pos = growsAtBegin ? Data::GrowsAtBeginning : Data::GrowsAtEnd;

        DataPointer oldData;
        this->detachAndGrow(pos, n, &data, &oldData);
        Q_ASSERT((pos == Data::GrowsAtBeginning && this->freeSpaceAtBegin() >= n) ||
                 (pos == Data::GrowsAtEnd && this->freeSpaceAtEnd() >= n));

        if (growsAtBegin) {
            // copy construct items in reverse order at the begin
            Q_ASSERT(this->freeSpaceAtBegin() >= n);
            while (n) {
                --n;
                new (this->begin() - 1) T(data[n]);
                --this->ptr;
                ++this->size;
            }
        } else {
            Inserter(this).insert(i, data, n);
        }
    }

    void insert(qsizetype i, qsizetype n, parameter_type t)
    {
        T copy(t);

        const bool growsAtBegin = this->size != 0 && i == 0;
        const auto pos = growsAtBegin ? Data::GrowsAtBeginning : Data::GrowsAtEnd;

        this->detachAndGrow(pos, n, nullptr, nullptr);
        Q_ASSERT((pos == Data::GrowsAtBeginning && this->freeSpaceAtBegin() >= n) ||
                 (pos == Data::GrowsAtEnd && this->freeSpaceAtEnd() >= n));

        if (growsAtBegin) {
            // copy construct items in reverse order at the begin
            Q_ASSERT(this->freeSpaceAtBegin() >= n);
            while (n--) {
                new (this->begin() - 1) T(copy);
                --this->ptr;
                ++this->size;
            }
        } else {
            Inserter(this).insert(i, copy, n);
        }
    }

    template<typename... Args>
    void emplace(qsizetype i, Args &&... args)
    {
        bool detach = this->needsDetach();
        if (!detach) {
            if (i == this->size && this->freeSpaceAtEnd()) {
                new (this->end()) T(std::forward<Args>(args)...);
                ++this->size;
                return;
            }
            if (i == 0 && this->freeSpaceAtBegin()) {
                new (this->begin() - 1) T(std::forward<Args>(args)...);
                --this->ptr;
                ++this->size;
                return;
            }
        }
        T tmp(std::forward<Args>(args)...);
        const bool growsAtBegin = this->size != 0 && i == 0;
        const auto pos = growsAtBegin ? Data::GrowsAtBeginning : Data::GrowsAtEnd;

        this->detachAndGrow(pos, 1, nullptr, nullptr);

        if (growsAtBegin) {
            Q_ASSERT(this->freeSpaceAtBegin());
            new (this->begin() - 1) T(std::move(tmp));
            --this->ptr;
            ++this->size;
        } else {
            Inserter(this).insertOne(i, std::move(tmp));
        }
    }

    void erase(T *b, qsizetype n)
    {
        T *e = b + n;
        Q_ASSERT(this->isMutable());
        Q_ASSERT(b < e);
        Q_ASSERT(b >= this->begin() && b < this->end());
        Q_ASSERT(e > this->begin() && e <= this->end());

        // Comply with std::vector::erase(): erased elements and all after them
        // are invalidated. However, erasing from the beginning effectively
        // means that all iterators are invalidated. We can use this freedom to
        // erase by moving towards the end.
        if (b == this->begin() && e != this->end()) {
            this->ptr = e;
        } else {
            const T *const end = this->end();

            // move (by assignment) the elements from e to end
            // onto b to the new end
            while (e != end) {
                *b = std::move(*e);
                ++b;
                ++e;
            }
        }
        this->size -= n;
        std::destroy(b, e);
    }

    void eraseFirst() noexcept
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(this->size);
        this->begin()->~T();
        ++this->ptr;
        --this->size;
    }

    void eraseLast() noexcept
    {
        Q_ASSERT(this->isMutable());
        Q_ASSERT(this->size);
        (this->end() - 1)->~T();
        --this->size;
    }


    void assign(T *b, T *e, parameter_type t)
    {
        Q_ASSERT(b <= e);
        Q_ASSERT(b >= this->begin() && e <= this->end());

        while (b != e)
            *b++ = t;
    }

    bool compare(const T *begin1, const T *begin2, size_t n) const
    {
        const T *end1 = begin1 + n;
        while (begin1 != end1) {
            if (*begin1 == *begin2) {
                ++begin1;
                ++begin2;
            } else {
                return false;
            }
        }
        return true;
    }
};

template <class T>
struct QMovableArrayOps
    : QGenericArrayOps<T>
{
    static_assert (std::is_nothrow_destructible_v<T>, "Types with throwing destructors are not supported in Qt containers.");

protected:
    typedef QTypedArrayData<T> Data;
    using DataPointer = QArrayDataPointer<T>;

public:
    // using QGenericArrayOps<T>::copyAppend;
    // using QGenericArrayOps<T>::moveAppend;
    // using QGenericArrayOps<T>::truncate;
    // using QGenericArrayOps<T>::destroyAll;
    typedef typename QGenericArrayOps<T>::parameter_type parameter_type;

    struct Inserter
    {
        QArrayDataPointer<T> *data;
        T *displaceFrom;
        T *displaceTo;
        qsizetype nInserts = 0;
        qsizetype bytes;

        Inserter(QArrayDataPointer<T> *d) : data(d) { }
        ~Inserter() {
            if constexpr (!std::is_nothrow_copy_constructible_v<T>) {
                if (displaceFrom != displaceTo) {
                    ::memmove(static_cast<void *>(displaceFrom), static_cast<void *>(displaceTo), bytes);
                    nInserts -= qAbs(displaceFrom - displaceTo);
                }
            }
            data->size += nInserts;
        }
        Q_DISABLE_COPY(Inserter)

        T *displace(qsizetype pos, qsizetype n)
        {
            nInserts = n;
            T *insertionPoint = data->ptr + pos;
            displaceFrom = data->ptr + pos;
            displaceTo = displaceFrom + n;
            bytes = data->size - pos;
            bytes *= sizeof(T);
            ::memmove(static_cast<void *>(displaceTo), static_cast<void *>(displaceFrom), bytes);
            return insertionPoint;
        }

        void insert(qsizetype pos, const T *source, qsizetype n)
        {
            T *where = displace(pos, n);

            while (n--) {
                new (where) T(*source);
                ++where;
                ++source;
                ++displaceFrom;
            }
        }

        void insert(qsizetype pos, const T &t, qsizetype n)
        {
            T *where = displace(pos, n);

            while (n--) {
                new (where) T(t);
                ++where;
                ++displaceFrom;
            }
        }

        void insertOne(qsizetype pos, T &&t)
        {
            T *where = displace(pos, 1);
            new (where) T(std::move(t));
            ++displaceFrom;
            Q_ASSERT(displaceFrom == displaceTo);
        }

    };


    void insert(qsizetype i, const T *data, qsizetype n)
    {
        const bool growsAtBegin = this->size != 0 && i == 0;
        const auto pos = growsAtBegin ? Data::GrowsAtBeginning : Data::GrowsAtEnd;

        DataPointer oldData;
        this->detachAndGrow(pos, n, &data, &oldData);
        Q_ASSERT((pos == Data::GrowsAtBeginning && this->freeSpaceAtBegin() >= n) ||
                 (pos == Data::GrowsAtEnd && this->freeSpaceAtEnd() >= n));

        if (growsAtBegin) {
            // copy construct items in reverse order at the begin
            Q_ASSERT(this->freeSpaceAtBegin() >= n);
            while (n) {
                --n;
                new (this->begin() - 1) T(data[n]);
                --this->ptr;
                ++this->size;
            }
        } else {
            Inserter(this).insert(i, data, n);
        }
    }

    void insert(qsizetype i, qsizetype n, parameter_type t)
    {
        T copy(t);

        const bool growsAtBegin = this->size != 0 && i == 0;
        const auto pos = growsAtBegin ? Data::GrowsAtBeginning : Data::GrowsAtEnd;

        this->detachAndGrow(pos, n, nullptr, nullptr);
        Q_ASSERT((pos == Data::GrowsAtBeginning && this->freeSpaceAtBegin() >= n) ||
                 (pos == Data::GrowsAtEnd && this->freeSpaceAtEnd() >= n));

        if (growsAtBegin) {
            // copy construct items in reverse order at the begin
            Q_ASSERT(this->freeSpaceAtBegin() >= n);
            while (n--) {
                new (this->begin() - 1) T(copy);
                --this->ptr;
                ++this->size;
            }
        } else {
            Inserter(this).insert(i, copy, n);
        }
    }

    template<typename... Args>
    void emplace(qsizetype i, Args &&... args)
    {
        bool detach = this->needsDetach();
        if (!detach) {
            if (i == this->size && this->freeSpaceAtEnd()) {
                new (this->end()) T(std::forward<Args>(args)...);
                ++this->size;
                return;
            }
            if (i == 0 && this->freeSpaceAtBegin()) {
                new (this->begin() - 1) T(std::forward<Args>(args)...);
                --this->ptr;
                ++this->size;
                return;
            }
        }
        T tmp(std::forward<Args>(args)...);
        const bool growsAtBegin = this->size != 0 && i == 0;
        const auto pos = growsAtBegin ? Data::GrowsAtBeginning : Data::GrowsAtEnd;

        this->detachAndGrow(pos, 1, nullptr, nullptr);
        if (growsAtBegin) {
            Q_ASSERT(this->freeSpaceAtBegin());
            new (this->begin() - 1) T(std::move(tmp));
            --this->ptr;
            ++this->size;
        } else {
            Inserter(this).insertOne(i, std::move(tmp));
        }
    }

    void erase(T *b, qsizetype n)
    {
        T *e = b + n;

        Q_ASSERT(this->isMutable());
        Q_ASSERT(b < e);
        Q_ASSERT(b >= this->begin() && b < this->end());
        Q_ASSERT(e > this->begin() && e <= this->end());

        // Comply with std::vector::erase(): erased elements and all after them
        // are invalidated. However, erasing from the beginning effectively
        // means that all iterators are invalidated. We can use this freedom to
        // erase by moving towards the end.

        std::destroy(b, e);
        if (b == this->begin() && e != this->end()) {
            this->ptr = e;
        } else if (e != this->end()) {
            memmove(static_cast<void *>(b), static_cast<const void *>(e), (static_cast<const T *>(this->end()) - e)*sizeof(T));
        }
        this->size -= n;
    }

    void reallocate(qsizetype alloc, QArrayData::AllocationOption option)
    {
        auto pair = Data::reallocateUnaligned(this->d, this->ptr, alloc, option);
        Q_CHECK_PTR(pair.second);
        Q_ASSERT(pair.first != nullptr);
        this->d = pair.first;
        this->ptr = pair.second;
    }
};

template <class T, class = void>
struct QArrayOpsSelector
{
    typedef QGenericArrayOps<T> Type;
};

template <class T>
struct QArrayOpsSelector<T,
    typename std::enable_if<
        !QTypeInfo<T>::isComplex && QTypeInfo<T>::isRelocatable
    >::type>
{
    typedef QPodArrayOps<T> Type;
};

template <class T>
struct QArrayOpsSelector<T,
    typename std::enable_if<
        QTypeInfo<T>::isComplex && QTypeInfo<T>::isRelocatable
    >::type>
{
    typedef QMovableArrayOps<T> Type;
};

template <class T>
struct QCommonArrayOps : QArrayOpsSelector<T>::Type
{
    using Base = typename QArrayOpsSelector<T>::Type;
    using Data = QTypedArrayData<T>;
    using DataPointer = QArrayDataPointer<T>;
    using parameter_type = typename Base::parameter_type;

protected:
    using Self = QCommonArrayOps<T>;

public:
    // using Base::truncate;
    // using Base::destroyAll;
    // using Base::assign;
    // using Base::compare;

    template<typename It>
    void appendIteratorRange(It b, It e, QtPrivate::IfIsForwardIterator<It> = true)
    {
        Q_ASSERT(this->isMutable() || b == e);
        Q_ASSERT(!this->isShared() || b == e);
        const qsizetype distance = std::distance(b, e);
        Q_ASSERT(distance >= 0 && distance <= this->allocatedCapacity() - this->size);
        Q_UNUSED(distance);

#if __cplusplus >= 202002L && defined(__cpp_concepts) && defined(__cpp_lib_concepts)
        constexpr bool canUseCopyAppend =
                std::contiguous_iterator<It> &&
                std::is_same_v<
                    std::remove_cv_t<typename std::iterator_traits<It>::value_type>,
                    T
                >;
        if constexpr (canUseCopyAppend) {
            this->copyAppend(std::to_address(b), std::to_address(e));
        } else
#endif
        {
            T *iter = this->end();
            for (; b != e; ++iter, ++b) {
                new (iter) T(*b);
                ++this->size;
            }
        }
    }

    // slightly higher level API than copyAppend() that also preallocates space
    void growAppend(const T *b, const T *e)
    {
        if (b == e)
            return;
        Q_ASSERT(b < e);
        const qsizetype n = e - b;
        DataPointer old;

        // points into range:
        if (QtPrivate::q_points_into_range(b, this->begin(), this->end())) {
            this->detachAndGrow(QArrayData::GrowsAtEnd, n, &b, &old);
        } else {
            this->detachAndGrow(QArrayData::GrowsAtEnd, n, nullptr, nullptr);
        }
        Q_ASSERT(this->freeSpaceAtEnd() >= n);
        // b might be updated so use [b, n)
        this->copyAppend(b, b + n);
    }
};

} // namespace QtPrivate

template <class T>
struct QArrayDataOps
    : QtPrivate::QCommonArrayOps<T>
{
};

QT_END_NAMESPACE

#endif // include guard
