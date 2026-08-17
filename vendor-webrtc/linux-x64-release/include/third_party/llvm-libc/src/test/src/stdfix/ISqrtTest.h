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
#include "src/__support/FPUtil/sqrt.h"
#include "src/__support/fixed_point/fx_rep.h"
#include "src/__support/fixed_point/sqrt.h"

template <typename T> class ISqrtTest : public LIBC_NAMESPACE::testing::Test {

  using OutType =
      typename LIBC_NAMESPACE::fixed_point::internal::SqrtConfig<T>::OutType;
  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<OutType>;
  static constexpr OutType zero = FXRep::ZERO();
  static constexpr OutType one = static_cast<OutType>(1);
  static constexpr OutType eps = FXRep::EPS();

public:
  typedef OutType (*SqrtFunc)(T);

  void testSpecificInput(T input, OutType result, double expected,
                         double tolerance) {
    double y_d = static_cast<double>(result);
    double errors = LIBC_NAMESPACE::fputil::abs((y_d / expected) - 1.0);
    if (errors > tolerance) {
      // Print out the failure input and output.
      EXPECT_EQ(input, T(0));
      EXPECT_EQ(result, zero);
    }
    ASSERT_TRUE(errors <= tolerance);
  }

  void testSpecialNumbers(SqrtFunc func) {
    EXPECT_EQ(zero, func(T(0)));

    EXPECT_EQ(one, func(T(1)));
    EXPECT_EQ(static_cast<OutType>(2.0), func(T(4)));
    EXPECT_EQ(static_cast<OutType>(4.0), func(T(16)));
    EXPECT_EQ(static_cast<OutType>(16.0), func(T(256)));

    constexpr int COUNT = 255;
    constexpr double ERR = 3.0 * static_cast<double>(eps);
    double x_d = 0.0;
    T x = 0;
    for (int i = 0; i < COUNT; ++i) {
      x_d += 1.0;
      ++x;
      OutType result = func(x);
      double expected = LIBC_NAMESPACE::fputil::sqrt<double>(x_d);
      testSpecificInput(x, result, expected, ERR);
    }
  }
};

#define LIST_ISQRT_TESTS(Name, T, func)                                        \
  using LlvmLibcISqrt##Name##Test = ISqrtTest<T>;                              \
  TEST_F(LlvmLibcISqrt##Name##Test, SpecialNumbers) {                          \
    testSpecialNumbers(&func);                                                 \
  }                                                                            \
  static_assert(true, "Require semicolon.")
