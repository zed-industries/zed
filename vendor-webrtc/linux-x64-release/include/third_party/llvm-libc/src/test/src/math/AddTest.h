//===-- Utility class to test different flavors of float add ----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_ADDTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_ADDTEST_H

#include "src/__support/CPP/algorithm.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename OutType, typename InType>
class AddTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

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
  using AddFunc = OutType (*)(InType, InType);

  void test_subnormal_range(AddFunc func) {
    constexpr int COUNT = 100'001;
    constexpr InStorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<InStorageType>((IN_MAX_SUBNORMAL_U - IN_MIN_SUBNORMAL_U) /
                                   COUNT),
        InStorageType(1));
    for (InStorageType i = IN_MIN_SUBNORMAL_U; i <= IN_MAX_SUBNORMAL_U;
         i += STEP) {
      InType x = InFPBits(i).get_val();
      InType y = InFPBits(static_cast<InStorageType>(IN_MAX_SUBNORMAL_U - i))
                     .get_val();
      mpfr::BinaryInput<InType> input{x, y};
      EXPECT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Add, input, func(x, y),
                                     0.5);
    }
  }

  void test_normal_range(AddFunc func) {
    constexpr int COUNT = 100'001;
    constexpr InStorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<InStorageType>((IN_MAX_NORMAL_U - IN_MIN_NORMAL_U) / COUNT),
        InStorageType(1));
    for (InStorageType i = IN_MIN_NORMAL_U; i <= IN_MAX_NORMAL_U; i += STEP) {
      InType x = InFPBits(i).get_val();
      InType y =
          InFPBits(static_cast<InStorageType>(IN_MAX_NORMAL_U - i)).get_val();
      mpfr::BinaryInput<InType> input{x, y};
      EXPECT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Add, input, func(x, y),
                                     0.5);
    }
  }
};

#define LIST_ADD_TESTS(OutType, InType, func)                                  \
  using LlvmLibcAddTest = AddTest<OutType, InType>;                            \
  TEST_F(LlvmLibcAddTest, SubnormalRange) { test_subnormal_range(&func); }     \
  TEST_F(LlvmLibcAddTest, NormalRange) { test_normal_range(&func); }

#define LIST_ADD_SAME_TYPE_TESTS(suffix, OutType, InType, func)                \
  using LlvmLibcAddTest##suffix = AddTest<OutType, InType>;                    \
  TEST_F(LlvmLibcAddTest##suffix, SubnormalRange) {                            \
    test_subnormal_range(&func);                                               \
  }                                                                            \
  TEST_F(LlvmLibcAddTest##suffix, NormalRange) { test_normal_range(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_ADDTEST_H
