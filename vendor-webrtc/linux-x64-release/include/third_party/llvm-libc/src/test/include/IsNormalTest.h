//===-- Utility class to test the isnormal macro  ---------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_ISNORMAL_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_ISNORMAL_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T>
class IsNormalTest : public LIBC_NAMESPACE::testing::Test {
  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*IsNormalFunc)(T);

  void testSpecialNumbers(IsNormalFunc func) {
    EXPECT_EQ(func(aNaN), 0);
    EXPECT_EQ(func(neg_aNaN), 0);
    EXPECT_EQ(func(sNaN), 0);
    EXPECT_EQ(func(neg_sNaN), 0);
    EXPECT_EQ(func(inf), 0);
    EXPECT_EQ(func(neg_inf), 0);
    EXPECT_EQ(func(min_normal), 1);
    EXPECT_EQ(func(max_normal), 1);
    EXPECT_EQ(func(neg_max_normal), 1);
    EXPECT_EQ(func(min_denormal), 0);
    EXPECT_EQ(func(neg_min_denormal), 0);
    EXPECT_EQ(func(max_denormal), 0);
    EXPECT_EQ(func(zero), 0);
    EXPECT_EQ(func(neg_zero), 0);
  }
};

#define LIST_ISNORMAL_TESTS(T, func)                                           \
  using LlvmLibcIsNormalTest = IsNormalTest<T>;                                \
  TEST_F(LlvmLibcIsNormalTest, SpecialNumbers) {                               \
    auto isnormal_func = [](T x) { return func(x); };                          \
    testSpecialNumbers(isnormal_func);                                         \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_ISNORMAL_H
