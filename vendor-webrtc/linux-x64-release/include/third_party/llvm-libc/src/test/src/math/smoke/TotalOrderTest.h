//===-- Utility class to test different flavors of totalorder ---*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBC_TEST_SRC_MATH_SMOKE_TOTALORDERTEST_H
#define LIBC_TEST_SRC_MATH_SMOKE_TOTALORDERTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

template <typename T>
class TotalOrderTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef int (*TotalOrderFunc)(const T *, const T *);

  bool funcWrapper(TotalOrderFunc func, T x, T y) { return func(&x, &y) != 0; }

  void testXLesserThanY(TotalOrderFunc func) {
    EXPECT_TRUE(funcWrapper(func, neg_inf, inf));

    EXPECT_TRUE(funcWrapper(func, T(0.0), T(0.1)));
    EXPECT_TRUE(funcWrapper(func, T(0.0), T(123.38)));

    EXPECT_TRUE(funcWrapper(func, T(-0.1), T(0.0)));
    EXPECT_TRUE(funcWrapper(func, T(-123.38), T(0.0)));

    EXPECT_TRUE(funcWrapper(func, T(-0.1), T(0.1)));
    EXPECT_TRUE(funcWrapper(func, T(-123.38), T(123.38)));
  }

  void testXGreaterThanY(TotalOrderFunc func) {
    EXPECT_FALSE(funcWrapper(func, inf, neg_inf));

    EXPECT_FALSE(funcWrapper(func, T(0.0), T(-0.1)));
    EXPECT_FALSE(funcWrapper(func, T(0.0), T(-123.38)));

    EXPECT_FALSE(funcWrapper(func, T(0.1), T(0.0)));
    EXPECT_FALSE(funcWrapper(func, T(123.38), T(0.0)));

    EXPECT_FALSE(funcWrapper(func, T(0.1), T(-0.1)));
    EXPECT_FALSE(funcWrapper(func, T(123.38), T(-123.38)));
  }

  void testXEqualToY(TotalOrderFunc func) {
    EXPECT_TRUE(funcWrapper(func, inf, inf));
    EXPECT_TRUE(funcWrapper(func, neg_inf, neg_inf));

    EXPECT_TRUE(funcWrapper(func, T(-0.0), T(0.0)));
    EXPECT_FALSE(funcWrapper(func, T(0.0), T(-0.0)));

    EXPECT_TRUE(funcWrapper(func, T(0.0), T(0.0)));
    EXPECT_TRUE(funcWrapper(func, T(-0.0), T(-0.0)));
    EXPECT_TRUE(funcWrapper(func, T(0.1), T(0.1)));
    EXPECT_TRUE(funcWrapper(func, T(-0.1), T(-0.1)));
    EXPECT_TRUE(funcWrapper(func, T(123.38), T(123.38)));
    EXPECT_TRUE(funcWrapper(func, T(-123.38), T(-123.38)));
  }

  void testSingleNaN(TotalOrderFunc func) {
    EXPECT_TRUE(funcWrapper(func, neg_aNaN, T(0.0)));
    EXPECT_TRUE(funcWrapper(func, neg_aNaN, T(0.1)));
    EXPECT_TRUE(funcWrapper(func, neg_aNaN, T(123.38)));

    EXPECT_FALSE(funcWrapper(func, T(0.0), neg_aNaN));
    EXPECT_FALSE(funcWrapper(func, T(0.1), neg_aNaN));
    EXPECT_FALSE(funcWrapper(func, T(123.38), neg_aNaN));

    EXPECT_TRUE(funcWrapper(func, T(0.0), aNaN));
    EXPECT_TRUE(funcWrapper(func, T(0.1), aNaN));
    EXPECT_TRUE(funcWrapper(func, T(123.38), aNaN));

    EXPECT_FALSE(funcWrapper(func, aNaN, T(0.0)));
    EXPECT_FALSE(funcWrapper(func, aNaN, T(0.1)));
    EXPECT_FALSE(funcWrapper(func, aNaN, T(123.38)));
  }

  void testNaNSigns(TotalOrderFunc func) {
    EXPECT_TRUE(funcWrapper(func, neg_aNaN, aNaN));
    EXPECT_TRUE(funcWrapper(func, neg_aNaN, sNaN));
    EXPECT_TRUE(funcWrapper(func, neg_sNaN, aNaN));
    EXPECT_TRUE(funcWrapper(func, neg_sNaN, sNaN));

    EXPECT_FALSE(funcWrapper(func, aNaN, neg_aNaN));
    EXPECT_FALSE(funcWrapper(func, aNaN, neg_sNaN));
    EXPECT_FALSE(funcWrapper(func, sNaN, neg_aNaN));
    EXPECT_FALSE(funcWrapper(func, sNaN, neg_sNaN));
  }

  void testQuietVsSignalingNaN(TotalOrderFunc func) {
    EXPECT_TRUE(funcWrapper(func, neg_aNaN, neg_sNaN));
    EXPECT_FALSE(funcWrapper(func, neg_sNaN, neg_aNaN));
    EXPECT_TRUE(funcWrapper(func, sNaN, aNaN));
    EXPECT_FALSE(funcWrapper(func, aNaN, sNaN));
  }

  void testNaNPayloads(TotalOrderFunc func) {
    T qnan_0x42 = FPBits::quiet_nan(Sign::POS, 0x42).get_val();
    T neg_qnan_0x42 = FPBits::quiet_nan(Sign::NEG, 0x42).get_val();
    T snan_0x42 = FPBits::signaling_nan(Sign::POS, 0x42).get_val();
    T neg_snan_0x42 = FPBits::signaling_nan(Sign::NEG, 0x42).get_val();

    EXPECT_TRUE(funcWrapper(func, aNaN, aNaN));
    EXPECT_TRUE(funcWrapper(func, sNaN, sNaN));
    EXPECT_TRUE(funcWrapper(func, aNaN, qnan_0x42));
    EXPECT_FALSE(funcWrapper(func, sNaN, snan_0x42));
    EXPECT_FALSE(funcWrapper(func, qnan_0x42, aNaN));
    EXPECT_TRUE(funcWrapper(func, snan_0x42, sNaN));

    EXPECT_TRUE(funcWrapper(func, neg_aNaN, neg_aNaN));
    EXPECT_TRUE(funcWrapper(func, neg_sNaN, neg_sNaN));
    EXPECT_FALSE(funcWrapper(func, neg_aNaN, neg_qnan_0x42));
    EXPECT_TRUE(funcWrapper(func, neg_sNaN, neg_snan_0x42));
    EXPECT_TRUE(funcWrapper(func, neg_qnan_0x42, neg_aNaN));
    EXPECT_FALSE(funcWrapper(func, neg_snan_0x42, neg_sNaN));
  }
};

#define LIST_TOTALORDER_TESTS(T, func)                                         \
  using LlvmLibcTotalOrderTest = TotalOrderTestTemplate<T>;                    \
  TEST_F(LlvmLibcTotalOrderTest, XLesserThanY) { testXLesserThanY(&func); }    \
  TEST_F(LlvmLibcTotalOrderTest, XGreaterThanY) { testXGreaterThanY(&func); }  \
  TEST_F(LlvmLibcTotalOrderTest, XEqualToY) { testXEqualToY(&func); }          \
  TEST_F(LlvmLibcTotalOrderTest, SingleNaN) { testSingleNaN(&func); }          \
  TEST_F(LlvmLibcTotalOrderTest, NaNSigns) { testNaNSigns(&func); }            \
  TEST_F(LlvmLibcTotalOrderTest, QuietVsSignalingNaN) {                        \
    testQuietVsSignalingNaN(&func);                                            \
  }                                                                            \
  TEST_F(LlvmLibcTotalOrderTest, NaNPayloads) { testNaNPayloads(&func); }

#endif // LIBC_TEST_SRC_MATH_SMOKE_TOTALORDERTEST_H
