//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FORMATTER_FORMAT_FUNCTIONS_TESTS_H
#define TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FORMATTER_FORMAT_FUNCTIONS_TESTS_H

#include <algorithm>
#include <array>
#include <charconv>
#include <concepts>
#include <deque>
#include <format>
#include <functional> // std::identity
#include <list>
#include <ranges>
#include <span>
#include <tuple>
#include <utility>
#include <vector>

#include "format.functions.common.h"
#include "make_string.h"
#include "platform_support.h" // locale name macros
#include "test_iterators.h"
#include "test_macros.h"

//
// Char
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_char_default(TestFunction check, ExceptionTest check_exception, auto&& input) {
  // Note when no range-underlying-spec is present the char is escaped,
  check(SV("['H', 'e', 'l', 'l', 'o']"), SV("{}"), input);
  check(SV("['H', 'e', 'l', 'l', 'o']^42"), SV("{}^42"), input);
  check(SV("['H', 'e', 'l', 'l', 'o']^42"), SV("{:}^42"), input);

  // when one is present there is no escaping,
  check(SV("[H, e, l, l, o]"), SV("{::}"), input);
  check(SV("[H, e, l, l, o]"), SV("{::<}"), input);
  // unless forced by the type specifier.
  check(SV("['H', 'e', 'l', 'l', 'o']"), SV("{::?}"), input);
  check(SV("['H', 'e', 'l', 'l', 'o']"), SV("{::<?}"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("['H', 'e', 'l', 'l', 'o']     "), SV("{:30}"), input);
  check(SV("['H', 'e', 'l', 'l', 'o']*****"), SV("{:*<30}"), input);
  check(SV("__['H', 'e', 'l', 'l', 'o']___"), SV("{:_^30}"), input);
  check(SV("#####['H', 'e', 'l', 'l', 'o']"), SV("{:#>30}"), input);

  check(SV("['H', 'e', 'l', 'l', 'o']     "), SV("{:{}}"), input, 30);
  check(SV("['H', 'e', 'l', 'l', 'o']*****"), SV("{:*<{}}"), input, 30);
  check(SV("__['H', 'e', 'l', 'l', 'o']___"), SV("{:_^{}}"), input, 30);
  check(SV("#####['H', 'e', 'l', 'l', 'o']"), SV("{:#>{}}"), input, 30);

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
  check(SV("__'H', 'e', 'l', 'l', 'o'___"), SV("{:_^28n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("[H   , e   , l   , l   , o   ]"), SV("{::4}"), input);
  check(SV("[H***, e***, l***, l***, o***]"), SV("{::*<4}"), input);
  check(SV("[_H__, _e__, _l__, _l__, _o__]"), SV("{::_^4}"), input);
  check(SV("[:::H, :::e, :::l, :::l, :::o]"), SV("{:::>4}"), input);

  check(SV("[H   , e   , l   , l   , o   ]"), SV("{::{}}"), input, 4);
  check(SV("[H***, e***, l***, l***, o***]"), SV("{::*<{}}"), input, 4);
  check(SV("[_H__, _e__, _l__, _l__, _o__]"), SV("{::_^{}}"), input, 4);
  check(SV("[:::H, :::e, :::l, :::l, :::o]"), SV("{:::>{}}"), input, 4);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier for a character does not allow the sign option", SV("{::-}"), input);
  check_exception("The format specifier for a character does not allow the sign option", SV("{::+}"), input);
  check_exception("The format specifier for a character does not allow the sign option", SV("{:: }"), input);

  check(SV("[72, 101, 108, 108, 111]"), SV("{::-d}"), input);
  check(SV("[+72, +101, +108, +108, +111]"), SV("{::+d}"), input);
  check(SV("[ 72,  101,  108,  108,  111]"), SV("{:: d}"), input);

  // *** alternate form ***
  check_exception("The format specifier for a character does not allow the alternate form option", SV("{::#}"), input);

  check(SV("[0x48, 0x65, 0x6c, 0x6c, 0x6f]"), SV("{::#x}"), input);

  // *** zero-padding ***
  check_exception("The format specifier for a character does not allow the zero-padding option", SV("{::05}"), input);

  check(SV("[00110, 00145, 00154, 00154, 00157]"), SV("{::05o}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::.}"), input);

  // *** locale-specific form ***
  check(SV("[H, e, l, l, o]"), SV("{::L}"), input);

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("bBcdoxX?"))
    check_exception("The type option contains an invalid value for a character formatting argument", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^[:H, :e, :l, :l, :o]^^^"), SV("{:^^25::>2}"), input);
  check(SV("^^[:H, :e, :l, :l, :o]^^^"), SV("{:^^{}::>2}"), input, 25);
  check(SV("^^[:H, :e, :l, :l, :o]^^^"), SV("{:^^{}::>{}}"), input, 25, 2);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>2}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>{}}"), input, 25);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_char_string(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("Hello"), SV("{:s}"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("Hello   "), SV("{:8s}"), input);
  check(SV("Hello***"), SV("{:*<8s}"), input);
  check(SV("_Hello__"), SV("{:_^8s}"), input);
  check(SV("###Hello"), SV("{:#>8s}"), input);

  check(SV("Hello   "), SV("{:{}s}"), input, 8);
  check(SV("Hello***"), SV("{:*<{}s}"), input, 8);
  check(SV("_Hello__"), SV("{:_^{}s}"), input, 8);
  check(SV("###Hello"), SV("{:#>{}s}"), input, 8);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<s}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<s}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-s}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:+s}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{: s}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#s}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0s}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.s}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:Ls}"), input);

  // *** n
  check_exception("The n option and type s can't be used together", SV("{:ns}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("The type option contains an invalid value for a character formatting argument", SV("{::<s}"), input);

  // ***** Only underlying has a format-spec
  check_exception("Type s and an underlying format specification can't be used together", SV("{:s:}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("bBcdoxX?"))
    check_exception("The type option contains an invalid value for a character formatting argument", fmt, input);

  // ***** Both have a format-spec
  check_exception("Type s and an underlying format specification can't be used together", SV("{:5s:5}"), input);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_char_escaped_string(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV(R"("\"Hello'")"), SV("{:?s}"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV(R"("\"Hello'"   )"), SV("{:13?s}"), input);
  check(SV(R"("\"Hello'"***)"), SV("{:*<13?s}"), input);
  check(SV(R"(_"\"Hello'"__)"), SV("{:_^13?s}"), input);
  check(SV(R"(###"\"Hello'")"), SV("{:#>13?s}"), input);

  check(SV(R"("\"Hello'"   )"), SV("{:{}?s}"), input, 13);
  check(SV(R"("\"Hello'"***)"), SV("{:*<{}?s}"), input, 13);
  check(SV(R"(_"\"Hello'"__)"), SV("{:_^{}?s}"), input, 13);
  check(SV(R"(###"\"Hello'")"), SV("{:#>{}?s}"), input, 13);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<?s}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<?s}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::<?s}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-?s}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:+?s}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{: ?s}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#?s}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0?s}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.?s}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L?s}"), input);

  // *** n
  check_exception("The n option and type ?s can't be used together", SV("{:n?s}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);

  // ***** Only underlying has a format-spec
  check_exception("Type ?s and an underlying format specification can't be used together", SV("{:?s:}"), input);

  // ***** Both have a format-spec
  check_exception("Type ?s and an underlying format specification can't be used together", SV("{:5?s:5}"), input);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_char(TestFunction check, ExceptionTest check_exception) {
  test_char_default<CharT>(
      check, check_exception, std::array{CharT('H'), CharT('e'), CharT('l'), CharT('l'), CharT('o')});

  // This tests two different implementations in libc++. A basic_string_view
  // formatter if the range is contiguous, a basic_string otherwise.
  test_char_escaped_string<CharT>(
      check,
      check_exception,
      std::array{CharT('"'), CharT('H'), CharT('e'), CharT('l'), CharT('l'), CharT('o'), CharT('\'')});
  test_char_escaped_string<CharT>(
      check,
      check_exception,
      std::list{CharT('"'), CharT('H'), CharT('e'), CharT('l'), CharT('l'), CharT('o'), CharT('\'')});

  // This tests two different implementations in libc++. A basic_string_view
  // formatter if the range is contiguous, a basic_string otherwise.
  test_char_string<CharT>(
      check, check_exception, std::array{CharT('H'), CharT('e'), CharT('l'), CharT('l'), CharT('o')});
  test_char_string<CharT>(
      check, check_exception, std::list{CharT('H'), CharT('e'), CharT('l'), CharT('l'), CharT('o')});
}

//
// char -> wchar_t
//

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
template <class TestFunction, class ExceptionTest>
void test_char_to_wchar(TestFunction check, ExceptionTest check_exception) {
  test_char_default<wchar_t>(check, check_exception, std::array{'H', 'e', 'l', 'l', 'o'});

  // The types s and ?s may only be used when using range_formatter<T, charT>
  // where the types T and charT are the same. This means this can't be used for
  // range_formatter<wchar_t, char> even when formatter<wchar_t, char> has a
  // debug-enabled specialization.

  using CharT = wchar_t;
  check_exception(
      "Type s requires character type as formatting argument", SV("{:s}"), std::array{'H', 'e', 'l', 'l', 'o'});
  check_exception(
      "Type ?s requires character type as formatting argument", SV("{:?s}"), std::array{'H', 'e', 'l', 'l', 'o'});
}
#endif

//
// Bool
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_bool(TestFunction check, ExceptionTest check_exception) {
  std::array input{true, true, false};

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

  // *** locale-specific form ***
  check(SV("[true, true, false]"), SV("{::L}"), input);

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

//
// Integral
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_int(TestFunction check, ExceptionTest check_exception, auto&& input, auto make_range) {
  check(SV("[1, 2, 42, -42]"), SV("{}"), make_range(input));
  check(SV("[1, 2, 42, -42]^42"), SV("{}^42"), make_range(input));
  check(SV("[1, 2, 42, -42]^42"), SV("{:}^42"), make_range(input));

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[1, 2, 42, -42]     "), SV("{:20}"), make_range(input));
  check(SV("[1, 2, 42, -42]*****"), SV("{:*<20}"), make_range(input));
  check(SV("__[1, 2, 42, -42]___"), SV("{:_^20}"), make_range(input));
  check(SV("#####[1, 2, 42, -42]"), SV("{:#>20}"), make_range(input));

  check(SV("[1, 2, 42, -42]     "), SV("{:{}}"), make_range(input), 20);
  check(SV("[1, 2, 42, -42]*****"), SV("{:*<{}}"), make_range(input), 20);
  check(SV("__[1, 2, 42, -42]___"), SV("{:_^{}}"), make_range(input), 20);
  check(SV("#####[1, 2, 42, -42]"), SV("{:#>{}}"), make_range(input), 20);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), make_range(input));
  check_exception("The fill option contains an invalid value", SV("{:{<}"), make_range(input));

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-}"), make_range(input));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:+}"), make_range(input));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{: }"), make_range(input));

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), make_range(input));

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), make_range(input));

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), make_range(input));

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), make_range(input));

  // *** n
  check(SV("__1, 2, 42, -42___"), SV("{:_^18n}"), make_range(input));

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), make_range(input));
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), make_range(input));
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), make_range(input));
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, make_range(input));

  // ***** Only underlying has a format-spec
  check(SV("[    1,     2,    42,   -42]"), SV("{::5}"), make_range(input));
  check(SV("[1****, 2****, 42***, -42**]"), SV("{::*<5}"), make_range(input));
  check(SV("[__1__, __2__, _42__, _-42_]"), SV("{::_^5}"), make_range(input));
  check(SV("[::::1, ::::2, :::42, ::-42]"), SV("{:::>5}"), make_range(input));

  check(SV("[    1,     2,    42,   -42]"), SV("{::{}}"), make_range(input), 5);
  check(SV("[1****, 2****, 42***, -42**]"), SV("{::*<{}}"), make_range(input), 5);
  check(SV("[__1__, __2__, _42__, _-42_]"), SV("{::_^{}}"), make_range(input), 5);
  check(SV("[::::1, ::::2, :::42, ::-42]"), SV("{:::>{}}"), make_range(input), 5);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), make_range(input));
  check_exception("The fill option contains an invalid value", SV("{::{<}"), make_range(input));

  // *** sign ***
  check(SV("[1, 2, 42, -42]"), SV("{::-}"), make_range(input));
  check(SV("[+1, +2, +42, -42]"), SV("{::+}"), make_range(input));
  check(SV("[ 1,  2,  42, -42]"), SV("{:: }"), make_range(input));

  // *** alternate form ***
  check(SV("[0x1, 0x2, 0x2a, -0x2a]"), SV("{::#x}"), make_range(input));

  // *** zero-padding ***
  check(SV("[00001, 00002, 00042, -0042]"), SV("{::05}"), make_range(input));
  check(SV("[00001, 00002, 0002a, -002a]"), SV("{::05x}"), make_range(input));
  check(SV("[0x001, 0x002, 0x02a, -0x2a]"), SV("{::#05x}"), make_range(input));

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::.}"), make_range(input));

  // *** locale-specific form ***
  check(SV("[1, 2, 42, -42]"), SV("{::L}"), make_range(input)); // does nothing in this test, but is accepted.

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("bBcdoxX"))
    check_exception(
        "The type option contains an invalid value for an integer formatting argument", fmt, make_range(input));

  // ***** Both have a format-spec
  check(SV("^^[::::1, ::::2, :::42, ::-42]^^^"), SV("{:^^33::>5}"), make_range(input));
  check(SV("^^[::::1, ::::2, :::42, ::-42]^^^"), SV("{:^^{}::>5}"), make_range(input), 33);
  check(SV("^^[::::1, ::::2, :::42, ::-42]^^^"), SV("{:^^{}::>{}}"), make_range(input), 33, 5);

  check_exception("The argument index value is too large for the number of arguments supplied",
                  SV("{:^^{}::>5}"),
                  make_range(input));
  check_exception("The argument index value is too large for the number of arguments supplied",
                  SV("{:^^{}::>{}}"),
                  make_range(input),
                  33);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_int(TestFunction check, ExceptionTest check_exception) {
  test_int<CharT>(check, check_exception, std::array{1, 2, 42, -42}, std::identity());
  test_int<CharT>(check, check_exception, std::list{1, 2, 42, -42}, std::identity());
  test_int<CharT>(check, check_exception, std::vector{1, 2, 42, -42}, std::identity());
  std::array input{1, 2, 42, -42};
  test_int<CharT>(check, check_exception, std::span{input}, std::identity());
}

