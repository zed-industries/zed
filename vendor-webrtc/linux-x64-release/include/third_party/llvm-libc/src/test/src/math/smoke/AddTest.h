//===-- Utility class to test different flavors of float add ----*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_SMOKE_ADDTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_SMOKE_ADDTEST_H

#include "hdr/errno_macros.h"
#include "hdr/fenv_macros.h"
#include "src/__support/macros/properties/os.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::Sign;

template <typename OutType, typename InType>
class AddTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(OutType)

  struct InConstants {
    DECLARE_SPECIAL_CONSTANTS(InType)
  };

  using InFPBits = typename InConstants::FPBits;
  using InStorageType = typename InConstants::StorageType;

  InConstants in;

public:
  using AddFunc = OutType (*)(InType, InType);

  void test_special_numbers(AddFunc func) {
    EXPECT_FP_IS_NAN(func(in.aNaN, in.aNaN));
    EXPECT_FP_IS_NAN_WITH_EXCEPTION(func(in.sNaN, in.sNaN), FE_INVALID);

    InType qnan_42 = InFPBits::quiet_nan(Sign::POS, 0x42).get_val();
    EXPECT_FP_IS_NAN(func(qnan_42, in.zero));
    EXPECT_FP_IS_NAN(func(in.zero, qnan_42));

    EXPECT_FP_EQ(inf, func(in.inf, in.zero));
    EXPECT_FP_EQ(neg_inf, func(in.neg_inf, in.zero));
    EXPECT_FP_EQ(inf, func(in.inf, in.neg_zero));
    EXPECT_FP_EQ(neg_inf, func(in.neg_inf, in.neg_zero));
  }

  void test_invalid_operations(AddFunc func) {
    EXPECT_FP_IS_NAN_WITH_EXCEPTION(func(in.inf, in.neg_inf), FE_INVALID);
    EXPECT_FP_IS_NAN_WITH_EXCEPTION(func(in.neg_inf, in.inf), FE_INVALID);
  }

  void test_range_errors(AddFunc func) {
#ifndef LIBC_TARGET_OS_IS_WINDOWS
    using namespace LIBC_NAMESPACE::fputil::testing;

    if (LIBC_NAMESPACE::fputil::get_fp_type<OutType>() ==
        LIBC_NAMESPACE::fputil::get_fp_type<InType>())
      return;

    if (ForceRoundingMode r(RoundingMode::Nearest); r.success) {
      EXPECT_FP_EQ_WITH_EXCEPTION(inf, func(in.max_normal, in.max_normal),
                                  FE_OVERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
      EXPECT_FP_EQ_WITH_EXCEPTION(-inf,
                                  func(in.neg_max_normal, in.neg_max_normal),
                                  FE_OVERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);

      EXPECT_FP_EQ_WITH_EXCEPTION(zero, func(in.min_denormal, in.min_denormal),
                                  FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
      EXPECT_FP_EQ_WITH_EXCEPTION(
          neg_zero, func(in.neg_min_denormal, in.neg_min_denormal),
          FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
    }

    if (ForceRoundingMode r(RoundingMode::TowardZero); r.success) {
      EXPECT_FP_EQ_WITH_EXCEPTION(max_normal,
                                  func(in.max_normal, in.max_normal),
                                  FE_OVERFLOW | FE_INEXACT);
      EXPECT_FP_EQ_WITH_EXCEPTION(neg_max_normal,
                                  func(in.neg_max_normal, in.neg_max_normal),
                                  FE_OVERFLOW | FE_INEXACT);

      EXPECT_FP_EQ_WITH_EXCEPTION(zero, func(in.min_denormal, in.min_denormal),
                                  FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
      EXPECT_FP_EQ_WITH_EXCEPTION(
          neg_zero, func(in.neg_min_denormal, in.neg_min_denormal),
          FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
    }

    if (ForceRoundingMode r(RoundingMode::Downward); r.success) {
      EXPECT_FP_EQ_WITH_EXCEPTION(max_normal,
                                  func(in.max_normal, in.max_normal),
                                  FE_OVERFLOW | FE_INEXACT);
      EXPECT_FP_EQ_WITH_EXCEPTION(-inf,
                                  func(in.neg_max_normal, in.neg_max_normal),
                                  FE_OVERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);

      EXPECT_FP_EQ_WITH_EXCEPTION(zero, func(in.min_denormal, in.min_denormal),
                                  FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
      EXPECT_FP_EQ_WITH_EXCEPTION(
          neg_min_denormal, func(in.neg_min_denormal, in.neg_min_denormal),
          FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
    }

    if (ForceRoundingMode r(RoundingMode::Upward); r.success) {
      EXPECT_FP_EQ_WITH_EXCEPTION(inf, func(in.max_normal, in.max_normal),
                                  FE_OVERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
      EXPECT_FP_EQ_WITH_EXCEPTION(neg_max_normal,
                                  func(in.neg_max_normal, in.neg_max_normal),
                                  FE_OVERFLOW | FE_INEXACT);

      EXPECT_FP_EQ_WITH_EXCEPTION(min_denormal,
                                  func(in.min_denormal, in.min_denormal),
                                  FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
      EXPECT_FP_EQ_WITH_EXCEPTION(
          neg_zero, func(in.neg_min_denormal, in.neg_min_denormal),
          FE_UNDERFLOW | FE_INEXACT);
      EXPECT_MATH_ERRNO(ERANGE);
    }
#endif
  }

  void test_inexact_results(AddFunc func) {
    func(InType(1.0), in.min_denormal);
    EXPECT_FP_EXCEPTION(FE_INEXACT);
  }

  void test_mixed_normality(AddFunc func) {
    if (LIBC_NAMESPACE::fputil::get_fp_type<OutType>() !=
        LIBC_NAMESPACE::fputil::get_fp_type<InType>())
      return;

    EXPECT_FP_EQ(FPBits::create_value(Sign::POS, 2U, 0b1U).get_val(),
                 func(InFPBits::create_value(Sign::POS, 2U, 0U).get_val(),
                      InFPBits::create_value(Sign::POS, 0U, 0b10U).get_val()));
  }
};

#define LIST_ADD_TESTS(OutType, InType, func)                                  \
  using LlvmLibcAddTest = AddTest<OutType, InType>;                            \
  TEST_F(LlvmLibcAddTest, SpecialNumbers) { test_special_numbers(&func); }     \
  TEST_F(LlvmLibcAddTest, InvalidOperations) {                                 \
    test_invalid_operations(&func);                                            \
  }                                                                            \
  TEST_F(LlvmLibcAddTest, RangeErrors) { test_range_errors(&func); }           \
  TEST_F(LlvmLibcAddTest, InexactResults) { test_inexact_results(&func); }     \
  TEST_F(LlvmLibcAddTest, MixedNormality) { test_mixed_normality(&func); }

#define LIST_ADD_SAME_TYPE_TESTS(suffix, OutType, InType, func)                \
  using LlvmLibcAddTest##suffix = AddTest<OutType, InType>;                    \
  TEST_F(LlvmLibcAddTest##suffix, SpecialNumbers) {                            \
    test_special_numbers(&func);                                               \
  }                                                                            \
  TEST_F(LlvmLibcAddTest##suffix, InvalidOperations) {                         \
    test_invalid_operations(&func);                                            \
  }                                                                            \
  TEST_F(LlvmLibcAddTest##suffix, RangeErrors) { test_range_errors(&func); }   \
  TEST_F(LlvmLibcAddTest##suffix, InexactResults) {                            \
    test_inexact_results(&func);                                               \
  }                                                                            \
  TEST_F(LlvmLibcAddTest##suffix, MixedNormality) {                            \
    test_mixed_normality(&func);                                               \
  }

#endif // LLVM_LIBC_TEST_SRC_MATH_SMOKE_ADDTEST_H
