//===-- Utility class to test fmod special numbers ------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_FMODTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_FMODTEST_H

#include "src/__support/FPUtil/FEnvImpl.h"
#include "src/errno/libc_errno.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/fenv_macros.h"

#define TEST_SPECIAL(x, y, expected, dom_err, expected_exception)              \
  LIBC_NAMESPACE::fputil::clear_except(FE_ALL_EXCEPT);                         \
  EXPECT_FP_EQ(expected, f(x, y));                                             \
  EXPECT_MATH_ERRNO((dom_err) ? EDOM : 0);                                     \
  EXPECT_FP_EXCEPTION(expected_exception);                                     \
  LIBC_NAMESPACE::fputil::clear_except(FE_ALL_EXCEPT)

#define TEST_REGULAR(x, y, expected) TEST_SPECIAL(x, y, expected, false, 0)

template <typename T>
class FmodTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*FModFunc)(T, T);

  void testSpecialNumbers(FModFunc f) {
    // fmod (+0, y) == +0 for y != 0.
    TEST_SPECIAL(zero, T(3.0), zero, false, 0);
    TEST_SPECIAL(zero, min_denormal, zero, false, 0);
    TEST_SPECIAL(zero, -min_denormal, zero, false, 0);
    TEST_SPECIAL(zero, min_normal, zero, false, 0);
    TEST_SPECIAL(zero, -min_normal, zero, false, 0);
    TEST_SPECIAL(zero, max_normal, zero, false, 0);
    TEST_SPECIAL(zero, -max_normal, zero, false, 0);

    // fmod (-0, y) == -0 for y != 0.
    TEST_SPECIAL(neg_zero, T(3.0), neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, -min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, min_normal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, -min_normal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, max_normal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, -max_normal, neg_zero, false, 0);

    // fmod (+inf, y) == aNaN plus invalid exception.
    TEST_SPECIAL(inf, T(3.0), aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, T(-1.1), aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, min_denormal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, min_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, max_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, inf, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, neg_inf, aNaN, true, FE_INVALID);

    // fmod (-inf, y) == aNaN plus invalid exception.
    TEST_SPECIAL(neg_inf, T(3.0), aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, T(-1.1), aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, min_denormal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, min_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, max_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, inf, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, neg_inf, aNaN, true, FE_INVALID);

    // fmod (x, +0) == aNaN plus invalid exception.
    TEST_SPECIAL(T(3.0), zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(T(-1.1), zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(zero, zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_zero, zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_denormal, zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_normal, zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(max_normal, zero, aNaN, true, FE_INVALID);

    // fmod (x, -0) == aNaN plus invalid exception.
    TEST_SPECIAL(T(3.0), neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(T(-1.1), neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(zero, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_zero, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_denormal, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_normal, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(max_normal, neg_zero, aNaN, true, FE_INVALID);

    // fmod (x, +inf) == x for x not infinite.
    TEST_SPECIAL(zero, inf, zero, false, 0);
    TEST_SPECIAL(neg_zero, inf, neg_zero, false, 0);
    TEST_SPECIAL(min_denormal, inf, min_denormal, false, 0);
    TEST_SPECIAL(min_normal, inf, min_normal, false, 0);
    TEST_SPECIAL(max_normal, inf, max_normal, false, 0);
    TEST_SPECIAL(T(3.0), inf, T(3.0), false, 0);
    // fmod (x, -inf) == x for x not infinite.
    TEST_SPECIAL(zero, neg_inf, zero, false, 0);
    TEST_SPECIAL(neg_zero, neg_inf, neg_zero, false, 0);
    TEST_SPECIAL(min_denormal, neg_inf, min_denormal, false, 0);
    TEST_SPECIAL(min_normal, neg_inf, min_normal, false, 0);
    TEST_SPECIAL(max_normal, neg_inf, max_normal, false, 0);
    TEST_SPECIAL(T(3.0), neg_inf, T(3.0), false, 0);

    TEST_SPECIAL(zero, aNaN, aNaN, false, 0);
    TEST_SPECIAL(zero, neg_aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_zero, aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_zero, neg_aNaN, aNaN, false, 0);
    TEST_SPECIAL(T(1.0), aNaN, aNaN, false, 0);
    TEST_SPECIAL(T(1.0), neg_aNaN, aNaN, false, 0);
    TEST_SPECIAL(inf, aNaN, aNaN, false, 0);
    TEST_SPECIAL(inf, neg_aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_inf, aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_inf, neg_aNaN, aNaN, false, 0);
    TEST_SPECIAL(zero, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(zero, neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_zero, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_zero, neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(T(1.0), sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(T(1.0), neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(inf, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(inf, neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_inf, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_inf, neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(aNaN, zero, aNaN, false, 0);
    TEST_SPECIAL(neg_aNaN, zero, aNaN, false, 0);
    TEST_SPECIAL(aNaN, neg_zero, aNaN, false, 0);
    TEST_SPECIAL(neg_aNaN, neg_zero, aNaN, false, 0);
    TEST_SPECIAL(aNaN, T(1.0), aNaN, false, 0);
    TEST_SPECIAL(neg_aNaN, T(1.0), aNaN, false, 0);
    TEST_SPECIAL(aNaN, inf, aNaN, false, 0);
    TEST_SPECIAL(neg_aNaN, inf, aNaN, false, 0);
    TEST_SPECIAL(aNaN, neg_inf, aNaN, false, 0);
    TEST_SPECIAL(neg_aNaN, neg_inf, aNaN, false, 0);
    TEST_SPECIAL(sNaN, zero, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, zero, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, neg_zero, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, neg_zero, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, T(1.0), aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, T(1.0), aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, neg_inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, neg_inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(aNaN, aNaN, aNaN, false, 0);
    TEST_SPECIAL(aNaN, neg_aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_aNaN, aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_aNaN, neg_aNaN, aNaN, false, 0);
    TEST_SPECIAL(aNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(aNaN, neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_aNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_aNaN, neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, neg_aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, neg_aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, neg_sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_sNaN, neg_sNaN, aNaN, false, FE_INVALID);

    TEST_SPECIAL(T(6.5), T(2.25), T(2.0), false, 0);
    TEST_SPECIAL(T(-6.5), T(2.25), T(-2.0), false, 0);
    TEST_SPECIAL(T(6.5), T(-2.25), T(2.0), false, 0);
    TEST_SPECIAL(T(-6.5), T(-2.25), T(-2.0), false, 0);

    TEST_SPECIAL(max_normal, max_normal, zero, false, 0);
    TEST_SPECIAL(max_normal, -max_normal, zero, false, 0);
    TEST_SPECIAL(max_normal, min_normal, zero, false, 0);
    TEST_SPECIAL(max_normal, -min_normal, zero, false, 0);
    TEST_SPECIAL(max_normal, min_denormal, zero, false, 0);
    TEST_SPECIAL(max_normal, -min_denormal, zero, false, 0);
    TEST_SPECIAL(-max_normal, max_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, -max_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, min_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, -min_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, -min_denormal, neg_zero, false, 0);

    TEST_SPECIAL(min_normal, max_normal, min_normal, false, 0);
    TEST_SPECIAL(min_normal, -max_normal, min_normal, false, 0);
    TEST_SPECIAL(min_normal, min_normal, zero, false, 0);
    TEST_SPECIAL(min_normal, -min_normal, zero, false, 0);
    TEST_SPECIAL(min_normal, min_denormal, zero, false, 0);
    TEST_SPECIAL(min_normal, -min_denormal, zero, false, 0);
    TEST_SPECIAL(-min_normal, max_normal, -min_normal, false, 0);
    TEST_SPECIAL(-min_normal, -max_normal, -min_normal, false, 0);
    TEST_SPECIAL(-min_normal, min_normal, neg_zero, false, 0);
    TEST_SPECIAL(-min_normal, -min_normal, neg_zero, false, 0);
    TEST_SPECIAL(-min_normal, min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(-min_normal, -min_denormal, neg_zero, false, 0);

    TEST_SPECIAL(min_denormal, max_normal, min_denormal, false, 0);
    TEST_SPECIAL(min_denormal, -max_normal, min_denormal, false, 0);
    TEST_SPECIAL(min_denormal, min_normal, min_denormal, false, 0);
    TEST_SPECIAL(min_denormal, -min_normal, min_denormal, false, 0);
    TEST_SPECIAL(min_denormal, min_denormal, zero, false, 0);
    TEST_SPECIAL(min_denormal, -min_denormal, zero, false, 0);
    TEST_SPECIAL(-min_denormal, max_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, -max_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, min_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, -min_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(-min_denormal, -min_denormal, neg_zero, false, 0);
  }

  void testRegularExtreme(FModFunc f) {
    if constexpr (sizeof(T) < sizeof(float))
      return;
    TEST_REGULAR(T(0x1p127), T(0x3p-149), T(0x1p-149));
    TEST_REGULAR(T(0x1p127), T(-0x3p-149), T(0x1p-149));
    TEST_REGULAR(T(0x1p127), T(0x3p-148), T(0x1p-147));
    TEST_REGULAR(T(0x1p127), T(-0x3p-148), T(0x1p-147));
    TEST_REGULAR(T(0x1p127), T(0x3p-126), T(0x1p-125));
    TEST_REGULAR(T(0x1p127), T(-0x3p-126), T(0x1p-125));
    TEST_REGULAR(T(-0x1p127), T(0x3p-149), T(-0x1p-149));
    TEST_REGULAR(T(-0x1p127), T(-0x3p-149), T(-0x1p-149));
    TEST_REGULAR(T(-0x1p127), T(0x3p-148), T(-0x1p-147));
    TEST_REGULAR(T(-0x1p127), T(-0x3p-148), T(-0x1p-147));
    TEST_REGULAR(T(-0x1p127), T(0x3p-126), T(-0x1p-125));
    TEST_REGULAR(T(-0x1p127), T(-0x3p-126), T(-0x1p-125));

    if constexpr (sizeof(T) < sizeof(double))
      return;
    TEST_REGULAR(T(0x1p1023), T(0x3p-1074), T(0x1p-1073));
    TEST_REGULAR(T(0x1p1023), T(-0x3p-1074), T(0x1p-1073));
    TEST_REGULAR(T(0x1p1023), T(0x3p-1073), T(0x1p-1073));
    TEST_REGULAR(T(0x1p1023), T(-0x3p-1073), T(0x1p-1073));
    TEST_REGULAR(T(0x1p1023), T(0x3p-1022), T(0x1p-1021));
    TEST_REGULAR(T(0x1p1023), T(-0x3p-1022), T(0x1p-1021));
    TEST_REGULAR(T(-0x1p1023), T(0x3p-1074), T(-0x1p-1073));
    TEST_REGULAR(T(-0x1p1023), T(-0x3p-1074), T(-0x1p-1073));
    TEST_REGULAR(T(-0x1p1023), T(0x3p-1073), T(-0x1p-1073));
    TEST_REGULAR(T(-0x1p1023), T(-0x3p-1073), T(-0x1p-1073));
    TEST_REGULAR(T(-0x1p1023), T(0x3p-1022), T(-0x1p-1021));
    TEST_REGULAR(T(-0x1p1023), T(-0x3p-1022), T(-0x1p-1021));
  }
};

#define LIST_FMOD_TESTS(T, func)                                               \
  using LlvmLibcFmodTest = FmodTest<T>;                                        \
  TEST_F(LlvmLibcFmodTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcFmodTest, RegularExtreme) { testRegularExtreme(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_FMODTEST_H
