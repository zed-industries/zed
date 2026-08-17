//===-- Utility class to test the fpclassify macro  -------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_FPCLASSIFY_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_FPCLASSIFY_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T>
class FpClassifyTest : public LIBC_NAMESPACE::testing::Test {
  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*FpClassifyFunc)(T);

  void testSpecialNumbers(FpClassifyFunc func) {
    EXPECT_EQ(func(aNaN), FP_NAN);
    EXPECT_EQ(func(neg_aNaN), FP_NAN);
    EXPECT_EQ(func(sNaN), FP_NAN);
    EXPECT_EQ(func(neg_sNaN), FP_NAN);
    EXPECT_EQ(func(inf), FP_INFINITE);
    EXPECT_EQ(func(neg_inf), FP_INFINITE);
    EXPECT_EQ(func(min_normal), FP_NORMAL);
    EXPECT_EQ(func(max_normal), FP_NORMAL);
    EXPECT_EQ(func(neg_max_normal), FP_NORMAL);
    EXPECT_EQ(func(min_denormal), FP_SUBNORMAL);
    EXPECT_EQ(func(neg_min_denormal), FP_SUBNORMAL);
    EXPECT_EQ(func(max_denormal), FP_SUBNORMAL);
    EXPECT_EQ(func(zero), FP_ZERO);
    EXPECT_EQ(func(neg_zero), FP_ZERO);
  }
};

#define LIST_FPCLASSIFY_TESTS(T, func)                                         \
  using LlvmLibcFpClassifyTest = FpClassifyTest<T>;                            \
  TEST_F(LlvmLibcFpClassifyTest, SpecialNumbers) {                             \
    auto fpclassify_func = [](T x) { return func(x); };                        \
    testSpecialNumbers(fpclassify_func);                                       \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_FPCLASSIFY_H
