//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//  A set of routines for testing the comparison operators of a type
//
//      FooOrder<expected-ordering>  All seven comparison operators, requires C++20 or newer.
//      FooComparison                All six pre-C++20 comparison operators
//      FooEquality                  Equality operators operator== and operator!=
//
//      AssertXAreNoexcept           static_asserts that the operations are all noexcept.
//      AssertXReturnBool            static_asserts that the operations return bool.
//      AssertOrderReturn            static_asserts that the pre-C++20 comparison operations
//                                   return bool and operator<=> returns the proper type.
//      AssertXConvertibleToBool     static_asserts that the operations return something convertible to bool.
//      testXValues                  returns the result of the comparison of all operations.
//
//      AssertOrderConvertibleToBool doesn't exist yet. It will be implemented when needed.

#ifndef TEST_COMPARISONS_H
#define TEST_COMPARISONS_H

#include <cassert>
#include <compare>
#include <concepts>
#include <limits>
#include <type_traits>
#include <utility>

#include "test_macros.h"

// Test the consistency of the six basic comparison operators for values that are ordered or unordered.
template <class T, class U = T>
TEST_NODISCARD TEST_CONSTEXPR_CXX14 bool
testComparisonsComplete(const T& t1, const U& t2, bool isEqual, bool isLess, bool isGreater) {
    assert(((isEqual ? 1 : 0) + (isLess ? 1 : 0) + (isGreater ? 1 : 0) <= 1) &&
           "at most one of isEqual, isLess, and isGreater can be true");
    if (isEqual) {
        if (!(t1 == t2)) return false;
        if (!(t2 == t1)) return false;
        if ( (t1 != t2)) return false;
        if ( (t2 != t1)) return false;
        if ( (t1  < t2)) return false;
        if ( (t2  < t1)) return false;
        if (!(t1 <= t2)) return false;
        if (!(t2 <= t1)) return false;
        if ( (t1  > t2)) return false;
        if ( (t2  > t1)) return false;
        if (!(t1 >= t2)) return false;
        if (!(t2 >= t1)) return false;
    } else if (isLess) {
        if ( (t1 == t2)) return false;
        if ( (t2 == t1)) return false;
        if (!(t1 != t2)) return false;
        if (!(t2 != t1)) return false;
        if (!(t1  < t2)) return false;
        if ( (t2  < t1)) return false;
        if (!(t1 <= t2)) return false;
        if ( (t2 <= t1)) return false;
        if ( (t1  > t2)) return false;
        if (!(t2  > t1)) return false;
        if ( (t1 >= t2)) return false;
        if (!(t2 >= t1)) return false;
    } else if (isGreater) {
        if ( (t1 == t2)) return false;
        if ( (t2 == t1)) return false;
        if (!(t1 != t2)) return false;
        if (!(t2 != t1)) return false;
        if ( (t1  < t2)) return false;
        if (!(t2  < t1)) return false;
        if ( (t1 <= t2)) return false;
        if (!(t2 <= t1)) return false;
        if (!(t1  > t2)) return false;
        if ( (t2  > t1)) return false;
        if (!(t1 >= t2)) return false;
        if ( (t2 >= t1)) return false;
    } else { // unordered
        if ( (t1 == t2)) return false;
        if ( (t2 == t1)) return false;
        if (!(t1 != t2)) return false;
        if (!(t2 != t1)) return false;
        if ( (t1  < t2)) return false;
        if ( (t2  < t1)) return false;
        if ( (t1 <= t2)) return false;
        if ( (t2 <= t1)) return false;
        if ( (t1  > t2)) return false;
        if ( (t2  > t1)) return false;
        if ( (t1 >= t2)) return false;
        if ( (t2 >= t1)) return false;
    }

    return true;
}