//
// Floating point
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_floating_point(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("[-42.5, 0, 1.25, 42.5]"), SV("{}"), input);
  check(SV("[-42.5, 0, 1.25, 42.5]^42"), SV("{}^42"), input);
  check(SV("[-42.5, 0, 1.25, 42.5]^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[-42.5, 0, 1.25, 42.5]     "), SV("{:27}"), input);
  check(SV("[-42.5, 0, 1.25, 42.5]*****"), SV("{:*<27}"), input);
  check(SV("__[-42.5, 0, 1.25, 42.5]___"), SV("{:_^27}"), input);
  check(SV("#####[-42.5, 0, 1.25, 42.5]"), SV("{:#>27}"), input);

  check(SV("[-42.5, 0, 1.25, 42.5]     "), SV("{:{}}"), input, 27);
  check(SV("[-42.5, 0, 1.25, 42.5]*****"), SV("{:*<{}}"), input, 27);
  check(SV("__[-42.5, 0, 1.25, 42.5]___"), SV("{:_^{}}"), input, 27);
  check(SV("#####[-42.5, 0, 1.25, 42.5]"), SV("{:#>{}}"), input, 27);

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
  check(SV("__-42.5, 0, 1.25, 42.5___"), SV("{:_^25n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("[-42.5,     0,  1.25,  42.5]"), SV("{::5}"), input);
  check(SV("[-42.5, 0****, 1.25*, 42.5*]"), SV("{::*<5}"), input);
  check(SV("[-42.5, __0__, 1.25_, 42.5_]"), SV("{::_^5}"), input);
  check(SV("[-42.5, ::::0, :1.25, :42.5]"), SV("{:::>5}"), input);

  check(SV("[-42.5,     0,  1.25,  42.5]"), SV("{::{}}"), input, 5);
  check(SV("[-42.5, 0****, 1.25*, 42.5*]"), SV("{::*<{}}"), input, 5);
  check(SV("[-42.5, __0__, 1.25_, 42.5_]"), SV("{::_^{}}"), input, 5);
  check(SV("[-42.5, ::::0, :1.25, :42.5]"), SV("{:::>{}}"), input, 5);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check(SV("[-42.5, 0, 1.25, 42.5]"), SV("{::-}"), input);
  check(SV("[-42.5, +0, +1.25, +42.5]"), SV("{::+}"), input);
  check(SV("[-42.5,  0,  1.25,  42.5]"), SV("{:: }"), input);

  // *** alternate form ***
  check(SV("[-42.5, 0., 1.25, 42.5]"), SV("{::#}"), input);

  // *** zero-padding ***
  check(SV("[-42.5, 00000, 01.25, 042.5]"), SV("{::05}"), input);
  check(SV("[-42.5, 0000., 01.25, 042.5]"), SV("{::#05}"), input);

  // *** precision ***
  check(SV("[-42, 0, 1.2, 42]"), SV("{::.2}"), input);
  check(SV("[-42.500, 0.000, 1.250, 42.500]"), SV("{::.3f}"), input);

  check(SV("[-42, 0, 1.2, 42]"), SV("{::.{}}"), input, 2);
  check(SV("[-42.500, 0.000, 1.250, 42.500]"), SV("{::.{}f}"), input, 3);

  check_exception("The precision option does not contain a value or an argument index", SV("{::.}"), input);

  // *** locale-specific form ***
  check(SV("[-42.5, 0, 1.25, 42.5]"), SV("{::L}"), input); // does not require locales present
#ifndef TEST_HAS_NO_LOCALIZATION
// TODO FMT Enable with locale testing active
#  if 0
  std::locale::global(std::locale(LOCALE_fr_FR_UTF_8));
  check(SV("[-42,5, 0, 1,25, 42,5]"), SV("{::L}"), input);

  std::locale::global(std::locale(LOCALE_en_US_UTF_8));
  check(SV("[-42.5, 0, 1.25, 42.5]"), SV("{::L}"), input);

  std::locale::global(std::locale::classic());
#  endif
#endif // TEST_HAS_NO_LOCALIZATION

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("aAeEfFgG"))
    check_exception("The type option contains an invalid value for a floating-point formatting argument", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^[-42.5, ::::0, :1.25, :42.5]^^^"), SV("{:^^33::>5}"), input);
  check(SV("^^[-42.5, ::::0, :1.25, :42.5]^^^"), SV("{:^^{}::>5}"), input, 33);
  check(SV("^^[-42.5, ::::0, :1.25, :42.5]^^^"), SV("{:^^{}::>{}}"), input, 33, 5);

  check(SV("^^[::-42, ::::0, ::1.2, :::42]^^^"), SV("{:^^33::>5.2}"), input);
  check(SV("^^[::-42, ::::0, ::1.2, :::42]^^^"), SV("{:^^{}::>5.2}"), input, 33);
  check(SV("^^[::-42, ::::0, ::1.2, :::42]^^^"), SV("{:^^{}::>{}.2}"), input, 33, 5);
  check(SV("^^[::-42, ::::0, ::1.2, :::42]^^^"), SV("{:^^{}::>{}.{}}"), input, 33, 5, 2);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>5.2}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>{}.2}"), input, 33);
  check_exception(
      "The argument index value is too large for the number of arguments supplied",
      SV("{:^^{}::>{}.{}}"),
      input,
      33,
      5);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_floating_point(TestFunction check, ExceptionTest check_exception) {
  test_floating_point<CharT>(check, check_exception, std::array{-42.5f, 0.0f, 1.25f, 42.5f});
  test_floating_point<CharT>(check, check_exception, std::vector{-42.5, 0.0, 1.25, 42.5});

  std::array input{-42.5l, 0.0l, 1.25l, 42.5l};
  test_floating_point<CharT>(check, check_exception, std::span{input});
}

