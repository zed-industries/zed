//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_FP_COMPARE_H
#define SUPPORT_FP_COMPARE_H

#include <algorithm> // for std::max
#include <cassert>
#include <cmath> // for std::abs

#include "test_macros.h"

// See https://www.boost.org/doc/libs/1_70_0/libs/test/doc/html/boost_test/testing_tools/extended_comparison/floating_point/floating_points_comparison_theory.html

template <typename T>
bool fptest_close(T val, T expected, T eps) {
  TEST_CONSTEXPR T zero = T(0);
  assert(eps >= zero);

  // Handle the zero cases
  if (eps == zero)
    return val == expected;
  if (val == zero)
    return std::abs(expected) <= eps;
  if (expected == zero)
    return std::abs(val) <= eps;

  return std::abs(val - expected) < eps && std::abs(val - expected) / std::abs(val) < eps;
}

template <typename T>
bool fptest_close_pct(T val, T expected, T percent) {
  assert(percent >= T(0));
  T eps = (percent / T(100)) * std::max(std::abs(val), std::abs(expected));
  return fptest_close(val, expected, eps);
}

#endif // SUPPORT_FP_COMPARE_H
