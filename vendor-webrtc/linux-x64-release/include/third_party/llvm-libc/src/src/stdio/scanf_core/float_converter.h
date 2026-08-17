//===-- Float type specifier converter for scanf ----------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_FLOAT_CONVERTER_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_FLOAT_CONVERTER_H

#include "src/__support/CPP/limits.h"
#include "src/__support/char_vector.h"
#include "src/__support/ctype_utils.h"
#include "src/__support/macros/config.h"
#include "src/stdio/scanf_core/converter_utils.h"
#include "src/stdio/scanf_core/core_structs.h"
#include "src/stdio/scanf_core/reader.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace scanf_core {

// All of the floating point conversions are the same for scanf, every name will
// accept every style.
template <typename T>
int convert_float(Reader<T> *reader, const FormatSection &to_conv) {
  // %a/A/e/E/f/F/g/G "Matches an optionally signed floating-point number,
  // infinity, or NaN, whose format is the same as expected for the subject
  // sequence of the strtod function. The corresponding argument shall be a
  // pointer to floating."

  CharVector out_str = CharVector();
  bool is_number = false;

  size_t max_width = cpp::numeric_limits<size_t>::max();
  if (to_conv.max_width > 0) {
    max_width = to_conv.max_width;
  }

  char cur_char = reader->getc();
  // Handle the sign.
  if (cur_char == '+' || cur_char == '-') {
    if (!out_str.append(cur_char)) {
      return ALLOCATION_FAILURE;
    }
    if (out_str.length() == max_width) {
      return MATCHING_FAILURE;
    } else {
      cur_char = reader->getc();
    }
  }

  static constexpr char DECIMAL_POINT = '.';
  static const char inf_string[] = "infinity";

  // Handle inf

  if (internal::tolower(cur_char) == inf_string[0]) {
    size_t inf_index = 0;

    for (;
         inf_index < (sizeof(inf_string) - 1) && out_str.length() < max_width &&
         internal::tolower(cur_char) == inf_string[inf_index];
         ++inf_index) {
      if (!out_str.append(cur_char)) {
        return ALLOCATION_FAILURE;
      }
      cur_char = reader->getc();
    }

    if (inf_index == 3 || inf_index == sizeof(inf_string) - 1) {
      write_float_with_length(out_str.c_str(), to_conv);
      return READ_OK;
    } else {
      return MATCHING_FAILURE;
    }
  }

  static const char nan_string[] = "nan";

  // Handle nan
  if (internal::tolower(cur_char) == nan_string[0]) {
    size_t nan_index = 0;

    for (;
         nan_index < (sizeof(nan_string) - 1) && out_str.length() < max_width &&
         internal::tolower(cur_char) == nan_string[nan_index];
         ++nan_index) {
      if (!out_str.append(cur_char)) {
        return ALLOCATION_FAILURE;
      }
      cur_char = reader->getc();
    }

    if (nan_index == sizeof(nan_string) - 1) {
      write_float_with_length(out_str.c_str(), to_conv);
      return READ_OK;
    } else {
      return MATCHING_FAILURE;
    }
  }

  // Assume base of 10 by default but check if it is actually base 16.
  int base = 10;

  // If the string starts with 0 it might be in hex.
  if (cur_char == '0') {
    is_number = true;
    // Read the next character to check.
    if (!out_str.append(cur_char)) {
      return ALLOCATION_FAILURE;
    }
    // If we've hit the end, then this is "0", which is valid.
    if (out_str.length() == max_width) {
      write_float_with_length(out_str.c_str(), to_conv);
      return READ_OK;
    } else {
      cur_char = reader->getc();
    }

    // If that next character is an 'x' then this is a hexadecimal number.
    if (internal::tolower(cur_char) == 'x') {
      base = 16;

      if (!out_str.append(cur_char)) {
        return ALLOCATION_FAILURE;
      }
      // If we've hit the end here, we have "0x" which is a valid prefix to a
      // floating point number, and will be evaluated to 0.
      if (out_str.length() == max_width) {
        write_float_with_length(out_str.c_str(), to_conv);
        return READ_OK;
      } else {
        cur_char = reader->getc();
      }
    }
  }

  const char exponent_mark = ((base == 10) ? 'e' : 'p');
  bool after_decimal = false;

  // The format for the remaining characters at this point is DD.DDe+/-DD for
  // base 10 and XX.XXp+/-DD for base 16

  // This handles the digits before and after the decimal point, but not the
  // exponent.
  while (out_str.length() < max_width) {
    if (internal::isalnum(cur_char) &&
        internal::b36_char_to_int(cur_char) < base) {
      is_number = true;
      if (!out_str.append(cur_char)) {
        return ALLOCATION_FAILURE;
      }
      cur_char = reader->getc();
    } else if (cur_char == DECIMAL_POINT && !after_decimal) {
      after_decimal = true;
      if (!out_str.append(cur_char)) {
        return ALLOCATION_FAILURE;
      }
      cur_char = reader->getc();
    } else {
      break;
    }
  }

  // Handle the exponent, which has an exponent mark, an optional sign, and
  // decimal digits.
  if (internal::tolower(cur_char) == exponent_mark) {
    if (!out_str.append(cur_char)) {
      return ALLOCATION_FAILURE;
    }
    if (out_str.length() == max_width) {
      // This is laid out in the standard as being a matching error (100e is not
      // a valid float) but may conflict with existing implementations.
      return MATCHING_FAILURE;
    } else {
      cur_char = reader->getc();
    }

    if (cur_char == '+' || cur_char == '-') {
      if (!out_str.append(cur_char)) {
        return ALLOCATION_FAILURE;
      }
      if (out_str.length() == max_width) {
        return MATCHING_FAILURE;
      } else {
        cur_char = reader->getc();
      }
    }

    // It is specified by the standard that "100er" is a matching failure since
    // the longest prefix of a possibly valid floating-point number (which is
    // "100e") is not a valid floating-point number. If there is an exponent
    // mark then there must be a digit after it else the number is not valid.
    // Some implementations will roll back two characters (to just "100") and
    // accept that since the prefix is not valid, and some will interpret an
    // exponent mark followed by no digits as an additional exponent of 0
    // (accepting "100e" and returning 100.0). Both of these behaviors are wrong
    // by the standard, but they may be used in real code, see Hyrum's law. This
    // code follows the standard, but may be incompatible due to code expecting
    // these bugs.
    if (!internal::isdigit(cur_char)) {
      return MATCHING_FAILURE;
    }

    while (internal::isdigit(cur_char) && out_str.length() < max_width) {
      if (!out_str.append(cur_char)) {
        return ALLOCATION_FAILURE;
      }
      cur_char = reader->getc();
    }
  }

  // We always read one more character than will be used, so we have to put the
  // last one back.
  reader->ungetc(cur_char);

  // If we haven't actually found any digits, this is a matching failure (this
  // catches cases like "+.")
  if (!is_number) {
    return MATCHING_FAILURE;
  }
  write_float_with_length(out_str.c_str(), to_conv);

  return READ_OK;
}

} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_FLOAT_CONVERTER_H
