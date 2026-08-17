//===-- Implementation header for epoll_pwait2 function ---------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_PWAIT2_H
#define LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_PWAIT2_H

#include "hdr/types/sigset_t.h"
#include "hdr/types/struct_epoll_event.h"
#include "hdr/types/struct_timespec.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

// TODO: sigmask and timeout should be nullable
int epoll_pwait2(int epfd, epoll_event *events, int maxevents,
                 const timespec *timeout, const sigset_t *sigmask);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_EPOLL_EPOLL_PWAIT2_H
