//===-- Utility class to test the isinf macro [f|l] -------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_ISINF_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_ISINF_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T> class IsInfTest : public LIBC_NAMESPACE::testing::Test {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*IsInfFunc)(T);

  void testSpecialNumbers(IsInfFunc func) {
    EXPECT_EQ(func(zero), 0);
    EXPECT_EQ(func(neg_zero), 0);
    EXPECT_EQ(func(inf), 1);
    EXPECT_EQ(func(neg_inf), 1);
  }
};

#define LIST_ISINF_TESTS(T, func)                                              \
  using LlvmLibcIsInfTest = IsInfTest<T>;                                      \
  TEST_F(LlvmLibcIsInfTest, SpecialNumbers) {                                  \
    auto isinf_func = [](T x) { return func(x); };                             \
    testSpecialNumbers(isinf_func);                                            \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_ISINF_H
