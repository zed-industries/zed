//===-- Implementation header for kill function -----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SIGNAL_KILL_H
#define LLVM_LIBC_SRC_SIGNAL_KILL_H

#include "src/__support/macros/config.h"
#include <signal.h>

namespace LIBC_NAMESPACE_DECL {

int kill(pid_t pid, int sig);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SIGNAL_KILL_H
