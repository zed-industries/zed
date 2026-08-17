//===-- Current position specifier converter for scanf ----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_CURRENT_POS_CONVERTER_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_CURRENT_POS_CONVERTER_H

#include "src/__support/macros/config.h"
#include "src/stdio/scanf_core/converter_utils.h"
#include "src/stdio/scanf_core/core_structs.h"
#include "src/stdio/scanf_core/reader.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace scanf_core {

template <typename T>
LIBC_INLINE int convert_current_pos(Reader<T> *reader,
                                    const FormatSection &to_conv) {
  write_int_with_length(reader->chars_read(), to_conv);
  return READ_OK;
}

} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_CURRENT_POS_CONVERTER_H
