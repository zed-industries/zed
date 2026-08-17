//===-- Utility class to test idivfx functions ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/Test.h"

#include "src/__support/fixed_point/fx_rep.h"
#include "src/__support/macros/sanitizer.h"

#include "hdr/signal_macros.h"

template <typename T, typename XType>
class IdivTest : public LIBC_NAMESPACE::testing::Test {

  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<T>;

  static constexpr T zero = FXRep::ZERO();
  static constexpr T max = FXRep::MAX();
  static constexpr T min = FXRep::MIN();
  static constexpr T one_half = FXRep::ONE_HALF();
  static constexpr T one_fourth = FXRep::ONE_FOURTH();

public:
  typedef XType (*IdivFunc)(T, T);

  void testSpecialNumbers(IdivFunc func) {
    constexpr bool is_signed = (FXRep::SIGN_LEN > 0);
    constexpr bool has_integral = (FXRep::INTEGRAL_LEN > 0);

    EXPECT_EQ(func(one_half, one_fourth), static_cast<XType>(2));
    EXPECT_EQ(func(one_half, one_half), static_cast<XType>(1));
    EXPECT_EQ(func(one_fourth, one_half), static_cast<XType>(0));
    EXPECT_EQ(func(0.75, 0.25), static_cast<XType>(3));
    EXPECT_EQ(func(0.625, 0.125), static_cast<XType>(5));

    if constexpr (is_signed) {
      EXPECT_EQ(func(min, one_half), static_cast<XType>(min) * 2);
    } else {
      EXPECT_EQ(func(min, one_half), static_cast<XType>(0));
    }

    if constexpr (has_integral && min <= 7 && max >= 5) {
      EXPECT_EQ(func(6.9, 4.2), static_cast<XType>(1));
      EXPECT_EQ(func(4.2, 6.9), static_cast<XType>(0));
      EXPECT_EQ(func(4.5, 2.2), static_cast<XType>(2));
      EXPECT_EQ(func(2.2, 1.1), static_cast<XType>(2));
      EXPECT_EQ(func(2.25, 1.0), static_cast<XType>(2));
      EXPECT_EQ(func(2.25, 3.0), static_cast<XType>(0));

      if constexpr (is_signed) {
        EXPECT_EQ(func(4.2, -6.9), static_cast<XType>(0));
        EXPECT_EQ(func(-6.9, 4.2), static_cast<XType>(-1));
        EXPECT_EQ(func(-2.5, 1.25), static_cast<XType>(-2));
        EXPECT_EQ(func(-2.25, 1.0), static_cast<XType>(-2));
        EXPECT_EQ(func(2.25, -3.0), static_cast<XType>(0));
      }
    }
  }

  void testInvalidNumbers(IdivFunc func) {
    constexpr bool has_integral = (FXRep::INTEGRAL_LEN > 0);

    EXPECT_DEATH([func] { func(0.5, 0.0); }, WITH_SIGNAL(SIGILL));
    if constexpr (has_integral) {
      EXPECT_DEATH([func] { func(2.5, 0.0); }, WITH_SIGNAL(SIGSEGV));
    }
  }
};

#if defined(LIBC_ADD_NULL_CHECKS) && !defined(LIBC_HAS_SANITIZER)
#define LIST_IDIV_TESTS(Name, T, XTYpe, func)                                  \
  using LlvmLibcIdiv##Name##Test = IdivTest<T, XType>;                         \
  TEST_F(LlvmLibcIdiv##Name##Test, InvalidNumbers) {                           \
    testInvalidNumbers(&func);                                                 \
  }                                                                            \
  TEST_F(LlvmLibcIdiv##Name##Test, SpecialNumbers) {                           \
    testSpecialNumbers(&func);                                                 \
  }                                                                            \
  static_assert(true, "Require semicolon.")
#else
#define LIST_IDIV_TESTS(Name, T, XType, func)                                  \
  using LlvmLibcIdiv##Name##Test = IdivTest<T, XType>;                         \
  TEST_F(LlvmLibcIdiv##Name##Test, SpecialNumbers) {                           \
    testSpecialNumbers(&func);                                                 \
  }                                                                            \
  static_assert(true, "Require semicolon.")
#endif // LIBC_HAS_ADDRESS_SANITIZER
