//===-- Format string parser for scanf -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_PARSER_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_PARSER_H

#include "src/__support/arg_list.h"
#include "src/__support/ctype_utils.h"
#include "src/__support/macros/config.h"
#include "src/__support/str_to_integer.h"
#include "src/stdio/scanf_core/core_structs.h"
#include "src/stdio/scanf_core/scanf_config.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace scanf_core {

#ifndef LIBC_COPT_SCANF_DISABLE_INDEX_MODE
#define GET_ARG_VAL_SIMPLEST(arg_type, index) get_arg_value<arg_type>(index)
#else
#define GET_ARG_VAL_SIMPLEST(arg_type, _) get_next_arg_value<arg_type>()
#endif // LIBC_COPT_SCANF_DISABLE_INDEX_MODE

template <typename ArgProvider> class Parser {
  const char *__restrict str;

  size_t cur_pos = 0;
  ArgProvider args_cur;

#ifndef LIBC_COPT_SCANF_DISABLE_INDEX_MODE
  // args_start stores the start of the va_args, which is used when a previous
  // argument is needed. In that case, we have to read the arguments from the
  // beginning since they don't support reading backwards.
  ArgProvider args_start;
  size_t args_index = 1;
#endif // LIBC_COPT_SCANF_DISABLE_INDEX_MODE

public:
#ifndef LIBC_COPT_SCANF_DISABLE_INDEX_MODE
  LIBC_INLINE Parser(const char *__restrict new_str, internal::ArgList &args)
      : str(new_str), args_cur(args), args_start(args) {}
#else
  LIBC_INLINE Parser(const char *__restrict new_str, internal::ArgList &args)
      : str(new_str), args_cur(args) {}
#endif // LIBC_COPT_SCANF_DISABLE_INDEX_MODE

  // get_next_section will parse the format string until it has a fully
  // specified format section. This can either be a raw format section with no
  // conversion, or a format section with a conversion that has all of its
  // variables stored in the format section.
  LIBC_INLINE FormatSection get_next_section() {
    FormatSection section;
    size_t starting_pos = cur_pos;
    if (str[cur_pos] == '%') {
      // format section
      section.has_conv = true;

      ++cur_pos;
      [[maybe_unused]] size_t conv_index = 0;

#ifndef LIBC_COPT_SCANF_DISABLE_INDEX_MODE
      conv_index = parse_index(&cur_pos);
#endif // LIBC_COPT_SCANF_DISABLE_INDEX_MODE

      if (str[cur_pos] == '*') {
        ++cur_pos;
        section.flags = FormatFlags::NO_WRITE;
      }

      // handle width
      section.max_width = -1;
      if (internal::isdigit(str[cur_pos])) {
        auto result = internal::strtointeger<int>(str + cur_pos, 10);
        section.max_width = result.value;
        cur_pos = cur_pos + static_cast<size_t>(result.parsed_len);
      }

      // TODO(michaelrj): add posix allocate flag support.
      // if (str[cur_pos] == 'm') {
      //   ++cur_pos;
      //   section.flags = FormatFlags::ALLOCATE;
      // }

      LengthModifier lm = parse_length_modifier(&cur_pos);
      section.length_modifier = lm;

      section.conv_name = str[cur_pos];

      // If NO_WRITE is not set, then read the next arg as the output pointer.
      if ((section.flags & FormatFlags::NO_WRITE) == 0) {
        // Since all outputs are pointers, there's no need to distinguish when
        // reading from va_args. They're all the same size and stored the same.
        section.output_ptr = GET_ARG_VAL_SIMPLEST(void *, conv_index);
      }

      // If the end of the format section is on the '\0'. This means we need to
      // not advance the cur_pos and we should not count this has having a
      // conversion.
      if (str[cur_pos] != '\0') {
        ++cur_pos;
      } else {
        section.has_conv = false;
      }

      // If the format is a bracketed one, then we need to parse out the insides
      // of the brackets.
      if (section.conv_name == '[') {
        constexpr char CLOSING_BRACKET = ']';
        constexpr char INVERT_FLAG = '^';
        constexpr char RANGE_OPERATOR = '-';

        cpp::bitset<256> scan_set;
        bool invert = false;

        // The circumflex in the first position represents the inversion flag,
        // but it's easier to apply that at the end so we just store it for now.
        if (str[cur_pos] == INVERT_FLAG) {
          invert = true;
          ++cur_pos;
        }

        // This is used to determine if a hyphen is being used as a literal or
        // as a range operator.
        size_t set_start_pos = cur_pos;

        // Normally the right bracket closes the set, but if it's the first
        // character (possibly after the inversion flag) then it's instead
        // included as a character in the set and the second right bracket
        // closes the set.
        if (str[cur_pos] == CLOSING_BRACKET) {
          scan_set.set(CLOSING_BRACKET);
          ++cur_pos;
        }

        while (str[cur_pos] != '\0' && str[cur_pos] != CLOSING_BRACKET) {
          // If a hyphen is being used as a range operator, since it's neither
          // at the beginning nor end of the set.
          if (str[cur_pos] == RANGE_OPERATOR && cur_pos != set_start_pos &&
              str[cur_pos + 1] != CLOSING_BRACKET && str[cur_pos + 1] != '\0') {
            // Technically there is no requirement to correct the ordering of
            // the range, but since the range operator is entirely
            // implementation defined it seems like a good convenience.
            char a = str[cur_pos - 1];
            char b = str[cur_pos + 1];
            char start = (a < b ? a : b);
            char end = (a < b ? b : a);
            scan_set.set_range(static_cast<size_t>(start),
                               static_cast<size_t>(end));
            cur_pos += 2;
          } else {
            scan_set.set(static_cast<size_t>(str[cur_pos]));
            ++cur_pos;
          }
        }
        if (invert)
          scan_set.flip();

        if (str[cur_pos] == CLOSING_BRACKET) {
          ++cur_pos;
          section.scan_set = scan_set;
        } else {
          // if the end of the string was encountered, this is not a valid set.
          section.has_conv = false;
        }
      }
    } else {
      // raw section
      section.has_conv = false;
      while (str[cur_pos] != '%' && str[cur_pos] != '\0')
        ++cur_pos;
    }
    section.raw_string = {str + starting_pos, cur_pos - starting_pos};
    return section;
  }

private:
  // parse_length_modifier parses the length modifier inside a format string. It
  // assumes that str[*local_pos] is inside a format specifier. It returns a
  // LengthModifier with the length modifier it found. It will advance local_pos
  // after the format specifier if one is found.
  LIBC_INLINE LengthModifier parse_length_modifier(size_t *local_pos) {
    switch (str[*local_pos]) {
    case ('l'):
      if (str[*local_pos + 1] == 'l') {
        *local_pos += 2;
        return LengthModifier::ll;
      } else {
        ++*local_pos;
        return LengthModifier::l;
      }
    case ('h'):
      if (str[*local_pos + 1] == 'h') {
        *local_pos += 2;
        return LengthModifier::hh;
      } else {
        ++*local_pos;
        return LengthModifier::h;
      }
    case ('L'):
      ++*local_pos;
      return LengthModifier::L;
    case ('j'):
      ++*local_pos;
      return LengthModifier::j;
    case ('z'):
      ++*local_pos;
      return LengthModifier::z;
    case ('t'):
      ++*local_pos;
      return LengthModifier::t;
    default:
      return LengthModifier::NONE;
    }
  }

  // get_next_arg_value gets the next value from the arg list as type T.
  template <class T> LIBC_INLINE T get_next_arg_value() {
    return args_cur.template next_var<T>();
  }

  //----------------------------------------------------
  // INDEX MODE ONLY FUNCTIONS AFTER HERE:
  //----------------------------------------------------

#ifndef LIBC_COPT_SCANF_DISABLE_INDEX_MODE

  // parse_index parses the index of a value inside a format string. It
  // assumes that str[*local_pos] points to character after a '%' or '*', and
  // returns 0 if there is no closing $, or if it finds no number. If it finds a
  // number, it will move local_pos past the end of the $, else it will not move
  // local_pos.
  LIBC_INLINE size_t parse_index(size_t *local_pos) {
    if (internal::isdigit(str[*local_pos])) {
      auto result = internal::strtointeger<int>(str + *local_pos, 10);
      size_t index = static_cast<size_t>(result.value);
      if (str[*local_pos + static_cast<size_t>(result.parsed_len)] != '$')
        return 0;
      *local_pos = static_cast<size_t>(1 + result.parsed_len) + *local_pos;
      return index;
    }
    return 0;
  }

  // get_arg_value gets the value from the arg list at index (starting at 1).
  // This may require parsing the format string. An index of 0 is interpreted as
  // the next value.
  template <class T> LIBC_INLINE T get_arg_value(size_t index) {
    if (!(index == 0 || index == args_index))
      args_to_index(index);

    ++args_index;
    return get_next_arg_value<T>();
  }

  // the ArgList can only return the next item in the list. This function is
  // used in index mode when the item that needs to be read is not the next one.
  // It moves cur_args to the index requested so the appropriate value may
  // be read. This may involve parsing the format string, and is in the worst
  // case an O(n^2) operation.
  LIBC_INLINE void args_to_index(size_t index) {
    if (args_index > index) {
      args_index = 1;
      args_cur = args_start;
    }

    while (args_index < index) {
      // Since all arguments must be pointers, we can just read all of them as
      // void * and not worry about type issues.
      args_cur.template next_var<void *>();
      ++args_index;
    }
  }

#endif // LIBC_COPT_SCANF_DISABLE_INDEX_MODE
};

} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_PARSER_H
