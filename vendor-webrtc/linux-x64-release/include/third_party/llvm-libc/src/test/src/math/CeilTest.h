//===-- Utility class to test ceil[f|l] -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_CEILTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_CEILTEST_H

#include "src/__support/CPP/algorithm.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

#include "hdr/math_macros.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename T>
class CeilTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*CeilFunc)(T);

  void testSpecialNumbers(CeilFunc func) {
    EXPECT_FP_EQ(zero, func(zero));
    EXPECT_FP_EQ(neg_zero, func(neg_zero));

    EXPECT_FP_EQ(inf, func(inf));
    EXPECT_FP_EQ(neg_inf, func(neg_inf));

    EXPECT_FP_EQ(aNaN, func(aNaN));
  }

  void testRoundedNumbers(CeilFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(1.0)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.0)));
    EXPECT_FP_EQ(T(10.0), func(T(10.0)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.0)));
    EXPECT_FP_EQ(T(1234.0), func(T(1234.0)));
    EXPECT_FP_EQ(T(-1234.0), func(T(-1234.0)));
  }

  void testFractions(CeilFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(0.5)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5)));
    EXPECT_FP_EQ(T(1.0), func(T(0.115)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115)));
    EXPECT_FP_EQ(T(1.0), func(T(0.715)));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.715)));
    EXPECT_FP_EQ(T(2.0), func(T(1.3)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.3)));
    EXPECT_FP_EQ(T(2.0), func(T(1.5)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.5)));
    EXPECT_FP_EQ(T(2.0), func(T(1.75)));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.75)));
    EXPECT_FP_EQ(T(11.0), func(T(10.32)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.32)));
    EXPECT_FP_EQ(T(11.0), func(T(10.65)));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.65)));
    EXPECT_FP_EQ(T(124.0), func(T(123.38)));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.38)));
    EXPECT_FP_EQ(T(124.0), func(T(123.96)));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.96)));
  }

  void testRange(CeilFunc func) {
    constexpr int COUNT = 100'000;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>(STORAGE_MAX / COUNT), StorageType(1));
    StorageType v = 0;
    for (int i = 0; i <= COUNT; ++i, v += STEP) {
      FPBits xbits(v);
      T x = xbits.get_val();
      if (xbits.is_inf_or_nan())
        continue;

      ASSERT_MPFR_MATCH(mpfr::Operation::Ceil, x, func(x), 0.0);
    }
  }
};

#define LIST_CEIL_TESTS(T, func)                                               \
  using LlvmLibcCeilTest = CeilTest<T>;                                        \
  TEST_F(LlvmLibcCeilTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcCeilTest, RoundedNubmers) { testRoundedNumbers(&func); }      \
  TEST_F(LlvmLibcCeilTest, Fractions) { testFractions(&func); }                \
  TEST_F(LlvmLibcCeilTest, Range) { testRange(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_CEILTEST_H
