//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef MIN_ALLOCATOR_H
#define MIN_ALLOCATOR_H

#include <cassert>
#include <climits>
#include <cstddef>
#include <cstdlib>
#include <iterator>
#include <memory>
#include <new>
#include <type_traits>
#include <cstring>

#include "test_macros.h"

template <class T>
class bare_allocator
{
public:
    typedef T value_type;

    bare_allocator() TEST_NOEXCEPT {}

    template <class U>
    bare_allocator(bare_allocator<U>) TEST_NOEXCEPT {}

    T* allocate(std::size_t n)
    {
        return static_cast<T*>(::operator new(n*sizeof(T)));
    }

    void deallocate(T* p, std::size_t)
    {
        return ::operator delete(static_cast<void*>(p));
    }

    friend bool operator==(bare_allocator, bare_allocator) {return true;}
    friend bool operator!=(bare_allocator x, bare_allocator y) {return !(x == y);}
};


template <class T>
class no_default_allocator
{
#if TEST_STD_VER >= 11
    no_default_allocator() = delete;
#else
    no_default_allocator();
#endif
    struct construct_tag {};
    explicit no_default_allocator(construct_tag) {}

public:
    static no_default_allocator create() {
      construct_tag tag;
      return no_default_allocator(tag);
    }

public:
    typedef T value_type;

    template <class U>
    no_default_allocator(no_default_allocator<U>) TEST_NOEXCEPT {}

    T* allocate(std::size_t n)
    {
        return static_cast<T*>(::operator new(n*sizeof(T)));
    }

    void deallocate(T* p, std::size_t)
    {
        return ::operator delete(static_cast<void*>(p));
    }

    friend bool operator==(no_default_allocator, no_default_allocator) {return true;}
    friend bool operator!=(no_default_allocator x, no_default_allocator y) {return !(x == y);}
};

struct malloc_allocator_base {
    static std::size_t outstanding_bytes;
    static std::size_t alloc_count;
    static std::size_t dealloc_count;
    static bool disable_default_constructor;

    static std::size_t outstanding_alloc() {
      assert(alloc_count >= dealloc_count);
      return (alloc_count - dealloc_count);
    }

    static void reset() {
        assert(outstanding_alloc() == 0);
        disable_default_constructor = false;
        outstanding_bytes = 0;
        alloc_count = 0;
        dealloc_count = 0;
    }
};

size_t malloc_allocator_base::outstanding_bytes = 0;
size_t malloc_allocator_base::alloc_count = 0;
size_t malloc_allocator_base::dealloc_count = 0;
bool malloc_allocator_base::disable_default_constructor = false;


template <class T>
class malloc_allocator : public malloc_allocator_base
{
public:
    typedef T value_type;

    malloc_allocator() TEST_NOEXCEPT { assert(!disable_default_constructor); }

    template <class U>
    malloc_allocator(malloc_allocator<U>) TEST_NOEXCEPT {}

    T* allocate(std::size_t n)
    {
        const std::size_t nbytes = n*sizeof(T);
        ++alloc_count;
        outstanding_bytes += nbytes;
        return static_cast<T*>(std::malloc(nbytes));
    }

    void deallocate(T* p, std::size_t n)
    {
        const std::size_t nbytes = n*sizeof(T);
        ++dealloc_count;
        outstanding_bytes -= nbytes;
        std::free(static_cast<void*>(p));
    }

    friend bool operator==(malloc_allocator, malloc_allocator) {return true;}
    friend bool operator!=(malloc_allocator x, malloc_allocator y) {return !(x == y);}
};

template <class T>
struct cpp03_allocator : bare_allocator<T>
{
    typedef T value_type;
    typedef value_type* pointer;

    static bool construct_called;

    // Returned value is not used but it's not prohibited.
    pointer construct(pointer p, const value_type& val)
    {
        ::new(p) value_type(val);
        construct_called = true;
        return p;
    }

