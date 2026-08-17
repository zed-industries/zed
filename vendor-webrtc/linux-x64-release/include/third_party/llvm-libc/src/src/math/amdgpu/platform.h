//===-- AMDGPU specific platform definitions for math support -------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_MATH_AMDGPU_PLATFORM_H
#define LLVM_LIBC_SRC_MATH_AMDGPU_PLATFORM_H

#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {

// The ROCm device library uses control globals to alter codegen for the
// different targets. To avoid needing to link them in manually we simply
// define them here.
extern "C" {

// Disable unsafe math optimizations in the implementation.
extern const LIBC_INLINE_VAR uint8_t __oclc_unsafe_math_opt = 0;

// Disable denormalization at zero optimizations in the implementation.
extern const LIBC_INLINE_VAR uint8_t __oclc_daz_opt = 0;

// Disable rounding optimizations for 32-bit square roots.
extern const LIBC_INLINE_VAR uint8_t __oclc_correctly_rounded_sqrt32 = 1;

// Disable finite math optimizations.
extern const LIBC_INLINE_VAR uint8_t __oclc_finite_only_opt = 0;

// Set the ISA value to a high enough value that the ROCm device library math
// functions will assume we have fast FMA operations among other features. This
// is determined to be safe on all targets by looking at the source code.
// https://github.com/ROCm/ROCm-Device-Libs/blob/amd-stg-open/ocml/src/opts.h
extern const LIBC_INLINE_VAR uint32_t __oclc_ISA_version = 9000;
}

// These aliases cause clang to emit the control constants with ODR linkage.
// This allows us to link against the symbols without preventing them from being
// optimized out or causing symbol collisions.
[[gnu::alias("__oclc_unsafe_math_opt")]] const uint8_t __oclc_unsafe_math_opt__;
[[gnu::alias("__oclc_daz_opt")]] const uint8_t __oclc_daz_opt__;
[[gnu::alias("__oclc_correctly_rounded_sqrt32")]] const uint8_t
    __oclc_correctly_rounded_sqrt32__;
[[gnu::alias("__oclc_finite_only_opt")]] const uint8_t __oclc_finite_only_opt__;
[[gnu::alias("__oclc_ISA_version")]] const uint32_t __oclc_ISA_version__;

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_MATH_AMDGPU_PLATFORM_H
