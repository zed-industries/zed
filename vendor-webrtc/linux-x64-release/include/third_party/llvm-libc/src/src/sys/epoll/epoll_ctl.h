//===-- Implementation header for epoll_ctl function ------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_CTL_H
#define LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_CTL_H

#include "hdr/types/struct_epoll_event.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

// TODO: event should be nullable
int epoll_ctl(int epfd, int op, int fd, struct epoll_event *event);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_CTL_H
