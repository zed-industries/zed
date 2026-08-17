//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FMTMAP_FORMAT_FUNCTIONS_TESTS_H
#define TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FMTMAP_FORMAT_FUNCTIONS_TESTS_H

#include <algorithm>
#include <deque>
#include <flat_map>
#include <format>
#include <map>
#include <unordered_map>

#include "format.functions.common.h"
#include "make_string.h"
#include "platform_support.h" // locale name macros
#include "test_macros.h"

//
// Char
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_char(TestFunction check, ExceptionTest check_exception) {
  std::map<CharT, CharT> input{{CharT('a'), CharT('A')}, {CharT('c'), CharT('C')}, {CharT('b'), CharT('B')}};

  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{}"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}^42"), SV("{}^42"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}     "), SV("{:35}"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}*****"), SV("{:*<35}"), input);
  check(SV("__{'a': 'A', 'b': 'B', 'c': 'C'}___"), SV("{:_^35}"), input);
  check(SV("#####{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{:#>35}"), input);

  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}     "), SV("{:{}}"), input, 35);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}*****"), SV("{:*<{}}"), input, 35);
  check(SV("__{'a': 'A', 'b': 'B', 'c': 'C'}___"), SV("{:_^{}}"), input, 35);
  check(SV("#####{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{:#>{}}"), input, 35);

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
  check(SV("__'a': 'A', 'b': 'B', 'c': 'C'___"), SV("{:_^33n}"), input);

  // *** type ***
  check(SV("__{'a': 'A', 'b': 'B', 'c': 'C'}___"), SV("{:_^35m}"), input); // the m type does the same as the default.
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec

  check(SV("{'a': 'A'     , 'b': 'B'     , 'c': 'C'     }"), SV("{::13}"), input);
  check(SV("{'a': 'A'*****, 'b': 'B'*****, 'c': 'C'*****}"), SV("{::*<13}"), input);
  check(SV("{__'a': 'A'___, __'b': 'B'___, __'c': 'C'___}"), SV("{::_^13}"), input);
  check(SV("{#####'a': 'A', #####'b': 'B', #####'c': 'C'}"), SV("{::#>13}"), input);

  check(SV("{'a': 'A'     , 'b': 'B'     , 'c': 'C'     }"), SV("{::{}}"), input, 13);
  check(SV("{'a': 'A'*****, 'b': 'B'*****, 'c': 'C'*****}"), SV("{::*<{}}"), input, 13);
  check(SV("{__'a': 'A'___, __'b': 'B'___, __'c': 'C'___}"), SV("{::_^{}}"), input, 13);
  check(SV("{#####'a': 'A', #####'b': 'B', #####'c': 'C'}"), SV("{::#>{}}"), input, 13);

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
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{::m}"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{::n}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^{###'a': 'A', ###'b': 'B', ###'c': 'C'}^^^"), SV("{:^^44:#>11}"), input);
  check(SV("^^{###'a': 'A', ###'b': 'B', ###'c': 'C'}^^^"), SV("{:^^{}:#>11}"), input, 44);
  check(SV("^^{###'a': 'A', ###'b': 'B', ###'c': 'C'}^^^"), SV("{:^^{}:#>{}}"), input, 44, 11);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>11}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 44);
}

