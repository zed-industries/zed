//===-- Utility class to test fminimum_mag[f|l] -----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_SMOKE_FMINIMUM_MAGTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_SMOKE_FMINIMUM_MAGTEST_H

#include "src/__support/CPP/algorithm.h"
#include "src/__support/FPUtil/BasicOperations.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

template <typename T>
class FMinimumMagTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*FMinimumMagFunc)(T, T);

  void testNaN(FMinimumMagFunc func) {
    EXPECT_FP_EQ(aNaN, func(aNaN, inf));
    EXPECT_FP_EQ(aNaN, func(neg_inf, aNaN));
    EXPECT_FP_EQ(aNaN, func(aNaN, zero));
    EXPECT_FP_EQ(aNaN, func(neg_zero, aNaN));
    EXPECT_FP_EQ(aNaN, func(aNaN, T(-1.2345)));
    EXPECT_FP_EQ(aNaN, func(T(1.2345), aNaN));
    EXPECT_FP_EQ(aNaN, func(aNaN, aNaN));
  }

  void testInfArg(FMinimumMagFunc func) {
    EXPECT_FP_EQ(neg_inf, func(neg_inf, inf));
    EXPECT_FP_EQ(zero, func(inf, zero));
    EXPECT_FP_EQ(neg_zero, func(neg_zero, inf));
    EXPECT_FP_EQ(T(1.2345), func(inf, T(1.2345)));
    EXPECT_FP_EQ(T(-1.2345), func(T(-1.2345), inf));
  }

  void testNegInfArg(FMinimumMagFunc func) {
    EXPECT_FP_EQ(neg_inf, func(inf, neg_inf));
    EXPECT_FP_EQ(zero, func(neg_inf, zero));
    EXPECT_FP_EQ(neg_zero, func(neg_zero, neg_inf));
    EXPECT_FP_EQ(T(-1.2345), func(neg_inf, T(-1.2345)));
    EXPECT_FP_EQ(T(1.2345), func(T(1.2345), neg_inf));
  }

  void testBothZero(FMinimumMagFunc func) {
    EXPECT_FP_EQ(zero, func(zero, zero));
    EXPECT_FP_EQ(neg_zero, func(neg_zero, zero));
    EXPECT_FP_EQ(neg_zero, func(zero, neg_zero));
    EXPECT_FP_EQ(neg_zero, func(neg_zero, neg_zero));
  }

  void testRange(FMinimumMagFunc func) {
    constexpr int COUNT = 100'001;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>(STORAGE_MAX / COUNT), StorageType(1));
    StorageType v = 0, w = STORAGE_MAX;
    for (int i = 0; i <= COUNT; ++i, v += STEP, w -= STEP) {
      FPBits xbits(v), ybits(w);
      if (xbits.is_inf_or_nan())
        continue;
      if (ybits.is_inf_or_nan())
        continue;
      T x = xbits.get_val();
      T y = ybits.get_val();
      if ((x == 0) && (y == 0))
        continue;

      if (LIBC_NAMESPACE::fputil::abs(x) < LIBC_NAMESPACE::fputil::abs(y))
        EXPECT_FP_EQ(x, func(x, y));
      else
        EXPECT_FP_EQ(y, func(x, y));
    }
  }
};

#define LIST_FMINIMUM_MAG_TESTS(T, func)                                       \
  using LlvmLibcFMinimumMagTest = FMinimumMagTest<T>;                          \
  TEST_F(LlvmLibcFMinimumMagTest, NaN) { testNaN(&func); }                     \
  TEST_F(LlvmLibcFMinimumMagTest, InfArg) { testInfArg(&func); }               \
  TEST_F(LlvmLibcFMinimumMagTest, NegInfArg) { testNegInfArg(&func); }         \
  TEST_F(LlvmLibcFMinimumMagTest, BothZero) { testBothZero(&func); }           \
  TEST_F(LlvmLibcFMinimumMagTest, Range) { testRange(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_SMOKE_FMINIMUM_MAGTEST_H
