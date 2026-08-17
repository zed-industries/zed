//===-- Implementation header for the locale --------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_LOCALE_LOCALECONV_H
#define LLVM_LIBC_SRC_LOCALE_LOCALECONV_H

#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"

#include "hdr/types/locale_t.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {

// We only support the "C" locale right now.
static constexpr size_t MAX_LOCALE_NAME_SIZE = 2;

struct __locale_data {
  char name[MAX_LOCALE_NAME_SIZE];
};

// The pointer to the default "C" locale.
extern __locale_t c_locale;

// The global locale instance.
extern locale_t locale;

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_LOCALE_LOCALECONV_H
