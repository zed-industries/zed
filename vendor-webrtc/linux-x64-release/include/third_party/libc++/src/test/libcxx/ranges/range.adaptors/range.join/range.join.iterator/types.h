//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_JOIN_RANGE_JOIN_ITERATOR_TYPES_H
#define TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_JOIN_RANGE_JOIN_ITERATOR_TYPES_H

#include <cassert>
#include <cstddef>
#include <ranges>

#include "test_iterators.h"

template <std::input_iterator Iter>
struct DieOnCopyIterator {
  using value_type      = std::iter_value_t<Iter>;
  using difference_type = std::iter_difference_t<Iter>;

  DieOnCopyIterator()
    requires std::default_initializable<Iter>
  = default;

  constexpr explicit DieOnCopyIterator(Iter iter) : iter_(std::move(iter)) {}
  constexpr DieOnCopyIterator(DieOnCopyIterator&& other) = default;
  DieOnCopyIterator& operator=(DieOnCopyIterator&&)      = default;

  constexpr DieOnCopyIterator(const DieOnCopyIterator&) { assert(false); }
  constexpr DieOnCopyIterator& operator=(const DieOnCopyIterator&) { assert(false); }

  constexpr DieOnCopyIterator& operator++() {
    ++iter_;
    return *this;
  }

  constexpr void operator++(int) { iter_++; }

  constexpr DieOnCopyIterator operator++(int)
    requires std::forward_iterator<Iter>
  {
    auto tmp = *this;
    ++tmp;
    return tmp;
  }

  constexpr decltype(auto) operator*() const { return *iter_; }

  friend constexpr bool operator==(const DieOnCopyIterator& left, const DieOnCopyIterator& right)
    requires std::equality_comparable<Iter>
  {
    return left.iter_ == right.iter_;
  }

  friend constexpr bool operator==(const DieOnCopyIterator& it, const sentinel_wrapper<Iter>& se) {
    return it.iter_ == se;
  }

private:
  Iter iter_ = Iter();
};

template <class Iter>
explicit DieOnCopyIterator(Iter) -> DieOnCopyIterator<Iter>;

static_assert(std::input_iterator<DieOnCopyIterator<cpp20_input_iterator<int*>>>);
static_assert(!std::forward_iterator<DieOnCopyIterator<cpp20_input_iterator<int*>>>);
static_assert(std::forward_iterator<DieOnCopyIterator<int*>>);
static_assert(!std::bidirectional_iterator<DieOnCopyIterator<int*>>);
static_assert(std::sentinel_for<sentinel_wrapper<int*>, DieOnCopyIterator<int*>>);

template <std::input_iterator Iter, std::sentinel_for<Iter> Sent = Iter>
struct MoveOnAccessSubrange : std::ranges::view_base {
  constexpr explicit MoveOnAccessSubrange(Iter iter, Sent sent) : iter_(std::move(iter)), sent_(std::move(sent)) {}

  MoveOnAccessSubrange(MoveOnAccessSubrange&&)            = default;
  MoveOnAccessSubrange& operator=(MoveOnAccessSubrange&&) = default;

  MoveOnAccessSubrange(const MoveOnAccessSubrange&)            = delete;
  MoveOnAccessSubrange& operator=(const MoveOnAccessSubrange&) = delete;

  constexpr Iter begin() { return std::move(iter_); }
  constexpr Sent end() { return std::move(sent_); }

private:
  Iter iter_;
  Sent sent_;
};

template <class Iter, class Sent>
MoveOnAccessSubrange(Iter, Sent) -> MoveOnAccessSubrange<Iter, Sent>;

static_assert(std::ranges::input_range<MoveOnAccessSubrange<int*, sentinel_wrapper<int*>>>);
static_assert(std::ranges::forward_range<MoveOnAccessSubrange<DieOnCopyIterator<int*>>>);

template <class Iter, class Sent>
  requires(!std::same_as<Iter, Sent>)
struct BufferView : std::ranges::view_base {
  using T = std::iter_value_t<Iter>;
  T* data_;
  std::size_t size_;

  template <std::size_t N>
  constexpr BufferView(T (&b)[N]) : data_(b), size_(N) {}

  constexpr Iter begin() const { return Iter(data_); }
  constexpr Sent end() const { return Sent(Iter(data_ + size_)); }
};

static_assert(std::ranges::input_range<BufferView<int*, sentinel_wrapper<int*>>>);

#endif // TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_JOIN_RANGE_JOIN_ITERATOR_TYPES_H
