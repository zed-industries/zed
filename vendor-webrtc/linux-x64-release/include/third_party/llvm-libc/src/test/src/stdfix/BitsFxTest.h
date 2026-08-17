//===-- Utility class to test bitsfx functions ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/Test.h"

#include "src/__support/fixed_point/fx_rep.h"

template <typename T, typename XType>
class BitsFxTest : public LIBC_NAMESPACE::testing::Test {

  using FXRep = LIBC_NAMESPACE::fixed_point::FXRep<T>;
  static constexpr T zero = FXRep::ZERO();
  static constexpr T max = FXRep::MAX();
  static constexpr T min = FXRep::MIN();
  static constexpr T one_half = FXRep::ONE_HALF();
  static constexpr T one_fourth = FXRep::ONE_FOURTH();
  static constexpr T eps = FXRep::EPS();

  static constexpr T zero_point_six_eight_seven_five_t = 0.6875;

  static constexpr T negative_zero_point_six_eight_seven_five_t = -0.6875;

  // an arbitrarily chosen special number
  static constexpr T special_num_t = 10.71875;

  static constexpr T negative_special_num_t = -10.71875;

public:
  typedef XType (*BitsFxFunc)(T);

  void testSpecialNumbers(BitsFxFunc func) {
    EXPECT_EQ(static_cast<XType>(0), func(zero));
    EXPECT_EQ(static_cast<XType>(1ULL << (FXRep::FRACTION_LEN - 1)),
              func(one_half));
    EXPECT_EQ(static_cast<XType>(1ULL << (FXRep::FRACTION_LEN - 2)),
              func(one_fourth));
    EXPECT_EQ(static_cast<XType>(1), func(eps));

    // (0.6875)_10 = (0.1011)_2
    EXPECT_EQ(static_cast<XType>(11ULL << (FXRep::FRACTION_LEN - 4)),
              func(zero_point_six_eight_seven_five_t));

    if constexpr (FXRep::SIGN_LEN > 0)
      EXPECT_EQ(static_cast<XType>(-(11ULL << (FXRep::FRACTION_LEN - 4))),
                func(negative_zero_point_six_eight_seven_five_t));

    if constexpr (FXRep::INTEGRAL_LEN > 0) {
      constexpr size_t kMinFbits = 7;

      if (max >= 11 && FXRep::FRACTION_LEN >= kMinFbits) {
        // (10.71875)_10 = (1010.1011100)_2
        constexpr long long kExpected = 1372;
        EXPECT_EQ(
            static_cast<XType>(kExpected << (FXRep::FRACTION_LEN - kMinFbits)),
            func(special_num_t));
      }

      if constexpr (FXRep::SIGN_LEN > 0) {
        if (min <= -11 && FXRep::FRACTION_LEN >= kMinFbits) {
          // (-10.71875)_10 = (-1010.1011100)_2
          constexpr long long kExpected = -1372;
          EXPECT_EQ(static_cast<XType>(kExpected
                                       << (FXRep::FRACTION_LEN - kMinFbits)),
                    func(negative_special_num_t));
        }
      }
    }
  }
};

#define LIST_BITSFX_TESTS(Name, T, XType, func)                                \
  using LlvmLibcBits##Name##Test = BitsFxTest<T, XType>;                       \
  TEST_F(LlvmLibcBits##Name##Test, SpecialNumbers) {                           \
    testSpecialNumbers(&func);                                                 \
  }                                                                            \
  static_assert(true, "Require semicolon.")
