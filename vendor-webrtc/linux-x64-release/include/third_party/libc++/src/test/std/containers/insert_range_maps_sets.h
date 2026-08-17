//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_INSERT_RANGE_MAPS_SETS_H
#define SUPPORT_INSERT_RANGE_MAPS_SETS_H

#include <algorithm>
#include <array>
#include <cassert>
#include <concepts>
#include <ranges>
#include <type_traits>
#include <vector>

#include "MoveOnly.h"
#include "almost_satisfies_types.h"
#include "count_new.h"
#include "exception_safety_helpers.h"
#include "insert_range_helpers.h"
#include "min_allocator.h"
#include "test_allocator.h"
#include "test_compare.h"
#include "test_hash.h"
#include "test_iterators.h"
#include "test_macros.h"
#include "type_algorithms.h"

template <class Container, class Range>
concept HasInsertRange = requires(Container& c, Range&& range) { c.insert_range(range); };

template <template <class...> class Container, class T, class U>
constexpr bool test_set_constraints_insert_range() {
  // Input range with the same value type.
  static_assert(HasInsertRange<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(HasInsertRange<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!HasInsertRange<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!HasInsertRange<Container<T>, InputRangeNotDerivedFrom>);
  static_assert(!HasInsertRange<Container<T>, InputRangeNotIndirectlyReadable>);
  static_assert(!HasInsertRange<Container<T>, InputRangeNotInputOrOutputIterator>);

  return true;
}

template <template <class...> class Container, class K, class V, class K2, class V2>
constexpr bool test_map_constraints_insert_range() {
  using ValueType = std::pair<const K, V>;

  // Input range with the same value type.
  static_assert(HasInsertRange<Container<K, V>, InputRange<ValueType>>);
  // Input range with a convertible value type.
  static_assert(HasInsertRange<Container<K, V>, InputRange<std::pair<const K2, V2>>>);
  // Input range with a non-convertible value type.
  static_assert(!HasInsertRange<Container<K, V>, InputRange<std::pair<const K, Empty>>>);
  static_assert(!HasInsertRange<Container<K, V>, InputRange<std::pair<const Empty, V>>>);
  // Not an input range.
  static_assert(!HasInsertRange<Container<K, V>, InputRangeNotDerivedFromGeneric<ValueType>>);

  return true;
}

template <class T>
struct TestCaseMapSet {
  Buffer<T> initial;
  Buffer<T> input;
  Buffer<T> expected;
  Buffer<T> expected_multi;
};

// Empty container.

template <class T>
TestCaseMapSet<T> constexpr EmptyContainer_EmptyRange{.initial = {}, .input = {}, .expected = {}};

template <class T>
TestCaseMapSet<T> constexpr EmptyContainer_OneElementRange{.initial = {}, .input = {1}, .expected = {1}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr EmptyContainer_OneElementRange<std::pair<K, V>>{
    .initial = {}, .input = {{1, 'a'}}, .expected = {{1, 'a'}}};

template <class T>
TestCaseMapSet<T> constexpr EmptyContainer_RangeNoDuplicates{
    .initial = {}, .input = {5, 1, 3, 8, 6}, .expected = {5, 1, 3, 8, 6}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr EmptyContainer_RangeNoDuplicates<std::pair<K, V>>{
    .initial  = {},
    .input    = {{5, 'a'}, {1, 'e'}, {3, 'i'}, {8, 'o'}, {6, 'u'}},
    .expected = {{5, 'a'}, {1, 'e'}, {3, 'i'}, {8, 'o'}, {6, 'u'}}};

template <class T>
TestCaseMapSet<T> constexpr EmptyContainer_RangeWithDuplicates{
    .initial        = {},
    .input          = {5, 1, 1, 3, 5, 8, 5, 6, 10},
    .expected       = {5, 1, 3, 8, 6, 10},
    .expected_multi = {5, 1, 1, 3, 5, 8, 5, 6, 10}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr EmptyContainer_RangeWithDuplicates<std::pair<K, V>>{
    .initial        = {},
    .input          = {{5, 'a'}, {1, 'a'}, {1, 'b'}, {3, 'a'}, {5, 'b'}, {8, 'a'}, {5, 'c'}, {6, 'a'}, {10, 'b'}},
    .expected       = {{5, 'a'}, {1, 'a'}, {3, 'a'}, {8, 'a'}, {6, 'a'}, {10, 'b'}},
    .expected_multi = {{5, 'a'}, {1, 'a'}, {1, 'b'}, {3, 'a'}, {5, 'b'}, {8, 'a'}, {5, 'c'}, {6, 'a'}, {10, 'b'}}};

// One-element container.

template <class T>
TestCaseMapSet<T> constexpr OneElementContainer_EmptyRange{.initial = {10}, .input = {}, .expected = {10}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr OneElementContainer_EmptyRange<std::pair<K, V>>{
    .initial = {{10, 'A'}}, .input = {}, .expected = {{10, 'A'}}};

template <class T>
TestCaseMapSet<T> constexpr OneElementContainer_OneElementRange{.initial = {10}, .input = {1}, .expected = {1, 10}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr OneElementContainer_OneElementRange<std::pair<K, V>>{
    .initial = {{10, 'A'}}, .input = {{1, 'a'}}, .expected = {{1, 'a'}, {10, 'A'}}};

template <class T>
TestCaseMapSet<T> constexpr OneElementContainer_RangeNoDuplicates{
    .initial = {10}, .input = {5, 1, 3, 8, 6}, .expected = {5, 1, 3, 8, 6, 10}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr OneElementContainer_RangeNoDuplicates<std::pair<K, V>>{
    .initial  = {{10, 'A'}},
    .input    = {{5, 'a'}, {1, 'e'}, {3, 'i'}, {8, 'o'}, {6, 'u'}},
    .expected = {{5, 'a'}, {1, 'e'}, {3, 'i'}, {8, 'o'}, {6, 'u'}, {10, 'A'}}};

template <class T>
TestCaseMapSet<T> constexpr OneElementContainer_RangeWithDuplicates{
    .initial        = {10},
    .input          = {5, 1, 1, 3, 5, 8, 5, 6, 10},
    .expected       = {5, 1, 3, 8, 6, 10},
    .expected_multi = {5, 1, 1, 3, 5, 8, 5, 6, 10, 10}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr OneElementContainer_RangeWithDuplicates<std::pair<K, V>>{
    .initial        = {{10, 'A'}},
    .input          = {{5, 'a'}, {1, 'a'}, {1, 'b'}, {3, 'a'}, {5, 'b'}, {8, 'a'}, {5, 'c'}, {6, 'a'}, {10, 'b'}},
    .expected       = {{5, 'a'}, {1, 'a'}, {3, 'a'}, {8, 'a'}, {6, 'a'}, {10, 'A'}},
    .expected_multi = {
        {5, 'a'}, {1, 'a'}, {1, 'b'}, {3, 'a'}, {5, 'b'}, {8, 'a'}, {5, 'c'}, {6, 'a'}, {10, 'A'}, {10, 'b'}}};

// N-elements container.

template <class T>
TestCaseMapSet<T> constexpr NElementsContainer_EmptyRange{
    .initial = {10, 15, 19, 16}, .input = {}, .expected = {10, 15, 19, 16}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr NElementsContainer_EmptyRange<std::pair<K, V>>{
    .initial  = {{10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}},
    .input    = {},
    .expected = {{10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}}};

template <class T>
TestCaseMapSet<T> constexpr NElementsContainer_OneElementRange{
    .initial = {10, 15, 19, 16}, .input = {1}, .expected = {1, 10, 15, 19, 16}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr NElementsContainer_OneElementRange<std::pair<K, V>>{
    .initial  = {{10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}},
    .input    = {{1, 'a'}},
    .expected = {{1, 'a'}, {10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}}};

template <class T>
TestCaseMapSet<T> constexpr NElementsContainer_RangeNoDuplicates{
    .initial = {10, 15, 19, 16}, .input = {5, 1, 3, 8, 6}, .expected = {5, 1, 3, 8, 6, 10, 15, 19, 16}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr NElementsContainer_RangeNoDuplicates<std::pair<K, V>>{
    .initial  = {{10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}},
    .input    = {{5, 'a'}, {1, 'e'}, {3, 'i'}, {8, 'o'}, {6, 'u'}},
    .expected = {{5, 'a'}, {1, 'e'}, {3, 'i'}, {8, 'o'}, {6, 'u'}, {10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}}};

template <class T>
TestCaseMapSet<T> constexpr NElementsContainer_RangeWithDuplicates{
    .initial        = {10, 15, 19, 16},
    .input          = {5, 1, 1, 3, 5, 8, 5, 6, 10},
    .expected       = {5, 1, 3, 8, 6, 10, 15, 19, 16},
    .expected_multi = {5, 1, 1, 3, 5, 8, 5, 6, 10, 10, 15, 19, 16}};
template <class K, class V>
TestCaseMapSet<std::pair<K, V>> constexpr NElementsContainer_RangeWithDuplicates<std::pair<K, V>>{
    .initial        = {{10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}},
    .input          = {{5, 'a'}, {1, 'a'}, {1, 'b'}, {3, 'a'}, {5, 'b'}, {8, 'a'}, {5, 'c'}, {6, 'a'}, {10, 'b'}},
    .expected       = {{5, 'a'}, {1, 'a'}, {3, 'a'}, {8, 'a'}, {6, 'a'}, {10, 'A'}, {15, 'B'}, {19, 'C'}, {16, 'D'}},
    .expected_multi = {
        {5, 'a'},
        {1, 'a'},
        {1, 'b'},
        {3, 'a'},
        {5, 'b'},
        {8, 'a'},
        {5, 'c'},
        {6, 'a'},
        {10, 'b'},
        {10, 'A'},
        {15, 'B'},
        {19, 'C'},
        {16, 'D'}}};

template <class Container, class T, class Iter, class Sent>
void test_map_set_insert_range(bool allow_duplicates = false) {
  auto test = [&](const TestCaseMapSet<T>& test_case, bool check_multi = false) {
    Container c(test_case.initial.begin(), test_case.initial.end());
    auto in = wrap_input<Iter, Sent>(test_case.input);

    c.insert_range(in);
    if (check_multi) {
      return std::ranges::is_permutation(c, test_case.expected_multi);
    } else {
      return std::ranges::is_permutation(c, test_case.expected);
    }
  };

  { // Empty container.
    // empty_c.insert_range(empty_range)
    assert(test(EmptyContainer_EmptyRange<T>));
    // empty_c.insert_range(one_element_range)
    assert(test(EmptyContainer_OneElementRange<T>));
    // empty_c.insert_range(range_no_duplicates)
    assert(test(EmptyContainer_RangeNoDuplicates<T>));
    // empty_c.insert_range(range_with_duplicates)
    assert(test(EmptyContainer_RangeWithDuplicates<T>, allow_duplicates));
  }

  { // One-element container.
    // one_element_c.insert_range(empty_range)
    assert(test(OneElementContainer_EmptyRange<T>));
    // one_element_c.insert_range(one_element_range)
    assert(test(OneElementContainer_OneElementRange<T>));
    // one_element_c.insert_range(range_no_duplicates)
    assert(test(OneElementContainer_RangeNoDuplicates<T>));
    // one_element_c.insert_range(range_with_duplicates)
    assert(test(OneElementContainer_RangeWithDuplicates<T>, allow_duplicates));
  }

  { // N-elements container.
    // n_elements_c.insert_range(empty_range)
    assert(test(NElementsContainer_EmptyRange<T>));
    // n_elements_c.insert_range(one_element_range)
    assert(test(NElementsContainer_OneElementRange<T>));
    // n_elements_c.insert_range(range_no_duplicates)
    assert(test(NElementsContainer_RangeNoDuplicates<T>));
    // n_elements_c.insert_range(range_with_duplicates)
    assert(test(NElementsContainer_RangeWithDuplicates<T>, allow_duplicates));
  }
}

// Move-only types.

template <template <class...> class Container>
void test_set_insert_range_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  Container<MoveOnly> c;
  c.insert_range(in);
}

template <template <class...> class Container>
void test_map_insert_range_move_only() {
  using Value = std::pair<const int, MoveOnly>;
  Value input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  Container<int, MoveOnly> c;
  c.insert_range(in);
}

// Exception safety.

template <template <class...> class Container>
void test_set_insert_range_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  using T = ThrowingCopy<3>;
  T::reset();
  T in[5] = {{1}, {2}, {3}, {4}, {5}};

  try {
    Container<T> c;
    c.insert_range(in);
    assert(false); // The constructor call above should throw.

  } catch (int) {
    assert(T::created_by_copying == 3);
    assert(T::destroyed == 2); // No destructor call for the partially-constructed element.
  }
#endif
}

template <template <class...> class Container>
void test_map_insert_range_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  using K = int;
  using V = ThrowingCopy<3>;

  V::throwing_enabled         = false;
  std::pair<const K, V> in[5] = {{1, {}}, {2, {}}, {3, {}}, {4, {}}, {5, {}}};
  V::throwing_enabled         = true;
  V::reset();

  try {
    Container<K, V> c;
    c.insert_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(V::created_by_copying == 3);
    assert(V::destroyed == 2); // No destructor call for the partially-constructed element.
  }
#endif
}

template <template <class...> class Container, class T>
void test_assoc_set_insert_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {1, 2};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, test_less<T>, ThrowingAllocator<T>> c(alloc);
    c.insert_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <template <class...> class Container, class T>
void test_unord_set_insert_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {1, 2};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, test_hash<T>, test_equal_to<T>, ThrowingAllocator<T>> c(alloc);
    c.insert_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <template <class...> class Container, class K, class V>
void test_assoc_map_insert_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  using ValueType = std::pair<const K, V>;
  ValueType in[]  = {ValueType{K{1}, V{1}}};

  try {
    ThrowingAllocator<ValueType> alloc;

    globalMemCounter.reset();
    Container<K, V, test_less<K>, ThrowingAllocator<ValueType>> c(alloc);
    c.insert_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <template <class...> class Container, class K, class V>
void test_unord_map_insert_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  using ValueType = std::pair<const K, V>;
  ValueType in[]  = {ValueType{K{1}, V{1}}};

  try {
    ThrowingAllocator<ValueType> alloc;

    globalMemCounter.reset();
    Container<K, V, test_hash<K>, test_equal_to<K>, ThrowingAllocator<ValueType>> c(alloc);
    c.insert_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

#endif // SUPPORT_INSERT_RANGE_MAPS_SETS_H