//
// Pointer
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_pointer(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("[0x0]"), SV("{}"), input);
  check(SV("[0x0]^42"), SV("{}^42"), input);
  check(SV("[0x0]^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[0x0]     "), SV("{:10}"), input);
  check(SV("[0x0]*****"), SV("{:*<10}"), input);
  check(SV("__[0x0]___"), SV("{:_^10}"), input);
  check(SV("#####[0x0]"), SV("{:#>10}"), input);

  check(SV("[0x0]     "), SV("{:{}}"), input, 10);
  check(SV("[0x0]*****"), SV("{:*<{}}"), input, 10);
  check(SV("__[0x0]___"), SV("{:_^{}}"), input, 10);
  check(SV("#####[0x0]"), SV("{:#>{}}"), input, 10);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), input);

  // *** n
  check(SV("_0x0_"), SV("{:_^5n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("[  0x0]"), SV("{::5}"), input);
  check(SV("[0x0**]"), SV("{::*<5}"), input);
  check(SV("[_0x0_]"), SV("{::_^5}"), input);
  check(SV("[::0x0]"), SV("{:::>5}"), input);

  check(SV("[  0x0]"), SV("{::{}}"), input, 5);
  check(SV("[0x0**]"), SV("{::*<{}}"), input, 5);
  check(SV("[_0x0_]"), SV("{::_^{}}"), input, 5);
  check(SV("[::0x0]"), SV("{:::>{}}"), input, 5);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::-}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::#}"), input);

  // *** zero-padding ***
  check(SV("[0x0000]"), SV("{::06}"), input);
  check(SV("[0x0000]"), SV("{::06p}"), input);
  check(SV("[0X0000]"), SV("{::06P}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::L}"), input);

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("pP"))
    check_exception("The type option contains an invalid value for a pointer formatting argument", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^[::0x0]^^^"), SV("{:^^12::>5}"), input);
  check(SV("^^[::0x0]^^^"), SV("{:^^{}::>5}"), input, 12);
  check(SV("^^[::0x0]^^^"), SV("{:^^{}::>{}}"), input, 12, 5);

  check(SV("^^[::0x0]^^^"), SV("{:^^12::>5}"), input);
  check(SV("^^[::0x0]^^^"), SV("{:^^{}::>5}"), input, 12);
  check(SV("^^[::0x0]^^^"), SV("{:^^{}::>{}}"), input, 12, 5);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>5}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>{}}"), input, 12);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_pointer(TestFunction check, ExceptionTest check_exception) {
  test_pointer<CharT>(check, check_exception, std::array{nullptr});
  test_pointer<CharT>(check, check_exception, std::array{static_cast<const void*>(0)});
  test_pointer<CharT>(check, check_exception, std::array{static_cast<void*>(0)});
}

