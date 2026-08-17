//===-- Utility class to test copysign[f|l] ---------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_SMOKE_COPYSIGNTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_SMOKE_COPYSIGNTEST_H

#include "src/__support/CPP/algorithm.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

template <typename T>
class CopySignTest : public LIBC_NAMESPACE::testing::FEnvSafeTest {

  DECLARE_SPECIAL_CONSTANTS(T)

public:
  typedef T (*CopySignFunc)(T, T);

  void testSpecialNumbers(CopySignFunc func) {
    EXPECT_FP_EQ(aNaN, func(aNaN, T(-1.0)));
    EXPECT_FP_EQ(aNaN, func(aNaN, T(1.0)));

    EXPECT_FP_EQ(neg_inf, func(inf, T(-1.0)));
    EXPECT_FP_EQ(inf, func(neg_inf, T(1.0)));

    EXPECT_FP_EQ(neg_zero, func(zero, T(-1.0)));
    EXPECT_FP_EQ(zero, func(neg_zero, T(1.0)));
  }

  void testRange(CopySignFunc func) {
    constexpr int COUNT = 100'000;
    constexpr StorageType STEP = LIBC_NAMESPACE::cpp::max(
        static_cast<StorageType>(STORAGE_MAX / COUNT), StorageType(1));
    StorageType v = 0;
    for (int i = 0; i <= COUNT; ++i, v += STEP) {
      FPBits x_bits = FPBits(v);
      if (x_bits.is_nan() || x_bits.is_inf())
        continue;
      T x = x_bits.get_val();

      T res1 = func(x, -x);
      ASSERT_FP_EQ(res1, -x);

      T res2 = func(x, x);
      ASSERT_FP_EQ(res2, x);
    }
  }
};

#define LIST_COPYSIGN_TESTS(T, func)                                           \
  using LlvmLibcCopySignTest = CopySignTest<T>;                                \
  TEST_F(LlvmLibcCopySignTest, SpecialNumbers) { testSpecialNumbers(&func); }  \
  TEST_F(LlvmLibcCopySignTest, Range) { testRange(&func); }

#endif // LLVM_LIBC_TEST_SRC_MATH_SMOKE_COPYSIGNTEST_H
