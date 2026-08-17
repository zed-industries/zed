//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_TIME_TIME_SYN_FORMATTER_TESTS_H
#define TEST_STD_TIME_TIME_SYN_FORMATTER_TESTS_H

#include "assert_macros.h"
#include "concat_macros.h"
#include "make_string.h"
#include "string_literal.h"
#include "test_format_string.h"
#include "test_macros.h"

#include <algorithm>
#include <cassert>
#include <iostream>
#include <set>
#include <string>
#include <string_view>

#define STR(S) MAKE_STRING(CharT, S)
#define SV(S) MAKE_STRING_VIEW(CharT, S)

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
template <class CharT>
using format_context = std::conditional_t<std::same_as<CharT, char>, std::format_context, std::wformat_context>;
#else
template <class CharT>
using format_context = std::format_context;
#endif

template <class CharT, class... Args>
void check(std::basic_string_view<CharT> expected, test_format_string<CharT, Args...> fmt, Args&&... args) {
  std::basic_string<CharT> out = std::format(fmt, std::forward<Args>(args)...);
  TEST_REQUIRE(out == expected,
               TEST_WRITE_CONCATENATED(
                   "\nFormat string   ", fmt.get(), "\nExpected output ", expected, "\nActual output   ", out, '\n'));
}

template <class CharT, class... Args>
void check(const std::locale& loc,
           std::basic_string_view<CharT> expected,
           test_format_string<CharT, Args...> fmt,
           Args&&... args) {
  std::basic_string<CharT> out = std::format(loc, fmt, std::forward<Args>(args)...);
  TEST_REQUIRE(out == expected,
               TEST_WRITE_CONCATENATED(
                   "\nFormat string   ", fmt.get(), "\nExpected output ", expected, "\nActual output   ", out, '\n'));
}

template <class CharT, class... Args>
void check_exception([[maybe_unused]] std::string_view what,
                     [[maybe_unused]] std::basic_string_view<CharT> fmt,
                     [[maybe_unused]] const Args&... args) {
  TEST_VALIDATE_EXCEPTION(
      std::format_error,
      [&]([[maybe_unused]] const std::format_error& e) {
        TEST_LIBCPP_REQUIRE(
            e.what() == what,
            TEST_WRITE_CONCATENATED(
                "\nFormat string   ", fmt, "\nExpected exception ", what, "\nActual exception   ", e.what(), '\n'));
      },
      TEST_IGNORE_NODISCARD std::vformat(fmt, std::make_format_args<format_context<CharT>>(args...)));
}

template <class CharT, class T>
void check_invalid_type(const std::set<std::basic_string_view<CharT>>& valid_types,
                        std::string_view what,
                        std::basic_string<CharT> type,
                        const T& arg) {
  std::basic_string<CharT> fmt{STR("{:%") + type + STR("}")};

  if (valid_types.contains(std::basic_string_view<CharT>{type})) {
#ifndef TEST_HAS_NO_EXCEPTIONS
    try {
#endif
      TEST_IGNORE_NODISCARD std::vformat(
          std::basic_string_view<CharT>{fmt}, std::make_format_args<format_context<CharT>>(arg));
#ifndef TEST_HAS_NO_EXCEPTIONS
    } catch (const std::format_error& e) {
      (void)e;
      if constexpr (std::same_as<CharT, char>)
        std::cerr << "\nFormat string        " << fmt << "\nUnexpected exception " << e.what() << '\n';
      assert(false);
    }
#endif
  } else {
    check_exception(what, std::basic_string_view<CharT>{fmt}, arg);
  }
}

template <class CharT, class T>
void check_invalid_types(const std::set<std::basic_string_view<CharT>>& valid_types, const T& arg) {
  check_invalid_type(valid_types, "The supplied date time doesn't contain a weekday", STR("a"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a weekday", STR("A"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a month", STR("b"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a month", STR("B"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date and time", STR("c"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a year", STR("C"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a day", STR("d"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("D"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a day", STR("e"), arg);
  // E - the modifier is checked separately
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("f"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("F"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("g"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("G"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a month", STR("h"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain an hour", STR("H"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("i"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain an hour", STR("I"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date or duration", STR("j"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("J"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("k"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("K"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("l"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("L"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a month", STR("m"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a minute", STR("M"), arg);
  // n - valid
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("N"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("o"), arg);
  // O - the modifier is checked separately
  check_invalid_type(valid_types, "The supplied date time doesn't contain an hour", STR("p"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("P"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a duration", STR("q"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a duration", STR("Q"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time", STR("r"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time", STR("R"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("s"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a second", STR("S"), arg);
  // t - valid
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time", STR("T"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a weekday", STR("u"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("U"), arg);
  check_invalid_type(valid_types, "The date time type specifier is invalid", STR("v"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("V"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a weekday", STR("w"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("W"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("x"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time", STR("X"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a year", STR("y"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a year", STR("y"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time zone", STR("z"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time zone", STR("Z"), arg);

  // *** E modifier
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ea"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EA"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Eb"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EB"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date and time", STR("Ec"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a year", STR("EC"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ed"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("ED"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ee"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EE"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ef"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EF"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Eg"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EG"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Eh"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EH"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ei"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EI"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ej"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EJ"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ek"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EK"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("El"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EL"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Em"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EM"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("En"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EN"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Eo"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EO"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ep"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EP"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Eq"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EQ"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Er"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("ER"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Es"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("ES"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Et"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("ET"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Eu"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EU"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ev"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EV"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("Ew"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EW"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("Ex"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time", STR("EX"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a year", STR("Ey"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a year", STR("EY"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time zone", STR("Ez"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("EZ"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier E is invalid", STR("E%"), arg);

  // *** O modifier
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Oa"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OA"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Ob"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OB"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Oc"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OC"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a day", STR("Od"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OD"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a day", STR("Oe"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OE"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Of"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OF"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Og"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OG"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Oh"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain an hour", STR("OH"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Oi"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain an hour", STR("OI"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Oj"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OJ"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Ok"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OK"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Ol"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OL"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a month", STR("Om"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a minute", STR("OM"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("On"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("ON"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Oo"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OO"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Op"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OP"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Oq"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OQ"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Or"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OR"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Os"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a second", STR("OS"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Ot"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OT"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a weekday", STR("Ou"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("OU"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Ov"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("OV"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a weekday", STR("Ow"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a date", STR("OW"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("Ox"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OX"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a year", STR("Oy"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OY"), arg);
  check_invalid_type(valid_types, "The supplied date time doesn't contain a time zone", STR("Oz"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("OZ"), arg);
  check_invalid_type(valid_types, "The date time type specifier for modifier O is invalid", STR("O%"), arg);
}

#endif // TEST_STD_TIME_TIME_SYN_FORMATTER_TESTS_H
