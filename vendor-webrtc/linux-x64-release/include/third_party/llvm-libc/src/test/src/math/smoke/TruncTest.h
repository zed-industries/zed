//===-- Utility class to test trunc[f|l] ------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_SMOKE_TRUNCTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_SMOKE_TRUNCTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

template <typename T>
class TruncTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*TruncFunc)(T);

  void testSpecialNumbers(TruncFunc func) {
    EXPECT_FP_EQ(zero, func(zero));
    EXPECT_FP_EQ(neg_zero, func(neg_zero));

    EXPECT_FP_EQ(inf, func(inf));
    EXPECT_FP_EQ(neg_inf, func(neg_inf));

    EXPECT_FP_EQ(aNaN, func(aNaN));
  }

  void testRoundedNumbers(TruncFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(1.0)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.0)));
    EXPECT_FP_EQ(T(10.0), func(T(10.0)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.0)));
    EXPECT_FP_EQ(T(1234.0), func(T(1234.0)));
    EXPECT_FP_EQ(T(-1234.0), func(T(-1234.0)));
  }

  void testFractions(TruncFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5)));
    EXPECT_FP_EQ(T(0.0), func(T(0.115)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115)));
    EXPECT_FP_EQ(T(0.0), func(T(0.715)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.715)));
    EXPECT_FP_EQ(T(1.0), func(T(1.3)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.3)));
    EXPECT_FP_EQ(T(1.0), func(T(1.5)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.5)));
    EXPECT_FP_EQ(T(1.0), func(T(1.75)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.75)));
    EXPECT_FP_EQ(T(10.0), func(T(10.32)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.32)));
    EXPECT_FP_EQ(T(10.0), func(T(10.65)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.65)));
    EXPECT_FP_EQ(T(123.0), func(T(123.38)));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.38)));
    EXPECT_FP_EQ(T(123.0), func(T(123.96)));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.96)));
  }
};

#define LIST_TRUNC_TESTS(T, func)                                              \
  using LlvmLibcTruncTest = TruncTest<T>;                                      \
  TEST_F(LlvmLibcTruncTest, SpecialNumbers) { testSpecialNumbers(&func); }     \
  TEST_F(LlvmLibcTruncTest, RoundedNubmers) { testRoundedNumbers(&func); }     \
  TEST_F(LlvmLibcTruncTest, Fractions) { testFractions(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_SMOKE_TRUNCTEST_H
