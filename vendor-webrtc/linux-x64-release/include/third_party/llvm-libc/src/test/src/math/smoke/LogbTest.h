//===-- Utility class to test logb[f|l] -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/CPP/algorithm.h"
#include "src/__support/FPUtil/ManipulationFunctions.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

template <typename T>
class LogbTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

  static constexpr StorageType HIDDEN_BIT =
      StorageType(1) << LIBC_NAMESPACE::fputil::FPBits<T>::FRACTION_LEN;

public:
  typedef T (*LogbFunc)(T);

  void testSpecialNumbers(LogbFunc func) {
    ASSERT_FP_EQ(aNaN, func(aNaN));
    ASSERT_FP_EQ(inf, func(inf));
    ASSERT_FP_EQ(inf, func(neg_inf));
    ASSERT_FP_EQ(neg_inf, func(zero));
    ASSERT_FP_EQ(neg_inf, func(neg_zero));
  }

  void testPowersOfTwo(LogbFunc func) {
    EXPECT_FP_EQ(T(0.0), func(T(1.0)));
    EXPECT_FP_EQ(T(0.0), func(T(-1.0)));

    EXPECT_FP_EQ(T(1.0), func(T(2.0)));
    EXPECT_FP_EQ(T(1.0), func(T(-2.0)));

    EXPECT_FP_EQ(T(2.0), func(T(4.0)));
    EXPECT_FP_EQ(T(2.0), func(T(-4.0)));

    EXPECT_FP_EQ(T(3.0), func(T(8.0)));
    EXPECT_FP_EQ(T(3.0), func(T(-8.0)));

    EXPECT_FP_EQ(T(4.0), func(T(16.0)));
    EXPECT_FP_EQ(T(4.0), func(T(-16.0)));

    EXPECT_FP_EQ(T(5.0), func(T(32.0)));
    EXPECT_FP_EQ(T(5.0), func(T(-32.0)));
  }

  void testSomeIntegers(LogbFunc func) {
    EXPECT_FP_EQ(T(1.0), func(T(3.0)));
    EXPECT_FP_EQ(T(1.0), func(T(-3.0)));

    EXPECT_FP_EQ(T(2.0), func(T(7.0)));
    EXPECT_FP_EQ(T(2.0), func(T(-7.0)));

    EXPECT_FP_EQ(T(3.0), func(T(10.0)));
    EXPECT_FP_EQ(T(3.0), func(T(-10.0)));

    EXPECT_FP_EQ(T(4.0), func(T(31.0)));
    EXPECT_FP_EQ(T(4.0), func(T(-31.0)));

    EXPECT_FP_EQ(T(5.0), func(T(55.0)));
    EXPECT_FP_EQ(T(5.0), func(T(-55.0)));
  }

  void testRange(LogbFunc func) {
    using StorageType = typename FPBits::StorageType;
    constexpr int COUNT = 100'000;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>(STORAGE_MAX / COUNT), StorageType(1));
    StorageType v = 0;
    for (int i = 0; i <= COUNT; ++i, v += STEP) {
      FPBits x_bits(v);
      if (x_bits.is_zero() || x_bits.is_inf_or_nan())
        continue;

      T x = x_bits.get_val();

      int exponent;
      LIBC_NAMESPACE::fputil::frexp(x, exponent);
      ASSERT_FP_EQ(T(exponent), func(x) + T(1.0));
    }
  }
};

#define LIST_LOGB_TESTS(T, func)                                               \
  using LlvmLibcLogbTest = LogbTest<T>;                                        \
  TEST_F(LlvmLibcLogbTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcLogbTest, PowersOfTwo) { testPowersOfTwo(&func); }            \
  TEST_F(LlvmLibcLogbTest, SomeIntegers) { testSomeIntegers(&func); }          \
  TEST_F(LlvmLibcLogbTest, InRange) { testRange(&func); }