//
// String
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_string(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV(R"(["Hello", "world"])"), SV("{}"), input);
  check(SV(R"(["Hello", "world"]^42)"), SV("{}^42"), input);
  check(SV(R"(["Hello", "world"]^42)"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV(R"(["Hello", "world"]     )"), SV("{:23}"), input);
  check(SV(R"(["Hello", "world"]*****)"), SV("{:*<23}"), input);
  check(SV(R"(__["Hello", "world"]___)"), SV("{:_^23}"), input);
  check(SV(R"(#####["Hello", "world"])"), SV("{:#>23}"), input);

  check(SV(R"(["Hello", "world"]     )"), SV("{:{}}"), input, 23);
  check(SV(R"(["Hello", "world"]*****)"), SV("{:*<{}}"), input, 23);
  check(SV(R"(__["Hello", "world"]___)"), SV("{:_^{}}"), input, 23);
  check(SV(R"(#####["Hello", "world"])"), SV("{:#>{}}"), input, 23);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), input);

  // *** n
  check(SV(R"(_"Hello", "world"_)"), SV("{:_^18n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV(R"([Hello   , world   ])"), SV("{::8}"), input);
  check(SV(R"([Hello***, world***])"), SV("{::*<8}"), input);
  check(SV(R"([_Hello__, _world__])"), SV("{::_^8}"), input);
  check(SV(R"([:::Hello, :::world])"), SV("{:::>8}"), input);

  check(SV(R"([Hello   , world   ])"), SV("{::{}}"), input, 8);
  check(SV(R"([Hello***, world***])"), SV("{::*<{}}"), input, 8);
  check(SV(R"([_Hello__, _world__])"), SV("{::_^{}}"), input, 8);
  check(SV(R"([:::Hello, :::world])"), SV("{:::>{}}"), input, 8);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::-}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{::05}"), input);

  // *** precision ***
  check(SV(R"([Hel, wor])"), SV("{::.3}"), input);

  check(SV(R"([Hel, wor])"), SV("{::.{}}"), input, 3);

  check_exception("The precision option does not contain a value or an argument index", SV("{::.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::L}"), input);

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("s?"))
    check_exception("The type option contains an invalid value for a string formatting argument", fmt, input);

  // ***** Both have a format-spec
  check(SV(R"(^^[:::Hello, :::world]^^^)"), SV("{:^^25::>8}"), input);
  check(SV(R"(^^[:::Hello, :::world]^^^)"), SV("{:^^{}::>8}"), input, 25);
  check(SV(R"(^^[:::Hello, :::world]^^^)"), SV("{:^^{}::>{}}"), input, 25, 8);

  check(SV(R"(^^[:::Hello, :::world]^^^)"), SV("{:^^25::>8}"), input);
  check(SV(R"(^^[:::Hello, :::world]^^^)"), SV("{:^^{}::>8}"), input, 25);
  check(SV(R"(^^[:::Hello, :::world]^^^)"), SV("{:^^{}::>{}}"), input, 25, 8);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>8}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>{}}"), input, 25);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_string(TestFunction check, ExceptionTest check_exception) {
  test_string<CharT>(check, check_exception, std::array{CSTR("Hello"), CSTR("world")});
  test_string<CharT>(check, check_exception, std::array{STR("Hello"), STR("world")});
  test_string<CharT>(check, check_exception, std::array{SV("Hello"), SV("world")});
}

