//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_NAIVE_STATIC_VECTOR_H
#define SUPPORT_NAIVE_STATIC_VECTOR_H

#include <cstddef>
#include <utility>
#include "test_iterators.h"
#include "test_macros.h"

template <class T, std::size_t N>
struct NaiveStaticVector {
  struct CapacityError {};

  using value_type      = T;
  using difference_type = short;
  using size_type       = unsigned short;
  using iterator        = random_access_iterator<T*>;
  using const_iterator  = random_access_iterator<const T*>;

  explicit NaiveStaticVector() = default;
  template <class It>
  explicit NaiveStaticVector(It first, It last) {
    while (first != last)
      insert(*first++);
  }

  // Moving-from a NaiveStaticVector leaves the source vector holding moved-from objects.
  // This is intentional (the "Naive" in the name).
  // Specifically, moving-out-of a sorted+uniqued NaiveStaticVector<MoveOnly>
  // will leave it in a non-sorted+uniqued state.

  NaiveStaticVector(const NaiveStaticVector&)            = default;
  NaiveStaticVector(NaiveStaticVector&&)                 = default; // deliberately don't reset size_
  NaiveStaticVector& operator=(const NaiveStaticVector&) = default;
  NaiveStaticVector& operator=(NaiveStaticVector&&)      = default;

  iterator begin() { return iterator(data_); }
  const_iterator begin() const { return const_iterator(data_); }
  const_iterator cbegin() const { return const_iterator(data_); }
  iterator end() { return begin() + size(); }
  const_iterator end() const { return begin() + size(); }
  size_type size() const { return size_; }
  bool empty() const { return size_ == 0; }

  void clear() { size_ = 0; }

  template <class It>
  iterator insert(const_iterator pos, It first, It last) {
    iterator result = pos - cbegin() + begin();
    while (first != last) {
      insert(pos++, *first++);
    }
    return result;
  }

  iterator insert(const_iterator pos, T value) {
    if (size_ == N) {
      throw CapacityError();
    }
    int i = pos - cbegin();
    size_ += 1;
    std::move_backward(&data_[i], &data_[size_ - 1], &data_[size_]);
    data_[i] = std::move(value);
    return begin() + i;
  }

  template <class... Args>
  iterator emplace(const_iterator pos, Args&&... args) {
    return insert(pos, T(std::forward<Args>(args)...));
  }

  iterator erase(const_iterator first, const_iterator last) {
    int i = first - cbegin();
    int j = last - cbegin();
    std::move(&data_[j], &data_[size_], &data_[i]);
    size_ -= (last - first);
    return begin() + i;
  }

  iterator erase(const_iterator pos) { return erase(pos, std::next(pos)); }

private:
  T data_[N];
  std::size_t size_ = 0;
};

#endif // SUPPORT_NAIVE_STATIC_VECTOR_H
