//===-- Utility class to test different flavors of issignaling --*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_SMOKE_ISSIGNALINGTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_SMOKE_ISSIGNALINGTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

template <typename T>
class IsSignalingTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*IsSignalingFunc)(T);

  void testSpecialNumbers(IsSignalingFunc func) {
    EXPECT_EQ(func(aNaN), 0);
    EXPECT_EQ(func(neg_aNaN), 0);
    EXPECT_EQ(func(sNaN), 1);
    EXPECT_EQ(func(neg_sNaN), 1);
    EXPECT_EQ(func(inf), 0);
    EXPECT_EQ(func(neg_inf), 0);
    EXPECT_EQ(func(min_normal), 0);
    EXPECT_EQ(func(max_normal), 0);
    EXPECT_EQ(func(neg_max_normal), 0);
    EXPECT_EQ(func(min_denormal), 0);
    EXPECT_EQ(func(neg_min_denormal), 0);
    EXPECT_EQ(func(max_denormal), 0);
    EXPECT_EQ(func(zero), 0);
    EXPECT_EQ(func(neg_zero), 0);
  }

  void testRoundedNumbers(IsSignalingFunc func) {
    EXPECT_EQ(func(T(1.0)), 0);
    EXPECT_EQ(func(T(-1.0)), 0);
    EXPECT_EQ(func(T(10.0)), 0);
    EXPECT_EQ(func(T(-10.0)), 0);
    EXPECT_EQ(func(T(1234.0)), 0);
    EXPECT_EQ(func(T(-1234.0)), 0);
  }
};

#define LIST_ISSIGNALING_TESTS(T, func)                                        \
  using LlvmLibcIsSignalingTest = IsSignalingTest<T>;                          \
  TEST_F(LlvmLibcIsSignalingTest, SpecialNumbers) {                            \
    testSpecialNumbers(&func);                                                 \
  }                                                                            \
  TEST_F(LlvmLibcIsSignalingTest, RoundedNubmers) { testRoundedNumbers(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_SMOKE_ISSIGNALINGTEST_H
