//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_INSERT_RANGE_SEQUENCE_CONTAINERS_H
#define SUPPORT_INSERT_RANGE_SEQUENCE_CONTAINERS_H

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

template <class Container, class Range>
concept HasInsertRange = requires(Container& c, Range&& range) { c.insert_range(c.end(), range); };

template <template <class...> class Container, class T, class U>
constexpr bool test_constraints_insert_range() {
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

template <class Container, class Range>
concept HasAppendRange = requires(Container& c, Range&& range) { c.append_range(range); };

template <template <class...> class Container, class T, class U>
constexpr bool test_constraints_append_range() {
  // Input range with the same value type.
  static_assert(HasAppendRange<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(HasAppendRange<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!HasAppendRange<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!HasAppendRange<Container<T>, InputRangeNotDerivedFrom>);
  static_assert(!HasAppendRange<Container<T>, InputRangeNotIndirectlyReadable>);
  static_assert(!HasAppendRange<Container<T>, InputRangeNotInputOrOutputIterator>);

  return true;
}

template <class Container, class Range>
concept HasPrependRange = requires(Container& c, Range&& range) { c.prepend_range(range); };

template <template <class...> class Container, class T, class U>
constexpr bool test_constraints_prepend_range() {
  // Input range with the same value type.
  static_assert(HasPrependRange<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(HasPrependRange<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!HasPrependRange<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!HasPrependRange<Container<T>, InputRangeNotDerivedFrom>);
  static_assert(!HasPrependRange<Container<T>, InputRangeNotIndirectlyReadable>);
  static_assert(!HasPrependRange<Container<T>, InputRangeNotInputOrOutputIterator>);

  return true;
}

template <class Container, class Range>
concept HasAssignRange = requires(Container& c, Range&& range) { c.assign_range(range); };

template <template <class...> class Container, class T, class U>
constexpr bool test_constraints_assign_range() {
  // Input range with the same value type.
  static_assert(HasAssignRange<Container<T>, InputRange<T>>);
  // Input range with a convertible value type.
  static_assert(HasAssignRange<Container<T>, InputRange<U>>);
  // Input range with a non-convertible value type.
  static_assert(!HasAssignRange<Container<T>, InputRange<Empty>>);
  // Not an input range.
  static_assert(!HasAssignRange<Container<T>, InputRangeNotDerivedFrom>);
  static_assert(!HasAssignRange<Container<T>, InputRangeNotIndirectlyReadable>);
  static_assert(!HasAssignRange<Container<T>, InputRangeNotInputOrOutputIterator>);

  return true;
}

// Empty container.

template <class T>
TestCase<T> constexpr EmptyContainer_EmptyRange{.initial = {}, .index = 0, .input = {}, .expected = {}};
// Note: specializations for `bool` still use `vector<int>` for inputs. This is to avoid dealing with `vector<bool>` and
// its iterators over proxy types.
template <>
constexpr TestCase<int> EmptyContainer_EmptyRange<bool>{.initial = {}, .index = 0, .input = {}, .expected = {}};

template <class T>
constexpr TestCase<T> EmptyContainer_OneElementRange;
template <>
constexpr TestCase<int> EmptyContainer_OneElementRange<int>{.initial = {}, .index = 0, .input = {5}, .expected = {5}};
template <>
constexpr TestCase<char> EmptyContainer_OneElementRange<char>{.initial = {}, .index = 0, .input = "a", .expected = "a"};
template <>
constexpr TestCase<int> EmptyContainer_OneElementRange<bool>{
    .initial = {}, .index = 0, .input = {true}, .expected = {true}};

template <class T>
constexpr TestCase<T> EmptyContainer_MidRange;
template <>
constexpr TestCase<int> EmptyContainer_MidRange<int>{
    .initial = {}, .index = 0, .input = {5, 3, 1, 7, 9}, .expected = {5, 3, 1, 7, 9}};
template <>
constexpr TestCase<char> EmptyContainer_MidRange<char>{
    .initial = {}, .index = 0, .input = "aeiou", .expected = "aeiou"};
template <>
constexpr TestCase<int> EmptyContainer_MidRange<bool>{
    .initial = {}, .index = 0, .input = {1, 1, 0, 1, 1}, .expected = {1, 1, 0, 1, 1}};

// One-element container.

template <class T>
constexpr TestCase<T> OneElementContainer_Begin_EmptyRange;
template <>
constexpr TestCase<int> OneElementContainer_Begin_EmptyRange<int>{
    .initial = {3}, .index = 0, .input = {}, .expected = {3}};
template <>
constexpr TestCase<char> OneElementContainer_Begin_EmptyRange<char>{
    .initial = "B", .index = 0, .input = {}, .expected = "B"};
template <>
constexpr TestCase<int> OneElementContainer_Begin_EmptyRange<bool>{
    .initial = {0}, .index = 0, .input = {}, .expected = {0}};

template <class T>
constexpr TestCase<T> OneElementContainer_End_EmptyRange;
template <>
constexpr TestCase<int> OneElementContainer_End_EmptyRange<int>{
    .initial = {3}, .index = 1, .input = {}, .expected = {3}};
template <>
constexpr TestCase<char> OneElementContainer_End_EmptyRange<char>{
    .initial = "B", .index = 1, .input = {}, .expected = "B"};
template <>
constexpr TestCase<int> OneElementContainer_End_EmptyRange<bool>{
    .initial = {0}, .index = 1, .input = {}, .expected = {0}};

template <class T>
constexpr TestCase<T> OneElementContainer_Begin_OneElementRange;
template <>
constexpr TestCase<int> OneElementContainer_Begin_OneElementRange<int>{
    .initial = {3}, .index = 0, .input = {-5}, .expected = {-5, 3}};
template <>
constexpr TestCase<char> OneElementContainer_Begin_OneElementRange<char>{
    .initial = "B", .index = 0, .input = "a", .expected = "aB"};
template <>
constexpr TestCase<int> OneElementContainer_Begin_OneElementRange<bool>{
    .initial = {0}, .index = 0, .input = {1}, .expected = {1, 0}};

template <class T>
constexpr TestCase<T> OneElementContainer_End_OneElementRange;
template <>
constexpr TestCase<int> OneElementContainer_End_OneElementRange<int>{
    .initial = {3}, .index = 1, .input = {-5}, .expected = {3, -5}};
template <>
constexpr TestCase<char> OneElementContainer_End_OneElementRange<char>{
    .initial = "B", .index = 1, .input = "a", .expected = "Ba"};
template <>
constexpr TestCase<int> OneElementContainer_End_OneElementRange<bool>{
    .initial = {0}, .index = 1, .input = {1}, .expected = {0, 1}};

template <class T>
constexpr TestCase<T> OneElementContainer_Begin_MidRange;
template <>
constexpr TestCase<int> OneElementContainer_Begin_MidRange<int>{
    .initial = {3}, .index = 0, .input = {-5, -3, -1, -7, -9}, .expected = {-5, -3, -1, -7, -9, 3}};
template <>
constexpr TestCase<char> OneElementContainer_Begin_MidRange<char>{
    .initial = "B", .index = 0, .input = "aeiou", .expected = "aeiouB"};
template <>
constexpr TestCase<int> OneElementContainer_Begin_MidRange<bool>{
    .initial = {0}, .index = 0, .input = {1, 1, 0, 1, 1}, .expected = {1, 1, 0, 1, 1, 0}};

template <class T>
constexpr TestCase<T> OneElementContainer_End_MidRange;
template <>
constexpr TestCase<int> OneElementContainer_End_MidRange<int>{
    .initial = {3}, .index = 1, .input = {-5, -3, -1, -7, -9}, .expected = {3, -5, -3, -1, -7, -9}};
template <>
constexpr TestCase<char> OneElementContainer_End_MidRange<char>{
    .initial = "B", .index = 1, .input = "aeiou", .expected = "Baeiou"};
template <>
constexpr TestCase<int> OneElementContainer_End_MidRange<bool>{
    .initial = {0}, .index = 1, .input = {1, 1, 0, 1, 1}, .expected = {0, 1, 1, 0, 1, 1}};

// Full container / empty range.

template <class T>
constexpr TestCase<T> FullContainer_Begin_EmptyRange;
template <>
constexpr TestCase<int> FullContainer_Begin_EmptyRange<int>{
    .initial = {11, 29, 35, 14, 84}, .index = 0, .input = {}, .expected = {11, 29, 35, 14, 84}};
template <>
constexpr TestCase<char> FullContainer_Begin_EmptyRange<char>{
    .initial = "_BCD_", .index = 0, .input = {}, .expected = "_BCD_"};
template <>
constexpr TestCase<int> FullContainer_Begin_EmptyRange<bool>{
    .initial = {0, 0, 1, 0, 0}, .index = 0, .input = {}, .expected = {0, 0, 1, 0, 0}};

template <class T>
constexpr TestCase<T> FullContainer_Mid_EmptyRange;
template <>
constexpr TestCase<int> FullContainer_Mid_EmptyRange<int>{
    .initial = {11, 29, 35, 14, 84}, .index = 2, .input = {}, .expected = {11, 29, 35, 14, 84}};
template <>
constexpr TestCase<char> FullContainer_Mid_EmptyRange<char>{
    .initial = "_BCD_", .index = 2, .input = {}, .expected = "_BCD_"};
template <>
constexpr TestCase<int> FullContainer_Mid_EmptyRange<bool>{
    .initial = {0, 0, 1, 0, 0}, .index = 2, .input = {}, .expected = {0, 0, 1, 0, 0}};

template <class T>
constexpr TestCase<T> FullContainer_End_EmptyRange;
template <>
constexpr TestCase<int> FullContainer_End_EmptyRange<int>{
    .initial = {11, 29, 35, 14, 84}, .index = 5, .input = {}, .expected = {11, 29, 35, 14, 84}};
template <>
constexpr TestCase<char> FullContainer_End_EmptyRange<char>{
    .initial = "_BCD_", .index = 5, .input = {}, .expected = "_BCD_"};
template <>
constexpr TestCase<int> FullContainer_End_EmptyRange<bool>{
    .initial = {0, 0, 1, 0, 0}, .index = 5, .input = {}, .expected = {0, 0, 1, 0, 0}};

// Full container / one-element range.

template <class T>
constexpr TestCase<T> FullContainer_Begin_OneElementRange;
template <>
constexpr TestCase<int> FullContainer_Begin_OneElementRange<int>{
    .initial = {11, 29, 35, 14, 84}, .index = 0, .input = {-5}, .expected = {-5, 11, 29, 35, 14, 84}};
template <>
constexpr TestCase<char> FullContainer_Begin_OneElementRange<char>{
    .initial = "_BCD_", .index = 0, .input = "a", .expected = "a_BCD_"};
template <>
constexpr TestCase<int> FullContainer_Begin_OneElementRange<bool>{
    .initial = {0, 0, 1, 0, 0}, .index = 0, .input = {1}, .expected = {1, 0, 0, 1, 0, 0}};

template <class T>
constexpr TestCase<T> FullContainer_Mid_OneElementRange;
template <>
constexpr TestCase<int> FullContainer_Mid_OneElementRange<int>{
    .initial = {11, 29, 35, 14, 84}, .index = 2, .input = {-5}, .expected = {11, 29, -5, 35, 14, 84}};
template <>
constexpr TestCase<char> FullContainer_Mid_OneElementRange<char>{
    .initial = "_BCD_", .index = 2, .input = "a", .expected = "_BaCD_"};
template <>
constexpr TestCase<int> FullContainer_Mid_OneElementRange<bool>{
    .initial = {0, 0, 1, 0, 0}, .index = 2, .input = {1}, .expected = {0, 0, 1, 1, 0, 0}};

template <class T>
constexpr TestCase<T> FullContainer_End_OneElementRange;
template <>
constexpr TestCase<int> FullContainer_End_OneElementRange<int>{
    .initial = {11, 29, 35, 14, 84}, .index = 5, .input = {-5}, .expected = {11, 29, 35, 14, 84, -5}};
template <>
constexpr TestCase<char> FullContainer_End_OneElementRange<char>{
    .initial = "_BCD_", .index = 5, .input = "a", .expected = "_BCD_a"};
template <>
constexpr TestCase<int> FullContainer_End_OneElementRange<bool>{
    .initial = {0, 0, 1, 0, 0}, .index = 5, .input = {1}, .expected = {0, 0, 1, 0, 0, 1}};

// Full container / mid-sized range.

template <class T>
constexpr TestCase<T> FullContainer_Begin_MidRange;
template <>
constexpr TestCase<int> FullContainer_Begin_MidRange<int>{
    .initial  = {11, 29, 35, 14, 84},
    .index    = 0,
    .input    = {-5, -3, -1, -7, -9},
    .expected = {-5, -3, -1, -7, -9, 11, 29, 35, 14, 84}};
template <>
constexpr TestCase<char> FullContainer_Begin_MidRange<char>{
    .initial = "_BCD_", .index = 0, .input = "aeiou", .expected = "aeiou_BCD_"};
template <>
constexpr TestCase<int> FullContainer_Begin_MidRange<bool>{
    .initial = {0, 0, 1, 0, 1}, .index = 0, .input = {1, 1, 0, 1, 1}, .expected = {1, 1, 0, 1, 1, 0, 0, 1, 0, 1}};

template <class T>
constexpr TestCase<T> FullContainer_Mid_MidRange;
template <>
constexpr TestCase<int> FullContainer_Mid_MidRange<int>{
    .initial  = {11, 29, 35, 14, 84},
    .index    = 2,
    .input    = {-5, -3, -1, -7, -9},
    .expected = {11, 29, -5, -3, -1, -7, -9, 35, 14, 84}};
template <>
constexpr TestCase<char> FullContainer_Mid_MidRange<char>{
    .initial = "_BCD_", .index = 2, .input = "aeiou", .expected = "_BaeiouCD_"};
template <>
constexpr TestCase<int> FullContainer_Mid_MidRange<bool>{
    .initial = {0, 0, 1, 0, 1}, .index = 2, .input = {1, 1, 0, 1, 1}, .expected = {0, 0, 1, 1, 0, 1, 1, 1, 0, 1}};

template <class T>
constexpr TestCase<T> FullContainer_End_MidRange;
template <>
constexpr TestCase<int> FullContainer_End_MidRange<int>{
    .initial  = {11, 29, 35, 14, 84},
    .index    = 5,
    .input    = {-5, -3, -1, -7, -9},
    .expected = {11, 29, 35, 14, 84, -5, -3, -1, -7, -9}};
template <>
constexpr TestCase<char> FullContainer_End_MidRange<char>{
    .initial = "_BCD_", .index = 5, .input = "aeiou", .expected = "_BCD_aeiou"};
template <>
constexpr TestCase<int> FullContainer_End_MidRange<bool>{
    .initial = {0, 0, 1, 0, 1}, .index = 5, .input = {1, 1, 0, 1, 1}, .expected = {0, 0, 1, 0, 1, 1, 1, 0, 1, 1}};

// Full container / long range.

template <class T>
constexpr TestCase<T> FullContainer_Begin_LongRange;
template <>
constexpr TestCase<int> FullContainer_Begin_LongRange<int>{
    .initial  = {11, 29, 35, 14, 84},
    .index    = 0,
    .input    = {-5, -3, -1, -7, -9, -19, -48, -56, -13, -14, -29, -88, -17, -1, -5, -11, -89, -21, -33, -48},
    .expected = {-5, -3, -1,  -7,  -9,  -19, -48, -56, -13, -14, -29, -88, -17,
                 -1, -5, -11, -89, -21, -33, -48, 11,  29,  35,  14,  84}};
template <>
constexpr TestCase<char> FullContainer_Begin_LongRange<char>{
    .initial = "_BCD_", .index = 0, .input = "aeiouqwxyz5781964203", .expected = "aeiouqwxyz5781964203_BCD_"};
template <>
constexpr TestCase<int> FullContainer_Begin_LongRange<bool>{
    .initial  = {0, 0, 1, 0, 0},
    .index    = 0,
    .input    = {1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0},
    .expected = {1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0}};

template <class T>
constexpr TestCase<T> FullContainer_Mid_LongRange;
template <>
constexpr TestCase<int> FullContainer_Mid_LongRange<int>{
    .initial  = {11, 29, 35, 14, 84},
    .index    = 2,
    .input    = {-5, -3, -1, -7, -9, -19, -48, -56, -13, -14, -29, -88, -17, -1, -5, -11, -89, -21, -33, -48},
    .expected = {11,  29,  -5, -3, -1,  -7,  -9,  -19, -48, -56, -13, -14, -29,
                 -88, -17, -1, -5, -11, -89, -21, -33, -48, 35,  14,  84}};
template <>
constexpr TestCase<char> FullContainer_Mid_LongRange<char>{
    .initial = "_BCD_", .index = 2, .input = "aeiouqwxyz5781964203", .expected = "_Baeiouqwxyz5781964203CD_"};
template <>
constexpr TestCase<int> FullContainer_Mid_LongRange<bool>{
    .initial  = {0, 0, 1, 0, 0},
    .index    = 2,
    .input    = {1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0},
    .expected = {0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0}};

template <class T>
constexpr TestCase<T> FullContainer_End_LongRange;
template <>
constexpr TestCase<int> FullContainer_End_LongRange<int>{
    .initial  = {11, 29, 35, 14, 84},
    .index    = 5,
    .input    = {-5, -3, -1, -7, -9, -19, -48, -56, -13, -14, -29, -88, -17, -1, -5, -11, -89, -21, -33, -48},
    .expected = {11,  29,  35,  14,  84,  -5, -3, -1,  -7,  -9,  -19, -48, -56,
                 -13, -14, -29, -88, -17, -1, -5, -11, -89, -21, -33, -48}};
template <>
constexpr TestCase<char> FullContainer_End_LongRange<char>{
    .initial = "_BCD_", .index = 5, .input = "aeiouqwxyz5781964203", .expected = "_BCD_aeiouqwxyz5781964203"};
template <>
constexpr TestCase<int> FullContainer_End_LongRange<bool>{
    .initial  = {0, 0, 1, 0, 1},
    .index    = 5,
    .input    = {1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0},
    .expected = {0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0}};

// Sequence containers tests.

template <class Container, class Iter, class Sent, class Validate>
constexpr void test_sequence_insert_range(Validate validate) {
  using T      = typename Container::value_type;
  using D      = typename Container::difference_type;
  auto get_pos = [](auto& c, auto& test_case) { return std::ranges::next(c.begin(), static_cast<D>(test_case.index)); };

  auto test = [&](auto& test_case) {
    Container c(test_case.initial.begin(), test_case.initial.end());
    auto in  = wrap_input<Iter, Sent>(test_case.input);
    auto pos = get_pos(c, test_case);

    auto result = c.insert_range(pos, in);
    assert(result == get_pos(c, test_case));
    validate(c);
    return std::ranges::equal(c, test_case.expected);
  };

  { // Empty container.
    // empty_c.insert_range(end, empty_range)
    assert(test(EmptyContainer_EmptyRange<T>));
    // empty_c.insert_range(end, one_element_range)
    assert(test(EmptyContainer_OneElementRange<T>));
    // empty_c.insert_range(end, mid_range)
    assert(test(EmptyContainer_MidRange<T>));
  }

  { // One-element container.
    // one_element_c.insert_range(begin, empty_range)
    assert(test(OneElementContainer_Begin_EmptyRange<T>));
    // one_element_c.insert_range(end, empty_range)
    assert(test(OneElementContainer_End_EmptyRange<T>));
    // one_element_c.insert_range(begin, one_element_range)
    assert(test(OneElementContainer_Begin_OneElementRange<T>));
    // one_element_c.insert_range(end, one_element_range)
    assert(test(OneElementContainer_End_OneElementRange<T>));
    // one_element_c.insert_range(begin, mid_range)
    assert(test(OneElementContainer_Begin_MidRange<T>));
    // one_element_c.insert_range(end, mid_range)
    assert(test(OneElementContainer_End_MidRange<T>));
  }

  { // Full container.
    // full_container.insert_range(begin, empty_range)
    assert(test(FullContainer_Begin_EmptyRange<T>));
    // full_container.insert_range(mid, empty_range)
    assert(test(FullContainer_Mid_EmptyRange<T>));
    // full_container.insert_range(end, empty_range)
    assert(test(FullContainer_End_EmptyRange<T>));
    // full_container.insert_range(begin, one_element_range)
    assert(test(FullContainer_Begin_OneElementRange<T>));
    // full_container.insert_range(end, one_element_range)
    assert(test(FullContainer_Mid_OneElementRange<T>));
    // full_container.insert_range(end, one_element_range)
    assert(test(FullContainer_End_OneElementRange<T>));
    // full_container.insert_range(begin, mid_range)
    assert(test(FullContainer_Begin_MidRange<T>));
    // full_container.insert_range(mid, mid_range)
    assert(test(FullContainer_Mid_MidRange<T>));
    // full_container.insert_range(end, mid_range)
    assert(test(FullContainer_End_MidRange<T>));
    // full_container.insert_range(begin, long_range)
    assert(test(FullContainer_Begin_LongRange<T>));
    // full_container.insert_range(mid, long_range)
    assert(test(FullContainer_Mid_LongRange<T>));
    // full_container.insert_range(end, long_range)
    assert(test(FullContainer_End_LongRange<T>));
  }
}

template <class Container, class Iter, class Sent, class Validate>
constexpr void test_sequence_prepend_range(Validate validate) {
  using T = typename Container::value_type;

  auto test = [&](auto& test_case) {
    Container c(test_case.initial.begin(), test_case.initial.end());
    auto in = wrap_input<Iter, Sent>(test_case.input);

    c.prepend_range(in);
    validate(c);
    return std::ranges::equal(c, test_case.expected);
  };

  { // Empty container.
    // empty_c.prepend_range(empty_range)
    assert(test(EmptyContainer_EmptyRange<T>));
    // empty_c.prepend_range(one_element_range)
    assert(test(EmptyContainer_OneElementRange<T>));
    // empty_c.prepend_range(mid_range)
    assert(test(EmptyContainer_MidRange<T>));
  }

  { // One-element container.
    // one_element_c.prepend_range(empty_range)
    assert(test(OneElementContainer_Begin_EmptyRange<T>));
    // one_element_c.prepend_range(one_element_range)
    assert(test(OneElementContainer_Begin_OneElementRange<T>));
    // one_element_c.prepend_range(mid_range)
    assert(test(OneElementContainer_Begin_MidRange<T>));
  }

  { // Full container.
    // full_container.prepend_range(empty_range)
    assert(test(FullContainer_Begin_EmptyRange<T>));
    // full_container.prepend_range(one_element_range)
    assert(test(FullContainer_Begin_OneElementRange<T>));
    // full_container.prepend_range(mid_range)
    assert(test(FullContainer_Begin_MidRange<T>));
    // full_container.prepend_range(long_range)
    assert(test(FullContainer_Begin_LongRange<T>));
  }
}

template <class Container, class Iter, class Sent, class Validate>
constexpr void test_sequence_append_range(Validate validate) {
  using T = typename Container::value_type;

  auto test = [&](auto& test_case) {
    Container c(test_case.initial.begin(), test_case.initial.end());
    auto in = wrap_input<Iter, Sent>(test_case.input);

    c.append_range(in);
    validate(c);
    return std::ranges::equal(c, test_case.expected);
  };

  { // Empty container.
    // empty_c.append_range(empty_range)
    assert(test(EmptyContainer_EmptyRange<T>));
    // empty_c.append_range(one_element_range)
    assert(test(EmptyContainer_OneElementRange<T>));
    // empty_c.append_range(mid_range)
    assert(test(EmptyContainer_MidRange<T>));
  }

  { // One-element container.
    // one_element_c.append_range(empty_range)
    assert(test(OneElementContainer_End_EmptyRange<T>));
    // one_element_c.append_range(one_element_range)
    assert(test(OneElementContainer_End_OneElementRange<T>));
    // one_element_c.append_range(mid_range)
    assert(test(OneElementContainer_End_MidRange<T>));
  }

  { // Full container.
    // full_container.append_range(empty_range)
    assert(test(FullContainer_End_EmptyRange<T>));
    // full_container.append_range(one_element_range)
    assert(test(FullContainer_End_OneElementRange<T>));
    // full_container.append_range(mid_range)
    assert(test(FullContainer_End_MidRange<T>));
    // full_container.append_range(long_range)
    assert(test(FullContainer_End_LongRange<T>));
  }
}

template <class Container, class Iter, class Sent, class Validate>
constexpr void test_sequence_assign_range(Validate validate) {
  using T = typename Container::value_type;

  auto& initial_empty       = EmptyContainer_EmptyRange<T>.initial;
  auto& initial_one_element = OneElementContainer_Begin_EmptyRange<T>.initial;
  auto& initial_full        = FullContainer_Begin_EmptyRange<T>.initial;
  auto& input_empty         = FullContainer_Begin_EmptyRange<T>.input;
  auto& input_one_element   = FullContainer_Begin_OneElementRange<T>.input;
  auto& input_mid_range     = FullContainer_Begin_MidRange<T>.input;
  auto& input_long_range    = FullContainer_Begin_LongRange<T>.input;

  auto test = [&](auto& initial, auto& input) {
    Container c(initial.begin(), initial.end());
    auto in = wrap_input<Iter, Sent>(input);

    c.assign_range(in);
    validate(c);
    return std::ranges::equal(c, input);
  };

  { // Empty container.
    // empty_container.assign_range(empty_range)
    assert(test(initial_empty, input_empty));
    // empty_container.assign_range(one_element_range)
    assert(test(initial_empty, input_one_element));
    // empty_container.assign_range(mid_range)
    assert(test(initial_empty, input_mid_range));
    // empty_container.assign_range(long_range)
    assert(test(initial_empty, input_long_range));
  }

  { // One-element container.
    // one_element_container.assign_range(empty_range)
    assert(test(initial_one_element, input_empty));
    // one_element_container.assign_range(one_element_range)
    assert(test(initial_one_element, input_one_element));
    // one_element_container.assign_range(mid_range)
    assert(test(initial_one_element, input_mid_range));
    // one_element_container.assign_range(long_range)
    assert(test(initial_one_element, input_long_range));
  }

  { // Full container.
    // full_container.assign_range(empty_range)
    assert(test(initial_full, input_empty));
    // full_container.assign_range(one_element_range)
    assert(test(initial_full, input_one_element));
    // full_container.assign_range(mid_range)
    assert(test(initial_full, input_mid_range));
    // full_container.assign_range(long_range)
    assert(test(initial_full, input_long_range));
  }
}

// Move-only types.

template <template <class...> class Container>
constexpr void test_sequence_insert_range_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  Container<MoveOnly> c;
  c.insert_range(c.end(), in);
}

template <template <class...> class Container>
constexpr void test_sequence_prepend_range_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  Container<MoveOnly> c;
  c.prepend_range(in);
}

template <template <class...> class Container>
constexpr void test_sequence_append_range_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  Container<MoveOnly> c;
  c.append_range(in);
}

