//===-- Utility class to test the isfinite macro [f|l] ----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_ISFINITE_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_ISFINITE_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T>
class IsFiniteTest : public LIBC_NAMESPACE::testing::Test {
  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*IsFiniteFunc)(T);

  void testSpecialNumbers(IsFiniteFunc func) {
    EXPECT_EQ(func(inf), 0);
    EXPECT_EQ(func(neg_inf), 0);
    EXPECT_EQ(func(zero), 1);
    EXPECT_EQ(func(neg_zero), 1);
  }
};

#define LIST_ISFINITE_TESTS(T, func)                                           \
  using LlvmLibcIsFiniteTest = IsFiniteTest<T>;                                \
  TEST_F(LlvmLibcIsFiniteTest, SpecialNumbers) {                               \
    auto isfinite_func = [](T x) { return func(x); };                          \
    testSpecialNumbers(isfinite_func);                                         \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_ISFINITE_H