//
// char -> wchar_t
//

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
template <class TestFunction, class ExceptionTest>
void test_char_to_wchar(TestFunction check, ExceptionTest check_exception) {
  std::map<char, char> input{{'a', 'A'}, {'c', 'C'}, {'b', 'B'}};

  using CharT = wchar_t;
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{}"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}^42"), SV("{}^42"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}     "), SV("{:35}"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}*****"), SV("{:*<35}"), input);
  check(SV("__{'a': 'A', 'b': 'B', 'c': 'C'}___"), SV("{:_^35}"), input);
  check(SV("#####{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{:#>35}"), input);

  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}     "), SV("{:{}}"), input, 35);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}*****"), SV("{:*<{}}"), input, 35);
  check(SV("__{'a': 'A', 'b': 'B', 'c': 'C'}___"), SV("{:_^{}}"), input, 35);
  check(SV("#####{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{:#>{}}"), input, 35);

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
  check(SV("__'a': 'A', 'b': 'B', 'c': 'C'___"), SV("{:_^33n}"), input);

  // *** type ***
  check(SV("__{'a': 'A', 'b': 'B', 'c': 'C'}___"), SV("{:_^35m}"), input); // the m type does the same as the default.
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("{'a': 'A'     , 'b': 'B'     , 'c': 'C'     }"), SV("{::13}"), input);
  check(SV("{'a': 'A'*****, 'b': 'B'*****, 'c': 'C'*****}"), SV("{::*<13}"), input);
  check(SV("{__'a': 'A'___, __'b': 'B'___, __'c': 'C'___}"), SV("{::_^13}"), input);
  check(SV("{#####'a': 'A', #####'b': 'B', #####'c': 'C'}"), SV("{::#>13}"), input);

  check(SV("{'a': 'A'     , 'b': 'B'     , 'c': 'C'     }"), SV("{::{}}"), input, 13);
  check(SV("{'a': 'A'*****, 'b': 'B'*****, 'c': 'C'*****}"), SV("{::*<{}}"), input, 13);
  check(SV("{__'a': 'A'___, __'b': 'B'___, __'c': 'C'___}"), SV("{::_^{}}"), input, 13);
  check(SV("{#####'a': 'A', #####'b': 'B', #####'c': 'C'}"), SV("{::#>{}}"), input, 13);

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
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{::m}"), input);
  check(SV("{'a': 'A', 'b': 'B', 'c': 'C'}"), SV("{::n}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^{###'a': 'A', ###'b': 'B', ###'c': 'C'}^^^"), SV("{:^^44:#>11}"), input);
  check(SV("^^{###'a': 'A', ###'b': 'B', ###'c': 'C'}^^^"), SV("{:^^{}:#>11}"), input, 44);
  check(SV("^^{###'a': 'A', ###'b': 'B', ###'c': 'C'}^^^"), SV("{:^^{}:#>{}}"), input, 44, 11);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>11}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 44);
}
#endif // TEST_HAS_NO_WIDE_CHARACTERS

//
// Bool
//
template <class CharT, class TestFunction, class ExceptionTest>
void test_bool(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("{false: 0, true: 42, true: 1}"), SV("{}"), input);
  check(SV("{false: 0, true: 42, true: 1}^42"), SV("{}^42"), input);
  check(SV("{false: 0, true: 42, true: 1}^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("{false: 0, true: 42, true: 1}     "), SV("{:34}"), input);
  check(SV("{false: 0, true: 42, true: 1}*****"), SV("{:*<34}"), input);
  check(SV("__{false: 0, true: 42, true: 1}___"), SV("{:_^34}"), input);
  check(SV("#####{false: 0, true: 42, true: 1}"), SV("{:#>34}"), input);

  check(SV("{false: 0, true: 42, true: 1}     "), SV("{:{}}"), input, 34);
  check(SV("{false: 0, true: 42, true: 1}*****"), SV("{:*<{}}"), input, 34);
  check(SV("__{false: 0, true: 42, true: 1}___"), SV("{:_^{}}"), input, 34);
  check(SV("#####{false: 0, true: 42, true: 1}"), SV("{:#>{}}"), input, 34);

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
  check(SV("__false: 0, true: 42, true: 1___"), SV("{:_^32n}"), input);

  // *** type ***
  check(SV("__{false: 0, true: 42, true: 1}___"), SV("{:_^34m}"), input); // the m type does the same as the default.
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("{false: 0  , true: 42  , true: 1   }"), SV("{::10}"), input);
  check(SV("{false: 0**, true: 42**, true: 1***}"), SV("{::*<10}"), input);
  check(SV("{_false: 0_, _true: 42_, _true: 1__}"), SV("{::_^10}"), input);
  check(SV("{##false: 0, ##true: 42, ###true: 1}"), SV("{::#>10}"), input);

  check(SV("{false: 0  , true: 42  , true: 1   }"), SV("{::{}}"), input, 10);
  check(SV("{false: 0**, true: 42**, true: 1***}"), SV("{::*<{}}"), input, 10);
  check(SV("{_false: 0_, _true: 42_, _true: 1__}"), SV("{::_^{}}"), input, 10);
  check(SV("{##false: 0, ##true: 42, ###true: 1}"), SV("{::#>{}}"), input, 10);

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
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>(""))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^{##false: 0, ##true: 42, ###true: 1}^^^"), SV("{:^^41:#>10}"), input);
  check(SV("^^{##false: 0, ##true: 42, ###true: 1}^^^"), SV("{:^^{}:#>10}"), input, 41);
  check(SV("^^{##false: 0, ##true: 42, ###true: 1}^^^"), SV("{:^^{}:#>{}}"), input, 41, 10);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>10}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 41);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_bool(TestFunction check, ExceptionTest check_exception) {
  // duplicates are stored in order of insertion
  test_bool<CharT>(check, check_exception, std::multimap<bool, int>{{true, 42}, {false, 0}, {true, 1}});
#if TEST_STD_VER >= 23
  test_bool<CharT>(check,
                   check_exception,
                   std::flat_multimap<bool, int, std::less<bool>, std::deque<bool>>{{true, 42}, {false, 0}, {true, 1}});
#endif
}

