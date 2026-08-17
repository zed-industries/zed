//===-- Template for diffing remquo results ---------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_FUZZING_MATH_REMQUO_H
#define LLVM_LIBC_FUZZING_MATH_REMQUO_H

#include "src/__support/FPUtil/FPBits.h"

#include "hdr/math_macros.h"
#include <stddef.h>
#include <stdint.h>

template <typename T> using RemQuoFunc = T (*)(T, T, int *);

template <typename T>
void RemQuoDiff(RemQuoFunc<T> func1, RemQuoFunc<T> func2, const uint8_t *data,
                size_t size) {
  constexpr size_t typeSize = sizeof(T);
  if (size < 2 * typeSize)
    return;

  T x = *reinterpret_cast<const T *>(data);
  T y = *reinterpret_cast<const T *>(data + typeSize);

  int q1, q2;
  T remainder1 = func1(x, y, &q1);
  T remainder2 = func2(x, y, &q2);

  LIBC_NAMESPACE::fputil::FPBits<T> bits1(remainder1);
  LIBC_NAMESPACE::fputil::FPBits<T> bits2(remainder2);

  if (bits1.is_nan()) {
    if (!bits2.is_nan())
      __builtin_trap();
    return;
  }

  if (bits1.is_inf() != bits2.is_inf())
    __builtin_trap();

  // Compare only the 3 LS bits of the quotient.
  if ((q1 & 0x7) != (q2 & 0x7))
    __builtin_trap();

  if (bits1.uintval() != bits2.uintval())
    __builtin_trap();
}

#endif // LLVM_LIBC_FUZZING_MATH_REMQUO_H
