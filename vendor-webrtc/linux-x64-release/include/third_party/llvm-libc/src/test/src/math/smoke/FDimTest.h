//===-- Utility class to test different flavors of fdim ---------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#include "src/__support/CPP/algorithm.h"
#include "src/__support/FPUtil/FPBits.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

template <typename T>
class FDimTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {
public:
  using FuncPtr = T (*)(T, T);
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
  using StorageType = typename FPBits::StorageType;

  const T inf = FPBits::inf(Sign::POS).get_val();
  const T neg_inf = FPBits::inf(Sign::NEG).get_val();
  const T zero = FPBits::zero(Sign::POS).get_val();
  const T neg_zero = FPBits::zero(Sign::NEG).get_val();
  const T nan = FPBits::quiet_nan().get_val();

  void test_nan_arg(FuncPtr func) {
    EXPECT_FP_EQ(nan, func(nan, inf));
    EXPECT_FP_EQ(nan, func(neg_inf, nan));
    EXPECT_FP_EQ(nan, func(nan, zero));
    EXPECT_FP_EQ(nan, func(neg_zero, nan));
    EXPECT_FP_EQ(nan, func(nan, T(-1.2345)));
    EXPECT_FP_EQ(nan, func(T(1.2345), nan));
    EXPECT_FP_EQ(func(nan, nan), nan);
  }

  void test_inf_arg(FuncPtr func) {
    EXPECT_FP_EQ(zero, func(neg_inf, inf));
    EXPECT_FP_EQ(inf, func(inf, zero));
    EXPECT_FP_EQ(zero, func(neg_zero, inf));
    EXPECT_FP_EQ(inf, func(inf, T(1.2345)));
    EXPECT_FP_EQ(zero, func(T(-1.2345), inf));
  }

  void test_neg_inf_arg(FuncPtr func) {
    EXPECT_FP_EQ(inf, func(inf, neg_inf));
    EXPECT_FP_EQ(zero, func(neg_inf, zero));
    EXPECT_FP_EQ(inf, func(neg_zero, neg_inf));
    EXPECT_FP_EQ(zero, func(neg_inf, T(-1.2345)));
    EXPECT_FP_EQ(inf, func(T(1.2345), neg_inf));
  }

  void test_both_zero(FuncPtr func) {
    EXPECT_FP_EQ(zero, func(zero, zero));
    EXPECT_FP_EQ(zero, func(zero, neg_zero));
    EXPECT_FP_EQ(zero, func(neg_zero, zero));
    EXPECT_FP_EQ(zero, func(neg_zero, neg_zero));
  }

  void test_in_range(FuncPtr func) {
    constexpr StorageType STORAGE_MAX =
        LIBC_NAMESPACE::cpp::numeric_limits<StorageType>::max();
    constexpr int COUNT = 100'001;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>(STORAGE_MAX / COUNT), StorageType(1));
    StorageType v = 0, w = STORAGE_MAX;
    for (int i = 0; i <= COUNT; ++i, v += STEP, w -= STEP) {
      FPBits xbits(v), ybits(w);
      if (xbits.is_inf_or_nan())
        continue;
      if (ybits.is_inf_or_nan())
        continue;

      T x = xbits.get_val();
      T y = ybits.get_val();

      if (x > y) {
        EXPECT_FP_EQ(x - y, func(x, y));
      } else {
        EXPECT_FP_EQ(zero, func(x, y));
      }
    }
  }
};

#define LIST_FDIM_TESTS(T, func)                                               \
  using LlvmLibcFDimTest = FDimTestTemplate<T>;                                \
  TEST_F(LlvmLibcFDimTest, NaNArg) { test_nan_arg(&func); }                    \
  TEST_F(LlvmLibcFDimTest, InfArg) { test_inf_arg(&func); }                    \
  TEST_F(LlvmLibcFDimTest, NegInfArg) { test_neg_inf_arg(&func); }             \
  TEST_F(LlvmLibcFDimTest, BothZero) { test_both_zero(&func); }                \
  TEST_F(LlvmLibcFDimTest, InFloatRange) { test_in_range(&func); }             \
  static_assert(true, "Require semicolon.")
