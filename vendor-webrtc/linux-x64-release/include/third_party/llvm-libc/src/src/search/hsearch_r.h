//===-- Implementation header for hsearch_r ---------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SEARCH_HSEARCH_R_H
#define LLVM_LIBC_SRC_SEARCH_HSEARCH_R_H

#include "src/__support/macros/config.h"
#include <search.h> // ENTRY, ACTION

namespace LIBC_NAMESPACE_DECL {
int hsearch_r(ENTRY item, ACTION action, ENTRY **retval,
              struct hsearch_data *htab);
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SEARCH_HSEARCH_R_H
