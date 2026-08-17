//===-- Utility class to test fixed-point sqrt ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/Test.h"

#include "src/__support/CPP/bit.h"
#include "src/__support/FPUtil/BasicOperations.h"
#include "src/__support/FPUtil/sqrt.h"
#include "src/__support/fixed_point/fx_rep.h"
#include "src/__support/fixed_point/sqrt.h"

template <typename T> class SqrtTest : public LIBC_NAMESPACE::testing::Test {

  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<T>;
  static constexpr T zero = FXRep::ZERO();
  static constexpr T min = FXRep::MIN();
  static constexpr T max = FXRep::MAX();
  static constexpr T half = static_cast<T>(0.5);
  static constexpr T quarter = static_cast<T>(0.25);
  static constexpr T one =
      (FXRep::INTEGRAL_LEN > 0) ? static_cast<T>(1) : FXRep::MAX();
  static constexpr T eps = FXRep::EPS();

public:
  typedef T (*SqrtFunc)(T);

  void testSpecialNumbers(SqrtFunc func) {
    EXPECT_EQ(zero, func(zero));
    EXPECT_EQ(half, func(quarter));

    if constexpr (FXRep::INTEGRAL_LEN) {
      EXPECT_EQ(one, func(one));
      EXPECT_EQ(static_cast<T>(2.0), func(static_cast<T>(4.0)));
    }

    using StorageType = typename FXRep::StorageType;

    constexpr size_t COUNT = 255;
    constexpr StorageType STEP =
        StorageType(~StorageType(0)) / static_cast<StorageType>(COUNT);
    constexpr double ERR = 3.0 * static_cast<double>(eps);
    StorageType x = 0;
    for (size_t i = 0; i < COUNT; ++i, x += STEP) {
      T v = LIBC_NAMESPACE::cpp::bit_cast<T>(x);
      double v_d = static_cast<double>(v);
      double errors = LIBC_NAMESPACE::fputil::abs(
          static_cast<double>(func(v)) -
          LIBC_NAMESPACE::fputil::sqrt<double>(v_d));
      if (errors > ERR) {
        // Print out the failure input and output.
        EXPECT_EQ(v, zero);
        EXPECT_EQ(func(v), zero);
      }
      ASSERT_TRUE(errors <= ERR);
    }
  }
};

#define LIST_SQRT_TESTS(T, func)                                               \
  using LlvmLibcSqrtTest = SqrtTest<T>;                                        \
  TEST_F(LlvmLibcSqrtTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  static_assert(true, "Require semicolon.")