// Test the six basic comparison operators for ordered values.
template <class T, class U = T>
TEST_NODISCARD TEST_CONSTEXPR_CXX14 bool testComparisons(const T& t1, const U& t2, bool isEqual, bool isLess) {
    assert(!(isEqual && isLess) && "isEqual and isLess cannot be both true");
    bool isGreater = !isEqual && !isLess;
    return testComparisonsComplete(t1, t2, isEqual, isLess, isGreater);
}

//  Easy call when you can init from something already comparable.
template <class T, class Param>
TEST_NODISCARD TEST_CONSTEXPR_CXX14 bool testComparisonsValues(Param val1, Param val2)
{
    const bool isEqual   = val1 == val2;
    const bool isLess    = val1 <  val2;
    const bool isGreater = val1  > val2;

    return testComparisonsComplete(T(val1), T(val2), isEqual, isLess, isGreater);
}

template <class T, class U = T>
TEST_CONSTEXPR_CXX14 void AssertComparisonsAreNoexcept() {
    ASSERT_NOEXCEPT(std::declval<const T&>() == std::declval<const U&>());
    ASSERT_NOEXCEPT(std::declval<const T&>() != std::declval<const U&>());
    ASSERT_NOEXCEPT(std::declval<const T&>() <  std::declval<const U&>());
    ASSERT_NOEXCEPT(std::declval<const T&>() <= std::declval<const U&>());
    ASSERT_NOEXCEPT(std::declval<const T&>() >  std::declval<const U&>());
    ASSERT_NOEXCEPT(std::declval<const T&>() >= std::declval<const U&>());
}

template <class T, class U = T>
TEST_CONSTEXPR_CXX14 void AssertComparisonsReturnBool() {
    ASSERT_SAME_TYPE(decltype(std::declval<const T&>() == std::declval<const U&>()), bool);
    ASSERT_SAME_TYPE(decltype(std::declval<const T&>() != std::declval<const U&>()), bool);
    ASSERT_SAME_TYPE(decltype(std::declval<const T&>() <  std::declval<const U&>()), bool);
    ASSERT_SAME_TYPE(decltype(std::declval<const T&>() <= std::declval<const U&>()), bool);
    ASSERT_SAME_TYPE(decltype(std::declval<const T&>() >  std::declval<const U&>()), bool);
    ASSERT_SAME_TYPE(decltype(std::declval<const T&>() >= std::declval<const U&>()), bool);
}

template <class T, class U = T>
void AssertComparisonsConvertibleToBool()
{
    static_assert((std::is_convertible<decltype(std::declval<const T&>() == std::declval<const U&>()), bool>::value), "");
    static_assert((std::is_convertible<decltype(std::declval<const T&>() != std::declval<const U&>()), bool>::value), "");
    static_assert((std::is_convertible<decltype(std::declval<const T&>() <  std::declval<const U&>()), bool>::value), "");
    static_assert((std::is_convertible<decltype(std::declval<const T&>() <= std::declval<const U&>()), bool>::value), "");
    static_assert((std::is_convertible<decltype(std::declval<const T&>() >  std::declval<const U&>()), bool>::value), "");
    static_assert((std::is_convertible<decltype(std::declval<const T&>() >= std::declval<const U&>()), bool>::value), "");
}

#if TEST_STD_VER > 17
template <class T, class U = T>
constexpr void AssertOrderAreNoexcept() {
    AssertComparisonsAreNoexcept<T, U>();
    ASSERT_NOEXCEPT(std::declval<const T&>() <=> std::declval<const U&>());
}

template <class Order, class T, class U = T>
constexpr void AssertOrderReturn() {
    AssertComparisonsReturnBool<T, U>();
    ASSERT_SAME_TYPE(decltype(std::declval<const T&>() <=> std::declval<const U&>()), Order);
}

template <class Order, class T, class U = T>
TEST_NODISCARD constexpr bool testOrder(const T& t1, const U& t2, Order order) {
    bool equal   = order == Order::equivalent;
    bool less    = order == Order::less;
    bool greater = order == Order::greater;

    return (t1 <=> t2 == order) && testComparisonsComplete(t1, t2, equal, less, greater);
}