//
// Integral
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_int(TestFunction check, ExceptionTest check_exception, auto&& input) {
  check(SV("{-42: 42, 1: -1, 42: -42}"), SV("{}"), input);
  check(SV("{-42: 42, 1: -1, 42: -42}^42"), SV("{}^42"), input);
  check(SV("{-42: 42, 1: -1, 42: -42}^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("{-42: 42, 1: -1, 42: -42}     "), SV("{:30}"), input);
  check(SV("{-42: 42, 1: -1, 42: -42}*****"), SV("{:*<30}"), input);
  check(SV("__{-42: 42, 1: -1, 42: -42}___"), SV("{:_^30}"), input);
  check(SV("#####{-42: 42, 1: -1, 42: -42}"), SV("{:#>30}"), input);

  check(SV("{-42: 42, 1: -1, 42: -42}     "), SV("{:{}}"), input, 30);
  check(SV("{-42: 42, 1: -1, 42: -42}*****"), SV("{:*<{}}"), input, 30);
  check(SV("__{-42: 42, 1: -1, 42: -42}___"), SV("{:_^{}}"), input, 30);
  check(SV("#####{-42: 42, 1: -1, 42: -42}"), SV("{:#>{}}"), input, 30);

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
  check(SV("__-42: 42, 1: -1, 42: -42___"), SV("{:_^28n}"), input);

  // *** type ***
  check(SV("__{-42: 42, 1: -1, 42: -42}___"), SV("{:_^30m}"), input); // the m type does the same as the default.
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("{-42: 42   , 1: -1     , 42: -42   }"), SV("{::10}"), input);
  check(SV("{-42: 42***, 1: -1*****, 42: -42***}"), SV("{::*<10}"), input);
  check(SV("{_-42: 42__, __1: -1___, _42: -42__}"), SV("{::_^10}"), input);
  check(SV("{###-42: 42, #####1: -1, ###42: -42}"), SV("{::#>10}"), input);

  check(SV("{-42: 42   , 1: -1     , 42: -42   }"), SV("{::{}}"), input, 10);
  check(SV("{-42: 42***, 1: -1*****, 42: -42***}"), SV("{::*<{}}"), input, 10);
  check(SV("{_-42: 42__, __1: -1___, _42: -42__}"), SV("{::_^{}}"), input, 10);
  check(SV("{###-42: 42, #####1: -1, ###42: -42}"), SV("{::#>{}}"), input, 10);

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
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>(""))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^{###-42: 42, #####1: -1, ###42: -42}^^^"), SV("{:^^41:#>10}"), input);
  check(SV("^^{###-42: 42, #####1: -1, ###42: -42}^^^"), SV("{:^^{}:#>10}"), input, 41);
  check(SV("^^{###-42: 42, #####1: -1, ###42: -42}^^^"), SV("{:^^{}:#>{}}"), input, 41, 10);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>10}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 41);
}

