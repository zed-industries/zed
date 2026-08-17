//===-- String Converter for printf -----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_PRINTF_CORE_STRING_CONVERTER_H
#define LLVM_LIBC_SRC_STDIO_PRINTF_CORE_STRING_CONVERTER_H

#include "src/__support/CPP/string_view.h"
#include "src/__support/macros/config.h"
#include "src/stdio/printf_core/converter_utils.h"
#include "src/stdio/printf_core/core_structs.h"
#include "src/stdio/printf_core/writer.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace printf_core {

template <WriteMode write_mode>
LIBC_INLINE int convert_string(Writer<write_mode> *writer,
                               const FormatSection &to_conv) {
  size_t string_len = 0;
  const char *str_ptr = reinterpret_cast<const char *>(to_conv.conv_val_ptr);

#ifndef LIBC_COPT_PRINTF_NO_NULLPTR_CHECKS
  if (str_ptr == nullptr) {
    str_ptr = "(null)";
  }
#endif // LIBC_COPT_PRINTF_NO_NULLPTR_CHECKS

  for (const char *cur_str = (str_ptr); cur_str[string_len]; ++string_len) {
    ;
  }

  if (to_conv.precision >= 0 &&
      static_cast<size_t>(to_conv.precision) < string_len)
    string_len = to_conv.precision;

  size_t padding_spaces = to_conv.min_width > static_cast<int>(string_len)
                              ? to_conv.min_width - string_len
                              : 0;

  // If the padding is on the left side, write the spaces first.
  if (padding_spaces > 0 &&
      (to_conv.flags & FormatFlags::LEFT_JUSTIFIED) == 0) {
    RET_IF_RESULT_NEGATIVE(writer->write(' ', padding_spaces));
  }

  RET_IF_RESULT_NEGATIVE(writer->write({(str_ptr), string_len}));

  // If the padding is on the right side, write the spaces last.
  if (padding_spaces > 0 &&
      (to_conv.flags & FormatFlags::LEFT_JUSTIFIED) != 0) {
    RET_IF_RESULT_NEGATIVE(writer->write(' ', padding_spaces));
  }
  return WRITE_OK;
}

} // namespace printf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_PRINTF_CORE_STRING_CONVERTER_H
