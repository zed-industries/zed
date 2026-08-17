//===-- Implementation header for wait4 -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_WAIT_WAIT4_H
#define LLVM_LIBC_SRC_SYS_WAIT_WAIT4_H

#include "src/__support/macros/config.h"
#include <sys/wait.h>

namespace LIBC_NAMESPACE_DECL {

pid_t wait4(pid_t pid, int *waitstatus, int options, struct rusage *usage);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_WAIT_WAIT4_H
