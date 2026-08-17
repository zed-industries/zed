//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_FROM_RANGE_CONTAINER_ADAPTORS_H
#define SUPPORT_FROM_RANGE_CONTAINER_ADAPTORS_H

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <queue>
#include <ranges>
#include <utility>
#include <vector>

#include "../exception_safety_helpers.h"
#include "../from_range_helpers.h"
#include "MoveOnly.h"
#include "almost_satisfies_types.h"
#include "count_new.h"
#include "test_macros.h"
#include "unwrap_container_adaptor.h"

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

  return true;
}

template <template <class...> class Adaptor,
          template <class...> class UnderlyingContainer,
          class T,
          class Iter,
          class Sent,
          class Alloc>
constexpr void test_container_adaptor_with_input(std::vector<T>&& input) {
  { // (range)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Adaptor<T> adaptor(std::from_range, in);
    UnwrapAdaptor<Adaptor<T>> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::equal(input, c));
    LIBCPP_ASSERT(c.__invariants());
  }

  { // (range, allocator)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    using C = UnderlyingContainer<T, Alloc>;
    Alloc alloc;
    Adaptor<T, C> adaptor(std::from_range, in, alloc);
    UnwrapAdaptor<Adaptor<T, C>> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();

    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::equal(input, c));
    LIBCPP_ASSERT(c.__invariants());
  }
}

template <template <class...> class UnderlyingContainer, class T, class Iter, class Sent, class Comp, class Alloc>
constexpr void test_priority_queue_with_input(std::vector<T>&& input) {
  { // (range)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    std::priority_queue<T> adaptor(std::from_range, in);
    UnwrapAdaptor<std::priority_queue<T>> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    LIBCPP_ASSERT(c.__invariants());
  }

  { // (range, comp)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    using C = UnderlyingContainer<T>;
    Comp comp;

    std::priority_queue<T, C, Comp> adaptor(std::from_range, in, comp);
    UnwrapAdaptor<std::priority_queue<T, C, Comp>> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    LIBCPP_ASSERT(c.__invariants());
    assert(unwrap_adaptor.get_comparator() == comp);
  }

  { // (range, allocator)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    using C = UnderlyingContainer<T, Alloc>;
    Alloc alloc;

    std::priority_queue<T, C> adaptor(std::from_range, in, alloc);
    UnwrapAdaptor<std::priority_queue<T, C>> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();

    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    LIBCPP_ASSERT(c.__invariants());
  }

  { // (range, comp, alloc)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    using C = UnderlyingContainer<T, Alloc>;
    Comp comp;
    Alloc alloc;

    std::priority_queue<T, C, Comp> adaptor(std::from_range, in, comp, alloc);
    UnwrapAdaptor<std::priority_queue<T, C, Comp>> unwrap_adaptor(std::move(adaptor));
    auto& c = unwrap_adaptor.get_container();

    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    LIBCPP_ASSERT(c.__invariants());
    assert(unwrap_adaptor.get_comparator() == comp);
  }
}

template <template <class...> class Adaptor,
          template <class...> class UnderlyingContainer,
          class T,
          class Iter,
          class Sent,
          class Alloc>
constexpr void test_container_adaptor() {
  auto test_with_input = &test_container_adaptor_with_input<Adaptor, UnderlyingContainer, T, Iter, Sent, Alloc>;

  // Normal input.
  test_with_input({0, 5, 12, 7, -1, 8, 26});
  // Empty input.
  test_with_input({});
  // Single-element input.
  test_with_input({5});
}

template <template <class...> class UnderlyingContainer, class T, class Iter, class Sent, class Comp, class Alloc>
constexpr void test_priority_queue() {
  auto test_with_input = &test_priority_queue_with_input<UnderlyingContainer, T, Iter, Sent, Comp, Alloc>;

  // Normal input.
  test_with_input({0, 5, 12, 7, -1, 8, 26});
  // Empty input.
  test_with_input({});
  // Single-element input.
  test_with_input({5});
}

template <template <class...> class Container>
constexpr void test_container_adaptor_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  [[maybe_unused]] Container<MoveOnly> c(std::from_range, in);
}

template <template <class...> class Adaptor>
void test_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  constexpr int ThrowOn = 3;
  using T               = ThrowingCopy<ThrowOn>;
  test_exception_safety_throwing_copy<ThrowOn, /*Size=*/5>([](T* from, T* to) {
    [[maybe_unused]] Adaptor<T, std::vector<T>> c(std::from_range, std::ranges::subrange(from, to));
  });
#endif
}

template <template <class...> class Adaptor, class T>
void test_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {0, 1};

  try {
    using C = std::vector<T, ThrowingAllocator<T>>;
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Adaptor<T, C> c(std::from_range, in, alloc);
    assert(false); // The constructor call should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

#endif // SUPPORT_FROM_RANGE_CONTAINER_ADAPTORS_H
