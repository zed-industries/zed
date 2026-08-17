//===-- Implementation header for mkdir -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_STAT_MKDIR_H
#define LLVM_LIBC_SRC_SYS_STAT_MKDIR_H

#include "src/__support/macros/config.h"
#include <sys/stat.h>

namespace LIBC_NAMESPACE_DECL {

int mkdir(const char *path, mode_t mode);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_STAT_MKDIR_H
