//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_ALLOCATOR_H
#define TEST_ALLOCATOR_H

#include <type_traits>
#include <new>
#include <memory>
#include <utility>
#include <cstddef>
#include <cstdlib>
#include <climits>
#include <cassert>

#include "test_macros.h"

template <class Alloc>
TEST_CONSTEXPR_CXX20 inline typename std::allocator_traits<Alloc>::size_type alloc_max_size(Alloc const& a) {
  typedef std::allocator_traits<Alloc> AT;
  return AT::max_size(a);
}

struct test_allocator_statistics {
  int time_to_throw   = 0;
  int throw_after     = INT_MAX;
  int count           = 0; // the number of active instances
  int alloc_count     = 0; // the number of allocations not deallocating
  int allocated_size  = 0; // the size of allocated elements
  int construct_count = 0; // the number of times that ::construct was called
  int destroy_count   = 0; // the number of times that ::destroy was called
  int copied          = 0;
  int moved           = 0;
  int converted       = 0;

  TEST_CONSTEXPR_CXX14 void clear() {
    assert(count == 0 && "clearing leaking allocator data?");
    count           = 0;
    time_to_throw   = 0;
    alloc_count     = 0;
    allocated_size  = 0;
    construct_count = 0;
    destroy_count   = 0;
    throw_after     = INT_MAX;
    clear_ctor_counters();
  }

  TEST_CONSTEXPR_CXX14 void clear_ctor_counters() {
    copied    = 0;
    moved     = 0;
    converted = 0;
  }
};

struct test_alloc_base {
  TEST_CONSTEXPR static const int destructed_value = -1;
  TEST_CONSTEXPR static const int moved_value      = INT_MAX;
};

template <class T>
class test_allocator {
  int data_                         = 0; // participates in equality
  int id_                           = 0; // unique identifier, doesn't participate in equality
  test_allocator_statistics* stats_ = nullptr;

  template <class U>
  friend class test_allocator;

public:
  typedef unsigned size_type;
  typedef int difference_type;
  typedef T value_type;
  typedef value_type* pointer;
  typedef const value_type* const_pointer;
  typedef typename std::add_lvalue_reference<value_type>::type reference;
  typedef typename std::add_lvalue_reference<const value_type>::type const_reference;

  template <class U>
  struct rebind {
    typedef test_allocator<U> other;
  };

  TEST_CONSTEXPR test_allocator() TEST_NOEXCEPT = default;

  TEST_CONSTEXPR_CXX14 explicit test_allocator(test_allocator_statistics* stats) TEST_NOEXCEPT : stats_(stats) {
    if (stats_ != nullptr)
      ++stats_->count;
  }

  TEST_CONSTEXPR explicit test_allocator(int data) TEST_NOEXCEPT : data_(data) {}

  TEST_CONSTEXPR_CXX14 explicit test_allocator(int data, test_allocator_statistics* stats) TEST_NOEXCEPT
      : data_(data),
        stats_(stats) {
    if (stats != nullptr)
      ++stats_->count;
  }

  TEST_CONSTEXPR explicit test_allocator(int data, int id) TEST_NOEXCEPT : data_(data), id_(id) {}

  TEST_CONSTEXPR_CXX14 explicit test_allocator(int data, int id, test_allocator_statistics* stats) TEST_NOEXCEPT
      : data_(data),
        id_(id),
        stats_(stats) {
    if (stats_ != nullptr)
      ++stats_->count;
  }

  TEST_CONSTEXPR_CXX14 test_allocator(const test_allocator& a) TEST_NOEXCEPT
      : data_(a.data_),
        id_(a.id_),
        stats_(a.stats_) {
    assert(a.data_ != test_alloc_base::destructed_value && a.id_ != test_alloc_base::destructed_value &&
           "copying from destroyed allocator");
    if (stats_ != nullptr) {
      ++stats_->count;
      ++stats_->copied;
    }
  }

  TEST_CONSTEXPR_CXX14 test_allocator(test_allocator&& a) TEST_NOEXCEPT : data_(a.data_), id_(a.id_), stats_(a.stats_) {
    if (stats_ != nullptr) {
      ++stats_->count;
      ++stats_->moved;
    }
    assert(a.data_ != test_alloc_base::destructed_value && a.id_ != test_alloc_base::destructed_value &&
           "moving from destroyed allocator");
    a.id_ = test_alloc_base::moved_value;
  }

  template <class U>
  TEST_CONSTEXPR_CXX14 test_allocator(const test_allocator<U>& a) TEST_NOEXCEPT
      : data_(a.data_),
        id_(a.id_),
        stats_(a.stats_) {
    if (stats_ != nullptr) {
      ++stats_->count;
      ++stats_->converted;
    }
  }

