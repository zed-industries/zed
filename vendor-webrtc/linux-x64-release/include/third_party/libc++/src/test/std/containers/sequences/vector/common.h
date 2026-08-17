//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_CONTAINERS_SEQUENCES_VECTOR_COMMON_H
#define TEST_STD_CONTAINERS_SEQUENCES_VECTOR_COMMON_H

#include <array>
#include <cassert>
#include <cstddef>
#include <cstdlib>
#include <memory>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "count_new.h"
#include "test_macros.h"

struct throwing_t {
  int* throw_after_n_ = nullptr;
  throwing_t() { throw 0; }

  explicit throwing_t(int& throw_after_n) : throw_after_n_(&throw_after_n) {
    if (throw_after_n == 0)
      throw 0;
    --throw_after_n;
  }

  throwing_t(const throwing_t& rhs) : throw_after_n_(rhs.throw_after_n_) {
    if (throw_after_n_ == nullptr || *throw_after_n_ == 0)
      throw 1;
    --*throw_after_n_;
  }

  throwing_t& operator=(const throwing_t& rhs) {
    throw_after_n_ = rhs.throw_after_n_;
    if (throw_after_n_ == nullptr || *throw_after_n_ == 0)
      throw 1;
    --*throw_after_n_;
    return *this;
  }

  friend bool operator==(const throwing_t& lhs, const throwing_t& rhs) {
    return lhs.throw_after_n_ == rhs.throw_after_n_;
  }
  friend bool operator!=(const throwing_t& lhs, const throwing_t& rhs) {
    return lhs.throw_after_n_ != rhs.throw_after_n_;
  }
};

#if TEST_STD_VER >= 11

template <typename T>
struct move_only_throwing_t {
  T data_;
  int* throw_after_n_ = nullptr;
  bool moved_from_    = false;

  move_only_throwing_t() = default;

  explicit move_only_throwing_t(const T& data, int& throw_after_n) : data_(data), throw_after_n_(&throw_after_n) {
    if (throw_after_n == 0)
      throw 1;
    --throw_after_n;
  }

  explicit move_only_throwing_t(T&& data, int& throw_after_n) : data_(std::move(data)), throw_after_n_(&throw_after_n) {
    if (throw_after_n == 0)
      throw 1;
    --throw_after_n;
  }

  move_only_throwing_t(const move_only_throwing_t&)            = delete;
  move_only_throwing_t& operator=(const move_only_throwing_t&) = delete;

  move_only_throwing_t(move_only_throwing_t&& rhs) : data_(std::move(rhs.data_)), throw_after_n_(rhs.throw_after_n_) {
    rhs.throw_after_n_ = nullptr;
    rhs.moved_from_    = true;
    if (throw_after_n_ == nullptr || *throw_after_n_ == 0)
      throw 1;
    --*throw_after_n_;
  }

  move_only_throwing_t& operator=(move_only_throwing_t&& rhs) {
    if (this == &rhs)
      return *this;
    data_              = std::move(rhs.data_);
    throw_after_n_     = rhs.throw_after_n_;
    rhs.moved_from_    = true;
    rhs.throw_after_n_ = nullptr;
    if (throw_after_n_ == nullptr || *throw_after_n_ == 0)
      throw 1;
    --*throw_after_n_;
    return *this;
  }

  friend bool operator==(const move_only_throwing_t& lhs, const move_only_throwing_t& rhs) {
    return lhs.data_ == rhs.data_;
  }
  friend bool operator!=(const move_only_throwing_t& lhs, const move_only_throwing_t& rhs) {
    return lhs.data_ != rhs.data_;
  }
};

#endif

template <typename T>
struct throwing_data {
  T data_;
  int* throw_after_n_ = nullptr;
  throwing_data() { throw 0; }

  throwing_data(const T& data, int& throw_after_n) : data_(data), throw_after_n_(&throw_after_n) {
    if (throw_after_n == 0)
      throw 0;
    --throw_after_n;
  }

  throwing_data(const throwing_data& rhs) : data_(rhs.data_), throw_after_n_(rhs.throw_after_n_) {
    if (throw_after_n_ == nullptr || *throw_after_n_ == 0)
      throw 1;
    --*throw_after_n_;
  }

