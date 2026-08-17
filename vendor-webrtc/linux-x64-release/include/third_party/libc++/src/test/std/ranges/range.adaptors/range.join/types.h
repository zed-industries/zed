//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_JOIN_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_JOIN_TYPES_H

#include <concepts>
#include <cstdint>
#include <string>
#include <tuple>

#include "test_macros.h"
#include "test_iterators.h"
#include "test_range.h"

inline int globalBuffer[4][4] = {
    {1111, 2222, 3333, 4444},
    {555, 666, 777, 888},
    {99, 1010, 1111, 1212},
    {13, 14, 15, 16},
};

template <template<class...> class Iter>
struct ChildViewBase : std::ranges::view_base {
  int* ptr_;

  using iterator = Iter<int*>;
  using const_iterator = Iter<const int*>;
  using sentinel = sentinel_wrapper<iterator>;
  using const_sentinel = sentinel_wrapper<const_iterator>;

  constexpr ChildViewBase(int* ptr = globalBuffer[0]) : ptr_(ptr) {}
  ChildViewBase(const ChildViewBase&) = delete;
  ChildViewBase(ChildViewBase&&) = default;
  ChildViewBase& operator=(const ChildViewBase&) = delete;
  ChildViewBase& operator=(ChildViewBase&&) = default;

  constexpr iterator begin() { return iterator(ptr_); }
  constexpr const_iterator begin() const { return const_iterator(ptr_); }
  constexpr sentinel end() { return sentinel(iterator(ptr_ + 4)); }
  constexpr const_sentinel end() const { return const_sentinel(const_iterator(ptr_ + 4)); }
};
using ChildView = ChildViewBase<cpp20_input_iterator>;

inline ChildView globalChildren[4] = {
    ChildView(globalBuffer[0]),
    ChildView(globalBuffer[1]),
    ChildView(globalBuffer[2]),
    ChildView(globalBuffer[3]),
};

template <class T, template<class...> class Iter = cpp17_input_iterator>
struct ParentView : std::ranges::view_base {
  T* ptr_;
  unsigned size_;

  using iterator = Iter<T*>;
  using const_iterator = Iter<const T*>;
  using sentinel = sentinel_wrapper<iterator>;
  using const_sentinel = sentinel_wrapper<const_iterator>;

  constexpr ParentView(T* ptr, unsigned size = 4) : ptr_(ptr), size_(size) {}
  constexpr ParentView(ChildView* ptr = globalChildren, unsigned size = 4)
    requires std::same_as<ChildView, T>
  : ptr_(ptr), size_(size) {}
  ParentView(const ParentView&) = delete;
  ParentView(ParentView&&) = default;
  ParentView& operator=(const ParentView&) = delete;
  ParentView& operator=(ParentView&&) = default;

  constexpr iterator begin() { return iterator(ptr_); }
  constexpr const_iterator begin() const { return const_iterator(ptr_); }
  constexpr sentinel end() { return sentinel(iterator(ptr_ + size_)); }
  constexpr const_sentinel end() const { return const_sentinel(const_iterator(ptr_ + size_)); }
};

template <class T>
ParentView(T*) -> ParentView<T>;

template<class T>
using ForwardParentView = ParentView<T, forward_iterator>;

struct CopyableChild : std::ranges::view_base {
  int* ptr_;
  unsigned size_;

  using iterator = cpp17_input_iterator<int*>;
  using const_iterator = cpp17_input_iterator<const int*>;
  using sentinel = sentinel_wrapper<iterator>;
  using const_sentinel = sentinel_wrapper<const_iterator>;

  constexpr CopyableChild(int* ptr = globalBuffer[0], unsigned size = 4) : ptr_(ptr), size_(size) {}

  constexpr iterator begin() { return iterator(ptr_); }
  constexpr const_iterator begin() const { return const_iterator(ptr_); }
  constexpr sentinel end() { return sentinel(iterator(ptr_ + size_)); }
  constexpr const_sentinel end() const { return const_sentinel(const_iterator(ptr_ + size_)); }
};

template<template<class...> class Iter>
struct CopyableParentTemplate : std::ranges::view_base {
  CopyableChild* ptr_;

  using iterator = Iter<CopyableChild*>;
  using const_iterator = Iter<const CopyableChild*>;
  using sentinel = sentinel_wrapper<iterator>;
  using const_sentinel = sentinel_wrapper<const_iterator>;

  constexpr CopyableParentTemplate(CopyableChild* ptr) : ptr_(ptr) {}

  constexpr iterator begin() { return iterator(ptr_); }
  constexpr const_iterator begin() const { return const_iterator(ptr_); }
  constexpr sentinel end() { return sentinel(iterator(ptr_ + 4)); }
  constexpr const_sentinel end() const { return const_sentinel(const_iterator(ptr_ + 4)); }
};

