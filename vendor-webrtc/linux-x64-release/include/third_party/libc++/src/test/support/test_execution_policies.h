//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TEST_EXECUTION_POLICIES
#define TEST_SUPPORT_TEST_EXECUTION_POLICIES

#include <cstdlib>
#include <exception>
#include <execution>
#include <type_traits>
#include <utility>

#include "test_macros.h"

#define EXECUTION_POLICY_SFINAE_TEST(function)                                                                         \
  template <class, class...>                                                                                           \
  struct sfinae_test_##function##_impl : std::true_type {};                                                            \
                                                                                                                       \
  template <class... Args>                                                                                             \
  struct sfinae_test_##function##_impl<std::void_t<decltype(std::function(std::declval<Args>()...))>, Args...>         \
      : std::false_type {};                                                                                            \
                                                                                                                       \
  template <class... Args>                                                                                             \
  constexpr bool sfinae_test_##function = sfinae_test_##function##_impl<void, Args...>::value;

template <class Functor>
bool test_execution_policies(Functor func) {
  func(std::execution::seq);
#if TEST_STD_VER >= 20
  func(std::execution::unseq);
#endif
  func(std::execution::par);
  func(std::execution::par_unseq);

  return true;
}

template <template <class Iter> class TestClass>
struct TestIteratorWithPolicies {
  template <class Iter>
  void operator()() {
    test_execution_policies(TestClass<Iter>{});
  }
};

struct Bool {
  bool b_;
  Bool() = default;
  Bool(bool b) : b_(b) {}

  operator bool&() {
    return b_;
  }
};

#endif // TEST_SUPPORT_TEST_EXECUTION_POLICIES
