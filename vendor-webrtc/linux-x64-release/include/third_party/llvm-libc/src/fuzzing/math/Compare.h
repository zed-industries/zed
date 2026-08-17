//===-- Template functions to compare scalar values -------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_FUZZING_MATH_COMPARE_H
#define LLVM_LIBC_FUZZING_MATH_COMPARE_H

#include "src/__support/CPP/type_traits.h"
#include "src/__support/FPUtil/FPBits.h"

template <typename T>
LIBC_NAMESPACE::cpp::enable_if_t<LIBC_NAMESPACE::cpp::is_floating_point_v<T>,
                                 bool>
ValuesEqual(T x1, T x2) {
  LIBC_NAMESPACE::fputil::FPBits<T> bits1(x1);
  LIBC_NAMESPACE::fputil::FPBits<T> bits2(x2);
  // If either is NaN, we want both to be NaN.
  if (bits1.is_nan() || bits2.is_nan())
    return bits1.is_nan() && bits2.is_nan();

  // For all other values, we want the values to be bitwise equal.
  return bits1.uintval() == bits2.uintval();
}

template <typename T>
LIBC_NAMESPACE::cpp::enable_if_t<LIBC_NAMESPACE::cpp::is_integral_v<T>, bool>
ValuesEqual(T x1, T x2) {
  return x1 == x2;
}

#endif // LLVM_LIBC_FUZZING_MATH_COMPARE_H
