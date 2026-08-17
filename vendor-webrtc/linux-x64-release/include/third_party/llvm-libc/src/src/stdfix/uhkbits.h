//===-- Implementation header for uhkbits -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDFIX_UHKBITS_H
#define LLVM_LIBC_SRC_STDFIX_UHKBITS_H

#include "include/llvm-libc-macros/stdfix-macros.h"
#include "include/llvm-libc-types/stdfix-types.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

unsigned short accum uhkbits(uint_uhk_t x);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDFIX_UHKBITS_H
