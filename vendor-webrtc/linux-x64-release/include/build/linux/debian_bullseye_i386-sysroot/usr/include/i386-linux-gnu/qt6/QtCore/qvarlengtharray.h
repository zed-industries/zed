// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QVARLENGTHARRAY_H
#define QVARLENGTHARRAY_H

#if 0
#pragma qt_class(QVarLengthArray)
#pragma qt_sync_stop_processing
#endif

#include <QtCore/qcontainerfwd.h>
#include <QtCore/qglobal.h>
#include <QtCore/qalgorithms.h>
#include <QtCore/qcontainertools_impl.h>
#include <QtCore/qhashfunctions.h>

#include <algorithm>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <new>

#include <string.h>
#include <stdlib.h>

QT_BEGIN_NAMESPACE

template <size_t Size, size_t Align, qsizetype Prealloc>
class QVLAStorage
{
    template <size_t> class print;
protected:
    ~QVLAStorage() = default;

    alignas(Align) char array[Prealloc * (Align > Size ? Align : Size)];
    QT_WARNING_PUSH
    QT_WARNING_DISABLE_DEPRECATED
    // ensure we maintain BC: std::aligned_storage_t was only specified by a
    // minimum size, but for BC we need the substitution to be exact in size:
    static_assert(std::is_same_v<print<sizeof(std::aligned_storage_t<Size, Align>[Prealloc])>,
                                 print<sizeof(array)>>);
    QT_WARNING_POP
};

class QVLABaseBase
{
protected:
    ~QVLABaseBase() = default;

    qsizetype a;      // capacity
    qsizetype s;      // size
    void *ptr;     // data

    Q_ALWAYS_INLINE constexpr void verify(qsizetype pos = 0, qsizetype n = 1) const
    {
        Q_ASSERT(pos >= 0);
        Q_ASSERT(pos <= size());
        Q_ASSERT(n >= 0);
        Q_ASSERT(n <= size() - pos);
    }

    struct free_deleter {
        void operator()(void *p) const noexcept { free(p); }
    };
    using malloced_ptr = std::unique_ptr<void, free_deleter>;

public:
    using size_type = qsizetype;

    constexpr size_type capacity() const noexcept { return a; }
    constexpr size_type size() const noexcept { return s; }
    constexpr bool empty() const noexcept { return size() == 0; }
};

template<class T>
class QVLABase : public QVLABaseBase
{
protected:
    ~QVLABase() = default;

public:
    T *data() noexcept { return static_cast<T *>(ptr); }
    const T *data() const noexcept { return static_cast<T *>(ptr); }

    using iterator = T*;
    using const_iterator = const T*;

    iterator begin() noexcept { return data(); }
    const_iterator begin() const noexcept { return data(); }
    const_iterator cbegin() const noexcept { return begin(); }
    iterator end() noexcept { return data() + size(); }
    const_iterator end() const noexcept { return data() + size(); }
    const_iterator cend() const noexcept { return end(); }

    using reverse_iterator = std::reverse_iterator<iterator>;
    using const_reverse_iterator = std::reverse_iterator<const_iterator>;

    reverse_iterator rbegin() noexcept { return reverse_iterator{end()}; }
    const_reverse_iterator rbegin() const noexcept { return const_reverse_iterator{end()}; }
    const_reverse_iterator crbegin() const noexcept { return rbegin(); }
    reverse_iterator rend() noexcept { return reverse_iterator{begin()}; }
    const_reverse_iterator rend() const noexcept { return const_reverse_iterator{begin()}; }
    const_reverse_iterator crend() const noexcept { return rend(); }

    using value_type = T;
    using reference = value_type&;
    using const_reference = const value_type&;
    using pointer = value_type*;
    using const_pointer = const value_type*;
    using difference_type = qptrdiff;

    reference front()
    {
        verify();
        return *begin();
    }

    const_reference front() const
    {
        verify();
        return *begin();
    }

    reference back()
    {
        verify();
        return *rbegin();
    }

    const_reference back() const
    {
        verify();
        return *rbegin();
    }

