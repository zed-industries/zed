//===-- Implementation header for rand utilities ----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDLIB_RAND_UTIL_H
#define LLVM_LIBC_SRC_STDLIB_RAND_UTIL_H

#include "src/__support/CPP/atomic.h"
#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

// The ISO C standard does not explicitly require thread-safe behavior for the
// generic `rand()` function. Some implementations expect it however, so we
// provide it here.
extern cpp::Atomic<unsigned long> rand_next;

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDLIB_RAND_UTIL_H
