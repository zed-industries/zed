//===-- Reader definition for scanf -----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_STRING_READER_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_STRING_READER_H

#include "src/__support/macros/attributes.h" // For LIBC_INLINE
#include "src/__support/macros/config.h"
#include "src/stdio/scanf_core/reader.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace scanf_core {

class StringReader : public Reader<StringReader> {
  const char *buffer;
  [[maybe_unused]] size_t buff_len;
  size_t buff_cur = 0;

public:
  LIBC_INLINE StringReader(const char *buffer, size_t buff_len)
      : buffer(buffer), buff_len(buff_len) {}

  LIBC_INLINE char getc() {
    char output = buffer[buff_cur];
    ++buff_cur;
    return output;
  }
  LIBC_INLINE void ungetc(int) {
    if (buff_cur > 0) {
      // While technically c should be written back to the buffer, in scanf we
      // always write the character that was already there. Additionally, the
      // buffer is most likely to contain a string that isn't part of a file,
      // which may not be writable.
      --buff_cur;
    }
  }
};

} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_STRING_READER_H