    void pop_back()
    {
        verify();
        if constexpr (QTypeInfo<T>::isComplex)
            data()[size() - 1].~T();
        --s;
    }

    template <typename AT = T>
    qsizetype indexOf(const AT &t, qsizetype from = 0) const;
    template <typename AT = T>
    qsizetype lastIndexOf(const AT &t, qsizetype from = -1) const;
    template <typename AT = T>
    bool contains(const AT &t) const;

    reference operator[](qsizetype idx)
    {
        verify(idx);
        return data()[idx];
    }
    const_reference operator[](qsizetype idx) const
    {
        verify(idx);
        return data()[idx];
    }

    value_type value(qsizetype i) const;
    value_type value(qsizetype i, const T& defaultValue) const;

    void replace(qsizetype i, const T &t);
    void remove(qsizetype i, qsizetype n = 1);
    template <typename AT = T>
    qsizetype removeAll(const AT &t);
    template <typename AT = T>
    bool removeOne(const AT &t);
    template <typename Predicate>
    qsizetype removeIf(Predicate pred);

    iterator erase(const_iterator begin, const_iterator end);
    iterator erase(const_iterator pos) { return erase(pos, pos + 1); }

    size_t hash(size_t seed) const noexcept(QtPrivate::QNothrowHashable_v<T>)
    {
        return qHashRange(begin(), end(), seed);
    }
protected:
    template <typename...Args>
    reference emplace_back_impl(qsizetype prealloc, void *array, Args&&...args)
    {
        if (size() == capacity()) // ie. size() != 0
            reallocate_impl(prealloc, array, size(), size() << 1);
        reference r = *new (end()) T(std::forward<Args>(args)...);
        ++s;
        return r;
    }
    template <typename...Args>
    iterator emplace_impl(qsizetype prealloc, void *array, const_iterator pos, Args&&...arg);

    iterator insert_impl(qsizetype prealloc, void *array, const_iterator pos, qsizetype n, const T &t);

    template <typename S>
    bool equal(const QVLABase<S> &other) const
    {
        return std::equal(begin(), end(), other.begin(), other.end());
    }
    template <typename S>
    bool less_than(const QVLABase<S> &other) const
    {
        return std::lexicographical_compare(begin(), end(), other.begin(), other.end());
    }

    void append_impl(qsizetype prealloc, void *array, const T *buf, qsizetype n);
    void reallocate_impl(qsizetype prealloc, void *array, qsizetype size, qsizetype alloc, const T *v = nullptr);
    void resize_impl(qsizetype prealloc, void *array, qsizetype sz, const T *v = nullptr)
    { reallocate_impl(prealloc, array, sz, qMax(sz, capacity()), v); }

    bool isValidIterator(const const_iterator &i) const
    {
        const std::less<const T *> less = {};
        return !less(cend(), i) && !less(i, cbegin());
    }
};

