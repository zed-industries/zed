//===-- Utility class to test fixed-point round -----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/Test.h"

#include "src/__support/fixed_point/fx_rep.h"

template <typename T> class RoundTest : public LIBC_NAMESPACE::testing::Test {

  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<T>;
  static constexpr T zero = FXRep::ZERO();
  static constexpr T min = FXRep::MIN();
  static constexpr T max = FXRep::MAX();
  static constexpr T half = static_cast<T>(0.5);
  static constexpr T neg_half = static_cast<T>(-0.5);
  static constexpr T one =
      (FXRep::INTEGRAL_LEN > 0) ? static_cast<T>(1) : FXRep::MAX();
  static constexpr T neg_one = static_cast<T>(-1);
  static constexpr T eps = FXRep::EPS();

public:
  typedef T (*RoundFunc)(T, int);

  void testSpecialNumbers(RoundFunc func) {
    EXPECT_EQ(zero, func(zero, FXRep::FRACTION_LEN - 5));
    EXPECT_EQ(min, func(min, 0));
    EXPECT_EQ(max, func(max, FXRep::FRACTION_LEN));

    EXPECT_EQ(one, func(half, 0));
    EXPECT_EQ(half, func(half, 1));
    EXPECT_EQ(half, func(half, FXRep::FRACTION_LEN));
    EXPECT_EQ(one, func(half + eps, 0));
    EXPECT_EQ(half, func(half + eps, 1));
    EXPECT_EQ(half, func(half + eps, 2));
    EXPECT_EQ(zero, func(half - eps, 0));
    EXPECT_EQ(half, func(half - eps, 1));
    EXPECT_EQ(half, func(half - eps, 2));
    EXPECT_EQ(eps, func(eps, FXRep::FRACTION_LEN + 10));
    EXPECT_EQ(eps << 1, func(eps, FXRep::FRACTION_LEN - 1));
    EXPECT_EQ(zero, func(eps, FXRep::FRACTION_LEN - 2));

    if constexpr (FXRep::SIGN_LEN) {
      EXPECT_EQ(zero, func(neg_half, 0));
      EXPECT_EQ(neg_half, func(neg_half, 1));
      EXPECT_EQ(neg_half, func(neg_half, 3));
      EXPECT_EQ(zero, func(neg_half + eps, 0));
      EXPECT_EQ(neg_half, func(neg_half + eps, 1));
      EXPECT_EQ(neg_half, func(neg_half + eps, 2));
      EXPECT_EQ(neg_one, func(neg_half - eps, 0));
      EXPECT_EQ(neg_half, func(neg_half - eps, 1));
      EXPECT_EQ(neg_half, func(neg_half - eps, 2));
      EXPECT_EQ(-eps, func(-eps, FXRep::FRACTION_LEN + 10));
    }
  }
};

#define LIST_ROUND_TESTS(T, func)                                              \
  using LlvmLibcRoundTest = RoundTest<T>;                                      \
  TEST_F(LlvmLibcRoundTest, SpecialNumbers) { testSpecialNumbers(&func); }     \
  static_assert(true, "Require semicolon.")