    std::size_t max_size() const
    {
        return UINT_MAX / sizeof(T);
    }
};
template <class T> bool cpp03_allocator<T>::construct_called = false;

template <class T>
struct cpp03_overload_allocator : bare_allocator<T>
{
    typedef T value_type;
    typedef value_type* pointer;

    static bool construct_called;

    void construct(pointer p, const value_type& val)
    {
        construct(p, val, std::is_class<T>());
    }
    void construct(pointer p, const value_type& val, std::true_type)
    {
        ::new(p) value_type(val);
        construct_called = true;
    }
    void construct(pointer p, const value_type& val, std::false_type)
    {
        ::new(p) value_type(val);
        construct_called = true;
    }

    std::size_t max_size() const
    {
        return UINT_MAX / sizeof(T);
    }
};
template <class T> bool cpp03_overload_allocator<T>::construct_called = false;

template <class T, class = std::integral_constant<std::size_t, 0> > class min_pointer;
template <class T, class ID> class min_pointer<const T, ID>;
template <class ID> class min_pointer<void, ID>;
template <class ID> class min_pointer<const void, ID>;
template <class T> class min_allocator;

template <class ID>
class min_pointer<const void, ID>
{
    const void* ptr_;
public:
    min_pointer() TEST_NOEXCEPT = default;
    min_pointer(std::nullptr_t) TEST_NOEXCEPT : ptr_(nullptr) {}
    template <class T>
    min_pointer(min_pointer<T, ID> p) TEST_NOEXCEPT : ptr_(p.ptr_) {}

    explicit operator bool() const {return ptr_ != nullptr;}

    friend bool operator==(min_pointer x, min_pointer y) {return x.ptr_ == y.ptr_;}
    friend bool operator!=(min_pointer x, min_pointer y) {return !(x == y);}
    template <class U, class XID> friend class min_pointer;
};

template <class ID>
class min_pointer<void, ID>
{
    void* ptr_;
public:
    min_pointer() TEST_NOEXCEPT = default;
    TEST_CONSTEXPR_CXX14 min_pointer(std::nullptr_t) TEST_NOEXCEPT : ptr_(nullptr) {}
    template <class T,
              class = typename std::enable_if
                       <
                            !std::is_const<T>::value
                       >::type
             >
    TEST_CONSTEXPR_CXX14 min_pointer(min_pointer<T, ID> p) TEST_NOEXCEPT : ptr_(p.ptr_) {}

    TEST_CONSTEXPR_CXX14 explicit operator bool() const {return ptr_ != nullptr;}

    TEST_CONSTEXPR_CXX14 friend bool operator==(min_pointer x, min_pointer y) {return x.ptr_ == y.ptr_;}
    TEST_CONSTEXPR_CXX14 friend bool operator!=(min_pointer x, min_pointer y) {return !(x == y);}
    template <class U, class XID> friend class min_pointer;
};

template <class T, class ID>
class min_pointer
{
    T* ptr_;

    TEST_CONSTEXPR_CXX14 explicit min_pointer(T* p) TEST_NOEXCEPT : ptr_(p) {}
public:
    min_pointer() TEST_NOEXCEPT = default;
    TEST_CONSTEXPR_CXX14 min_pointer(std::nullptr_t) TEST_NOEXCEPT : ptr_(nullptr) {}
    TEST_CONSTEXPR_CXX14 explicit min_pointer(min_pointer<void, ID> p) TEST_NOEXCEPT : ptr_(static_cast<T*>(p.ptr_)) {}

    TEST_CONSTEXPR_CXX14 explicit operator bool() const {return ptr_ != nullptr;}

    typedef std::ptrdiff_t difference_type;
    typedef T& reference;
    typedef T* pointer;
    typedef T value_type;
    typedef std::random_access_iterator_tag iterator_category;

