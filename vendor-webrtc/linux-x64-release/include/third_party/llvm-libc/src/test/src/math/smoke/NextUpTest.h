//===-- Utility class to test different flavors of nextup -------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_NEXTUPTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_NEXTUPTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

template <typename T>
class NextUpTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*NextUpFunc)(T);

  void testNaN(NextUpFunc func) { ASSERT_FP_EQ(func(aNaN), aNaN); }

  void testBoundaries(NextUpFunc func) {
    ASSERT_FP_EQ(neg_zero, func(neg_min_denormal));

    ASSERT_FP_EQ(min_denormal, func(zero));
    ASSERT_FP_EQ(min_denormal, func(neg_zero));

    ASSERT_FP_EQ(max_normal, func(max_normal));
    ASSERT_FP_EQ(inf, func(inf));

    ASSERT_FP_EQ(neg_max_normal, func(neg_inf));
  }
};

#define LIST_NEXTUP_TESTS(T, func)                                             \
  using LlvmLibcNextUpTest = NextUpTestTemplate<T>;                            \
  TEST_F(LlvmLibcNextUpTest, TestNaN) { testNaN(&func); }                      \
  TEST_F(LlvmLibcNextUpTest, TestBoundaries) { testBoundaries(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_NEXTUPTEST_H
