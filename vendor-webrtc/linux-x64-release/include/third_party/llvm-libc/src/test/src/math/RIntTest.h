//===-- Utility class to test different flavors of rint ---------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_RINTTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_RINTTEST_H

#include "src/__support/CPP/algorithm.h"
#include "src/__support/FPUtil/FEnvImpl.h"
#include "src/__support/FPUtil/FPBits.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

#include "hdr/fenv_macros.h"
#include "hdr/math_macros.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;
using LIBC_NAMESPACE::Sign;

static constexpr int ROUNDING_MODES[4] = {FE_UPWARD, FE_DOWNWARD, FE_TOWARDZERO,
                                          FE_TONEAREST};

template <typename T>
class RIntTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {
public:
  typedef T (*RIntFunc)(T);

private:
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
  using StorageType = typename FPBits::StorageType;

  const T inf = FPBits::inf(Sign::POS).get_val();
  const T neg_inf = FPBits::inf(Sign::NEG).get_val();
  const T zero = FPBits::zero(Sign::POS).get_val();
  const T neg_zero = FPBits::zero(Sign::NEG).get_val();
  const T nan = FPBits::quiet_nan().get_val();

  static constexpr StorageType MIN_SUBNORMAL =
      FPBits::min_subnormal().uintval();
  static constexpr StorageType MAX_SUBNORMAL =
      FPBits::max_subnormal().uintval();
  static constexpr StorageType MIN_NORMAL = FPBits::min_normal().uintval();
  static constexpr StorageType MAX_NORMAL = FPBits::max_normal().uintval();

  static inline mpfr::RoundingMode to_mpfr_rounding_mode(int mode) {
    switch (mode) {
    case FE_UPWARD:
      return mpfr::RoundingMode::Upward;
    case FE_DOWNWARD:
      return mpfr::RoundingMode::Downward;
    case FE_TOWARDZERO:
      return mpfr::RoundingMode::TowardZero;
    case FE_TONEAREST:
      return mpfr::RoundingMode::Nearest;
    default:
      __builtin_unreachable();
    }
  }

public:
  void testSpecialNumbers(RIntFunc func) {
    for (int mode : ROUNDING_MODES) {
      LIBC_NAMESPACE::fputil::set_round(mode);
      ASSERT_FP_EQ(inf, func(inf));
      ASSERT_FP_EQ(neg_inf, func(neg_inf));
      ASSERT_FP_EQ(nan, func(nan));
      ASSERT_FP_EQ(zero, func(zero));
      ASSERT_FP_EQ(neg_zero, func(neg_zero));
    }
  }

  void testRoundNumbers(RIntFunc func) {
    for (int mode : ROUNDING_MODES) {
      LIBC_NAMESPACE::fputil::set_round(mode);
      mpfr::RoundingMode mpfr_mode = to_mpfr_rounding_mode(mode);
      ASSERT_FP_EQ(func(T(1.0)), mpfr::round(T(1.0), mpfr_mode));
      ASSERT_FP_EQ(func(T(-1.0)), mpfr::round(T(-1.0), mpfr_mode));
      ASSERT_FP_EQ(func(T(10.0)), mpfr::round(T(10.0), mpfr_mode));
      ASSERT_FP_EQ(func(T(-10.0)), mpfr::round(T(-10.0), mpfr_mode));
      ASSERT_FP_EQ(func(T(1234.0)), mpfr::round(T(1234.0), mpfr_mode));
      ASSERT_FP_EQ(func(T(-1234.0)), mpfr::round(T(-1234.0), mpfr_mode));
    }
  }

  void testFractions(RIntFunc func) {
    for (int mode : ROUNDING_MODES) {
      LIBC_NAMESPACE::fputil::set_round(mode);
      mpfr::RoundingMode mpfr_mode = to_mpfr_rounding_mode(mode);
      ASSERT_FP_EQ(func(T(0.5)), mpfr::round(T(0.5), mpfr_mode));
      ASSERT_FP_EQ(func(T(-0.5)), mpfr::round(T(-0.5), mpfr_mode));
      ASSERT_FP_EQ(func(T(0.115)), mpfr::round(T(0.115), mpfr_mode));
      ASSERT_FP_EQ(func(T(-0.115)), mpfr::round(T(-0.115), mpfr_mode));
      ASSERT_FP_EQ(func(T(0.715)), mpfr::round(T(0.715), mpfr_mode));
      ASSERT_FP_EQ(func(T(-0.715)), mpfr::round(T(-0.715), mpfr_mode));
    }
  }

  void testSubnormalRange(RIntFunc func) {
    constexpr int COUNT = 100'001;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>((MAX_SUBNORMAL - MIN_SUBNORMAL) / COUNT),
        StorageType(1));
    for (StorageType i = MIN_SUBNORMAL; i <= MAX_SUBNORMAL; i += STEP) {
      T x = FPBits(i).get_val();
      for (int mode : ROUNDING_MODES) {
        LIBC_NAMESPACE::fputil::set_round(mode);
        mpfr::RoundingMode mpfr_mode = to_mpfr_rounding_mode(mode);
        ASSERT_FP_EQ(func(x), mpfr::round(x, mpfr_mode));
      }
    }
  }

  void testNormalRange(RIntFunc func) {
    constexpr int COUNT = 100'001;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>((MAX_NORMAL - MIN_NORMAL) / COUNT),
        StorageType(1));
    for (StorageType i = MIN_NORMAL; i <= MAX_NORMAL; i += STEP) {
      FPBits xbits(i);
      T x = xbits.get_val();
      // In normal range on x86 platforms, the long double implicit 1 bit can be
      // zero making the numbers NaN. We will skip them.
      if (xbits.is_nan())
        continue;

      for (int mode : ROUNDING_MODES) {
        LIBC_NAMESPACE::fputil::set_round(mode);
        mpfr::RoundingMode mpfr_mode = to_mpfr_rounding_mode(mode);
        ASSERT_FP_EQ(func(x), mpfr::round(x, mpfr_mode));
      }
    }
  }
};

#define LIST_RINT_TESTS(F, func)                                               \
  using LlvmLibcRIntTest = RIntTestTemplate<F>;                                \
  TEST_F(LlvmLibcRIntTest, specialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcRIntTest, RoundNumbers) { testRoundNumbers(&func); }          \
  TEST_F(LlvmLibcRIntTest, Fractions) { testFractions(&func); }                \
  TEST_F(LlvmLibcRIntTest, SubnormalRange) { testSubnormalRange(&func); }      \
  TEST_F(LlvmLibcRIntTest, NormalRange) { testNormalRange(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_RINTTEST_H
