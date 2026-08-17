//===-- Implementation header for select ------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_SELECT_SELECT_H
#define LLVM_LIBC_SRC_SYS_SELECT_SELECT_H

#include "src/__support/macros/config.h"
#include <sys/select.h>

namespace LIBC_NAMESPACE_DECL {

int select(int nfds, fd_set *__restrict read_set, fd_set *__restrict write_set,
           fd_set *__restrict error_set, struct timeval *__restrict timeout);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_SELECT_SELECT_H
