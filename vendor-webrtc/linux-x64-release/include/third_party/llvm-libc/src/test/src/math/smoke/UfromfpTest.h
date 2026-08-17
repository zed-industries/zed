//===-- Utility class to test different flavors of ufromfp ------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBC_TEST_SRC_MATH_SMOKE_UFROMFPTEST_H
#define LIBC_TEST_SRC_MATH_SMOKE_UFROMFPTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

template <typename T>
class UfromfpTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*UfromfpFunc)(T, int, unsigned int);

  void testSpecialNumbersNonzeroWidth(UfromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ(zero, func(zero, rnd, 32U));
      EXPECT_FP_EQ(neg_zero, func(neg_zero, rnd, 32U));

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(inf, rnd, 32U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(neg_inf, rnd, 32U), FE_INVALID);

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(aNaN, rnd, 32U), FE_INVALID);
    }
  }

  void testSpecialNumbersZeroWidth(UfromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(zero, rnd, 0U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(neg_zero, rnd, 0U), FE_INVALID);

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(inf, rnd, 0U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(neg_inf, rnd, 0U), FE_INVALID);

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(aNaN, rnd, 0U), FE_INVALID);
    }
  }

  void testRoundedNumbersWithinRange(UfromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ(T(1.0), func(T(1.0), rnd, 1U));
      EXPECT_FP_EQ(T(10.0), func(T(10.0), rnd, 4U));
      EXPECT_FP_EQ(T(1234.0), func(T(1234.0), rnd, 11U));
      EXPECT_FP_EQ(T(1234.0), func(T(1234.0), rnd, 64U));
    }
  }

  void testRoundedNumbersOutsideRange(UfromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.0), rnd, 32U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.0), rnd, 3U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.0), rnd, 32U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1234.0), rnd, 10U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1234.0), rnd, 32U), FE_INVALID);
    }
  }

  void testFractionsUpwardWithinRange(UfromfpFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(0.5), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.115), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.715), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.3), FP_INT_UPWARD, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), FP_INT_UPWARD, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), FP_INT_UPWARD, 2U));
    EXPECT_FP_EQ(T(11.0), func(T(10.32), FP_INT_UPWARD, 4U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), FP_INT_UPWARD, 4U));
    EXPECT_FP_EQ(T(124.0), func(T(123.38), FP_INT_UPWARD, 7U));
    EXPECT_FP_EQ(T(124.0), func(T(123.96), FP_INT_UPWARD, 7U));
  }

  void testFractionsUpwardOutsideRange(UfromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.3), FP_INT_UPWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.3), FP_INT_UPWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.5), FP_INT_UPWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.5), FP_INT_UPWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.75), FP_INT_UPWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.75), FP_INT_UPWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_UPWARD, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_UPWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_UPWARD, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_UPWARD, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_UPWARD, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_UPWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_UPWARD, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_UPWARD, 32U),
                                FE_INVALID);
  }

  void testFractionsDownwardWithinRange(UfromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.715), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.5), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.75), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_DOWNWARD, 4U));
    EXPECT_FP_EQ(T(10.0), func(T(10.65), FP_INT_DOWNWARD, 4U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_DOWNWARD, 7U));
    EXPECT_FP_EQ(T(123.0), func(T(123.96), FP_INT_DOWNWARD, 7U));
  }

  void testFractionsDownwardOutsideRange(UfromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-0.5), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-0.115), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-0.715), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.3), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.5), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.75), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_DOWNWARD, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_DOWNWARD, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_DOWNWARD, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_DOWNWARD, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_DOWNWARD, 32U),
                                FE_INVALID);
  }

  void testFractionsTowardZeroWithinRange(UfromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.715), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.715), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.5), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.75), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_TOWARDZERO, 4U));
    EXPECT_FP_EQ(T(10.0), func(T(10.65), FP_INT_TOWARDZERO, 4U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_TOWARDZERO, 7U));
    EXPECT_FP_EQ(T(123.0), func(T(123.96), FP_INT_TOWARDZERO, 7U));
  }

  void testFractionsTowardZeroOutsideRange(UfromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.3), FP_INT_TOWARDZERO, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.5), FP_INT_TOWARDZERO, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.75), FP_INT_TOWARDZERO, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_TOWARDZERO, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_TOWARDZERO, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_TOWARDZERO, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_TOWARDZERO, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_TOWARDZERO, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_TOWARDZERO, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_TOWARDZERO, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_TOWARDZERO, 32U),
                                FE_INVALID);
  }

  void testFractionsToNearestFromZeroWithinRange(UfromfpFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(0.5), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), FP_INT_TONEARESTFROMZERO, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), FP_INT_TONEARESTFROMZERO, 2U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_TONEARESTFROMZERO, 4U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), FP_INT_TONEARESTFROMZERO, 4U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_TONEARESTFROMZERO, 7U));
    EXPECT_FP_EQ(T(124.0), func(T(123.96), FP_INT_TONEARESTFROMZERO, 7U));
  }

  void testFractionsToNearestFromZeroOutsideRange(UfromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-0.5), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-0.715), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.3), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.5), FP_INT_TONEARESTFROMZERO, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.5), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.75), FP_INT_TONEARESTFROMZERO, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.75), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.32), FP_INT_TONEARESTFROMZERO, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.32), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.65), FP_INT_TONEARESTFROMZERO, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.65), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.38), FP_INT_TONEARESTFROMZERO, 6U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.38), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.96), FP_INT_TONEARESTFROMZERO, 6U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.96), FP_INT_TONEARESTFROMZERO, 32U), FE_INVALID);
  }

  void testFractionsToNearestWithinRange(UfromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_TONEAREST, 4U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), FP_INT_TONEAREST, 4U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_TONEAREST, 7U));
    EXPECT_FP_EQ(T(124.0), func(T(123.96), FP_INT_TONEAREST, 7U));

    EXPECT_FP_EQ(T(2.0), func(T(2.3), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(2.5), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(3.0), func(T(2.75), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(5.0), func(T(5.3), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(6.0), func(T(5.5), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(6.0), func(T(5.75), FP_INT_TONEAREST, 3U));
  }

  void testFractionsToNearestOutsideRange(UfromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-0.715), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.3), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.5), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.5), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.75), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.75), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_TONEAREST, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_TONEAREST, 6U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_TONEAREST, 32U),
                                FE_INVALID);

    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(2.3), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-2.3), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(2.5), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-2.5), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(2.75), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-2.75), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(5.3), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-5.3), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(5.5), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-5.5), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(5.75), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-5.75), FP_INT_TONEAREST, 32U),
                                FE_INVALID);
  }

  void testFractionsToNearestFallbackWithinRange(UfromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
    EXPECT_FP_EQ(T(123.0),
                 func(T(123.38), UNKNOWN_MATH_ROUNDING_DIRECTION, 7U));
    EXPECT_FP_EQ(T(124.0),
                 func(T(123.96), UNKNOWN_MATH_ROUNDING_DIRECTION, 7U));

    EXPECT_FP_EQ(T(2.0), func(T(2.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(2.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(3.0), func(T(2.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(5.0), func(T(5.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(6.0), func(T(5.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(6.0), func(T(5.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
  }

  void testFractionsToNearestFallbackOutsideRange(UfromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-0.715), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U),
        FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.32), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.32), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U),
        FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.65), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.65), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U),
        FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.38), UNKNOWN_MATH_ROUNDING_DIRECTION, 6U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.38), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U),
        FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.96), UNKNOWN_MATH_ROUNDING_DIRECTION, 6U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.96), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U),
        FE_INVALID);

    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(2.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-2.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(2.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-2.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(2.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-2.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(5.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-5.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(5.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-5.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(5.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-5.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 32U), FE_INVALID);
  }
};

#define LIST_UFROMFP_TESTS(T, func)                                            \
  using LlvmLibcUfromfpTest = UfromfpTestTemplate<T>;                          \
  TEST_F(LlvmLibcUfromfpTest, SpecialNumbersNonzeroWidth) {                    \
    testSpecialNumbersNonzeroWidth(&func);                                     \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, SpecialNumbersZeroWidth) {                       \
    testSpecialNumbersZeroWidth(&func);                                        \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, RoundedNumbersWithinRange) {                     \
    testRoundedNumbersWithinRange(&func);                                      \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, RoundedNumbersOutsideRange) {                    \
    testRoundedNumbersOutsideRange(&func);                                     \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsUpwardWithinRange) {                    \
    testFractionsUpwardWithinRange(&func);                                     \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsUpwardOutsideRange) {                   \
    testFractionsUpwardOutsideRange(&func);                                    \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsDownwardWithinRange) {                  \
    testFractionsDownwardWithinRange(&func);                                   \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsDownwardOutsideRange) {                 \
    testFractionsDownwardOutsideRange(&func);                                  \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsTowardZeroWithinRange) {                \
    testFractionsTowardZeroWithinRange(&func);                                 \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsTowardZeroOutsideRange) {               \
    testFractionsTowardZeroOutsideRange(&func);                                \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsToNearestFromZeroWithinRange) {         \
    testFractionsToNearestFromZeroWithinRange(&func);                          \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsToNearestFromZeroOutsideRange) {        \
    testFractionsToNearestFromZeroOutsideRange(&func);                         \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsToNearestWithinRange) {                 \
    testFractionsToNearestWithinRange(&func);                                  \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsToNearestOutsideRange) {                \
    testFractionsToNearestOutsideRange(&func);                                 \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsToNearestFallbackWithinRange) {         \
    testFractionsToNearestFallbackWithinRange(&func);                          \
  }                                                                            \
  TEST_F(LlvmLibcUfromfpTest, FractionsToNearestFallbackOutsideRange) {        \
    testFractionsToNearestFallbackOutsideRange(&func);                         \
  }

#endif // LIBC_TEST_SRC_MATH_SMOKE_UFROMFPTEST_H
