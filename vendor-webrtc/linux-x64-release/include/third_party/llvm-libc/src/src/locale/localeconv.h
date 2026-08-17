//===-- Implementation header for localeconv --------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_LOCALE_LOCALECONV_H
#define LLVM_LIBC_SRC_LOCALE_LOCALECONV_H

#include "src/__support/macros/config.h"

#include "include/llvm-libc-types/struct_lconv.h"

namespace LIBC_NAMESPACE_DECL {

struct lconv *localeconv();

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_LOCALE_LOCALECONV_H
