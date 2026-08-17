//===-- Implementation header for bitshk function ---------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDFIX_BITSHK_H
#define LLVM_LIBC_SRC_STDFIX_BITSHK_H

#include "include/llvm-libc-macros/stdfix-macros.h" // short accum
#include "include/llvm-libc-types/stdfix-types.h"   // int_hk_t
#include "src/__support/macros/config.h"            // LIBC_NAMESPACE_DECL

namespace LIBC_NAMESPACE_DECL {

int_hk_t bitshk(short accum f);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDFIX_BITSHK_H
