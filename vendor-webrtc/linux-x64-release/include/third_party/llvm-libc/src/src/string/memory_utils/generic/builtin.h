//===-- Trivial builtin implementations  ----------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STRING_MEMORY_UTILS_GENERIC_BUILTIN_H
#define LLVM_LIBC_SRC_STRING_MEMORY_UTILS_GENERIC_BUILTIN_H

#include "src/__support/macros/attributes.h" // LIBC_INLINE
#include "src/__support/macros/config.h"     // LIBC_NAMESPACE_DECL
#include "src/string/memory_utils/utils.h" // Ptr, CPtr

#include <stddef.h> // size_t

namespace LIBC_NAMESPACE_DECL {

#if !__has_builtin(__builtin_memcpy) || !__has_builtin(__builtin_memset) ||    \
    !__has_builtin(__builtin_memmove)
#error "Builtin not defined");
#endif

[[maybe_unused]] LIBC_INLINE void
inline_memcpy_builtin(Ptr dst, CPtr src, size_t count, size_t offset = 0) {
  __builtin_memcpy(dst + offset, src + offset, count);
}

[[maybe_unused]] LIBC_INLINE void inline_memmove_builtin(Ptr dst, CPtr src,
                                                         size_t count) {
  __builtin_memmove(dst, src, count);
}

[[maybe_unused]] LIBC_INLINE static void
inline_memset_builtin(Ptr dst, uint8_t value, size_t count, size_t offset = 0) {
  __builtin_memset(dst + offset, value, count);
}

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STRING_MEMORY_UTILS_GENERIC_BUILTIN_H