    TEST_CONSTEXPR_CXX14 reference operator*() const {return *ptr_;}
    TEST_CONSTEXPR_CXX14 pointer operator->() const {return ptr_;}

    TEST_CONSTEXPR_CXX14 min_pointer& operator++() {++ptr_; return *this;}
    TEST_CONSTEXPR_CXX14 min_pointer operator++(int) {min_pointer tmp(*this); ++ptr_; return tmp;}

    TEST_CONSTEXPR_CXX14 min_pointer& operator--() {--ptr_; return *this;}
    TEST_CONSTEXPR_CXX14 min_pointer operator--(int) {min_pointer tmp(*this); --ptr_; return tmp;}

    TEST_CONSTEXPR_CXX14 min_pointer& operator+=(difference_type n) {ptr_ += n; return *this;}
    TEST_CONSTEXPR_CXX14 min_pointer& operator-=(difference_type n) {ptr_ -= n; return *this;}

    TEST_CONSTEXPR_CXX14 min_pointer operator+(difference_type n) const
    {
        min_pointer tmp(*this);
        tmp += n;
        return tmp;
    }

    friend TEST_CONSTEXPR_CXX14 min_pointer operator+(difference_type n, min_pointer x)
    {
        return x + n;
    }

    TEST_CONSTEXPR_CXX14 min_pointer operator-(difference_type n) const
    {
        min_pointer tmp(*this);
        tmp -= n;
        return tmp;
    }

    friend TEST_CONSTEXPR_CXX14 difference_type operator-(min_pointer x, min_pointer y)
    {
        return x.ptr_ - y.ptr_;
    }

    TEST_CONSTEXPR_CXX14 reference operator[](difference_type n) const {return ptr_[n];}

    friend TEST_CONSTEXPR_CXX14 bool operator< (min_pointer x, min_pointer y) {return x.ptr_ < y.ptr_;}
    friend TEST_CONSTEXPR_CXX14 bool operator> (min_pointer x, min_pointer y) {return y < x;}
    friend TEST_CONSTEXPR_CXX14 bool operator<=(min_pointer x, min_pointer y) {return !(y < x);}
    friend TEST_CONSTEXPR_CXX14 bool operator>=(min_pointer x, min_pointer y) {return !(x < y);}

    static TEST_CONSTEXPR_CXX14 min_pointer pointer_to(T& t) {return min_pointer(std::addressof(t));}

    friend TEST_CONSTEXPR_CXX14 bool operator==(min_pointer x, min_pointer y) {return x.ptr_ == y.ptr_;}
    friend TEST_CONSTEXPR_CXX14 bool operator!=(min_pointer x, min_pointer y) {return !(x == y);}
    template <class U, class XID> friend class min_pointer;
    template <class U> friend class min_allocator;
};

template <class T, class ID>
class min_pointer<const T, ID>
{
    const T* ptr_;

    TEST_CONSTEXPR_CXX14 explicit min_pointer(const T* p) : ptr_(p) {}
public:
    min_pointer() TEST_NOEXCEPT = default;
    TEST_CONSTEXPR_CXX14 min_pointer(std::nullptr_t) : ptr_(nullptr) {}
    TEST_CONSTEXPR_CXX14 min_pointer(min_pointer<T, ID> p) : ptr_(p.ptr_) {}
    TEST_CONSTEXPR_CXX14 explicit min_pointer(min_pointer<const void, ID> p) : ptr_(static_cast<const T*>(p.ptr_)) {}

    TEST_CONSTEXPR_CXX14 explicit operator bool() const {return ptr_ != nullptr;}

    typedef std::ptrdiff_t difference_type;
    typedef const T& reference;
    typedef const T* pointer;
    typedef const T value_type;
    typedef std::random_access_iterator_tag iterator_category;

    TEST_CONSTEXPR_CXX14 reference operator*() const {return *ptr_;}
    TEST_CONSTEXPR_CXX14 pointer operator->() const {return ptr_;}

