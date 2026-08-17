//===-- Utility class to test different flavors of nextafter ----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_NEXTAFTERTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_NEXTAFTERTEST_H

#include "hdr/math_macros.h"
#include "src/__support/CPP/bit.h"
#include "src/__support/CPP/type_traits.h"
#include "src/__support/FPUtil/BasicOperations.h"
#include "src/__support/FPUtil/FPBits.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

template <typename T>
class NextAfterTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
  using StorageType = typename FPBits::StorageType;

  const T inf = FPBits::inf(Sign::POS).get_val();
  const T neg_inf = FPBits::inf(Sign::NEG).get_val();
  const T zero = FPBits::zero(Sign::POS).get_val();
  const T neg_zero = FPBits::zero(Sign::NEG).get_val();
  const T nan = FPBits::quiet_nan().get_val();

  const StorageType min_subnormal = FPBits::min_subnormal().uintval();
  const StorageType max_subnormal = FPBits::max_subnormal().uintval();
  const StorageType min_normal = FPBits::min_normal().uintval();
  const StorageType max_normal = FPBits::max_normal().uintval();

public:
  typedef T (*NextAfterFunc)(T, T);

  void testNaN(NextAfterFunc func) {
    ASSERT_FP_EQ(func(nan, 0), nan);
    ASSERT_FP_EQ(func(0, nan), nan);
  }

  void testBoundaries(NextAfterFunc func) {
    ASSERT_FP_EQ(func(zero, neg_zero), neg_zero);
    ASSERT_FP_EQ(func(neg_zero, zero), zero);

    // 'from' is zero|neg_zero.
    T x = zero;
    T result = func(x, T(1));
    StorageType expected_bits = 1;
    T expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    result = func(x, T(-1));
    expected_bits = FPBits::SIGN_MASK + 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    x = neg_zero;
    result = func(x, 1);
    expected_bits = 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    result = func(x, -1);
    expected_bits = FPBits::SIGN_MASK + 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    // 'from' is max subnormal value.
    x = LIBC_NAMESPACE::cpp::bit_cast<T>(max_subnormal);
    result = func(x, 1);
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(min_normal);
    ASSERT_FP_EQ(result, expected);

    result = func(x, 0);
    expected_bits = max_subnormal - 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    x = -x;

    result = func(x, -1);
    expected_bits = FPBits::SIGN_MASK + min_normal;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    result = func(x, 0);
    expected_bits = FPBits::SIGN_MASK + max_subnormal - 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    // 'from' is min subnormal value.
    x = LIBC_NAMESPACE::cpp::bit_cast<T>(min_subnormal);
    result = func(x, 1);
    expected_bits = min_subnormal + 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);
    ASSERT_FP_EQ(func(x, 0), zero);

    x = -x;
    result = func(x, -1);
    expected_bits = FPBits::SIGN_MASK + min_subnormal + 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);
    ASSERT_FP_EQ(func(x, 0), T(-0.0));

    // 'from' is min normal.
    x = LIBC_NAMESPACE::cpp::bit_cast<T>(min_normal);
    result = func(x, 0);
    expected_bits = max_subnormal;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    result = func(x, inf);
    expected_bits = min_normal + 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    x = -x;
    result = func(x, 0);
    expected_bits = FPBits::SIGN_MASK + max_subnormal;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    result = func(x, -inf);
    expected_bits = FPBits::SIGN_MASK + min_normal + 1;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);

    // 'from' is max normal and 'to' is infinity.
    x = LIBC_NAMESPACE::cpp::bit_cast<T>(max_normal);
    result = func(x, inf);
    ASSERT_FP_EQ(result, inf);

    result = func(-x, -inf);
    ASSERT_FP_EQ(result, -inf);

    // 'from' is infinity.
    x = inf;
    result = func(x, 0);
    expected_bits = max_normal;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);
    ASSERT_FP_EQ(func(x, inf), inf);

    x = neg_inf;
    result = func(x, 0);
    expected_bits = FPBits::SIGN_MASK + max_normal;
    expected = LIBC_NAMESPACE::cpp::bit_cast<T>(expected_bits);
    ASSERT_FP_EQ(result, expected);
    ASSERT_FP_EQ(func(x, neg_inf), neg_inf);

    // 'from' is a power of 2.
    x = T(32.0);
    result = func(x, 0);
    FPBits x_bits = FPBits(x);
    FPBits result_bits = FPBits(result);
    ASSERT_EQ(result_bits.get_biased_exponent(),
              uint16_t(x_bits.get_biased_exponent() - 1));
    ASSERT_EQ(result_bits.get_mantissa(), FPBits::FRACTION_MASK);

    result = func(x, T(33.0));
    result_bits = FPBits(result);
    ASSERT_EQ(result_bits.get_biased_exponent(), x_bits.get_biased_exponent());
    ASSERT_EQ(result_bits.get_mantissa(),
              x_bits.get_mantissa() + StorageType(1));

    x = -x;

    result = func(x, 0);
    result_bits = FPBits(result);
    ASSERT_EQ(result_bits.get_biased_exponent(),
              uint16_t(x_bits.get_biased_exponent() - 1));
    ASSERT_EQ(result_bits.get_mantissa(), FPBits::FRACTION_MASK);

    result = func(x, T(-33.0));
    result_bits = FPBits(result);
    ASSERT_EQ(result_bits.get_biased_exponent(), x_bits.get_biased_exponent());
    ASSERT_EQ(result_bits.get_mantissa(),
              x_bits.get_mantissa() + StorageType(1));
  }
};

#define LIST_NEXTAFTER_TESTS(T, func)                                          \
  using LlvmLibcNextAfterTest = NextAfterTestTemplate<T>;                      \
  TEST_F(LlvmLibcNextAfterTest, TestNaN) { testNaN(&func); }                   \
  TEST_F(LlvmLibcNextAfterTest, TestBoundaries) { testBoundaries(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_NEXTAFTERTEST_H
