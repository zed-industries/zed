//===-- Implementation header for lstat -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_STAT_LSTAT_H
#define LLVM_LIBC_SRC_SYS_STAT_LSTAT_H

#include "src/__support/macros/config.h"
#include <sys/stat.h>

namespace LIBC_NAMESPACE_DECL {

int lstat(const char *__restrict path, struct stat *__restrict statbuf);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_STAT_LSTAT_H
