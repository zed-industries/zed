//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_FROM_RANGE_UNORDERED_CONTAINERS_H
#define SUPPORT_FROM_RANGE_UNORDERED_CONTAINERS_H

#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstddef>
#include <limits>
#include <ranges>
#include <vector>
#include <utility>

#include "../exception_safety_helpers.h"
#include "../from_range_helpers.h"
#include "../test_compare.h"
#include "../test_hash.h"
#include "MoveOnly.h"
#include "almost_satisfies_types.h"
#include "count_new.h"
#include "test_macros.h"

// template<container-compatible-range<value_type> R>
//   unordered-container(from_range_t, R&& rg, size_type n = see below,
//     const hasher& hf = hasher(), const key_equal& eql = key_equal(),
//     const allocator_type& a = allocator_type()); // C++23
//
// template<container-compatible-range<value_type> R>
//   unordered-container(from_range_t, R&& rg, size_type n, const allocator_type& a)
//     : unordered-container(from_range, std::forward<R>(rg), n, hasher(), key_equal(), a) { } // C++23
//
// template<container-compatible-range<value_type> R>
//   unordered-container(from_range_t, R&& rg, size_type n, const hasher& hf, const allocator_type& a)
//     : unordered-container(from_range, std::forward<R>(rg), n, hf, key_equal(), a) { }       // C++23

template <class Container, class Range>
concept HasFromRangeCtr = requires(Range&& range) {
  // (from_range, range)
  Container(std::from_range, std::forward<Range>(range));
  // (from_range, range, n)
  Container(std::from_range, std::forward<Range>(range), 0);
  // (from_range, range, n, hash)
  Container(std::from_range, std::forward<Range>(range), 0, std::hash<typename Container::key_type>());
  // (from_range, range, n, hash, equal)
  Container(std::from_range,
            std::forward<Range>(range),
            0,
            std::hash<typename Container::key_type>(),
            std::equal_to<typename Container::key_type>());
  // (from_range, range, n, hash, equal, alloc)
  Container(std::from_range,
            std::forward<Range>(range),
            0,
            std::hash<typename Container::key_type>(),
            std::equal_to<typename Container::key_type>(),
            std::allocator<typename Container::value_type>());
  // (from_range, range, n, alloc)
  Container(std::from_range, std::forward<Range>(range), 0, std::allocator<typename Container::value_type>());
  // (from_range, range, n, hash, alloc)
  Container(std::from_range,
            std::forward<Range>(range),
            0,
            std::hash<typename Container::key_type>(),
            std::allocator<typename Container::value_type>());
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
          class Hash,
          class Equal,
          class Alloc,
          class ValueType = std::pair<const K, V>>
void test_unordered_map_with_input(std::vector<ValueType>&& input) {
  using DefaultHash  = std::hash<int>;
  using DefaultEqual = std::equal_to<int>;

  auto validate = [](auto&& c) {
    if (!c.empty()) {
      auto diff = c.load_factor() - (static_cast<float>(c.size()) / c.bucket_count());
      assert(std::fabs(diff) < std::numeric_limits<float>::epsilon());
    }
    assert(c.max_load_factor() == 1);
  };

  { // (range)
    auto in = wrap_input<Iter, Sent>(input);
    Container<K, V> c(std::from_range, in);

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n)
    auto in = wrap_input<Iter, Sent>(input);
    Container<K, V> c(std::from_range, in, 123);

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher)
    auto in = wrap_input<Iter, Sent>(input);
    Container<K, V, Hash> c(std::from_range, in, 123, Hash());

    assert(c.hash_function() == Hash());
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher, key_equal)
    auto in = wrap_input<Iter, Sent>(input);
    Container<K, V, Hash, Equal> c(std::from_range, in, 123, Hash(), Equal());

    assert(c.hash_function() == Hash());
    assert(c.key_eq() == Equal());
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher, key_equal, allocator)
    auto in = wrap_input<Iter, Sent>(input);
    Alloc alloc;
    Container<K, V, Hash, Equal, Alloc> c(std::from_range, in, 123, Hash(), Equal(), alloc);

    assert(c.hash_function() == Hash());
    assert(c.key_eq() == Equal());
    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, allocator)
    auto in = wrap_input<Iter, Sent>(input);
    Alloc alloc;
    Container<K, V, DefaultHash, DefaultEqual, Alloc> c(std::from_range, in, 123, alloc);

    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher, allocator)
    auto in = wrap_input<Iter, Sent>(input);
    Alloc alloc;
    Container<K, V, Hash, DefaultEqual, Alloc> c(std::from_range, in, 123, Hash(), alloc);

    assert(c.hash_function() == Hash());
    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }
}

template <template <class...> class Container,
          class K,
          class V,
          class Iter,
          class Sent,
          class Hash,
          class Equal,
          class Alloc>
