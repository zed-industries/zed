//===-- Starting point for strftime ------------------------------*- C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_STRFTIME_MAIN_H
#define LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_STRFTIME_MAIN_H

#include "hdr/types/struct_tm.h"
#include "src/__support/macros/config.h"
#include "src/stdio/printf_core/writer.h"
#include "src/time/strftime_core/converter.h"
#include "src/time/strftime_core/core_structs.h"
#include "src/time/strftime_core/parser.h"

namespace LIBC_NAMESPACE_DECL {
namespace strftime_core {

template <printf_core::WriteMode write_mode>
int strftime_main(printf_core::Writer<write_mode> *writer,
                  const char *__restrict str, const tm *timeptr) {
  Parser parser(str);
  int result = 0;
  for (strftime_core::FormatSection cur_section = parser.get_next_section();
       !cur_section.raw_string.empty();
       cur_section = parser.get_next_section()) {
    if (cur_section.has_conv)
      result = convert(writer, cur_section, timeptr);
    else
      result = writer->write(cur_section.raw_string);

    if (result < 0)
      return result;
  }

  return writer->get_chars_written();
}

} // namespace strftime_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_STRFTIME_MAIN_H
