//===-- Utility class to test integer sqrt ----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "src/__support/CPP/bit.h"
#include "src/__support/FPUtil/BasicOperations.h"
#include "src/__support/fixed_point/fx_rep.h"
#include "src/__support/fixed_point/sqrt.h"

#include "src/math/exp.h"

template <typename T> class ExpTest : public LIBC_NAMESPACE::testing::Test {

  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<T>;
  static constexpr T zero = FXRep::ZERO();
  static constexpr T one = static_cast<T>(1);
  static constexpr T eps = FXRep::EPS();

public:
  typedef T (*ExpFunc)(T);

  void test_special_numbers(ExpFunc func) {
    EXPECT_EQ(one, func(T(0)));
    EXPECT_EQ(FXRep::MAX(), func(T(30)));
    EXPECT_EQ(zero, func(T(-30)));
  }

  void test_range_with_step(ExpFunc func, T step, bool rel_error) {
    constexpr int COUNT = 255;
    constexpr double ERR = 3.0 * static_cast<double>(eps);
    double x_d = 0.0;
    T x = step;
    for (int i = 0; i < COUNT; ++i) {
      x += step;
      x_d = static_cast<double>(x);
      double y_d = static_cast<double>(func(x));
      double result = LIBC_NAMESPACE::exp(x_d);
      double errors = rel_error
                          ? LIBC_NAMESPACE::fputil::abs((y_d / result) - 1.0)
                          : LIBC_NAMESPACE::fputil::abs(y_d - result);
      if (errors > ERR) {
        // Print out the failure input and output.
        EXPECT_EQ(x, T(0));
        EXPECT_EQ(func(x), zero);
      }
      ASSERT_TRUE(errors <= ERR);
    }
  }

  void test_positive_range(ExpFunc func) {
    test_range_with_step(func, T(0x1.0p-6), /*rel_error*/ true);
  }

  void test_negative_range(ExpFunc func) {
    test_range_with_step(func, T(-0x1.0p-6), /*rel_error*/ false);
  }
};

#define LIST_EXP_TESTS(Name, T, func)                                          \
  using LlvmLibcExp##Name##Test = ExpTest<T>;                                  \
  TEST_F(LlvmLibcExp##Name##Test, SpecialNumbers) {                            \
    test_special_numbers(&func);                                               \
  }                                                                            \
  TEST_F(LlvmLibcExp##Name##Test, PositiveRange) {                             \
    test_positive_range(&func);                                                \
  }                                                                            \
  TEST_F(LlvmLibcExp##Name##Test, NegativeRange) {                             \
    test_negative_range(&func);                                                \
  }                                                                            \
  static_assert(true, "Require semicolon.")
