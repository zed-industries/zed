//===-- Definition of macros from math.h ----------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_MACROS_MATH_MACROS_H
#define LLVM_LIBC_MACROS_MATH_MACROS_H

#include "limits-macros.h"

#define FP_NAN 0
#define FP_INFINITE 1
#define FP_ZERO 2
#define FP_SUBNORMAL 3
#define FP_NORMAL 4

#define FP_INT_UPWARD 0
#define FP_INT_DOWNWARD 1
#define FP_INT_TOWARDZERO 2
#define FP_INT_TONEARESTFROMZERO 3
#define FP_INT_TONEAREST 4

#define MATH_ERRNO 1
#define MATH_ERREXCEPT 2

#define HUGE_VAL __builtin_huge_val()
#define HUGE_VALF __builtin_huge_valf()
#define INFINITY __builtin_inff()
#define NAN __builtin_nanf("")

#define FP_ILOGB0 (-INT_MAX - 1)
#define FP_LLOGB0 (-LONG_MAX - 1)

#ifdef __FP_LOGBNAN_MIN
#define FP_ILOGBNAN (-INT_MAX - 1)
#define FP_LLOGBNAN (-LONG_MAX - 1)
#else
#define FP_ILOGBNAN INT_MAX
#define FP_LLOGBNAN LONG_MAX
#endif

#if defined(__NVPTX__) || defined(__AMDGPU__) || defined(__FAST_MATH__)
#define math_errhandling 0
#elif defined(__NO_MATH_ERRNO__)
#define math_errhandling (MATH_ERREXCEPT)
#else
#define math_errhandling (MATH_ERRNO | MATH_ERREXCEPT)
#endif

#endif // LLVM_LIBC_MACROS_MATH_MACROS_H
