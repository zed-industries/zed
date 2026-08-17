//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_ZIP_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_ZIP_TYPES_H

#include <functional>
#include <ranges>

#include "test_macros.h"
#include "test_iterators.h"
#include "test_range.h"

#if TEST_STD_VER <= 20
#  error "range.zip/types.h" can only be included in builds supporting C++20
#endif // TEST_STD_VER <= 20

template <class T>
struct BufferView : std::ranges::view_base {
  T* buffer_;
  std::size_t size_;

  template <std::size_t N>
  constexpr BufferView(T (&b)[N]) : buffer_(b), size_(N) {}
};

using IntBufferView = BufferView<int>;

template <bool Simple>
struct Common :  IntBufferView {
  using IntBufferView::IntBufferView;

  constexpr int* begin()
    requires(!Simple)
  {
    return buffer_;
  }
  constexpr const int* begin() const { return buffer_; }
  constexpr int* end()
    requires(!Simple)
  {
    return buffer_ + size_;
  }
  constexpr const int* end() const { return buffer_ + size_; }
};
using SimpleCommon = Common<true>;
using NonSimpleCommon = Common<false>;

using SimpleCommonRandomAccessSized = SimpleCommon;
using NonSimpleCommonRandomAccessSized = NonSimpleCommon;

static_assert(std::ranges::common_range<Common<true>>);
static_assert(std::ranges::random_access_range<SimpleCommon>);
static_assert(std::ranges::sized_range<SimpleCommon>);
static_assert(simple_view<SimpleCommon>);
static_assert(!simple_view<NonSimpleCommon>);

template <bool Simple>
struct CommonNonRandom : IntBufferView {
  using IntBufferView::IntBufferView;
  using const_iterator = forward_iterator<const int*>;
  using iterator = forward_iterator<int*>;
  constexpr iterator begin()
    requires(!Simple) {
    return iterator(buffer_);
  }
  constexpr const_iterator begin() const { return const_iterator(buffer_); }
  constexpr iterator end()
    requires(!Simple) {
    return iterator(buffer_ + size_);
  }
  constexpr const_iterator end() const { return const_iterator(buffer_ + size_); }
};

using SimpleCommonNonRandom = CommonNonRandom<true>;
using NonSimpleCommonNonRandom = CommonNonRandom<false>;

static_assert(std::ranges::common_range<SimpleCommonNonRandom>);
static_assert(!std::ranges::random_access_range<SimpleCommonNonRandom>);
static_assert(!std::ranges::sized_range<SimpleCommonNonRandom>);
static_assert(simple_view<SimpleCommonNonRandom>);
static_assert(!simple_view<NonSimpleCommonNonRandom>);

template <bool Simple>
struct NonCommon : IntBufferView {
  using IntBufferView::IntBufferView;
  constexpr int* begin()
    requires(!Simple) {
    return buffer_;
  }
  constexpr const int* begin() const { return buffer_; }
  constexpr sentinel_wrapper<int*> end()
    requires(!Simple) {
    return sentinel_wrapper<int*>(buffer_ + size_);
  }
  constexpr sentinel_wrapper<const int*> end() const { return sentinel_wrapper<const int*>(buffer_ + size_); }
};

using SimpleNonCommon = NonCommon<true>;
using NonSimpleNonCommon = NonCommon<false>;

static_assert(!std::ranges::common_range<SimpleNonCommon>);
static_assert(std::ranges::random_access_range<SimpleNonCommon>);
static_assert(!std::ranges::sized_range<SimpleNonCommon>);
static_assert(simple_view<SimpleNonCommon>);
static_assert(!simple_view<NonSimpleNonCommon>);

template <bool Simple>
struct NonCommonSized : IntBufferView {
  using IntBufferView::IntBufferView;
  constexpr int* begin()
    requires(!Simple) {
    return buffer_;
  }
  constexpr const int* begin() const { return buffer_; }
  constexpr sentinel_wrapper<int*> end()
    requires(!Simple) {
    return sentinel_wrapper<int*>(buffer_ + size_);
  }
  constexpr sentinel_wrapper<const int*> end() const { return sentinel_wrapper<const int*>(buffer_ + size_); }
  constexpr std::size_t size() const { return size_; }
};

