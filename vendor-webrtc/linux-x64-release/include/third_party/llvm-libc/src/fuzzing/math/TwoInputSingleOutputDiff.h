//===-- Template to diff two-input-single-output functions ------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_FUZZING_MATH_TWO_INPUT_SINGLE_OUTPUT_DIFF_H
#define LLVM_LIBC_FUZZING_MATH_TWO_INPUT_SINGLE_OUTPUT_DIFF_H

#include "fuzzing/math/Compare.h"

#include <stddef.h>
#include <stdint.h>

template <typename T1, typename T2>
using TwoInputSingleOutputFunc = T1 (*)(T1, T2);

template <typename T1, typename T2>
void TwoInputSingleOutputDiff(TwoInputSingleOutputFunc<T1, T2> func1,
                              TwoInputSingleOutputFunc<T1, T2> func2,
                              const uint8_t *data, size_t size) {
  constexpr size_t t1Size = sizeof(T1);
  if (size < t1Size + sizeof(T2))
    return;

  T1 x = *reinterpret_cast<const T1 *>(data);
  T2 y = *reinterpret_cast<const T2 *>(data + t1Size);

  T1 result1 = func1(x, y);
  T1 result2 = func2(x, y);

  if (!ValuesEqual(result1, result2))
    __builtin_trap();
}

#endif // LLVM_LIBC_FUZZING_MATH_TWO_INPUT_SINGLE_OUTPUT_DIFF_H
