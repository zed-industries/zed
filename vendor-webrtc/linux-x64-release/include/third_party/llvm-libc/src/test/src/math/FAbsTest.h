//===-- Utility class to test fabs[f|l] -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_FABSTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_FABSTEST_H

#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

#include "hdr/math_macros.h"

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename T>
class FAbsTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*FabsFunc)(T);

  void testSpecialNumbers(FabsFunc func) {
    EXPECT_FP_EQ(aNaN, func(aNaN));

    EXPECT_FP_EQ(inf, func(inf));
    EXPECT_FP_EQ(inf, func(neg_inf));

    EXPECT_FP_EQ(zero, func(zero));
    EXPECT_FP_EQ(zero, func(neg_zero));
  }

  void testRange(FabsFunc func) {
    constexpr StorageType COUNT = 100'000;
    constexpr StorageType STEP = STORAGE_MAX / COUNT;
    for (StorageType i = 0, v = 0; i <= COUNT; ++i, v += STEP) {
      T x = FPBits(v).get_val();
      if (FPBits(v).is_nan() || FPBits(v).is_inf())
        continue;
      ASSERT_MPFR_MATCH(mpfr::Operation::Abs, x, func(x), 0.0);
    }
  }
};

#define LIST_FABS_TESTS(T, func)                                               \
  using LlvmLibcFAbsTest = FAbsTest<T>;                                        \
  TEST_F(LlvmLibcFAbsTest, SpecialNumbers) { testSpecialNumbers(&func); }      \
  TEST_F(LlvmLibcFAbsTest, Range) { testRange(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_FABSTEST_H
