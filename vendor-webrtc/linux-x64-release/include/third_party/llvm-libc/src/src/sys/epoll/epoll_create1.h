//===-- Implementation header for epoll_create1 function --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_CREATE1_H
#define LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_CREATE1_H

#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

int epoll_create1(int flags);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_CREATE1_H
