//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FMTSTR_FORMAT_FUNCTIONS_TESTS_H
#define TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FMTSTR_FORMAT_FUNCTIONS_TESTS_H

#include <array>
#include <format>
#include <list>

#include "format.functions.common.h"
#include "make_string.h"
#include "platform_support.h" // locale name macros
#include "test_macros.h"

//
// Types
//

template <class Container>
class test_range_format_string {
public:
  explicit test_range_format_string(Container str) : str_(std::move(str)) {}

  typename Container::const_iterator begin() const { return str_.begin(); }
  typename Container::const_iterator end() const { return str_.end(); }

private:
  Container str_;
};

template <class Container>
constexpr std::range_format std::format_kind<test_range_format_string<Container>> = std::range_format::string;

template <class Container>
class test_range_format_debug_string {
public:
  explicit test_range_format_debug_string(Container str) : str_(std::move(str)) {}

  typename Container::const_iterator begin() const { return str_.begin(); }
  typename Container::const_iterator end() const { return str_.end(); }

private:
  Container str_;
};

template <class Container>
constexpr std::range_format std::format_kind<test_range_format_debug_string<Container>> =
    std::range_format::debug_string;

//
// String
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_string(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("hello"), SV("{}"), input);
  check(SV("hello^42"), SV("{}^42"), input);
  check(SV("hello^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("hello     "), SV("{:10}"), input);
  check(SV("hello*****"), SV("{:*<10}"), input);
  check(SV("__hello___"), SV("{:_^10}"), input);
  check(SV(":::::hello"), SV("{::>10}"), input);

  check(SV("hello     "), SV("{:{}}"), input, 10);
  check(SV("hello*****"), SV("{:*<{}}"), input, 10);
  check(SV("__hello___"), SV("{:_^{}}"), input, 10);
  check(SV(":::::hello"), SV("{::>{}}"), input, 10);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), input);

  // *** precision ***
  check(SV("hel"), SV("{:.3}"), input);
  check(SV("hel"), SV("{:.{}}"), input, 3);

  check(SV("hel  "), SV("{:5.3}"), input);
  check(SV("hel  "), SV("{:{}.{}}"), input, 5, 3);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), input);

  // *** type ***
  check(SV("hello"), SV("{:s}"), input);
  check(SV("\"hello\""), SV("{:?}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s?"))
    check_exception("The type option contains an invalid value for a string formatting argument", fmt, input);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_string(TestFunction check, ExceptionTest check_exception) {
  // libc++ uses different containers for contiguous and non-contiguous ranges.
  std::basic_string<CharT> input = STR("hello");
  test_string<CharT>(check, check_exception, test_range_format_string<std::basic_string<CharT>>{input});
  test_string<CharT>(check, check_exception, test_range_format_string<std::basic_string_view<CharT>>{input});
  test_string<CharT>(
      check, check_exception, test_range_format_string<std::list<CharT>>{std::list<CharT>{input.begin(), input.end()}});
}

//
// String range
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_range_string(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV(R"([Hello, world])"), SV("{}"), input);
  check(SV(R"([Hello, world]^42)"), SV("{}^42"), input);
  check(SV(R"([Hello, world]^42)"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV(R"([Hello, world]     )"), SV("{:19}"), input);
  check(SV(R"([Hello, world]*****)"), SV("{:*<19}"), input);
  check(SV(R"(__[Hello, world]___)"), SV("{:_^19}"), input);
  check(SV(R"(#####[Hello, world])"), SV("{:#>19}"), input);

  check(SV(R"([Hello, world]     )"), SV("{:{}}"), input, 19);
  check(SV(R"([Hello, world]*****)"), SV("{:*<{}}"), input, 19);
  check(SV(R"(__[Hello, world]___)"), SV("{:_^{}}"), input, 19);
  check(SV(R"(#####[Hello, world])"), SV("{:#>{}}"), input, 19);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), input);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), input);

  // *** n
  check(SV(R"(_Hello, world_)"), SV("{:_^14n}"), input);

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
void test_range_string(TestFunction check, ExceptionTest check_exception) {
  // libc++ uses different containers for contiguous and non-contiguous ranges.
  std::array input{STR("Hello"), STR("world")};
  test_range_string<CharT>(
      check,
      check_exception,
      std::array{test_range_format_string<std::basic_string<CharT>>{input[0]},
                 test_range_format_string<std::basic_string<CharT>>{input[1]}});
  test_range_string<CharT>(
      check,
      check_exception,
      std::array{test_range_format_string<std::basic_string_view<CharT>>{input[0]},
                 test_range_format_string<std::basic_string_view<CharT>>{input[1]}});
  test_range_string<CharT>(
      check,
      check_exception,
      std::array{test_range_format_string<std::list<CharT>>{std::list<CharT>{input[0].begin(), input[0].end()}},
                 test_range_format_string<std::list<CharT>>{std::list<CharT>{input[1].begin(), input[1].end()}}});
  test_range_string<CharT>(
      check,
      check_exception,
      std::list{test_range_format_string<std::list<CharT>>{std::list<CharT>{input[0].begin(), input[0].end()}},
                test_range_format_string<std::list<CharT>>{std::list<CharT>{input[1].begin(), input[1].end()}}});
}

