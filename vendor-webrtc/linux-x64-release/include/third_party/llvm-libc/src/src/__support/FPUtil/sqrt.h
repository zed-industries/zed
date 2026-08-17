//===-- Square root of IEEE 754 floating point numbers ----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_FPUTIL_SQRT_H
#define LLVM_LIBC_SRC___SUPPORT_FPUTIL_SQRT_H

#include "src/__support/macros/properties/architectures.h"
#include "src/__support/macros/properties/cpu_features.h"

#include "src/__support/FPUtil/generic/sqrt.h"

// Generic instruction specializations with __builtin_elementwise_sqrt.
#if defined(LIBC_TARGET_CPU_HAS_FPU_FLOAT) ||                                  \
    defined(LIBC_TARGET_CPU_HAS_FPU_DOUBLE)

#if __has_builtin(__builtin_elementwise_sqrt)

namespace LIBC_NAMESPACE_DECL {
namespace fputil {

#ifdef LIBC_TARGET_CPU_HAS_FPU_FLOAT
template <> LIBC_INLINE float sqrt<float>(float x) {
  return __builtin_elementwise_sqrt(x);
}
#endif // LIBC_TARGET_CPU_HAS_FPU_FLOAT

#ifdef LIBC_TARGET_CPU_HAS_FPU_DOUBLE
template <> LIBC_INLINE double sqrt<double>(double x) {
  return __builtin_elementwise_sqrt(x);
}
#endif // LIBC_TARGET_CPU_HAS_FPU_DOUBLE

} // namespace fputil
} // namespace LIBC_NAMESPACE_DECL

#else // __builtin_elementwise_sqrt
// Use inline assembly when __builtin_elementwise_sqrt is not available.
#if defined(LIBC_TARGET_CPU_HAS_SSE2)
#include "x86_64/sqrt.h"
#elif defined(LIBC_TARGET_ARCH_IS_AARCH64)
#include "aarch64/sqrt.h"
#elif defined(LIBC_TARGET_ARCH_IS_ARM)
#include "arm/sqrt.h"
#elif defined(LIBC_TARGET_ARCH_IS_ANY_RISCV)
#include "riscv/sqrt.h"
#endif // Target specific header of inline asm.

#endif // __builtin_elementwise_sqrt

#endif // LIBC_TARGET_CPU_HAS_FPU_FLOAT or DOUBLE

#endif // LLVM_LIBC_SRC___SUPPORT_FPUTIL_SQRT_H
