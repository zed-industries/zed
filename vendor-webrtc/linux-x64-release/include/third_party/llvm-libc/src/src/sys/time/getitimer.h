//===-- Implementation header for getitimer -------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_TIME_GETITIMER_H
#define LLVM_LIBC_SRC_SYS_TIME_GETITIMER_H

#include "hdr/types/struct_itimerval.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {
int getitimer(int which, struct itimerval *curr_value);
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_TIME_GETITIMER_H
