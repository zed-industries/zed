//===-- Utility class to test the isnan macro [f|l] -------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license nanormation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_ISNAN_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_ISNAN_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T> class IsNanTest : public LIBC_NAMESPACE::testing::Test {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*IsNanFunc)(T);

  void testSpecialNumbers(IsNanFunc func) {
    EXPECT_EQ(func(zero), 0);
    EXPECT_EQ(func(neg_zero), 0);
    EXPECT_EQ(func(aNaN), 1);
    EXPECT_EQ(func(sNaN), 1);
  }
};

#define LIST_ISNAN_TESTS(T, func)                                              \
  using LlvmLibcIsNanTest = IsNanTest<T>;                                      \
  TEST_F(LlvmLibcIsNanTest, SpecialNumbers) {                                  \
    auto isnan_func = [](T x) { return func(x); };                             \
    testSpecialNumbers(isnan_func);                                            \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_ISNAN_H
