//===-- Implementation header for longjmp -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SETJMP_LONGJMP_H
#define LLVM_LIBC_SRC_SETJMP_LONGJMP_H

#include "hdr/types/jmp_buf.h"
#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/compiler.h"

namespace LIBC_NAMESPACE_DECL {

// TODO(https://github.com/llvm/llvm-project/issues/112427)
// Some of the architecture-specific definitions are marked `naked`, which in
// GCC implies `nothrow`.
//
// Right now, our aliases aren't marked `nothrow`, so we wind up in a situation
// where clang will emit -Wmissing-exception-spec if we add `nothrow` here, but
// GCC will emit -Wmissing-attributes here without `nothrow`. We need to update
// LLVM_LIBC_FUNCTION to denote when a function throws or not.

#ifdef LIBC_COMPILER_IS_GCC
[[gnu::nothrow]]
#endif
void longjmp(jmp_buf buf, int val);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SETJMP_LONGJMP_H
