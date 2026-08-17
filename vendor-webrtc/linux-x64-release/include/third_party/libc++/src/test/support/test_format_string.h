// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_TEST_FORMAT_STRING_HPP
#define SUPPORT_TEST_FORMAT_STRING_HPP

#include <concepts>
#include <format>
#include <type_traits>

#include "test_macros.h"

#if TEST_STD_VER < 20
#  error "The format header requires at least C++20"
#endif

// Wrapper for basic_format_string.
//
// This layer of indirection is used since it's not possible to use
// std::basic_format_string<CharT, Args...> in the test function directly.
//
// In C++20 the basic-format-string was an exposition only type. In C++23 is
// has been replaced with basic_format_string. Both libc++ and MSVC STL offer
// it as an extension in C++20.
#if TEST_STD_VER > 20 || defined(_LIBCPP_VERSION) || defined(_MSVC_STL_VERSION)
#  ifndef TEST_HAS_NO_WIDE_CHARACTERS
template <class CharT, class... Args>
using test_format_string =
    std::conditional_t<std::same_as<CharT, char>, std::format_string<Args...>, std::wformat_string<Args...>>;
#  else
template <class CharT, class... Args>
using test_format_string = std::format_string<Args...>;
#  endif

#else // TEST_STD_VER > 20 || defined(_LIBCPP_VERSION) || defined( _MSVC_STL_VERSION)

#  error                                                                                                               \
      "Please create a vendor specific version of the test typedef and file a PR at https://github.com/llvm/llvm-project"

#endif // TEST_STD_VER > 20 || defined(_LIBCPP_VERSION) || defined( _MSVC_STL_VERSION)

#endif // SUPPORT_TEST_FORMAT_STRING_HPP
