//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_FROM_RANGE_SEQUENCE_CONTAINERS_H
#define SUPPORT_FROM_RANGE_SEQUENCE_CONTAINERS_H

#include <algorithm>
#include <array>
#include <cassert>
#include <cstddef>
#include <iterator>
#include <ranges>
#include <utility>

#include "../exception_safety_helpers.h"
#include "../from_range_helpers.h"
#include "MoveOnly.h"
#include "almost_satisfies_types.h"
#include "count_new.h"
#include "test_iterators.h"
#include "test_macros.h"

template <class T>
concept HasSize = requires(const T& value) { value.size(); };

template <class Container, class Range>
concept HasFromRangeCtr = requires(Range&& range) {
  Container(std::from_range, std::forward<Range>(range));
  Container(std::from_range, std::forward<Range>(range), std::allocator<typename Container::value_type>());
};

template <template <class...> class Container, class T, class U>
constexpr bool test_constraints() {
  // Input range with the same value type.
  static_assert(HasFromRangeCtr<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(HasFromRangeCtr<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!HasFromRangeCtr<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!HasFromRangeCtr<Container<T>, InputRangeNotDerivedFrom>);
  static_assert(!HasFromRangeCtr<Container<T>, InputRangeNotIndirectlyReadable>);
  static_assert(!HasFromRangeCtr<Container<T>, InputRangeNotInputOrOutputIterator>);

  // Note: there are no constraints on the allocator (it's not a separate template type of the constructor)`.

  return true;
}

// Note: `std::array` is used to avoid dealing with `vector<bool>`.
template <template <class...> class Container,
          class T,
          class Iter,
          class Sent,
          class Alloc,
          std::size_t N,
          class ValidateFunc>
constexpr void test_sequence_container_with_input(std::array<T, N>&& input, ValidateFunc validate) {
  { // (range)
    auto in = wrap_input<Iter, Sent>(input);
    Container<T> c(std::from_range, in);

    if constexpr (HasSize<Container<T>>) {
      assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    }
    assert(std::ranges::equal(input, c));
    validate(c);
  }

  { // (range, allocator)
    auto in = wrap_input<Iter, Sent>(input);
    Alloc alloc;
    Container<T, Alloc> c(std::from_range, in, alloc);

    assert(c.get_allocator() == alloc);
    if constexpr (HasSize<Container<T, Alloc>>) {
      assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    }
    assert(std::ranges::equal(input, c));
    validate(c);
  }
}

template <template <class...> class Container, class T, class Iter, class Sent, class Alloc, class ValidateFunc>
constexpr void test_sequence_container(ValidateFunc validate) {
  // Normal input.
  test_sequence_container_with_input<Container, T, Iter, Sent, Alloc>(std::array{0, 5, 12, 7, -1, 8, 26}, validate);
  // Empty input.
  test_sequence_container_with_input<Container, T, Iter, Sent, Alloc>(std::array<int, 0>{}, validate);
  // Single-element input.
  test_sequence_container_with_input<Container, T, Iter, Sent, Alloc>(std::array{5}, validate);
}

template <template <class...> class Container>
constexpr void test_sequence_container_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  [[maybe_unused]] Container<MoveOnly> c(std::from_range, in);
}

template <class Iter, class Sent, class Alloc, class ValidateFunc>
constexpr void test_vector_bool(ValidateFunc validate) {
  // Normal input.
  test_sequence_container_with_input<std::vector, bool, Iter, Sent, Alloc>(
      std::array{true, false, false, true, false, true, true, true, false, true}, validate);
  // Empty input.
  test_sequence_container_with_input<std::vector, bool, Iter, Sent, Alloc>(std::array<bool, 0>{}, validate);
  // Single-element input.
  test_sequence_container_with_input<std::vector, bool, Iter, Sent, Alloc>(std::array{true}, validate);
}

template <template <class...> class Container>
void test_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  constexpr int ThrowOn = 3;
  using T               = ThrowingCopy<ThrowOn>;
  test_exception_safety_throwing_copy<ThrowOn, /*Size=*/5>([](T* from, T* to) {
    [[maybe_unused]] Container<T> c(std::from_range, std::ranges::subrange(from, to));
  });
#endif
}

template <template <class...> class Container, class T>
void test_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {0, 1};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, ThrowingAllocator<T>> c(std::from_range, in, alloc);
    assert(false); // The constructor call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

#endif // SUPPORT_FROM_RANGE_SEQUENCE_CONTAINERS_H
