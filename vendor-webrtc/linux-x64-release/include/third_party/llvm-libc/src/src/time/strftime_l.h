//===-- Implementation header for strftime_l --------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_TIME_STRFTIME_L_H
#define LLVM_LIBC_SRC_TIME_STRFTIME_L_H

#include "hdr/types/locale_t.h"
#include "hdr/types/size_t.h"
#include "hdr/types/struct_tm.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

size_t strftime_l(char *__restrict buffer, size_t count,
                  const char *__restrict format, const tm *timeptr, locale_t);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_TIME_STRFTIME_L_H