  TEST_CONSTEXPR_CXX20 ~test_allocator() TEST_NOEXCEPT {
    assert(data_ != test_alloc_base::destructed_value);
    assert(id_ != test_alloc_base::destructed_value);
    if (stats_ != nullptr)
      --stats_->count;
    data_ = test_alloc_base::destructed_value;
    id_   = test_alloc_base::destructed_value;
  }

  TEST_CONSTEXPR pointer address(reference x) const { return &x; }
  TEST_CONSTEXPR const_pointer address(const_reference x) const { return &x; }

  TEST_CONSTEXPR_CXX14 pointer allocate(size_type n, const void* = nullptr) {
    assert(data_ != test_alloc_base::destructed_value);
    if (stats_ != nullptr) {
      if (stats_->time_to_throw >= stats_->throw_after)
        TEST_THROW(std::bad_alloc());
      ++stats_->time_to_throw;
      ++stats_->alloc_count;
      stats_->allocated_size += n;
    }
    return std::allocator<value_type>().allocate(n);
  }

  TEST_CONSTEXPR_CXX14 void deallocate(pointer p, size_type s) {
    assert(data_ != test_alloc_base::destructed_value);
    if (stats_ != nullptr) {
      --stats_->alloc_count;
      stats_->allocated_size -= s;
    }
    std::allocator<value_type>().deallocate(p, s);
  }

  TEST_CONSTEXPR size_type max_size() const TEST_NOEXCEPT { return UINT_MAX / sizeof(T); }

  template <class U>
  TEST_CONSTEXPR_CXX20 void construct(pointer p, U&& val) {
    if (stats_ != nullptr)
      ++stats_->construct_count;
#if TEST_STD_VER > 17
    std::construct_at(std::to_address(p), std::forward<U>(val));
#else
    ::new (static_cast<void*>(p)) T(std::forward<U>(val));
#endif
  }

  TEST_CONSTEXPR_CXX14 void destroy(pointer p) {
    if (stats_ != nullptr)
      ++stats_->destroy_count;
    p->~T();
  }
  TEST_CONSTEXPR friend bool operator==(const test_allocator& x, const test_allocator& y) { return x.data_ == y.data_; }
  TEST_CONSTEXPR friend bool operator!=(const test_allocator& x, const test_allocator& y) { return !(x == y); }

  TEST_CONSTEXPR int get_data() const { return data_; }
  TEST_CONSTEXPR int get_id() const { return id_; }
};

template <>
class test_allocator<void> {
  int data_                         = 0;
  int id_                           = 0;
  test_allocator_statistics* stats_ = nullptr;

  template <class U>
  friend class test_allocator;

public:
  typedef unsigned size_type;
  typedef int difference_type;
  typedef void value_type;
  typedef value_type* pointer;
  typedef const value_type* const_pointer;

  template <class U>
  struct rebind {
    typedef test_allocator<U> other;
  };

  TEST_CONSTEXPR test_allocator() TEST_NOEXCEPT = default;

  TEST_CONSTEXPR_CXX14 explicit test_allocator(test_allocator_statistics* stats) TEST_NOEXCEPT : stats_(stats) {}

  TEST_CONSTEXPR explicit test_allocator(int data) TEST_NOEXCEPT : data_(data) {}

  TEST_CONSTEXPR explicit test_allocator(int data, test_allocator_statistics* stats) TEST_NOEXCEPT
      : data_(data),
        stats_(stats) {}

  TEST_CONSTEXPR explicit test_allocator(int data, int id) : data_(data), id_(id) {}

  TEST_CONSTEXPR_CXX14 explicit test_allocator(int data, int id, test_allocator_statistics* stats) TEST_NOEXCEPT
      : data_(data),
        id_(id),
        stats_(stats) {}

  TEST_CONSTEXPR_CXX14 explicit test_allocator(const test_allocator& a) TEST_NOEXCEPT
      : data_(a.data_),
        id_(a.id_),
        stats_(a.stats_) {}

  template <class U>
  TEST_CONSTEXPR_CXX14 test_allocator(const test_allocator<U>& a) TEST_NOEXCEPT
      : data_(a.data_),
        id_(a.id_),
        stats_(a.stats_) {}

  TEST_CONSTEXPR_CXX20 ~test_allocator() TEST_NOEXCEPT {
    data_ = test_alloc_base::destructed_value;
    id_   = test_alloc_base::destructed_value;
  }

  TEST_CONSTEXPR int get_id() const { return id_; }
  TEST_CONSTEXPR int get_data() const { return data_; }

  TEST_CONSTEXPR friend bool operator==(const test_allocator& x, const test_allocator& y) { return x.data_ == y.data_; }
  TEST_CONSTEXPR friend bool operator!=(const test_allocator& x, const test_allocator& y) { return !(x == y); }
};

template <class T>
class other_allocator {
  int data_ = -1;

