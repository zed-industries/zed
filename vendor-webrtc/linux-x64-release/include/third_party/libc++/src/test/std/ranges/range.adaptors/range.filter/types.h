//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_FILTER_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_FILTER_TYPES_H

#include <ranges>
#include <utility>

struct TrackInitialization {
  constexpr explicit TrackInitialization(bool* moved, bool* copied) : moved_(moved), copied_(copied) { }
  constexpr TrackInitialization(TrackInitialization const& other) : moved_(other.moved_), copied_(other.copied_) {
    *copied_ = true;
  }
  constexpr TrackInitialization(TrackInitialization&& other) : moved_(other.moved_), copied_(other.copied_) {
    *moved_ = true;
  }
  TrackInitialization& operator=(TrackInitialization const&) = default;
  TrackInitialization& operator=(TrackInitialization&&) = default;
  bool* moved_;
  bool* copied_;
};

struct AlwaysTrue {
  template <typename T>
  constexpr bool operator()(T const&) const { return true; }
};

template <class Iter, class Sent>
struct minimal_view : std::ranges::view_base {
  constexpr explicit minimal_view(Iter it, Sent sent)
    : it_(base(std::move(it)))
    , sent_(base(std::move(sent)))
  { }

  minimal_view(minimal_view&&) = default;
  minimal_view& operator=(minimal_view&&) = default;

  constexpr Iter begin() const { return Iter(it_); }
  constexpr Sent end() const { return Sent(sent_); }

private:
  decltype(base(std::declval<Iter>())) it_;
  decltype(base(std::declval<Sent>())) sent_;
};

template <bool IsNoexcept>
class NoexceptIterMoveInputIterator {
  int *it_;

public:
  using iterator_category = std::input_iterator_tag;
  using value_type = int;
  using difference_type = typename std::iterator_traits<int *>::difference_type;
  using pointer = int*;
  using reference = int&;

  NoexceptIterMoveInputIterator() = default;
  explicit constexpr NoexceptIterMoveInputIterator(int *it) : it_(it) {}

  friend constexpr decltype(auto) iter_move(const NoexceptIterMoveInputIterator& it) noexcept(IsNoexcept) {
    return std::ranges::iter_move(it.it_);
  }

  friend constexpr int* base(const NoexceptIterMoveInputIterator& i) { return i.it_; }

  constexpr reference operator*() const { return *it_; }
  constexpr NoexceptIterMoveInputIterator& operator++() {++it_; return *this;}
  constexpr NoexceptIterMoveInputIterator operator++(int)
  { NoexceptIterMoveInputIterator tmp(*this); ++(*this); return tmp; }
};

template <bool IsNoexcept>
class NoexceptIterSwapInputIterator {
  int *it_;

public:
  using iterator_category = std::input_iterator_tag;
  using value_type = int;
  using difference_type = typename std::iterator_traits<int *>::difference_type;
  using pointer = int*;
  using reference = int&;

  NoexceptIterSwapInputIterator() = default;
  explicit constexpr NoexceptIterSwapInputIterator(int *it) : it_(it) {}

  friend constexpr void iter_swap(const NoexceptIterSwapInputIterator& a, const NoexceptIterSwapInputIterator& b) noexcept(IsNoexcept) {
    return std::ranges::iter_swap(a.it_, b.it_);
  }

  friend constexpr int* base(const NoexceptIterSwapInputIterator& i) { return i.it_; }

  constexpr reference operator*() const { return *it_; }
  constexpr NoexceptIterSwapInputIterator& operator++() {++it_; return *this;}
  constexpr NoexceptIterSwapInputIterator operator++(int)
  { NoexceptIterSwapInputIterator tmp(*this); ++(*this); return tmp; }
};

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_FILTER_TYPES_H