template <template <class...> class Container>
constexpr void test_sequence_assign_range_move_only() {
  MoveOnly input[5];
  std::ranges::subrange in(std::move_iterator{input}, std::move_iterator{input + 5});

  Container<MoveOnly> c;
  c.assign_range(in);
}

// Exception safety.

template <template <class...> class Container>
void test_insert_range_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  constexpr int ThrowOn = 3;
  using T               = ThrowingCopy<ThrowOn>;
  test_exception_safety_throwing_copy<ThrowOn, /*Size=*/5>([](T* from, T* to) {
    Container<T> c;
    c.insert_range(c.end(), std::ranges::subrange(from, to));
  });
#endif
}

template <template <class...> class Container, class T>
void test_insert_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {0, 1};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, ThrowingAllocator<T>> c(alloc);
    c.insert_range(c.end(), in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <template <class...> class Container>
void test_prepend_range_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  constexpr int ThrowOn = 3;
  using T               = ThrowingCopy<ThrowOn>;
  test_exception_safety_throwing_copy<ThrowOn, /*Size=*/5>([](T* from, T* to) {
    Container<T> c;
    c.prepend_range(std::ranges::subrange(from, to));
  });
#endif
}

template <template <class...> class Container, class T>
void test_prepend_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {0, 1};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, ThrowingAllocator<T>> c(alloc);
    c.prepend_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <template <class...> class Container>
