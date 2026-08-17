//===-- Utility class to test int to fixed point conversions ----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/Test.h"

#include "include/llvm-libc-types/stdfix-types.h"
#include "src/__support/CPP/bit.h"
#include "src/__support/fixed_point/fx_bits.h"

template <typename T, typename XType>
class FxBitsTest : public LIBC_NAMESPACE::testing::Test {
  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<T>;
  static constexpr T zero = FXRep::ZERO();
  static constexpr T min = FXRep::MIN();
  static constexpr T max = FXRep::MAX();
  static constexpr T half = static_cast<T>(0.5);
  static constexpr T quarter = static_cast<T>(0.25);
  static constexpr T one =
      (FXRep::INTEGRAL_LEN > 0) ? static_cast<T>(1) : FXRep::MAX();
  static constexpr T eps = FXRep::EPS();
  constexpr XType get_one_or_saturated_fraction() {
    if (FXRep::INTEGRAL_LEN > 0) {
      return static_cast<XType>(static_cast<XType>(0x1) << FXRep::FRACTION_LEN);
    } else {
      return static_cast<XType>(
          LIBC_NAMESPACE::mask_trailing_ones<typename FXRep::StorageType,
                                             FXRep::FRACTION_LEN>());
    }
  }

public:
  typedef T (*FxBitsFunc)(XType);

  void test_special_numbers(FxBitsFunc func) {
    EXPECT_EQ(zero, func(0));
    EXPECT_EQ(eps, func(0x1));
    // x.1000...
    EXPECT_EQ(half, func(static_cast<XType>(0x1) << (FXRep::FRACTION_LEN - 1)));
    // Occupy the bit to the left of the fixed point for Accum types
    // Saturate fraction portion for Fract types
    EXPECT_EQ(one, func(get_one_or_saturated_fraction()));
  }
};

#define LIST_FXBITS_TEST(Name, T, XType, func)                                 \
  using LlvmLibc##Name##BitsTest = FxBitsTest<T, XType>;                       \
  TEST_F(LlvmLibc##Name##BitsTest, SpecialNumbers) {                           \
    test_special_numbers(&func);                                               \
  }                                                                            \
  static_assert(true, "Require semicolon.")
