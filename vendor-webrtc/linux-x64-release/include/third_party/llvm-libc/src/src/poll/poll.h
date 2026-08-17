//===-- Implementation header for poll ----------------------------*-C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_POLL_POLL_H
#define LLVM_LIBC_SRC_POLL_POLL_H

#include "hdr/types/nfds_t.h"
#include "hdr/types/struct_pollfd.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

int poll(pollfd *fds, nfds_t nfds, int timeout);

} // namespace LIBC_NAMESPACE_DECL

#endif //  LLVM_LIBC_SRC_POLL_POLL_H