//
// Handle
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_status(TestFunction check, ExceptionTest check_exception) {
  std::array input{status::foo, status::bar, status::foobar};

  check(SV("[0xaaaa, 0x5555, 0xaa55]"), SV("{}"), input);
  check(SV("[0xaaaa, 0x5555, 0xaa55]^42"), SV("{}^42"), input);
  check(SV("[0xaaaa, 0x5555, 0xaa55]^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[0xaaaa, 0x5555, 0xaa55]     "), SV("{:29}"), input);
  check(SV("[0xaaaa, 0x5555, 0xaa55]*****"), SV("{:*<29}"), input);
  check(SV("__[0xaaaa, 0x5555, 0xaa55]___"), SV("{:_^29}"), input);
  check(SV("#####[0xaaaa, 0x5555, 0xaa55]"), SV("{:#>29}"), input);

  check(SV("[0xaaaa, 0x5555, 0xaa55]     "), SV("{:{}}"), input, 29);
  check(SV("[0xaaaa, 0x5555, 0xaa55]*****"), SV("{:*<{}}"), input, 29);
  check(SV("__[0xaaaa, 0x5555, 0xaa55]___"), SV("{:_^{}}"), input, 29);
  check(SV("#####[0xaaaa, 0x5555, 0xaa55]"), SV("{:#>{}}"), input, 29);

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
  check(SV("__0xaaaa, 0x5555, 0xaa55___"), SV("{:_^27n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check_exception("The type option contains an invalid value for a status formatting argument", SV("{::*<7}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("sxX"))
    check_exception("The type option contains an invalid value for a status formatting argument", fmt, input);

  check(SV("[0xaaaa, 0x5555, 0xaa55]"), SV("{::x}"), input);
  check(SV("[0XAAAA, 0X5555, 0XAA55]"), SV("{::X}"), input);
  check(SV("[foo, bar, foobar]"), SV("{::s}"), input);

  // ***** Both have a format-spec
  check(SV("^^[0XAAAA, 0X5555, 0XAA55]^^^"), SV("{:^^29:X}"), input);
  check(SV("^^[0XAAAA, 0X5555, 0XAA55]^^^"), SV("{:^^{}:X}"), input, 29);

  check_exception("The argument index value is too large for the number of arguments supplied", SV("{:^^{}:X}"), input);
}

//
// Pair
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_pair_tuple(TestFunction check, ExceptionTest check_exception, auto&& input) {
  // [format.range.formatter]/3
  //   For range_formatter<T, charT>, the format-spec in a
  //   range-underlying-spec, if any, is interpreted by formatter<T, charT>.
  //
  //   template<class ParseContext>
  //   constexpr typename ParseContext::iterator
  //    parse(ParseContext& ctx);
  // [format.tuple]/7
  //   ... if e.set_debug_format() is a valid expression, calls
  //   e.set_debug_format().
  // So when there is no range-underlying-spec, there is no need to call parse
  // thus the char element is not escaped.
  // TODO FMT P2733 addresses this issue.
  check(SV("[(1, 'a'), (42, '*')]"), SV("{}"), input);
  check(SV("[(1, 'a'), (42, '*')]^42"), SV("{}^42"), input);
  check(SV("[(1, 'a'), (42, '*')]^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[(1, 'a'), (42, '*')]     "), SV("{:26}"), input);
  check(SV("[(1, 'a'), (42, '*')]*****"), SV("{:*<26}"), input);
  check(SV("__[(1, 'a'), (42, '*')]___"), SV("{:_^26}"), input);
  check(SV("#####[(1, 'a'), (42, '*')]"), SV("{:#>26}"), input);

  check(SV("[(1, 'a'), (42, '*')]     "), SV("{:{}}"), input, 26);
  check(SV("[(1, 'a'), (42, '*')]*****"), SV("{:*<{}}"), input, 26);
  check(SV("__[(1, 'a'), (42, '*')]___"), SV("{:_^{}}"), input, 26);
  check(SV("#####[(1, 'a'), (42, '*')]"), SV("{:#>{}}"), input, 26);

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
  check(SV("__(1, 'a'), (42, '*')___"), SV("{:_^24n}"), input);
  check(SV("__(1, 'a'), (42, '*')___"), SV("{:_^24nm}"), input); // m should have no effect

  // *** type ***
  check(SV("__{(1, 'a'), (42, '*')}___"), SV("{:_^26m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("[(1, 'a')   , (42, '*')  ]"), SV("{::11}"), input);
  check(SV("[(1, 'a')***, (42, '*')**]"), SV("{::*<11}"), input);
  check(SV("[_(1, 'a')__, _(42, '*')_]"), SV("{::_^11}"), input);
  check(SV("[###(1, 'a'), ##(42, '*')]"), SV("{::#>11}"), input);

  check(SV("[(1, 'a')   , (42, '*')  ]"), SV("{::{}}"), input, 11);
  check(SV("[(1, 'a')***, (42, '*')**]"), SV("{::*<{}}"), input, 11);
  check(SV("[_(1, 'a')__, _(42, '*')_]"), SV("{::_^{}}"), input, 11);
  check(SV("[###(1, 'a'), ##(42, '*')]"), SV("{::#>{}}"), input, 11);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::-}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::+}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:: }"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{::05}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::L}"), input);

  // *** type ***
  check(SV("[1: 'a', 42: '*']"), SV("{::m}"), input);
  check(SV("[1, 'a', 42, '*']"), SV("{::n}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>(""))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^[###(1, 'a'), ##(42, '*')]^^^"), SV("{:^^31:#>11}"), input);
  check(SV("^^[###(1, 'a'), ##(42, '*')]^^^"), SV("{:^^31:#>11}"), input);
  check(SV("^^[###(1, 'a'), ##(42, '*')]^^^"), SV("{:^^{}:#>11}"), input, 31);
  check(SV("^^[###(1, 'a'), ##(42, '*')]^^^"), SV("{:^^{}:#>{}}"), input, 31, 11);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>5}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 31);

  check(SV("1: 'a', 42: '*'"), SV("{:n:m}"), input);
  check(SV("1, 'a', 42, '*'"), SV("{:n:n}"), input);
  check(SV("{1: 'a', 42: '*'}"), SV("{:m:m}"), input);
  check(SV("{1, 'a', 42, '*'}"), SV("{:m:n}"), input);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_pair_tuple(TestFunction check, ExceptionTest check_exception) {
  test_pair_tuple<CharT>(
      check, check_exception, std::array{std::make_pair(1, CharT('a')), std::make_pair(42, CharT('*'))});
  test_pair_tuple<CharT>(
      check, check_exception, std::array{std::make_tuple(1, CharT('a')), std::make_tuple(42, CharT('*'))});
}

