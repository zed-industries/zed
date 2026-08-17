//===-- Implementation header for fstatvfs ----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_STATVFS_FSTATVFS_H
#define LLVM_LIBC_SRC_SYS_STATVFS_FSTATVFS_H

#include "include/llvm-libc-types/struct_statvfs.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

int fstatvfs(int fd, struct statvfs *buf);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_STATVFS_FSTATVFS_H