using CopyableParent = CopyableParentTemplate<cpp17_input_iterator>;
using ForwardCopyableParent = CopyableParentTemplate<forward_iterator>;

struct Box {
  int x;
};

template <class T>
struct InputValueIter {
  typedef std::input_iterator_tag iterator_category;
  typedef T value_type;
  typedef int difference_type;
  typedef T reference;

  T* ptr_ = nullptr;
  constexpr InputValueIter() = default;
  constexpr InputValueIter(T* ptr) : ptr_(ptr) {}

  constexpr T operator*() const { return std::move(*ptr_); }
  constexpr void operator++(int) { ++ptr_; }
  constexpr InputValueIter& operator++() {
    ++ptr_;
    return *this;
  }

  constexpr T* operator->() { return ptr_; }

  constexpr friend bool operator==(const InputValueIter&, const InputValueIter&) = default;
};

template <class T>
struct ValueView : std::ranges::view_base {
  InputValueIter<T> ptr_;

  using sentinel = sentinel_wrapper<InputValueIter<T>>;

  constexpr ValueView(T* ptr) : ptr_(ptr) {}

  constexpr ValueView(ValueView&& other) : ptr_(other.ptr_) { other.ptr_.ptr_ = nullptr; }

  constexpr ValueView& operator=(ValueView&& other) {
    ptr_ = other.ptr_;
    other.ptr_ = InputValueIter<T>(nullptr);
    return *this;
  }

  ValueView(const ValueView&) = delete;
  ValueView& operator=(const ValueView&) = delete;

  constexpr InputValueIter<T> begin() const { return ptr_; }
  constexpr sentinel end() const { return sentinel(InputValueIter<T>(ptr_.ptr_ + 4)); }
};

template <class Iter, class Sent = Iter, class NonConstIter = Iter, class NonConstSent = Sent>
struct BufferView : std::ranges::view_base {

  using T = std::iter_value_t<Iter>;
  T* data_;
  std::size_t size_;

  template <std::size_t N>
  constexpr BufferView(T (&b)[N]) : data_(b), size_(N) {}
  constexpr BufferView(T* p, std::size_t s) : data_(p), size_(s) {}

  constexpr NonConstIter begin()
    requires(!std::is_same_v<Iter, NonConstIter>) {
    return NonConstIter(this->data_);
  }
  constexpr Iter begin() const { return Iter(this->data_); }

  constexpr NonConstSent end()
    requires(!std::is_same_v<Sent, NonConstSent>) {
    if constexpr (std::is_same_v<NonConstIter, NonConstSent>) {
      return NonConstIter(this->data_ + this->size_);
    } else {
      return NonConstSent(NonConstIter(this->data_ + this->size_));
    }
  }

  constexpr Sent end() const {
    if constexpr (std::is_same_v<Iter, Sent>) {
      return Iter(this->data_ + this->size_);
    } else {
      return Sent(Iter(this->data_ + this->size_));
    }
  }
};

using InputCommonInner = BufferView<common_input_iterator<int*>>;
static_assert(std::ranges::input_range<InputCommonInner>);
static_assert(!std::ranges::forward_range<InputCommonInner>);
static_assert(std::ranges::common_range<InputCommonInner>);

using InputNonCommonInner = BufferView<common_input_iterator<int*>, sentinel_wrapper<common_input_iterator<int*>>>;
static_assert(std::ranges::input_range<InputNonCommonInner>);
static_assert(!std::ranges::forward_range<InputNonCommonInner>);
static_assert(!std::ranges::common_range<InputNonCommonInner>);

using ForwardCommonInner = BufferView<forward_iterator<int*>>;
static_assert(std::ranges::forward_range<ForwardCommonInner>);
static_assert(!std::ranges::bidirectional_range<ForwardCommonInner>);
static_assert(std::ranges::common_range<ForwardCommonInner>);

using ForwardNonCommonInner = BufferView<forward_iterator<int*>, sentinel_wrapper<forward_iterator<int*>>>;
static_assert(std::ranges::forward_range<ForwardNonCommonInner>);
static_assert(!std::ranges::bidirectional_range<ForwardNonCommonInner>);
static_assert(!std::ranges::common_range<ForwardNonCommonInner>);

using BidiCommonInner = BufferView<bidirectional_iterator<int*>>;
static_assert(std::ranges::bidirectional_range<BidiCommonInner>);
static_assert(std::ranges::common_range<BidiCommonInner>);

using BidiNonCommonInner = BufferView<bidirectional_iterator<int*>, sentinel_wrapper<bidirectional_iterator<int*>>>;
static_assert(std::ranges::bidirectional_range<BidiNonCommonInner>);
static_assert(!std::ranges::common_range<BidiNonCommonInner>);