  template <class U>
  friend class other_allocator;

public:
  typedef T value_type;

  TEST_CONSTEXPR_CXX14 other_allocator() {}
  TEST_CONSTEXPR_CXX14 explicit other_allocator(int i) : data_(i) {}

  template <class U>
  TEST_CONSTEXPR_CXX14 other_allocator(const other_allocator<U>& a) : data_(a.data_) {}

  TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) { return std::allocator<value_type>().allocate(n); }
  TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t s) { std::allocator<value_type>().deallocate(p, s); }

  TEST_CONSTEXPR_CXX14 other_allocator select_on_container_copy_construction() const { return other_allocator(-2); }

  TEST_CONSTEXPR_CXX14 friend bool operator==(const other_allocator& x, const other_allocator& y) {
    return x.data_ == y.data_;
  }

  TEST_CONSTEXPR_CXX14 friend bool operator!=(const other_allocator& x, const other_allocator& y) { return !(x == y); }
  TEST_CONSTEXPR int get_data() const { return data_; }

  typedef std::true_type propagate_on_container_copy_assignment;
  typedef std::true_type propagate_on_container_move_assignment;
  typedef std::true_type propagate_on_container_swap;

#if TEST_STD_VER < 11
  std::size_t max_size() const { return UINT_MAX / sizeof(T); }
#endif
};

struct Ctor_Tag {};

template <typename T>
class TaggingAllocator;

struct Tag_X {
  // All constructors must be passed the Tag type.

  // DefaultInsertable into vector<X, TaggingAllocator<X>>,
  TEST_CONSTEXPR Tag_X(Ctor_Tag) {}
  // CopyInsertable into vector<X, TaggingAllocator<X>>,
  TEST_CONSTEXPR Tag_X(Ctor_Tag, const Tag_X&) {}
  // MoveInsertable into vector<X, TaggingAllocator<X>>, and
  TEST_CONSTEXPR Tag_X(Ctor_Tag, Tag_X&&) {}

  // EmplaceConstructible into vector<X, TaggingAllocator<X>> from args.
  template <typename... Args>
  TEST_CONSTEXPR Tag_X(Ctor_Tag, Args&&...) {}

  // not DefaultConstructible, CopyConstructible or MoveConstructible.
  Tag_X()             = delete;
  Tag_X(const Tag_X&) = delete;
  Tag_X(Tag_X&&)      = delete;

  // CopyAssignable.
  TEST_CONSTEXPR_CXX14 Tag_X& operator=(const Tag_X&) { return *this; };

  // MoveAssignable.
  TEST_CONSTEXPR_CXX14 Tag_X& operator=(Tag_X&&) { return *this; };

private:
  ~Tag_X() = default;
  // Erasable from vector<X, TaggingAllocator<X>>.
  friend class TaggingAllocator<Tag_X>;
};

template <typename T>
class TaggingAllocator {
public:
  using value_type   = T;
  TaggingAllocator() = default;

  template <typename U>
  TEST_CONSTEXPR TaggingAllocator(const TaggingAllocator<U>&) {}

  template <typename... Args>
  TEST_CONSTEXPR_CXX20 void construct(Tag_X* p, Args&&... args) {
#if TEST_STD_VER > 17
    std::construct_at(p, Ctor_Tag{}, std::forward<Args>(args)...);
#else
    ::new (static_cast<void*>(p)) Tag_X(Ctor_Tag(), std::forward<Args>(args)...);
#endif
  }

  template <typename U>
  TEST_CONSTEXPR_CXX20 void destroy(U* p) {
    p->~U();
  }

  TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) { return std::allocator<T>().allocate(n); }
  TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t n) { std::allocator<T>().deallocate(p, n); }
};

template <std::size_t MaxAllocs>
struct limited_alloc_handle {
  std::size_t outstanding_ = 0;
  void* last_alloc_        = nullptr;

  template <class T>
  TEST_CONSTEXPR_CXX20 T* allocate(std::size_t N) {
    if (N + outstanding_ > MaxAllocs)
      TEST_THROW(std::bad_alloc());
    auto alloc  = std::allocator<T>().allocate(N);
    last_alloc_ = alloc;
    outstanding_ += N;
    return alloc;
  }

  template <class T>
  TEST_CONSTEXPR_CXX20 void deallocate(T* ptr, std::size_t N) {
    if (ptr == last_alloc_) {
      last_alloc_ = nullptr;
      assert(outstanding_ >= N);
      outstanding_ -= N;
    }
    std::allocator<T>().deallocate(ptr, N);
  }
};

namespace detail {
template <class T>
class thread_unsafe_shared_ptr {
public:
  thread_unsafe_shared_ptr() = default;

  TEST_CONSTEXPR_CXX14 thread_unsafe_shared_ptr(const thread_unsafe_shared_ptr& other) : block(other.block) {
    ++block->ref_count;
  }