//
// Tuple 1
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_tuple_int(TestFunction check, ExceptionTest check_exception) {
  std::array input{std::make_tuple(42), std::make_tuple(99)};

  check(SV("[(42), (99)]"), SV("{}"), input);
  check(SV("[(42), (99)]^42"), SV("{}^42"), input);
  check(SV("[(42), (99)]^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[(42), (99)]     "), SV("{:17}"), input);
  check(SV("[(42), (99)]*****"), SV("{:*<17}"), input);
  check(SV("__[(42), (99)]___"), SV("{:_^17}"), input);
  check(SV("#####[(42), (99)]"), SV("{:#>17}"), input);

  check(SV("[(42), (99)]     "), SV("{:{}}"), input, 17);
  check(SV("[(42), (99)]*****"), SV("{:*<{}}"), input, 17);
  check(SV("__[(42), (99)]___"), SV("{:_^{}}"), input, 17);
  check(SV("#####[(42), (99)]"), SV("{:#>{}}"), input, 17);

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
  check(SV("__(42), (99)___"), SV("{:_^15n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("[(42)   , (99)   ]"), SV("{::7}"), input);
  check(SV("[(42)***, (99)***]"), SV("{::*<7}"), input);
  check(SV("[_(42)__, _(99)__]"), SV("{::_^7}"), input);
  check(SV("[###(42), ###(99)]"), SV("{::#>7}"), input);

  check(SV("[(42)   , (99)   ]"), SV("{::{}}"), input, 7);
  check(SV("[(42)***, (99)***]"), SV("{::*<{}}"), input, 7);
  check(SV("[_(42)__, _(99)__]"), SV("{::_^{}}"), input, 7);
  check(SV("[###(42), ###(99)]"), SV("{::#>{}}"), input, 7);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::-}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::+}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:: }"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{::05}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::L}"), input);

  // *** type ***
  check(SV("[42, 99]"), SV("{::n}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>(""))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^[###(42), ###(99)]^^^"), SV("{:^^23:#>7}"), input);
  check(SV("^^[###(42), ###(99)]^^^"), SV("{:^^23:#>7}"), input);
  check(SV("^^[###(42), ###(99)]^^^"), SV("{:^^{}:#>7}"), input, 23);
  check(SV("^^[###(42), ###(99)]^^^"), SV("{:^^{}:#>{}}"), input, 23, 7);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>5}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 23);
}

