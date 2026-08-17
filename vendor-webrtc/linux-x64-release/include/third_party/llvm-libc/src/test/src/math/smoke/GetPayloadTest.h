//===-- Utility class to test different flavors of getpayload ---*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBC_TEST_SRC_MATH_SMOKE_GETPAYLOADTEST_H
#define LIBC_TEST_SRC_MATH_SMOKE_GETPAYLOADTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

template <typename T>
class GetPayloadTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*GetPayloadFunc)(const T *);

  T funcWrapper(GetPayloadFunc func, T x) { return func(&x); }

  void testNonNaNs(GetPayloadFunc func) {
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, T(0.0)));
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, T(-0.0)));
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, T(0.1)));
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, T(-0.1)));
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, T(123.38)));
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, T(-123.38)));
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, inf));
    EXPECT_FP_EQ(T(-1.0), funcWrapper(func, neg_inf));
  }

  void testNaNs(GetPayloadFunc func) {
    EXPECT_FP_EQ(T(0.0), funcWrapper(func, aNaN));
    EXPECT_FP_EQ(T(0.0), funcWrapper(func, neg_aNaN));

    // Essentially this:
    //   T default_snan_payload = StorageType(1) << (FPBits::FRACTION_LEN - 2);
    // but supports StorageType being a BigInt.
    FPBits default_snan_payload_bits = FPBits::one();
    default_snan_payload_bits.set_biased_exponent(FPBits::FRACTION_LEN - 2 +
                                                  FPBits::EXP_BIAS);
    T default_snan_payload = default_snan_payload_bits.get_val();

    EXPECT_FP_EQ(default_snan_payload, funcWrapper(func, sNaN));
    EXPECT_FP_EQ(default_snan_payload, funcWrapper(func, neg_sNaN));

    T qnan_42 = FPBits::quiet_nan(Sign::POS, 0x42).get_val();
    T neg_qnan_42 = FPBits::quiet_nan(Sign::NEG, 0x42).get_val();
    T snan_42 = FPBits::signaling_nan(Sign::POS, 0x42).get_val();
    T neg_snan_42 = FPBits::signaling_nan(Sign::NEG, 0x42).get_val();
    EXPECT_FP_EQ(T(0x42.0p+0), funcWrapper(func, qnan_42));
    EXPECT_FP_EQ(T(0x42.0p+0), funcWrapper(func, neg_qnan_42));
    EXPECT_FP_EQ(T(0x42.0p+0), funcWrapper(func, snan_42));
    EXPECT_FP_EQ(T(0x42.0p+0), funcWrapper(func, neg_snan_42));

    T qnan_123 = FPBits::quiet_nan(Sign::POS, 0x123).get_val();
    T neg_qnan_123 = FPBits::quiet_nan(Sign::NEG, 0x123).get_val();
    T snan_123 = FPBits::signaling_nan(Sign::POS, 0x123).get_val();
    T neg_snan_123 = FPBits::signaling_nan(Sign::NEG, 0x123).get_val();
    EXPECT_FP_EQ(T(0x123.0p+0), funcWrapper(func, qnan_123));
    EXPECT_FP_EQ(T(0x123.0p+0), funcWrapper(func, neg_qnan_123));
    EXPECT_FP_EQ(T(0x123.0p+0), funcWrapper(func, snan_123));
    EXPECT_FP_EQ(T(0x123.0p+0), funcWrapper(func, neg_snan_123));
  }
};

#define LIST_GETPAYLOAD_TESTS(T, func)                                         \
  using LlvmLibcGetPayloadTest = GetPayloadTestTemplate<T>;                    \
  TEST_F(LlvmLibcGetPayloadTest, NonNaNs) { testNonNaNs(&func); }              \
  TEST_F(LlvmLibcGetPayloadTest, NaNs) { testNaNs(&func); }

#endif // LIBC_TEST_SRC_MATH_SMOKE_GETPAYLOADTEST_H