using SimpleNonCommonSized = NonCommonSized<true>;
using SimpleNonCommonRandomAccessSized    = SimpleNonCommonSized;
using NonSimpleNonCommonSized = NonCommonSized<false>;
using NonSimpleNonCommonRandomAccessSized = NonSimpleNonCommonSized;

static_assert(!std::ranges::common_range<SimpleNonCommonSized>);
static_assert(std::ranges::random_access_range<SimpleNonCommonSized>);
static_assert(std::ranges::sized_range<SimpleNonCommonSized>);
static_assert(simple_view<SimpleNonCommonSized>);
static_assert(!simple_view<NonSimpleNonCommonSized>);

template <bool Simple>
struct NonCommonNonRandom : IntBufferView {
  using IntBufferView::IntBufferView;

  using const_iterator = forward_iterator<const int*>;
  using iterator = forward_iterator<int*>;

  constexpr iterator begin()
    requires(!Simple) {
    return iterator(buffer_);
  }
  constexpr const_iterator begin() const { return const_iterator(buffer_); }
  constexpr sentinel_wrapper<iterator> end()
    requires(!Simple) {
    return sentinel_wrapper<iterator>(iterator(buffer_ + size_));
  }
  constexpr sentinel_wrapper<const_iterator> end() const {
    return sentinel_wrapper<const_iterator>(const_iterator(buffer_ + size_));
  }
};

using SimpleNonCommonNonRandom = NonCommonNonRandom<true>;
using NonSimpleNonCommonNonRandom = NonCommonNonRandom<false>;

static_assert(!std::ranges::common_range<SimpleNonCommonNonRandom>);
static_assert(!std::ranges::random_access_range<SimpleNonCommonNonRandom>);
static_assert(!std::ranges::sized_range<SimpleNonCommonNonRandom>);
static_assert(simple_view<SimpleNonCommonNonRandom>);
static_assert(!simple_view<NonSimpleNonCommonNonRandom>);

template <class Iter, class Sent = Iter, class NonConstIter = Iter, class NonConstSent = Sent>
struct BasicView : IntBufferView {
  using IntBufferView::IntBufferView;

  constexpr NonConstIter begin()
    requires(!std::is_same_v<Iter, NonConstIter>) {
    return NonConstIter(buffer_);
  }
  constexpr Iter begin() const { return Iter(buffer_); }

  constexpr NonConstSent end()
    requires(!std::is_same_v<Sent, NonConstSent>) {
    if constexpr (std::is_same_v<NonConstIter, NonConstSent>) {
      return NonConstIter(buffer_ + size_);
    } else {
      return NonConstSent(NonConstIter(buffer_ + size_));
    }
  }

  constexpr Sent end() const {
    if constexpr (std::is_same_v<Iter, Sent>) {
      return Iter(buffer_ + size_);
    } else {
      return Sent(Iter(buffer_ + size_));
    }
  }
};

template <class Base = int*>
struct forward_sized_iterator {
  Base it_ = nullptr;

  using iterator_category = std::forward_iterator_tag;
  using value_type = int;
  using difference_type = std::intptr_t;
  using pointer = Base;
  using reference = decltype(*Base{});

  forward_sized_iterator() = default;
  constexpr forward_sized_iterator(Base it) : it_(it) {}

  constexpr reference operator*() const { return *it_; }

  constexpr forward_sized_iterator& operator++() {
    ++it_;
    return *this;
  }
  constexpr forward_sized_iterator operator++(int) { return forward_sized_iterator(it_++); }

  friend constexpr bool operator==(const forward_sized_iterator&, const forward_sized_iterator&) = default;

  friend constexpr difference_type operator-(const forward_sized_iterator& x, const forward_sized_iterator& y) {
    return x.it_ - y.it_;
  }
};
static_assert(std::forward_iterator<forward_sized_iterator<>>);
static_assert(std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>);

using ForwardSizedView = BasicView<forward_sized_iterator<>>;
static_assert(std::ranges::forward_range<ForwardSizedView>);
static_assert(std::ranges::sized_range<ForwardSizedView>);
static_assert(std::ranges::common_range<ForwardSizedView>);
static_assert(!std::ranges::random_access_range<ForwardSizedView>);
static_assert(simple_view<ForwardSizedView>);

