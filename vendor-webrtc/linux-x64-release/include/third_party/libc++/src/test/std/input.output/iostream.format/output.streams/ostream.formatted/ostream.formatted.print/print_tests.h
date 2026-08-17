//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_INPUT_OUTPUT_IOSTREAM_FORMAT_OUTPUT_STREAMS_OSTREAM_FORMATTED_OSTREAM_FORMATTED_PRINT_PRINT_TESTS_H
#define TEST_STD_INPUT_OUTPUT_IOSTREAM_FORMAT_OUTPUT_STREAMS_OSTREAM_FORMATTED_OSTREAM_FORMATTED_PRINT_PRINT_TESTS_H

template <class TestFunction, class ExceptionTest>
void print_tests(TestFunction check, ExceptionTest check_exception) {
  // *** Test escaping  ***

  check("{", "{{");
  check("{:^}", "{{:^}}");
  check("{: ^}", "{{:{}^}}", ' ');
  check("{:{}^}", "{{:{{}}^}}");
  check("{:{ }^}", "{{:{{{}}}^}}", ' ');

  // *** Test argument ID ***
  check("hello false true", "hello {0:} {1:}", false, true);
  check("hello true false", "hello {1:} {0:}", false, true);

  // *** Test many arguments ***
  check(
      "1234567890\t1234567890",
      "{}{}{}{}{}{}{}{}{}{}\t{}{}{}{}{}{}{}{}{}{}",
      1,
      2,
      3,
      4,
      5,
      6,
      7,
      8,
      9,
      0,
      1,
      2,
      3,
      4,
      5,
      6,
      7,
      8,
      9,
      0);

  // *** Test embedded NUL character ***
  using namespace std::literals;
  check("hello\0world"sv, "hello{}{}", '\0', "world");
  check("hello\0world"sv, "hello\0{}"sv, "world");
  check("hello\0world"sv, "hello{}", "\0world"sv);

  // *** Test Unicode ***
  // 2-byte code points
  check("\u00a1"sv, "{}"sv, "\u00a1");  // INVERTED EXCLAMATION MARK
  check("\u07ff"sv, "{:}"sv, "\u07ff"); // NKO TAMAN SIGN

  // 3-byte code points
  check("\u0800"sv, "{}"sv, "\u0800"); // SAMARITAN LETTER ALAF
  check("\ufffd"sv, "{}"sv, "\ufffd"); // REPLACEMENT CHARACTER

  // 4-byte code points
  check("\U00010000"sv, "{}"sv, "\U00010000"); // LINEAR B SYLLABLE B008 A
  check("\U0010FFFF"sv, "{}"sv, "\U0010FFFF"); // Undefined Character

  // *** Test invalid format strings ***
  check_exception("The format string terminates at a '{'", "{");
  check_exception("The replacement field misses a terminating '}'", "{:", 42);

  check_exception("The format string contains an invalid escape sequence", "}");
  check_exception("The format string contains an invalid escape sequence", "{:}-}", 42);

  check_exception("The format string contains an invalid escape sequence", "} ");
  check_exception("The argument index starts with an invalid character", "{-", 42);
  check_exception("The argument index value is too large for the number of arguments supplied", "hello {}");
  check_exception("The argument index value is too large for the number of arguments supplied", "hello {0}");
  check_exception("The argument index value is too large for the number of arguments supplied", "hello {1}", 42);
}

#endif // TEST_STD_INPUT_OUTPUT_IOSTREAM_FORMAT_OUTPUT_STREAMS_OSTREAM_FORMATTED_OSTREAM_FORMATTED_PRINT_PRINT_TESTS_H
