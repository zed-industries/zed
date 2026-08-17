//===-- Implementation header for readlinkat --------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_UNISTD_READLINKAT_H
#define LLVM_LIBC_SRC_UNISTD_READLINKAT_H

#include "hdr/types/size_t.h"
#include "hdr/types/ssize_t.h"
#include "hdr/unistd_macros.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

ssize_t readlinkat(int, const char *__restrict, char *__restrict, size_t);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_UNISTD_READLINKAT_H
