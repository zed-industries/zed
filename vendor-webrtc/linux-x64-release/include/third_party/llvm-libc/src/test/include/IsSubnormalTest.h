//===-- Utility class to test the issubnormal macro  ------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_INCLUDE_MATH_ISSUBNORMAL_H
#define LLVM_LIBC_TEST_INCLUDE_MATH_ISSUBNORMAL_H

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "include/llvm-libc-macros/math-function-macros.h"

template <typename T>
class IsSubnormalTest : public LIBC_NAMESPACE::testing::Test {
  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef bool (*IsSubnormalFunc)(T);

  void testSpecialNumbers(IsSubnormalFunc func) {
    EXPECT_FALSE(func(aNaN));
    EXPECT_FALSE(func(neg_aNaN));
    EXPECT_FALSE(func(sNaN));
    EXPECT_FALSE(func(neg_sNaN));
    EXPECT_FALSE(func(inf));
    EXPECT_FALSE(func(neg_inf));
    EXPECT_FALSE(func(min_normal));
    EXPECT_FALSE(func(max_normal));
    EXPECT_FALSE(func(neg_max_normal));
    EXPECT_TRUE(func(min_denormal));
    EXPECT_TRUE(func(neg_min_denormal));
    EXPECT_TRUE(func(max_denormal));
    EXPECT_FALSE(func(zero));
    EXPECT_FALSE(func(neg_zero));
  }
};

#define LIST_ISSUBNORMAL_TESTS(T, func)                                        \
  using LlvmLibcIsSubnormalTest = IsSubnormalTest<T>;                          \
  TEST_F(LlvmLibcIsSubnormalTest, SpecialNumbers) {                            \
    auto issubnormal_func = [](T x) { return func(x); };                       \
    testSpecialNumbers(issubnormal_func);                                      \
  }

#endif // LLVM_LIBC_TEST_INCLUDE_MATH_ISSUBNORMAL_H
