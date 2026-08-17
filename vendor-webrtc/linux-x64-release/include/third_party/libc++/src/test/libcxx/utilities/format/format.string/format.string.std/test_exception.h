//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP_TEST_STD_UTILITIES_FORMAT_FORMAT_STRING_FORMAT_STRING_STD_TEST_EXCEPTION_H
#define _LIBCPP_TEST_STD_UTILITIES_FORMAT_FORMAT_STRING_FORMAT_STRING_STD_TEST_EXCEPTION_H

#include <format>
#include <string_view>

namespace detail {
#ifndef TEST_HAS_NO_EXCEPTIONS
template <class Parser, class CharT>
void test_exception(std::string_view what, const CharT* fmt) {
  try {
    std::basic_format_parse_context<CharT> parse_ctx(fmt);
    (void)Parser{}.parse(parse_ctx);
    assert(false);
  } catch (std::format_error& e) {
    LIBCPP_ASSERT(e.what() == what);
    return;
  }

  assert(false);
}
#endif
} // namespace detail

/**
 *  Wrapper for the exception tests.
 *
 *  When using the real function directly during in a constexpr test and add
 *  the `std::is_constant_evaluated()` test there the compilation fails. This
 *  happens since assert calls the non-constexpr function '__assert_fail'.
 *  Solve this issue with an layer of indirection.
 */
template <class Parser, class CharT>
constexpr void test_exception(std::string_view what, const CharT* fmt) {
#ifndef TEST_HAS_NO_EXCEPTIONS
  if (!std::is_constant_evaluated())
    detail::test_exception<Parser>(what, fmt);
#else
  (void)what;
  (void)fmt;
#endif
}

#endif // _LIBCPP_TEST_STD_UTILITIES_FORMAT_FORMAT_STRING_FORMAT_STRING_STD_TEST_EXCEPTION_H
