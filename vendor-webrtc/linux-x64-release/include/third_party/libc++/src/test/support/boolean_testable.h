//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCXX_TEST_SUPPORT_BOOLEAN_TESTABLE_H
#define LIBCXX_TEST_SUPPORT_BOOLEAN_TESTABLE_H

#include "test_macros.h"

#include <iterator>
#include <utility>

#if TEST_STD_VER > 17

struct BooleanTestable {
  constexpr operator bool() const { return value_; }

  friend constexpr BooleanTestable operator==(const BooleanTestable& lhs, const BooleanTestable& rhs) {
    return lhs.value_ == rhs.value_;
  }

  friend constexpr BooleanTestable operator!=(const BooleanTestable& lhs, const BooleanTestable& rhs) {
    return lhs.value_ != rhs.value_;
  }

  constexpr BooleanTestable&& operator!() && {
    value_ = !value_;
    return std::move(*this);
  }

  // this class should behave like a bool, so the constructor shouldn't be explicit
  constexpr BooleanTestable(bool value) : value_{value} {}
  constexpr BooleanTestable(const BooleanTestable&) = delete;
  constexpr BooleanTestable(BooleanTestable&&)      = delete;

private:
  bool value_;
};

static constexpr BooleanTestable yes(true);
static constexpr BooleanTestable no(false);

template <class T>
struct StrictComparable {
  StrictComparable() = default;

  // this shouldn't be explicit to make it easier to initialize inside arrays (which it almost always is)
  constexpr StrictComparable(T value) : value_{value} {}

  friend constexpr BooleanTestable const& operator==(StrictComparable const& a, StrictComparable const& b) {
    return a.value_ == b.value_ ? yes : no;
  }

  friend constexpr BooleanTestable const& operator!=(StrictComparable const& a, StrictComparable const& b) {
    return a.value_ != b.value_ ? yes : no;
  }

  friend constexpr BooleanTestable const& operator<(StrictComparable const& a, StrictComparable const& b) {
    return a.value_ < b.value_ ? yes : no;
  }
  friend constexpr BooleanTestable const& operator<=(StrictComparable const& a, StrictComparable const& b) {
    return a.value_ <= b.value_ ? yes : no;
  }
  friend constexpr BooleanTestable const& operator>(StrictComparable const& a, StrictComparable const& b) {
    return a.value_ > b.value_ ? yes : no;
  }
  friend constexpr BooleanTestable const& operator>=(StrictComparable const& a, StrictComparable const& b) {
    return a.value_ >= b.value_ ? yes : no;
  }

  T value_;
};

auto StrictUnaryPredicate = []<class T>(StrictComparable<T> const& x) -> BooleanTestable const& {
  return x.value_ < 0 ? yes : no;
};

auto StrictBinaryPredicate =
    []<class T>(StrictComparable<T> const& x, StrictComparable<T> const& y) -> BooleanTestable const& {
  return x.value_ < y.value_ ? yes : no;
};

template <class It>
struct StrictBooleanIterator {
  using value_type                  = typename std::iterator_traits<It>::value_type;
  using reference                   = typename std::iterator_traits<It>::reference;
  using difference_type             = typename std::iterator_traits<It>::difference_type;
  constexpr StrictBooleanIterator() = default;
  constexpr explicit StrictBooleanIterator(It it) : iter_(it) {}
  constexpr reference operator*() const { return *iter_; }
  constexpr reference operator[](difference_type n) const { return iter_[n]; }
  constexpr StrictBooleanIterator& operator++() {
    ++iter_;
    return *this;
  }
  constexpr StrictBooleanIterator operator++(int) {
    auto copy = *this;
    ++iter_;
    return copy;
  }
  constexpr StrictBooleanIterator& operator--() {
    --iter_;
    return *this;
  }
  constexpr StrictBooleanIterator operator--(int) {
    auto copy = *this;
    --iter_;
    return copy;
  }
  constexpr StrictBooleanIterator& operator+=(difference_type n) {
    iter_ += n;
    return *this;
  }
  constexpr StrictBooleanIterator& operator-=(difference_type n) {
    iter_ -= n;
    return *this;
  }
  friend constexpr StrictBooleanIterator operator+(StrictBooleanIterator x, difference_type n) {
    x += n;
    return x;
  }
  friend constexpr StrictBooleanIterator operator+(difference_type n, StrictBooleanIterator x) {
    x += n;
    return x;
  }
  friend constexpr StrictBooleanIterator operator-(StrictBooleanIterator x, difference_type n) {
    x -= n;
    return x;
  }
  friend constexpr difference_type operator-(StrictBooleanIterator x, StrictBooleanIterator y) {
    return x.iter_ - y.iter_;
  }
  constexpr BooleanTestable const& operator==(StrictBooleanIterator const& other) const {
    return iter_ == other.iter_ ? yes : no;
  }
  constexpr BooleanTestable const& operator!=(StrictBooleanIterator const& other) const {
    return iter_ != other.iter_ ? yes : no;
  }
  constexpr BooleanTestable const& operator<(StrictBooleanIterator const& other) const {
    return iter_ < other.iter_ ? yes : no;
  }
  constexpr BooleanTestable const& operator<=(StrictBooleanIterator const& other) const {
    return iter_ <= other.iter_ ? yes : no;
  }
  constexpr BooleanTestable const& operator>(StrictBooleanIterator const& other) const {
    return iter_ > other.iter_ ? yes : no;
  }
  constexpr BooleanTestable const& operator>=(StrictBooleanIterator const& other) const {
    return iter_ >= other.iter_ ? yes : no;
  }

private:
  It iter_;
};
static_assert(std::forward_iterator<StrictBooleanIterator<int*>>);
static_assert(std::sentinel_for<StrictBooleanIterator<int*>, StrictBooleanIterator<int*>>);

#endif // TEST_STD_VER > 17

#endif // LIBCXX_TEST_SUPPORT_BOOLEAN_TESTABLE_H
