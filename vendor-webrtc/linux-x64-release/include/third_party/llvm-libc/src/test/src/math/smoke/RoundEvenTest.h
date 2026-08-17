//===-- Utility class to test roundeven[f|l] --------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_SMOKE_ROUNDEVENTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_SMOKE_ROUNDEVENTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

template <typename T>
class RoundEvenTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*RoundEvenFunc)(T);

  void testSpecialNumbers(RoundEvenFunc func) {
    EXPECT_FP_EQ(zero, func(zero));
    EXPECT_FP_EQ(neg_zero, func(neg_zero));

    EXPECT_FP_EQ(inf, func(inf));
    EXPECT_FP_EQ(neg_inf, func(neg_inf));

    EXPECT_FP_EQ(aNaN, func(aNaN));
  }

  void testRoundedNumbers(RoundEvenFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(1.0)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.0)));
    EXPECT_FP_EQ(T(10.0), func(T(10.0)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.0)));
    EXPECT_FP_EQ(T(1234.0), func(T(1234.0)));
    EXPECT_FP_EQ(T(-1234.0), func(T(-1234.0)));
  }

  void testFractions(RoundEvenFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5)));
    EXPECT_FP_EQ(T(0.0), func(T(0.115)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115)));
    EXPECT_FP_EQ(T(1.0), func(T(0.715)));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.715)));
    EXPECT_FP_EQ(T(2.0), func(T(1.5)));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.5)));
    EXPECT_FP_EQ(T(2.0), func(T(1.75)));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.75)));
    EXPECT_FP_EQ(T(10.0), func(T(10.50)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.50)));
    EXPECT_FP_EQ(T(11.0), func(T(10.65)));
    EXPECT_FP_EQ(T(-11.0), func(T(-10.65)));
    EXPECT_FP_EQ(T(124.0), func(T(124.50)));
    EXPECT_FP_EQ(T(-124.0), func(T(-124.50)));
    EXPECT_FP_EQ(T(126.0), func(T(125.50)));
    EXPECT_FP_EQ(T(-126.0), func(T(-125.50)));
  }
};

#define LIST_ROUNDEVEN_TESTS(T, func)                                          \
  using LlvmLibcRoundEvenTest = RoundEvenTest<T>;                              \
  TEST_F(LlvmLibcRoundEvenTest, SpecialNumbers) { testSpecialNumbers(&func); } \
  TEST_F(LlvmLibcRoundEvenTest, RoundedNubmers) { testRoundedNumbers(&func); } \
  TEST_F(LlvmLibcRoundEvenTest, Fractions) { testFractions(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_SMOKE_ROUNDEVENTEST_H