// Prealloc = 256 by default, specified in qcontainerfwd.h
template<class T, qsizetype Prealloc>
class QVarLengthArray
    : public QVLABase<T>, // ### Qt 7: swap base class order
      public QVLAStorage<sizeof(T), alignof(T), Prealloc>
{
    template <class S, qsizetype Prealloc2>
    friend class QVarLengthArray;
    using Base = QVLABase<T>;
    using Storage = QVLAStorage<sizeof(T), alignof(T), Prealloc>;
    static_assert(std::is_nothrow_destructible_v<T>, "Types with throwing destructors are not supported in Qt containers.");
    using Base::verify;

    template <typename U>
    using if_copyable = std::enable_if_t<std::is_copy_constructible_v<U>, bool>;
public:
    using size_type = typename Base::size_type;
    using value_type = typename Base::value_type;
    using pointer = typename Base::pointer;
    using const_pointer = typename Base::const_pointer;
    using reference = typename Base::reference;
    using const_reference = typename Base::const_reference;
    using difference_type = typename Base::difference_type;

    using iterator = typename Base::iterator;
    using const_iterator = typename Base::const_iterator;
    using reverse_iterator = typename Base::reverse_iterator;
    using const_reverse_iterator = typename Base::const_reverse_iterator;

    QVarLengthArray() noexcept
    {
        this->a = Prealloc;
        this->s = 0;
        this->ptr = this->array;
    }

    inline explicit QVarLengthArray(qsizetype size);

#ifndef Q_QDOC
    template <typename U = T, if_copyable<U> = true>
#endif
    explicit QVarLengthArray(qsizetype sz, const T &v)
        : QVarLengthArray{}
    {
        resize(sz, v);
    }

    QVarLengthArray(const QVarLengthArray &other)
        : QVarLengthArray{}
    {
        append(other.constData(), other.size());
    }

    QVarLengthArray(QVarLengthArray &&other)
            noexcept(std::is_nothrow_move_constructible_v<T>)
        : Base(other)
    {
        const auto otherInlineStorage = reinterpret_cast<T*>(other.array);
        if (data() == otherInlineStorage) {
            // inline buffer - move into our inline buffer:
            this->ptr = this->array;
            QtPrivate::q_uninitialized_relocate_n(otherInlineStorage, size(), data());
        } else {
            // heap buffer - we just stole the memory
        }
        // reset other to internal storage:
        other.a = Prealloc;
        other.s = 0;
        other.ptr = otherInlineStorage;
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

    inline ~QVarLengthArray()
    {
        if constexpr (QTypeInfo<T>::isComplex)
            std::destroy_n(data(), size());
        if (data() != reinterpret_cast<T *>(this->array))
            free(data());
    }
    inline QVarLengthArray<T, Prealloc> &operator=(const QVarLengthArray<T, Prealloc> &other)
    {
        if (this != &other) {
            clear();
            append(other.constData(), other.size());
        }
        return *this;
    }

    QVarLengthArray &operator=(QVarLengthArray &&other)
        noexcept(std::is_nothrow_move_constructible_v<T>)
    {
        // we're only required to be self-move-assignment-safe
        // when we're in the moved-from state (Hinnant criterion)
        // the moved-from state is the empty state, so we're good with the clear() here:
        clear();
        Q_ASSERT(capacity() >= Prealloc);
        const auto otherInlineStorage = other.array;
        if (other.ptr != otherInlineStorage) {
            // heap storage: steal the external buffer, reset other to otherInlineStorage
            this->a = std::exchange(other.a, Prealloc);
            this->ptr = std::exchange(other.ptr, otherInlineStorage);
        } else {
            // inline storage: move into our storage (doesn't matter whether inline or external)
            QtPrivate::q_uninitialized_relocate_n(other.data(), other.size(), data());
        }
        this->s = std::exchange(other.s, 0);
        return *this;
    }

    QVarLengthArray<T, Prealloc> &operator=(std::initializer_list<T> list)
    {
        resize(qsizetype(list.size()));
        std::copy(list.begin(), list.end(),
                  QT_MAKE_CHECKED_ARRAY_ITERATOR(begin(), size()));
        return *this;
    }

    inline void removeLast()
    {
        Base::pop_back();
    }
#ifdef Q_QDOC
    inline qsizetype size() const { return this->s; }
#endif
    using Base::size;
    inline qsizetype count() const { return size(); }
    inline qsizetype length() const { return size(); }
    inline T &first()
    {
        return front();
    }
    inline const T &first() const
    {
        return front();
    }
    T &last()
    {
        return back();
    }
    const T &last() const
    {
        return back();
    }
    bool isEmpty() const { return empty(); }
    void resize(qsizetype sz) { Base::resize_impl(Prealloc, this->array, sz); }
#ifndef Q_QDOC
    template <typename U = T, if_copyable<U> = true>
#endif
    void resize(qsizetype sz, const T &v)
    { Base::resize_impl(Prealloc, this->array, sz, &v); }
    inline void clear() { resize(0); }
    void squeeze() { reallocate(size(), size()); }

    using Base::capacity;
#ifdef Q_QDOC
    qsizetype capacity() const { return this->a; }
#endif
    void reserve(qsizetype sz) { if (sz > capacity()) reallocate(size(), sz); }

#ifdef Q_QDOC
    template <typename AT = T>
    inline qsizetype indexOf(const AT &t, qsizetype from = 0) const;
    template <typename AT = T>
    inline qsizetype lastIndexOf(const AT &t, qsizetype from = -1) const;
    template <typename AT = T>
    inline bool contains(const AT &t) const;
#endif
    using Base::indexOf;
    using Base::lastIndexOf;
    using Base::contains;

#ifdef Q_QDOC
    inline T &operator[](qsizetype idx)
    {
        verify(idx);
        return data()[idx];
    }
    inline const T &operator[](qsizetype idx) const
    {
        verify(idx);
        return data()[idx];
    }
#endif
    using Base::operator[];
    inline const T &at(qsizetype idx) const { return operator[](idx); }

#ifdef Q_QDOC
    T value(qsizetype i) const;
    T value(qsizetype i, const T &defaultValue) const;
#endif
    using Base::value;

    inline void append(const T &t)
    {
        if (size() == capacity())
            emplace_back(T(t));
        else
            emplace_back(t);
    }

    void append(T &&t)
    {
        emplace_back(std::move(t));
    }

    void append(const T *buf, qsizetype sz)
    { Base::append_impl(Prealloc, this->array, buf, sz); }
    inline QVarLengthArray<T, Prealloc> &operator<<(const T &t)
    { append(t); return *this; }
    inline QVarLengthArray<T, Prealloc> &operator<<(T &&t)
    { append(std::move(t)); return *this; }
    inline QVarLengthArray<T, Prealloc> &operator+=(const T &t)
    { append(t); return *this; }
    inline QVarLengthArray<T, Prealloc> &operator+=(T &&t)
    { append(std::move(t)); return *this; }

#if QT_DEPRECATED_SINCE(6, 3)
    QT_DEPRECATED_VERSION_X_6_3("This is slow. If you must, use insert(cbegin(), ~~~) instead.")
    void prepend(T &&t);
    QT_DEPRECATED_VERSION_X_6_3("This is slow. If you must, use insert(cbegin(), ~~~) instead.")
    void prepend(const T &t);
#endif
    void insert(qsizetype i, T &&t);
    void insert(qsizetype i, const T &t);
    void insert(qsizetype i, qsizetype n, const T &t);
#ifdef Q_QDOC
    void replace(qsizetype i, const T &t);
    void remove(qsizetype i, qsizetype n = 1);
    template <typename AT = T>
    qsizetype removeAll(const AT &t);
    template <typename AT = T>
    bool removeOne(const AT &t);
    template <typename Predicate>
    qsizetype removeIf(Predicate pred);
#endif
    using Base::replace;
    using Base::remove;
    using Base::removeAll;
    using Base::removeOne;
    using Base::removeIf;

#ifdef Q_QDOC
    inline T *data() { return this->ptr; }
    inline const T *data() const { return this->ptr; }
#endif
    using Base::data;
    inline const T *constData() const { return data(); }
#ifdef Q_QDOC
    inline iterator begin() { return data(); }
    inline const_iterator begin() const { return data(); }
    inline const_iterator cbegin() const { return begin(); }
    inline const_iterator constBegin() const { return begin(); }
    inline iterator end() { return data() + size(); }
    inline const_iterator end() const { return data() + size(); }
    inline const_iterator cend() const { return end(); }
#endif

    using Base::begin;
    using Base::cbegin;
    auto constBegin() const -> const_iterator { return begin(); }
    using Base::end;
    using Base::cend;
    inline const_iterator constEnd() const { return end(); }
#ifdef Q_QDOC
    reverse_iterator rbegin() { return reverse_iterator(end()); }
    reverse_iterator rend() { return reverse_iterator(begin()); }
    const_reverse_iterator rbegin() const { return const_reverse_iterator(end()); }
    const_reverse_iterator rend() const { return const_reverse_iterator(begin()); }
    const_reverse_iterator crbegin() const { return const_reverse_iterator(end()); }
    const_reverse_iterator crend() const { return const_reverse_iterator(begin()); }
#endif
    using Base::rbegin;
    using Base::crbegin;
    using Base::rend;
    using Base::crend;

    iterator insert(const_iterator before, qsizetype n, const T &x)
    { return Base::insert_impl(Prealloc, this->array, before, n, x); }
    iterator insert(const_iterator before, T &&x) { return emplace(before, std::move(x)); }
    inline iterator insert(const_iterator before, const T &x) { return insert(before, 1, x); }
#ifdef Q_QDOC
    iterator erase(const_iterator begin, const_iterator end);
    inline iterator erase(const_iterator pos) { return erase(pos, pos + 1); }
#endif
    using Base::erase;

    // STL compatibility:
#ifdef Q_QDOC
    inline bool empty() const { return isEmpty(); }
#endif
    using Base::empty;
    inline void push_back(const T &t) { append(t); }
    void push_back(T &&t) { append(std::move(t)); }
#ifdef Q_QDOC
    inline void pop_back() { removeLast(); }
    inline T &front() { return first(); }
    inline const T &front() const { return first(); }
    inline T &back() { return last(); }
    inline const T &back() const { return last(); }
#endif
    using Base::pop_back;
    using Base::front;
    using Base::back;
    void shrink_to_fit() { squeeze(); }
    template <typename...Args>
    iterator emplace(const_iterator pos, Args &&...args)
    { return Base::emplace_impl(Prealloc, this->array, pos, std::forward<Args>(args)...); }
    template <typename...Args>
    T &emplace_back(Args &&...args)
    { return Base::emplace_back_impl(Prealloc, this->array, std::forward<Args>(args)...); }


#ifdef Q_QDOC
    template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
    friend inline bool operator==(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r);
    template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
    friend inline bool operator!=(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r);
    template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
    friend inline bool operator< (const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r);
    template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
    friend inline bool operator> (const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r);
    template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
    friend inline bool operator<=(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r);
    template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
    friend inline bool operator>=(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r);
#else
    template <typename U = T, qsizetype Prealloc2 = Prealloc> friend
    QTypeTraits::compare_eq_result<U> operator==(const QVarLengthArray<T, Prealloc> &l, const QVarLengthArray<T, Prealloc2> &r)
    {
        return l.equal(r);
    }

    template <typename U = T, qsizetype Prealloc2 = Prealloc> friend
    QTypeTraits::compare_eq_result<U> operator!=(const QVarLengthArray<T, Prealloc> &l, const QVarLengthArray<T, Prealloc2> &r)
    {
        return !(l == r);
    }

    template <typename U = T, qsizetype Prealloc2 = Prealloc> friend
    QTypeTraits::compare_lt_result<U> operator<(const QVarLengthArray<T, Prealloc> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
        noexcept(noexcept(std::lexicographical_compare(lhs.begin(), lhs.end(),
                                                       rhs.begin(), rhs.end())))
    {
        return lhs.less_than(rhs);
    }

    template <typename U = T, qsizetype Prealloc2 = Prealloc> friend
    QTypeTraits::compare_lt_result<U> operator>(const QVarLengthArray<T, Prealloc> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
        noexcept(noexcept(lhs < rhs))
    {
        return rhs < lhs;
    }

    template <typename U = T, qsizetype Prealloc2 = Prealloc> friend
    QTypeTraits::compare_lt_result<U> operator<=(const QVarLengthArray<T, Prealloc> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
        noexcept(noexcept(lhs < rhs))
    {
        return !(lhs > rhs);
    }

    template <typename U = T, qsizetype Prealloc2 = Prealloc> friend
    QTypeTraits::compare_lt_result<U> operator>=(const QVarLengthArray<T, Prealloc> &lhs, const QVarLengthArray<T, Prealloc2> &rhs)
        noexcept(noexcept(lhs < rhs))
    {
        return !(lhs < rhs);
    }
#endif

private:
    template <typename U, qsizetype Prealloc2>
    bool equal(const QVarLengthArray<U, Prealloc2> &other) const
    { return Base::equal(other); }
    template <typename U, qsizetype Prealloc2>
    bool less_than(const QVarLengthArray<U, Prealloc2> &other) const
    { return Base::less_than(other); }

    void reallocate(qsizetype sz, qsizetype alloc)
    { Base::reallocate_impl(Prealloc, this->array, sz, alloc); }

    using Base::isValidIterator;
};

template <typename InputIterator,
          typename ValueType = typename std::iterator_traits<InputIterator>::value_type,
          QtPrivate::IfIsInputIterator<InputIterator> = true>
QVarLengthArray(InputIterator, InputIterator) -> QVarLengthArray<ValueType>;

template <class T, qsizetype Prealloc>
Q_INLINE_TEMPLATE QVarLengthArray<T, Prealloc>::QVarLengthArray(qsizetype asize)
{
    this->s = asize;
    static_assert(Prealloc > 0, "QVarLengthArray Prealloc must be greater than 0.");
    Q_ASSERT_X(size() >= 0, "QVarLengthArray::QVarLengthArray()", "Size must be greater than or equal to 0.");
    if (size() > Prealloc) {
        this->ptr = malloc(size() * sizeof(T));
        Q_CHECK_PTR(data());
        this->a = size();
    } else {
        this->ptr = this->array;
        this->a = Prealloc;
    }
    if constexpr (QTypeInfo<T>::isComplex) {
        T *i = end();
        while (i != begin())
            new (--i) T;
    }
}

template <class T>
template <typename AT>
Q_INLINE_TEMPLATE qsizetype QVLABase<T>::indexOf(const AT &t, qsizetype from) const
{
    if (from < 0)
        from = qMax(from + size(), qsizetype(0));
    if (from < size()) {
        const T *n = data() + from - 1;
        const T *e = end();
        while (++n != e)
            if (*n == t)
                return n - data();
    }
    return -1;
}

template <class T>
template <typename AT>
Q_INLINE_TEMPLATE qsizetype QVLABase<T>::lastIndexOf(const AT &t, qsizetype from) const
{
    if (from < 0)
        from += size();
    else if (from >= size())
        from = size() - 1;
    if (from >= 0) {
        const T *b = begin();
        const T *n = b + from + 1;
        while (n != b) {
            if (*--n == t)
                return n - b;
        }
    }
    return -1;
}

template <class T>
template <typename AT>
Q_INLINE_TEMPLATE bool QVLABase<T>::contains(const AT &t) const
{
    const T *b = begin();
    const T *i = end();
    while (i != b) {
        if (*--i == t)
            return true;
    }
    return false;
}

template <class T>
Q_OUTOFLINE_TEMPLATE void QVLABase<T>::append_impl(qsizetype prealloc, void *array, const T *abuf, qsizetype increment)
{
    Q_ASSERT(abuf || increment == 0);
    if (increment <= 0)
        return;

    const qsizetype asize = size() + increment;

    if (asize >= capacity())
        reallocate_impl(prealloc, array, size(), qMax(size() * 2, asize));

    if constexpr (QTypeInfo<T>::isComplex)
        std::uninitialized_copy_n(abuf, increment, end());
    else
        memcpy(static_cast<void *>(end()), static_cast<const void *>(abuf), increment * sizeof(T));

    this->s = asize;
}

template <class T>
Q_OUTOFLINE_TEMPLATE void QVLABase<T>::reallocate_impl(qsizetype prealloc, void *array, qsizetype asize, qsizetype aalloc, const T *v)
{
    Q_ASSERT(aalloc >= asize);
    Q_ASSERT(data());
    T *oldPtr = data();
    qsizetype osize = size();

    const qsizetype copySize = qMin(asize, osize);
    Q_ASSUME(copySize >= 0);

    if (aalloc != capacity()) {
        QVLABaseBase::malloced_ptr guard;
        void *newPtr;
        qsizetype newA;
        if (aalloc > prealloc) {
            newPtr = malloc(aalloc * sizeof(T));
            guard.reset(newPtr);
            Q_CHECK_PTR(newPtr); // could throw
            // by design: in case of QT_NO_EXCEPTIONS malloc must not fail or it crashes here
            newA = aalloc;
        } else {
            newPtr = array;
            newA = prealloc;
        }
        QtPrivate::q_uninitialized_relocate_n(oldPtr, copySize,
                                              reinterpret_cast<T *>(newPtr));
        // commit:
        ptr = newPtr;
        guard.release();
        a = newA;
    }
    s = copySize;

    // destroy remaining old objects
    if constexpr (QTypeInfo<T>::isComplex) {
        if (osize > asize)
            std::destroy(oldPtr + asize, oldPtr + osize);
    }

    if (oldPtr != reinterpret_cast<T *>(array) && oldPtr != data())
        free(oldPtr);

    if (v) {
        if constexpr (std::is_copy_constructible_v<T>) {
            while (size() < asize) {
                new (data() + size()) T(*v);
                ++s;
            }
        } else {
            Q_UNREACHABLE();
        }
    } else if constexpr (QTypeInfo<T>::isComplex) {
        // call default constructor for new objects (which can throw)
        while (size() < asize) {
            new (data() + size()) T;
            ++s;
        }
    } else {
        s = asize;
    }

}

template <class T>
Q_OUTOFLINE_TEMPLATE T QVLABase<T>::value(qsizetype i) const
{
    if (size_t(i) >= size_t(size()))
        return T();
    return operator[](i);
}
template <class T>
Q_OUTOFLINE_TEMPLATE T QVLABase<T>::value(qsizetype i, const T &defaultValue) const
{
    return (size_t(i) >= size_t(size())) ? defaultValue : operator[](i);
}

template <class T, qsizetype Prealloc>
inline void QVarLengthArray<T, Prealloc>::insert(qsizetype i, T &&t)
{ verify(i, 0);
  insert(cbegin() + i, std::move(t)); }
template <class T, qsizetype Prealloc>
inline void QVarLengthArray<T, Prealloc>::insert(qsizetype i, const T &t)
{ verify(i, 0);
  insert(begin() + i, 1, t); }
template <class T, qsizetype Prealloc>
inline void QVarLengthArray<T, Prealloc>::insert(qsizetype i, qsizetype n, const T &t)
{ verify(i, 0);
  insert(begin() + i, n, t); }
template <class T>
inline void QVLABase<T>::remove(qsizetype i, qsizetype n)
{ verify(i, n);
  erase(begin() + i, begin() + i + n); }
template <class T>
template <typename AT>
inline qsizetype QVLABase<T>::removeAll(const AT &t)
{ return QtPrivate::sequential_erase_with_copy(*this, t); }
template <class T>
template <typename AT>
inline bool QVLABase<T>::removeOne(const AT &t)
{ return QtPrivate::sequential_erase_one(*this, t); }
template <class T>
template <typename Predicate>
inline qsizetype QVLABase<T>::removeIf(Predicate pred)
{ return QtPrivate::sequential_erase_if(*this, pred); }
#if QT_DEPRECATED_SINCE(6, 3)
template <class T, qsizetype Prealloc>
inline void QVarLengthArray<T, Prealloc>::prepend(T &&t)
{ insert(cbegin(), std::move(t)); }
template <class T, qsizetype Prealloc>
inline void QVarLengthArray<T, Prealloc>::prepend(const T &t)
{ insert(begin(), 1, t); }
#endif

template <class T>
inline void QVLABase<T>::replace(qsizetype i, const T &t)
{
    verify(i);
    data()[i] = t;
}

template <class T>
template <typename...Args>
Q_OUTOFLINE_TEMPLATE auto QVLABase<T>::emplace_impl(qsizetype prealloc, void *array, const_iterator before, Args &&...args) -> iterator
{
    Q_ASSERT_X(isValidIterator(before), "QVarLengthArray::insert", "The specified const_iterator argument 'before' is invalid");
    Q_ASSERT(size() <= capacity());
    Q_ASSERT(capacity() > 0);

    qsizetype offset = qsizetype(before - cbegin());
    if (size() == capacity())
        reallocate_impl(prealloc, array, size(), size() * 2);
    if constexpr (!QTypeInfo<T>::isRelocatable) {
        T *b = begin() + offset;
        T *i = end();
        T *j = i + 1;
        // The new end-element needs to be constructed, the rest must be move assigned
        if (i != b) {
            new (--j) T(std::move(*--i));
            while (i != b)
                *--j = std::move(*--i);
            *b = T(std::forward<Args>(args)...);
        } else {
            new (b) T(std::forward<Args>(args)...);
        }
    } else {
        T *b = begin() + offset;
        memmove(static_cast<void *>(b + 1), static_cast<const void *>(b), (size() - offset) * sizeof(T));
        new (b) T(std::forward<Args>(args)...);
    }
    this->s += 1;
    return data() + offset;
}

template <class T>
Q_OUTOFLINE_TEMPLATE auto QVLABase<T>::insert_impl(qsizetype prealloc, void *array, const_iterator before, qsizetype n, const T &t) -> iterator
{
    Q_ASSERT_X(isValidIterator(before), "QVarLengthArray::insert", "The specified const_iterator argument 'before' is invalid");

    qsizetype offset = qsizetype(before - cbegin());
    if (n != 0) {
        const T copy(t); // `t` could alias an element in [begin(), end()[
        resize_impl(prealloc, array, size() + n);
        if constexpr (!QTypeInfo<T>::isRelocatable) {
            T *b = begin() + offset;
            T *j = end();
            T *i = j - n;
            while (i != b)
                *--j = *--i;
            i = b + n;
            while (i != b)
                *--i = copy;
        } else {
            T *b = begin() + offset;
            T *i = b + n;
            memmove(static_cast<void *>(i), static_cast<const void *>(b), (size() - offset - n) * sizeof(T));
            while (i != b)
                new (--i) T(copy);
        }
    }
    return data() + offset;
}

template <class T>
Q_OUTOFLINE_TEMPLATE auto QVLABase<T>::erase(const_iterator abegin, const_iterator aend) -> iterator
{
    Q_ASSERT_X(isValidIterator(abegin), "QVarLengthArray::insert", "The specified const_iterator argument 'abegin' is invalid");
    Q_ASSERT_X(isValidIterator(aend), "QVarLengthArray::insert", "The specified const_iterator argument 'aend' is invalid");

    qsizetype f = qsizetype(abegin - cbegin());
    qsizetype l = qsizetype(aend - cbegin());
    qsizetype n = l - f;

    if (n == 0) // avoid UB in std::move() below
        return data() + f;

    Q_ASSERT(n > 0); // aend must be reachable from abegin

    if constexpr (QTypeInfo<T>::isComplex) {
        std::move(begin() + l, end(), QT_MAKE_CHECKED_ARRAY_ITERATOR(begin() + f, size() - f));
        std::destroy(end() - n, end());
    } else {
        memmove(static_cast<void *>(data() + f), static_cast<const void *>(data() + l), (size() - l) * sizeof(T));
    }
    this->s -= n;
    return data() + f;
}

#ifdef Q_QDOC
// Fake definitions for qdoc, only the redeclaration is used.
template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
bool operator==(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{ return bool{}; }
template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
bool operator!=(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{ return bool{}; }
template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
bool operator< (const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{ return bool{}; }
template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
bool operator> (const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{ return bool{}; }
template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
bool operator<=(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{ return bool{}; }
template <typename T, qsizetype Prealloc1, qsizetype Prealloc2>
bool operator>=(const QVarLengthArray<T, Prealloc1> &l, const QVarLengthArray<T, Prealloc2> &r)
{ return bool{}; }
#endif

template <typename T, qsizetype Prealloc>
size_t qHash(const QVarLengthArray<T, Prealloc> &key, size_t seed = 0)
    noexcept(QtPrivate::QNothrowHashable_v<T>)
{
    return key.hash(seed);
}

template <typename T, qsizetype Prealloc, typename AT>
qsizetype erase(QVarLengthArray<T, Prealloc> &array, const AT &t)
{
    return array.removeAll(t);
}

template <typename T, qsizetype Prealloc, typename Predicate>
qsizetype erase_if(QVarLengthArray<T, Prealloc> &array, Predicate pred)
{
    return array.removeIf(pred);
}

QT_END_NAMESPACE

#endif // QVARLENGTHARRAY_H
