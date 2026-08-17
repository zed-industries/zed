//===-- Utility class to test the iszero macro  -----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_ISZERO_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_ISZERO_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T> class IsZeroTest : public LIBC_NAMESPACE::testing::Test {
  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef bool (*IsZeroFunc)(T);

  void testSpecialNumbers(IsZeroFunc func) {
    EXPECT_FALSE(func(inf));
    EXPECT_FALSE(func(neg_inf));
    EXPECT_TRUE(func(zero));
    EXPECT_TRUE(func(neg_zero));
  }
};

#define LIST_ISZERO_TESTS(T, func)                                             \
  using LlvmLibcIsZeroTest = IsZeroTest<T>;                                    \
  TEST_F(LlvmLibcIsZeroTest, SpecialNumbers) {                                 \
    auto iszero_func = [](T x) { return func(x); };                            \
    testSpecialNumbers(iszero_func);                                           \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_ISZERO_H
