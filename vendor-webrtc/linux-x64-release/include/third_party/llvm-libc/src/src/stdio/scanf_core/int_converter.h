//===-- Int type specifier converter for scanf ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_INT_CONVERTER_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_INT_CONVERTER_H

#include "src/__support/CPP/limits.h"
#include "src/__support/ctype_utils.h"
#include "src/__support/macros/config.h"
#include "src/stdio/scanf_core/converter_utils.h"
#include "src/stdio/scanf_core/core_structs.h"
#include "src/stdio/scanf_core/reader.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace scanf_core {

// This code is very similar to the code in __support/str_to_integer.h but is
// not quite the same. Here is the list of differences and why they exist:
//  1) This takes a reader and a format section instead of a char* and the base.
//      This should be fairly self explanatory. While the char* could be adapted
//      to a reader and the base could be calculated ahead of time, the
//      semantics are slightly different, specifically a char* can be indexed
//      freely (I can read str[2] and then str[0]) whereas a File (which the
//      reader may contain) cannot.
//  2) Because this uses a Reader, this function can only unget once.
//      This is relevant because scanf specifies it reads the "longest sequence
//      of input characters which does not exceed any specified field width and
//      which is, or is a prefix of, a matching input sequence." Whereas the
//      strtol function accepts "the longest initial subsequence of the input
//      string (...) that is of the expected form." This is demonstrated by the
//      differences in how they deal with the string "0xZZZ" when parsing as
//      hexadecimal. Scanf will read the "0x" as a valid prefix and return 0,
//      since it reads the first 'Z', sees that it's not a valid hex digit, and
//      reverses one character. The strtol function on the other hand only
//      accepts the "0" since that's the longest valid hexadecimal sequence. It
//      sees the 'Z' after the "0x" and determines that this is not the prefix
//      to a valid hex string.
//  3) This conversion may have a maximum width.
//      If a maximum width is specified, this conversion is only allowed to
//      accept a certain number of characters. Strtol doesn't have any such
//      limitation.
template <typename T>
int convert_int(Reader<T> *reader, const FormatSection &to_conv) {
  // %d "Matches an optionally signed decimal integer [...] with the value 10
  // for the base argument. The corresponding argument shall be a pointer to
  // signed integer."

  // %i "Matches an optionally signed integer [...] with the value 0 for the
  // base argument. The corresponding argument shall be a pointer to signed
  // integer."

  // %u "Matches an optionally signed decimal integer [...] with the value 10
  // for the base argument. The corresponding argument shall be a pointer to
  // unsigned integer"

  // %o "Matches an optionally signed octal integer [...] with the value 8 for
  // the base argument. The corresponding argument shall be a pointer to
  // unsigned integer"

  // %x/X "Matches an optionally signed hexadecimal integer [...] with the value
  // 16 for the base argument. The corresponding argument shall be a pointer to
  // unsigned integer"

  size_t max_width = cpp::numeric_limits<size_t>::max();
  if (to_conv.max_width > 0) {
    max_width = to_conv.max_width;
  }

  uintmax_t result = 0;
  bool is_number = false;
  bool is_signed = false;
  int base = 0;
  if (to_conv.conv_name == 'i') {
    base = 0;
    is_signed = true;
  } else if (to_conv.conv_name == 'o') {
    base = 8;
  } else if (internal::tolower(to_conv.conv_name) == 'x' ||
             to_conv.conv_name == 'p') {
    base = 16;
  } else if (to_conv.conv_name == 'd') {
    base = 10;
    is_signed = true;
  } else { // conv_name must be 'u'
    base = 10;
  }

  char cur_char = reader->getc();

  char result_sign = '+';
  if (cur_char == '+' || cur_char == '-') {
    result_sign = cur_char;
    if (max_width > 1) {
      --max_width;
      cur_char = reader->getc();
    } else {
      // If the max width has been hit already, then the return value must be 0
      // since no actual digits of the number have been parsed yet.
      write_int_with_length(0, to_conv);
      return MATCHING_FAILURE;
    }
  }
  const bool is_negative = result_sign == '-';

  // Base of 0 means automatically determine the base. Base of 16 may have a
  // prefix of "0x"
  if (base == 0 || base == 16) {
    // If the first character is 0, then it could be octal or hex.
    if (cur_char == '0') {
      is_number = true;

      // Read the next character to check.
      if (max_width > 1) {
        --max_width;
        cur_char = reader->getc();
      } else {
        write_int_with_length(0, to_conv);
        return READ_OK;
      }

      if (internal::tolower(cur_char) == 'x') {
        // This is a valid hex prefix.

        is_number = false;
        // A valid hex prefix is not necessarily a valid number. For the
        // conversion to be valid it needs to use all of the characters it
        // consumes. From the standard:
        // 7.23.6.2 paragraph 9: "An input item is defined as the longest
        // sequence of input characters which does not exceed any specified
        // field width and which is, or is a prefix of, a matching input
        // sequence."
        // 7.23.6.2 paragraph 10: "If the input item is not a matching sequence,
        // the execution of the directive fails: this condition is a matching
        // failure"
        base = 16;
        if (max_width > 1) {
          --max_width;
          cur_char = reader->getc();
        } else {
          return MATCHING_FAILURE;
        }

      } else {
        if (base == 0) {
          base = 8;
        }
      }
    } else if (base == 0) {
      if (internal::isdigit(cur_char)) {
        // If the first character is a different number, then it's 10.
        base = 10;
      } else {
        // If the first character isn't a valid digit, then there are no valid
        // digits at all. The number is 0.
        reader->ungetc(cur_char);
        write_int_with_length(0, to_conv);
        return MATCHING_FAILURE;
      }
    }
  }

  constexpr uintmax_t UNSIGNED_MAX = cpp::numeric_limits<uintmax_t>::max();
  constexpr uintmax_t SIGNED_MAX =
      static_cast<uintmax_t>(cpp::numeric_limits<intmax_t>::max());
  constexpr uintmax_t NEGATIVE_SIGNED_MAX =
      static_cast<uintmax_t>(cpp::numeric_limits<intmax_t>::max()) + 1;

  const uintmax_t MAX =
      (is_signed ? (is_negative ? NEGATIVE_SIGNED_MAX : SIGNED_MAX)
                 : UNSIGNED_MAX);

  const uintmax_t max_div_by_base = MAX / base;

  if (internal::isalnum(cur_char) &&
      internal::b36_char_to_int(cur_char) < base) {
    is_number = true;
  }

  bool has_overflow = false;
  size_t i = 0;
  for (; i < max_width && internal::isalnum(cur_char) &&
         internal::b36_char_to_int(cur_char) < base;
       ++i, cur_char = reader->getc()) {

    uintmax_t cur_digit = internal::b36_char_to_int(cur_char);

    if (result == MAX) {
      has_overflow = true;
      continue;
    } else if (result > max_div_by_base) {
      result = MAX;
      has_overflow = true;
    } else {
      result = result * base;
    }

    if (result > MAX - cur_digit) {
      result = MAX;
      has_overflow = true;
    } else {
      result = result + cur_digit;
    }
  }

  // We always read one more character than will be used, so we have to put the
  // last one back.
  reader->ungetc(cur_char);

  if (!is_number)
    return MATCHING_FAILURE;

  if (has_overflow) {
    write_int_with_length(MAX, to_conv);
  } else {
    if (is_negative)
      result = -result;

    write_int_with_length(result, to_conv);
  }

  return READ_OK;
}

} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_INT_CONVERTER_H
