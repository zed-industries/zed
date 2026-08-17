//===-- Utility class to test different flavors of float mul ----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_MULTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_MULTEST_H

#include "src/stdlib/rand.h"
#include "src/stdlib/srand.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename OutType, typename InType>
class MulTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  struct InConstants {
    DECLARE_SPECIAL_CONSTANTS(InType)
  };

  using InFPBits = typename InConstants::FPBits;
  using InStorageType = typename InConstants::StorageType;

  static constexpr InStorageType IN_MAX_NORMAL_U =
      InFPBits::max_normal().uintval();
  static constexpr InStorageType IN_MIN_NORMAL_U =
      InFPBits::min_normal().uintval();
  static constexpr InStorageType IN_MAX_SUBNORMAL_U =
      InFPBits::max_subnormal().uintval();
  static constexpr InStorageType IN_MIN_SUBNORMAL_U =
      InFPBits::min_subnormal().uintval();

  InStorageType get_random_bit_pattern() {
    InStorageType bits{0};
    for (InStorageType i = 0; i < sizeof(InStorageType) / 2; ++i)
      bits = (bits << 2) + static_cast<uint16_t>(LIBC_NAMESPACE::rand());
    return bits;
  }

public:
  using MulFunc = OutType (*)(InType, InType);

  void test_subnormal_range(MulFunc func) {
    constexpr InStorageType COUNT = 10'001;
    constexpr InStorageType STEP =
        (IN_MAX_SUBNORMAL_U - IN_MIN_SUBNORMAL_U) / COUNT;
    LIBC_NAMESPACE::srand(1);
    for (int signs = 0; signs < 4; signs++) {
      for (InStorageType i = 0, v = 0; i <= COUNT; ++i, v += STEP) {
        InType x = InFPBits(get_random_bit_pattern()).get_val();
        InType y = InFPBits(v).get_val();
        if ((signs & 1) != 0)
          x = -x;
        if ((signs & 2) != 0)
          y = -y;
        mpfr::BinaryInput<InType> input{x, y};
        EXPECT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Mul, input, func(x, y),
                                       0.5);
      }
    }
  }

  void test_normal_range(MulFunc func) {
    constexpr InStorageType COUNT = 10'001;
    constexpr InStorageType STEP = (IN_MAX_NORMAL_U - IN_MIN_NORMAL_U) / COUNT;
    LIBC_NAMESPACE::srand(1);
    for (int signs = 0; signs < 4; signs++) {
      for (InStorageType i = 0, v = 0; i <= COUNT; ++i, v += STEP) {
        InType x = InFPBits(get_random_bit_pattern()).get_val();
        InType y = InFPBits(v).get_val();
        if ((signs & 1) != 0)
          x = -x;
        if ((signs & 2) != 0)
          y = -y;
        mpfr::BinaryInput<InType> input{x, y};
        EXPECT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Mul, input, func(x, y),
                                       0.5);
      }
    }
  }
};

#define LIST_MUL_TESTS(OutType, InType, func)                                  \
  using LlvmLibcMulTest = MulTest<OutType, InType>;                            \
  TEST_F(LlvmLibcMulTest, SubnormalRange) { test_subnormal_range(&func); }     \
  TEST_F(LlvmLibcMulTest, NormalRange) { test_normal_range(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_MULTEST_H
