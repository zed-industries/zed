//===-- Implementation header for idivur -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDFIX_IDIVUR_H
#define LLVM_LIBC_SRC_STDFIX_IDIVUR_H

#include "include/llvm-libc-macros/stdfix-macros.h" // unsigned fract
#include "src/__support/macros/config.h"            // LIBC_NAMESPACE_DECL

namespace LIBC_NAMESPACE_DECL {

unsigned int idivur(unsigned fract x, unsigned fract y);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDFIX_IDIVUR_H