//
// Debug string
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_debug_string(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("\"hello\""), SV("{}"), input);
  check(SV("\"hello\"^42"), SV("{}^42"), input);
  check(SV("\"hello\"^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("\"hello\"     "), SV("{:12}"), input);
  check(SV("\"hello\"*****"), SV("{:*<12}"), input);
  check(SV("__\"hello\"___"), SV("{:_^12}"), input);
  check(SV(":::::\"hello\""), SV("{::>12}"), input);

  check(SV("\"hello\"     "), SV("{:{}}"), input, 12);
  check(SV("\"hello\"*****"), SV("{:*<{}}"), input, 12);
  check(SV("__\"hello\"___"), SV("{:_^{}}"), input, 12);
  check(SV(":::::\"hello\""), SV("{::>{}}"), input, 12);

  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), input);

  // *** precision ***
  check(SV("\"he"), SV("{:.3}"), input);
  check(SV("\"he"), SV("{:.{}}"), input, 3);

  check(SV("\"he  "), SV("{:5.3}"), input);
  check(SV("\"he  "), SV("{:{}.{}}"), input, 5, 3);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), input);

  // *** type ***
  check(SV("\"hello\""), SV("{:s}"), input); // escape overrides the type option s
  check(SV("\"hello\""), SV("{:?}"), input);
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s?"))
    check_exception("The type option contains an invalid value for a string formatting argument", fmt, input);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_debug_string(TestFunction check, ExceptionTest check_exception) {
  // libc++ uses different containers for contiguous and non-contiguous ranges.
  std::basic_string<CharT> input = STR("hello");
  test_debug_string<CharT>(check, check_exception, test_range_format_debug_string<std::basic_string<CharT>>{input});
  test_debug_string<CharT>(
      check, check_exception, test_range_format_debug_string<std::basic_string_view<CharT>>{input});
  test_debug_string<CharT>(
      check,
      check_exception,
      test_range_format_debug_string<std::list<CharT>>{std::list<CharT>{input.begin(), input.end()}});
}

//
// Debug string range
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_range_debug_string(TestFunction check, ExceptionTest check_exception, auto&& input) {
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
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-}"), input);

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
  check(SV(R"(["Hello"   , "world"   ])"), SV("{::10}"), input);
  check(SV(R"(["Hello"***, "world"***])"), SV("{::*<10}"), input);
  check(SV(R"([_"Hello"__, _"world"__])"), SV("{::_^10}"), input);
  check(SV(R"([:::"Hello", :::"world"])"), SV("{:::>10}"), input);

  check(SV(R"(["Hello"   , "world"   ])"), SV("{::{}}"), input, 10);
  check(SV(R"(["Hello"***, "world"***])"), SV("{::*<{}}"), input, 10);
  check(SV(R"([_"Hello"__, _"world"__])"), SV("{::_^{}}"), input, 10);
  check(SV(R"([:::"Hello", :::"world"])"), SV("{:::>{}}"), input, 10);

  check_exception("The format string contains an invalid escape sequence", SV("{::}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{::{<}"), input);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::-}"), input);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{::05}"), input);

  // *** precision ***
  check(SV(R"(["He, "wo])"), SV("{::.3}"), input);

  check(SV(R"(["He, "wo])"), SV("{::.{}}"), input, 3);

  check_exception("The precision option does not contain a value or an argument index", SV("{::.}"), input);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{::L}"), input);

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>("s?"))
    check_exception("The type option contains an invalid value for a string formatting argument", fmt, input);

  // ***** Both have a format-spec
  check(SV(R"(^^[:::"Hello", :::"world"]^^^)"), SV("{:^^29::>10}"), input);
  check(SV(R"(^^[:::"Hello", :::"world"]^^^)"), SV("{:^^{}::>10}"), input, 29);
  check(SV(R"(^^[:::"Hello", :::"world"]^^^)"), SV("{:^^{}::>{}}"), input, 29, 10);

  check(SV(R"(^^[:::"Hello", :::"world"]^^^)"), SV("{:^^29::>10}"), input);
  check(SV(R"(^^[:::"Hello", :::"world"]^^^)"), SV("{:^^{}::>10}"), input, 29);
  check(SV(R"(^^[:::"Hello", :::"world"]^^^)"), SV("{:^^{}::>{}}"), input, 29, 10);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>10}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}::>{}}"), input, 29);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_range_debug_string(TestFunction check, ExceptionTest check_exception) {
  // libc++ uses different containers for contiguous and non-contiguous ranges.
  std::array input{STR("Hello"), STR("world")};
  test_range_debug_string<CharT>(
      check,
      check_exception,
      std::array{test_range_format_debug_string<std::basic_string<CharT>>{input[0]},
                 test_range_format_debug_string<std::basic_string<CharT>>{input[1]}});
  test_range_debug_string<CharT>(
      check,
      check_exception,
      std::array{test_range_format_debug_string<std::basic_string_view<CharT>>{input[0]},
                 test_range_format_debug_string<std::basic_string_view<CharT>>{input[1]}});
  test_range_debug_string<CharT>(
      check,
      check_exception,
      std::array{test_range_format_debug_string<std::list<CharT>>{std::list<CharT>{input[0].begin(), input[0].end()}},
                 test_range_format_debug_string<std::list<CharT>>{std::list<CharT>{input[1].begin(), input[1].end()}}});
  test_range_debug_string<CharT>(
      check,
      check_exception,
      std::list{test_range_format_debug_string<std::list<CharT>>{std::list<CharT>{input[0].begin(), input[0].end()}},
                test_range_format_debug_string<std::list<CharT>>{std::list<CharT>{input[1].begin(), input[1].end()}}});
}

//
// Driver
//

template <class CharT, class TestFunction, class ExceptionTest>
void format_tests(TestFunction check, ExceptionTest check_exception) {
  test_string<CharT>(check, check_exception);
  test_range_string<CharT>(check, check_exception);

  test_debug_string<CharT>(check, check_exception);
  test_range_debug_string<CharT>(check, check_exception);
}

#endif //  TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FMTSTR_FORMAT_FUNCTIONS_TESTS_H
