//===-- Implementation header for libc_errno --------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_ERRNO_LIBC_ERRNO_H
#define LLVM_LIBC_SRC_ERRNO_LIBC_ERRNO_H

#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"

#include "hdr/errno_macros.h"

// This header is to be consumed by internal implementations, in which all of
// them should refer to `libc_errno` instead of using `errno` directly from
// <errno.h> header.

// Unit and hermetic tests should:
// - #include "src/errno/libc_errno.h"
// - NOT #include <errno.h>
// - Only use `libc_errno` in the code
// - Depend on libc.src.errno.errno

// Integration tests should:
// - NOT #include "src/errno/libc_errno.h"
// - #include <errno.h>
// - Use regular `errno` in the code
// - Still depend on libc.src.errno.errno

namespace LIBC_NAMESPACE_DECL {

extern "C" int *__llvm_libc_errno() noexcept;

struct Errno {
  void operator=(int);
  operator int();
};

extern Errno libc_errno;

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_ERRNO_LIBC_ERRNO_H
