//===-- Utility class to test different flavors of conj ---------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_COMPLEX_CONJTEST_H
#define LLVM_LIBC_TEST_SRC_COMPLEX_CONJTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

template <typename CFPT, typename FPT>
class ConjTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(FPT)

public:
  typedef CFPT (*ConjFunc)(CFPT);

  void testSpecialNumbers(ConjFunc func) {
    EXPECT_CFP_EQ(func(CFPT(aNaN + 67.123i)), CFPT(aNaN - 67.123i));
    EXPECT_CFP_EQ(func(CFPT(neg_aNaN + 78.319i)), CFPT(neg_aNaN - 78.319i));
    EXPECT_CFP_EQ(func(CFPT(sNaN + 7813.131i)), CFPT(sNaN - 7813.131i));
    EXPECT_CFP_EQ(func(CFPT(neg_sNaN + 7824.152i)), CFPT(neg_sNaN - 7824.152i));
    EXPECT_CFP_EQ(func(CFPT(inf + 9024.2442i)), CFPT(inf - 9024.2442i));
    EXPECT_CFP_EQ(func(CFPT(neg_inf + 8923.124i)), CFPT(neg_inf - 8923.124i));
    EXPECT_CFP_EQ(func(CFPT(min_normal + 782.124i)),
                  CFPT(min_normal - 782.124i));
    EXPECT_CFP_EQ(func(CFPT(max_normal + 2141.2352i)),
                  CFPT(max_normal - 2141.2352i));
    EXPECT_CFP_EQ(func(CFPT(neg_max_normal + 341.134i)),
                  CFPT(neg_max_normal - 341.134i));
    EXPECT_CFP_EQ(func(CFPT(min_denormal + 781.142i)),
                  CFPT(min_denormal - 781.142i));
    EXPECT_CFP_EQ(func(CFPT(neg_min_denormal + 781.134i)),
                  CFPT(neg_min_denormal - 781.134i));
    EXPECT_CFP_EQ(func(CFPT(max_denormal + 1241.112i)),
                  CFPT(max_denormal - 1241.112i));
    EXPECT_CFP_EQ(func(CFPT(zero + 121.121i)), CFPT(zero - 121.121i));
    EXPECT_CFP_EQ(func(CFPT(67.123 + aNaN * 1.0i)), CFPT(67.123 - aNaN * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(78.319 + neg_aNaN * 1.0i)),
                  CFPT(78.319 - neg_aNaN * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(7813.131 + sNaN * 1.0i)),
                  CFPT(7813.131 - sNaN * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(7824.152 + neg_sNaN * 1.0i)),
                  CFPT(7824.152 - neg_sNaN * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(9024.2442 + inf * 1.0i)),
                  CFPT(9024.2442 - inf * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(8923.124 + neg_inf * 1.0i)),
                  CFPT(8923.124 - neg_inf * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(782.124 + min_normal * 1.0i)),
                  CFPT(782.124 - min_normal * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(2141.2352 + max_normal * 1.0i)),
                  CFPT(2141.2352 - max_normal * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(341.134 + neg_max_normal * 1.0i)),
                  CFPT(341.134 - neg_max_normal * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(781.142 + min_denormal * 1.0i)),
                  CFPT(781.142 - min_denormal * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(781.134 + neg_min_denormal * 1.0i)),
                  CFPT(781.134 - neg_min_denormal * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(1241.112 + max_denormal * 1.0i)),
                  CFPT(1241.112 - max_denormal * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(121.121 + zero * 1.0i)),
                  CFPT(121.121 - zero * 1.0i));
    EXPECT_CFP_EQ(func(CFPT(0.0 - 0.0i)), CFPT(0.0 + 0.0i));
    EXPECT_CFP_EQ(func(CFPT(0.0 + 0.0i)), CFPT(0.0 - 0.0i));
    // This test passes because the conjugate of -0.0 - 0.0i is CMPLX(-0.0, 0.0)
    // which cannot be represented as -0.0 + 0.0i because -0.0 + 0.0i is
    // actually CMPLX(-0.0, 0.0) + CMPLX(0.0, 0.0) = 0.0 + 0.0i so to represent
    // CMPLX(-0.0, 0.0), we use -0.0
    EXPECT_CFP_EQ(func(CFPT(-0.0 - 0.0i)), CFPT(-0.0));
    // This test passes because -0.0 + 0.0i is actually
    // CMPLX(-0.0, 0.0) + CMPLX(0.0, 0.0) = CMPLX(-0.0 + 0.0, 0.0) = 0.0 + 0.0i
    EXPECT_CFP_EQ(func(CFPT(-0.0 + 0.0i)), CFPT(0.0 - 0.0i));
    EXPECT_CFP_EQ(func(CFPT(0.0)), CFPT(0.0 - 0.0i));
    EXPECT_CFP_EQ(func(CFPT(-0.0)), CFPT(-0.0 - 0.0i));
    EXPECT_CFP_EQ(func(CFPT(0.0i)), CFPT(0.0 - 0.0i));
    EXPECT_CFP_EQ(func(CFPT(-0.0i)), CFPT(-0.0));
  }

  void testRoundedNumbers(ConjFunc func) {
    EXPECT_CFP_EQ(func((CFPT)(4523.1413 + 12413.1414i)),
                  CFPT(4523.1413 - 12413.1414i));
    EXPECT_CFP_EQ(func((CFPT)(-4523.1413 + 12413.1414i)),
                  CFPT(-4523.1413 - 12413.1414i));
    EXPECT_CFP_EQ(func((CFPT)(4523.1413 - 12413.1414i)),
                  CFPT(4523.1413 + 12413.1414i));
    EXPECT_CFP_EQ(func((CFPT)(-4523.1413 - 12413.1414i)),
                  CFPT(-4523.1413 + 12413.1414i));

    EXPECT_CFP_EQ(func((CFPT)(3210.5678 + 9876.5432i)),
                  CFPT(3210.5678 - 9876.5432i));
    EXPECT_CFP_EQ(func((CFPT)(-3210.5678 + 9876.5432i)),
                  CFPT(-3210.5678 - 9876.5432i));
    EXPECT_CFP_EQ(func((CFPT)(3210.5678 - 9876.5432i)),
                  CFPT(3210.5678 + 9876.5432i));
    EXPECT_CFP_EQ(func((CFPT)(-3210.5678 - 9876.5432i)),
                  CFPT(-3210.5678 + 9876.5432i));

    EXPECT_CFP_EQ(func((CFPT)(1234.4321 + 4321.1234i)),
                  CFPT(1234.4321 - 4321.1234i));
    EXPECT_CFP_EQ(func((CFPT)(-1234.4321 + 4321.1234i)),
                  CFPT(-1234.4321 - 4321.1234i));
    EXPECT_CFP_EQ(func((CFPT)(1234.4321 - 4321.1234i)),
                  CFPT(1234.4321 + 4321.1234i));
    EXPECT_CFP_EQ(func((CFPT)(-1234.4321 - 4321.1234i)),
                  CFPT(-1234.4321 + 4321.1234i));

    EXPECT_CFP_EQ(func((CFPT)(6789.1234 + 8765.6789i)),
                  CFPT(6789.1234 - 8765.6789i));
    EXPECT_CFP_EQ(func((CFPT)(-6789.1234 + 8765.6789i)),
                  CFPT(-6789.1234 - 8765.6789i));
    EXPECT_CFP_EQ(func((CFPT)(6789.1234 - 8765.6789i)),
                  CFPT(6789.1234 + 8765.6789i));
    EXPECT_CFP_EQ(func((CFPT)(-6789.1234 - 8765.6789i)),
                  CFPT(-6789.1234 + 8765.6789i));
  }
};

#define LIST_CONJ_TESTS(U, T, func)                                            \
  using LlvmLibcConjTest = ConjTest<U, T>;                                     \
  TEST_F(LlvmLibcConjTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcConjTest, RoundedNumbers) { testRoundedNumbers(&func); }

#endif // LLVM_LIBC_TEST_SRC_COMPLEX_CONJTEST_H
