//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_CHUNK_BY_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_CHUNK_BY_TYPES_H

#include <ranges>
#include <utility>

#include "test_iterators.h"

struct TrackInitialization {
  constexpr explicit TrackInitialization(bool* moved, bool* copied) : moved_(moved), copied_(copied) {}
  constexpr TrackInitialization(TrackInitialization const& other) : moved_(other.moved_), copied_(other.copied_) {
    *copied_ = true;
  }
  constexpr TrackInitialization(TrackInitialization&& other) : moved_(other.moved_), copied_(other.copied_) {
    *moved_ = true;
  }
  TrackInitialization& operator=(TrackInitialization const&) = default;
  TrackInitialization& operator=(TrackInitialization&&)      = default;
  bool* moved_;
  bool* copied_;
};

enum class IsConst : bool { no, yes };

template <std::forward_iterator Iter, std::sentinel_for<Iter> Sent = sentinel_wrapper<Iter>>
struct View : std::ranges::view_base {
  constexpr explicit View(Iter b, Sent e) : begin_(b), end_(e) {}
  constexpr Iter begin() { return begin_; }
  constexpr Sent end() { return end_; }

private:
  Iter begin_;
  Sent end_;
};

template <class I, class S>
View(I b, S e) -> View<I, S>;

struct IntWrapper {
  constexpr IntWrapper(int v) : value_(v) {}

  int value_;
  constexpr bool lessEqual(IntWrapper other) const { return value_ <= other.value_; }
};

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_CHUNK_BY_TYPES_H
