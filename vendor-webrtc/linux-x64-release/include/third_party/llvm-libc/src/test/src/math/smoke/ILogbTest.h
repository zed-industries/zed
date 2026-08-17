//===-- Utility class to test different flavors of ilogb --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_ILOGBTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_ILOGBTEST_H

#include "src/__support/CPP/algorithm.h"
#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/FPUtil/ManipulationFunctions.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

template <typename OutType, typename InType>
class LlvmLibcILogbTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<InType>;
  using StorageType = typename FPBits::StorageType;

public:
  typedef OutType (*Func)(InType);

  void test_special_numbers(Func func) {
    EXPECT_EQ(LIBC_NAMESPACE::fputil::IntLogbConstants<OutType>::FP_LOGB0,
              func(FPBits::zero(Sign::POS).get_val()));
    EXPECT_EQ(LIBC_NAMESPACE::fputil::IntLogbConstants<OutType>::FP_LOGB0,
              func(FPBits::zero(Sign::NEG).get_val()));
    EXPECT_EQ(LIBC_NAMESPACE::fputil::IntLogbConstants<OutType>::FP_LOGBNAN,
              func(FPBits::quiet_nan().get_val()));
    EXPECT_EQ(LIBC_NAMESPACE::fputil::IntLogbConstants<OutType>::T_MAX,
              func(FPBits::inf(Sign::POS).get_val()));
    EXPECT_EQ(LIBC_NAMESPACE::fputil::IntLogbConstants<OutType>::T_MAX,
              func(FPBits::inf(Sign::NEG).get_val()));
  }

  void test_powers_of_two(Func func) {
    EXPECT_EQ(OutType(0), func(InType(1.0)));
    EXPECT_EQ(OutType(0), func(InType(-1.0)));

    EXPECT_EQ(OutType(1), func(InType(2.0)));
    EXPECT_EQ(OutType(1), func(InType(-2.0)));

    EXPECT_EQ(OutType(2), func(InType(4.0)));
    EXPECT_EQ(OutType(2), func(InType(-4.0)));

    EXPECT_EQ(OutType(3), func(InType(8.0)));
    EXPECT_EQ(OutType(3), func(InType(-8.0)));

    EXPECT_EQ(OutType(4), func(InType(16.0)));
    EXPECT_EQ(OutType(4), func(InType(-16.0)));

    EXPECT_EQ(OutType(5), func(InType(32.0)));
    EXPECT_EQ(OutType(5), func(InType(-32.0)));
  }

  void test_some_integers(Func func) {
    EXPECT_EQ(OutType(1), func(InType(3.0)));
    EXPECT_EQ(OutType(1), func(InType(-3.0)));

    EXPECT_EQ(OutType(2), func(InType(7.0)));
    EXPECT_EQ(OutType(2), func(InType(-7.0)));

    EXPECT_EQ(OutType(3), func(InType(10.0)));
    EXPECT_EQ(OutType(3), func(InType(-10.0)));

    EXPECT_EQ(OutType(4), func(InType(31.0)));
    EXPECT_EQ(OutType(4), func(InType(-31.0)));

    EXPECT_EQ(OutType(5), func(InType(55.0)));
    EXPECT_EQ(OutType(5), func(InType(-55.0)));
  }

  void test_subnormal_range(Func func) {
    constexpr StorageType MIN_SUBNORMAL = FPBits::min_subnormal().uintval();
    constexpr StorageType MAX_SUBNORMAL = FPBits::max_subnormal().uintval();
    constexpr int COUNT = 10'001;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>((MAX_SUBNORMAL - MIN_SUBNORMAL) / COUNT),
        StorageType(1));
    for (StorageType v = MIN_SUBNORMAL; v <= MAX_SUBNORMAL; v += STEP) {
      FPBits x_bits(v);
      if (x_bits.is_zero() || x_bits.is_inf_or_nan())
        continue;

      InType x = x_bits.get_val();

      int exponent;
      LIBC_NAMESPACE::fputil::frexp(x, exponent);
      ASSERT_EQ(static_cast<OutType>(exponent), func(x) + OutType(1));
    }
  }

  void test_normal_range(Func func) {
    constexpr StorageType MIN_NORMAL = FPBits::min_normal().uintval();
    constexpr StorageType MAX_NORMAL = FPBits::max_normal().uintval();
    constexpr int COUNT = 10'001;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>((MAX_NORMAL - MIN_NORMAL) / COUNT),
        StorageType(1));
    for (StorageType v = MIN_NORMAL; v <= MAX_NORMAL; v += STEP) {
      FPBits x_bits(v);
      if (x_bits.is_zero() || x_bits.is_inf_or_nan())
        continue;

      InType x = x_bits.get_val();

      int exponent;
      LIBC_NAMESPACE::fputil::frexp(x, exponent);
      ASSERT_EQ(static_cast<OutType>(exponent), func(x) + OutType(1));
    }
  }
};

#define LIST_INTLOGB_TESTS(OutType, InType, Func)                              \
  using LlvmLibcIntLogbTest = LlvmLibcILogbTest<OutType, InType>;              \
  TEST_F(LlvmLibcIntLogbTest, SpecialNumbers) { test_special_numbers(&Func); } \
  TEST_F(LlvmLibcIntLogbTest, PowersOfTwo) { test_powers_of_two(&Func); }      \
  TEST_F(LlvmLibcIntLogbTest, SomeIntegers) { test_some_integers(&Func); }     \
  TEST_F(LlvmLibcIntLogbTest, SubnormalRange) { test_subnormal_range(&Func); } \
  TEST_F(LlvmLibcIntLogbTest, NormalRange) { test_normal_range(&Func); }       \
  static_assert(true)

#endif // LLVM_LIBC_TEST_SRC_MATH_ILOGBTEST_H
