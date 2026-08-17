//===-- Utility class to test the signbit macro [f|l] -----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_SIGNBIT_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_SIGNBIT_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T> class SignbitTest : public LIBC_NAMESPACE::testing::Test {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*SignbitFunc)(T);

  void testSpecialNumbers(SignbitFunc func) {
    EXPECT_EQ(func(1), 0);
    EXPECT_NE(func(-1), 0);
  }
};

#define LIST_SIGNBIT_TESTS(T, func)                                            \
  using LlvmLibcSignbitTest = SignbitTest<T>;                                  \
  TEST_F(LlvmLibcSignbitTest, SpecialNumbers) {                                \
    auto signbit_func = [](T x) { return func(x); };                           \
    testSpecialNumbers(signbit_func);                                          \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_SIGNBIT_H
