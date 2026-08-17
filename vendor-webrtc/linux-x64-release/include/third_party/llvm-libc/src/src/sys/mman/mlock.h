//===-- Implementation header for mlock function ----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_MMAN_MLOCK_H
#define LLVM_LIBC_SRC_SYS_MMAN_MLOCK_H

#include "src/__support/macros/config.h"
#include <sys/mman.h>
#include <sys/syscall.h>

namespace LIBC_NAMESPACE_DECL {

int mlock(const void *addr, size_t len);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_MMAN_MLOCK_H
