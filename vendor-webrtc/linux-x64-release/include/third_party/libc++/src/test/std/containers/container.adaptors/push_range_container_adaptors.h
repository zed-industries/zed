//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_PUSH_RANGE_CONTAINER_ADAPTORS_H
#define SUPPORT_PUSH_RANGE_CONTAINER_ADAPTORS_H

#include <algorithm>
#include <cassert>
#include <concepts>
#include <cstddef>
#include <initializer_list>
#include <ranges>
#include <type_traits>
#include <vector>

#include "../exception_safety_helpers.h"
#include "../from_range_helpers.h"
#include "../insert_range_helpers.h"
#include "MoveOnly.h"
#include "almost_satisfies_types.h"
#include "count_new.h"
#include "min_allocator.h"
#include "test_allocator.h"
#include "test_iterators.h"
#include "test_macros.h"
#include "type_algorithms.h"
#include "unwrap_container_adaptor.h"

template <class Container, class Range>
concept HasPushRange = requires(Container& c, Range&& range) { c.push_range(range); };

template <template <class...> class Container, class T, class U>
constexpr bool test_constraints_push_range() {
  // Input range with the same value type.
  static_assert(HasPushRange<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(HasPushRange<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!HasPushRange<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!HasPushRange<Container<T>, InputRangeNotDerivedFrom>);
  static_assert(!HasPushRange<Container<T>, InputRangeNotIndirectlyReadable>);
  static_assert(!HasPushRange<Container<T>, InputRangeNotInputOrOutputIterator>);

  return true;
}

// Empty container.

template <class T>
TestCase<T> constexpr EmptyContainer_EmptyRange{.initial = {}, .input = {}, .expected = {}};

template <class T>
constexpr TestCase<T> EmptyContainer_OneElementRange{.initial = {}, .input = {5}, .expected = {5}};

template <class T>
constexpr TestCase<T> EmptyContainer_MidRange{.initial = {}, .input = {5, 3, 1, 7, 9}, .expected = {5, 3, 1, 7, 9}};

// One-element container.

template <class T>
constexpr TestCase<T> OneElementContainer_EmptyRange{.initial = {3}, .input = {}, .expected = {3}};

template <class T>
constexpr TestCase<T> OneElementContainer_OneElementRange{.initial = {3}, .input = {-5}, .expected = {3, -5}};

template <class T>
constexpr TestCase<T> OneElementContainer_MidRange{
    .initial = {3}, .input = {-5, -3, -1, -7, -9}, .expected = {3, -5, -3, -1, -7, -9}};

// Full container.

template <class T>
constexpr TestCase<T> FullContainer_EmptyRange{
    .initial = {11, 29, 35, 14, 84}, .input = {}, .expected = {11, 29, 35, 14, 84}};

template <class T>
constexpr TestCase<T> FullContainer_OneElementRange{
    .initial = {11, 29, 35, 14, 84}, .input = {-5}, .expected = {11, 29, 35, 14, 84, -5}};

template <class T>
constexpr TestCase<T> FullContainer_MidRange{
    .initial  = {11, 29, 35, 14, 84},
    .input    = {-5, -3, -1, -7, -9},
    .expected = {11, 29, 35, 14, 84, -5, -3, -1, -7, -9}};

template <class T>
constexpr TestCase<T> FullContainer_LongRange{
    .initial  = {11, 29, 35, 14, 84},
    .input    = {-5, -3, -1, -7, -9, -19, -48, -56, -13, -14, -29, -88, -17, -1, -5, -11, -89, -21, -33, -48},
    .expected = {11,  29,  35,  14,  84,  -5, -3, -1,  -7,  -9,  -19, -48, -56,
                 -13, -14, -29, -88, -17, -1, -5, -11, -89, -21, -33, -48}};

// Container adaptors tests.

template <class Adaptor, class Iter, class Sent>
constexpr void test_push_range(bool is_result_heapified = false) {
  using T = typename Adaptor::value_type;

  auto test = [&](auto& test_case) {
    Adaptor adaptor(test_case.initial.begin(), test_case.initial.end());
    auto in = wrap_input<Iter, Sent>(test_case.input);

    adaptor.push_range(in);
    UnwrapAdaptor<Adaptor> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();

    if (is_result_heapified) {
      assert(std::ranges::is_heap(c));
      return std::ranges::is_permutation(c, test_case.expected);
    } else {
      return std::ranges::equal(c, test_case.expected);
    }
  };

  { // Empty container.
    // empty_c.push_range(empty_range)
    assert(test(EmptyContainer_EmptyRange<T>));
    // empty_c.push_range(one_element_range)
    assert(test(EmptyContainer_OneElementRange<T>));
    // empty_c.push_range(mid_range)
    assert(test(EmptyContainer_MidRange<T>));
  }

  { // One-element container.
    // one_element_c.push_range(empty_range)
    assert(test(OneElementContainer_EmptyRange<T>));
    // one_element_c.push_range(one_element_range)
    assert(test(OneElementContainer_OneElementRange<T>));
    // one_element_c.push_range(mid_range)
    assert(test(OneElementContainer_MidRange<T>));
  }

  { // Full container.
    // full_container.push_range(empty_range)
    assert(test(FullContainer_EmptyRange<T>));
    // full_container.push_range(one_element_range)
    assert(test(FullContainer_OneElementRange<T>));
    // full_container.push_range(mid_range)
    assert(test(FullContainer_MidRange<T>));
    // full_container.push_range(long_range)
    assert(test(FullContainer_LongRange<T>));
  }
}

// Move-only types.

template <template <class...> class Container>
constexpr void test_push_range_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  Container<MoveOnly> c;
  c.push_range(in);
}

