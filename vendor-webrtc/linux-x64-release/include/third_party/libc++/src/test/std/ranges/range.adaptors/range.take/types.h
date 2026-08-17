#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TAKE_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TAKE_TYPES_H

#include <ranges>

#include "test_macros.h"
#include "test_iterators.h"
#include "test_range.h"

struct MoveOnlyView : std::ranges::view_base {
  int *ptr_;

  constexpr explicit MoveOnlyView(int* ptr) : ptr_(ptr) {}
  MoveOnlyView(MoveOnlyView&&) = default;
  MoveOnlyView& operator=(MoveOnlyView&&) = default;

  constexpr int* begin() const {return ptr_;}
  constexpr sentinel_wrapper<int*> end() const {return sentinel_wrapper<int*>{ptr_ + 8};}
};
static_assert( std::ranges::view<MoveOnlyView>);
static_assert( std::ranges::contiguous_range<MoveOnlyView>);
static_assert(!std::copyable<MoveOnlyView>);

struct CopyableView : std::ranges::view_base {
  int *ptr_;
  constexpr explicit CopyableView(int* ptr) : ptr_(ptr) {}

  constexpr int* begin() const {return ptr_;}
  constexpr sentinel_wrapper<int*> end() const {return sentinel_wrapper<int*>{ptr_ + 8};}
};
static_assert(std::ranges::view<CopyableView>);
static_assert(std::ranges::contiguous_range<CopyableView>);
static_assert(std::copyable<CopyableView>);

using ForwardIter = forward_iterator<int*>;
struct SizedForwardView : std::ranges::view_base {
  int *ptr_;
  constexpr explicit SizedForwardView(int* ptr) : ptr_(ptr) {}
  constexpr auto begin() const { return ForwardIter(ptr_); }
  constexpr auto end() const { return sized_sentinel<ForwardIter>(ForwardIter(ptr_ + 8)); }
};
static_assert(std::ranges::view<SizedForwardView>);
static_assert(std::ranges::forward_range<SizedForwardView>);
static_assert(std::ranges::sized_range<SizedForwardView>);

using RandomAccessIter = random_access_iterator<int*>;
struct SizedRandomAccessView : std::ranges::view_base {
  int *ptr_;
  constexpr explicit SizedRandomAccessView(int* ptr) : ptr_(ptr) {}
  constexpr auto begin() const { return RandomAccessIter(ptr_); }
  constexpr auto end() const { return sized_sentinel<RandomAccessIter>(RandomAccessIter(ptr_ + 8)); }
};
static_assert(std::ranges::view<SizedRandomAccessView>);
static_assert(std::ranges::random_access_range<SizedRandomAccessView>);
static_assert(std::ranges::sized_range<SizedRandomAccessView>);

struct View : std::ranges::view_base {
  constexpr explicit View(int* b, int* e) : begin_(b), end_(e) { }

  constexpr int* begin() const { return begin_; }
  constexpr int* end() const { return end_; }

private:
  int* begin_;
  int* end_;
};

template <template <class...> typename Iter, bool Simple, bool Sized>
struct CommonInputView : std::ranges::view_base {
  constexpr explicit CommonInputView(int* b, int* e) : begin_(b), end_(e) {}

  constexpr Iter<int*> begin() const { return Iter<int*>(begin_); }
  constexpr Iter<int*> end() const { return Iter<int*>(end_); }

  constexpr Iter<const int*> begin()
    requires(!Simple)
  {
    return Iter<const int*>(begin_);
  }
  constexpr Iter<const int*> end()
    requires(!Simple)
  {
    return Iter<const int*>(end_);
  }

  constexpr auto size() const
    requires Sized
  {
    return end_ - begin_;
  }

private:
  int* begin_;
  int* end_;
};

using NonSimpleNonSizedView = CommonInputView<common_input_iterator, /*Simple=*/false, /*Sized=*/false>;
static_assert(std::ranges::view<NonSimpleNonSizedView>);
static_assert(!simple_view<NonSimpleNonSizedView>);
static_assert(!std::ranges::sized_range<NonSimpleNonSizedView>);

using SimpleViewNonSized = CommonInputView<common_input_iterator, /*Simple=*/true, /*Sized=*/false>;
static_assert(std::ranges::view<SimpleViewNonSized>);
static_assert(simple_view<SimpleViewNonSized>);
static_assert(!std::ranges::sized_range<SimpleViewNonSized>);

using NonSimpleSizedView = CommonInputView<common_input_iterator, /*Simple=*/false, /*Sized=*/true>;
static_assert(std::ranges::view<NonSimpleSizedView>);
static_assert(!simple_view<NonSimpleSizedView>);
static_assert(std::ranges::sized_range<NonSimpleSizedView>);

using NonSimpleSizedRandomView = CommonInputView<random_access_iterator, /*Simple=*/false, /*Sized=*/true>;
static_assert(std::ranges::view<NonSimpleSizedRandomView>);
static_assert(!simple_view<NonSimpleSizedRandomView>);
static_assert(std::ranges::sized_range<NonSimpleSizedRandomView>);
static_assert(std::ranges::random_access_range<NonSimpleSizedRandomView>);

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_TAKE_TYPES_H
