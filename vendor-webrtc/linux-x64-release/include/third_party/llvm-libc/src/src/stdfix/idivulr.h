//===-- Implementation header for idivulr ----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDFIX_IDIVULR_H
#define LLVM_LIBC_SRC_STDFIX_IDIVULR_H

#include "include/llvm-libc-macros/stdfix-macros.h" // unsigned long fract
#include "src/__support/macros/config.h"            // LIBC_NAMESPACE_DECL

namespace LIBC_NAMESPACE_DECL {

unsigned long int idivulr(unsigned long fract x, unsigned long fract y);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDFIX_IDIVULR_H
