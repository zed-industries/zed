//===-- Utility class to test different flavors of hypot ------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_MATH_HYPOTTEST_H
#define LLVM_LIBC_TEST_SRC_MATH_HYPOTTEST_H

#include "src/__support/FPUtil/FPBits.h"
#include "test/UnitTest/FEnvSafeTest.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

#include "hdr/math_macros.h"

using LIBC_NAMESPACE::Sign;

namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename T>
class HypotTestTemplate : public LIBC_NAMESPACE::testing::FEnvSafeTest {
private:
  using Func = T (*)(T, T);
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;

  using StorageType = typename FPBits::StorageType;
  const T nan = FPBits::quiet_nan().get_val();
  const T inf = FPBits::inf().get_val();
  const T neg_inf = FPBits::inf(Sign::NEG).get_val();
  const T zero = FPBits::zero().get_val();
  const T neg_zero = FPBits::zero(Sign::NEG).get_val();
  const T max_normal = FPBits::max_normal().get_val();
  const T min_normal = FPBits::min_normal().get_val();
  const T max_subnormal = FPBits::max_subnormal().get_val();
  const T min_subnormal = FPBits::min_subnormal().get_val();

  static constexpr StorageType MAX_NORMAL = FPBits::max_normal().uintval();
  static constexpr StorageType MIN_NORMAL = FPBits::min_normal().uintval();
  static constexpr StorageType MAX_SUBNORMAL =
      FPBits::max_subnormal().uintval();
  static constexpr StorageType MIN_SUBNORMAL =
      FPBits::min_subnormal().uintval();

public:
  void test_special_numbers(Func func) {
    constexpr int N = 13;
    const T SpecialInputs[N] = {inf,           neg_inf,        zero,
                                neg_zero,      max_normal,     min_normal,
                                max_subnormal, min_subnormal,  -max_normal,
                                -min_normal,   -max_subnormal, -min_subnormal};

    EXPECT_FP_EQ(func(inf, nan), inf);
    EXPECT_FP_EQ(func(nan, neg_inf), inf);
    EXPECT_FP_EQ(func(nan, nan), nan);
    EXPECT_FP_EQ(func(nan, zero), nan);
    EXPECT_FP_EQ(func(neg_zero, nan), nan);

    for (int i = 0; i < N; ++i) {
      for (int j = 0; j < N; ++j) {
        mpfr::BinaryInput<T> input{SpecialInputs[i], SpecialInputs[j]};
        EXPECT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Hypot, input,
                                       func(SpecialInputs[i], SpecialInputs[j]),
                                       0.5);
      }
    }
  }

  void test_subnormal_range(Func func) {
    constexpr StorageType COUNT = 10'001;
    for (unsigned scale = 0; scale < 4; ++scale) {
      StorageType max_value = MAX_SUBNORMAL << scale;
      StorageType step = (max_value - MIN_SUBNORMAL) / COUNT + 1;
      for (int signs = 0; signs < 4; ++signs) {
        for (StorageType v = MIN_SUBNORMAL, w = max_value;
             v <= max_value && w >= MIN_SUBNORMAL; v += step, w -= step) {
          T x = FPBits(v).get_val(), y = FPBits(w).get_val();
          if (signs % 2 == 1) {
            x = -x;
          }
          if (signs >= 2) {
            y = -y;
          }

          mpfr::BinaryInput<T> input{x, y};
          ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Hypot, input,
                                         func(x, y), 0.5);
        }
      }
    }
  }

  void test_normal_range(Func func) {
    constexpr StorageType COUNT = 10'001;
    constexpr StorageType STEP = (MAX_NORMAL - MIN_NORMAL) / COUNT;
    for (int signs = 0; signs < 4; ++signs) {
      for (StorageType v = MIN_NORMAL, w = MAX_NORMAL;
           v <= MAX_NORMAL && w >= MIN_NORMAL; v += STEP, w -= STEP) {
        T x = FPBits(v).get_val(), y = FPBits(w).get_val();
        if (signs % 2 == 1) {
          x = -x;
        }
        if (signs >= 2) {
          y = -y;
        }

        mpfr::BinaryInput<T> input{x, y};
        ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Hypot, input,
                                       func(x, y), 0.5);
      }
    }
  }

  void test_input_list(Func func, int n, const mpfr::BinaryInput<T> *inputs) {
    for (int i = 0; i < n; ++i) {
      ASSERT_MPFR_MATCH_ALL_ROUNDING(mpfr::Operation::Hypot, inputs[i],
                                     func(inputs[i].x, inputs[i].y), 0.5);
    }
  }
};

#endif // LLVM_LIBC_TEST_SRC_MATH_HYPOTTEST_H