using NonSimpleForwardSizedView = BasicView<forward_sized_iterator<const int*>, forward_sized_iterator<const int*>,
                                            forward_sized_iterator<int*>, forward_sized_iterator<int*>>;
static_assert(std::ranges::forward_range<NonSimpleForwardSizedView>);
static_assert(std::ranges::sized_range<NonSimpleForwardSizedView>);
static_assert(std::ranges::common_range<NonSimpleForwardSizedView>);
static_assert(!std::ranges::random_access_range<NonSimpleForwardSizedView>);
static_assert(!simple_view<NonSimpleForwardSizedView>);

using ForwardSizedNonCommon = BasicView<forward_sized_iterator<>, sized_sentinel<forward_sized_iterator<>>>;
static_assert(std::ranges::forward_range<ForwardSizedNonCommon>);
static_assert(std::ranges::sized_range<ForwardSizedNonCommon>);
static_assert(!std::ranges::common_range<ForwardSizedNonCommon>);
static_assert(!std::ranges::random_access_range<ForwardSizedNonCommon>);
static_assert(simple_view<ForwardSizedNonCommon>);

using NonSimpleForwardSizedNonCommon =
    BasicView<forward_sized_iterator<const int*>, sized_sentinel<forward_sized_iterator<const int*>>,
              forward_sized_iterator<int*>, sized_sentinel<forward_sized_iterator<int*>>>;
static_assert(std::ranges::forward_range<NonSimpleForwardSizedNonCommon>);
static_assert(std::ranges::sized_range<NonSimpleForwardSizedNonCommon>);
static_assert(!std::ranges::common_range<NonSimpleForwardSizedNonCommon>);
static_assert(!std::ranges::random_access_range<NonSimpleForwardSizedNonCommon>);
static_assert(!simple_view<NonSimpleForwardSizedNonCommon>);

struct SizedRandomAccessView : IntBufferView {
  using IntBufferView::IntBufferView;
  using iterator = random_access_iterator<int*>;

  constexpr auto begin() const { return iterator(buffer_); }
  constexpr auto end() const { return sized_sentinel<iterator>(iterator(buffer_ + size_)); }

  constexpr decltype(auto) operator[](std::size_t n) const { return *(begin() + n); }
};
static_assert(std::ranges::view<SizedRandomAccessView>);
static_assert(std::ranges::random_access_range<SizedRandomAccessView>);
static_assert(std::ranges::sized_range<SizedRandomAccessView>);

using NonSizedRandomAccessView =
    BasicView<random_access_iterator<int*>, sentinel_wrapper<random_access_iterator<int*>>>;
static_assert(!std::ranges::contiguous_range<NonSizedRandomAccessView>);
static_assert(std::ranges::random_access_range<SizedRandomAccessView>);
static_assert(!std::ranges::common_range<NonSizedRandomAccessView>);
static_assert(!std::ranges::sized_range<NonSizedRandomAccessView>);
static_assert(simple_view<NonSizedRandomAccessView>);

using NonSimpleNonSizedRandomAccessView =
    BasicView<random_access_iterator<const int*>, sentinel_wrapper<random_access_iterator<const int*>>,
              random_access_iterator<int*>, sentinel_wrapper<random_access_iterator<int*>> >;
static_assert(!std::ranges::contiguous_range<NonSimpleNonSizedRandomAccessView>);
static_assert(std::ranges::random_access_range<NonSimpleNonSizedRandomAccessView>);
static_assert(!std::ranges::common_range<NonSimpleNonSizedRandomAccessView>);
static_assert(!std::ranges::sized_range<NonSimpleNonSizedRandomAccessView>);
static_assert(!simple_view<NonSimpleNonSizedRandomAccessView>);

using ContiguousCommonView = BasicView<int*>;
static_assert(std::ranges::contiguous_range<ContiguousCommonView>);
static_assert(std::ranges::common_range<ContiguousCommonView>);
static_assert(std::ranges::sized_range<ContiguousCommonView>);