void test_unordered_map() {
  auto test_with_input = &test_unordered_map_with_input<Container, K, V, Iter, Sent, Hash, Equal, Alloc>;

  // Normal input.
  test_with_input({{1, 2}, {3, 4}, {5, 6}, {7, 8}, {9, 10}});
  // Empty input.
  test_with_input({});
  // Single-element input.
  test_with_input({{1, 2}});
}

template <template <class...> class Container>
void test_unordered_map_move_only() {
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
    Container<K, V, test_hash<K>, test_equal_to<K>, ThrowingAllocator<ValueType>> c(
        std::from_range, in, /*n=*/0, alloc);
    assert(false); // The constructor call should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <class Container, class Range>
concept SetHasFromRangeCtr = requires(Range&& range) {
  // (from_range, range)
  Container(std::from_range, std::forward<Range>(range));
  // (from_range, range, n)
  Container(std::from_range, std::forward<Range>(range), 0);
  // (from_range, range, n, hash)
  Container(std::from_range, std::forward<Range>(range), 0, std::hash<typename Container::value_type>());
  // (from_range, range, n, hash, equal)
  Container(std::from_range,
            std::forward<Range>(range),
            0,
            std::hash<typename Container::value_type>(),
            std::equal_to<typename Container::value_type>());
  // (from_range, range, n, hash, equal, alloc)
  Container(std::from_range,
            std::forward<Range>(range),
            0,
            std::hash<typename Container::value_type>(),
            std::equal_to<typename Container::value_type>(),
            std::allocator<typename Container::value_type>());
  // (from_range, range, n, alloc)
  Container(std::from_range, std::forward<Range>(range), 0, std::allocator<typename Container::value_type>());
  // (from_range, range, n, hash, alloc)
  Container(std::from_range,
            std::forward<Range>(range),
            0,
            std::hash<typename Container::value_type>(),
            std::allocator<typename Container::value_type>());
};

template <template <class...> class Container, class T, class U>
constexpr bool test_set_constraints() {
  // Input range with the same value type.
  static_assert(HasFromRangeCtr<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(HasFromRangeCtr<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!HasFromRangeCtr<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!HasFromRangeCtr<Container<T>, InputRangeNotDerivedFromGeneric<T>>);

  return true;
}

template <template <class...> class Container, class T, class Iter, class Sent, class Hash, class Equal, class Alloc>
void test_unordered_set_with_input(std::vector<T>&& input) {
  using DefaultHash  = std::hash<int>;
  using DefaultEqual = std::equal_to<int>;

  auto validate = [](auto&& c) {
    if (!c.empty()) {
      auto diff = c.load_factor() - (static_cast<float>(c.size()) / c.bucket_count());
      assert(std::fabs(diff) < std::numeric_limits<float>::epsilon());
    }
    assert(c.max_load_factor() == 1);
  };

  { // (range)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Container<T> c(std::from_range, in);

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Container<T> c(std::from_range, in, 123);

    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Container<T, Hash> c(std::from_range, in, 123, Hash());

    assert(c.hash_function() == Hash());
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher, key_equal)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Container<T, Hash, Equal> c(std::from_range, in, 123, Hash(), Equal());

    assert(c.hash_function() == Hash());
    assert(c.key_eq() == Equal());
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher, key_equal, allocator)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Alloc alloc;
    Container<T, Hash, Equal, Alloc> c(std::from_range, in, 123, Hash(), Equal(), alloc);

    assert(c.hash_function() == Hash());
    assert(c.key_eq() == Equal());
    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, allocator)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Alloc alloc;
    Container<T, DefaultHash, DefaultEqual, Alloc> c(std::from_range, in, 123, alloc);

    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }

  { // (range, n, hasher, allocator)
    std::ranges::subrange in(Iter(input.data()), Sent(Iter(input.data() + input.size())));
    Alloc alloc;
    Container<T, Hash, DefaultEqual, Alloc> c(std::from_range, in, 123, Hash(), alloc);

    assert(c.hash_function() == Hash());
    assert(c.get_allocator() == alloc);
    assert(c.size() == static_cast<std::size_t>(std::distance(c.begin(), c.end())));
    assert(std::ranges::is_permutation(input, c));
    validate(c);
  }
}

template <template <class...> class Container, class T, class Iter, class Sent, class Hash, class Equal, class Alloc>
void test_unordered_set() {
  auto test_with_input = &test_unordered_set_with_input<Container, T, Iter, Sent, Hash, Equal, Alloc>;

  // Normal input.
  test_with_input({0, 5, 12, 7, -1, 8, 26});
  // Empty input.
  test_with_input({});
  // Single-element input.
  test_with_input({5});
}

template <template <class...> class Container>
void test_unordered_set_move_only() {
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
    Container<T, test_hash<T>> c(std::from_range, in);
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
  T in[] = {1, 2, 3};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, test_hash<T>, test_equal_to<T>, ThrowingAllocator<T>> c(std::from_range, in, /*n=*/0, alloc);
    assert(false); // The constructor call should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

#endif // SUPPORT_FROM_RANGE_UNORDERED_CONTAINERS_H
