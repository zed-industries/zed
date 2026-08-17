//===-- Utility class to test different flavors of fromfp -------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBC_TEST_SRC_MATH_SMOKE_FROMFPTEST_H
#define LIBC_TEST_SRC_MATH_SMOKE_FROMFPTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

template <typename T>
class FromfpTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*FromfpFunc)(T, int, unsigned int);

  void testSpecialNumbersNonzeroWidth(FromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ(zero, func(zero, rnd, 32U));
      EXPECT_FP_EQ(neg_zero, func(neg_zero, rnd, 32U));

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(inf, rnd, 32U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(neg_inf, rnd, 32U), FE_INVALID);

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(aNaN, rnd, 32U), FE_INVALID);
    }
  }

  void testSpecialNumbersZeroWidth(FromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(zero, rnd, 0U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(neg_zero, rnd, 0U), FE_INVALID);

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(inf, rnd, 0U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(neg_inf, rnd, 0U), FE_INVALID);

      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(aNaN, rnd, 0U), FE_INVALID);
    }
  }

  void testRoundedNumbersWithinRange(FromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ(T(1.0), func(T(1.0), rnd, 2U));
      EXPECT_FP_EQ(T(-1.0), func(T(-1.0), rnd, 1U));
      EXPECT_FP_EQ(T(10.0), func(T(10.0), rnd, 5U));
      EXPECT_FP_EQ(T(-10.0), func(T(-10.0), rnd, 5U));
      EXPECT_FP_EQ(T(1234.0), func(T(1234.0), rnd, 12U));
      EXPECT_FP_EQ(T(-1234.0), func(T(-1234.0), rnd, 12U));
      EXPECT_FP_EQ(T(1234.0), func(T(1234.0), rnd, 65U));
      EXPECT_FP_EQ(T(-1234.0), func(T(-1234.0), rnd, 65U));
    }
  }

  void testRoundedNumbersOutsideRange(FromfpFunc func) {
    for (int rnd : MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN) {
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.0), rnd, 1U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.0), rnd, 4U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.0), rnd, 4U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1234.0), rnd, 11U), FE_INVALID);
      EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1234.0), rnd, 11U), FE_INVALID);
    }
  }

  void testFractionsUpwardWithinRange(FromfpFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(0.5), FP_INT_UPWARD, 2U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.115), FP_INT_UPWARD, 2U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), FP_INT_UPWARD, 2U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.715), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.3), FP_INT_UPWARD, 3U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.3), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), FP_INT_UPWARD, 3U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.5), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), FP_INT_UPWARD, 3U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.75), FP_INT_UPWARD, 1U));
    EXPECT_FP_EQ(T(11.0), func(T(10.32), FP_INT_UPWARD, 5U));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.32), FP_INT_UPWARD, 5U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), FP_INT_UPWARD, 5U));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.65), FP_INT_UPWARD, 5U));
    EXPECT_FP_EQ(T(124.0), func(T(123.38), FP_INT_UPWARD, 8U));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.38), FP_INT_UPWARD, 8U));
    EXPECT_FP_EQ(T(124.0), func(T(123.96), FP_INT_UPWARD, 8U));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.96), FP_INT_UPWARD, 8U));
  }

  void testFractionsUpwardOutsideRange(FromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(0.5), FP_INT_UPWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(0.115), FP_INT_UPWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(0.715), FP_INT_UPWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.3), FP_INT_UPWARD, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.5), FP_INT_UPWARD, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.75), FP_INT_UPWARD, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_UPWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_UPWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_UPWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_UPWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_UPWARD, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_UPWARD, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_UPWARD, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_UPWARD, 7U),
                                FE_INVALID);
  }

  void testFractionsDownwardWithinRange(FromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.5), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.115), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.715), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.715), FP_INT_DOWNWARD, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_DOWNWARD, 2U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.3), FP_INT_DOWNWARD, 2U));
    EXPECT_FP_EQ(T(1.0), func(T(1.5), FP_INT_DOWNWARD, 2U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.5), FP_INT_DOWNWARD, 2U));
    EXPECT_FP_EQ(T(1.0), func(T(1.75), FP_INT_DOWNWARD, 2U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.75), FP_INT_DOWNWARD, 2U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_DOWNWARD, 5U));
    EXPECT_FP_EQ(T(-11.0), func(T(-10.32), FP_INT_DOWNWARD, 5U));
    EXPECT_FP_EQ(T(10.0), func(T(10.65), FP_INT_DOWNWARD, 5U));
    EXPECT_FP_EQ(T(-11.0), func(T(-10.65), FP_INT_DOWNWARD, 5U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_DOWNWARD, 8U));
    EXPECT_FP_EQ(T(-124.0), func(T(-123.38), FP_INT_DOWNWARD, 8U));
    EXPECT_FP_EQ(T(123.0), func(T(123.96), FP_INT_DOWNWARD, 8U));
    EXPECT_FP_EQ(T(-124.0), func(T(-123.96), FP_INT_DOWNWARD, 8U));
  }

  void testFractionsDownwardOutsideRange(FromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.3), FP_INT_DOWNWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.3), FP_INT_DOWNWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.5), FP_INT_DOWNWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.5), FP_INT_DOWNWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.75), FP_INT_DOWNWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.75), FP_INT_DOWNWARD, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_DOWNWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_DOWNWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_DOWNWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_DOWNWARD, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_DOWNWARD, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_DOWNWARD, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_DOWNWARD, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_DOWNWARD, 7U),
                                FE_INVALID);
  }

  void testFractionsTowardZeroWithinRange(FromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.715), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.715), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_TOWARDZERO, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.3), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.5), FP_INT_TOWARDZERO, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.5), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.75), FP_INT_TOWARDZERO, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.75), FP_INT_TOWARDZERO, 1U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_TOWARDZERO, 5U));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.32), FP_INT_TOWARDZERO, 5U));
    EXPECT_FP_EQ(T(10.0), func(T(10.65), FP_INT_TOWARDZERO, 5U));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.65), FP_INT_TOWARDZERO, 5U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_TOWARDZERO, 8U));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.38), FP_INT_TOWARDZERO, 8U));
    EXPECT_FP_EQ(T(123.0), func(T(123.96), FP_INT_TOWARDZERO, 8U));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.96), FP_INT_TOWARDZERO, 8U));
  }

  void testFractionsTowardZeroOutsideRange(FromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.3), FP_INT_TOWARDZERO, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.5), FP_INT_TOWARDZERO, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.75), FP_INT_TOWARDZERO, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_TOWARDZERO, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_TOWARDZERO, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_TOWARDZERO, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_TOWARDZERO, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_TOWARDZERO, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_TOWARDZERO, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_TOWARDZERO, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_TOWARDZERO, 7U),
                                FE_INVALID);
  }

  void testFractionsToNearestFromZeroWithinRange(FromfpFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(0.5), FP_INT_TONEARESTFROMZERO, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.5), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), FP_INT_TONEARESTFROMZERO, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.715), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_TONEARESTFROMZERO, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.3), FP_INT_TONEARESTFROMZERO, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), FP_INT_TONEARESTFROMZERO, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.5), FP_INT_TONEARESTFROMZERO, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), FP_INT_TONEARESTFROMZERO, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.75), FP_INT_TONEARESTFROMZERO, 2U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_TONEARESTFROMZERO, 5U));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.32), FP_INT_TONEARESTFROMZERO, 5U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), FP_INT_TONEARESTFROMZERO, 5U));
    EXPECT_FP_EQ(T(-11.0), func(T(-10.65), FP_INT_TONEARESTFROMZERO, 5U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_TONEARESTFROMZERO, 8U));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.38), FP_INT_TONEARESTFROMZERO, 8U));
    EXPECT_FP_EQ(T(124.0), func(T(123.96), FP_INT_TONEARESTFROMZERO, 8U));
    EXPECT_FP_EQ(T(-124.0), func(T(-123.96), FP_INT_TONEARESTFROMZERO, 8U));
  }

  void testFractionsToNearestFromZeroOutsideRange(FromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(0.5), FP_INT_TONEARESTFROMZERO, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(0.715), FP_INT_TONEARESTFROMZERO, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.3), FP_INT_TONEARESTFROMZERO, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.5), FP_INT_TONEARESTFROMZERO, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.5), FP_INT_TONEARESTFROMZERO, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.75), FP_INT_TONEARESTFROMZERO, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.75), FP_INT_TONEARESTFROMZERO, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.32), FP_INT_TONEARESTFROMZERO, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.32), FP_INT_TONEARESTFROMZERO, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.65), FP_INT_TONEARESTFROMZERO, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.65), FP_INT_TONEARESTFROMZERO, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.38), FP_INT_TONEARESTFROMZERO, 7U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.38), FP_INT_TONEARESTFROMZERO, 7U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.96), FP_INT_TONEARESTFROMZERO, 7U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.96), FP_INT_TONEARESTFROMZERO, 7U), FE_INVALID);
  }

  void testFractionsToNearestWithinRange(FromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.715), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.3), FP_INT_TONEAREST, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.5), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.75), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), FP_INT_TONEAREST, 5U));
    EXPECT_FP_EQ(T(-10.0), func(T(-10.32), FP_INT_TONEAREST, 5U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), FP_INT_TONEAREST, 5U));
    EXPECT_FP_EQ(T(-11.0), func(T(-10.65), FP_INT_TONEAREST, 5U));
    EXPECT_FP_EQ(T(123.0), func(T(123.38), FP_INT_TONEAREST, 8U));
    EXPECT_FP_EQ(T(-123.0), func(T(-123.38), FP_INT_TONEAREST, 8U));
    EXPECT_FP_EQ(T(124.0), func(T(123.96), FP_INT_TONEAREST, 8U));
    EXPECT_FP_EQ(T(-124.0), func(T(-123.96), FP_INT_TONEAREST, 8U));

    EXPECT_FP_EQ(T(2.0), func(T(2.3), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-2.3), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(2.5), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-2.5), FP_INT_TONEAREST, 2U));
    EXPECT_FP_EQ(T(3.0), func(T(2.75), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(-3.0), func(T(-2.75), FP_INT_TONEAREST, 3U));
    EXPECT_FP_EQ(T(5.0), func(T(5.3), FP_INT_TONEAREST, 4U));
    EXPECT_FP_EQ(T(-5.0), func(T(-5.3), FP_INT_TONEAREST, 4U));
    EXPECT_FP_EQ(T(6.0), func(T(5.5), FP_INT_TONEAREST, 4U));
    EXPECT_FP_EQ(T(-6.0), func(T(-5.5), FP_INT_TONEAREST, 4U));
    EXPECT_FP_EQ(T(6.0), func(T(5.75), FP_INT_TONEAREST, 4U));
    EXPECT_FP_EQ(T(-6.0), func(T(-5.75), FP_INT_TONEAREST, 4U));
  }

  void testFractionsToNearestOutsideRange(FromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(0.715), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.3), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.5), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.5), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(1.75), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-1.75), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.32), FP_INT_TONEAREST, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.32), FP_INT_TONEAREST, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(10.65), FP_INT_TONEAREST, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-10.65), FP_INT_TONEAREST, 4U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.38), FP_INT_TONEAREST, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.38), FP_INT_TONEAREST, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(123.96), FP_INT_TONEAREST, 7U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-123.96), FP_INT_TONEAREST, 7U),
                                FE_INVALID);

    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(2.3), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-2.3), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(2.5), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-2.5), FP_INT_TONEAREST, 1U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(2.75), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-2.75), FP_INT_TONEAREST, 2U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(5.3), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-5.3), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(5.5), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-5.5), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(5.75), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(aNaN, func(T(-5.75), FP_INT_TONEAREST, 3U),
                                FE_INVALID);
  }

  void testFractionsToNearestFallbackWithinRange(FromfpFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(0.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(0.0), func(T(0.115), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(-0.0), func(T(-0.115), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(0.715), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-0.715), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(1.0), func(T(1.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(-1.0), func(T(-1.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U));
    EXPECT_FP_EQ(T(2.0), func(T(1.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(1.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-1.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(10.0), func(T(10.32), UNKNOWN_MATH_ROUNDING_DIRECTION, 5U));
    EXPECT_FP_EQ(T(-10.0),
                 func(T(-10.32), UNKNOWN_MATH_ROUNDING_DIRECTION, 5U));
    EXPECT_FP_EQ(T(11.0), func(T(10.65), UNKNOWN_MATH_ROUNDING_DIRECTION, 5U));
    EXPECT_FP_EQ(T(-11.0),
                 func(T(-10.65), UNKNOWN_MATH_ROUNDING_DIRECTION, 5U));
    EXPECT_FP_EQ(T(123.0),
                 func(T(123.38), UNKNOWN_MATH_ROUNDING_DIRECTION, 8U));
    EXPECT_FP_EQ(T(-123.0),
                 func(T(-123.38), UNKNOWN_MATH_ROUNDING_DIRECTION, 8U));
    EXPECT_FP_EQ(T(124.0),
                 func(T(123.96), UNKNOWN_MATH_ROUNDING_DIRECTION, 8U));
    EXPECT_FP_EQ(T(-124.0),
                 func(T(-123.96), UNKNOWN_MATH_ROUNDING_DIRECTION, 8U));

    EXPECT_FP_EQ(T(2.0), func(T(2.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-2.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(2.0), func(T(2.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(-2.0), func(T(-2.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U));
    EXPECT_FP_EQ(T(3.0), func(T(2.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(-3.0), func(T(-2.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U));
    EXPECT_FP_EQ(T(5.0), func(T(5.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
    EXPECT_FP_EQ(T(-5.0), func(T(-5.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
    EXPECT_FP_EQ(T(6.0), func(T(5.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
    EXPECT_FP_EQ(T(-6.0), func(T(-5.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
    EXPECT_FP_EQ(T(6.0), func(T(5.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
    EXPECT_FP_EQ(T(-6.0), func(T(-5.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U));
  }

  void testFractionsToNearestFallbackOutsideRange(FromfpFunc func) {
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(0.715), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(1.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-1.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.32), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.32), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(10.65), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-10.65), UNKNOWN_MATH_ROUNDING_DIRECTION, 4U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.38), UNKNOWN_MATH_ROUNDING_DIRECTION, 7U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.38), UNKNOWN_MATH_ROUNDING_DIRECTION, 7U),
        FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(123.96), UNKNOWN_MATH_ROUNDING_DIRECTION, 7U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-123.96), UNKNOWN_MATH_ROUNDING_DIRECTION, 7U),
        FE_INVALID);

    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(2.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-2.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(2.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-2.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 1U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(2.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-2.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 2U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(5.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-5.3), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(5.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-5.5), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(5.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
    EXPECT_FP_EQ_WITH_EXCEPTION(
        aNaN, func(T(-5.75), UNKNOWN_MATH_ROUNDING_DIRECTION, 3U), FE_INVALID);
  }
};

#define LIST_FROMFP_TESTS(T, func)                                             \
  using LlvmLibcFromfpTest = FromfpTestTemplate<T>;                            \
  TEST_F(LlvmLibcFromfpTest, SpecialNumbersNonzeroWidth) {                     \
    testSpecialNumbersNonzeroWidth(&func);                                     \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, SpecialNumbersZeroWidth) {                        \
    testSpecialNumbersZeroWidth(&func);                                        \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, RoundedNumbersWithinRange) {                      \
    testRoundedNumbersWithinRange(&func);                                      \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, RoundedNumbersOutsideRange) {                     \
    testRoundedNumbersOutsideRange(&func);                                     \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsUpwardWithinRange) {                     \
    testFractionsUpwardWithinRange(&func);                                     \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsUpwardOutsideRange) {                    \
    testFractionsUpwardOutsideRange(&func);                                    \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsDownwardWithinRange) {                   \
    testFractionsDownwardWithinRange(&func);                                   \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsDownwardOutsideRange) {                  \
    testFractionsDownwardOutsideRange(&func);                                  \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsTowardZeroWithinRange) {                 \
    testFractionsTowardZeroWithinRange(&func);                                 \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsTowardZeroOutsideRange) {                \
    testFractionsTowardZeroOutsideRange(&func);                                \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsToNearestFromZeroWithinRange) {          \
    testFractionsToNearestFromZeroWithinRange(&func);                          \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsToNearestFromZeroOutsideRange) {         \
    testFractionsToNearestFromZeroOutsideRange(&func);                         \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsToNearestWithinRange) {                  \
    testFractionsToNearestWithinRange(&func);                                  \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsToNearestOutsideRange) {                 \
    testFractionsToNearestOutsideRange(&func);                                 \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsToNearestFallbackWithinRange) {          \
    testFractionsToNearestFallbackWithinRange(&func);                          \
  }                                                                            \
  TEST_F(LlvmLibcFromfpTest, FractionsToNearestFallbackOutsideRange) {         \
    testFractionsToNearestFallbackOutsideRange(&func);                         \
  }

#endif // LIBC_TEST_SRC_MATH_SMOKE_FROMFPTEST_H
