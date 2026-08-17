//===-- Collection of utils for implementing wide char functions --*-C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_WCTYPE_UTILS_H
#define LLVM_LIBC_SRC___SUPPORT_WCTYPE_UTILS_H

#include "hdr/types/wint_t.h"
#include "src/__support/CPP/optional.h"
#include "src/__support/macros/attributes.h" // LIBC_INLINE
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {
namespace internal {

// ------------------------------------------------------
// Rationale: Since these classification functions are
// called in other functions, we will avoid the overhead
// of a function call by inlining them.
// ------------------------------------------------------

LIBC_INLINE cpp::optional<int> wctob(wint_t c) {
  // This needs to be translated to EOF at the callsite. This is to avoid
  // including stdio.h in this file.
  // The standard states that wint_t may either be an alias of wchar_t or
  // an alias of an integer type, different platforms define this type with
  // different signedness. This is equivalent to `(c > 127) || (c < 0)` but also
  // works without -Wtype-limits warnings when `wint_t` is unsigned.
  if ((c & ~127) != 0)
    return cpp::nullopt;
  return static_cast<int>(c);
}

LIBC_INLINE cpp::optional<wint_t> btowc(int c) {
  if (c > 127 || c < 0)
    return cpp::nullopt;
  return static_cast<wint_t>(c);
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC___SUPPORT_WCTYPE_UTILS_H
