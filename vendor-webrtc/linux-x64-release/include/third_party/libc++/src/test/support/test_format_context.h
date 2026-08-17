// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_TEST_FORMAT_CONTEXT_HPP
#define SUPPORT_TEST_FORMAT_CONTEXT_HPP

/**
 * @file Helper functions to create a @ref std::basic_format_context.
 *
 * Since the standard doesn't specify how a @ref std::basic_format_context is
 * constructed this is implementation defined. To make the public API tests of
 * the class generic this header defines helper functions to create the
 * required object.
 *
 * @note This requires every standard library implementation to write their own
 * helper function. Vendors are encouraged to file a PR at
 * https://github.com/llvm/llvm-project so their specific implementation can be
 * part of this file.
 */

#include "test_macros.h"

#if TEST_STD_VER < 20
#error "The format header requires at least C++20"
#endif

#include <format>

#ifndef TEST_HAS_NO_LOCALIZATION
#  include <locale>
#endif

#ifdef _LIBCPP_VERSION

/** Creates a std::basic_format_context as-if the formatting function takes no locale. */
template <class OutIt, class CharT>
std::basic_format_context<OutIt, CharT> test_format_context_create(
    OutIt out_it,
    std::basic_format_args<std::basic_format_context<OutIt, CharT>> args) {
  return std::__format_context_create(std::move(out_it), args);
}

#  ifndef TEST_HAS_NO_LOCALIZATION
/** Creates a std::basic_format_context as-if the formatting function takes locale. */
template <class OutIt, class CharT>
std::basic_format_context<OutIt, CharT> test_format_context_create(
    OutIt out_it,
    std::basic_format_args<std::basic_format_context<OutIt, CharT>> args,
    std::locale loc) {
  return std::__format_context_create(std::move(out_it), args, std::move(loc));
}
#  endif // TEST_HAS_NO_LOCALIZATION
#else    // _LIBCPP_VERSION
#  error                                                                                                               \
      "Please create a vendor specific version of the test typedef and file a PR at https://github.com/llvm/llvm-project"
#endif // _LIBCPP_VERSION

#endif // SUPPORT_TEST_FORMAT_CONTEXT_HPP
