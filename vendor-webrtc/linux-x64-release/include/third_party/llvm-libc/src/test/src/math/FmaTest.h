//===-- Utility class to test different flavors of fma --------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_FMATEST_H
#define LLVM_LIBC_TEST_SRC_MATH_FMATEST_H

#include "src/stdlib/rand.h"
#include "src/stdlib/srand.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename OutType, typename InType = OutType>
class FmaTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  struct OutConstants {
    DECLARE_SPECIAL_CONSTANTS(OutType)
  };

  struct InConstants {
    DECLARE_SPECIAL_CONSTANTS(InType)
  };

  using OutFPBits = typename OutConstants::FPBits;
  using OutStorageType = typename OutConstants::StorageType;
  using InFPBits = typename InConstants::FPBits;
  using InStorageType = typename InConstants::StorageType;

  static constexpr OutStorageType OUT_MIN_NORMAL_U =
      OutFPBits::min_normal().uintval();
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
    for (InStorageType i = 0; i < sizeof(InStorageType) / 2; ++i) {
      bits = (bits << 2) + static_cast<uint16_t>(LIBC_NAMESPACE::rand());
    }
    return bits;
  }

public:
  using FmaFunc = OutType (*)(InType, InType, InType);

  void test_subnormal_range(FmaFunc func) {
    constexpr InStorageType COUNT = 100'001;
    constexpr InStorageType RAW_STEP =
        (IN_MAX_SUBNORMAL_U - IN_MIN_SUBNORMAL_U) / COUNT;
    constexpr InStorageType STEP = (RAW_STEP == 0 ? 1 : RAW_STEP);
    LIBC_NAMESPACE::srand(1);
    for (InStorageType v = IN_MIN_SUBNORMAL_U, w = IN_MAX_SUBNORMAL_U;
         v <= IN_MAX_SUBNORMAL_U && w >= IN_MIN_SUBNORMAL_U;
         v += STEP, w -= STEP) {
      InType x = InFPBits(get_random_bit_pattern()).get_val();
      InType y = InFPBits(v).get_val();
      InType z = InFPBits(w).get_val();
      mpfr::TernaryInput<InType> input{x, y, z};
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Fma, input, func(x, y, z),
                                     0.5);
    }
  }

  void test_normal_range(FmaFunc func) {
    constexpr InStorageType COUNT = 100'001;
    constexpr InStorageType RAW_STEP =
        (IN_MAX_NORMAL_U - IN_MIN_NORMAL_U) / COUNT;
    constexpr InStorageType STEP = (RAW_STEP == 0 ? 1 : RAW_STEP);
    LIBC_NAMESPACE::srand(1);
    for (InStorageType v = IN_MIN_NORMAL_U, w = IN_MAX_NORMAL_U;
         v <= IN_MAX_NORMAL_U && w >= IN_MIN_NORMAL_U; v += STEP, w -= STEP) {
      InType x = InFPBits(v).get_val();
      InType y = InFPBits(w).get_val();
      InType z = InFPBits(get_random_bit_pattern()).get_val();
      mpfr::TernaryInput<InType> input{x, y, z};
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Fma, input, func(x, y, z),
                                     0.5);
    }
  }
};

#define LIST_FMA_TESTS(T, func)                                                \
  using LlvmLibcFmaTest = FmaTestTemplate<T>;                                  \
  TEST_F(LlvmLibcFmaTest, SubnormalRange) { test_subnormal_range(&func); }     \
  TEST_F(LlvmLibcFmaTest, NormalRange) { test_normal_range(&func); }

#define LIST_NARROWING_FMA_TESTS(OutType, InType, func)                        \
  using LlvmLibcFmaTest = FmaTestTemplate<OutType, InType>;                    \
  TEST_F(LlvmLibcFmaTest, SubnormalRange) { test_subnormal_range(&func); }     \
  TEST_F(LlvmLibcFmaTest, NormalRange) { test_normal_range(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_FMATEST_H
