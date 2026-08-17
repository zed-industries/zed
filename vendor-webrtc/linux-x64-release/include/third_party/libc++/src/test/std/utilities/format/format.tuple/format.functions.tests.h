//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_UTILITIES_FORMAT_FORMAT_TUPLE_FORMAT_TESTS_H
#define TEST_STD_UTILITIES_FORMAT_FORMAT_TUPLE_FORMAT_TESTS_H

#include <concepts>
#include <format>
#include <tuple>

#include "format.functions.common.h"

enum class color { black, red, gold };

template <class CharT>
struct std::formatter<color, CharT> : std::formatter<basic_string_view<CharT>, CharT> {
  static constexpr basic_string_view<CharT> color_names[] = {SV("black"), SV("red"), SV("gold")};
  auto format(color c, auto& ctx) const {
    return formatter<basic_string_view<CharT>, CharT>::format(color_names[static_cast<int>(c)], ctx);
  }
};

//
// Generic tests for a tuple and pair with two elements.
//
template <class CharT, class TestFunction, class ExceptionTest, class TupleOrPair>
void test_tuple_or_pair_int_int(TestFunction check, ExceptionTest check_exception, TupleOrPair&& input) {
  check(SV("(42, 99)"), SV("{}"), input);
  check(SV("(42, 99)^42"), SV("{}^42"), input);
  check(SV("(42, 99)^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("(42, 99)     "), SV("{:13}"), input);
  check(SV("(42, 99)*****"), SV("{:*<13}"), input);
  check(SV("__(42, 99)___"), SV("{:_^13}"), input);
  check(SV("#####(42, 99)"), SV("{:#>13}"), input);

  check(SV("(42, 99)     "), SV("{:{}}"), input, 13);
  check(SV("(42, 99)*****"), SV("{:*<{}}"), input, 13);
  check(SV("__(42, 99)___"), SV("{:_^{}}"), input, 13);
  check(SV("#####(42, 99)"), SV("{:#>{}}"), input, 13);

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

  // *** type ***
  check(SV("__42: 99___"), SV("{:_^11m}"), input);
  check(SV("__42, 99___"), SV("{:_^11n}"), input);

  for (CharT c : SV("aAbBcdeEfFgGopPsxX?")) {
    check_exception("The format specifier should consume the input or end with a '}'",
                    std::basic_string_view{STR("{:") + c + STR("}")},
                    input);
  }
}

template <class CharT, class TestFunction, class ExceptionTest, class TupleOrPair>
void test_tuple_or_pair_int_string(TestFunction check, ExceptionTest check_exception, TupleOrPair&& input) {
  check(SV("(42, \"hello\")"), SV("{}"), input);
  check(SV("(42, \"hello\")^42"), SV("{}^42"), input);
  check(SV("(42, \"hello\")^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("(42, \"hello\")     "), SV("{:18}"), input);
  check(SV("(42, \"hello\")*****"), SV("{:*<18}"), input);
  check(SV("__(42, \"hello\")___"), SV("{:_^18}"), input);
  check(SV("#####(42, \"hello\")"), SV("{:#>18}"), input);

  check(SV("(42, \"hello\")     "), SV("{:{}}"), input, 18);
  check(SV("(42, \"hello\")*****"), SV("{:*<{}}"), input, 18);
  check(SV("__(42, \"hello\")___"), SV("{:_^{}}"), input, 18);
  check(SV("#####(42, \"hello\")"), SV("{:#>{}}"), input, 18);

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

  // *** type ***
  check(SV("__42: \"hello\"___"), SV("{:_^16m}"), input);
  check(SV("__42, \"hello\"___"), SV("{:_^16n}"), input);

  for (CharT c : SV("aAbBcdeEfFgGopPsxX?")) {
    check_exception("The format specifier should consume the input or end with a '}'",
                    std::basic_string_view{STR("{:") + c + STR("}")},
                    input);
  }
}

template <class CharT, class TestFunction, class TupleOrPair>
void test_escaping(TestFunction check, TupleOrPair&& input) {
  static_assert(std::same_as<std::remove_cvref_t<decltype(std::get<0>(input))>, CharT>);
  static_assert(std::same_as<std::remove_cvref_t<decltype(std::get<1>(input))>, std::basic_string<CharT>>);

  check(SV(R"(('*', ""))"), SV("{}"), input);

  // Char
  std::get<0>(input) = CharT('\t');
  check(SV(R"(('\t', ""))"), SV("{}"), input);
  std::get<0>(input) = CharT('\n');
  check(SV(R"(('\n', ""))"), SV("{}"), input);
  std::get<0>(input) = CharT('\0');
  check(SV(R"(('\u{0}', ""))"), SV("{}"), input);

  // String
  std::get<0>(input) = CharT('*');
  std::get<1>(input) = SV("hellö");
  check(SV("('*', \"hellö\")"), SV("{}"), input);
}

//
// pair tests
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_pair_int_int(TestFunction check, ExceptionTest check_exception) {
  test_tuple_or_pair_int_int<CharT>(check, check_exception, std::make_pair(42, 99));
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_pair_int_string(TestFunction check, ExceptionTest check_exception) {
  test_tuple_or_pair_int_string<CharT>(check, check_exception, std::make_pair(42, SV("hello")));
  test_tuple_or_pair_int_string<CharT>(check, check_exception, std::make_pair(42, STR("hello")));
  test_tuple_or_pair_int_string<CharT>(check, check_exception, std::make_pair(42, CSTR("hello")));
}

//
// tuple tests
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_tuple_int(TestFunction check, ExceptionTest check_exception) {
  auto input = std::make_tuple(42);

  check(SV("(42)"), SV("{}"), input);
  check(SV("(42)^42"), SV("{}^42"), input);
  check(SV("(42)^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("(42)     "), SV("{:9}"), input);
  check(SV("(42)*****"), SV("{:*<9}"), input);
  check(SV("__(42)___"), SV("{:_^9}"), input);
  check(SV("#####(42)"), SV("{:#>9}"), input);

  check(SV("(42)     "), SV("{:{}}"), input, 9);
  check(SV("(42)*****"), SV("{:*<{}}"), input, 9);
  check(SV("__(42)___"), SV("{:_^{}}"), input, 9);
  check(SV("#####(42)"), SV("{:#>{}}"), input, 9);

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

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check(SV("__42___"), SV("{:_^7n}"), input);

  for (CharT c : SV("aAbBcdeEfFgGopPsxX?")) {
    check_exception("The format specifier should consume the input or end with a '}'",
                    std::basic_string_view{STR("{:") + c + STR("}")},
                    input);
  }
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_tuple_int_string_color(TestFunction check, ExceptionTest check_exception) {
  const auto input = std::make_tuple(42, SV("hello"), color::red);

  check(SV("(42, \"hello\", \"red\")"), SV("{}"), input);
  check(SV("(42, \"hello\", \"red\")^42"), SV("{}^42"), input);
  check(SV("(42, \"hello\", \"red\")^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("(42, \"hello\", \"red\")     "), SV("{:25}"), input);
  check(SV("(42, \"hello\", \"red\")*****"), SV("{:*<25}"), input);
  check(SV("__(42, \"hello\", \"red\")___"), SV("{:_^25}"), input);
  check(SV("#####(42, \"hello\", \"red\")"), SV("{:#>25}"), input);

  check(SV("(42, \"hello\", \"red\")     "), SV("{:{}}"), input, 25);
  check(SV("(42, \"hello\", \"red\")*****"), SV("{:*<{}}"), input, 25);
  check(SV("__(42, \"hello\", \"red\")___"), SV("{:_^{}}"), input, 25);
  check(SV("#####(42, \"hello\", \"red\")"), SV("{:#>{}}"), input, 25);

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

  // *** type ***
  check_exception("Type m requires a pair or a tuple with two elements", SV("{:m}"), input);
  check(SV("__42, \"hello\", \"red\"___"), SV("{:_^23n}"), input);

  for (CharT c : SV("aAbBcdeEfFgGopPsxX?")) {
    check_exception("The format specifier should consume the input or end with a '}'",
                    std::basic_string_view{STR("{:") + c + STR("}")},
                    input);
  }
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_tuple_int_int(TestFunction check, ExceptionTest check_exception) {
  test_tuple_or_pair_int_int<CharT>(check, check_exception, std::make_tuple(42, 99));
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_tuple_int_string(TestFunction check, ExceptionTest check_exception) {
  test_tuple_or_pair_int_string<CharT>(check, check_exception, std::make_tuple(42, SV("hello")));
  test_tuple_or_pair_int_string<CharT>(check, check_exception, std::make_tuple(42, STR("hello")));
  test_tuple_or_pair_int_string<CharT>(check, check_exception, std::make_tuple(42, CSTR("hello")));
}

//
// nested tests
//

template <class CharT, class TestFunction, class ExceptionTest, class Nested>
void test_nested(TestFunction check, ExceptionTest check_exception, Nested&& input) {
  // [format.formatter.spec]/2
  //   A debug-enabled specialization of formatter additionally provides a
  //   public, constexpr, non-static member function set_debug_format()
  //   which modifies the state of the formatter to be as if the type of the
  //   std-format-spec parsed by the last call to parse were ?.
  // pair and tuple are not debug-enabled specializations to the
  // set_debug_format is not propagated. The paper
  //   P2733 Fix handling of empty specifiers in std::format
  // addressed this.

  check(SV("(42, (\"hello\", \"red\"))"), SV("{}"), input);
  check(SV("(42, (\"hello\", \"red\"))^42"), SV("{}^42"), input);
  check(SV("(42, (\"hello\", \"red\"))^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("(42, (\"hello\", \"red\"))     "), SV("{:27}"), input);
  check(SV("(42, (\"hello\", \"red\"))*****"), SV("{:*<27}"), input);
  check(SV("__(42, (\"hello\", \"red\"))___"), SV("{:_^27}"), input);
  check(SV("#####(42, (\"hello\", \"red\"))"), SV("{:#>27}"), input);

  check(SV("(42, (\"hello\", \"red\"))     "), SV("{:{}}"), input, 27);
  check(SV("(42, (\"hello\", \"red\"))*****"), SV("{:*<{}}"), input, 27);
  check(SV("__(42, (\"hello\", \"red\"))___"), SV("{:_^{}}"), input, 27);
  check(SV("#####(42, (\"hello\", \"red\"))"), SV("{:#>{}}"), input, 27);

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

  // *** type ***
  check(SV("__42: (\"hello\", \"red\")___"), SV("{:_^25m}"), input);
  check(SV("__42, (\"hello\", \"red\")___"), SV("{:_^25n}"), input);

  for (CharT c : SV("aAbBcdeEfFgGopPsxX?")) {
    check_exception("The format specifier should consume the input or end with a '}'",
                    std::basic_string_view{STR("{:") + c + STR("}")},
                    input);
  }
}

template <class CharT, class TestFunction, class ExceptionTest>
void run_tests(TestFunction check, ExceptionTest check_exception) {
  test_pair_int_int<CharT>(check, check_exception);
  test_pair_int_string<CharT>(check, check_exception);

  test_tuple_int<CharT>(check, check_exception);
  test_tuple_int_int<CharT>(check, check_exception);
  test_tuple_int_string<CharT>(check, check_exception);
  test_tuple_int_string_color<CharT>(check, check_exception);

  test_nested<CharT>(check, check_exception, std::make_pair(42, std::make_pair(SV("hello"), color::red)));
  test_nested<CharT>(check, check_exception, std::make_pair(42, std::make_tuple(SV("hello"), color::red)));
  test_nested<CharT>(check, check_exception, std::make_tuple(42, std::make_pair(SV("hello"), color::red)));
  test_nested<CharT>(check, check_exception, std::make_tuple(42, std::make_tuple(SV("hello"), color::red)));

  test_escaping<CharT>(check, std::make_pair(CharT('*'), STR("")));
  test_escaping<CharT>(check, std::make_tuple(CharT('*'), STR("")));

  // Test const ref-qualified types.
  // clang-format off
  check(SV("(42)"), SV("{}"), std::tuple<      int  >{42});
  check(SV("(42)"), SV("{}"), std::tuple<const int  >{42});

  int answer = 42;
  check(SV("(42)"), SV("{}"), std::tuple<      int& >{answer});
  check(SV("(42)"), SV("{}"), std::tuple<const int& >{answer});

  check(SV("(42)"), SV("{}"), std::tuple<      int&&>{42});
  check(SV("(42)"), SV("{}"), std::tuple<const int&&>{42});
  // clang-format on
}

#endif // TEST_STD_UTILITIES_FORMAT_FORMAT_TUPLE_FORMAT_TESTS_H
