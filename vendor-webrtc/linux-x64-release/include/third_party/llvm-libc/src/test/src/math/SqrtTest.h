//===-- Utility class to test sqrt[f|l] -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename OutType, typename InType>
class SqrtTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(InType)

  static constexpr StorageType HIDDEN_BIT =
      StorageType(1) << LIBC_NAMESPACE::fputil::FPBits<InType>::FRACTION_LEN;

public:
  using SqrtFunc = OutType (*)(InType);

  void test_denormal_values(SqrtFunc func) {
    for (StorageType mant = 1; mant < HIDDEN_BIT; mant <<= 1) {
      FPBits denormal(zero);
      denormal.set_mantissa(mant);
      InType x = denormal.get_val();
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Sqrt, x, func(x), 0.5);
    }

    constexpr StorageType COUNT = 200'001;
    constexpr StorageType STEP = HIDDEN_BIT / COUNT;
    for (StorageType i = 0, v = 0; i <= COUNT; ++i, v += STEP) {
      InType x = FPBits(i).get_val();
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Sqrt, x, func(x), 0.5);
    }
  }

  void test_normal_range(SqrtFunc func) {
    constexpr StorageType COUNT = 200'001;
    constexpr StorageType STEP = STORAGE_MAX / COUNT;
    for (StorageType i = 0, v = 0; i <= COUNT; ++i, v += STEP) {
      FPBits x_bits(v);
      InType x = x_bits.get_val();
      if (x_bits.is_nan() || (x < 0))
        continue;
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Sqrt, x, func(x), 0.5);
    }
  }
};

#define LIST_SQRT_TESTS(T, func)                                               \
  using LlvmLibcSqrtTest = SqrtTest<T, T>;                                     \
  TEST_F(LlvmLibcSqrtTest, DenormalValues) { test_denormal_values(&func); }    \
  TEST_F(LlvmLibcSqrtTest, NormalRange) { test_normal_range(&func); }

#define LIST_NARROWING_SQRT_TESTS(OutType, InType, func)                       \
  using LlvmLibcSqrtTest = SqrtTest<OutType, InType>;                          \
  TEST_F(LlvmLibcSqrtTest, DenormalValues) { test_denormal_values(&func); }    \
  TEST_F(LlvmLibcSqrtTest, NormalRange) { test_normal_range(&func); }
