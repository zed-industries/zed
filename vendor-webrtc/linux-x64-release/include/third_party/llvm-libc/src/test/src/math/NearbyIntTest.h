//===-- Utility class to test different flavors of nearbyint ----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_NEARBYINTTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_NEARBYINTTEST_H

#include "src/__support/CPP/algorithm.h"
#include "src/__support/CPP/array.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

using namespace LIBC_NAMESPACE::fputil::testing;

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename T>
class NearbyIntTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

  static constexpr LIBC_NAMESPACE::cpp::array<RoundingMode, 4> ROUNDING_MODES =
      {
          RoundingMode::Upward,
          RoundingMode::Downward,
          RoundingMode::TowardZero,
          RoundingMode::Nearest,
      };

  static constexpr StorageType MIN_SUBNORMAL =
      FPBits::min_subnormal().uintval();
  static constexpr StorageType MAX_SUBNORMAL =
      FPBits::max_subnormal().uintval();
  static constexpr StorageType MIN_NORMAL = FPBits::min_normal().uintval();
  static constexpr StorageType MAX_NORMAL = FPBits::max_normal().uintval();

public:
  using NearbyIntFunc = T (*)(T);

  void test_round_numbers(NearbyIntFunc func) {
    for (RoundingMode mode : ROUNDING_MODES) {
      if (ForceRoundingMode r(mode); r.success) {
        EXPECT_FP_EQ(func(T(1.0)), mpfr::round(T(1.0), mode));
        EXPECT_FP_EQ(func(T(-1.0)), mpfr::round(T(-1.0), mode));
        EXPECT_FP_EQ(func(T(10.0)), mpfr::round(T(10.0), mode));
        EXPECT_FP_EQ(func(T(-10.0)), mpfr::round(T(-10.0), mode));
        EXPECT_FP_EQ(func(T(1234.0)), mpfr::round(T(1234.0), mode));
        EXPECT_FP_EQ(func(T(-1234.0)), mpfr::round(T(-1234.0), mode));
      }
    }
  }

  void test_fractions(NearbyIntFunc func) {
    for (RoundingMode mode : ROUNDING_MODES) {
      if (ForceRoundingMode r(mode); r.success) {
        EXPECT_FP_EQ(func(T(0.5)), mpfr::round(T(0.5), mode));
        EXPECT_FP_EQ(func(T(-0.5)), mpfr::round(T(-0.5), mode));
        EXPECT_FP_EQ(func(T(0.115)), mpfr::round(T(0.115), mode));
        EXPECT_FP_EQ(func(T(-0.115)), mpfr::round(T(-0.115), mode));
        EXPECT_FP_EQ(func(T(0.715)), mpfr::round(T(0.715), mode));
        EXPECT_FP_EQ(func(T(-0.715)), mpfr::round(T(-0.715), mode));
      }
    }
  }

  void test_subnormal_range(NearbyIntFunc func) {
    constexpr int COUNT = 100'001;
    const StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>((MAX_SUBNORMAL - MIN_SUBNORMAL) / COUNT),
        StorageType(1));
    for (StorageType i = MIN_SUBNORMAL; i <= MAX_SUBNORMAL; i += STEP) {
      T x = FPBits(i).get_val();
      for (RoundingMode mode : ROUNDING_MODES) {
        if (ForceRoundingMode r(mode); r.success) {
          EXPECT_FP_EQ(func(x), mpfr::round(x, mode));
        }
      }
    }
  }

  void test_normal_range(NearbyIntFunc func) {
    constexpr int COUNT = 100'001;
    const StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>((MAX_NORMAL - MIN_NORMAL) / COUNT),
        StorageType(1));
    for (StorageType i = MIN_NORMAL; i <= MAX_NORMAL; i += STEP) {
      FPBits xbits(i);
      T x = xbits.get_val();
      // In normal range on x86 platforms, the long double implicit 1 bit can be
      // zero making the numbers NaN. We will skip them.
      if (xbits.is_nan())
        continue;

      for (RoundingMode mode : ROUNDING_MODES) {
        if (ForceRoundingMode r(mode); r.success) {
          EXPECT_FP_EQ(func(x), mpfr::round(x, mode));
        }
      }
    }
  }
};

#define LIST_NEARBYINT_TESTS(F, func)                                          \
  using LlvmLibcNearbyIntTest = NearbyIntTestTemplate<F>;                      \
  TEST_F(LlvmLibcNearbyIntTest, RoundNumbers) { test_round_numbers(&func); }   \
  TEST_F(LlvmLibcNearbyIntTest, Fractions) { test_fractions(&func); }          \
  TEST_F(LlvmLibcNearbyIntTest, SubnormalRange) {                              \
    test_subnormal_range(&func);                                               \
  }                                                                            \
  TEST_F(LlvmLibcNearbyIntTest, NormalRange) { test_normal_range(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_NEARBYINTTEST_H
