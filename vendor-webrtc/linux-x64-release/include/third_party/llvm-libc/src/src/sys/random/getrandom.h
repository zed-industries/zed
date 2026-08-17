//===-- Implementation header for getrandom ----------------------*- C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_RANDOM_GETRANDOM_H
#define LLVM_LIBC_SRC_SYS_RANDOM_GETRANDOM_H

#include "src/__support/macros/config.h"
#include <sys/random.h>

namespace LIBC_NAMESPACE_DECL {

ssize_t getrandom(void *buf, size_t buflen, unsigned int flags);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_RANDOM_GETRANDOM_H