    TEST_CONSTEXPR_CXX14 min_pointer& operator++() {++ptr_; return *this;}
    TEST_CONSTEXPR_CXX14 min_pointer operator++(int) {min_pointer tmp(*this); ++ptr_; return tmp;}

    TEST_CONSTEXPR_CXX14 min_pointer& operator--() {--ptr_; return *this;}
    TEST_CONSTEXPR_CXX14 min_pointer operator--(int) {min_pointer tmp(*this); --ptr_; return tmp;}

    TEST_CONSTEXPR_CXX14 min_pointer& operator+=(difference_type n) {ptr_ += n; return *this;}
    TEST_CONSTEXPR_CXX14 min_pointer& operator-=(difference_type n) {ptr_ -= n; return *this;}

    TEST_CONSTEXPR_CXX14 min_pointer operator+(difference_type n) const
    {
        min_pointer tmp(*this);
        tmp += n;
        return tmp;
    }

    friend TEST_CONSTEXPR_CXX14 min_pointer operator+(difference_type n, min_pointer x)
    {
        return x + n;
    }

    TEST_CONSTEXPR_CXX14 min_pointer operator-(difference_type n) const
    {
        min_pointer tmp(*this);
        tmp -= n;
        return tmp;
    }

    friend TEST_CONSTEXPR_CXX14 difference_type operator-(min_pointer x, min_pointer y)
    {
        return x.ptr_ - y.ptr_;
    }

    TEST_CONSTEXPR_CXX14 reference operator[](difference_type n) const {return ptr_[n];}

    friend TEST_CONSTEXPR_CXX14 bool operator< (min_pointer x, min_pointer y) {return x.ptr_ < y.ptr_;}
    friend TEST_CONSTEXPR_CXX14 bool operator> (min_pointer x, min_pointer y) {return y < x;}
    friend TEST_CONSTEXPR_CXX14 bool operator<=(min_pointer x, min_pointer y) {return !(y < x);}
    friend TEST_CONSTEXPR_CXX14 bool operator>=(min_pointer x, min_pointer y) {return !(x < y);}

    static TEST_CONSTEXPR_CXX14 min_pointer pointer_to(const T& t) {return min_pointer(std::addressof(t));}

    friend TEST_CONSTEXPR_CXX14 bool operator==(min_pointer x, min_pointer y) {return x.ptr_ == y.ptr_;}
    friend TEST_CONSTEXPR_CXX14 bool operator!=(min_pointer x, min_pointer y) {return x.ptr_ != y.ptr_;}
    friend TEST_CONSTEXPR_CXX14 bool operator==(min_pointer x, std::nullptr_t) {return x.ptr_ == nullptr;}
    friend TEST_CONSTEXPR_CXX14 bool operator!=(min_pointer x, std::nullptr_t) {return x.ptr_ != nullptr;}
    friend TEST_CONSTEXPR_CXX14 bool operator==(std::nullptr_t, min_pointer x) {return x.ptr_ == nullptr;}
    friend TEST_CONSTEXPR_CXX14 bool operator!=(std::nullptr_t, min_pointer x) {return x.ptr_ != nullptr;}
    template <class U, class XID> friend class min_pointer;
};

template <class T>
class min_allocator
{
public:
    typedef T value_type;
    typedef min_pointer<T> pointer;

    min_allocator() = default;
    template <class U>
    TEST_CONSTEXPR_CXX20 min_allocator(min_allocator<U>) {}

    TEST_CONSTEXPR_CXX20 pointer allocate(std::size_t n) { return pointer(std::allocator<T>().allocate(n)); }

    TEST_CONSTEXPR_CXX20 void deallocate(pointer p, std::size_t n) { std::allocator<T>().deallocate(p.ptr_, n); }

    TEST_CONSTEXPR_CXX20 friend bool operator==(min_allocator, min_allocator) {return true;}
    TEST_CONSTEXPR_CXX20 friend bool operator!=(min_allocator x, min_allocator y) {return !(x == y);}
};