void test_append_range_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  constexpr int ThrowOn = 3;
  using T               = ThrowingCopy<ThrowOn>;
  test_exception_safety_throwing_copy<ThrowOn, /*Size=*/5>([](T* from, T* to) {
    Container<T> c;
    c.append_range(std::ranges::subrange(from, to));
  });
#endif
}

template <template <class...> class Container, class T>
void test_append_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {0, 1};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, ThrowingAllocator<T>> c(alloc);
    c.append_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

template <template <class...> class Container>
void test_assign_range_exception_safety_throwing_copy() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  constexpr int ThrowOn = 3;
  using T               = ThrowingCopy<ThrowOn>;
  test_exception_safety_throwing_copy<ThrowOn, /*Size=*/5>([](T* from, T* to) {
    Container<T> c;
    c.assign_range(std::ranges::subrange(from, to));
  });
#endif
}

template <template <class...> class Container, class T>
void test_assign_range_exception_safety_throwing_allocator() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
  T in[] = {0, 1};

  try {
    ThrowingAllocator<T> alloc;

    globalMemCounter.reset();
    Container<T, ThrowingAllocator<T>> c(alloc);
    c.assign_range(in);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(globalMemCounter.new_called == globalMemCounter.delete_called);
  }
#endif
}

#endif // SUPPORT_INSERT_RANGE_SEQUENCE_CONTAINERS_H
