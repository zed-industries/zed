//===-- Implementation header for sched_getparam ----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SCHED_SCHED_GETPARAM_H
#define LLVM_LIBC_SRC_SCHED_SCHED_GETPARAM_H

#include "src/__support/macros/config.h"
#include <sched.h>

namespace LIBC_NAMESPACE_DECL {

int sched_getparam(pid_t tid, struct sched_param *param);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SCHED_SCHED_GETPARAM_H
