//===-- Implementation header for setitimer -------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_TIME_SETITIMER_H
#define LLVM_LIBC_SRC_SYS_TIME_SETITIMER_H

#include "hdr/types/struct_itimerval.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {
int setitimer(int which, const struct itimerval *new_value,
              struct itimerval *old_value);
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_TIME_SETITIMER_H
