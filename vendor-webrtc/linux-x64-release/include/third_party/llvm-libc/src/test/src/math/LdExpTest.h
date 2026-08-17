//===-- Utility class to test different flavors of ldexp --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_LDEXPTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_LDEXPTEST_H

#include "src/__support/CPP/limits.h" // INT_MAX
#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/FPUtil/NormalFloat.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"
#include <stdint.h>

using LIBC_NAMESPACE::Sign;

template <typename T>
class LdExpTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
  using NormalFloat = LIBC_NAMESPACE::fputil::NormalFloat<T>;
  using StorageType = typename FPBits::StorageType;

  const T inf = FPBits::inf(Sign::POS).get_val();
  const T neg_inf = FPBits::inf(Sign::NEG).get_val();
  const T zero = FPBits::zero(Sign::POS).get_val();
  const T neg_zero = FPBits::zero(Sign::NEG).get_val();
  const T nan = FPBits::quiet_nan().get_val();

  // A normalized mantissa to be used with tests.
  static constexpr StorageType MANTISSA = NormalFloat::ONE + 0x1234;

public:
  typedef T (*LdExpFunc)(T, int);

  void testSpecialNumbers(LdExpFunc func) {
    int exp_array[5] = {-INT_MAX - 1, -10, 0, 10, INT_MAX};
    for (int exp : exp_array) {
      ASSERT_FP_EQ(zero, func(zero, exp));
      ASSERT_FP_EQ(neg_zero, func(neg_zero, exp));
      ASSERT_FP_EQ(inf, func(inf, exp));
      ASSERT_FP_EQ(neg_inf, func(neg_inf, exp));
      ASSERT_FP_EQ(nan, func(nan, exp));
    }
  }

  void testPowersOfTwo(LdExpFunc func) {
    int32_t exp_array[5] = {1, 2, 3, 4, 5};
    int32_t val_array[6] = {1, 2, 4, 8, 16, 32};
    for (int32_t exp : exp_array) {
      for (int32_t val : val_array) {
        ASSERT_FP_EQ(T(val << exp), func(T(val), exp));
        ASSERT_FP_EQ(T(-1 * (val << exp)), func(T(-val), exp));
      }
    }
  }

  void testOverflow(LdExpFunc func) {
    NormalFloat x(Sign::POS, FPBits::MAX_BIASED_EXPONENT - 10,
                  NormalFloat::ONE + 0xF00BA);
    for (int32_t exp = 10; exp < 100; ++exp) {
      ASSERT_FP_EQ(inf, func(T(x), exp));
      ASSERT_FP_EQ(neg_inf, func(-T(x), exp));
    }
  }

  void testUnderflowToZeroOnNormal(LdExpFunc func) {
    // In this test, we pass a normal nubmer to func and expect zero
    // to be returned due to underflow.
    int32_t base_exponent = FPBits::EXP_BIAS + FPBits::FRACTION_LEN;
    int32_t exp_array[] = {base_exponent + 5, base_exponent + 4,
                           base_exponent + 3, base_exponent + 2,
                           base_exponent + 1};
    T x = NormalFloat(Sign::POS, 0, MANTISSA);
    for (int32_t exp : exp_array) {
      ASSERT_FP_EQ(func(x, -exp), x > 0 ? zero : neg_zero);
    }
  }

  void testUnderflowToZeroOnSubnormal(LdExpFunc func) {
    // In this test, we pass a normal nubmer to func and expect zero
    // to be returned due to underflow.
    int32_t base_exponent = FPBits::EXP_BIAS + FPBits::FRACTION_LEN;
    int32_t exp_array[] = {base_exponent + 5, base_exponent + 4,
                           base_exponent + 3, base_exponent + 2,
                           base_exponent + 1};
    T x = NormalFloat(Sign::POS, -FPBits::EXP_BIAS, MANTISSA);
    for (int32_t exp : exp_array) {
      ASSERT_FP_EQ(func(x, -exp), x > 0 ? zero : neg_zero);
    }
  }

  void testNormalOperation(LdExpFunc func) {
    T val_array[] = {// Normal numbers
                     NormalFloat(Sign::POS, 100, MANTISSA),
                     NormalFloat(Sign::POS, -100, MANTISSA),
                     NormalFloat(Sign::NEG, 100, MANTISSA),
                     NormalFloat(Sign::NEG, -100, MANTISSA),
                     // Subnormal numbers
                     NormalFloat(Sign::POS, -FPBits::EXP_BIAS, MANTISSA),
                     NormalFloat(Sign::NEG, -FPBits::EXP_BIAS, MANTISSA)};
    for (int32_t exp = 0; exp <= FPBits::FRACTION_LEN; ++exp) {
      for (T x : val_array) {
        // We compare the result of ldexp with the result
        // of the native multiplication/division instruction.

        // We need to use a NormalFloat here (instead of 1 << exp), because
        // there are 32 bit systems that don't support 128bit long ints but
        // support long doubles. This test can do 1 << 64, which would fail
        // in these systems.
        NormalFloat two_to_exp = NormalFloat(static_cast<T>(1.L));
        two_to_exp = two_to_exp.mul2(exp);

        ASSERT_FP_EQ(func(x, exp), x * two_to_exp);
        ASSERT_FP_EQ(func(x, -exp), x / two_to_exp);
      }
    }

    // Normal which trigger mantissa overflow.
    T x = NormalFloat(Sign::POS, -FPBits::EXP_BIAS + 1,
                      StorageType(2) * NormalFloat::ONE - StorageType(1));
    ASSERT_FP_EQ(func(x, -1), x / 2);
    ASSERT_FP_EQ(func(-x, -1), -x / 2);

    // Start with a normal number high exponent but pass a very low number for
    // exp. The result should be a subnormal number.
    x = NormalFloat(Sign::POS, FPBits::EXP_BIAS, NormalFloat::ONE);
    int exp = -FPBits::MAX_BIASED_EXPONENT - 5;
    T result = func(x, exp);
    FPBits result_bits(result);
    ASSERT_FALSE(result_bits.is_zero());
    // Verify that the result is indeed subnormal.
    ASSERT_EQ(result_bits.get_biased_exponent(), uint16_t(0));
    // But if the exp is so less that normalization leads to zero, then
    // the result should be zero.
    result = func(x, -FPBits::MAX_BIASED_EXPONENT - FPBits::FRACTION_LEN - 5);
    ASSERT_TRUE(FPBits(result).is_zero());

    // Start with a subnormal number but pass a very high number for exponent.
    // The result should not be infinity.
    x = NormalFloat(Sign::POS, -FPBits::EXP_BIAS + 1, NormalFloat::ONE >> 10);
    exp = FPBits::MAX_BIASED_EXPONENT + 5;
    ASSERT_FALSE(FPBits(func(x, exp)).is_inf());
    // But if the exp is large enough to oversome than the normalization shift,
    // then it should result in infinity.
    exp = FPBits::MAX_BIASED_EXPONENT + 15;
    ASSERT_FP_EQ(func(x, exp), inf);
  }
};

#define LIST_LDEXP_TESTS(T, func)                                              \
  using LlvmLibcLdExpTest = LdExpTestTemplate<T>;                              \
  TEST_F(LlvmLibcLdExpTest, SpecialNumbers) { testSpecialNumbers(&func); }     \
  TEST_F(LlvmLibcLdExpTest, PowersOfTwo) { testPowersOfTwo(&func); }           \
  TEST_F(LlvmLibcLdExpTest, OverFlow) { testOverflow(&func); }                 \
  TEST_F(LlvmLibcLdExpTest, UnderflowToZeroOnNormal) {                         \
    testUnderflowToZeroOnNormal(&func);                                        \
  }                                                                            \
  TEST_F(LlvmLibcLdExpTest, UnderflowToZeroOnSubnormal) {                      \
    testUnderflowToZeroOnSubnormal(&func);                                     \
  }                                                                            \
  TEST_F(LlvmLibcLdExpTest, NormalOperation) { testNormalOperation(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_LDEXPTEST_H
