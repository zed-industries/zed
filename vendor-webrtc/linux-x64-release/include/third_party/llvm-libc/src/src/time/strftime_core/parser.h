//===-- Format string parser for printf -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_PARSER_H
#define LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_PARSER_H

#include "core_structs.h"
#include "hdr/types/struct_tm.h"
#include "src/__support/CPP/string_view.h"
#include "src/__support/ctype_utils.h"
#include "src/__support/macros/config.h"
#include "src/__support/str_to_integer.h"

namespace LIBC_NAMESPACE_DECL {
namespace strftime_core {

class Parser {
  const char *str;
  size_t cur_pos = 0;

public:
  LIBC_INLINE Parser(const char *new_str) : str(new_str) {}

  // get_next_section will parse the format string until it has a fully
  // specified format section. This can either be a raw format section with no
  // conversion, or a format section with a conversion that has all of its
  // variables stored in the format section.
  LIBC_INLINE FormatSection get_next_section() {
    FormatSection section;
    size_t starting_pos = cur_pos;

    if (str[cur_pos] != '%') {
      // raw section
      section.has_conv = false;
      while (str[cur_pos] != '%' && str[cur_pos] != '\0')
        ++cur_pos;
      section.raw_string = {str + starting_pos, cur_pos - starting_pos};
      return section;
    }

    // format section
    section.has_conv = true;
    ++cur_pos;

    // flags
    section.flags = parse_flags(&cur_pos);

    // handle width
    section.min_width = 0;
    if (internal::isdigit(str[cur_pos])) {
      auto result = internal::strtointeger<int>(str + cur_pos, 10);
      section.min_width = result.value;
      cur_pos = cur_pos + result.parsed_len;
    }

    // modifiers
    switch (str[cur_pos]) {
    case ('E'):
      section.modifier = ConvModifier::E;
      ++cur_pos;
      break;
    case ('O'):
      section.modifier = ConvModifier::O;
      ++cur_pos;
      break;
    default:
      section.modifier = ConvModifier::none;
    }

    section.conv_name = str[cur_pos];

    // If the end of the format section is on the '\0'. This means we need to
    // not advance the cur_pos.
    if (str[cur_pos] != '\0')
      ++cur_pos;

    section.raw_string = {str + starting_pos, cur_pos - starting_pos};
    return section;
  }

private:
  LIBC_INLINE FormatFlags parse_flags(size_t *local_pos) {
    bool found_flag = true;
    FormatFlags flags = FormatFlags(0);
    while (found_flag) {
      switch (str[*local_pos]) {
      case '+':
        flags = static_cast<FormatFlags>(flags | FormatFlags::FORCE_SIGN);
        break;
      case '0':
        flags = static_cast<FormatFlags>(flags | FormatFlags::LEADING_ZEROES);
        break;
      default:
        found_flag = false;
      }
      if (found_flag)
        ++*local_pos;
    }
    return flags;
  }
};

} // namespace strftime_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_PARSER_H
