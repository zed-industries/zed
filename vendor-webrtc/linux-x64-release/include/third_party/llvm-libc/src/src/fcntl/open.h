//===-- Implementation header of open ---------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_FCNTL_OPEN_H
#define LLVM_LIBC_SRC_FCNTL_OPEN_H

#include "hdr/fcntl_macros.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

int open(const char *path, int flags, ...);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_FCNTL_OPEN_H
