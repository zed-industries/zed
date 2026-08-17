//===-- Linux futex related definitions -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_THREADS_LINUX_FUTEX_H
#define LLVM_LIBC_SRC_THREADS_LINUX_FUTEX_H

#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h" // Architecture macros

namespace LIBC_NAMESPACE_DECL {

#if (defined(LIBC_TARGET_ARCH_IS_AARCH64) ||                                   \
     defined(LIBC_TARGET_ARCH_IS_X86_64))
// The futex data has to be exactly 4 bytes long. However, we use a uint type
// here as we do not want to use `uint32_t` type to match the public definitions
// of types which include a field for a futex word. With public definitions, we
// cannot include <stdint.h> so we stick to the `unsigned int` type for x86_64
// and aarch64
using FutexWordType = unsigned int;
static_assert(sizeof(FutexWordType) == 4,
              "Unexpected size of unsigned int type.");
#else
#error "Futex word base type not defined for the target architecture."
#endif

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_THREADS_LINUX_FUTEX_H
