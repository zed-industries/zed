//===-- Implementation header for send --------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_SOCKET_SEND_H
#define LLVM_LIBC_SRC_SYS_SOCKET_SEND_H

#include "src/__support/macros/config.h"
#include <sys/socket.h>

namespace LIBC_NAMESPACE_DECL {

ssize_t send(int sockfd, const void *buf, size_t len, int flags);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_SOCKET_SEND_H