template <class T, class Param>
TEST_NODISCARD constexpr bool testOrderValues(Param val1, Param val2) {
  return testOrder(T(val1), T(val2), val1 <=> val2);
}

#endif

//  Test all two comparison operations for sanity
template <class T, class U = T>
TEST_NODISCARD TEST_CONSTEXPR_CXX14 bool testEquality(const T& t1, const U& t2, bool isEqual)
{
    if (isEqual)
        {
        if (!(t1 == t2)) return false;
        if (!(t2 == t1)) return false;
        if ( (t1 != t2)) return false;
        if ( (t2 != t1)) return false;
        }
    else /* not equal */
        {
        if ( (t1 == t2)) return false;
        if ( (t2 == t1)) return false;
        if (!(t1 != t2)) return false;
        if (!(t2 != t1)) return false;
        }

    return true;
}

//  Easy call when you can init from something already comparable.
template <class T, class Param>
TEST_NODISCARD TEST_CONSTEXPR_CXX14 bool testEqualityValues(Param val1, Param val2)
{
    const bool isEqual = val1 == val2;

    return testEquality(T(val1), T(val2), isEqual);
}

template <class T, class U = T>
void AssertEqualityAreNoexcept()
{
    ASSERT_NOEXCEPT(std::declval<const T&>() == std::declval<const U&>());
    ASSERT_NOEXCEPT(std::declval<const T&>() != std::declval<const U&>());
}

template <class T, class U = T>
TEST_CONSTEXPR_CXX14 void AssertEqualityReturnBool() {
  ASSERT_SAME_TYPE(decltype(std::declval<const T&>() == std::declval<const U&>()), bool);
  ASSERT_SAME_TYPE(decltype(std::declval<const T&>() != std::declval<const U&>()), bool);
}

template <class T, class U = T>
void AssertEqualityConvertibleToBool()
{
    static_assert((std::is_convertible<decltype(std::declval<const T&>() == std::declval<const U&>()), bool>::value), "");
    static_assert((std::is_convertible<decltype(std::declval<const T&>() != std::declval<const U&>()), bool>::value), "");
}

struct LessAndEqComp {
  int value;

  TEST_CONSTEXPR_CXX14 LessAndEqComp(int v) : value(v) {}

  friend TEST_CONSTEXPR_CXX14 bool operator<(const LessAndEqComp& lhs, const LessAndEqComp& rhs) {
    return lhs.value < rhs.value;
  }

  friend TEST_CONSTEXPR_CXX14 bool operator==(const LessAndEqComp& lhs, const LessAndEqComp& rhs) {
    return lhs.value == rhs.value;
  }
};

#if TEST_STD_VER >= 20

struct StrongOrder {
  int value;
  constexpr StrongOrder(int v) : value(v) {}
  friend std::strong_ordering operator<=>(StrongOrder, StrongOrder) = default;
};

struct WeakOrder {
  int value;
  constexpr WeakOrder(int v) : value(v) {}
  friend std::weak_ordering operator<=>(WeakOrder, WeakOrder) = default;
};

struct PartialOrder {
  int value;
  constexpr PartialOrder(int v) : value(v) {}
  friend constexpr std::partial_ordering operator<=>(PartialOrder lhs, PartialOrder rhs) {
    if (lhs.value == std::numeric_limits<int>::min() || rhs.value == std::numeric_limits<int>::min())
      return std::partial_ordering::unordered;
    if (lhs.value == std::numeric_limits<int>::max() || rhs.value == std::numeric_limits<int>::max())
      return std::partial_ordering::unordered;
    return lhs.value <=> rhs.value;
  }
  friend constexpr bool operator==(PartialOrder lhs, PartialOrder rhs) {
    return (lhs <=> rhs) == std::partial_ordering::equivalent;
  }
};

#endif

#endif // TEST_COMPARISONS_H
