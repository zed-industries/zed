//===-- Utility class to test fmod special numbers ------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_FMODTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_FMODTEST_H

#include "src/__support/FPUtil/BasicOperations.h"
#include "src/__support/FPUtil/NearestIntegerOperations.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

#define TEST_SPECIAL(x, y, expected, dom_err, expected_exception)              \
  LIBC_NAMESPACE::fputil::clear_except(FE_ALL_EXCEPT);                         \
  EXPECT_FP_EQ(expected, f(x, y));                                             \
  EXPECT_MATH_ERRNO((dom_err) ? EDOM : 0);                                     \
  EXPECT_FP_EXCEPTION(expected_exception)

#define TEST_REGULAR(x, y, expected) TEST_SPECIAL(x, y, expected, false, 0)

template <typename T>
class FmodTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*FModFunc)(T, T);

  void testSpecialNumbers(FModFunc f) {
    // fmod (+0, y) == +0 for y != 0.
    TEST_SPECIAL(0.0, 3.0, 0.0, false, 0);
    TEST_SPECIAL(0.0, min_denormal, 0.0, false, 0);
    TEST_SPECIAL(0.0, -min_denormal, 0.0, false, 0);
    TEST_SPECIAL(0.0, min_normal, 0.0, false, 0);
    TEST_SPECIAL(0.0, -min_normal, 0.0, false, 0);
    TEST_SPECIAL(0.0, max_normal, 0.0, false, 0);
    TEST_SPECIAL(0.0, -max_normal, 0.0, false, 0);

    // fmod (-0, y) == -0 for y != 0.
    TEST_SPECIAL(neg_zero, 3.0, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, -min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, min_normal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, -min_normal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, max_normal, neg_zero, false, 0);
    TEST_SPECIAL(neg_zero, -max_normal, neg_zero, false, 0);

    // fmod (+inf, y) == aNaN plus invalid exception.
    TEST_SPECIAL(inf, 3.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, static_cast<float>(-1.1L), aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, min_denormal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, min_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, max_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, inf, aNaN, true, FE_INVALID);
    TEST_SPECIAL(inf, neg_inf, aNaN, true, FE_INVALID);

    // fmod (-inf, y) == aNaN plus invalid exception.
    TEST_SPECIAL(neg_inf, 3.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, static_cast<float>(-1.1L), aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, min_denormal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, min_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, max_normal, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, inf, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_inf, neg_inf, aNaN, true, FE_INVALID);

    // fmod (x, +0) == aNaN plus invalid exception.
    TEST_SPECIAL(3.0, 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(static_cast<float>(-1.1L), 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(0.0, 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_zero, 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_denormal, 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_normal, 0.0, aNaN, true, FE_INVALID);
    TEST_SPECIAL(max_normal, 0.0, aNaN, true, FE_INVALID);

    // fmod (x, -0) == aNaN plus invalid exception.
    TEST_SPECIAL(3.0, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(static_cast<float>(-1.1L), neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(0.0, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(neg_zero, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_denormal, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(min_normal, neg_zero, aNaN, true, FE_INVALID);
    TEST_SPECIAL(max_normal, neg_zero, aNaN, true, FE_INVALID);

    // fmod (x, +inf) == x for x not infinite.
    TEST_SPECIAL(0.0, inf, 0.0, false, 0);
    TEST_SPECIAL(neg_zero, inf, neg_zero, false, 0);
    TEST_SPECIAL(min_denormal, inf, min_denormal, false, 0);
    TEST_SPECIAL(min_normal, inf, min_normal, false, 0);
    TEST_SPECIAL(max_normal, inf, max_normal, false, 0);
    TEST_SPECIAL(3.0, inf, 3.0, false, 0);
    // fmod (x, -inf) == x for x not infinite.
    TEST_SPECIAL(0.0, neg_inf, 0.0, false, 0);
    TEST_SPECIAL(neg_zero, neg_inf, neg_zero, false, 0);
    TEST_SPECIAL(min_denormal, neg_inf, min_denormal, false, 0);
    TEST_SPECIAL(min_normal, neg_inf, min_normal, false, 0);
    TEST_SPECIAL(max_normal, neg_inf, max_normal, false, 0);
    TEST_SPECIAL(3.0, neg_inf, 3.0, false, 0);

    TEST_SPECIAL(0.0, aNaN, aNaN, false, 0);
    TEST_SPECIAL(0.0, -aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_zero, aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_zero, -aNaN, aNaN, false, 0);
    TEST_SPECIAL(1.0, aNaN, aNaN, false, 0);
    TEST_SPECIAL(1.0, -aNaN, aNaN, false, 0);
    TEST_SPECIAL(inf, aNaN, aNaN, false, 0);
    TEST_SPECIAL(inf, -aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_inf, aNaN, aNaN, false, 0);
    TEST_SPECIAL(neg_inf, -aNaN, aNaN, false, 0);
    TEST_SPECIAL(0.0, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(0.0, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_zero, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_zero, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(1.0, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(1.0, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(inf, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(inf, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_inf, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(neg_inf, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(aNaN, 0.0, aNaN, false, 0);
    TEST_SPECIAL(-aNaN, 0.0, aNaN, false, 0);
    TEST_SPECIAL(aNaN, neg_zero, aNaN, false, 0);
    TEST_SPECIAL(-aNaN, neg_zero, aNaN, false, 0);
    TEST_SPECIAL(aNaN, 1.0, aNaN, false, 0);
    TEST_SPECIAL(-aNaN, 1.0, aNaN, false, 0);
    TEST_SPECIAL(aNaN, inf, aNaN, false, 0);
    TEST_SPECIAL(-aNaN, inf, aNaN, false, 0);
    TEST_SPECIAL(aNaN, neg_inf, aNaN, false, 0);
    TEST_SPECIAL(-aNaN, neg_inf, aNaN, false, 0);
    TEST_SPECIAL(sNaN, 0.0, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, 0.0, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, neg_zero, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, neg_zero, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, 1.0, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, 1.0, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, neg_inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, neg_inf, aNaN, false, FE_INVALID);
    TEST_SPECIAL(aNaN, aNaN, aNaN, false, 0);
    TEST_SPECIAL(aNaN, -aNaN, aNaN, false, 0);
    TEST_SPECIAL(-aNaN, aNaN, aNaN, false, 0);
    TEST_SPECIAL(-aNaN, -aNaN, aNaN, false, 0);
    TEST_SPECIAL(aNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(aNaN, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-aNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-aNaN, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, -aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, -aNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(sNaN, -sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, sNaN, aNaN, false, FE_INVALID);
    TEST_SPECIAL(-sNaN, -sNaN, aNaN, false, FE_INVALID);

    TEST_SPECIAL(6.5, 2.25L, 2.0L, false, 0);
    TEST_SPECIAL(-6.5, 2.25L, -2.0L, false, 0);
    TEST_SPECIAL(6.5, -2.25L, 2.0L, false, 0);
    TEST_SPECIAL(-6.5, -2.25L, -2.0L, false, 0);

    TEST_SPECIAL(max_normal, max_normal, 0.0, false, 0);
    TEST_SPECIAL(max_normal, -max_normal, 0.0, false, 0);
    TEST_SPECIAL(max_normal, min_normal, 0.0, false, 0);
    TEST_SPECIAL(max_normal, -min_normal, 0.0, false, 0);
    TEST_SPECIAL(max_normal, min_denormal, 0.0, false, 0);
    TEST_SPECIAL(max_normal, -min_denormal, 0.0, false, 0);
    TEST_SPECIAL(-max_normal, max_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, -max_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, min_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, -min_normal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(-max_normal, -min_denormal, neg_zero, false, 0);

    TEST_SPECIAL(min_normal, max_normal, min_normal, false, 0);
    TEST_SPECIAL(min_normal, -max_normal, min_normal, false, 0);
    TEST_SPECIAL(min_normal, min_normal, 0.0, false, 0);
    TEST_SPECIAL(min_normal, -min_normal, 0.0, false, 0);
    TEST_SPECIAL(min_normal, min_denormal, 0.0, false, 0);
    TEST_SPECIAL(min_normal, -min_denormal, 0.0, false, 0);
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
    TEST_SPECIAL(min_denormal, min_denormal, 0.0, false, 0);
    TEST_SPECIAL(min_denormal, -min_denormal, 0.0, false, 0);
    TEST_SPECIAL(-min_denormal, max_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, -max_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, min_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, -min_normal, -min_denormal, false, 0);
    TEST_SPECIAL(-min_denormal, min_denormal, neg_zero, false, 0);
    TEST_SPECIAL(-min_denormal, -min_denormal, neg_zero, false, 0);
  }

  void testRegularExtreme(FModFunc f) {

    TEST_REGULAR(0x1p127L, 0x3p-149L, 0x1p-149L);
    TEST_REGULAR(0x1p127L, -0x3p-149L, 0x1p-149L);
    TEST_REGULAR(0x1p127L, 0x3p-148L, 0x1p-147L);
    TEST_REGULAR(0x1p127L, -0x3p-148L, 0x1p-147L);
    TEST_REGULAR(0x1p127L, 0x3p-126L, 0x1p-125L);
    TEST_REGULAR(0x1p127L, -0x3p-126L, 0x1p-125L);
    TEST_REGULAR(-0x1p127L, 0x3p-149L, -0x1p-149L);
    TEST_REGULAR(-0x1p127L, -0x3p-149L, -0x1p-149L);
    TEST_REGULAR(-0x1p127L, 0x3p-148L, -0x1p-147L);
    TEST_REGULAR(-0x1p127L, -0x3p-148L, -0x1p-147L);
    TEST_REGULAR(-0x1p127L, 0x3p-126L, -0x1p-125L);
    TEST_REGULAR(-0x1p127L, -0x3p-126L, -0x1p-125L);

    if constexpr (sizeof(T) >= sizeof(double)) {
      TEST_REGULAR(0x1p1023L, 0x3p-1074L, 0x1p-1073L);
      TEST_REGULAR(0x1p1023L, -0x3p-1074L, 0x1p-1073L);
      TEST_REGULAR(0x1p1023L, 0x3p-1073L, 0x1p-1073L);
      TEST_REGULAR(0x1p1023L, -0x3p-1073L, 0x1p-1073L);
      TEST_REGULAR(0x1p1023L, 0x3p-1022L, 0x1p-1021L);
      TEST_REGULAR(0x1p1023L, -0x3p-1022L, 0x1p-1021L);
      TEST_REGULAR(-0x1p1023L, 0x3p-1074L, -0x1p-1073L);
      TEST_REGULAR(-0x1p1023L, -0x3p-1074L, -0x1p-1073L);
      TEST_REGULAR(-0x1p1023L, 0x3p-1073L, -0x1p-1073L);
      TEST_REGULAR(-0x1p1023L, -0x3p-1073L, -0x1p-1073L);
      TEST_REGULAR(-0x1p1023L, 0x3p-1022L, -0x1p-1021L);
      TEST_REGULAR(-0x1p1023L, -0x3p-1022L, -0x1p-1021L);
    }
  }
};

#define LIST_FMOD_TESTS(T, func)                                               \
  using LlvmLibcFmodTest = FmodTest<T>;                                        \
  TEST_F(LlvmLibcFmodTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcFmodTest, RegularExtreme) { testRegularExtreme(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_FMODTEST_H
