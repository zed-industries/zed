//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef ALLOCATORS_H
#define ALLOCATORS_H

#include <cstddef>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

#include "test_macros.h"

#if TEST_STD_VER >= 11

template <class T>
class A1
{
    int id_;
public:
    explicit A1(int id = 0) TEST_NOEXCEPT : id_(id) {}

    typedef T value_type;

    int id() const {return id_;}

    static bool copy_called;
    static bool move_called;
    static bool allocate_called;
    static std::pair<T*, std::size_t> deallocate_called;

    A1(const A1& a) TEST_NOEXCEPT : id_(a.id()) {copy_called = true;}
    A1(A1&& a)      TEST_NOEXCEPT : id_(a.id()) {move_called = true;}
    A1& operator=(const A1& a) TEST_NOEXCEPT { id_ = a.id(); copy_called = true; return *this;}
    A1& operator=(A1&& a)      TEST_NOEXCEPT { id_ = a.id(); move_called = true; return *this;}

    template <class U>
        A1(const A1<U>& a) TEST_NOEXCEPT : id_(a.id()) {copy_called = true;}
    template <class U>
        A1(A1<U>&& a) TEST_NOEXCEPT : id_(a.id()) {move_called = true;}

    T* allocate(std::size_t n)
    {
        allocate_called = true;
        return (T*)n;
    }

    void deallocate(T* p, std::size_t n)
    {
        deallocate_called = std::pair<T*, std::size_t>(p, n);
    }

    std::size_t max_size() const {return id_;}
};

template <class T> bool A1<T>::copy_called = false;
template <class T> bool A1<T>::move_called = false;
template <class T> bool A1<T>::allocate_called = false;
template <class T> std::pair<T*, std::size_t> A1<T>::deallocate_called;

template <class T, class U>
inline
bool operator==(const A1<T>& x, const A1<U>& y)
{
    return x.id() == y.id();
}

template <class T, class U>
inline
bool operator!=(const A1<T>& x, const A1<U>& y)
{
    return !(x == y);
}

template <class T>
class A2
{
    int id_;
public:
    explicit A2(int id = 0) TEST_NOEXCEPT : id_(id) {}

    typedef T value_type;

    typedef unsigned size_type;
    typedef int difference_type;

    typedef std::true_type propagate_on_container_move_assignment;

    int id() const {return id_;}

    static bool copy_called;
    static bool move_called;
    static bool allocate_called;

    A2(const A2& a) TEST_NOEXCEPT : id_(a.id()) {copy_called = true;}
    A2(A2&& a)      TEST_NOEXCEPT : id_(a.id()) {move_called = true;}
    A2& operator=(const A2& a) TEST_NOEXCEPT { id_ = a.id(); copy_called = true; return *this;}
    A2& operator=(A2&& a)      TEST_NOEXCEPT { id_ = a.id(); move_called = true; return *this;}

    T* allocate(std::size_t, const void* hint)
    {
        allocate_called = true;
        return (T*) const_cast<void *>(hint);
    }
};

template <class T> bool A2<T>::copy_called = false;
template <class T> bool A2<T>::move_called = false;
template <class T> bool A2<T>::allocate_called = false;

template <class T, class U>
inline
bool operator==(const A2<T>& x, const A2<U>& y)
{
    return x.id() == y.id();
}

template <class T, class U>
inline
bool operator!=(const A2<T>& x, const A2<U>& y)
{
    return !(x == y);
}

template <class T>
class A3
{
    int id_;
public:
    explicit A3(int id = 0) TEST_NOEXCEPT : id_(id) {}

    typedef T value_type;

    typedef std::true_type propagate_on_container_copy_assignment;
    typedef std::true_type propagate_on_container_swap;

    int id() const {return id_;}

    static bool copy_called;
    static bool move_called;
    static bool constructed;
    static bool destroy_called;

    A3(const A3& a) TEST_NOEXCEPT : id_(a.id()) {copy_called = true;}
    A3(A3&& a)      TEST_NOEXCEPT : id_(a.id())  {move_called = true;}
    A3& operator=(const A3& a) TEST_NOEXCEPT { id_ = a.id(); copy_called = true; return *this;}
    A3& operator=(A3&& a)      TEST_NOEXCEPT { id_ = a.id(); move_called = true; return *this;}

    template <class U, class ...Args>
    void construct(U* p, Args&& ...args)
    {
        ::new (p) U(std::forward<Args>(args)...);
        constructed = true;
    }

    template <class U>
    void destroy(U* p)
    {
        p->~U();
        destroy_called = true;
    }

    A3 select_on_container_copy_construction() const {return A3(-1);}
};

template <class T> bool A3<T>::copy_called = false;
template <class T> bool A3<T>::move_called = false;
template <class T> bool A3<T>::constructed = false;
template <class T> bool A3<T>::destroy_called = false;

template <class T, class U>
inline
bool operator==(const A3<T>& x, const A3<U>& y)
{
    return x.id() == y.id();
}

template <class T, class U>
inline
bool operator!=(const A3<T>& x, const A3<U>& y)
{
    return !(x == y);
}

template <class T, bool POCCAValue>
class MaybePOCCAAllocator {
    int id_ = 0;
    bool* copy_assigned_into_ = nullptr;

    template<class, bool> friend class MaybePOCCAAllocator;

public:
    typedef std::integral_constant<bool, POCCAValue> propagate_on_container_copy_assignment;
    typedef T value_type;

    template <class U>
    struct rebind {
        typedef MaybePOCCAAllocator<U, POCCAValue> other;
    };

    TEST_CONSTEXPR MaybePOCCAAllocator() = default;
    TEST_CONSTEXPR MaybePOCCAAllocator(int id, bool* copy_assigned_into)
        : id_(id), copy_assigned_into_(copy_assigned_into) {}

    template <class U>
    TEST_CONSTEXPR MaybePOCCAAllocator(const MaybePOCCAAllocator<U, POCCAValue>& that)
        : id_(that.id_), copy_assigned_into_(that.copy_assigned_into_) {}

    MaybePOCCAAllocator(const MaybePOCCAAllocator&) = default;
    TEST_CONSTEXPR_CXX14 MaybePOCCAAllocator& operator=(const MaybePOCCAAllocator& a)
    {
        id_ = a.id();
        if (copy_assigned_into_)
            *copy_assigned_into_ = true;
        return *this;
    }

    TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n)
    {
        return std::allocator<T>().allocate(n);
    }

    TEST_CONSTEXPR_CXX20 void deallocate(T* ptr, std::size_t n)
    {
        std::allocator<T>().deallocate(ptr, n);
    }

    TEST_CONSTEXPR int id() const { return id_; }

    template <class U>
    TEST_CONSTEXPR friend bool operator==(const MaybePOCCAAllocator& lhs, const MaybePOCCAAllocator<U, POCCAValue>& rhs)
    {
        return lhs.id() == rhs.id();
    }

    template <class U>
    TEST_CONSTEXPR friend bool operator!=(const MaybePOCCAAllocator& lhs, const MaybePOCCAAllocator<U, POCCAValue>& rhs)
    {
        return !(lhs == rhs);
    }
};

template <class T>
using POCCAAllocator = MaybePOCCAAllocator<T, /*POCCAValue = */true>;
template <class T>
using NonPOCCAAllocator = MaybePOCCAAllocator<T, /*POCCAValue = */false>;

#endif // TEST_STD_VER >= 11

#endif // ALLOCATORS_H
