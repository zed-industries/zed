//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_CONTAINERS_SEQUENCES_VECTOR_BOOL_VECTOR_BOOL_FMT_FORMAT_FUNCTIONS_TESTS_H
#define TEST_STD_CONTAINERS_SEQUENCES_VECTOR_BOOL_VECTOR_BOOL_FMT_FORMAT_FUNCTIONS_TESTS_H

#include <vector>

#include "format.functions.common.h"
#include "test_macros.h"

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_vector_bool(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("[true, true, false]"), SV("{}"), input);
  check(SV("[true, true, false]^42"), SV("{}^42"), input);
  check(SV("[true, true, false]^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[true, true, false]     "), SV("{:24}"), input);
  check(SV("[true, true, false]*****"), SV("{:*<24}"), input);
  check(SV("__[true, true, false]___"), SV("{:_^24}"), input);
  check(SV("#####[true, true, false]"), SV("{:#>24}"), input);

  check(SV("[true, true, false]     "), SV("{:{}}"), input, 24);
  check(SV("[true, true, false]*****"), SV("{:*<{}}"), input, 24);
  check(SV("__[true, true, false]___"), SV("{:_^{}}"), input, 24);
  check(SV("#####[true, true, false]"), SV("{:#>{}}"), input, 24);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:+}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{: }"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), input);

  // *** n
  check(SV("__true, true, false___"), SV("{:_^22n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("[true   , true   , false  ]"), SV("{::7}"), input);
  check(SV("[true***, true***, false**]"), SV("{::*<7}"), input);
  check(SV("[_true__, _true__, _false_]"), SV("{::_^7}"), input);
  check(SV("[:::true, :::true, ::false]"), SV("{:::>7}"), input);

  check(SV("[true   , true   , false  ]"), SV("{::{}}"), input, 7);
  check(SV("[true***, true***, false**]"), SV("{::*<{}}"), input, 7);
  check(SV("[_true__, _true__, _false_]"), SV("{::_^{}}"), input, 7);
  check(SV("[:::true, :::true, ::false]"), SV("{:::>{}}"), input, 7);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier for a bool does not allow the sign option", SV("{::-}"), input);
  check_exception("The format specifier for a bool does not allow the sign option", SV("{::+}"), input);
  check_exception("The format specifier for a bool does not allow the sign option", SV("{:: }"), input);

  check(SV("[1, 1, 0]"), SV("{::-d}"), input);
  check(SV("[+1, +1, +0]"), SV("{::+d}"), input);
  check(SV("[ 1,  1,  0]"), SV("{:: d}"), input);

  // *** alternate form ***
  check_exception("The format specifier for a bool does not allow the alternate form option", SV("{::#}"), input);

  check(SV("[0x1, 0x1, 0x0]"), SV("{::#x}"), input);

  // *** zero-padding ***
  check_exception("The format specifier for a bool does not allow the zero-padding option", SV("{::05}"), input);

  check(SV("[00001, 00001, 00000]"), SV("{::05o}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::.}"), input);

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("bBdosxX"))
    check_exception("The type option contains an invalid value for a bool formatting argument", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^[:::true, :::true, ::false]^^^"), SV("{:^^32::>7}"), input);
  check(SV("^^[:::true, :::true, ::false]^^^"), SV("{:^^{}::>7}"), input, 32);
  check(SV("^^[:::true, :::true, ::false]^^^"), SV("{:^^{}::>{}}"), input, 32, 7);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>5}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>{}}"), input, 32);
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_tests(TestFunction check, ExceptionTest check_exception) {
  format_test_vector_bool<CharT>(check, check_exception, std::vector{true, true, false});

  // The const_reference shall be a bool.
  // However libc++ uses a __bit_const_reference<vector> when
  // _LIBCPP_ABI_BITSET_VECTOR_BOOL_CONST_SUBSCRIPT_RETURN_BOOL is defined.
  const std::vector input{true, true, false};
  format_test_vector_bool<CharT>(check, check_exception, input);
}

#endif // TEST_STD_CONTAINERS_SEQUENCES_VECTOR_BOOL_VECTOR_BOOL_FMT_FORMAT_FUNCTIONS_TESTS_H