template <class CharT, class TestFunction, class ExceptionTest>
void test_int(TestFunction check, ExceptionTest check_exception) {
  test_int<CharT>(check, check_exception, std::map<int, int>{{1, -1}, {42, -42}, {-42, 42}});
#if TEST_STD_VER >= 23
  test_int<CharT>(check, check_exception, std::flat_map<int, int>{{1, -1}, {42, -42}, {-42, 42}});
#endif
}

//
// Floating point
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_floating_point(TestFunction check, ExceptionTest check_exception) {
  std::map<double, double> input{{1.0, -1.0}, {-42, 42}};

  check(SV("{-42: 42, 1: -1}"), SV("{}"), input);
  check(SV("{-42: 42, 1: -1}^42"), SV("{}^42"), input);
  check(SV("{-42: 42, 1: -1}^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("{-42: 42, 1: -1}     "), SV("{:21}"), input);
  check(SV("{-42: 42, 1: -1}*****"), SV("{:*<21}"), input);
  check(SV("__{-42: 42, 1: -1}___"), SV("{:_^21}"), input);
  check(SV("#####{-42: 42, 1: -1}"), SV("{:#>21}"), input);

  check(SV("{-42: 42, 1: -1}     "), SV("{:{}}"), input, 21);
  check(SV("{-42: 42, 1: -1}*****"), SV("{:*<{}}"), input, 21);
  check(SV("__{-42: 42, 1: -1}___"), SV("{:_^{}}"), input, 21);
  check(SV("#####{-42: 42, 1: -1}"), SV("{:#>{}}"), input, 21);

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
  check(SV("__-42: 42, 1: -1___"), SV("{:_^19n}"), input);

  // *** type ***
  check(SV("__{-42: 42, 1: -1}___"), SV("{:_^21m}"), input); // the m type does the same as the default.
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("{-42: 42   , 1: -1     }"), SV("{::10}"), input);
  check(SV("{-42: 42***, 1: -1*****}"), SV("{::*<10}"), input);
  check(SV("{_-42: 42__, __1: -1___}"), SV("{::_^10}"), input);
  check(SV("{###-42: 42, #####1: -1}"), SV("{::#>10}"), input);

  check(SV("{-42: 42   , 1: -1     }"), SV("{::{}}"), input, 10);
  check(SV("{-42: 42***, 1: -1*****}"), SV("{::*<{}}"), input, 10);
  check(SV("{_-42: 42__, __1: -1___}"), SV("{::_^{}}"), input, 10);
  check(SV("{###-42: 42, #####1: -1}"), SV("{::#>{}}"), input, 10);

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
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>(""))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^{###-42: 42, #####1: -1}^^^"), SV("{:^^29:#>10}"), input);
  check(SV("^^{###-42: 42, #####1: -1}^^^"), SV("{:^^{}:#>10}"), input, 29);
  check(SV("^^{###-42: 42, #####1: -1}^^^"), SV("{:^^{}:#>{}}"), input, 29, 10);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>10}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 29);
}

