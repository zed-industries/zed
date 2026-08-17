//===-- Internal implementation header of scanf -----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/OSUtil/io.h"
#include "src/__support/macros/config.h"
#include "src/stdio/scanf_core/reader.h"

namespace LIBC_NAMESPACE_DECL {

namespace scanf_core {

struct StdinReader : public Reader<StdinReader> {
  LIBC_INLINE char getc() {
    char buf[1];
    auto result = read_from_stdin(buf, sizeof(buf));
    if (result <= 0)
      return EOF;
    return buf[0];
  }
  LIBC_INLINE void ungetc(int) {}
};

} // namespace scanf_core

} // namespace LIBC_NAMESPACE_DECL
