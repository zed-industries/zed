//===-- Implementation header for bitsk function ----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDFIX_BITSK_H
#define LLVM_LIBC_SRC_STDFIX_BITSK_H

#include "include/llvm-libc-macros/stdfix-macros.h" // accum
#include "include/llvm-libc-types/stdfix-types.h"   // int_k_t
#include "src/__support/macros/config.h"            // LIBC_NAMESPACE_DECL

namespace LIBC_NAMESPACE_DECL {

int_k_t bitsk(accum f);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDFIX_BITSK_H