template <class Inner = BufferView<int*>>
using SimpleInputCommonOuter = BufferView<common_input_iterator<Inner*>>;
static_assert(!std::ranges::forward_range<SimpleInputCommonOuter<>>);
static_assert(!std::ranges::bidirectional_range<SimpleInputCommonOuter<>>);
static_assert(std::ranges::common_range<SimpleInputCommonOuter<>>);
static_assert(simple_view<SimpleInputCommonOuter<>>);

template <class Inner = BufferView<int*>>
using NonSimpleInputCommonOuter = BufferView<common_input_iterator<const Inner*>, common_input_iterator<const Inner*>,
                                             common_input_iterator< Inner*>, common_input_iterator< Inner*>>;
static_assert(!std::ranges::forward_range<NonSimpleInputCommonOuter<>>);
static_assert(!std::ranges::bidirectional_range<NonSimpleInputCommonOuter<>>);
static_assert(std::ranges::common_range<NonSimpleInputCommonOuter<>>);
static_assert(!simple_view<NonSimpleInputCommonOuter<>>);

template <class Inner = BufferView<int*>>
using SimpleForwardCommonOuter = BufferView<forward_iterator<Inner*>>;
static_assert(std::ranges::forward_range<SimpleForwardCommonOuter<>>);
static_assert(!std::ranges::bidirectional_range<SimpleForwardCommonOuter<>>);
static_assert(std::ranges::common_range<SimpleForwardCommonOuter<>>);
static_assert(simple_view<SimpleForwardCommonOuter<>>);

template <class Inner = BufferView<int*>>
using NonSimpleForwardCommonOuter = BufferView<forward_iterator<const Inner*>, forward_iterator<const Inner*>,
                                               forward_iterator<Inner*>, forward_iterator<Inner*>>;
static_assert(std::ranges::forward_range<NonSimpleForwardCommonOuter<>>);
static_assert(!std::ranges::bidirectional_range<NonSimpleForwardCommonOuter<>>);
static_assert(std::ranges::common_range<NonSimpleForwardCommonOuter<>>);
static_assert(!simple_view<NonSimpleForwardCommonOuter<>>);

template <class Inner = BufferView<int*>>
using SimpleForwardNonCommonOuter = BufferView<forward_iterator<Inner*>, sentinel_wrapper<forward_iterator<Inner*>>>;
static_assert(std::ranges::forward_range<SimpleForwardNonCommonOuter<>>);
static_assert(!std::ranges::bidirectional_range<SimpleForwardNonCommonOuter<>>);
static_assert(!std::ranges::common_range<SimpleForwardNonCommonOuter<>>);
static_assert(simple_view<SimpleForwardNonCommonOuter<>>);

template <class Inner = BufferView<int*>>
using NonSimpleForwardNonCommonOuter =
    BufferView<forward_iterator<const Inner*>, sentinel_wrapper<forward_iterator<const Inner*>>,
               forward_iterator<Inner*>, sentinel_wrapper<forward_iterator<Inner*>>>;
static_assert(std::ranges::forward_range<NonSimpleForwardNonCommonOuter<>>);
static_assert(!std::ranges::bidirectional_range<NonSimpleForwardNonCommonOuter<>>);
static_assert(!std::ranges::common_range<NonSimpleForwardNonCommonOuter<>>);
static_assert(!simple_view<NonSimpleForwardNonCommonOuter<>>);

template <class Inner = BufferView<int*>>
using BidiCommonOuter = BufferView<bidirectional_iterator<Inner*>>;
static_assert(std::ranges::bidirectional_range<BidiCommonOuter<>>);
static_assert(std::ranges::common_range<BidiCommonOuter<>>);
static_assert(simple_view<BidiCommonOuter<>>);

// an iterator where its operator* makes a copy of underlying operator*
template <class It>
struct copying_iterator {
  It it_ = It();

  using value_type = typename std::iterator_traits<It>::value_type;
  using difference_type = typename std::iterator_traits<It>::difference_type;
  using pointer = typename std::iterator_traits<It>::pointer;

  copying_iterator() requires std::default_initializable<It> = default;
  constexpr copying_iterator(It it) : it_(std::move(it)) {}

  // makes a copy of underlying operator* to create a PRValue
  constexpr value_type operator*() const { return *it_; }

  constexpr copying_iterator& operator++() {
    ++it_;
    return *this;
  }
  constexpr copying_iterator& operator--()
    requires std::bidirectional_iterator<It> {
    --it_;
    return *this;
  }
  constexpr copying_iterator operator++(int)
    requires std::forward_iterator<It> {
    return copying_iterator(it_++);
  }
  constexpr void operator++(int) { return it_++; }
  constexpr copying_iterator operator--(int)
    requires std::bidirectional_iterator<It> {
    return copying_iterator(it_--);
  }

  friend constexpr bool operator==(const copying_iterator& x, const copying_iterator& y) = default;
};

