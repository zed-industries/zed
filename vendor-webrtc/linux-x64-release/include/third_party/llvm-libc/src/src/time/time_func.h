//===-- Implementation header of time ---------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_TIME_TIME_FUNC_H
#define LLVM_LIBC_SRC_TIME_TIME_FUNC_H

#include "hdr/types/time_t.h"
#include "src/__support/macros/config.h"

// Note this header file is named time_func.h to avoid conflicts with the
// public header file time.h.
namespace LIBC_NAMESPACE_DECL {

time_t time(time_t *tp);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_TIME_TIME_FUNC_H
