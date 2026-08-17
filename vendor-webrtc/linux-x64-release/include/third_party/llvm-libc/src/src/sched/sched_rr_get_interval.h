//===-- Implementation header for sched_rr_get_interval ----------*- C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SCHED_SCHED_RR_GET_INTERVAL_H
#define LLVM_LIBC_SRC_SCHED_SCHED_RR_GET_INTERVAL_H

#include "src/__support/macros/config.h"
#include <sched.h>

namespace LIBC_NAMESPACE_DECL {

int sched_rr_get_interval(pid_t tid, struct timespec *tp);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SCHED_SCHED_RR_GET_INTERVAL_H