// Check that `append_range` is preferred if available and `push_back` is used as a fallback.

enum class InserterChoice { Invalid, PushBack, AppendRange };

template <class T, InserterChoice Inserter>
struct Container {
  InserterChoice inserter_choice = InserterChoice::Invalid;

  using value_type      = T;
  using iterator        = T*;
  using reference       = T&;
  using const_reference = const T&;
  using size_type       = std::size_t;

  static constexpr int Capacity = 8;
  int size_                     = 0;
  value_type buffer_[Capacity]  = {};

  iterator begin() { return buffer_; }
  iterator end() { return buffer_ + size_; }
  size_type size() const { return size_; }

  template <class U>
  void push_back(U val)
    requires(Inserter >= InserterChoice::PushBack)
  {
    inserter_choice = InserterChoice::PushBack;
    buffer_[size_]  = val;
    ++size_;
  }

  template <std::ranges::input_range Range>
  void append_range(Range&& range)
    requires(Inserter >= InserterChoice::AppendRange)
  {
    assert(size() + std::ranges::distance(range) <= Capacity);

    inserter_choice = InserterChoice::AppendRange;

    for (auto&& e : range) {
      buffer_[size_] = e;
      ++size_;
    }
  }

  friend bool operator==(const Container&, const Container&) = default;
};

template <template <class...> class AdaptorT, class T>
void test_push_range_inserter_choice(bool is_result_heapified = false) {
  { // `append_range` is preferred if available.
    using BaseContainer = Container<T, InserterChoice::AppendRange>;
    using Adaptor       = AdaptorT<T, BaseContainer>;
    T in[]              = {1, 2, 3, 4, 5};

    Adaptor adaptor;
    adaptor.push_range(in);

    UnwrapAdaptor<Adaptor> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();
    assert(c.inserter_choice == InserterChoice::AppendRange);
    if (is_result_heapified) {
      assert(std::ranges::is_heap(c));
      assert(std::ranges::is_permutation(c, in));
    } else {
      assert(std::ranges::equal(c, in));
    }
  }

  { // `push_back` is used as a fallback (via `back_inserter`).
    using BaseContainer = Container<T, InserterChoice::PushBack>;
    using Adaptor       = AdaptorT<T, BaseContainer>;
    T in[]              = {1, 2, 3, 4, 5};

    Adaptor adaptor;
    adaptor.push_range(in);

    UnwrapAdaptor<Adaptor> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();
    assert(c.inserter_choice == InserterChoice::PushBack);
    if (is_result_heapified) {
      assert(std::ranges::is_heap(c));
      assert(std::ranges::is_permutation(c, in));
    } else {
      assert(std::ranges::equal(c, in));
    }
  }
}

// Exception safety.

template <template <class...> class Container>
void test_push_range_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  constexpr int ThrowOn = 3;
  using T               = ThrowingCopy<ThrowOn>;
  test_exception_safety_throwing_copy<ThrowOn, /*Size=*/5>([](auto* from, auto* to) {
    Container<T> c;
    c.push_range(std::ranges::subrange(from, to));
  });
#endif
}

template <template <class...> class Adaptor, template <class...> class BaseContainer, class T>
void test_push_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {0, 1};

  try {
    globalMemCounter.reset();
    Adaptor<T, BaseContainer<T, ThrowingAllocator<T>>> c;
    c.push_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

#endif // SUPPORT_PUSH_RANGE_CONTAINER_ADAPTORS_H
