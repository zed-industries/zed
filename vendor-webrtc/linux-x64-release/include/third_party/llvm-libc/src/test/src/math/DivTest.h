//===-- Utility class to test different flavors of float div ----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_DIVTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_DIVTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename OutType, typename InType>
class DivTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

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

public:
  using DivFunc = OutType (*)(InType, InType);

  void test_subnormal_range(DivFunc func) {
    constexpr InStorageType COUNT = 100'001;
    constexpr InStorageType STEP =
        (IN_MAX_SUBNORMAL_U - IN_MIN_SUBNORMAL_U) / COUNT;
    for (InStorageType i = 0, v = 0, w = IN_MAX_SUBNORMAL_U; i <= COUNT;
         ++i, v += STEP, w -= STEP) {
      InType x = InFPBits(v).get_val();
      InType y = InFPBits(w).get_val();
      mpfr::BinaryInput<InType> input{x, y};
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Div, input, func(x, y),
                                     0.5);
    }
  }

  void test_normal_range(DivFunc func) {
    constexpr InStorageType COUNT = 100'001;
    constexpr InStorageType STEP = (IN_MAX_NORMAL_U - IN_MIN_NORMAL_U) / COUNT;
    for (InStorageType i = 0, v = 0, w = IN_MAX_NORMAL_U; i <= COUNT;
         ++i, v += STEP, w -= STEP) {
      InType x = InFPBits(v).get_val();
      InType y = InFPBits(w).get_val();
      mpfr::BinaryInput<InType> input{x, y};
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Div, input, func(x, y),
                                     0.5);
    }
  }
};

#define LIST_DIV_TESTS(OutType, InType, func)                                  \
  using LlvmLibcDivTest = DivTest<OutType, InType>;                            \
  TEST_F(LlvmLibcDivTest, SubnormalRange) { test_subnormal_range(&func); }     \
  TEST_F(LlvmLibcDivTest, NormalRange) { test_normal_range(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_DIVTEST_H