template <class Outer>
struct InnerRValue : Outer {

  using iterator = copying_iterator<std::ranges::iterator_t<Outer>>;
  using const_iterator = copying_iterator<std::ranges::iterator_t<const Outer>>;
  using sentinel = copying_iterator<std::ranges::sentinel_t<Outer>>;
  using const_sentinel = copying_iterator<std::ranges::sentinel_t<const Outer>>;

  using Outer::Outer;
  static_assert(std::ranges::common_range<Outer>, "non-common range is not supported yet");

  constexpr iterator begin() { return Outer::begin(); }
  constexpr const_iterator begin() const
    requires std::ranges::range<const Outer> {
    return Outer::begin();
  }

  constexpr auto end() { return iterator{Outer::end()}; }
  constexpr auto end() const
    requires std::ranges::range<const Outer> {
    return const_iterator{Outer::end()};
  }
};
static_assert(std::ranges::forward_range<InnerRValue<SimpleForwardCommonOuter<>>>);
static_assert(!std::ranges::bidirectional_range<InnerRValue<SimpleForwardCommonOuter<>>>);
static_assert(std::ranges::common_range<InnerRValue<SimpleForwardCommonOuter<>>>);
static_assert(simple_view<InnerRValue<SimpleForwardCommonOuter<>>>);
static_assert(!std::is_lvalue_reference_v<std::ranges::range_reference_t<InnerRValue<SimpleForwardCommonOuter<>>>>);

struct move_swap_aware_iter {

  // This is a proxy-like iterator where `reference` is a prvalue, and
  // `reference` and `value_type` are distinct types (similar to `zip_view::iterator`).
  using value_type = std::pair<int, int>;
  using reference = std::pair<int&, int&>;
  using rvalue_reference = std::pair<int&&, int&&>;

  using difference_type = std::intptr_t;
  using iterator_concept = std::input_iterator_tag;

  int* iter_move_called = nullptr;
  int* iter_swap_called = nullptr;
  int* i_ = nullptr;

  constexpr move_swap_aware_iter& operator++() {
    ++i_;
    return *this;
  }
  constexpr void operator++(int) { ++i_; }

  constexpr reference operator*() const { return reference(*i_, *i_); }
  constexpr friend bool operator==(const move_swap_aware_iter& x, const move_swap_aware_iter& y) {
    return x.i_ == y.i_;
  }

  constexpr friend rvalue_reference iter_move(const move_swap_aware_iter& x) noexcept {
    ++(*x.iter_move_called);
    return rvalue_reference{std::move(*x.i_), std::move(*x.i_)};
  }

  constexpr friend void iter_swap(const move_swap_aware_iter& x, const move_swap_aware_iter& y) noexcept {
    ++(*x.iter_swap_called);
    std::swap(*x.i_, *y.i_);
  }
};

struct IterMoveSwapAwareView : BufferView<int*> {
  int iter_move_called = 0;
  int iter_swap_called = 0;
  using BufferView<int*>::BufferView;

  constexpr auto begin() { return move_swap_aware_iter{&iter_move_called, &iter_swap_called, data_}; }

  constexpr auto end() { return move_swap_aware_iter{&iter_move_called, &iter_swap_called, data_ + size_}; }
};
static_assert(std::ranges::input_range<IterMoveSwapAwareView>);

class StashingIterator {
public:
  using difference_type = std::ptrdiff_t;
  using value_type      = std::string;

  constexpr StashingIterator() : letter_('a') {}

  constexpr StashingIterator& operator++() {
    str_ += letter_;
    ++letter_;
    return *this;
  }

  constexpr void operator++(int) { ++*this; }

  constexpr value_type operator*() const { return str_; }

  constexpr bool operator==(std::default_sentinel_t) const { return letter_ > 'z'; }

private:
  char letter_;
  value_type str_;
};

using StashingRange = std::ranges::subrange<StashingIterator, std::default_sentinel_t>;
static_assert(std::ranges::input_range<StashingRange>);
static_assert(!std::ranges::forward_range<StashingRange>);

class ConstNonJoinableRange : public std::ranges::view_base {
public:
  constexpr StashingIterator begin() { return {}; }
  constexpr std::default_sentinel_t end() { return {}; }

  constexpr const int* begin() const { return &val_; }
  constexpr const int* end() const { return &val_ + 1; }

private:
  int val_ = 1;
};
static_assert(std::ranges::input_range<ConstNonJoinableRange>);
static_assert(std::ranges::input_range<const ConstNonJoinableRange>);
static_assert(std::ranges::input_range<std::ranges::range_reference_t<ConstNonJoinableRange>>);
static_assert(!std::ranges::input_range<std::ranges::range_reference_t<const ConstNonJoinableRange>>);

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_JOIN_TYPES_H