//
// Pointer
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_pointer(TestFunction check, ExceptionTest check_exception) {
  std::unordered_map<const void*, std::nullptr_t> input{{0, 0}};

  check(SV("{0x0: 0x0}"), SV("{}"), input);
  check(SV("{0x0: 0x0}^42"), SV("{}^42"), input);
  check(SV("{0x0: 0x0}^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("{0x0: 0x0}     "), SV("{:15}"), input);
  check(SV("{0x0: 0x0}*****"), SV("{:*<15}"), input);
  check(SV("__{0x0: 0x0}___"), SV("{:_^15}"), input);
  check(SV("#####{0x0: 0x0}"), SV("{:#>15}"), input);

  check(SV("{0x0: 0x0}     "), SV("{:{}}"), input, 15);
  check(SV("{0x0: 0x0}*****"), SV("{:*<{}}"), input, 15);
  check(SV("__{0x0: 0x0}___"), SV("{:_^{}}"), input, 15);
  check(SV("#####{0x0: 0x0}"), SV("{:#>{}}"), input, 15);

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
  check(SV("__0x0: 0x0___"), SV("{:_^13n}"), input);

  // *** type ***
  check(SV("__{0x0: 0x0}___"), SV("{:_^15m}"), input); // the m type does the same as the default.
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV("{0x0: 0x0     }"), SV("{::13}"), input);
  check(SV("{0x0: 0x0*****}"), SV("{::*<13}"), input);
  check(SV("{__0x0: 0x0___}"), SV("{::_^13}"), input);
  check(SV("{#####0x0: 0x0}"), SV("{::#>13}"), input);

  check(SV("{0x0: 0x0     }"), SV("{::{}}"), input, 13);
  check(SV("{0x0: 0x0*****}"), SV("{::*<{}}"), input, 13);
  check(SV("{__0x0: 0x0___}"), SV("{::_^{}}"), input, 13);
  check(SV("{#####0x0: 0x0}"), SV("{::#>{}}"), input, 13);

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
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>(""))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec
  check(SV("^^{###0x0: 0x0}^^^"), SV("{:^^18:#>11}"), input);
  check(SV("^^{###0x0: 0x0}^^^"), SV("{:^^{}:#>11}"), input, 18);
  check(SV("^^{###0x0: 0x0}^^^"), SV("{:^^{}:#>{}}"), input, 18, 11);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>11}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 18);
}

//
// String
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_string(TestFunction check, ExceptionTest check_exception) {
  std::map<std::basic_string<CharT>, std::basic_string<CharT>> input{
      {STR("hello"), STR("HELLO")}, {STR("world"), STR("WORLD")}};

  check(SV(R"({"hello": "HELLO", "world": "WORLD"})"), SV("{}"), input);
  check(SV(R"({"hello": "HELLO", "world": "WORLD"}^42)"), SV("{}^42"), input);
  check(SV(R"({"hello": "HELLO", "world": "WORLD"}^42)"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV(R"({"hello": "HELLO", "world": "WORLD"}     )"), SV("{:41}"), input);
  check(SV(R"({"hello": "HELLO", "world": "WORLD"}*****)"), SV("{:*<41}"), input);
  check(SV(R"(__{"hello": "HELLO", "world": "WORLD"}___)"), SV("{:_^41}"), input);
  check(SV(R"(#####{"hello": "HELLO", "world": "WORLD"})"), SV("{:#>41}"), input);

  check(SV(R"({"hello": "HELLO", "world": "WORLD"}     )"), SV("{:{}}"), input, 41);
  check(SV(R"({"hello": "HELLO", "world": "WORLD"}*****)"), SV("{:*<{}}"), input, 41);
  check(SV(R"(__{"hello": "HELLO", "world": "WORLD"}___)"), SV("{:_^{}}"), input, 41);
  check(SV(R"(#####{"hello": "HELLO", "world": "WORLD"})"), SV("{:#>{}}"), input, 41);

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
  check(SV(R"(__"hello": "HELLO", "world": "WORLD"___)"), SV("{:_^39n}"), input);

  // *** type ***
  check(SV(R"(__{"hello": "HELLO", "world": "WORLD"}___)"), SV("{:_^41m}"), input);
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>("s"))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Only underlying has a format-spec
  check(SV(R"({"hello": "HELLO"     , "world": "WORLD"     })"), SV("{::21}"), input);
  check(SV(R"({"hello": "HELLO"*****, "world": "WORLD"*****})"), SV("{::*<21}"), input);
  check(SV(R"({__"hello": "HELLO"___, __"world": "WORLD"___})"), SV("{::_^21}"), input);
  check(SV(R"({#####"hello": "HELLO", #####"world": "WORLD"})"), SV("{::#>21}"), input);

  check(SV(R"({"hello": "HELLO"     , "world": "WORLD"     })"), SV("{::{}}"), input, 21);
  check(SV(R"({"hello": "HELLO"*****, "world": "WORLD"*****})"), SV("{::*<{}}"), input, 21);
  check(SV(R"({__"hello": "HELLO"___, __"world": "WORLD"___})"), SV("{::_^{}}"), input, 21);
  check(SV(R"({#####"hello": "HELLO", #####"world": "WORLD"})"), SV("{::#>{}}"), input, 21);

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
  for (std::basic_string_view<CharT> fmt : fmt_invalid_nested_types<CharT>(""))
    check_exception("The format specifier should consume the input or end with a '}'", fmt, input);

  // ***** Both have a format-spec

  check(SV(R"(^^{#####"hello": "HELLO", #####"world": "WORLD"}^^^)"), SV("{:^^51:#>21}"), input);
  check(SV(R"(^^{#####"hello": "HELLO", #####"world": "WORLD"}^^^)"), SV("{:^^{}:#>21}"), input, 51);
  check(SV(R"(^^{#####"hello": "HELLO", #####"world": "WORLD"}^^^)"), SV("{:^^{}:#>{}}"), input, 51, 21);

  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>21}"), input);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("{:^^{}:#>{}}"), input, 51);
}

