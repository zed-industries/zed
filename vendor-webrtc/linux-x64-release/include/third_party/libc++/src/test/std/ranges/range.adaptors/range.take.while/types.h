//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TAKE_WHILE_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TAKE_WHILE_TYPES_H

#include <array>
#include <functional>
#include <ranges>

#include "test_macros.h"
#include "test_iterators.h"
#include "test_range.h"

template <class T>
struct BufferViewBase : std::ranges::view_base {
  T* buffer_;
  std::size_t size_;

  template <std::size_t N>
  constexpr BufferViewBase(T (&b)[N]) : buffer_(b), size_(N) {}

  template <std::size_t N>
  constexpr BufferViewBase(std::array<T, N>& arr) : buffer_(arr.data()), size_(N) {}
};

using IntBufferViewBase = BufferViewBase<int>;

struct SimpleView : IntBufferViewBase {
  using IntBufferViewBase::IntBufferViewBase;
  constexpr int* begin() const { return buffer_; }
  constexpr int* end() const { return buffer_ + size_; }
};
static_assert(simple_view<SimpleView>);

struct ConstNotRange : IntBufferViewBase {
  using IntBufferViewBase::IntBufferViewBase;
  constexpr int* begin() { return buffer_; }
  constexpr int* end() { return buffer_ + size_; }
};
static_assert(std::ranges::view<ConstNotRange>);
static_assert(!std::ranges::range<const ConstNotRange>);

struct NonSimple : IntBufferViewBase {
  using IntBufferViewBase::IntBufferViewBase;
  constexpr const int* begin() const { return buffer_; }
  constexpr const int* end() const { return buffer_ + size_; }
  constexpr int* begin() { return buffer_; }
  constexpr int* end() { return buffer_ + size_; }
};
static_assert(std::ranges::view<NonSimple>);
static_assert(!simple_view<NonSimple>);

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TAKE_WHILE_TYPES_H
