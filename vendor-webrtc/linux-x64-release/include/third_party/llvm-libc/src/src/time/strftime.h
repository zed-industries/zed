//===-- Implementation header of strftime -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_TIME_STRFTIME_H
#define LLVM_LIBC_SRC_TIME_STRFTIME_H

#include "hdr/types/size_t.h"
#include "hdr/types/struct_tm.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

size_t strftime(char *__restrict, size_t max, const char *__restrict format,
                const tm *timeptr);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_TIME_STRFTIME_H