//
// Tuple 3
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_tuple_int_int_int(TestFunction check, ExceptionTest check_exception) {
  std::array input{std::make_tuple(42, 99, 0), std::make_tuple(1, 10, 100)};

  check(SV("[(42, 99, 0), (1, 10, 100)]"), SV("{}"), input);
  check(SV("[(42, 99, 0), (1, 10, 100)]^42"), SV("{}^42"), input);
  check(SV("[(42, 99, 0), (1, 10, 100)]^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("[(42, 99, 0), (1, 10, 100)]     "), SV("{:32}"), input);
  check(SV("[(42, 99, 0), (1, 10, 100)]*****"), SV("{:*<32}"), input);
  check(SV("__[(42, 99, 0), (1, 10, 100)]___"), SV("{:_^32}"), input);
  check(SV("#####[(42, 99, 0), (1, 10, 100)]"), SV("{:#>32}"), input);

  check(SV("[(42, 99, 0), (1, 10, 100)]     "), SV("{:{}}"), input, 32);
  check(SV("[(42, 99, 0), (1, 10, 100)]*****"), SV("{:*<{}}"), input, 32);
  check(SV("__[(42, 99, 0), (1, 10, 100)]___"), SV("{:_^{}}"), input, 32);
  check(SV("#####[(42, 99, 0), (1, 10, 100)]"), SV("{:#>{}}"), input, 32);

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
  check(SV("__(42, 99, 0), (1, 10, 100)___"), SV("{:_^30n}"), input);

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("[(42, 99, 0)   , (1, 10, 100)  ]"), SV("{::14}"), input);
  check(SV("[(42, 99, 0)***, (1, 10, 100)**]"), SV("{::*<14}"), input);
  check(SV("[_(42, 99, 0)__, _(1, 10, 100)_]"), SV("{::_^14}"), input);
  check(SV("[###(42, 99, 0), ##(1, 10, 100)]"), SV("{::#>14}"), input);

  check(SV("[(42, 99, 0)   , (1, 10, 100)  ]"), SV("{::{}}"), input, 14);
  check(SV("[(42, 99, 0)***, (1, 10, 100)**]"), SV("{::*<{}}"), input, 14);
  check(SV("[_(42, 99, 0)__, _(1, 10, 100)_]"), SV("{::_^{}}"), input, 14);
  check(SV("[###(42, 99, 0), ##(1, 10, 100)]"), SV("{::#>{}}"), input, 14);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::-}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::+}"), input);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:: }"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{::05}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::L}"), input);

  // *** type ***
  check(SV("[42, 99, 0, 1, 10, 100]"), SV("{::n}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^[###(42, 99, 0), ##(1, 10, 100)]^^^"), SV("{:^^37:#>14}"), input);
  check(SV("^^[###(42, 99, 0), ##(1, 10, 100)]^^^"), SV("{:^^37:#>14}"), input);
  check(SV("^^[###(42, 99, 0), ##(1, 10, 100)]^^^"), SV("{:^^{}:#>14}"), input, 37);
  check(SV("^^[###(42, 99, 0), ##(1, 10, 100)]^^^"), SV("{:^^{}:#>{}}"), input, 37, 14);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>5}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 37);
}

