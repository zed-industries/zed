//===-- Implementation header for recvfrom ----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_SOCKET_RECVFROM_H
#define LLVM_LIBC_SRC_SYS_SOCKET_RECVFROM_H

#include "hdr/types/socklen_t.h"
#include "hdr/types/ssize_t.h"
#include "hdr/types/struct_sockaddr.h"
#include "src/__support/macros/config.h"
#include <stddef.h> // For size_t

namespace LIBC_NAMESPACE_DECL {

ssize_t recvfrom(int sockfd, void *buf, size_t len, int flags,
                 sockaddr *__restrict src_addr, socklen_t *__restrict addrlen);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_SOCKET_RECVFROM_H
