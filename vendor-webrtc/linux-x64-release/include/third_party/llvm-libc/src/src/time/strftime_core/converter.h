//===-- Format specifier converter for strftime -----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_CONVERTER_H
#define LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_CONVERTER_H

#include "hdr/types/struct_tm.h"
#include "src/__support/macros/config.h"
#include "src/stdio/printf_core/writer.h"
#include "src/time/strftime_core/core_structs.h"

#include "composite_converter.h"
#include "num_converter.h"
#include "str_converter.h"

namespace LIBC_NAMESPACE_DECL {
namespace strftime_core {

// convert will call a conversion function to convert the FormatSection into
// its string representation, and then that will write the result to the
// writer.
template <printf_core::WriteMode write_mode>
int convert(printf_core::Writer<write_mode> *writer,
            const FormatSection &to_conv, const tm *timeptr) {
  // TODO: Implement the locale support.
  // Currently locale flags are ignored, as described by the posix standard for
  // the default locale.

  if (!to_conv.has_conv)
    return writer->write(to_conv.raw_string);
  switch (to_conv.conv_name) {
    // The cases are grouped by type, then alphabetized with lowercase before
    // uppercase.

    // raw conversions
  case '%':
    return writer->write("%");
  case 'n':
    return writer->write("\n");
  case 't':
    return writer->write("\t");

    // numeric conversions
  case 'C': // Century [00-99]
  case 'd': // Day of the month [01-31]
  case 'e': // Day of the month [1-31]
  case 'g': // last 2 digits of ISO year [00-99]
  case 'G': // ISO year
  case 'H': // 24-hour format [00-23]
  case 'I': // 12-hour format [01-12]
  case 'j': // Day of the year [001-366]
  case 'm': // Month of the year [01-12]
  case 'M': // Minute of the hour [00-59]
  case 's': // Seconds since the epoch
  case 'S': // Second of the minute [00-60]
  case 'u': // ISO day of the week ([1-7] starting Monday)
  case 'U': // Week of the year ([00-53] week 1 starts on first *Sunday*)
  case 'V': // ISO week number ([01-53], 01 is first week majority in this year)
  case 'w': // Day of week ([0-6] starting Sunday)
  case 'W': // Week of the year ([00-53] week 1 starts on first *Monday*)
  case 'y': // Year of the Century [00-99]
  case 'Y': // Full year
    return convert_int(writer, to_conv, timeptr);

    // string conversions
  case 'a': // Abbreviated weekday name
  case 'A': // Full weekday name
  case 'b': // Abbreviated month name
  case 'B': // Full month name
  case 'h': // same as %b
  case 'p': // AM/PM designation
    return convert_str(writer, to_conv, timeptr);

    // composite conversions
  case 'c': // locale specified date and time
  case 'D': // %m/%d/%y (month/day/year)
  case 'F': // %Y-%m-%d (year-month-day)
  case 'r': // %I:%M:%S %p (hour:minute:second AM/PM)
  case 'R': // %H:%M (hour:minute)
  case 'T': // %H:%M:%S (hour:minute:second)
  case 'x': // locale specified date
  case 'X': // locale specified time
    return convert_composite(writer, to_conv, timeptr);

    // timezone conversions
  case 'z': // Timezone offset (+/-hhmm) (num conv)
  case 'Z': // Timezone name (string conv)
    // the standard says if no time zone is determinable, write no characters.
    // Leave this here until time zones are implemented.
    return 0;
  default:
    return writer->write(to_conv.raw_string);
  }
  return 0;
}

} // namespace strftime_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_CONVERTER_H
