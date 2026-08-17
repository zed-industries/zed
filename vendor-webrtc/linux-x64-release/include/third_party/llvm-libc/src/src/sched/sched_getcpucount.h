//===-- Implementation header for sched_getcpucount -------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SCHED_SCHED_GETCPUCOUNT_H
#define LLVM_LIBC_SRC_SCHED_SCHED_GETCPUCOUNT_H

#include "src/__support/macros/config.h"
#include <sched.h>
#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {

// This function is for internal use in the CPU_COUNT macro, but since that's a
// macro and will be applied to client files, this must be a public entrypoint.
int __sched_getcpucount(size_t cpuset_size, const cpu_set_t *mask);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SCHED_SCHED_GETCPUCOUNT_H
