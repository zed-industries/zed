//===-- Utility class to test frexp[f|l] ------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/FPUtil/BasicOperations.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

#include "hdr/math_macros.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename T>
class FrexpTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

  static constexpr StorageType HIDDEN_BIT =
      StorageType(1) << LIBC_NAMESPACE::fputil::FPBits<T>::FRACTION_LEN;

public:
  typedef T (*FrexpFunc)(T, int *);

  void testSpecialNumbers(FrexpFunc func) {
    int exponent;
    ASSERT_FP_EQ(aNaN, func(aNaN, &exponent));
    ASSERT_FP_EQ(inf, func(inf, &exponent));
    ASSERT_FP_EQ(neg_inf, func(neg_inf, &exponent));

    ASSERT_FP_EQ(zero, func(zero, &exponent));
    ASSERT_EQ(exponent, 0);

    ASSERT_FP_EQ(neg_zero, func(neg_zero, &exponent));
    ASSERT_EQ(exponent, 0);
  }

  void testPowersOfTwo(FrexpFunc func) {
    int exponent;

    EXPECT_FP_EQ(T(0.5), func(T(1.0), &exponent));
    EXPECT_EQ(exponent, 1);
    EXPECT_FP_EQ(T(-0.5), func(T(-1.0), &exponent));
    EXPECT_EQ(exponent, 1);

    EXPECT_FP_EQ(T(0.5), func(T(2.0), &exponent));
    EXPECT_EQ(exponent, 2);
    EXPECT_FP_EQ(T(-0.5), func(T(-2.0), &exponent));
    EXPECT_EQ(exponent, 2);

    EXPECT_FP_EQ(T(0.5), func(T(4.0), &exponent));
    EXPECT_EQ(exponent, 3);
    EXPECT_FP_EQ(T(-0.5), func(T(-4.0), &exponent));
    EXPECT_EQ(exponent, 3);

    EXPECT_FP_EQ(T(0.5), func(T(8.0), &exponent));
    EXPECT_EQ(exponent, 4);
    EXPECT_FP_EQ(T(-0.5), func(T(-8.0), &exponent));
    EXPECT_EQ(exponent, 4);

    EXPECT_FP_EQ(T(0.5), func(T(16.0), &exponent));
    EXPECT_EQ(exponent, 5);
    EXPECT_FP_EQ(T(-0.5), func(T(-16.0), &exponent));
    EXPECT_EQ(exponent, 5);

    EXPECT_FP_EQ(T(0.5), func(T(32.0), &exponent));
    EXPECT_EQ(exponent, 6);
    EXPECT_FP_EQ(T(-0.5), func(T(-32.0), &exponent));
    EXPECT_EQ(exponent, 6);
  }

  void testSomeIntegers(FrexpFunc func) {
    int exponent;

    EXPECT_FP_EQ(T(0.75), func(T(24.0), &exponent));
    EXPECT_EQ(exponent, 5);
    EXPECT_FP_EQ(T(-0.75), func(T(-24.0), &exponent));
    EXPECT_EQ(exponent, 5);

    EXPECT_FP_EQ(T(0.625), func(T(40.0), &exponent));
    EXPECT_EQ(exponent, 6);
    EXPECT_FP_EQ(T(-0.625), func(T(-40.0), &exponent));
    EXPECT_EQ(exponent, 6);

    EXPECT_FP_EQ(T(0.78125), func(T(800.0), &exponent));
    EXPECT_EQ(exponent, 10);
    EXPECT_FP_EQ(T(-0.78125), func(T(-800.0), &exponent));
    EXPECT_EQ(exponent, 10);
  }

  void testRange(FrexpFunc func) {
    using StorageType = typename FPBits::StorageType;
    constexpr StorageType COUNT = 100'000;
    constexpr StorageType STEP = STORAGE_MAX / COUNT;
    for (StorageType i = 0, v = 0; i <= COUNT; ++i, v += STEP) {
      T x = FPBits(v).get_val();
      if (FPBits(v).is_nan() || FPBits(v).is_inf() || x == 0.0l)
        continue;

      mpfr::BinaryOutput<T> result;
      result.f = func(x, &result.i);

      ASSERT_TRUE(LIBC_NAMESPACE::fputil::abs(result.f) < 1.0);
      ASSERT_TRUE(LIBC_NAMESPACE::fputil::abs(result.f) >= 0.5);
      ASSERT_MPFR_MATCH(mpfr::Operation::Frexp, x, result, 0.0);
    }
  }
};

#define LIST_FREXP_TESTS(T, func)                                              \
  using LlvmLibcFrexpTest = FrexpTest<T>;                                      \
  TEST_F(LlvmLibcFrexpTest, SpecialNumbers) { testSpecialNumbers(&func); }     \
  TEST_F(LlvmLibcFrexpTest, PowersOfTwo) { testPowersOfTwo(&func); }           \
  TEST_F(LlvmLibcFrexpTest, SomeIntegers) { testSomeIntegers(&func); }         \
  TEST_F(LlvmLibcFrexpTest, InRange) { testRange(&func); }