  TEST_CONSTEXPR_CXX20 ~thread_unsafe_shared_ptr() {
    --block->ref_count;
    if (block->ref_count != 0)
      return;
    typedef std::allocator_traits<std::allocator<control_block> > allocator_traits;
    std::allocator<control_block> alloc;
    allocator_traits::destroy(alloc, block);
    allocator_traits::deallocate(alloc, block, 1);
  }

  TEST_CONSTEXPR const T& operator*() const { return block->content; }
  TEST_CONSTEXPR const T* operator->() const { return &block->content; }
  TEST_CONSTEXPR_CXX14 T& operator*() { return block->content; }
  TEST_CONSTEXPR_CXX14 T* operator->() { return &block->content; }
  TEST_CONSTEXPR_CXX14 T* get() { return &block->content; }
  TEST_CONSTEXPR const T* get() const { return &block->content; }

private:
  struct control_block {
    template <class... Args>
    TEST_CONSTEXPR control_block(Args... args) : content(std::forward<Args>(args)...) {}
    std::size_t ref_count = 1;
    T content;
  };

  control_block* block = nullptr;

  template <class U, class... Args>
  friend TEST_CONSTEXPR_CXX20 thread_unsafe_shared_ptr<U> make_thread_unsafe_shared(Args...);
};

template <class T, class... Args>
TEST_CONSTEXPR_CXX20 thread_unsafe_shared_ptr<T> make_thread_unsafe_shared(Args... args) {
  typedef typename thread_unsafe_shared_ptr<T>::control_block control_block_type;
  typedef std::allocator_traits<std::allocator<control_block_type> > allocator_traits;

  thread_unsafe_shared_ptr<T> ptr;
  std::allocator<control_block_type> alloc;
  ptr.block = allocator_traits::allocate(alloc, 1);
  allocator_traits::construct(alloc, ptr.block, std::forward<Args>(args)...);

  return ptr;
}
} // namespace detail

template <class T, std::size_t N>
class limited_allocator {
  template <class U, std::size_t UN>
  friend class limited_allocator;
  typedef limited_alloc_handle<N> BuffT;
  detail::thread_unsafe_shared_ptr<BuffT> handle_;

public:
  typedef T value_type;
  typedef value_type* pointer;
  typedef const value_type* const_pointer;
  typedef value_type& reference;
  typedef const value_type& const_reference;
  typedef std::size_t size_type;
  typedef std::ptrdiff_t difference_type;

  template <class U>
  struct rebind {
    typedef limited_allocator<U, N> other;
  };

  TEST_CONSTEXPR_CXX20 limited_allocator() : handle_(detail::make_thread_unsafe_shared<BuffT>()) {}

  limited_allocator(limited_allocator const&) = default;

  template <class U>
  TEST_CONSTEXPR explicit limited_allocator(limited_allocator<U, N> const& other) : handle_(other.handle_) {}

  limited_allocator& operator=(const limited_allocator&) = delete;

  TEST_CONSTEXPR_CXX20 pointer allocate(size_type n) { return handle_->template allocate<T>(n); }
  TEST_CONSTEXPR_CXX20 void deallocate(pointer p, size_type n) { handle_->template deallocate<T>(p, n); }
  TEST_CONSTEXPR size_type max_size() const { return N; }
  TEST_CONSTEXPR const BuffT* getHandle() const { return handle_.get(); }
};

template <class T, class U, std::size_t N>
TEST_CONSTEXPR inline bool operator==(limited_allocator<T, N> const& LHS, limited_allocator<U, N> const& RHS) {
  return LHS.getHandle() == RHS.getHandle();
}

template <class T, class U, std::size_t N>
TEST_CONSTEXPR inline bool operator!=(limited_allocator<T, N> const& LHS, limited_allocator<U, N> const& RHS) {
  return !(LHS == RHS);
}

// Track the "provenance" of this allocator instance: how many times was
// select_on_container_copy_construction called in order to produce it?
//
template <class T>
struct SocccAllocator {
  using value_type = T;

  int count_ = 0;
  explicit SocccAllocator(int i) : count_(i) {}

  template <class U>
  SocccAllocator(const SocccAllocator<U>& a) : count_(a.count_) {}

  T* allocate(std::size_t n) { return std::allocator<T>().allocate(n); }
  void deallocate(T* p, std::size_t n) { std::allocator<T>().deallocate(p, n); }

  SocccAllocator select_on_container_copy_construction() const { return SocccAllocator(count_ + 1); }

  bool operator==(const SocccAllocator&) const { return true; }

  using propagate_on_container_copy_assignment = std::false_type;
  using propagate_on_container_move_assignment = std::false_type;
  using propagate_on_container_swap            = std::false_type;
};

#endif // TEST_ALLOCATOR_H
