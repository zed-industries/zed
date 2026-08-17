//===-- Implementation header for duplocale ---------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_LOCALE_DUPLOCALE_H
#define LLVM_LIBC_SRC_LOCALE_DUPLOCALE_H

#include "src/__support/macros/config.h"

#include "hdr/types/locale_t.h"

namespace LIBC_NAMESPACE_DECL {

locale_t duplocale(locale_t loc);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_LOCALE_DUPLOCALE_H