  throwing_data& operator=(const throwing_data& rhs) {
    data_          = rhs.data_;
    throw_after_n_ = rhs.throw_after_n_;
    if (throw_after_n_ == nullptr || *throw_after_n_ == 0)
      throw 1;
    --*throw_after_n_;
    return *this;
  }

  friend bool operator==(const throwing_data& lhs, const throwing_data& rhs) {
    return lhs.data_ == rhs.data_ && lhs.throw_after_n_ == rhs.throw_after_n_;
  }
  friend bool operator!=(const throwing_data& lhs, const throwing_data& rhs) { return !(lhs == rhs); }
};

template <class T>
struct throwing_allocator {
  using value_type = T;

  bool throw_on_copy_ = false;

  explicit throwing_allocator(bool throw_on_ctor = true) {
    if (throw_on_ctor)
      throw 0;
  }

  explicit throwing_allocator(bool throw_on_ctor, bool throw_on_copy) : throw_on_copy_(throw_on_copy) {
    if (throw_on_ctor)
      throw 0;
  }

  throwing_allocator(const throwing_allocator& rhs) : throw_on_copy_(rhs.throw_on_copy_) {
    if (throw_on_copy_)
      throw 0;
  }

  template <class U>
  throwing_allocator(const throwing_allocator<U>& rhs) : throw_on_copy_(rhs.throw_on_copy_) {
    if (throw_on_copy_)
      throw 0;
  }

  T* allocate(std::size_t n) { return std::allocator<T>().allocate(n); }
  void deallocate(T* ptr, std::size_t n) { std::allocator<T>().deallocate(ptr, n); }

  template <class U>
  friend bool operator==(const throwing_allocator&, const throwing_allocator<U>&) {
    return true;
  }
};

template <class T, class IterCat>
struct throwing_iterator {
  using iterator_category = IterCat;
  using difference_type   = std::ptrdiff_t;
  using value_type        = T;
  using reference         = T&;
  using pointer           = T*;

  int i_;
  T v_;

  explicit throwing_iterator(int i = 0, const T& v = T()) : i_(i), v_(v) {}

  reference operator*() {
    if (i_ == 1)
      throw 1;
    return v_;
  }

  friend bool operator==(const throwing_iterator& lhs, const throwing_iterator& rhs) { return lhs.i_ == rhs.i_; }
  friend bool operator!=(const throwing_iterator& lhs, const throwing_iterator& rhs) { return lhs.i_ != rhs.i_; }

  throwing_iterator& operator++() {
    ++i_;
    return *this;
  }

  throwing_iterator operator++(int) {
    auto tmp = *this;
    ++i_;
    return tmp;
  }
};

inline void check_new_delete_called() {
  assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  assert(globalMemCounter.new_array_called == globalMemCounter.delete_array_called);
  assert(globalMemCounter.aligned_new_called == globalMemCounter.aligned_delete_called);
  assert(globalMemCounter.aligned_new_array_called == globalMemCounter.aligned_delete_array_called);
}

template <class T, typename Alloc>
void use_unspecified_but_valid_state_vector(std::vector<T, Alloc> const& v) {
  assert(v.size() >= 0); // make sure it can be called
  assert(v.capacity() >= 0);
  assert(v.empty() || !v.empty());
  for (auto it = v.begin(); it != v.end(); ++it) {
    auto& element = *it;
    (void)element;
  }
}

static const std::array<char, 62> letters = {
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};

inline std::string getString(std::size_t n, std::size_t len) {
  std::string s;
  s.reserve(len);
  for (std::size_t i = 0; i < len; ++i)
    s += letters[(i * i + n) % letters.size()];
  return s;
}

inline std::vector<int> getIntegerInputs(std::size_t n) {
  std::vector<int> v;
  v.reserve(n);
  for (std::size_t i = 0; i < n; ++i)
    v.push_back(static_cast<int>(i * i + n));
  return v;
}

inline std::vector<std::string> getStringInputsWithLength(std::size_t n, std::size_t len) {
  std::vector<std::string> v;
  v.reserve(n);
  for (std::size_t i = 0; i < n; ++i)
    v.push_back(getString(i, len));
  return v;
}

#endif // TEST_STD_CONTAINERS_SEQUENCES_VECTOR_COMMON_H