using ContiguousNonCommonView = BasicView<int*, sentinel_wrapper<int*>>;
static_assert(std::ranges::contiguous_range<ContiguousNonCommonView>);
static_assert(!std::ranges::common_range<ContiguousNonCommonView>);
static_assert(!std::ranges::sized_range<ContiguousNonCommonView>);

using ContiguousNonCommonSized = BasicView<int*, sized_sentinel<int*>>;

static_assert(std::ranges::contiguous_range<ContiguousNonCommonSized>);
static_assert(!std::ranges::common_range<ContiguousNonCommonSized>);
static_assert(std::ranges::sized_range<ContiguousNonCommonSized>);

using InputCommonView = BasicView<common_input_iterator<int*>>;
static_assert(std::ranges::input_range<InputCommonView>);
static_assert(!std::ranges::forward_range<InputCommonView>);
static_assert(std::ranges::common_range<InputCommonView>);
static_assert(simple_view<InputCommonView>);

using NonSimpleInputCommonView = BasicView<common_input_iterator<const int*>, common_input_iterator<const int*>,
                                           common_input_iterator<int*>, common_input_iterator<int*>>;
static_assert(std::ranges::input_range<NonSimpleInputCommonView>);
static_assert(!std::ranges::forward_range<NonSimpleInputCommonView>);
static_assert(std::ranges::common_range<NonSimpleInputCommonView>);
static_assert(!simple_view<NonSimpleInputCommonView>);

using InputNonCommonView = BasicView<common_input_iterator<int*>, sentinel_wrapper<common_input_iterator<int*>>>;
static_assert(std::ranges::input_range<InputNonCommonView>);
static_assert(!std::ranges::forward_range<InputNonCommonView>);
static_assert(!std::ranges::common_range<InputNonCommonView>);
static_assert(simple_view<InputNonCommonView>);

using NonSimpleInputNonCommonView =
    BasicView<common_input_iterator<const int*>, sentinel_wrapper<common_input_iterator<const int*>>,
              common_input_iterator<int*>, sentinel_wrapper<common_input_iterator<int*>>>;
static_assert(std::ranges::input_range<InputNonCommonView>);
static_assert(!std::ranges::forward_range<InputNonCommonView>);
static_assert(!std::ranges::common_range<InputNonCommonView>);
static_assert(!simple_view<NonSimpleInputNonCommonView>);

using BidiCommonView = BasicView<bidirectional_iterator<int*>>;
static_assert(!std::ranges::sized_range<BidiCommonView>);
static_assert(std::ranges::bidirectional_range<BidiCommonView>);
static_assert(!std::ranges::random_access_range<BidiCommonView>);
static_assert(std::ranges::common_range<BidiCommonView>);
static_assert(simple_view<BidiCommonView>);

using NonSimpleBidiCommonView = BasicView<bidirectional_iterator<const int*>, bidirectional_iterator<const int*>,
                                          bidirectional_iterator<int*>, bidirectional_iterator<int*>>;
static_assert(!std::ranges::sized_range<NonSimpleBidiCommonView>);
static_assert(std::ranges::bidirectional_range<NonSimpleBidiCommonView>);
static_assert(!std::ranges::random_access_range<NonSimpleBidiCommonView>);
static_assert(std::ranges::common_range<NonSimpleBidiCommonView>);
static_assert(!simple_view<NonSimpleBidiCommonView>);

struct SizedBidiCommon : BidiCommonView {
  using BidiCommonView::BidiCommonView;
  std::size_t size() const { return base(end()) - base(begin()); }
};
static_assert(std::ranges::sized_range<SizedBidiCommon>);
static_assert(std::ranges::bidirectional_range<SizedBidiCommon>);
static_assert(!std::ranges::random_access_range<SizedBidiCommon>);
static_assert(std::ranges::common_range<SizedBidiCommon>);
static_assert(simple_view<SizedBidiCommon>);

