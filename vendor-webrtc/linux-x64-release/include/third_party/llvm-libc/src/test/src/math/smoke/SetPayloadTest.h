//===-- Utility class to test different flavors of setpayload ---*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBC_TEST_SRC_MATH_SMOKE_SETPAYLOADTEST_H
#define LIBC_TEST_SRC_MATH_SMOKE_SETPAYLOADTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

template <typename T>
class SetPayloadTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*SetPayloadFunc)(T *, T);

  void testInvalidPayloads(SetPayloadFunc func) {
    T res;

    EXPECT_EQ(1, func(&res, T(aNaN)));
    EXPECT_EQ(1, func(&res, T(neg_aNaN)));
    EXPECT_EQ(1, func(&res, T(inf)));
    EXPECT_EQ(1, func(&res, T(neg_inf)));
    EXPECT_EQ(1, func(&res, T(0.1)));
    EXPECT_EQ(1, func(&res, T(-0.1)));
    EXPECT_EQ(1, func(&res, T(-1.0)));
    EXPECT_EQ(1, func(&res, T(0x42.1p+0)));
    EXPECT_EQ(1, func(&res, T(-0x42.1p+0)));

    FPBits nan_payload_bits = FPBits::one();
    nan_payload_bits.set_biased_exponent(FPBits::FRACTION_LEN - 1 +
                                         FPBits::EXP_BIAS);
    T nan_payload = nan_payload_bits.get_val();
    EXPECT_EQ(1, func(&res, nan_payload));
  }

  void testValidPayloads(SetPayloadFunc func) {
    T res;

    EXPECT_EQ(0, func(&res, T(0.0)));
    EXPECT_TRUE(FPBits(res).is_quiet_nan());
    EXPECT_EQ(FPBits(aNaN).uintval(), FPBits(res).uintval());

    EXPECT_EQ(0, func(&res, T(1.0)));
    EXPECT_TRUE(FPBits(res).is_quiet_nan());
    EXPECT_EQ(FPBits::quiet_nan(Sign::POS, 1).uintval(), FPBits(res).uintval());

    EXPECT_EQ(0, func(&res, T(0x42.0p+0)));
    EXPECT_TRUE(FPBits(res).is_quiet_nan());
    EXPECT_EQ(FPBits::quiet_nan(Sign::POS, 0x42).uintval(),
              FPBits(res).uintval());

    EXPECT_EQ(0, func(&res, T(0x123.0p+0)));
    EXPECT_TRUE(FPBits(res).is_quiet_nan());
    EXPECT_EQ(FPBits::quiet_nan(Sign::POS, 0x123).uintval(),
              FPBits(res).uintval());

    // The following code is creating a NaN payload manually to prevent a
    // conversion from BigInt to float128.
    FPBits nan_payload_bits = FPBits::one();
    nan_payload_bits.set_biased_exponent(FPBits::FRACTION_LEN - 2 +
                                         FPBits::EXP_BIAS);
    nan_payload_bits.set_mantissa(FPBits::SIG_MASK - 3);
    T nan_payload = nan_payload_bits.get_val();

    EXPECT_EQ(0, func(&res, nan_payload));
    EXPECT_TRUE(FPBits(res).is_quiet_nan());
    EXPECT_EQ(
        FPBits::quiet_nan(Sign::POS, FPBits::FRACTION_MASK >> 1).uintval(),
        FPBits(res).uintval());
  }
};

#define LIST_SETPAYLOAD_TESTS(T, func)                                         \
  using LlvmLibcSetPayloadTest = SetPayloadTestTemplate<T>;                    \
  TEST_F(LlvmLibcSetPayloadTest, InvalidPayloads) {                            \
    testInvalidPayloads(&func);                                                \
  }                                                                            \
  TEST_F(LlvmLibcSetPayloadTest, ValidPayloads) { testValidPayloads(&func); }

#endif // LIBC_TEST_SRC_MATH_SMOKE_SETPAYLOADTEST_H
