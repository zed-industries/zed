//===-- Utility class to test floor[f|l] ------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/FPUtil/BasicOperations.h"
#include "src/__support/FPUtil/NearestIntegerOperations.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

#include "hdr/math_macros.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename T>
class ModfTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*ModfFunc)(T, T *);

  void testSpecialNumbers(ModfFunc func) {
    T integral;

    EXPECT_FP_EQ(zero, func(zero, &integral));
    EXPECT_FP_EQ(integral, zero);
    EXPECT_FP_EQ(neg_zero, func(neg_zero, &integral));
    EXPECT_FP_EQ(integral, neg_zero);

    EXPECT_FP_EQ(zero, func(inf, &integral));
    EXPECT_FP_EQ(inf, integral);
    EXPECT_FP_EQ(neg_zero, func(neg_inf, &integral));
    EXPECT_FP_EQ(neg_inf, integral);

    EXPECT_FP_EQ(aNaN, func(aNaN, &integral));
  }

  void testIntegers(ModfFunc func) {
    T integral;

    EXPECT_FP_EQ(T(0.0), func(T(1.0), &integral));
    EXPECT_FP_EQ(T(1.0), integral);

    EXPECT_FP_EQ(T(-0.0), func(T(-1.0), &integral));
    EXPECT_FP_EQ(T(-1.0), integral);

    EXPECT_FP_EQ(T(0.0), func(T(10.0), &integral));
    EXPECT_FP_EQ(T(10.0), integral);

    EXPECT_FP_EQ(T(-0.0), func(T(-10.0), &integral));
    EXPECT_FP_EQ(T(-10.0), integral);

    EXPECT_FP_EQ(T(0.0), func(T(12345.0), &integral));
    EXPECT_FP_EQ(T(12345.0), integral);

    EXPECT_FP_EQ(T(-0.0), func(T(-12345.0), &integral));
    EXPECT_FP_EQ(T(-12345.0), integral);
  }

  void testFractions(ModfFunc func) {
    T integral;

    EXPECT_FP_EQ(T(0.5), func(T(1.5), &integral));
    EXPECT_FP_EQ(integral, T(1.0));

    EXPECT_FP_EQ(T(-0.5), func(T(-1.5), &integral));
    EXPECT_FP_EQ(integral, T(-1.0));

    EXPECT_FP_EQ(T(0.75), func(T(10.75), &integral));
    EXPECT_FP_EQ(integral, T(10.0));

    EXPECT_FP_EQ(T(-0.75), func(T(-10.75), &integral));
    EXPECT_FP_EQ(integral, T(-10.0));

    EXPECT_FP_EQ(T(0.125), func(T(100.125), &integral));
    EXPECT_FP_EQ(integral, T(100.0));

    EXPECT_FP_EQ(T(-0.125), func(T(-100.125), &integral));
    EXPECT_FP_EQ(integral, T(-100.0));
  }

  void testRange(ModfFunc func) {
    constexpr StorageType COUNT = 100'000;
    constexpr StorageType STEP = STORAGE_MAX / COUNT;
    for (StorageType i = 0, v = 0; i <= COUNT; ++i, v += STEP) {
      T x = FPBits(v).get_val();
      if (FPBits(v).is_nan() || FPBits(v).is_inf() || x == T(0.0))
        continue;

      T integral;
      T frac = func(x, &integral);
      ASSERT_TRUE(LIBC_NAMESPACE::fputil::abs(frac) < 1.0l);
      ASSERT_TRUE(LIBC_NAMESPACE::fputil::trunc(x) == integral);
      ASSERT_TRUE(integral + frac == x);
    }
  }
};

#define LIST_MODF_TESTS(T, func)                                               \
  using LlvmLibcModfTest = ModfTest<T>;                                        \
  TEST_F(LlvmLibcModfTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcModfTest, RoundedNubmers) { testIntegers(&func); }            \
  TEST_F(LlvmLibcModfTest, Fractions) { testFractions(&func); }                \
  TEST_F(LlvmLibcModfTest, Range) { testRange(&func); }
