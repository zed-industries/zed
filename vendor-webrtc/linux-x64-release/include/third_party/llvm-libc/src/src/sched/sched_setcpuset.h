//===-- Implementation header for sched_setcpuset ---------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SCHED_SCHED_SETCPUSET_H
#define LLVM_LIBC_SRC_SCHED_SCHED_SETCPUSET_H

#include "src/__support/macros/config.h"

#include "hdr/types/cpu_set_t.h"
#include "hdr/types/size_t.h"

namespace LIBC_NAMESPACE_DECL {

// for internal use in the CPU_SET macro
void __sched_setcpuset(int cpu, const size_t cpuset_size, cpu_set_t *set);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SCHED_SCHED_SETCPUSET_H