template <class T>
class explicit_allocator
{
public:
    typedef T value_type;

    TEST_CONSTEXPR_CXX20 explicit_allocator() TEST_NOEXCEPT {}

    template <class U>
    TEST_CONSTEXPR_CXX20 explicit explicit_allocator(explicit_allocator<U>) TEST_NOEXCEPT {}

    TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n)
    {
        return static_cast<T*>(std::allocator<T>().allocate(n));
    }

    TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t n)
    {
        std::allocator<T>().deallocate(p, n);
    }

    TEST_CONSTEXPR_CXX20 friend bool operator==(explicit_allocator, explicit_allocator) {return true;}
    TEST_CONSTEXPR_CXX20 friend bool operator!=(explicit_allocator x, explicit_allocator y) {return !(x == y);}
};

template <class T>
class unaligned_allocator {
public:
  static_assert(TEST_ALIGNOF(T) == 1, "Type T cannot be created on unaligned address (UB)");
  typedef T value_type;

  TEST_CONSTEXPR_CXX20 unaligned_allocator() TEST_NOEXCEPT {}

  template <class U>
  TEST_CONSTEXPR_CXX20 explicit unaligned_allocator(unaligned_allocator<U>) TEST_NOEXCEPT {}

  TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) { return std::allocator<T>().allocate(n + 1) + 1; }

  TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t n) { std::allocator<T>().deallocate(p - 1, n + 1); }

  TEST_CONSTEXPR_CXX20 friend bool operator==(unaligned_allocator, unaligned_allocator) { return true; }
  TEST_CONSTEXPR_CXX20 friend bool operator!=(unaligned_allocator x, unaligned_allocator y) { return !(x == y); }
};

template <class T>
class safe_allocator {
public:
  typedef T value_type;

  TEST_CONSTEXPR_CXX20 safe_allocator() TEST_NOEXCEPT {}

  template <class U>
  TEST_CONSTEXPR_CXX20 safe_allocator(safe_allocator<U>) TEST_NOEXCEPT {}

  TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) {
    T* memory = std::allocator<T>().allocate(n);
    if (!TEST_IS_CONSTANT_EVALUATED)
      std::memset(static_cast<void*>(memory), 0, sizeof(T) * n);

    return memory;
  }

  TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t n) {
    if (!TEST_IS_CONSTANT_EVALUATED)
      DoNotOptimize(std::memset(static_cast<void*>(p), 0, sizeof(T) * n));
    std::allocator<T>().deallocate(p, n);
  }

  TEST_CONSTEXPR_CXX20 friend bool operator==(safe_allocator, safe_allocator) { return true; }
  TEST_CONSTEXPR_CXX20 friend bool operator!=(safe_allocator x, safe_allocator y) { return !(x == y); }
};

template <std::size_t MaxSize, class T>
struct tiny_size_allocator {
  using value_type = T;
  using size_type  = unsigned;

  template <class U>
  struct rebind {
    using other = tiny_size_allocator<MaxSize, U>;
  };

  tiny_size_allocator() = default;

  template <class U>
  TEST_CONSTEXPR_CXX20 tiny_size_allocator(tiny_size_allocator<MaxSize, U>) {}

  TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) {
    assert(n <= MaxSize);
    return std::allocator<T>().allocate(n);
  }

  TEST_CONSTEXPR_CXX20 void deallocate(T* ptr, std::size_t n) { std::allocator<T>().deallocate(ptr, n); }

  TEST_CONSTEXPR_CXX20 size_type max_size() const { return MaxSize; }

  friend bool operator==(tiny_size_allocator, tiny_size_allocator) { return true; }
  friend bool operator!=(tiny_size_allocator, tiny_size_allocator) { return false; }
};

#endif // MIN_ALLOCATOR_H