//
// Handle
//

template <class CharT, class TestFunction, class ExceptionTest>
void test_status(TestFunction check, ExceptionTest check_exception) {
  std::unordered_multimap<status, status> input{{status::foobar, status::foo}, {status::foobar, status::bar}};

  check(SV("{0xaa55: 0xaaaa, 0xaa55: 0x5555}"), SV("{}"), input);
  check(SV("{0xaa55: 0xaaaa, 0xaa55: 0x5555}^42"), SV("{}^42"), input);
  check(SV("{0xaa55: 0xaaaa, 0xaa55: 0x5555}^42"), SV("{:}^42"), input);

  // ***** underlying has no format-spec

  // *** align-fill & width ***
  check(SV("{0xaa55: 0xaaaa, 0xaa55: 0x5555}     "), SV("{:37}"), input);
  check(SV("{0xaa55: 0xaaaa, 0xaa55: 0x5555}*****"), SV("{:*<37}"), input);
  check(SV("__{0xaa55: 0xaaaa, 0xaa55: 0x5555}___"), SV("{:_^37}"), input);
  check(SV("#####{0xaa55: 0xaaaa, 0xaa55: 0x5555}"), SV("{:#>37}"), input);

  check(SV("{0xaa55: 0xaaaa, 0xaa55: 0x5555}     "), SV("{:{}}"), input, 37);
  check(SV("{0xaa55: 0xaaaa, 0xaa55: 0x5555}*****"), SV("{:*<{}}"), input, 37);
  check(SV("__{0xaa55: 0xaaaa, 0xaa55: 0x5555}___"), SV("{:_^{}}"), input, 37);
  check(SV("#####{0xaa55: 0xaaaa, 0xaa55: 0x5555}"), SV("{:#>{}}"), input, 37);

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
  check(SV("__0xaa55: 0xaaaa, 0xaa55: 0x5555___"), SV("{:_^35n}"), input);

  // *** type ***
  check(SV("__{0xaa55: 0xaaaa, 0xaa55: 0x5555}___"), SV("{:_^37}"), input); // the m type does the same as the default.
  check_exception("Type s requires character type as formatting argument", SV("{:s}"), input);
  check_exception("Type ?s requires character type as formatting argument", SV("{:?s}"), input);

  // Underlying can't have a format-spec
}

//
// Adaptor
//

class adaptor {
  using adaptee = std::map<int, int>;

public:
  using key_type    = typename adaptee::key_type;
  using mapped_type = typename adaptee::mapped_type;
  using iterator    = typename adaptee::iterator;

  iterator begin() { return data_.begin(); }
  iterator end() { return data_.end(); }

  explicit adaptor(std::map<int, int>&& data) : data_(std::move(data)) {}

private:
  adaptee data_;
};

static_assert(std::format_kind<adaptor> == std::range_format::map);

template <class CharT, class TestFunction, class ExceptionTest>
void test_adaptor(TestFunction check, ExceptionTest check_exception) {
  test_int<CharT>(check, check_exception, adaptor{std::map<int, int>{{1, -1}, {42, -42}, {-42, 42}}});
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

  test_status<CharT>(check, check_exception);

  test_adaptor<CharT>(check, check_exception);
}

#endif // TEST_STD_UTILITIES_FORMAT_FORMAT_RANGE_FORMAT_RANGE_FMTMAP_FORMAT_FUNCTIONS_TESTS_H
