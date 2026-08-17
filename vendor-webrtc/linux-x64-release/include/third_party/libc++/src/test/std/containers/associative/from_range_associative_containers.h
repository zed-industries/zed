//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_FROM_RANGE_ASSOCIATIVE_CONTAINERS_H
#define SUPPORT_FROM_RANGE_ASSOCIATIVE_CONTAINERS_H

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <ranges>
#include <vector>
#include <utility>

#include "../exception_safety_helpers.h"
#include "../from_range_helpers.h"
#include "../test_compare.h"
#include "MoveOnly.h"
#include "almost_satisfies_types.h"
#include "count_new.h"
#include "test_macros.h"

template <class Container, class Range>
concept HasFromRangeCtr = requires(Range&& range) {
  Container(std::from_range, std::forward<Range>(range));
  Container(std::from_range, std::forward<Range>(range), std::less<typename Container::key_type>());
  Container(std::from_range,
            std::forward<Range>(range),
            std::less<typename Container::key_type>(),
            std::allocator<typename Container::value_type>());
  Container(std::from_range, std::forward<Range>(range), std::allocator<typename Container::value_type>());
};

template <template <class...> class Container, class K, class V, class K2, class V2>
constexpr bool test_map_constraints() {
  using ValueType = std::pair<const K, V>;

  // Input range with the same value type.
  static_assert(HasFromRangeCtr<Container<K, V>, InputRange<ValueType>>);
  // Input range with a convertible value type.
  static_assert(HasFromRangeCtr<Container<K, V>, InputRange<std::pair<const K2, V2>>>);
  // Input range with a non-convertible value type.
  static_assert(!HasFromRangeCtr<Container<K, V>, InputRange<std::pair<const Empty, V>>>);
  static_assert(!HasFromRangeCtr<Container<K, V>, InputRange<std::pair<const K, Empty>>>);
  // Not an input range.
  static_assert(!HasFromRangeCtr<Container<K, V>, InputRangeNotDerivedFromGeneric<ValueType>>);

  return true;
}

template <template <class...> class Container,
          class K,
          class V,
          class Iter,
          class Sent,
          class Comp,
          class Alloc,
          class ValueType = std::pair<const K, V>>
void test_associative_map_with_input(std::vector<ValueType>&& input) {
  { // (range)
    auto in = wrap_input<Iter, Sent>(input);
    Container<K, V> c(std::from_range, in);

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }

  { // (range, comp)
    auto in = wrap_input<Iter, Sent>(input);
    Comp comp;
    Container<K, V, Comp> c(std::from_range, in, comp);

    assert(c.key_comp() == comp);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }

  { // (range, allocator)
    auto in = wrap_input<Iter, Sent>(input);
    Alloc alloc;
    Container<K, V, std::less<K>, Alloc> c(std::from_range, in, alloc);

    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }

  { // (range, comp, allocator)
    auto in = wrap_input<Iter, Sent>(input);
    Comp comp;
    Alloc alloc;
    Container<K, V, Comp, Alloc> c(std::from_range, in, comp, alloc);

    assert(c.key_comp() == comp);
    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }
}

template <template <class...> class Container, class K, class V, class Iter, class Sent, class Comp, class Alloc>
void test_associative_map() {
  auto test_with_input = &test_associative_map_with_input<Container, K, V, Iter, Sent, Comp, Alloc>;

  // Normal input.
  test_with_input({{1, 2}, {3, 4}, {5, 6}, {7, 8}, {9, 10}});
  // Empty input.
  test_with_input({});
  // Single-element input.
  test_with_input({{1, 2}});
}

template <template <class...> class Container>
void test_associative_map_move_only() {
  std::pair<const int, MoveOnly> input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  [[maybe_unused]] Container<int, MoveOnly> c(std::from_range, in);
}

template <template <class...> class Container>
void test_map_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  using K = int;
  using V = ThrowingCopy<3>;

  V::throwing_enabled         = false;
  std::pair<const K, V> in[5] = {{1, {}}, {2, {}}, {3, {}}, {4, {}}, {5, {}}};
  V::throwing_enabled         = true;
  V::reset();

  try {
    Container<K, V> c(std::from_range, in);
    assert(false); // The constructor call above should throw.

  } catch (int) {
    assert(V::created_by_copying == 3);
    assert(V::destroyed == 2); // No destructor call for the partially-constructed element.
  }
#endif
}

template <template <class...> class Container, class K, class V>
void test_map_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  using ValueType = std::pair<const K, V>;
  ValueType in[]  = {ValueType{K{1}, V{1}}};

  try {
    ThrowingAllocator<ValueType> alloc;

    globalMemCounter.reset();
    Container<K, V, test_less<K>, ThrowingAllocator<ValueType>> c(std::from_range, in, alloc);
    assert(false); // The constructor call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <class Container, class Range>
concept SetHasFromRangeCtr = requires(Range&& range) {
  Container(std::from_range, std::forward<Range>(range));
  Container(std::from_range, std::forward<Range>(range), std::less<typename Container::value_type>());
  Container(std::from_range,
            std::forward<Range>(range),
            std::less<typename Container::value_type>(),
            std::allocator<typename Container::value_type>());
  Container(std::from_range, std::forward<Range>(range), std::allocator<typename Container::value_type>());
};

template <template <class...> class Container, class T, class U>
constexpr bool test_set_constraints() {
  // Input range with the same value type.
  static_assert(SetHasFromRangeCtr<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(SetHasFromRangeCtr<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!SetHasFromRangeCtr<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!SetHasFromRangeCtr<Container<T>, InputRangeNotDerivedFromGeneric<T>>);

  return true;
}

template <template <class...> class Container, class T, class Iter, class Sent, class Comp, class Alloc>
void test_associative_set_with_input(std::vector<T>&& input) {
  { // (range)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Container<T> c(std::from_range, in);

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }

  { // (range, comp)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Comp comp;
    Container<T, Comp> c(std::from_range, in, comp);

    assert(c.key_comp() == comp);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }

  { // (range, allocator)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Alloc alloc;
    Container<T, std::less<T>, Alloc> c(std::from_range, in, alloc);

    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }

  { // (range, comp, allocator)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Comp comp;
    Alloc alloc;
    Container<T, Comp, Alloc> c(std::from_range, in, comp, alloc);

    assert(c.key_comp() == comp);
    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
  }
}

template <template <class...> class Container, class T, class Iter, class Sent, class Comp, class Alloc>
void test_associative_set() {
  auto test_with_input = &test_associative_set_with_input<Container, T, Iter, Sent, Comp, Alloc>;

  // Normal input.
  test_with_input({0, 5, 12, 7, -1, 8, 26});
  // Empty input.
  test_with_input({});
  // Single-element input.
  test_with_input({5});
}

template <template <class...> class Container>
void test_associative_set_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  [[maybe_unused]] Container<MoveOnly> c(std::from_range, in);
}

template <template <class...> class Container>
void test_set_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  using T = ThrowingCopy<3>;
  T::reset();
  T in[5] = {{1}, {2}, {3}, {4}, {5}};

  try {
    Container<T> c(std::from_range, in);
    assert(false); // The constructor call above should throw.

  } catch (int) {
    assert(T::created_by_copying == 3);
    assert(T::destroyed == 2); // No destructor call for the partially-constructed element.
  }
#endif
}

template <template <class...> class Container, class T>
void test_set_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {1, 2};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, test_less<T>, ThrowingAllocator<T>> c(std::from_range, in, alloc);
    assert(false); // The constructor call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

#endif // SUPPORT_FROM_RANGE_ASSOCIATIVE_CONTAINERS_H
