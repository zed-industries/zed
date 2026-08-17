//===-- Implementation header for call_once function ------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_THREADS_CALL_ONCE_H
#define LLVM_LIBC_SRC_THREADS_CALL_ONCE_H

#include "src/__support/macros/config.h"
#include <threads.h>

namespace LIBC_NAMESPACE_DECL {

void call_once(once_flag *flag, __call_once_func_t func);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_THREADS_CALL_ONCE_H
