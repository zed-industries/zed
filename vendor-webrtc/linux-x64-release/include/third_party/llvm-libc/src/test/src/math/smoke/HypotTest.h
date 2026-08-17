//===-- Utility class to test different flavors of hypot ------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_HYPOTTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_HYPOTTEST_H

#include "src/__support/macros/properties/architectures.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

template <typename T>
struct HypotTestTemplate : public LIBC_NAMESPACE::testing::Test {
  using Func = T (*)(T, T);

  DECLARE_SPECIAL_CONSTANTS(T)

  void test_special_numbers(Func func) {
    constexpr int N = 4;
    // Pythagorean triples.
    constexpr T PYT[N][3] = {{3, 4, 5}, {5, 12, 13}, {8, 15, 17}, {7, 24, 25}};

#ifndef LIBC_TARGET_ARCH_IS_NVPTX
    // TODO: Investigate why sNaN tests are failing on nVidia.
    // https://github.com/llvm/llvm-project/issues/99706.
    EXPECT_FP_EQ(func(inf, sNaN), aNaN);
    EXPECT_FP_EQ(func(sNaN, neg_inf), aNaN);
#endif // !LIBC_TARGET_ARCH_IS_NVPTX

    EXPECT_FP_EQ(func(inf, aNaN), inf);
    EXPECT_FP_EQ(func(aNaN, neg_inf), inf);
    EXPECT_FP_EQ(func(aNaN, aNaN), aNaN);
    EXPECT_FP_EQ(func(aNaN, zero), aNaN);
    EXPECT_FP_EQ(func(neg_zero, aNaN), aNaN);

    for (int i = 0; i < N; ++i) {
      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(PYT[i][0], PYT[i][1]));
      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(-PYT[i][0], PYT[i][1]));
      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(PYT[i][0], -PYT[i][1]));
      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(-PYT[i][0], -PYT[i][1]));

      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(PYT[i][1], PYT[i][0]));
      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(-PYT[i][1], PYT[i][0]));
      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(PYT[i][1], -PYT[i][0]));
      EXPECT_FP_EQ_ALL_ROUNDING(PYT[i][2], func(-PYT[i][1], -PYT[i][0]));
    }
  }
};

#endif // LLVM_LIBC_TEST_SRC_MATH_HYPOTTEST_H