struct NonSimpleSizedBidiCommon : NonSimpleBidiCommonView {
  using NonSimpleBidiCommonView::NonSimpleBidiCommonView;
  std::size_t size() const { return base(end()) - base(begin()); }
};
static_assert(std::ranges::sized_range<NonSimpleSizedBidiCommon>);
static_assert(std::ranges::bidirectional_range<NonSimpleSizedBidiCommon>);
static_assert(!std::ranges::random_access_range<NonSimpleSizedBidiCommon>);
static_assert(std::ranges::common_range<NonSimpleSizedBidiCommon>);
static_assert(!simple_view<NonSimpleSizedBidiCommon>);

using BidiNonCommonView = BasicView<bidirectional_iterator<int*>, sentinel_wrapper<bidirectional_iterator<int*>>>;
static_assert(!std::ranges::sized_range<BidiNonCommonView>);
static_assert(std::ranges::bidirectional_range<BidiNonCommonView>);
static_assert(!std::ranges::random_access_range<BidiNonCommonView>);
static_assert(!std::ranges::common_range<BidiNonCommonView>);
static_assert(simple_view<BidiNonCommonView>);

using NonSimpleBidiNonCommonView =
    BasicView<bidirectional_iterator<const int*>, sentinel_wrapper<bidirectional_iterator<const int*>>,
              bidirectional_iterator<int*>, sentinel_wrapper<bidirectional_iterator<int*>>>;
static_assert(!std::ranges::sized_range<NonSimpleBidiNonCommonView>);
static_assert(std::ranges::bidirectional_range<NonSimpleBidiNonCommonView>);
static_assert(!std::ranges::random_access_range<NonSimpleBidiNonCommonView>);
static_assert(!std::ranges::common_range<NonSimpleBidiNonCommonView>);
static_assert(!simple_view<NonSimpleBidiNonCommonView>);

using SizedBidiNonCommonView = BasicView<bidirectional_iterator<int*>, sized_sentinel<bidirectional_iterator<int*>>>;
static_assert(std::ranges::sized_range<SizedBidiNonCommonView>);
static_assert(std::ranges::bidirectional_range<SizedBidiNonCommonView>);
static_assert(!std::ranges::random_access_range<SizedBidiNonCommonView>);
static_assert(!std::ranges::common_range<SizedBidiNonCommonView>);
static_assert(simple_view<SizedBidiNonCommonView>);

using NonSimpleSizedBidiNonCommonView =
    BasicView<bidirectional_iterator<const int*>, sized_sentinel<bidirectional_iterator<const int*>>,
              bidirectional_iterator<int*>, sized_sentinel<bidirectional_iterator<int*>>>;
static_assert(std::ranges::sized_range<NonSimpleSizedBidiNonCommonView>);
static_assert(std::ranges::bidirectional_range<NonSimpleSizedBidiNonCommonView>);
static_assert(!std::ranges::random_access_range<NonSimpleSizedBidiNonCommonView>);
static_assert(!std::ranges::common_range<NonSimpleSizedBidiNonCommonView>);
static_assert(!simple_view<NonSimpleSizedBidiNonCommonView>);

namespace adltest{
struct iter_move_swap_iterator {

  std::reference_wrapper<int> iter_move_called_times;
  std::reference_wrapper<int> iter_swap_called_times;
  int i = 0;

  using iterator_category = std::input_iterator_tag;
  using value_type = int;
  using difference_type = std::intptr_t;

  constexpr int operator*() const { return i; }

  constexpr iter_move_swap_iterator& operator++() {
    ++i;
    return *this;
  }
  constexpr void operator++(int) { ++i; }

  friend constexpr bool operator==(const iter_move_swap_iterator& x, std::default_sentinel_t) { return x.i == 5; }

  friend constexpr int iter_move(iter_move_swap_iterator const& it) {
    ++it.iter_move_called_times;
    return it.i;
  }
  friend constexpr void iter_swap(iter_move_swap_iterator const& x, iter_move_swap_iterator const& y) {
    ++x.iter_swap_called_times;
    ++y.iter_swap_called_times;
  }
};

struct IterMoveSwapRange {
  int iter_move_called_times = 0;
  int iter_swap_called_times = 0;
  constexpr auto begin() { return iter_move_swap_iterator{iter_move_called_times, iter_swap_called_times}; }
  constexpr auto end() const { return std::default_sentinel; }
};
} // namespace adltest

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_ZIP_TYPES_H
