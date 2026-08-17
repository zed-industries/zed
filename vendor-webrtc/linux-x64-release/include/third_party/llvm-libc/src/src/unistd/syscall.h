//===-- Implementation header for syscall -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_UNISTD_SYSCALL_H
#define LLVM_LIBC_SRC_UNISTD_SYSCALL_H

#include "hdr/unistd_macros.h"
#include "src/__support/macros/config.h"
#include <stdarg.h>

namespace LIBC_NAMESPACE_DECL {

long __llvm_libc_syscall(long number, long arg1, long arg2, long arg3,
                         long arg4, long arg5, long arg6);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_UNISTD_SYSCALL_H
