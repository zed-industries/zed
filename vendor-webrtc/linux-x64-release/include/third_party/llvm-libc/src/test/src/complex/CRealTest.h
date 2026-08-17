//===-- Utility class to test different flavors of creal --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_COMPLEX_CREALTEST_H
#define LLVM_LIBC_TEST_SRC_COMPLEX_CREALTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

template <typename CFPT, typename FPT>
class CRealTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(FPT)

public:
  typedef FPT (*CRealFunc)(CFPT);

  void testSpecialNumbers(CRealFunc func) {
    EXPECT_FP_EQ(func(CFPT(aNaN + 67.123i)), aNaN);
    EXPECT_FP_EQ(func(CFPT(neg_aNaN + 78.319i)), neg_aNaN);
    EXPECT_FP_EQ(func(CFPT(sNaN + 7813.131i)), sNaN);
    EXPECT_FP_EQ(func(CFPT(neg_sNaN + 7824.152i)), neg_sNaN);
    EXPECT_FP_EQ(func(CFPT(inf + 9024.2442i)), inf);
    EXPECT_FP_EQ(func(CFPT(neg_inf + 8923.124i)), neg_inf);
    EXPECT_FP_EQ(func(CFPT(min_normal + 782.124i)), min_normal);
    EXPECT_FP_EQ(func(CFPT(max_normal + 2141.2352i)), max_normal);
    EXPECT_FP_EQ(func(CFPT(neg_max_normal + 341.134i)), neg_max_normal);
    EXPECT_FP_EQ(func(CFPT(min_denormal + 781.142i)), min_denormal);
    EXPECT_FP_EQ(func(CFPT(neg_min_denormal + 781.134i)), neg_min_denormal);
    EXPECT_FP_EQ(func(CFPT(max_denormal + 1241.112i)), max_denormal);
    EXPECT_FP_EQ(func(CFPT(zero + 121.121i)), zero);
    EXPECT_FP_EQ(func(CFPT(0.0 + 0.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(-0.0 + 0.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(0.0 - 0.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(-0.0 - 0.0i)), neg_zero);
    EXPECT_FP_EQ(func(CFPT(0.0)), zero);
    EXPECT_FP_EQ(func(CFPT(-0.0)), neg_zero);
    EXPECT_FP_EQ(func(CFPT(0.0i)), zero);
    EXPECT_FP_EQ(func(CFPT(-0.0i)), neg_zero);
  }

  void testRoundedNumbers(CRealFunc func) {
    EXPECT_FP_EQ(func((CFPT)(4523.1413 + 12413.1414i)), (FPT)(4523.1413));
    EXPECT_FP_EQ(func((CFPT)(-4523.1413 + 12413.1414i)), (FPT)(-4523.1413));
    EXPECT_FP_EQ(func((CFPT)(4523.1413 - 12413.1414i)), (FPT)(4523.1413));
    EXPECT_FP_EQ(func((CFPT)(-4523.1413 - 12413.1414i)), (FPT)(-4523.1413));

    EXPECT_FP_EQ(func((CFPT)(3210.5678 + 9876.5432i)), (FPT)(3210.5678));
    EXPECT_FP_EQ(func((CFPT)(-3210.5678 + 9876.5432i)), (FPT)(-3210.5678));
    EXPECT_FP_EQ(func((CFPT)(3210.5678 - 9876.5432i)), (FPT)(3210.5678));
    EXPECT_FP_EQ(func((CFPT)(-3210.5678 - 9876.5432i)), (FPT)(-3210.5678));

    EXPECT_FP_EQ(func((CFPT)(1234.4321 + 4321.1234i)), (FPT)(1234.4321));
    EXPECT_FP_EQ(func((CFPT)(-1234.4321 + 4321.1234i)), (FPT)(-1234.4321));
    EXPECT_FP_EQ(func((CFPT)(1234.4321 - 4321.1234i)), (FPT)(1234.4321));
    EXPECT_FP_EQ(func((CFPT)(-1234.4321 - 4321.1234i)), (FPT)(-1234.4321));

    EXPECT_FP_EQ(func((CFPT)(6789.1234 + 8765.6789i)), (FPT)(6789.1234));
    EXPECT_FP_EQ(func((CFPT)(-6789.1234 + 8765.6789i)), (FPT)(-6789.1234));
    EXPECT_FP_EQ(func((CFPT)(6789.1234 - 8765.6789i)), (FPT)(6789.1234));
    EXPECT_FP_EQ(func((CFPT)(-6789.1234 - 8765.6789i)), (FPT)(-6789.1234));
  }
};

#define LIST_CREAL_TESTS(U, T, func)                                           \
  using LlvmLibcCRealTest = CRealTest<U, T>;                                   \
  TEST_F(LlvmLibcCRealTest, SpecialNumbers) { testSpecialNumbers(&func); }     \
  TEST_F(LlvmLibcCRealTest, RoundedNumbers) { testRoundedNumbers(&func); }

#endif // LLVM_LIBC_TEST_SRC_COMPLEX_CREALTEST_H
