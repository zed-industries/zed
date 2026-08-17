//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_REVERSE_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_REVERSE_TYPES_H

#include "test_macros.h"
#include "test_iterators.h"

struct BidirRange : std::ranges::view_base {
  int *begin_;
  int* end_;

  constexpr BidirRange(int* b, int* e) : begin_(b), end_(e) { }

  constexpr bidirectional_iterator<int*> begin() { return bidirectional_iterator<int*>{begin_}; }
  constexpr bidirectional_iterator<const int*> begin() const { return bidirectional_iterator<const int*>{begin_}; }
  constexpr bidirectional_iterator<int*> end() { return bidirectional_iterator<int*>{end_}; }
  constexpr bidirectional_iterator<const int*> end() const { return bidirectional_iterator<const int*>{end_}; }
};
static_assert( std::ranges::bidirectional_range<BidirRange>);
static_assert( std::ranges::common_range<BidirRange>);
static_assert( std::ranges::view<BidirRange>);
static_assert( std::copyable<BidirRange>);

enum CopyCategory { MoveOnly, Copyable };
template<CopyCategory CC>
struct BidirSentRange : std::ranges::view_base {
  using sent_t = sentinel_wrapper<bidirectional_iterator<int*>>;
  using sent_const_t = sentinel_wrapper<bidirectional_iterator<const int*>>;

  int* begin_;
  int* end_;

  constexpr BidirSentRange(int* b, int* e) : begin_(b), end_(e) { }
  constexpr BidirSentRange(const BidirSentRange &) requires (CC == Copyable) = default;
  constexpr BidirSentRange(BidirSentRange &&) requires (CC == MoveOnly) = default;
  constexpr BidirSentRange& operator=(const BidirSentRange &) requires (CC == Copyable) = default;
  constexpr BidirSentRange& operator=(BidirSentRange &&) requires (CC == MoveOnly) = default;

  constexpr bidirectional_iterator<int*> begin() { return bidirectional_iterator<int*>{begin_}; }
  constexpr bidirectional_iterator<const int*> begin() const { return bidirectional_iterator<const int*>{begin_}; }
  constexpr sent_t end() { return sent_t{bidirectional_iterator<int*>{end_}}; }
  constexpr sent_const_t end() const { return sent_const_t{bidirectional_iterator<const int*>{end_}}; }
};
static_assert( std::ranges::bidirectional_range<BidirSentRange<MoveOnly>>);
static_assert(!std::ranges::common_range<BidirSentRange<MoveOnly>>);
static_assert( std::ranges::view<BidirSentRange<MoveOnly>>);
static_assert(!std::copyable<BidirSentRange<MoveOnly>>);
static_assert( std::ranges::bidirectional_range<BidirSentRange<Copyable>>);
static_assert(!std::ranges::common_range<BidirSentRange<Copyable>>);
static_assert( std::ranges::view<BidirSentRange<Copyable>>);
static_assert( std::copyable<BidirSentRange<Copyable>>);

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_REVERSE_TYPES_H
