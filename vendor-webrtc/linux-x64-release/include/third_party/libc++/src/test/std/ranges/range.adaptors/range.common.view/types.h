//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_COMMON_VIEW_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_COMMON_VIEW_TYPES_H

#include <ranges>

#include "test_iterators.h"

struct DefaultConstructibleView : std::ranges::view_base {
  int* begin_ = nullptr;
  int* end_ = nullptr;
  explicit DefaultConstructibleView() = default;
  constexpr int *begin() const { return begin_; }
  constexpr auto end() const { return sentinel_wrapper<int*>(end_); }
};
static_assert(std::ranges::view<DefaultConstructibleView>);
static_assert(std::default_initializable<DefaultConstructibleView>);

struct MoveOnlyView : std::ranges::view_base {
  int* begin_;
  int* end_;
  constexpr explicit MoveOnlyView(int* b, int* e) : begin_(b), end_(e) { }
  constexpr MoveOnlyView(MoveOnlyView&&) = default;
  constexpr MoveOnlyView& operator=(MoveOnlyView&&) = default;
  constexpr int *begin() const { return begin_; }
  constexpr auto end() const { return sentinel_wrapper<int*>(end_); }
};
static_assert( std::ranges::view<MoveOnlyView>);
static_assert( std::ranges::contiguous_range<MoveOnlyView>);
static_assert(!std::copyable<MoveOnlyView>);

struct CopyableView : std::ranges::view_base {
  int* begin_;
  int* end_;
  constexpr explicit CopyableView(int* b, int* e) : begin_(b), end_(e) { }
  constexpr int *begin() const { return begin_; }
  constexpr auto end() const { return sentinel_wrapper<int*>(end_); }
};
static_assert(std::ranges::view<CopyableView>);
static_assert(std::copyable<CopyableView>);

using ForwardIter = forward_iterator<int*>;
struct SizedForwardView : std::ranges::view_base {
  int* begin_;
  int* end_;
  constexpr explicit SizedForwardView(int* b, int* e) : begin_(b), end_(e) { }
  constexpr auto begin() const { return forward_iterator<int*>(begin_); }
  constexpr auto end() const { return sized_sentinel<forward_iterator<int*>>(forward_iterator<int*>(end_)); }
};
static_assert(std::ranges::view<SizedForwardView>);
static_assert(std::ranges::forward_range<SizedForwardView>);
static_assert(std::ranges::sized_range<SizedForwardView>);

using RandomAccessIter = random_access_iterator<int*>;
struct SizedRandomAccessView : std::ranges::view_base {
  int* begin_;
  int* end_;
  constexpr explicit SizedRandomAccessView(int* b, int* e) : begin_(b), end_(e) { }
  constexpr auto begin() const { return random_access_iterator<int*>(begin_); }
  constexpr auto end() const { return sized_sentinel<random_access_iterator<int*>>(random_access_iterator<int*>(end_)); }
};
static_assert(std::ranges::view<SizedRandomAccessView>);
static_assert(std::ranges::random_access_range<SizedRandomAccessView>);
static_assert(std::ranges::sized_range<SizedRandomAccessView>);

struct CommonView : std::ranges::view_base {
  int* begin_;
  int* end_;
  constexpr explicit CommonView(int* b, int* e) : begin_(b), end_(e) { }
  constexpr int *begin() const { return begin_; }
  constexpr int *end() const { return end_; }
};
static_assert(std::ranges::view<CommonView>);
static_assert(std::ranges::common_range<CommonView>);

struct NonCommonView : std::ranges::view_base {
  int* begin_;
  int* end_;
  constexpr explicit NonCommonView(int* b, int* e) : begin_(b), end_(e) { }
  constexpr int *begin() const { return begin_; }
  constexpr auto end() const { return sentinel_wrapper<int*>(end_); }
};
static_assert( std::ranges::view<NonCommonView>);
static_assert(!std::ranges::common_range<NonCommonView>);

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_COMMON_VIEW_TYPES_H
