//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_DROP_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_DROP_TYPES_H

#include "test_macros.h"
#include "test_iterators.h"

int globalBuff[8];

template <class T>
struct drop_sentinel {
  T* ptr_;
  int* num_of_sentinel_cmp_calls;

public:
  friend constexpr bool operator==(drop_sentinel const s, T* const ptr) noexcept {
    ++(*s.num_of_sentinel_cmp_calls);
    return {s.ptr_ == ptr};
  }
  friend constexpr bool operator==(T* const ptr, drop_sentinel const s) noexcept {
    ++(*s.num_of_sentinel_cmp_calls);
    return {s.ptr_ == ptr};
  }
  friend constexpr bool operator!=(drop_sentinel const s, T* const ptr) noexcept { return !(s == ptr); }
  friend constexpr bool operator!=(T* const ptr, drop_sentinel const s) noexcept { return !(s == ptr); }
};

template <bool IsSimple>
struct MaybeSimpleNonCommonView : std::ranges::view_base {
  int start_;
  int* num_of_sentinel_cmp_calls;
  constexpr std::size_t size() const { return 8; }
  constexpr int* begin() { return globalBuff + start_; }
  constexpr std::conditional_t<IsSimple, int*, const int*> begin() const { return globalBuff + start_; }
  constexpr drop_sentinel<int> end() { return drop_sentinel<int>{globalBuff + size(), num_of_sentinel_cmp_calls}; }
  constexpr auto end() const {
    return std::conditional_t<IsSimple, drop_sentinel<int>, drop_sentinel<const int>>{
        globalBuff + size(), num_of_sentinel_cmp_calls};
  }
};

struct MoveOnlyView : std::ranges::view_base {
  int start_;
  constexpr explicit MoveOnlyView(int start = 0) : start_(start) {}
  constexpr MoveOnlyView(MoveOnlyView&&) = default;
  constexpr MoveOnlyView& operator=(MoveOnlyView&&) = default;
  constexpr int *begin() const { return globalBuff + start_; }
  constexpr int *end() const { return globalBuff + 8; }
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

using ForwardIter = forward_iterator<int*>;
struct ForwardView : std::ranges::view_base {
  constexpr explicit ForwardView() = default;
  constexpr ForwardView(ForwardView&&) = default;
  constexpr ForwardView& operator=(ForwardView&&) = default;
  constexpr forward_iterator<int*> begin() const { return forward_iterator<int*>(globalBuff); }
  constexpr forward_iterator<int*> end() const { return forward_iterator<int*>(globalBuff + 8); }
};

struct ForwardRange {
  ForwardIter begin() const;
  ForwardIter end() const;
};

struct ThrowingDefaultCtorForwardView : std::ranges::view_base {
  ThrowingDefaultCtorForwardView() noexcept(false);
  ForwardIter begin() const;
  ForwardIter end() const;
};

struct NoDefaultCtorForwardView : std::ranges::view_base {
  NoDefaultCtorForwardView() = delete;
  ForwardIter begin() const;
  ForwardIter end() const;
};

struct BorrowableRange {
  int *begin() const;
  int *end() const;
};
template<>
inline constexpr bool std::ranges::enable_borrowed_range<BorrowableRange> = true;

struct BorrowableView : std::ranges::view_base {
  int *begin() const;
  int *end() const;
};
template<>
inline constexpr bool std::ranges::enable_borrowed_range<BorrowableView> = true;

struct InputView : std::ranges::view_base {
  constexpr cpp20_input_iterator<int*> begin() const { return cpp20_input_iterator<int*>(globalBuff); }
  constexpr int* end() const { return globalBuff + 8; }
};
// TODO: remove these bogus operators
constexpr bool operator==(const cpp20_input_iterator<int*> &lhs, int* rhs) { return base(lhs) == rhs; }
constexpr bool operator==(int* lhs, const cpp20_input_iterator<int*> &rhs) { return base(rhs) == lhs; }

struct Range {
  int *begin() const;
  int *end() const;
};

using CountedIter = operation_counting_iterator<forward_iterator<int*>>;
struct CountedView : std::ranges::view_base {
  explicit constexpr CountedView(IteratorOpCounts* opcounts) noexcept : opcounts_(opcounts) {}
  constexpr CountedIter begin() const { return CountedIter(ForwardIter(globalBuff), opcounts_); }
  constexpr CountedIter end() const { return CountedIter(ForwardIter(globalBuff + 8), opcounts_); }

private:
  IteratorOpCounts* opcounts_;
};

struct View : std::ranges::view_base {
  constexpr explicit View(int* b, int* e) : begin_(b), end_(e) { }

  constexpr int* begin() const { return begin_; }
  constexpr int* end() const { return end_; }

private:
  int* begin_;
  int* end_;
};

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_DROP_TYPES_H
