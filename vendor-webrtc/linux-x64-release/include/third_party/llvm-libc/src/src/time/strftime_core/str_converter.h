//===-- String converter for strftime ---------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See htto_conv.times://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_STR_CONVERTER_H
#define LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_STR_CONVERTER_H

#include "hdr/types/struct_tm.h"
#include "src/__support/CPP/string_view.h"
#include "src/__support/macros/config.h"
#include "src/stdio/printf_core/writer.h"
#include "src/time/strftime_core/core_structs.h"
#include "src/time/time_constants.h"
#include "src/time/time_utils.h"

namespace LIBC_NAMESPACE_DECL {
namespace strftime_core {

static constexpr cpp::string_view OUT_OF_BOUNDS_STR = "?";

LIBC_INLINE cpp::string_view
unwrap_opt(cpp::optional<cpp::string_view> str_opt) {
  return str_opt.has_value() ? *str_opt : OUT_OF_BOUNDS_STR;
}

template <printf_core::WriteMode write_mode>
LIBC_INLINE int convert_str(printf_core::Writer<write_mode> *writer,
                            const FormatSection &to_conv, const tm *timeptr) {
  cpp::string_view str;
  cpp::optional<cpp::string_view> str_opt;
  const time_utils::TMReader time_reader(timeptr);

  switch (to_conv.conv_name) {
  case 'a': // Abbreviated weekday name
    str_opt = time_reader.get_weekday_short_name();
    str = unwrap_opt(str_opt);
    break;
  case 'A': // Full weekday name
    str_opt = time_reader.get_weekday_full_name();
    str = unwrap_opt(str_opt);
    break;
  case 'b': // Abbreviated month name
  case 'h': // same as 'b'
    str_opt = time_reader.get_month_short_name();
    str = unwrap_opt(str_opt);
    break;
  case 'B': // Full month name
    str_opt = time_reader.get_month_full_name();
    str = unwrap_opt(str_opt);
    break;
  case 'p': // AM/PM designation
    str = time_reader.get_am_pm();
    break;
  case 'Z': // Timezone name
    // the standard says if no time zone is determinable, write no characters.
    return WRITE_OK;
    // str = time_reader.get_timezone_name();
    break;
  default:
    __builtin_trap(); // this should be unreachable, but trap if you hit it.
  }

  int spaces = to_conv.min_width - static_cast<int>(str.size());
  if (spaces > 0)
    RET_IF_RESULT_NEGATIVE(writer->write(' ', spaces));
  RET_IF_RESULT_NEGATIVE(writer->write(str));

  return WRITE_OK;
}

} // namespace strftime_core
} // namespace LIBC_NAMESPACE_DECL
#endif // LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_STR_CONVERTER_H
