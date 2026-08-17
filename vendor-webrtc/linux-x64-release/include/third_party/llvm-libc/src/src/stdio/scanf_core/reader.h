//===-- Reader definition for scanf -----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_READER_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_READER_H

#include "src/__support/macros/attributes.h" // For LIBC_INLINE
#include "src/__support/macros/config.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace scanf_core {

template <typename Derived> class Reader {
  size_t cur_chars_read = 0;

public:
  // This returns the next character from the input and advances it by one
  // character. When it hits the end of the string or file it returns '\0' to
  // signal to stop parsing.
  LIBC_INLINE char getc() {
    ++cur_chars_read;
    return static_cast<Derived *>(this)->getc();
  }

  // This moves the input back by one character, placing c into the buffer if
  // this is a file reader, else c is ignored.
  LIBC_INLINE void ungetc(int c) {
    --cur_chars_read;
    static_cast<Derived *>(this)->ungetc(c);
  }

  LIBC_INLINE size_t chars_read() { return cur_chars_read; }
};

} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_READER_H
