//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_THREAD_THREAD_THREADS_THREAD_THREAD_CLASS_THREAD_THREAD_ID_FORMAT_FUNCTIONS_TESTS_H
#define TEST_STD_THREAD_THREAD_THREADS_THREAD_THREAD_CLASS_THREAD_THREAD_ID_FORMAT_FUNCTIONS_TESTS_H

#include <thread>

#include "format.functions.common.h"
#include "test_macros.h"

template <class CharT, class TestFunction, class ExceptionTest>
void format_tests(TestFunction check, ExceptionTest check_exception) {
  // Note the output of std::thread::id is unspecified. The output text is the
  // same as the stream operator. Since that format is already released this
  // test follows the practice on existing systems.
  std::thread::id input{};

  /***** Test the type specific part *****/
#if !defined(__APPLE__) && !defined(__FreeBSD__)
  check(SV("0"), SV("{}"), input);
  check(SV("0^42"), SV("{}^42"), input);
  check(SV("0^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("    0"), SV("{:5}"), input);
  check(SV("0****"), SV("{:*<5}"), input);
  check(SV("__0__"), SV("{:_^5}"), input);
  check(SV("::::0"), SV("{::>5}"), input); // This is not a range, so : is allowed as fill character.

  check(SV("    0"), SV("{:{}}"), input, 5);
  check(SV("0****"), SV("{:*<{}}"), input, 5);
  check(SV("__0__"), SV("{:_^{}}"), input, 5);
  check(SV("####0"), SV("{:#>{}}"), input, 5);
#else  // !defined(__APPLE__) && !defined(__FreeBSD__)
  check(SV("0x0"), SV("{}"), input);
  check(SV("0x0^42"), SV("{}^42"), input);
  check(SV("0x0^42"), SV("{:}^42"), input);

  // *** align-fill & width ***
  check(SV("    0x0"), SV("{:7}"), input);
  check(SV("0x0****"), SV("{:*<7}"), input);
  check(SV("__0x0__"), SV("{:_^7}"), input);
  check(SV("::::0x0"), SV("{::>7}"), input); // This is not a range, so : is allowed as fill character.

  check(SV("    0x0"), SV("{:{}}"), input, 7);
  check(SV("0x0****"), SV("{:*<{}}"), input, 7);
  check(SV("__0x0__"), SV("{:_^{}}"), input, 7);
  check(SV("####0x0"), SV("{:#>{}}"), input, 7);
#endif // !defined(__APPLE__) && !defined(__FreeBSD__)

  /***** Test the type generic part *****/
  check_exception("The format string contains an invalid escape sequence", SV("{:}<}"), input);
  check_exception("The fill option contains an invalid value", SV("{:{<}"), input);

  // *** sign ***
  check_exception("The replacement field misses a terminating '}'", SV("{:-}"), input);
  check_exception("The replacement field misses a terminating '}'", SV("{:+}"), input);
  check_exception("The replacement field misses a terminating '}'", SV("{: }"), input);

  // *** alternate form ***
  check_exception("The replacement field misses a terminating '}'", SV("{:#}"), input);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("{:0}"), input);

  // *** precision ***
  check_exception("The replacement field misses a terminating '}'", SV("{:.}"), input);

  // *** locale-specific form ***
  check_exception("The replacement field misses a terminating '}'", SV("{:L}"), input);

  // *** type ***
  for (std::basic_string_view<CharT> fmt : fmt_invalid_types<CharT>(""))
    check_exception("The replacement field misses a terminating '}'", fmt, input);
}

#endif // TEST_STD_THREAD_THREAD_THREADS_THREAD_THREAD_CLASS_THREAD_THREAD_ID_FORMAT_FUNCTIONS_TESTS_H
