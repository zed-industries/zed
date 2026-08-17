//===-- Utility class to test different flavors of cimag --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_COMPLEX_CIMAGTEST_H
#define LLVM_LIBC_TEST_SRC_COMPLEX_CIMAGTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

template <typename CFPT, typename FPT>
class CImagTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(FPT)

public:
  typedef FPT (*CImagFunc)(CFPT);

  void testSpecialNumbers(CImagFunc func) {
    EXPECT_FP_EQ(func(CFPT(67.123 + aNaN * 1.0i)), aNaN);
    EXPECT_FP_EQ(func(CFPT(78.319 + neg_aNaN * 1.0i)), neg_aNaN);
    EXPECT_FP_EQ(func(CFPT(7813.131 + sNaN * 1.0i)), sNaN);
    EXPECT_FP_EQ(func(CFPT(7824.152 + neg_sNaN * 1.0i)), neg_sNaN);
    EXPECT_FP_EQ(func(CFPT(9024.2442 + inf * 1.0i)), inf);
    EXPECT_FP_EQ(func(CFPT(8923.124 + neg_inf * 1.0i)), neg_inf);
    EXPECT_FP_EQ(func(CFPT(782.124 + min_normal * 1.0i)), min_normal);
    EXPECT_FP_EQ(func(CFPT(2141.2352 + max_normal * 1.0i)), max_normal);
    EXPECT_FP_EQ(func(CFPT(341.134 + neg_max_normal * 1.0i)), neg_max_normal);
    EXPECT_FP_EQ(func(CFPT(781.142 + min_denormal * 1.0i)), min_denormal);
    EXPECT_FP_EQ(func(CFPT(781.134 + neg_min_denormal * 1.0i)),
                 neg_min_denormal);
    EXPECT_FP_EQ(func(CFPT(1241.112 + max_denormal * 1.0i)), max_denormal);
    EXPECT_FP_EQ(func(CFPT(121.121 + zero * 1.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(0.0 + 0.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(-0.0 + 0.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(0.0 - 0.0i)), neg_zero);
    EXPECT_FP_EQ(func(CFPT(-0.0 - 0.0i)), neg_zero);
    EXPECT_FP_EQ(func(CFPT(0.0)), zero);
    EXPECT_FP_EQ(func(CFPT(-0.0)), zero);
    EXPECT_FP_EQ(func(CFPT(0.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(-0.0i)), neg_zero);
  }

  void testRoundedNumbers(CImagFunc func) {
    EXPECT_FP_EQ(func((CFPT)(4523.1413 + 12413.1414i)), (FPT)(12413.1414));
    EXPECT_FP_EQ(func((CFPT)(-4523.1413 + 12413.1414i)), (FPT)(12413.1414));
    EXPECT_FP_EQ(func((CFPT)(4523.1413 - 12413.1414i)), (FPT)(-12413.1414));
    EXPECT_FP_EQ(func((CFPT)(-4523.1413 - 12413.1414i)), (FPT)(-12413.1414));

    EXPECT_FP_EQ(func((CFPT)(3210.5678 + 9876.5432i)), (FPT)(9876.5432));
    EXPECT_FP_EQ(func((CFPT)(-3210.5678 + 9876.5432i)), (FPT)(9876.5432));
    EXPECT_FP_EQ(func((CFPT)(3210.5678 - 9876.5432i)), (FPT)(-9876.5432));
    EXPECT_FP_EQ(func((CFPT)(-3210.5678 - 9876.5432i)), (FPT)(-9876.5432));

    EXPECT_FP_EQ(func((CFPT)(1234.4321 + 4321.1234i)), (FPT)(4321.1234));
    EXPECT_FP_EQ(func((CFPT)(-1234.4321 + 4321.1234i)), (FPT)(4321.1234));
    EXPECT_FP_EQ(func((CFPT)(1234.4321 - 4321.1234i)), (FPT)(-4321.1234));
    EXPECT_FP_EQ(func((CFPT)(-1234.4321 - 4321.1234i)), (FPT)(-4321.1234));

    EXPECT_FP_EQ(func((CFPT)(6789.1234 + 8765.6789i)), (FPT)(8765.6789));
    EXPECT_FP_EQ(func((CFPT)(-6789.1234 + 8765.6789i)), (FPT)(8765.6789));
    EXPECT_FP_EQ(func((CFPT)(6789.1234 - 8765.6789i)), (FPT)(-8765.6789));
    EXPECT_FP_EQ(func((CFPT)(-6789.1234 - 8765.6789i)), (FPT)(-8765.6789));
  }
};

#define LIST_CIMAG_TESTS(U, T, func)                                           \
  using LlvmLibcCImagTest = CImagTest<U, T>;                                   \
  TEST_F(LlvmLibcCImagTest, SpecialNumbers) { testSpecialNumbers(&func); }     \
  TEST_F(LlvmLibcCImagTest, RoundedNumbers) { testRoundedNumbers(&func); }

#endif // LLVM_LIBC_TEST_SRC_COMPLEX_CIMAGTEST_H
