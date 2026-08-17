//===-- Utility class to test different flavors of ilogb --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_ILOGBTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_ILOGBTEST_H

#include "hdr/math_macros.h"
#include "src/__support/CPP/limits.h" // INT_MAX
#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/FPUtil/ManipulationFunctions.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

class LlvmLibcILogbTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {
public:
  template <typename T> struct ILogbFunc {
    typedef int (*Func)(T);
  };

  template <typename T>
  void test_special_numbers(typename ILogbFunc<T>::Func func) {
    using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;

    EXPECT_EQ(FP_ILOGB0, func(FPBits::zero(Sign::POS).get_val()));
    EXPECT_EQ(FP_ILOGB0, func(FPBits::zero(Sign::NEG).get_val()));
    EXPECT_EQ(FP_ILOGBNAN, func(FPBits::quiet_nan().get_val()));
    EXPECT_EQ(INT_MAX, func(FPBits::inf(Sign::POS).get_val()));
    EXPECT_EQ(INT_MAX, func(FPBits::inf(Sign::NEG).get_val()));
  }

  template <typename T>
  void test_powers_of_two(typename ILogbFunc<T>::Func func) {
    EXPECT_EQ(0, func(T(1.0)));
    EXPECT_EQ(0, func(T(-1.0)));

    EXPECT_EQ(1, func(T(2.0)));
    EXPECT_EQ(1, func(T(-2.0)));

    EXPECT_EQ(2, func(T(4.0)));
    EXPECT_EQ(2, func(T(-4.0)));

    EXPECT_EQ(3, func(T(8.0)));
    EXPECT_EQ(3, func(-8.0));

    EXPECT_EQ(4, func(16.0));
    EXPECT_EQ(4, func(-16.0));

    EXPECT_EQ(5, func(32.0));
    EXPECT_EQ(5, func(-32.0));
  }

  template <typename T>
  void test_some_integers(typename ILogbFunc<T>::Func func) {
    EXPECT_EQ(1, func(T(3.0)));
    EXPECT_EQ(1, func(T(-3.0)));

    EXPECT_EQ(2, func(T(7.0)));
    EXPECT_EQ(2, func(T(-7.0)));

    EXPECT_EQ(3, func(T(10.0)));
    EXPECT_EQ(3, func(T(-10.0)));

    EXPECT_EQ(4, func(T(31.0)));
    EXPECT_EQ(4, func(-31.0));

    EXPECT_EQ(5, func(55.0));
    EXPECT_EQ(5, func(-55.0));
  }

  template <typename T>
  void test_subnormal_range(typename ILogbFunc<T>::Func func) {
    using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
    using StorageType = typename FPBits::StorageType;
    constexpr StorageType MIN_SUBNORMAL = FPBits::min_subnormal().uintval();
    constexpr StorageType MAX_SUBNORMAL = FPBits::max_subnormal().uintval();
    constexpr StorageType COUNT = 10'001;
    constexpr StorageType STEP = (MAX_SUBNORMAL - MIN_SUBNORMAL) / COUNT;
    for (StorageType v = MIN_SUBNORMAL; v <= MAX_SUBNORMAL; v += STEP) {
      T x = FPBits(v).get_val();
      if (FPBits(v).is_nan() || FPBits(v).is_inf() || x == 0.0)
        continue;

      int exponent;
      LIBC_NAMESPACE::fputil::frexp(x, exponent);
      ASSERT_EQ(exponent, func(x) + 1);
    }
  }

  template <typename T>
  void test_normal_range(typename ILogbFunc<T>::Func func) {
    using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
    using StorageType = typename FPBits::StorageType;
    constexpr StorageType MIN_NORMAL = FPBits::min_normal().uintval();
    constexpr StorageType MAX_NORMAL = FPBits::max_normal().uintval();
    constexpr StorageType COUNT = 10'001;
    constexpr StorageType STEP = (MAX_NORMAL - MIN_NORMAL) / COUNT;
    for (StorageType v = MIN_NORMAL; v <= MAX_NORMAL; v += STEP) {
      T x = FPBits(v).get_val();
      if (FPBits(v).is_nan() || FPBits(v).is_inf() || x == 0.0)
        continue;

      int exponent;
      LIBC_NAMESPACE::fputil::frexp(x, exponent);
      ASSERT_EQ(exponent, func(x) + 1);
    }
  }
};

#endif // LLVM_LIBC_TEST_SRC_MATH_ILOGBTEST_H
