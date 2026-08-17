//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TRANSFORM_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TRANSFORM_TYPES_H

#include <cstddef>

#include "test_macros.h"
#include "test_iterators.h"
#include "test_range.h"

int globalBuff[8] = {0,1,2,3,4,5,6,7};

struct MoveOnlyView : std::ranges::view_base {
  int start_;
  int *ptr_;
  constexpr explicit MoveOnlyView(int* ptr = globalBuff, int start = 0) : start_(start), ptr_(ptr) {}
  constexpr MoveOnlyView(MoveOnlyView&&) = default;
  constexpr MoveOnlyView& operator=(MoveOnlyView&&) = default;
  constexpr int *begin() const { return ptr_ + start_; }
  constexpr int *end() const { return ptr_ + 8; }
};
static_assert( std::ranges::view<MoveOnlyView>);
static_assert( std::ranges::contiguous_range<MoveOnlyView>);
static_assert(!std::copyable<MoveOnlyView>);

struct CopyableView : std::ranges::view_base {
  int start_;
  constexpr explicit CopyableView(int start = 0) : start_(start) {}
  constexpr CopyableView(CopyableView const&) = default;
  constexpr CopyableView& operator=(CopyableView const&) = default;
  constexpr int *begin() const { return globalBuff + start_; }
  constexpr int *end() const { return globalBuff + 8; }
};
static_assert(std::ranges::view<CopyableView>);
static_assert(std::ranges::contiguous_range<CopyableView>);
static_assert(std::copyable<CopyableView>);

using ForwardIter = forward_iterator<int*>;
struct ForwardView : std::ranges::view_base {
  int *ptr_;
  constexpr explicit ForwardView(int* ptr = globalBuff) : ptr_(ptr) {}
  constexpr ForwardView(ForwardView&&) = default;
  constexpr ForwardView& operator=(ForwardView&&) = default;
  constexpr auto begin() const { return forward_iterator<int*>(ptr_); }
  constexpr auto end() const { return forward_iterator<int*>(ptr_ + 8); }
};
static_assert(std::ranges::view<ForwardView>);
static_assert(std::ranges::forward_range<ForwardView>);

using ForwardRange = test_common_range<forward_iterator>;
static_assert(!std::ranges::view<ForwardRange>);
static_assert( std::ranges::forward_range<ForwardRange>);

using RandomAccessIter = random_access_iterator<int*>;
struct RandomAccessView : std::ranges::view_base {
  RandomAccessIter begin() const noexcept;
  RandomAccessIter end() const noexcept;
};
static_assert( std::ranges::view<RandomAccessView>);
static_assert( std::ranges::random_access_range<RandomAccessView>);

using BidirectionalIter = bidirectional_iterator<int*>;
struct BidirectionalView : std::ranges::view_base {
  BidirectionalIter begin() const;
  BidirectionalIter end() const;
};
static_assert( std::ranges::view<BidirectionalView>);
static_assert( std::ranges::bidirectional_range<BidirectionalView>);

struct BorrowableRange {
  int *begin() const;
  int *end() const;
};
template<>
inline constexpr bool std::ranges::enable_borrowed_range<BorrowableRange> = true;
static_assert(!std::ranges::view<BorrowableRange>);
static_assert( std::ranges::contiguous_range<BorrowableRange>);
static_assert( std::ranges::borrowed_range<BorrowableRange>);

struct InputView : std::ranges::view_base {
  int *ptr_;
  constexpr explicit InputView(int* ptr = globalBuff) : ptr_(ptr) {}
  constexpr auto begin() const { return cpp20_input_iterator<int*>(ptr_); }
  constexpr auto end() const { return sentinel_wrapper<cpp20_input_iterator<int*>>(cpp20_input_iterator<int*>(ptr_ + 8)); }
};
static_assert( std::ranges::view<InputView>);
static_assert(!std::ranges::sized_range<InputView>);

struct SizedSentinelView : std::ranges::view_base {
  int count_;
  constexpr explicit SizedSentinelView(int count = 8) : count_(count) {}
  constexpr auto begin() const { return RandomAccessIter(globalBuff); }
  constexpr int *end() const { return globalBuff + count_; }
};
// TODO: remove these bogus operators
constexpr auto operator- (const RandomAccessIter &lhs, int* rhs) { return base(lhs) - rhs; }
constexpr auto operator- (int* lhs, const RandomAccessIter &rhs) { return lhs - base(rhs); }
constexpr bool operator==(const RandomAccessIter &lhs, int* rhs) { return base(lhs) == rhs; }
constexpr bool operator==(int* lhs, const RandomAccessIter &rhs) { return base(rhs) == lhs; }

struct SizedSentinelNotConstView : std::ranges::view_base {
  ForwardIter begin() const;
  int *end() const;
  std::size_t size();
};
// TODO: remove these bogus operators
bool operator==(const ForwardIter &lhs, int* rhs);
bool operator==(int* lhs, const ForwardIter &rhs);

struct Range {
  int *begin() const;
  int *end() const;
};

struct TimesTwo {
  constexpr int operator()(int x) const { return x * 2; }
};

struct PlusOneMutable {
  constexpr int operator()(int x) { return x + 1; }
};

struct PlusOne {
  constexpr int operator()(int x) const { return x + 1; }
};

struct PlusOneMutableOverload {
  constexpr int operator()(int x) const { return x + 1; }
  constexpr int& operator()(int& x) { return ++x; }
};

struct Increment {
  constexpr int& operator()(int& x) { return ++x; }
};

struct IncrementRvalueRef {
  constexpr int&& operator()(int& x) { return std::move(++x); }
};

struct PlusOneNoexcept {
  constexpr int operator()(int x) noexcept { return x + 1; }
};

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TRANSFORM_TYPES_H
