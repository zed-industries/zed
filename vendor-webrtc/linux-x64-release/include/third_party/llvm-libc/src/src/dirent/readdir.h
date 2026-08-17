//===-- Implementation header of readdir ------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_DIRENT_READDIR_H
#define LLVM_LIBC_SRC_DIRENT_READDIR_H

#include "src/__support/macros/config.h" // LIBC_NAMESPACE_DECL
#include <dirent.h>

namespace LIBC_NAMESPACE_DECL {

struct ::dirent *readdir(DIR *dir);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_DIRENT_READDIR_H
