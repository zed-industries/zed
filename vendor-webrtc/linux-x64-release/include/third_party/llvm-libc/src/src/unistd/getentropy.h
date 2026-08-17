//===-- Implementation header for getentropy ------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "hdr/types/size_t.h"
#include "src/__support/common.h"

#ifndef LLVM_LIBC_SRC_UNISTD_GETENTROPY_H
#define LLVM_LIBC_SRC_UNISTD_GETENTROPY_H

namespace LIBC_NAMESPACE_DECL {
int getentropy(void *buffer, size_t length);
}

#endif // LLVM_LIBC_SRC_UNISTD_GETENTROPY_H