//
// Ranges
//

template <class CharT, class Iterator, class TestFunction, class ExceptionTest, class Array>
void test_with_ranges_impl(TestFunction check, ExceptionTest check_exception, Array input) {
  auto make_range = [](auto& in) {
    std::counted_iterator it(Iterator(in.data()), in.size());
    std::ranges::subrange range{std::move(it), std::default_sentinel};
    return range;
  };
  test_int<CharT>(check, check_exception, input, make_range);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_with_ranges(TestFunction check, ExceptionTest check_exception) {
  std::array input{1, 2, 42, -42};
  test_with_ranges_impl<CharT, cpp20_input_iterator<int*>>(check, check_exception, input);
  test_with_ranges_impl<CharT, forward_iterator<int*>>(check, check_exception, input);
  test_with_ranges_impl<CharT, bidirectional_iterator<int*>>(check, check_exception, input);
  test_with_ranges_impl<CharT, random_access_iterator<int*>>(check, check_exception, input);
  test_with_ranges_impl<CharT, contiguous_iterator<int*>>(check, check_exception, input);
}

//
// Adaptor
//

template <class CharT>
class non_contiguous {
  // A deque iterator is random access, but not contiguous.
  using adaptee = std::deque<CharT>;

public:
  using iterator = typename adaptee::iterator;
  using pointer  = typename adaptee::pointer;

  iterator begin() { return data_.begin(); }
  iterator end() { return data_.end(); }

  explicit non_contiguous(adaptee&& data) : data_(std::move(data)) {}

private:
  adaptee data_;
};

template <class CharT>
class contiguous {
  // A vector iterator is contiguous.
  using adaptee = std::vector<CharT>;

public:
  using iterator = typename adaptee::iterator;
  using pointer  = typename adaptee::pointer;

  iterator begin() { return data_.begin(); }
  iterator end() { return data_.end(); }

  explicit contiguous(adaptee&& data) : data_(std::move(data)) {}

private:
  adaptee data_;
};

// This tests two different implementations in libc++. A basic_string_view
// formatter if the range is contiguous, a basic_string otherwise.
template <class CharT, class TestFunction, class ExceptionTest>
void test_adaptor(TestFunction check, ExceptionTest check_exception) {
  static_assert(std::format_kind<non_contiguous<CharT>> == std::range_format::sequence);
  static_assert(std::ranges::sized_range<non_contiguous<CharT>>);
  static_assert(!std::ranges::contiguous_range<non_contiguous<CharT>>);
  test_char_string<CharT>(
      check,
      check_exception,
      non_contiguous<CharT>{std::deque{CharT('H'), CharT('e'), CharT('l'), CharT('l'), CharT('o')}});

  static_assert(std::format_kind<contiguous<CharT>> == std::range_format::sequence);
  static_assert(std::ranges::sized_range<contiguous<CharT>>);
  static_assert(std::ranges::contiguous_range<contiguous<CharT>>);
  test_char_string<CharT>(check,
                          check_exception,
                          contiguous<CharT>{std::vector{CharT('H'), CharT('e'), CharT('l'), CharT('l'), CharT('o')}});
}

//
// Driver
//

template <class CharT, class TestFunction, class ExceptionTest>
void format_tests(TestFunction check, ExceptionTest check_exception) {
  test_char<CharT>(check, check_exception);
#ifndef TEST_HAS_NO_WIDE_CHARACTERS
  if (std::same_as<CharT, wchar_t>) // avoid testing twice
    test_char_to_wchar(check, check_exception);
#endif
  test_bool<CharT>(check, check_exception);
  test_int<CharT>(check, check_exception);
  test_floating_point<CharT>(check, check_exception);
  test_pointer<CharT>(check, check_exception);
  test_string<CharT>(check, check_exception);

  test_status<CharT>(check, check_exception); // Has its own handler with its own parser

  test_pair_tuple<CharT>(check, check_exception);
  test_tuple_int<CharT>(check, check_exception);
  test_tuple_int_int_int<CharT>(check, check_exception);

  test_with_ranges<CharT>(check, check_exception);

  test_adaptor<CharT>(check, check_exception);
}

#endif // TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FORMATTER_FORMAT_FUNCTIONS_TESTS_H
