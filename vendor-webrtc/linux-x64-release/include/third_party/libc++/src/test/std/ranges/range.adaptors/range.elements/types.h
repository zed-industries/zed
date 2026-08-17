//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_ELEMENTS_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_ELEMENTS_TYPES_H

#include <array>
#include <functional>
#include <ranges>
#include <tuple>

#include "test_macros.h"
#include "test_iterators.h"
#include "test_range.h"

template <class T>
struct BufferView : std::ranges::view_base {
  T* buffer_;
  std::size_t size_;

  template <std::size_t N>
  constexpr BufferView(T (&b)[N]) : buffer_(b), size_(N) {}

  template <std::size_t N>
  constexpr BufferView(std::array<T, N>& arr) : buffer_(arr.data()), size_(N) {}
};

using TupleBufferView = BufferView<std::tuple<int>>;

template <bool Simple>
struct Common : TupleBufferView {
  using TupleBufferView::TupleBufferView;

  constexpr std::tuple<int>* begin()
    requires(!Simple)
  {
    return buffer_;
  }
  constexpr const std::tuple<int>* begin() const { return buffer_; }
  constexpr std::tuple<int>* end()
    requires(!Simple)
  {
    return buffer_ + size_;
  }
  constexpr const std::tuple<int>* end() const { return buffer_ + size_; }
};
using SimpleCommon    = Common<true>;
using NonSimpleCommon = Common<false>;

using SimpleCommonRandomAccessSized    = SimpleCommon;
using NonSimpleCommonRandomAccessSized = NonSimpleCommon;

static_assert(std::ranges::common_range<Common<true>>);
static_assert(std::ranges::random_access_range<SimpleCommon>);
static_assert(std::ranges::sized_range<SimpleCommon>);
static_assert(simple_view<SimpleCommon>);
static_assert(!simple_view<NonSimpleCommon>);

template <bool Simple>
struct NonCommon : TupleBufferView {
  using TupleBufferView::TupleBufferView;
  constexpr std::tuple<int>* begin()
    requires(!Simple)
  {
    return buffer_;
  }
  constexpr const std::tuple<int>* begin() const { return buffer_; }
  constexpr sentinel_wrapper<std::tuple<int>*> end()
    requires(!Simple)
  {
    return sentinel_wrapper<std::tuple<int>*>(buffer_ + size_);
  }
  constexpr sentinel_wrapper<const std::tuple<int>*> end() const {
    return sentinel_wrapper<const std::tuple<int>*>(buffer_ + size_);
  }
};

using SimpleNonCommon    = NonCommon<true>;
using NonSimpleNonCommon = NonCommon<false>;

static_assert(!std::ranges::common_range<SimpleNonCommon>);
static_assert(std::ranges::random_access_range<SimpleNonCommon>);
static_assert(!std::ranges::sized_range<SimpleNonCommon>);
static_assert(simple_view<SimpleNonCommon>);
static_assert(!simple_view<NonSimpleNonCommon>);

template <class Derived>
struct IterBase {
  using iterator_concept = std::random_access_iterator_tag;
  using value_type       = std::tuple<int>;
  using difference_type  = std::intptr_t;

  constexpr std::tuple<int> operator*() const { return std::tuple<int>(5); }

  constexpr Derived& operator++() { return *this; }
  constexpr void operator++(int) {}

  friend constexpr bool operator==(const IterBase&, const IterBase&) = default;
};

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_ELEMENTS_TYPES_H
