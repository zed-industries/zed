//===-- Utility class to test different flavors of iscanonical --*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_SMOKE_ISCANONICALTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_SMOKE_ISCANONICALTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

template <typename T>
class IsCanonicalTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*IsCanonicalFunc)(T);

  void testSpecialNumbers(IsCanonicalFunc func) {
    EXPECT_EQ(func(aNaN), 1);
    EXPECT_EQ(func(neg_aNaN), 1);
    EXPECT_EQ(func(sNaN), 0);
    EXPECT_EQ(func(neg_sNaN), 0);
    EXPECT_EQ(func(inf), 1);
    EXPECT_EQ(func(neg_inf), 1);
    EXPECT_EQ(func(min_normal), 1);
    EXPECT_EQ(func(max_normal), 1);
    EXPECT_EQ(func(neg_max_normal), 1);
    EXPECT_EQ(func(min_denormal), 1);
    EXPECT_EQ(func(neg_min_denormal), 1);
    EXPECT_EQ(func(max_denormal), 1);
    EXPECT_EQ(func(zero), 1);
    EXPECT_EQ(func(neg_zero), 1);
  }

  void testRoundedNumbers(IsCanonicalFunc func) {
    EXPECT_EQ(func(T(1.0)), 1);
    EXPECT_EQ(func(T(-1.0)), 1);
    EXPECT_EQ(func(T(10.0)), 1);
    EXPECT_EQ(func(T(-10.0)), 1);
    EXPECT_EQ(func(T(1234.0)), 1);
    EXPECT_EQ(func(T(-1234.0)), 1);
  }
};

#define LIST_ISCANONICAL_TESTS(T, func)                                        \
  using LlvmLibcIsCanonicalTest = IsCanonicalTest<T>;                          \
  TEST_F(LlvmLibcIsCanonicalTest, SpecialNumbers) {                            \
    testSpecialNumbers(&func);                                                 \
  }                                                                            \
  TEST_F(LlvmLibcIsCanonicalTest, RoundedNubmers) { testRoundedNumbers(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_SMOKE_ISCANONICALTEST_H
