//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_FUNCTIONOBJECTS_REFWRAP_HELPER_CONCEPTS_H
#define TEST_STD_FUNCTIONOBJECTS_REFWRAP_HELPER_CONCEPTS_H

#include <concepts>
#include <utility>

// Equality

template <typename T>
concept HasEqualityOperatorWithInt = requires(T t, int i) {
  { t.get() == i } -> std::convertible_to<bool>;
};

// Spaceship

template <class T>
concept BooleanTestableImpl = std::convertible_to<T, bool>;

template <class T>
concept BooleanTestable = BooleanTestableImpl<T> && requires(T&& t) {
  { !std::forward<T>(t) } -> BooleanTestableImpl;
};

template <typename T>
concept HasSpaceshipOperatorWithInt = requires(T t, int i) {
  { t < i } -> BooleanTestable;
  { i < t } -> BooleanTestable;
};

#endif // TEST_STD_FUNCTIONOBJECTS_REFWRAP_HELPER_CONCEPTS_H
