//===-- Implementation header for setrlimit ---------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_RESOURCE_SETRLIMIT_H
#define LLVM_LIBC_SRC_SYS_RESOURCE_SETRLIMIT_H

#include "src/__support/macros/config.h"
#include <sys/resource.h>

namespace LIBC_NAMESPACE_DECL {

int setrlimit(int resource, const struct rlimit *lim);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_RESOURCE_SETRLIMIT_H
