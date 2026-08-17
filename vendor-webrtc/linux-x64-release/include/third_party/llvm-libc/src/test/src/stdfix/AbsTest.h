//===-- Utility class to test fixed-point abs -------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/Test.h"

#include "src/__support/fixed_point/fx_rep.h"

template <typename T> class AbsTest : public LIBC_NAMESPACE::testing::Test {

  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<T>;
  static constexpr T zero = FXRep::ZERO();
  static constexpr T min = FXRep::MIN();
  static constexpr T max = FXRep::MAX();
  static constexpr T half = static_cast<T>(0.5);
  static constexpr T neg_half = static_cast<T>(-0.5);

public:
  typedef T (*AbsFunc)(T);

  void testSpecialNumbers(AbsFunc func) {
    EXPECT_EQ(zero, func(zero));
    EXPECT_EQ(max, func(min));
    EXPECT_EQ(max, func(max));
    EXPECT_EQ(half, func(half));
    EXPECT_EQ(half, func(neg_half));
  }
};

#define LIST_ABS_TESTS(T, func)                                                \
  using LlvmLibcAbsTest = AbsTest<T>;                                          \
  TEST_F(LlvmLibcAbsTest, SpecialNumbers) { testSpecialNumbers(&func); }       \
  static_assert(true, "Require semicolon.")
