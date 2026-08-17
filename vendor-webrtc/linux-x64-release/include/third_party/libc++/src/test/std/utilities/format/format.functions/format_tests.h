//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_UTILITIES_FORMAT_FORMAT_FUNCTIONS_FORMAT_TESTS_H
#define TEST_STD_UTILITIES_FORMAT_FORMAT_FUNCTIONS_FORMAT_TESTS_H

#include <format>

#include <algorithm>
#include <cassert>
#include <charconv>
#include <cmath>
#include <cstdint>
#include <iterator>

#include "string_literal.h"
#include "test_macros.h"
#include "format.functions.common.h"

// In this file the following template types are used:
// TestFunction must be callable as check(expected-result, string-to-format, args-to-format...)
// ExceptionTest must be callable as check_exception(expected-exception, string-to-format, args-to-format...)

enum class execution_modus { partial, full };

template <class CharT>
std::vector<std::basic_string_view<CharT>> invalid_types(std::string_view valid) {
  std::vector<std::basic_string_view<CharT>> result;

#define CASE(T)                                                                                                        \
case #T[0]:                                                                                                            \
  result.push_back(SV("Invalid formatter type {:" #T "}"));                                                            \
  break;

#if TEST_STD_VER > 20
  constexpr std::string_view types = "aAbBcdeEfFgGopPsxX?";
#else
  constexpr std::string_view types = "aAbBcdeEfFgGopPsxX";
#endif

  for (auto type : types) {
    if (valid.find(type) != std::string::npos)
      continue;

    switch (type) {
      CASE(a)
      CASE(A)
      CASE(b)
      CASE(B)
      CASE(c)
      CASE(d)
      CASE(e)
      CASE(E)
      CASE(f)
      CASE(F)
      CASE(g)
      CASE(G)
      CASE(o)
      CASE(p)
      CASE(P)
      CASE(s)
      CASE(x)
      CASE(X)
      CASE(?)
    case 0:
      break;
    default:
      assert(false && "Add the type to the list of cases.");
    }
  }
#undef CASE

  return result;
}

template <class CharT, class TestFunction>
void format_test_buffer_copy(TestFunction check) {
  // *** copy ***
  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(
      SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
      SV("{}"),
      SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  // *** copy + push_back ***

  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "X"),
        SV("{}X"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "X"),
        SV("{}X"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "X"),
        SV("{}X"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "X"),
        SV("{}X"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(
      SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "X"),
      SV("{}X"),
      SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  // ***  push_back + copy ***

  check(SV("X"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("X{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("X"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("X{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("X"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("X{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(SV("X"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
        SV("X{}"),
        SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
           "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));

  check(
      SV("X"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
      SV("X{}"),
      SV("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
         "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));
}

template <class CharT, class TestFunction>
void format_test_buffer_full(TestFunction check) {
  // *** fill ***
  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"), SV("{:|<64}"), SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("{:|<128}"),
        SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("{:|<256}"),
        SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("{:|<512}"),
        SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("{:|<1024}"),
        SV(""));

  // *** fill + push_back ***

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "X"),
        SV("{:|<64}X"),
        SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "X"),
        SV("{:|<128}X"),
        SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "X"),
        SV("{:|<256}X"),
        SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "X"),
        SV("{:|<512}X"),
        SV(""));

  check(SV("||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "X"),
        SV("{:|<1024}X"),
        SV(""));

  // *** push_back + fill ***

  check(SV("X"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("X{:|<64}"),
        SV(""));

  check(SV("X"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("X{:|<128}"),
        SV(""));

  check(SV("X"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("X{:|<256}"),
        SV(""));

  check(SV("X"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("X{:|<512}"),
        SV(""));

  check(SV("X"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"
           "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||"),
        SV("X{:|<1024}"),
        SV(""));
}

// Using a const ref for world and universe so a string literal will be a character array.
// When passed as character array W and U have different types.
template <class CharT, class W, class U, class TestFunction, class ExceptionTest>
void format_test_string(const W& world, const U& universe, TestFunction check, ExceptionTest check_exception) {

  // *** Valid input tests ***
  // Unused argument is ignored. TODO FMT what does the Standard mandate?
  check(SV("hello world"), SV("hello {}"), world, universe);
  check(SV("hello world and universe"), SV("hello {} and {}"), world, universe);
  check(SV("hello world"), SV("hello {0}"), world, universe);
  check(SV("hello universe"), SV("hello {1}"), world, universe);
  check(SV("hello universe and world"), SV("hello {1} and {0}"), world, universe);

  check(SV("hello world"), SV("hello {:_>}"), world);
  check(SV("hello world   "), SV("hello {:8}"), world);
  check(SV("hello    world"), SV("hello {:>8}"), world);
  check(SV("hello ___world"), SV("hello {:_>8}"), world);
  check(SV("hello _world__"), SV("hello {:_^8}"), world);
  check(SV("hello world___"), SV("hello {:_<8}"), world);

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("hello :::world"), SV("hello {::>8}"), world);
  check(SV("hello <<<world"), SV("hello {:<>8}"), world);
  check(SV("hello ^^^world"), SV("hello {:^>8}"), world);

  check(SV("hello $world"), SV("hello {:$>{}}"), world, 6);
  check(SV("hello $world"), SV("hello {0:$>{1}}"), world, 6);
  check(SV("hello $world"), SV("hello {1:$>{0}}"), 6, world);

  check(SV("hello world"), SV("hello {:.5}"), world);
  check(SV("hello unive"), SV("hello {:.5}"), universe);

  check(SV("hello univer"), SV("hello {:.{}}"), universe, 6);
  check(SV("hello univer"), SV("hello {0:.{1}}"), universe, 6);
  check(SV("hello univer"), SV("hello {1:.{0}}"), 6, universe);

  check(SV("hello %world%"), SV("hello {:%^7.7}"), world);
  check(SV("hello univers"), SV("hello {:%^7.7}"), universe);
  check(SV("hello %world%"), SV("hello {:%^{}.{}}"), world, 7, 7);
  check(SV("hello %world%"), SV("hello {0:%^{1}.{2}}"), world, 7, 7);
  check(SV("hello %world%"), SV("hello {0:%^{2}.{1}}"), world, 7, 7);
  check(SV("hello %world%"), SV("hello {1:%^{0}.{2}}"), 7, world, 7);

  check(SV("hello world"), SV("hello {:_>s}"), world);
  check(SV("hello $world"), SV("hello {:$>{}s}"), world, 6);
  check(SV("hello world"), SV("hello {:.5s}"), world);
  check(SV("hello univer"), SV("hello {:.{}s}"), universe, 6);
  check(SV("hello %world%"), SV("hello {:%^7.7s}"), world);

  check(SV("hello #####uni"), SV("hello {:#>8.3s}"), universe);
  check(SV("hello ##uni###"), SV("hello {:#^8.3s}"), universe);
  check(SV("hello uni#####"), SV("hello {:#<8.3s}"), universe);

  // *** sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("hello {:-}"), world);

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("hello {:#}"), world);

  // *** zero-padding ***
  check_exception("The width option should not have a leading zero", SV("hello {:0}"), world);

  // *** width ***
  // Width 0 allowed, but not useful for string arguments.
  check(SV("hello world"), SV("hello {:{}}"), world, 0);

#ifdef _LIBCPP_VERSION
  // This limit isn't specified in the Standard.
  static_assert(std::__format::__number_max == 2'147'483'647, "Update the assert and the test.");
  check_exception("The numeric value of the format specifier is too large", SV("{:2147483648}"), world);
  check_exception("The numeric value of the format specifier is too large", SV("{:5000000000}"), world);
  check_exception("The numeric value of the format specifier is too large", SV("{:10000000000}"), world);
#endif

  check_exception("An argument index may not have a negative value", SV("hello {:{}}"), world, -1);
  check_exception("The value of the argument index exceeds its maximum value", SV("hello {:{}}"), world, unsigned(-1));
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("hello {:{}}"), world);
  check_exception(
      "Replacement argument isn't a standard signed or unsigned integer type", SV("hello {:{}}"), world, universe);
  check_exception("Using manual argument numbering in automatic argument numbering mode", SV("hello {:{0}}"), world, 1);
  check_exception("Using automatic argument numbering in manual argument numbering mode", SV("hello {0:{}}"), world, 1);
  // Arg-id may not have leading zeros.
  check_exception("The argument index is invalid", SV("hello {0:{01}}"), world, 1);

  // *** precision ***
#ifdef _LIBCPP_VERSION
  // This limit isn't specified in the Standard.
  static_assert(std::__format::__number_max == 2'147'483'647, "Update the assert and the test.");
  check_exception("The numeric value of the format specifier is too large", SV("{:.2147483648}"), world);
  check_exception("The numeric value of the format specifier is too large", SV("{:.5000000000}"), world);
  check_exception("The numeric value of the format specifier is too large", SV("{:.10000000000}"), world);
#endif

  // Precision 0 allowed, but not useful for string arguments.
  check(SV("hello "), SV("hello {:.{}}"), world, 0);
  // Precision may have leading zeros. Secondly tests the value is still base 10.
  check(SV("hello 0123456789"), SV("hello {:.000010}"), STR("0123456789abcdef"));
  check_exception("An argument index may not have a negative value", SV("hello {:.{}}"), world, -1);
  check_exception("The value of the argument index exceeds its maximum value", SV("hello {:.{}}"), world, ~0u);
  check_exception(
      "The argument index value is too large for the number of arguments supplied", SV("hello {:.{}}"), world);
  check_exception(
      "Replacement argument isn't a standard signed or unsigned integer type", SV("hello {:.{}}"), world, universe);
  check_exception("Using manual argument numbering in automatic argument numbering mode", SV("hello {:.{0}}"), world,
                  1);
  check_exception("Using automatic argument numbering in manual argument numbering mode", SV("hello {0:.{}}"), world,
                  1);
  // Arg-id may not have leading zeros.
  check_exception("The argument index is invalid", SV("hello {0:.{01}}"), world, 1);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("hello {:L}"), world);

  // *** type ***
#if TEST_STD_VER > 20
  const char* valid_types = "s?";
#else
  const char* valid_types = "s";
#endif
  for (const auto& fmt : invalid_types<CharT>(valid_types))
    check_exception("The type option contains an invalid value for a string formatting argument", fmt, world);
}

template <class CharT, class TestFunction>
void format_test_string_unicode([[maybe_unused]] TestFunction check) {
  // unicode.pass.cpp and ascii.pass.cpp have additional tests.
#ifndef TEST_HAS_NO_UNICODE
  // Make sure all possible types are tested. For clarity don't use macros.
  if constexpr (std::same_as<CharT, char>) {
    const char* c_string = "aßc";
    check(SV("*aßc*"), SV("{:*^5}"), c_string);
    check(SV("*aß*"), SV("{:*^4.2}"), c_string);

    check(SV("*aßc*"), SV("{:*^5}"), const_cast<char*>(c_string));
    check(SV("*aß*"), SV("{:*^4.2}"), const_cast<char*>(c_string));

    check(SV("*aßc*"), SV("{:*^5}"), "aßc");
    check(SV("*aß*"), SV("{:*^4.2}"), "aßc");

    check(SV("*aßc*"), SV("{:*^5}"), std::string("aßc"));
    check(SV("*aß*"), SV("{:*^4.2}"), std::string("aßc"));

    check(SV("*aßc*"), SV("{:*^5}"), std::string_view("aßc"));
    check(SV("*aß*"), SV("{:*^4.2}"), std::string_view("aßc"));
  }
#  ifndef TEST_HAS_NO_WIDE_CHARACTERS
  else {
    const wchar_t* c_string = L"aßc";
    check(SV("*aßc*"), SV("{:*^5}"), c_string);
    check(SV("*aß*"), SV("{:*^4.2}"), c_string);

    check(SV("*aßc*"), SV("{:*^5}"), const_cast<wchar_t*>(c_string));
    check(SV("*aß*"), SV("{:*^4.2}"), const_cast<wchar_t*>(c_string));

    check(SV("*aßc*"), SV("{:*^5}"), L"aßc");
    check(SV("*aß*"), SV("{:*^4.2}"), L"aßc");

    check(SV("*aßc*"), SV("{:*^5}"), std::wstring(L"aßc"));
    check(SV("*aß*"), SV("{:*^4.2}"), std::wstring(L"aßc"));

    check(SV("*aßc*"), SV("{:*^5}"), std::wstring_view(L"aßc"));
    check(SV("*aß*"), SV("{:*^4.2}"), std::wstring_view(L"aßc"));
  }
#  endif // TEST_HAS_NO_WIDE_CHARACTERS

  // ß requires one column
  check(SV("aßc"), SV("{}"), STR("aßc"));

  check(SV("aßc"), SV("{:.3}"), STR("aßc"));
  check(SV("aß"), SV("{:.2}"), STR("aßc"));
  check(SV("a"), SV("{:.1}"), STR("aßc"));

  check(SV("aßc"), SV("{:3.3}"), STR("aßc"));
  check(SV("aß"), SV("{:2.2}"), STR("aßc"));
  check(SV("a"), SV("{:1.1}"), STR("aßc"));

  check(SV("aßc---"), SV("{:-<6}"), STR("aßc"));
  check(SV("-aßc--"), SV("{:-^6}"), STR("aßc"));
  check(SV("---aßc"), SV("{:->6}"), STR("aßc"));

  // \u1000 requires two columns
  check(SV("a\u1110c"), SV("{}"), STR("a\u1110c"));

  check(SV("a\u1100c"), SV("{:.4}"), STR("a\u1100c"));
  check(SV("a\u1100"), SV("{:.3}"), STR("a\u1100c"));
  check(SV("a"), SV("{:.2}"), STR("a\u1100c"));
  check(SV("a"), SV("{:.1}"), STR("a\u1100c"));

  check(SV("a\u1100c"), SV("{:-<4.4}"), STR("a\u1100c"));
  check(SV("a\u1100"), SV("{:-<3.3}"), STR("a\u1100c"));
  check(SV("a-"), SV("{:-<2.2}"), STR("a\u1100c"));
  check(SV("a"), SV("{:-<1.1}"), STR("a\u1100c"));

  check(SV("a\u1110c---"), SV("{:-<7}"), STR("a\u1110c"));
  check(SV("-a\u1110c--"), SV("{:-^7}"), STR("a\u1110c"));
  check(SV("---a\u1110c"), SV("{:->7}"), STR("a\u1110c"));

  // Examples used in P1868R2
  check(SV("*\u0041*"), SV("{:*^3}"), STR("\u0041")); // { LATIN CAPITAL LETTER A }
  check(SV("*\u00c1*"), SV("{:*^3}"), STR("\u00c1")); // { LATIN CAPITAL LETTER A WITH ACUTE }
  check(SV("*\u0041\u0301*"),
        SV("{:*^3}"),
        STR("\u0041\u0301"));                         // { LATIN CAPITAL LETTER A } { COMBINING ACUTE ACCENT }
  check(SV("*\u0132*"), SV("{:*^3}"), STR("\u0132")); // { LATIN CAPITAL LIGATURE IJ }
  check(SV("*\u0394*"), SV("{:*^3}"), STR("\u0394")); // { GREEK CAPITAL LETTER DELTA }

  check(SV("*\u0429*"), SV("{:*^3}"), STR("\u0429"));         // { CYRILLIC CAPITAL LETTER SHCHA }
  check(SV("*\u05d0*"), SV("{:*^3}"), STR("\u05d0"));         // { HEBREW LETTER ALEF }
  check(SV("*\u0634*"), SV("{:*^3}"), STR("\u0634"));         // { ARABIC LETTER SHEEN }
  check(SV("*\u3009*"), SV("{:*^4}"), STR("\u3009"));         // { RIGHT-POINTING ANGLE BRACKET }
  check(SV("*\u754c*"), SV("{:*^4}"), STR("\u754c"));         // { CJK Unified Ideograph-754C }
  check(SV("*\U0001f921*"), SV("{:*^4}"), STR("\U0001f921")); // { UNICORN FACE }
  check(SV("*\U0001f468\u200d\U0001F469\u200d\U0001F467\u200d\U0001F466*"),
        SV("{:*^4}"),
        STR("\U0001f468\u200d\U0001F469\u200d\U0001F467\u200d\U0001F466")); // { Family: Man, Woman, Girl, Boy }
#endif // TEST_HAS_NO_UNICODE
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_string_tests(TestFunction check, ExceptionTest check_exception) {
  std::basic_string<CharT> world = STR("world");
  std::basic_string<CharT> universe = STR("universe");

  // Test a string literal in a way it won't decay to a pointer.
  if constexpr (std::same_as<CharT, char>)
    format_test_string<CharT>("world", "universe", check, check_exception);
#ifndef TEST_HAS_NO_WIDE_CHARACTERS
  else
    format_test_string<CharT>(L"world", L"universe", check, check_exception);
#endif

  format_test_string<CharT>(world.c_str(), universe.c_str(), check, check_exception);
  format_test_string<CharT>(const_cast<CharT*>(world.c_str()), const_cast<CharT*>(universe.c_str()), check,
                            check_exception);
  format_test_string<CharT>(std::basic_string_view<CharT>(world), std::basic_string_view<CharT>(universe), check,
                            check_exception);
  format_test_string<CharT>(world, universe, check, check_exception);
  format_test_string_unicode<CharT>(check);
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_bool(TestFunction check, ExceptionTest check_exception) {

  // *** align-fill & width ***
  check(SV("answer is 'true   '"), SV("answer is '{:7}'"), true);
  check(SV("answer is '   true'"), SV("answer is '{:>7}'"), true);
  check(SV("answer is 'true   '"), SV("answer is '{:<7}'"), true);
  check(SV("answer is ' true  '"), SV("answer is '{:^7}'"), true);

  check(SV("answer is 'false   '"), SV("answer is '{:8s}'"), false);
  check(SV("answer is '   false'"), SV("answer is '{:>8s}'"), false);
  check(SV("answer is 'false   '"), SV("answer is '{:<8s}'"), false);
  check(SV("answer is ' false  '"), SV("answer is '{:^8s}'"), false);

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::true'"), SV("answer is '{::>7}'"), true);
  check(SV("answer is 'true:::'"), SV("answer is '{::<7}'"), true);
  check(SV("answer is ':true::'"), SV("answer is '{::^7}'"), true);

  check(SV("answer is '---false'"), SV("answer is '{:->8s}'"), false);
  check(SV("answer is 'false---'"), SV("answer is '{:-<8s}'"), false);
  check(SV("answer is '-false--'"), SV("answer is '{:-^8s}'"), false);

  // *** Sign ***
  check_exception("The format specifier for a bool does not allow the sign option", SV("{:-}"), true);
  check_exception("The format specifier for a bool does not allow the sign option", SV("{:+}"), true);
  check_exception("The format specifier for a bool does not allow the sign option", SV("{: }"), true);

  check_exception("The format specifier for a bool does not allow the sign option", SV("{:-s}"), true);
  check_exception("The format specifier for a bool does not allow the sign option", SV("{:+s}"), true);
  check_exception("The format specifier for a bool does not allow the sign option", SV("{: s}"), true);

  // *** alternate form ***
  check_exception("The format specifier for a bool does not allow the alternate form option", SV("{:#}"), true);
  check_exception("The format specifier for a bool does not allow the alternate form option", SV("{:#s}"), true);

  // *** zero-padding ***
  check_exception("The format specifier for a bool does not allow the zero-padding option", SV("{:0}"), true);
  check_exception("The format specifier for a bool does not allow the zero-padding option", SV("{:0s}"), true);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0}"), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42}"), true);

  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.s}"), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0s}"), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42s}"), true);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp

  // *** type ***
  for (const auto& fmt : invalid_types<CharT>("bBdosxX"))
    check_exception("The type option contains an invalid value for a bool formatting argument", fmt, true);
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_bool_as_integer(TestFunction check, ExceptionTest check_exception) {
  // *** align-fill & width ***
  check(SV("answer is '1'"), SV("answer is '{:<1d}'"), true);
  check(SV("answer is '1 '"), SV("answer is '{:<2d}'"), true);
  check(SV("answer is '0 '"), SV("answer is '{:<2d}'"), false);

  check(SV("answer is '     1'"), SV("answer is '{:6d}'"), true);
  check(SV("answer is '     1'"), SV("answer is '{:>6d}'"), true);
  check(SV("answer is '1     '"), SV("answer is '{:<6d}'"), true);
  check(SV("answer is '  1   '"), SV("answer is '{:^6d}'"), true);

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::::0'"), SV("answer is '{::>6d}'"), false);
  check(SV("answer is '0:::::'"), SV("answer is '{::<6d}'"), false);
  check(SV("answer is '::0:::'"), SV("answer is '{::^6d}'"), false);

  // Test whether zero padding is ignored
  check(SV("answer is '     1'"), SV("answer is '{:>06d}'"), true);
  check(SV("answer is '1     '"), SV("answer is '{:<06d}'"), true);
  check(SV("answer is '  1   '"), SV("answer is '{:^06d}'"), true);

  // *** Sign ***
  check(SV("answer is 1"), SV("answer is {:d}"), true);
  check(SV("answer is 0"), SV("answer is {:-d}"), false);
  check(SV("answer is +1"), SV("answer is {:+d}"), true);
  check(SV("answer is  0"), SV("answer is {: d}"), false);

  // *** alternate form ***
  check(SV("answer is +1"), SV("answer is {:+#d}"), true);
  check(SV("answer is +1"), SV("answer is {:+b}"), true);
  check(SV("answer is +0b1"), SV("answer is {:+#b}"), true);
  check(SV("answer is +0B1"), SV("answer is {:+#B}"), true);
  check(SV("answer is +1"), SV("answer is {:+o}"), true);
  check(SV("answer is +01"), SV("answer is {:+#o}"), true);
  check(SV("answer is +1"), SV("answer is {:+x}"), true);
  check(SV("answer is +0x1"), SV("answer is {:+#x}"), true);
  check(SV("answer is +1"), SV("answer is {:+X}"), true);
  check(SV("answer is +0X1"), SV("answer is {:+#X}"), true);

  check(SV("answer is 0"), SV("answer is {:#d}"), false);
  check(SV("answer is 0"), SV("answer is {:b}"), false);
  check(SV("answer is 0b0"), SV("answer is {:#b}"), false);
  check(SV("answer is 0B0"), SV("answer is {:#B}"), false);
  check(SV("answer is 0"), SV("answer is {:o}"), false);
  check(SV("answer is 0"), SV("answer is {:#o}"), false);
  check(SV("answer is 0"), SV("answer is {:x}"), false);
  check(SV("answer is 0x0"), SV("answer is {:#x}"), false);
  check(SV("answer is 0"), SV("answer is {:X}"), false);
  check(SV("answer is 0X0"), SV("answer is {:#X}"), false);

  // *** zero-padding & width ***
  check(SV("answer is +00000000001"), SV("answer is {:+#012d}"), true);
  check(SV("answer is +00000000001"), SV("answer is {:+012b}"), true);
  check(SV("answer is +0b000000001"), SV("answer is {:+#012b}"), true);
  check(SV("answer is +0B000000001"), SV("answer is {:+#012B}"), true);
  check(SV("answer is +00000000001"), SV("answer is {:+012o}"), true);
  check(SV("answer is +00000000001"), SV("answer is {:+#012o}"), true);
  check(SV("answer is +00000000001"), SV("answer is {:+012x}"), true);
  check(SV("answer is +0x000000001"), SV("answer is {:+#012x}"), true);
  check(SV("answer is +00000000001"), SV("answer is {:+012X}"), true);
  check(SV("answer is +0X000000001"), SV("answer is {:+#012X}"), true);

  check(SV("answer is 000000000000"), SV("answer is {:#012d}"), false);
  check(SV("answer is 000000000000"), SV("answer is {:012b}"), false);
  check(SV("answer is 0b0000000000"), SV("answer is {:#012b}"), false);
  check(SV("answer is 0B0000000000"), SV("answer is {:#012B}"), false);
  check(SV("answer is 000000000000"), SV("answer is {:012o}"), false);
  check(SV("answer is 000000000000"), SV("answer is {:#012o}"), false);
  check(SV("answer is 000000000000"), SV("answer is {:012x}"), false);
  check(SV("answer is 0x0000000000"), SV("answer is {:#012x}"), false);
  check(SV("answer is 000000000000"), SV("answer is {:012X}"), false);
  check(SV("answer is 0X0000000000"), SV("answer is {:#012X}"), false);

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0}"), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42}"), true);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp

  // *** type ***
  for (const auto& fmt : invalid_types<CharT>("bBcdosxX"))
    check_exception("The type option contains an invalid value for a bool formatting argument", fmt, true);
}

template <class I, class CharT, class TestFunction, class ExceptionTest>
void format_test_integer_as_integer(TestFunction check, ExceptionTest check_exception) {
  // *** align-fill & width ***
  check(SV("answer is '42'"), SV("answer is '{:<1}'"), I(42));
  check(SV("answer is '42'"), SV("answer is '{:<2}'"), I(42));
  check(SV("answer is '42 '"), SV("answer is '{:<3}'"), I(42));

  check(SV("answer is '     42'"), SV("answer is '{:7}'"), I(42));
  check(SV("answer is '     42'"), SV("answer is '{:>7}'"), I(42));
  check(SV("answer is '42     '"), SV("answer is '{:<7}'"), I(42));
  check(SV("answer is '  42   '"), SV("answer is '{:^7}'"), I(42));

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::::42'"), SV("answer is '{::>7}'"), I(42));
  check(SV("answer is '42:::::'"), SV("answer is '{::<7}'"), I(42));
  check(SV("answer is '::42:::'"), SV("answer is '{::^7}'"), I(42));

  // Test whether zero padding is ignored
  check(SV("answer is '     42'"), SV("answer is '{:>07}'"), I(42));
  check(SV("answer is '42     '"), SV("answer is '{:<07}'"), I(42));
  check(SV("answer is '  42   '"), SV("answer is '{:^07}'"), I(42));

  // *** Sign ***
  if constexpr (std::signed_integral<I>)
    check(SV("answer is -42"), SV("answer is {}"), I(-42));
  check(SV("answer is 0"), SV("answer is {}"), I(0));
  check(SV("answer is 42"), SV("answer is {}"), I(42));

  if constexpr (std::signed_integral<I>)
    check(SV("answer is -42"), SV("answer is {:-}"), I(-42));
  check(SV("answer is 0"), SV("answer is {:-}"), I(0));
  check(SV("answer is 42"), SV("answer is {:-}"), I(42));

  if constexpr (std::signed_integral<I>)
    check(SV("answer is -42"), SV("answer is {:+}"), I(-42));
  check(SV("answer is +0"), SV("answer is {:+}"), I(0));
  check(SV("answer is +42"), SV("answer is {:+}"), I(42));

  if constexpr (std::signed_integral<I>)
    check(SV("answer is -42"), SV("answer is {: }"), I(-42));
  check(SV("answer is  0"), SV("answer is {: }"), I(0));
  check(SV("answer is  42"), SV("answer is {: }"), I(42));

  // *** alternate form ***
  if constexpr (std::signed_integral<I>) {
    check(SV("answer is -42"), SV("answer is {:#}"), I(-42));
    check(SV("answer is -42"), SV("answer is {:#d}"), I(-42));
    check(SV("answer is -101010"), SV("answer is {:b}"), I(-42));
    check(SV("answer is -0b101010"), SV("answer is {:#b}"), I(-42));
    check(SV("answer is -0B101010"), SV("answer is {:#B}"), I(-42));
    check(SV("answer is -52"), SV("answer is {:o}"), I(-42));
    check(SV("answer is -052"), SV("answer is {:#o}"), I(-42));
    check(SV("answer is -2a"), SV("answer is {:x}"), I(-42));
    check(SV("answer is -0x2a"), SV("answer is {:#x}"), I(-42));
    check(SV("answer is -2A"), SV("answer is {:X}"), I(-42));
    check(SV("answer is -0X2A"), SV("answer is {:#X}"), I(-42));
  }
  check(SV("answer is 0"), SV("answer is {:#}"), I(0));
  check(SV("answer is 0"), SV("answer is {:#d}"), I(0));
  check(SV("answer is 0"), SV("answer is {:b}"), I(0));
  check(SV("answer is 0b0"), SV("answer is {:#b}"), I(0));
  check(SV("answer is 0B0"), SV("answer is {:#B}"), I(0));
  check(SV("answer is 0"), SV("answer is {:o}"), I(0));
  check(SV("answer is 0"), SV("answer is {:#o}"), I(0));
  check(SV("answer is 0"), SV("answer is {:x}"), I(0));
  check(SV("answer is 0x0"), SV("answer is {:#x}"), I(0));
  check(SV("answer is 0"), SV("answer is {:X}"), I(0));
  check(SV("answer is 0X0"), SV("answer is {:#X}"), I(0));

  check(SV("answer is +42"), SV("answer is {:+#}"), I(42));
  check(SV("answer is +42"), SV("answer is {:+#d}"), I(42));
  check(SV("answer is +101010"), SV("answer is {:+b}"), I(42));
  check(SV("answer is +0b101010"), SV("answer is {:+#b}"), I(42));
  check(SV("answer is +0B101010"), SV("answer is {:+#B}"), I(42));
  check(SV("answer is +52"), SV("answer is {:+o}"), I(42));
  check(SV("answer is +052"), SV("answer is {:+#o}"), I(42));
  check(SV("answer is +2a"), SV("answer is {:+x}"), I(42));
  check(SV("answer is +0x2a"), SV("answer is {:+#x}"), I(42));
  check(SV("answer is +2A"), SV("answer is {:+X}"), I(42));
  check(SV("answer is +0X2A"), SV("answer is {:+#X}"), I(42));

  // *** zero-padding & width ***
  if constexpr (std::signed_integral<I>) {
    check(SV("answer is -00000000042"), SV("answer is {:#012}"), I(-42));
    check(SV("answer is -00000000042"), SV("answer is {:#012d}"), I(-42));
    check(SV("answer is -00000101010"), SV("answer is {:012b}"), I(-42));
    check(SV("answer is -0b000101010"), SV("answer is {:#012b}"), I(-42));
    check(SV("answer is -0B000101010"), SV("answer is {:#012B}"), I(-42));
    check(SV("answer is -00000000052"), SV("answer is {:012o}"), I(-42));
    check(SV("answer is -00000000052"), SV("answer is {:#012o}"), I(-42));
    check(SV("answer is -0000000002a"), SV("answer is {:012x}"), I(-42));
    check(SV("answer is -0x00000002a"), SV("answer is {:#012x}"), I(-42));
    check(SV("answer is -0000000002A"), SV("answer is {:012X}"), I(-42));
    check(SV("answer is -0X00000002A"), SV("answer is {:#012X}"), I(-42));
  }

  check(SV("answer is 000000000000"), SV("answer is {:#012}"), I(0));
  check(SV("answer is 000000000000"), SV("answer is {:#012d}"), I(0));
  check(SV("answer is 000000000000"), SV("answer is {:012b}"), I(0));
  check(SV("answer is 0b0000000000"), SV("answer is {:#012b}"), I(0));
  check(SV("answer is 0B0000000000"), SV("answer is {:#012B}"), I(0));
  check(SV("answer is 000000000000"), SV("answer is {:012o}"), I(0));
  check(SV("answer is 000000000000"), SV("answer is {:#012o}"), I(0));
  check(SV("answer is 000000000000"), SV("answer is {:012x}"), I(0));
  check(SV("answer is 0x0000000000"), SV("answer is {:#012x}"), I(0));
  check(SV("answer is 000000000000"), SV("answer is {:012X}"), I(0));
  check(SV("answer is 0X0000000000"), SV("answer is {:#012X}"), I(0));

  check(SV("answer is +00000000042"), SV("answer is {:+#012}"), I(42));
  check(SV("answer is +00000000042"), SV("answer is {:+#012d}"), I(42));
  check(SV("answer is +00000101010"), SV("answer is {:+012b}"), I(42));
  check(SV("answer is +0b000101010"), SV("answer is {:+#012b}"), I(42));
  check(SV("answer is +0B000101010"), SV("answer is {:+#012B}"), I(42));
  check(SV("answer is +00000000052"), SV("answer is {:+012o}"), I(42));
  check(SV("answer is +00000000052"), SV("answer is {:+#012o}"), I(42));
  check(SV("answer is +0000000002a"), SV("answer is {:+012x}"), I(42));
  check(SV("answer is +0x00000002a"), SV("answer is {:+#012x}"), I(42));
  check(SV("answer is +0000000002A"), SV("answer is {:+012X}"), I(42));
  check(SV("answer is +0X00000002A"), SV("answer is {:+#012X}"), I(42));

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), I(0));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0}"), I(0));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42}"), I(0));

  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}}"), I(0));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}}"), I(0), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}}"), I(0), 1.0);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp

  // *** type ***
  for (const auto& fmt : invalid_types<CharT>("bBcdoxX"))
    check_exception("The type option contains an invalid value for an integer formatting argument", fmt, I(0));
}

template <class I, class CharT, class TestFunction, class ExceptionTest>
void format_test_integer_as_char(TestFunction check, ExceptionTest check_exception) {
  // *** align-fill & width ***
  check(SV("answer is '*     '"), SV("answer is '{:6c}'"), I(42));
  check(SV("answer is '     *'"), SV("answer is '{:>6c}'"), I(42));
  check(SV("answer is '*     '"), SV("answer is '{:<6c}'"), I(42));
  check(SV("answer is '  *   '"), SV("answer is '{:^6c}'"), I(42));

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::::*'"), SV("answer is '{::>6c}'"), I(42));
  check(SV("answer is '*:::::'"), SV("answer is '{::<6c}'"), I(42));
  check(SV("answer is '::*:::'"), SV("answer is '{::^6c}'"), I(42));

  // *** Sign ***
  check(SV("answer is *"), SV("answer is {:c}"), I(42));
  check_exception("The format specifier for an integer does not allow the sign option", SV("answer is {:-c}"), I(42));
  check_exception("The format specifier for an integer does not allow the sign option", SV("answer is {:+c}"), I(42));
  check_exception("The format specifier for an integer does not allow the sign option", SV("answer is {: c}"), I(42));

  // *** alternate form ***
  check_exception(
      "The format specifier for an integer does not allow the alternate form option", SV("answer is {:#c}"), I(42));

  // *** zero-padding & width ***
  check_exception(
      "The format specifier for an integer does not allow the zero-padding option", SV("answer is {:01c}"), I(42));

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.c}"), I(0));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0c}"), I(0));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42c}"), I(0));

  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}c}"), I(0));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}c}"), I(0), true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}c}"), I(0), 1.0);

  // *** locale-specific form ***
  // Note it has no effect but it's allowed.
  check(SV("answer is '*'"), SV("answer is '{:Lc}'"), I(42));

  // *** type ***
  for (const auto& fmt : invalid_types<CharT>("bBcdoxX"))
    check_exception("The type option contains an invalid value for an integer formatting argument", fmt, I(42));

  // *** Validate range ***
  // The code has some duplications to keep the if statement readable.
  if constexpr (std::signed_integral<CharT>) {
    if constexpr (std::signed_integral<I> && sizeof(I) > sizeof(CharT)) {
      check_exception("Integral value outside the range of the char type", SV("{:c}"), std::numeric_limits<I>::min());
      check_exception("Integral value outside the range of the char type", SV("{:c}"), std::numeric_limits<I>::max());
    } else if constexpr (std::unsigned_integral<I> && sizeof(I) >= sizeof(CharT)) {
      check_exception("Integral value outside the range of the char type", SV("{:c}"), std::numeric_limits<I>::max());
    }
  } else if constexpr (sizeof(I) > sizeof(CharT)) {
    check_exception("Integral value outside the range of the char type", SV("{:c}"), std::numeric_limits<I>::max());
  }
}

template <class I, class CharT, class TestFunction, class ExceptionTest>
void format_test_integer(TestFunction check, ExceptionTest check_exception) {
  format_test_integer_as_integer<I, CharT>(check, check_exception);
  format_test_integer_as_char<I, CharT>(check, check_exception);
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_signed_integer(TestFunction check, ExceptionTest check_exception) {
  format_test_integer<signed char, CharT>(check, check_exception);
  format_test_integer<short, CharT>(check, check_exception);
  format_test_integer<int, CharT>(check, check_exception);
  format_test_integer<long, CharT>(check, check_exception);
  format_test_integer<long long, CharT>(check, check_exception);
#ifndef TEST_HAS_NO_INT128
  format_test_integer<__int128_t, CharT>(check, check_exception);
#endif
  // *** check the minima and maxima ***
  check(SV("-0b10000000"), SV("{:#b}"), std::numeric_limits<std::int8_t>::min());
  check(SV("-0200"), SV("{:#o}"), std::numeric_limits<std::int8_t>::min());
  check(SV("-128"), SV("{:#}"), std::numeric_limits<std::int8_t>::min());
  check(SV("-0x80"), SV("{:#x}"), std::numeric_limits<std::int8_t>::min());

  check(SV("-0b1000000000000000"), SV("{:#b}"), std::numeric_limits<std::int16_t>::min());
  check(SV("-0100000"), SV("{:#o}"), std::numeric_limits<std::int16_t>::min());
  check(SV("-32768"), SV("{:#}"), std::numeric_limits<std::int16_t>::min());
  check(SV("-0x8000"), SV("{:#x}"), std::numeric_limits<std::int16_t>::min());

  check(SV("-0b10000000000000000000000000000000"), SV("{:#b}"), std::numeric_limits<std::int32_t>::min());
  check(SV("-020000000000"), SV("{:#o}"), std::numeric_limits<std::int32_t>::min());
  check(SV("-2147483648"), SV("{:#}"), std::numeric_limits<std::int32_t>::min());
  check(SV("-0x80000000"), SV("{:#x}"), std::numeric_limits<std::int32_t>::min());

  check(SV("-0b1000000000000000000000000000000000000000000000000000000000000000"),
        SV("{:#b}"),
        std::numeric_limits<std::int64_t>::min());
  check(SV("-01000000000000000000000"), SV("{:#o}"), std::numeric_limits<std::int64_t>::min());
  check(SV("-9223372036854775808"), SV("{:#}"), std::numeric_limits<std::int64_t>::min());
  check(SV("-0x8000000000000000"), SV("{:#x}"), std::numeric_limits<std::int64_t>::min());

#ifndef TEST_HAS_NO_INT128
  check(SV("-0b1000000000000000000000000000000000000000000000000000000000000000"
           "0000000000000000000000000000000000000000000000000000000000000000"),
        SV("{:#b}"),
        std::numeric_limits<__int128_t>::min());
  check(SV("-02000000000000000000000000000000000000000000"), SV("{:#o}"), std::numeric_limits<__int128_t>::min());
  check(SV("-170141183460469231731687303715884105728"), SV("{:#}"), std::numeric_limits<__int128_t>::min());
  check(SV("-0x80000000000000000000000000000000"), SV("{:#x}"), std::numeric_limits<__int128_t>::min());
#endif

  check(SV("0b1111111"), SV("{:#b}"), std::numeric_limits<std::int8_t>::max());
  check(SV("0177"), SV("{:#o}"), std::numeric_limits<std::int8_t>::max());
  check(SV("127"), SV("{:#}"), std::numeric_limits<std::int8_t>::max());
  check(SV("0x7f"), SV("{:#x}"), std::numeric_limits<std::int8_t>::max());

  check(SV("0b111111111111111"), SV("{:#b}"), std::numeric_limits<std::int16_t>::max());
  check(SV("077777"), SV("{:#o}"), std::numeric_limits<std::int16_t>::max());
  check(SV("32767"), SV("{:#}"), std::numeric_limits<std::int16_t>::max());
  check(SV("0x7fff"), SV("{:#x}"), std::numeric_limits<std::int16_t>::max());

  check(SV("0b1111111111111111111111111111111"), SV("{:#b}"), std::numeric_limits<std::int32_t>::max());
  check(SV("017777777777"), SV("{:#o}"), std::numeric_limits<std::int32_t>::max());
  check(SV("2147483647"), SV("{:#}"), std::numeric_limits<std::int32_t>::max());
  check(SV("0x7fffffff"), SV("{:#x}"), std::numeric_limits<std::int32_t>::max());

  check(SV("0b111111111111111111111111111111111111111111111111111111111111111"),
        SV("{:#b}"),
        std::numeric_limits<std::int64_t>::max());
  check(SV("0777777777777777777777"), SV("{:#o}"), std::numeric_limits<std::int64_t>::max());
  check(SV("9223372036854775807"), SV("{:#}"), std::numeric_limits<std::int64_t>::max());
  check(SV("0x7fffffffffffffff"), SV("{:#x}"), std::numeric_limits<std::int64_t>::max());

#ifndef TEST_HAS_NO_INT128
  check(SV("0b111111111111111111111111111111111111111111111111111111111111111"
           "1111111111111111111111111111111111111111111111111111111111111111"),
        SV("{:#b}"),
        std::numeric_limits<__int128_t>::max());
  check(SV("01777777777777777777777777777777777777777777"), SV("{:#o}"), std::numeric_limits<__int128_t>::max());
  check(SV("170141183460469231731687303715884105727"), SV("{:#}"), std::numeric_limits<__int128_t>::max());
  check(SV("0x7fffffffffffffffffffffffffffffff"), SV("{:#x}"), std::numeric_limits<__int128_t>::max());
#endif
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_unsigned_integer(TestFunction check, ExceptionTest check_exception) {
  format_test_integer<unsigned char, CharT>(check, check_exception);
  format_test_integer<unsigned short, CharT>(check, check_exception);
  format_test_integer<unsigned, CharT>(check, check_exception);
  format_test_integer<unsigned long, CharT>(check, check_exception);
  format_test_integer<unsigned long long, CharT>(check, check_exception);
#ifndef TEST_HAS_NO_INT128
  format_test_integer<__uint128_t, CharT>(check, check_exception);
#endif
  // *** test the maxima ***
  check(SV("0b11111111"), SV("{:#b}"), std::numeric_limits<std::uint8_t>::max());
  check(SV("0377"), SV("{:#o}"), std::numeric_limits<std::uint8_t>::max());
  check(SV("255"), SV("{:#}"), std::numeric_limits<std::uint8_t>::max());
  check(SV("0xff"), SV("{:#x}"), std::numeric_limits<std::uint8_t>::max());

  check(SV("0b1111111111111111"), SV("{:#b}"), std::numeric_limits<std::uint16_t>::max());
  check(SV("0177777"), SV("{:#o}"), std::numeric_limits<std::uint16_t>::max());
  check(SV("65535"), SV("{:#}"), std::numeric_limits<std::uint16_t>::max());
  check(SV("0xffff"), SV("{:#x}"), std::numeric_limits<std::uint16_t>::max());

  check(SV("0b11111111111111111111111111111111"), SV("{:#b}"), std::numeric_limits<std::uint32_t>::max());
  check(SV("037777777777"), SV("{:#o}"), std::numeric_limits<std::uint32_t>::max());
  check(SV("4294967295"), SV("{:#}"), std::numeric_limits<std::uint32_t>::max());
  check(SV("0xffffffff"), SV("{:#x}"), std::numeric_limits<std::uint32_t>::max());

  check(SV("0b1111111111111111111111111111111111111111111111111111111111111111"),
        SV("{:#b}"),
        std::numeric_limits<std::uint64_t>::max());
  check(SV("01777777777777777777777"), SV("{:#o}"), std::numeric_limits<std::uint64_t>::max());
  check(SV("18446744073709551615"), SV("{:#}"), std::numeric_limits<std::uint64_t>::max());
  check(SV("0xffffffffffffffff"), SV("{:#x}"), std::numeric_limits<std::uint64_t>::max());

#ifndef TEST_HAS_NO_INT128
  check(SV("0b1111111111111111111111111111111111111111111111111111111111111111"
           "1111111111111111111111111111111111111111111111111111111111111111"),
        SV("{:#b}"),
        std::numeric_limits<__uint128_t>::max());
  check(SV("03777777777777777777777777777777777777777777"), SV("{:#o}"), std::numeric_limits<__uint128_t>::max());
  check(SV("340282366920938463463374607431768211455"), SV("{:#}"), std::numeric_limits<__uint128_t>::max());
  check(SV("0xffffffffffffffffffffffffffffffff"), SV("{:#x}"), std::numeric_limits<__uint128_t>::max());
#endif
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_char(TestFunction check, ExceptionTest check_exception) {

  // ***** Char type *****
  // *** align-fill & width ***
  check(SV("answer is '*     '"), SV("answer is '{:6}'"), CharT('*'));
  check(SV("answer is '     *'"), SV("answer is '{:>6}'"), CharT('*'));
  check(SV("answer is '*     '"), SV("answer is '{:<6}'"), CharT('*'));
  check(SV("answer is '  *   '"), SV("answer is '{:^6}'"), CharT('*'));

  check(SV("answer is '*     '"), SV("answer is '{:6c}'"), CharT('*'));
  check(SV("answer is '     *'"), SV("answer is '{:>6c}'"), CharT('*'));
  check(SV("answer is '*     '"), SV("answer is '{:<6c}'"), CharT('*'));
  check(SV("answer is '  *   '"), SV("answer is '{:^6c}'"), CharT('*'));

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::::*'"), SV("answer is '{::>6}'"), CharT('*'));
  check(SV("answer is '*:::::'"), SV("answer is '{::<6}'"), CharT('*'));
  check(SV("answer is '::*:::'"), SV("answer is '{::^6}'"), CharT('*'));

  check(SV("answer is '-----*'"), SV("answer is '{:->6c}'"), CharT('*'));
  check(SV("answer is '*-----'"), SV("answer is '{:-<6c}'"), CharT('*'));
  check(SV("answer is '--*---'"), SV("answer is '{:-^6c}'"), CharT('*'));

  // *** Sign ***
  check_exception("The format specifier for a character does not allow the sign option", SV("{:-}"), CharT('*'));
  check_exception("The format specifier for a character does not allow the sign option", SV("{:+}"), CharT('*'));
  check_exception("The format specifier for a character does not allow the sign option", SV("{: }"), CharT('*'));

  check_exception("The format specifier for a character does not allow the sign option", SV("{:-c}"), CharT('*'));
  check_exception("The format specifier for a character does not allow the sign option", SV("{:+c}"), CharT('*'));
  check_exception("The format specifier for a character does not allow the sign option", SV("{: c}"), CharT('*'));

  // *** alternate form ***
  check_exception(
      "The format specifier for a character does not allow the alternate form option", SV("{:#}"), CharT('*'));
  check_exception(
      "The format specifier for a character does not allow the alternate form option", SV("{:#c}"), CharT('*'));

  // *** zero-padding ***
  check_exception(
      "The format specifier for a character does not allow the zero-padding option", SV("{:0}"), CharT('*'));
  check_exception(
      "The format specifier for a character does not allow the zero-padding option", SV("{:0c}"), CharT('*'));

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), CharT('*'));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0}"), CharT('*'));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42}"), CharT('*'));

  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.c}"), CharT('*'));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0c}"), CharT('*'));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42c}"), CharT('*'));

  // *** locale-specific form ***
  // Note it has no effect but it's allowed.
  check(SV("answer is '*'"), SV("answer is '{:L}'"), '*');
  check(SV("answer is '*'"), SV("answer is '{:Lc}'"), '*');

  // *** type ***
#if TEST_STD_VER > 20
  const char* valid_types = "bBcdoxX?";
#else
  const char* valid_types = "bBcdoxX";
#endif
  for (const auto& fmt : invalid_types<CharT>(valid_types))
    check_exception("The type option contains an invalid value for a character formatting argument", fmt, CharT('*'));
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_char_as_integer(TestFunction check, ExceptionTest check_exception) {
  // *** align-fill & width ***
  check(SV("answer is '42'"), SV("answer is '{:<1d}'"), CharT('*'));

  check(SV("answer is '42'"), SV("answer is '{:<2d}'"), CharT('*'));
  check(SV("answer is '42 '"), SV("answer is '{:<3d}'"), CharT('*'));

  check(SV("answer is '     42'"), SV("answer is '{:7d}'"), CharT('*'));
  check(SV("answer is '     42'"), SV("answer is '{:>7d}'"), CharT('*'));
  check(SV("answer is '42     '"), SV("answer is '{:<7d}'"), CharT('*'));
  check(SV("answer is '  42   '"), SV("answer is '{:^7d}'"), CharT('*'));

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::::42'"), SV("answer is '{::>7d}'"), CharT('*'));
  check(SV("answer is '42:::::'"), SV("answer is '{::<7d}'"), CharT('*'));
  check(SV("answer is '::42:::'"), SV("answer is '{::^7d}'"), CharT('*'));

  // Test whether zero padding is ignored
  check(SV("answer is '     42'"), SV("answer is '{:>07d}'"), CharT('*'));
  check(SV("answer is '42     '"), SV("answer is '{:<07d}'"), CharT('*'));
  check(SV("answer is '  42   '"), SV("answer is '{:^07d}'"), CharT('*'));

  // *** Sign ***
  check(SV("answer is 42"), SV("answer is {:d}"), CharT('*'));
  check(SV("answer is 42"), SV("answer is {:-d}"), CharT('*'));
  check(SV("answer is +42"), SV("answer is {:+d}"), CharT('*'));
  check(SV("answer is  42"), SV("answer is {: d}"), CharT('*'));

  // *** alternate form ***
  check(SV("answer is +42"), SV("answer is {:+#d}"), CharT('*'));
  check(SV("answer is +101010"), SV("answer is {:+b}"), CharT('*'));
  check(SV("answer is +0b101010"), SV("answer is {:+#b}"), CharT('*'));
  check(SV("answer is +0B101010"), SV("answer is {:+#B}"), CharT('*'));
  check(SV("answer is +52"), SV("answer is {:+o}"), CharT('*'));
  check(SV("answer is +052"), SV("answer is {:+#o}"), CharT('*'));
  check(SV("answer is +2a"), SV("answer is {:+x}"), CharT('*'));
  check(SV("answer is +0x2a"), SV("answer is {:+#x}"), CharT('*'));
  check(SV("answer is +2A"), SV("answer is {:+X}"), CharT('*'));
  check(SV("answer is +0X2A"), SV("answer is {:+#X}"), CharT('*'));

  // *** zero-padding & width ***
  check(SV("answer is +00000000042"), SV("answer is {:+#012d}"), CharT('*'));
  check(SV("answer is +00000101010"), SV("answer is {:+012b}"), CharT('*'));
  check(SV("answer is +0b000101010"), SV("answer is {:+#012b}"), CharT('*'));
  check(SV("answer is +0B000101010"), SV("answer is {:+#012B}"), CharT('*'));
  check(SV("answer is +00000000052"), SV("answer is {:+012o}"), CharT('*'));
  check(SV("answer is +00000000052"), SV("answer is {:+#012o}"), CharT('*'));
  check(SV("answer is +0000000002a"), SV("answer is {:+012x}"), CharT('*'));
  check(SV("answer is +0x00000002a"), SV("answer is {:+#012x}"), CharT('*'));
  check(SV("answer is +0000000002A"), SV("answer is {:+012X}"), CharT('*'));

  check(SV("answer is +0X00000002A"), SV("answer is {:+#012X}"), CharT('*'));

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.d}"), CharT('*'));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0d}"), CharT('*'));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42d}"), CharT('*'));

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp

  // *** type ***
#if TEST_STD_VER > 20
  const char* valid_types = "bBcdoxX?";
#else
  const char* valid_types = "bBcdoxX";
#endif
  for (const auto& fmt : invalid_types<CharT>(valid_types))
    check_exception("The type option contains an invalid value for a character formatting argument", fmt, CharT('*'));
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_hex_lower_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // Test whether the hexadecimal letters are the proper case.
  // The precision is too large for float, so two tests are used.
  check(SV("answer is '1.abcp+0'"), SV("answer is '{:a}'"), F(0x1.abcp+0));
  check(SV("answer is '1.defp+0'"), SV("answer is '{:a}'"), F(0x1.defp+0));

  // *** align-fill & width ***
  check(SV("answer is '   1p-2'"), SV("answer is '{:7a}'"), F(0.25));
  check(SV("answer is '   1p-2'"), SV("answer is '{:>7a}'"), F(0.25));
  check(SV("answer is '1p-2   '"), SV("answer is '{:<7a}'"), F(0.25));
  check(SV("answer is ' 1p-2  '"), SV("answer is '{:^7a}'"), F(0.25));

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::1p-3'"), SV("answer is '{::>7a}'"), F(125e-3));
  check(SV("answer is '1p-3:::'"), SV("answer is '{::<7a}'"), F(125e-3));
  check(SV("answer is ':1p-3::'"), SV("answer is '{::^7a}'"), F(125e-3));

  check(SV("answer is '***inf'"), SV("answer is '{:*>6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf***'"), SV("answer is '{:*<6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*inf**'"), SV("answer is '{:*^6a}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-inf'"), SV("answer is '{:#>7a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf###'"), SV("answer is '{:#<7a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-inf##'"), SV("answer is '{:#^7a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^nan'"), SV("answer is '{:^>6a}'"), nan_pos);
  check(SV("answer is 'nan^^^'"), SV("answer is '{:^<6a}'"), nan_pos);
  check(SV("answer is '^nan^^'"), SV("answer is '{:^^6a}'"), nan_pos);

  check(SV("answer is '000-nan'"), SV("answer is '{:0>7a}'"), nan_neg);
  check(SV("answer is '-nan000'"), SV("answer is '{:0<7a}'"), nan_neg);
  check(SV("answer is '0-nan00'"), SV("answer is '{:0^7a}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   1p-2'"), SV("answer is '{:>07a}'"), F(0.25));
  check(SV("answer is '1p-2   '"), SV("answer is '{:<07a}'"), F(0.25));
  check(SV("answer is ' 1p-2  '"), SV("answer is '{:^07a}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0p+0'"), SV("answer is '{:a}'"), F(0));
  check(SV("answer is '0p+0'"), SV("answer is '{:-a}'"), F(0));
  check(SV("answer is '+0p+0'"), SV("answer is '{:+a}'"), F(0));
  check(SV("answer is ' 0p+0'"), SV("answer is '{: a}'"), F(0));

  check(SV("answer is '-0p+0'"), SV("answer is '{:a}'"), F(-0.));
  check(SV("answer is '-0p+0'"), SV("answer is '{:-a}'"), F(-0.));
  check(SV("answer is '-0p+0'"), SV("answer is '{:+a}'"), F(-0.));
  check(SV("answer is '-0p+0'"), SV("answer is '{: a}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'inf'"), SV("answer is '{:a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf'"), SV("answer is '{:-a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+inf'"), SV("answer is '{:+a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' inf'"), SV("answer is '{: a}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-inf'"), SV("answer is '{:a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:-a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:+a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{: a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:a}'"), nan_pos);
  check(SV("answer is 'nan'"), SV("answer is '{:-a}'"), nan_pos);
  check(SV("answer is '+nan'"), SV("answer is '{:+a}'"), nan_pos);
  check(SV("answer is ' nan'"), SV("answer is '{: a}'"), nan_pos);

  check(SV("answer is '-nan'"), SV("answer is '{:a}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:-a}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:+a}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{: a}'"), nan_neg);

  // *** alternate form ***
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0p+0'"), SV("answer is '{:a}'"), F(0));
  check(SV("answer is '0.p+0'"), SV("answer is '{:#a}'"), F(0));

  check(SV("answer is '1p+1'"), SV("answer is '{:.0a}'"), F(2.5));
  check(SV("answer is '1.p+1'"), SV("answer is '{:#.0a}'"), F(2.5));
  check(SV("answer is '1.4p+1'"), SV("answer is '{:#a}'"), F(2.5));

  check(SV("answer is 'inf'"), SV("answer is '{:#a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:#a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:#a}'"), nan_pos);
  check(SV("answer is '-nan'"), SV("answer is '{:#a}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '1p-5'"), SV("answer is '{:04a}'"), 0.03125);
  check(SV("answer is '+1p-5'"), SV("answer is '{:+05a}'"), 0.03125);
  check(SV("answer is '+01p-5'"), SV("answer is '{:+06a}'"), 0.03125);

  check(SV("answer is '0001p-5'"), SV("answer is '{:07a}'"), 0.03125);
  check(SV("answer is '0001p-5'"), SV("answer is '{:-07a}'"), 0.03125);
  check(SV("answer is '+001p-5'"), SV("answer is '{:+07a}'"), 0.03125);
  check(SV("answer is ' 001p-5'"), SV("answer is '{: 07a}'"), 0.03125);

  check(SV("answer is '       inf'"), SV("answer is '{:010a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{:-010a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +inf'"), SV("answer is '{:+010a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{: 010a}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -inf'"), SV("answer is '{:010a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:-010a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:+010a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{: 010a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       nan'"), SV("answer is '{:010a}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{:-010a}'"), nan_pos);
  check(SV("answer is '      +nan'"), SV("answer is '{:+010a}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{: 010a}'"), nan_pos);

  check(SV("answer is '      -nan'"), SV("answer is '{:010a}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:-010a}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:+010a}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{: 010a}'"), nan_neg);

  // *** precision ***
  // See format_test_floating_point_hex_lower_case_precision

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_hex_upper_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // Test whether the hexadecimal letters are the proper case.
  // The precision is too large for float, so two tests are used.
  check(SV("answer is '1.ABCP+0'"), SV("answer is '{:A}'"), F(0x1.abcp+0));
  check(SV("answer is '1.DEFP+0'"), SV("answer is '{:A}'"), F(0x1.defp+0));

  // *** align-fill & width ***
  check(SV("answer is '   1P-2'"), SV("answer is '{:7A}'"), F(0.25));
  check(SV("answer is '   1P-2'"), SV("answer is '{:>7A}'"), F(0.25));
  check(SV("answer is '1P-2   '"), SV("answer is '{:<7A}'"), F(0.25));
  check(SV("answer is ' 1P-2  '"), SV("answer is '{:^7A}'"), F(0.25));

  check(SV("answer is '---1P-3'"), SV("answer is '{:->7A}'"), F(125e-3));
  check(SV("answer is '1P-3---'"), SV("answer is '{:-<7A}'"), F(125e-3));
  check(SV("answer is '-1P-3--'"), SV("answer is '{:-^7A}'"), F(125e-3));

  check(SV("answer is '***INF'"), SV("answer is '{:*>6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF***'"), SV("answer is '{:*<6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*INF**'"), SV("answer is '{:*^6A}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-INF'"), SV("answer is '{:#>7A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF###'"), SV("answer is '{:#<7A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-INF##'"), SV("answer is '{:#^7A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^NAN'"), SV("answer is '{:^>6A}'"), nan_pos);
  check(SV("answer is 'NAN^^^'"), SV("answer is '{:^<6A}'"), nan_pos);
  check(SV("answer is '^NAN^^'"), SV("answer is '{:^^6A}'"), nan_pos);

  check(SV("answer is '000-NAN'"), SV("answer is '{:0>7A}'"), nan_neg);
  check(SV("answer is '-NAN000'"), SV("answer is '{:0<7A}'"), nan_neg);
  check(SV("answer is '0-NAN00'"), SV("answer is '{:0^7A}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   1P-2'"), SV("answer is '{:>07A}'"), F(0.25));
  check(SV("answer is '1P-2   '"), SV("answer is '{:<07A}'"), F(0.25));
  check(SV("answer is ' 1P-2  '"), SV("answer is '{:^07A}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0P+0'"), SV("answer is '{:A}'"), F(0));
  check(SV("answer is '0P+0'"), SV("answer is '{:-A}'"), F(0));
  check(SV("answer is '+0P+0'"), SV("answer is '{:+A}'"), F(0));
  check(SV("answer is ' 0P+0'"), SV("answer is '{: A}'"), F(0));

  check(SV("answer is '-0P+0'"), SV("answer is '{:A}'"), F(-0.));
  check(SV("answer is '-0P+0'"), SV("answer is '{:-A}'"), F(-0.));
  check(SV("answer is '-0P+0'"), SV("answer is '{:+A}'"), F(-0.));
  check(SV("answer is '-0P+0'"), SV("answer is '{: A}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'INF'"), SV("answer is '{:A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF'"), SV("answer is '{:-A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+INF'"), SV("answer is '{:+A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' INF'"), SV("answer is '{: A}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-INF'"), SV("answer is '{:A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:-A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:+A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{: A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:A}'"), nan_pos);
  check(SV("answer is 'NAN'"), SV("answer is '{:-A}'"), nan_pos);
  check(SV("answer is '+NAN'"), SV("answer is '{:+A}'"), nan_pos);
  check(SV("answer is ' NAN'"), SV("answer is '{: A}'"), nan_pos);

  check(SV("answer is '-NAN'"), SV("answer is '{:A}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:-A}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:+A}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{: A}'"), nan_neg);

  // *** alternate form ***
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0P+0'"), SV("answer is '{:A}'"), F(0));
  check(SV("answer is '0.P+0'"), SV("answer is '{:#A}'"), F(0));

  check(SV("answer is '1P+1'"), SV("answer is '{:.0A}'"), F(2.5));
  check(SV("answer is '1.P+1'"), SV("answer is '{:#.0A}'"), F(2.5));
  check(SV("answer is '1.4P+1'"), SV("answer is '{:#A}'"), F(2.5));

  check(SV("answer is 'INF'"), SV("answer is '{:#A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:#A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:#A}'"), nan_pos);
  check(SV("answer is '-NAN'"), SV("answer is '{:#A}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '1P-5'"), SV("answer is '{:04A}'"), 0.03125);
  check(SV("answer is '+1P-5'"), SV("answer is '{:+05A}'"), 0.03125);
  check(SV("answer is '+01P-5'"), SV("answer is '{:+06A}'"), 0.03125);

  check(SV("answer is '0001P-5'"), SV("answer is '{:07A}'"), 0.03125);
  check(SV("answer is '0001P-5'"), SV("answer is '{:-07A}'"), 0.03125);
  check(SV("answer is '+001P-5'"), SV("answer is '{:+07A}'"), 0.03125);
  check(SV("answer is ' 001P-5'"), SV("answer is '{: 07A}'"), 0.03125);

  check(SV("answer is '       INF'"), SV("answer is '{:010A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{:-010A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +INF'"), SV("answer is '{:+010A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{: 010A}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -INF'"), SV("answer is '{:010A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:-010A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:+010A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{: 010A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       NAN'"), SV("answer is '{:010A}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{:-010A}'"), nan_pos);
  check(SV("answer is '      +NAN'"), SV("answer is '{:+010A}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{: 010A}'"), nan_pos);

  check(SV("answer is '      -NAN'"), SV("answer is '{:010A}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:-010A}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:+010A}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{: 010A}'"), nan_neg);

  // *** precision ***
  // See format_test_floating_point_hex_upper_case_precision

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_hex_lower_case_precision(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   1.000000p-2'"), SV("answer is '{:14.6a}'"), F(0.25));
  check(SV("answer is '   1.000000p-2'"), SV("answer is '{:>14.6a}'"), F(0.25));
  check(SV("answer is '1.000000p-2   '"), SV("answer is '{:<14.6a}'"), F(0.25));
  check(SV("answer is ' 1.000000p-2  '"), SV("answer is '{:^14.6a}'"), F(0.25));

  check(SV("answer is '---1.000000p-3'"), SV("answer is '{:->14.6a}'"), F(125e-3));
  check(SV("answer is '1.000000p-3---'"), SV("answer is '{:-<14.6a}'"), F(125e-3));
  check(SV("answer is '-1.000000p-3--'"), SV("answer is '{:-^14.6a}'"), F(125e-3));

  check(SV("answer is '***inf'"), SV("answer is '{:*>6.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf***'"), SV("answer is '{:*<6.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*inf**'"), SV("answer is '{:*^6.6a}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-inf'"), SV("answer is '{:#>7.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf###'"), SV("answer is '{:#<7.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-inf##'"), SV("answer is '{:#^7.6a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^nan'"), SV("answer is '{:^>6.6a}'"), nan_pos);
  check(SV("answer is 'nan^^^'"), SV("answer is '{:^<6.6a}'"), nan_pos);
  check(SV("answer is '^nan^^'"), SV("answer is '{:^^6.6a}'"), nan_pos);

  check(SV("answer is '000-nan'"), SV("answer is '{:0>7.6a}'"), nan_neg);
  check(SV("answer is '-nan000'"), SV("answer is '{:0<7.6a}'"), nan_neg);
  check(SV("answer is '0-nan00'"), SV("answer is '{:0^7.6a}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   1.000000p-2'"), SV("answer is '{:>014.6a}'"), F(0.25));
  check(SV("answer is '1.000000p-2   '"), SV("answer is '{:<014.6a}'"), F(0.25));
  check(SV("answer is ' 1.000000p-2  '"), SV("answer is '{:^014.6a}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0.000000p+0'"), SV("answer is '{:.6a}'"), F(0));
  check(SV("answer is '0.000000p+0'"), SV("answer is '{:-.6a}'"), F(0));
  check(SV("answer is '+0.000000p+0'"), SV("answer is '{:+.6a}'"), F(0));
  check(SV("answer is ' 0.000000p+0'"), SV("answer is '{: .6a}'"), F(0));

  check(SV("answer is '-0.000000p+0'"), SV("answer is '{:.6a}'"), F(-0.));
  check(SV("answer is '-0.000000p+0'"), SV("answer is '{:-.6a}'"), F(-0.));
  check(SV("answer is '-0.000000p+0'"), SV("answer is '{:+.6a}'"), F(-0.));
  check(SV("answer is '-0.000000p+0'"), SV("answer is '{: .6a}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'inf'"), SV("answer is '{:.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf'"), SV("answer is '{:-.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+inf'"), SV("answer is '{:+.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' inf'"), SV("answer is '{: .6a}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-inf'"), SV("answer is '{:.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:-.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:+.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{: .6a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:.6a}'"), nan_pos);
  check(SV("answer is 'nan'"), SV("answer is '{:-.6a}'"), nan_pos);
  check(SV("answer is '+nan'"), SV("answer is '{:+.6a}'"), nan_pos);
  check(SV("answer is ' nan'"), SV("answer is '{: .6a}'"), nan_pos);

  check(SV("answer is '-nan'"), SV("answer is '{:.6a}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:-.6a}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:+.6a}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{: .6a}'"), nan_neg);

  // *** alternate form ***
  check(SV("answer is '1.400000p+1'"), SV("answer is '{:#.6a}'"), F(2.5));

  check(SV("answer is 'inf'"), SV("answer is '{:#.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:#.6a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:#.6a}'"), nan_pos);
  check(SV("answer is '-nan'"), SV("answer is '{:#.6a}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '1.000000p-5'"), SV("answer is '{:011.6a}'"), 0.03125);
  check(SV("answer is '+1.000000p-5'"), SV("answer is '{:+012.6a}'"), 0.03125);
  check(SV("answer is '+01.000000p-5'"), SV("answer is '{:+013.6a}'"), 0.03125);

  check(SV("answer is '0001.000000p-5'"), SV("answer is '{:014.6a}'"), 0.03125);
  check(SV("answer is '0001.000000p-5'"), SV("answer is '{:-014.6a}'"), 0.03125);
  check(SV("answer is '+001.000000p-5'"), SV("answer is '{:+014.6a}'"), 0.03125);
  check(SV("answer is ' 001.000000p-5'"), SV("answer is '{: 014.6a}'"), 0.03125);

  check(SV("answer is '       inf'"), SV("answer is '{:010.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{:-010.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +inf'"), SV("answer is '{:+010.6a}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{: 010.6a}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -inf'"), SV("answer is '{:010.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:-010.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:+010.6a}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{: 010.6a}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       nan'"), SV("answer is '{:010.6a}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{:-010.6a}'"), nan_pos);
  check(SV("answer is '      +nan'"), SV("answer is '{:+010.6a}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{: 010.6a}'"), nan_pos);

  check(SV("answer is '      -nan'"), SV("answer is '{:010.6a}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:-010.6a}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:+010.6a}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{: 010.6a}'"), nan_neg);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_hex_upper_case_precision(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   1.000000P-2'"), SV("answer is '{:14.6A}'"), F(0.25));
  check(SV("answer is '   1.000000P-2'"), SV("answer is '{:>14.6A}'"), F(0.25));
  check(SV("answer is '1.000000P-2   '"), SV("answer is '{:<14.6A}'"), F(0.25));
  check(SV("answer is ' 1.000000P-2  '"), SV("answer is '{:^14.6A}'"), F(0.25));

  check(SV("answer is '---1.000000P-3'"), SV("answer is '{:->14.6A}'"), F(125e-3));
  check(SV("answer is '1.000000P-3---'"), SV("answer is '{:-<14.6A}'"), F(125e-3));
  check(SV("answer is '-1.000000P-3--'"), SV("answer is '{:-^14.6A}'"), F(125e-3));

  check(SV("answer is '***INF'"), SV("answer is '{:*>6.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF***'"), SV("answer is '{:*<6.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*INF**'"), SV("answer is '{:*^6.6A}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-INF'"), SV("answer is '{:#>7.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF###'"), SV("answer is '{:#<7.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-INF##'"), SV("answer is '{:#^7.6A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^NAN'"), SV("answer is '{:^>6.6A}'"), nan_pos);
  check(SV("answer is 'NAN^^^'"), SV("answer is '{:^<6.6A}'"), nan_pos);
  check(SV("answer is '^NAN^^'"), SV("answer is '{:^^6.6A}'"), nan_pos);

  check(SV("answer is '000-NAN'"), SV("answer is '{:0>7.6A}'"), nan_neg);
  check(SV("answer is '-NAN000'"), SV("answer is '{:0<7.6A}'"), nan_neg);
  check(SV("answer is '0-NAN00'"), SV("answer is '{:0^7.6A}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   1.000000P-2'"), SV("answer is '{:>014.6A}'"), F(0.25));
  check(SV("answer is '1.000000P-2   '"), SV("answer is '{:<014.6A}'"), F(0.25));
  check(SV("answer is ' 1.000000P-2  '"), SV("answer is '{:^014.6A}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0.000000P+0'"), SV("answer is '{:.6A}'"), F(0));
  check(SV("answer is '0.000000P+0'"), SV("answer is '{:-.6A}'"), F(0));
  check(SV("answer is '+0.000000P+0'"), SV("answer is '{:+.6A}'"), F(0));
  check(SV("answer is ' 0.000000P+0'"), SV("answer is '{: .6A}'"), F(0));

  check(SV("answer is '-0.000000P+0'"), SV("answer is '{:.6A}'"), F(-0.));
  check(SV("answer is '-0.000000P+0'"), SV("answer is '{:-.6A}'"), F(-0.));
  check(SV("answer is '-0.000000P+0'"), SV("answer is '{:+.6A}'"), F(-0.));
  check(SV("answer is '-0.000000P+0'"), SV("answer is '{: .6A}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'INF'"), SV("answer is '{:.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF'"), SV("answer is '{:-.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+INF'"), SV("answer is '{:+.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' INF'"), SV("answer is '{: .6A}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-INF'"), SV("answer is '{:.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:-.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:+.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{: .6A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:.6A}'"), nan_pos);
  check(SV("answer is 'NAN'"), SV("answer is '{:-.6A}'"), nan_pos);
  check(SV("answer is '+NAN'"), SV("answer is '{:+.6A}'"), nan_pos);
  check(SV("answer is ' NAN'"), SV("answer is '{: .6A}'"), nan_pos);

  check(SV("answer is '-NAN'"), SV("answer is '{:.6A}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:-.6A}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:+.6A}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{: .6A}'"), nan_neg);

  // *** alternate form ***
  check(SV("answer is '1.400000P+1'"), SV("answer is '{:#.6A}'"), F(2.5));

  check(SV("answer is 'INF'"), SV("answer is '{:#.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:#.6A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:#.6A}'"), nan_pos);
  check(SV("answer is '-NAN'"), SV("answer is '{:#.6A}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '1.000000P-5'"), SV("answer is '{:011.6A}'"), 0.03125);
  check(SV("answer is '+1.000000P-5'"), SV("answer is '{:+012.6A}'"), 0.03125);
  check(SV("answer is '+01.000000P-5'"), SV("answer is '{:+013.6A}'"), 0.03125);

  check(SV("answer is '0001.000000P-5'"), SV("answer is '{:014.6A}'"), 0.03125);
  check(SV("answer is '0001.000000P-5'"), SV("answer is '{:-014.6A}'"), 0.03125);
  check(SV("answer is '+001.000000P-5'"), SV("answer is '{:+014.6A}'"), 0.03125);
  check(SV("answer is ' 001.000000P-5'"), SV("answer is '{: 014.6A}'"), 0.03125);

  check(SV("answer is '       INF'"), SV("answer is '{:010.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{:-010.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +INF'"), SV("answer is '{:+010.6A}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{: 010.6A}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -INF'"), SV("answer is '{:010.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:-010.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:+010.6A}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{: 010.6A}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       NAN'"), SV("answer is '{:010.6A}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{:-010.6A}'"), nan_pos);
  check(SV("answer is '      +NAN'"), SV("answer is '{:+010.6A}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{: 010.6A}'"), nan_pos);

  check(SV("answer is '      -NAN'"), SV("answer is '{:010.6A}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:-010.6A}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:+010.6A}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{: 010.6A}'"), nan_neg);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_scientific_lower_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   2.500000e-01'"), SV("answer is '{:15e}'"), F(0.25));
  check(SV("answer is '   2.500000e-01'"), SV("answer is '{:>15e}'"), F(0.25));
  check(SV("answer is '2.500000e-01   '"), SV("answer is '{:<15e}'"), F(0.25));
  check(SV("answer is ' 2.500000e-01  '"), SV("answer is '{:^15e}'"), F(0.25));

  check(SV("answer is '---1.250000e-01'"), SV("answer is '{:->15e}'"), F(125e-3));
  check(SV("answer is '1.250000e-01---'"), SV("answer is '{:-<15e}'"), F(125e-3));
  check(SV("answer is '-1.250000e-01--'"), SV("answer is '{:-^15e}'"), F(125e-3));

  check(SV("answer is '***inf'"), SV("answer is '{:*>6e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf***'"), SV("answer is '{:*<6e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*inf**'"), SV("answer is '{:*^6e}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-inf'"), SV("answer is '{:#>7e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf###'"), SV("answer is '{:#<7e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-inf##'"), SV("answer is '{:#^7e}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^nan'"), SV("answer is '{:^>6e}'"), nan_pos);
  check(SV("answer is 'nan^^^'"), SV("answer is '{:^<6e}'"), nan_pos);
  check(SV("answer is '^nan^^'"), SV("answer is '{:^^6e}'"), nan_pos);

  check(SV("answer is '000-nan'"), SV("answer is '{:0>7e}'"), nan_neg);
  check(SV("answer is '-nan000'"), SV("answer is '{:0<7e}'"), nan_neg);
  check(SV("answer is '0-nan00'"), SV("answer is '{:0^7e}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   2.500000e-01'"), SV("answer is '{:>015e}'"), F(0.25));
  check(SV("answer is '2.500000e-01   '"), SV("answer is '{:<015e}'"), F(0.25));
  check(SV("answer is ' 2.500000e-01  '"), SV("answer is '{:^015e}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0.000000e+00'"), SV("answer is '{:e}'"), F(0));
  check(SV("answer is '0.000000e+00'"), SV("answer is '{:-e}'"), F(0));
  check(SV("answer is '+0.000000e+00'"), SV("answer is '{:+e}'"), F(0));
  check(SV("answer is ' 0.000000e+00'"), SV("answer is '{: e}'"), F(0));

  check(SV("answer is '-0.000000e+00'"), SV("answer is '{:e}'"), F(-0.));
  check(SV("answer is '-0.000000e+00'"), SV("answer is '{:-e}'"), F(-0.));
  check(SV("answer is '-0.000000e+00'"), SV("answer is '{:+e}'"), F(-0.));
  check(SV("answer is '-0.000000e+00'"), SV("answer is '{: e}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'inf'"), SV("answer is '{:e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf'"), SV("answer is '{:-e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+inf'"), SV("answer is '{:+e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' inf'"), SV("answer is '{: e}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-inf'"), SV("answer is '{:e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:-e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:+e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{: e}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:e}'"), nan_pos);
  check(SV("answer is 'nan'"), SV("answer is '{:-e}'"), nan_pos);
  check(SV("answer is '+nan'"), SV("answer is '{:+e}'"), nan_pos);
  check(SV("answer is ' nan'"), SV("answer is '{: e}'"), nan_pos);

  check(SV("answer is '-nan'"), SV("answer is '{:e}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:-e}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:+e}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{: e}'"), nan_neg);

  // *** alternate form **
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0e+00'"), SV("answer is '{:.0e}'"), F(0));
  check(SV("answer is '0.e+00'"), SV("answer is '{:#.0e}'"), F(0));

  check(SV("answer is '0.000000e+00'"), SV("answer is '{:#e}'"), F(0));
  check(SV("answer is '2.500000e+00'"), SV("answer is '{:#e}'"), F(2.5));

  check(SV("answer is 'inf'"), SV("answer is '{:#e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:#e}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:#e}'"), nan_pos);
  check(SV("answer is '-nan'"), SV("answer is '{:#e}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '3.125000e-02'"), SV("answer is '{:07e}'"), 0.03125);
  check(SV("answer is '+3.125000e-02'"), SV("answer is '{:+07e}'"), 0.03125);
  check(SV("answer is '+3.125000e-02'"), SV("answer is '{:+08e}'"), 0.03125);
  check(SV("answer is '+3.125000e-02'"), SV("answer is '{:+09e}'"), 0.03125);

  check(SV("answer is '003.125000e-02'"), SV("answer is '{:014e}'"), 0.03125);
  check(SV("answer is '003.125000e-02'"), SV("answer is '{:-014e}'"), 0.03125);
  check(SV("answer is '+03.125000e-02'"), SV("answer is '{:+014e}'"), 0.03125);
  check(SV("answer is ' 03.125000e-02'"), SV("answer is '{: 014e}'"), 0.03125);

  check(SV("answer is '       inf'"), SV("answer is '{:010e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{:-010e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +inf'"), SV("answer is '{:+010e}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{: 010e}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -inf'"), SV("answer is '{:010e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:-010e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:+010e}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{: 010e}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       nan'"), SV("answer is '{:010e}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{:-010e}'"), nan_pos);
  check(SV("answer is '      +nan'"), SV("answer is '{:+010e}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{: 010e}'"), nan_pos);

  check(SV("answer is '      -nan'"), SV("answer is '{:010e}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:-010e}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:+010e}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{: 010e}'"), nan_neg);

  // *** precision ***
  check(SV("answer is '3e-02'"), SV("answer is '{:.0e}'"), 0.03125);
  check(SV("answer is '3.1e-02'"), SV("answer is '{:.1e}'"), 0.03125);
  check(SV("answer is '3.125e-02'"), SV("answer is '{:.3e}'"), 0.03125);
  check(SV("answer is '3.1250000000e-02'"), SV("answer is '{:.10e}'"), 0.03125);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_scientific_upper_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   2.500000E-01'"), SV("answer is '{:15E}'"), F(0.25));
  check(SV("answer is '   2.500000E-01'"), SV("answer is '{:>15E}'"), F(0.25));
  check(SV("answer is '2.500000E-01   '"), SV("answer is '{:<15E}'"), F(0.25));
  check(SV("answer is ' 2.500000E-01  '"), SV("answer is '{:^15E}'"), F(0.25));

  check(SV("answer is '---1.250000E-01'"), SV("answer is '{:->15E}'"), F(125e-3));
  check(SV("answer is '1.250000E-01---'"), SV("answer is '{:-<15E}'"), F(125e-3));
  check(SV("answer is '-1.250000E-01--'"), SV("answer is '{:-^15E}'"), F(125e-3));

  check(SV("answer is '***INF'"), SV("answer is '{:*>6E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF***'"), SV("answer is '{:*<6E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*INF**'"), SV("answer is '{:*^6E}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-INF'"), SV("answer is '{:#>7E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF###'"), SV("answer is '{:#<7E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-INF##'"), SV("answer is '{:#^7E}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^NAN'"), SV("answer is '{:^>6E}'"), nan_pos);
  check(SV("answer is 'NAN^^^'"), SV("answer is '{:^<6E}'"), nan_pos);
  check(SV("answer is '^NAN^^'"), SV("answer is '{:^^6E}'"), nan_pos);

  check(SV("answer is '000-NAN'"), SV("answer is '{:0>7E}'"), nan_neg);
  check(SV("answer is '-NAN000'"), SV("answer is '{:0<7E}'"), nan_neg);
  check(SV("answer is '0-NAN00'"), SV("answer is '{:0^7E}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   2.500000E-01'"), SV("answer is '{:>015E}'"), F(0.25));
  check(SV("answer is '2.500000E-01   '"), SV("answer is '{:<015E}'"), F(0.25));
  check(SV("answer is ' 2.500000E-01  '"), SV("answer is '{:^015E}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0.000000E+00'"), SV("answer is '{:E}'"), F(0));
  check(SV("answer is '0.000000E+00'"), SV("answer is '{:-E}'"), F(0));
  check(SV("answer is '+0.000000E+00'"), SV("answer is '{:+E}'"), F(0));
  check(SV("answer is ' 0.000000E+00'"), SV("answer is '{: E}'"), F(0));

  check(SV("answer is '-0.000000E+00'"), SV("answer is '{:E}'"), F(-0.));
  check(SV("answer is '-0.000000E+00'"), SV("answer is '{:-E}'"), F(-0.));
  check(SV("answer is '-0.000000E+00'"), SV("answer is '{:+E}'"), F(-0.));
  check(SV("answer is '-0.000000E+00'"), SV("answer is '{: E}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'INF'"), SV("answer is '{:E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF'"), SV("answer is '{:-E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+INF'"), SV("answer is '{:+E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' INF'"), SV("answer is '{: E}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-INF'"), SV("answer is '{:E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:-E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:+E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{: E}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:E}'"), nan_pos);
  check(SV("answer is 'NAN'"), SV("answer is '{:-E}'"), nan_pos);
  check(SV("answer is '+NAN'"), SV("answer is '{:+E}'"), nan_pos);
  check(SV("answer is ' NAN'"), SV("answer is '{: E}'"), nan_pos);

  check(SV("answer is '-NAN'"), SV("answer is '{:E}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:-E}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:+E}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{: E}'"), nan_neg);

  // *** alternate form **
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0E+00'"), SV("answer is '{:.0E}'"), F(0));
  check(SV("answer is '0.E+00'"), SV("answer is '{:#.0E}'"), F(0));

  check(SV("answer is '0.000000E+00'"), SV("answer is '{:#E}'"), F(0));
  check(SV("answer is '2.500000E+00'"), SV("answer is '{:#E}'"), F(2.5));

  check(SV("answer is 'INF'"), SV("answer is '{:#E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:#E}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:#E}'"), nan_pos);
  check(SV("answer is '-NAN'"), SV("answer is '{:#E}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '3.125000E-02'"), SV("answer is '{:07E}'"), 0.03125);
  check(SV("answer is '+3.125000E-02'"), SV("answer is '{:+07E}'"), 0.03125);
  check(SV("answer is '+3.125000E-02'"), SV("answer is '{:+08E}'"), 0.03125);
  check(SV("answer is '+3.125000E-02'"), SV("answer is '{:+09E}'"), 0.03125);

  check(SV("answer is '003.125000E-02'"), SV("answer is '{:014E}'"), 0.03125);
  check(SV("answer is '003.125000E-02'"), SV("answer is '{:-014E}'"), 0.03125);
  check(SV("answer is '+03.125000E-02'"), SV("answer is '{:+014E}'"), 0.03125);
  check(SV("answer is ' 03.125000E-02'"), SV("answer is '{: 014E}'"), 0.03125);

  check(SV("answer is '       INF'"), SV("answer is '{:010E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{:-010E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +INF'"), SV("answer is '{:+010E}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{: 010E}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -INF'"), SV("answer is '{:010E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:-010E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:+010E}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{: 010E}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       NAN'"), SV("answer is '{:010E}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{:-010E}'"), nan_pos);
  check(SV("answer is '      +NAN'"), SV("answer is '{:+010E}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{: 010E}'"), nan_pos);

  check(SV("answer is '      -NAN'"), SV("answer is '{:010E}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:-010E}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:+010E}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{: 010E}'"), nan_neg);

  // *** precision ***
  check(SV("answer is '3E-02'"), SV("answer is '{:.0E}'"), 0.03125);
  check(SV("answer is '3.1E-02'"), SV("answer is '{:.1E}'"), 0.03125);
  check(SV("answer is '3.125E-02'"), SV("answer is '{:.3E}'"), 0.03125);
  check(SV("answer is '3.1250000000E-02'"), SV("answer is '{:.10E}'"), 0.03125);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_fixed_lower_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   0.250000'"), SV("answer is '{:11f}'"), F(0.25));
  check(SV("answer is '   0.250000'"), SV("answer is '{:>11f}'"), F(0.25));
  check(SV("answer is '0.250000   '"), SV("answer is '{:<11f}'"), F(0.25));
  check(SV("answer is ' 0.250000  '"), SV("answer is '{:^11f}'"), F(0.25));

  check(SV("answer is '---0.125000'"), SV("answer is '{:->11f}'"), F(125e-3));
  check(SV("answer is '0.125000---'"), SV("answer is '{:-<11f}'"), F(125e-3));
  check(SV("answer is '-0.125000--'"), SV("answer is '{:-^11f}'"), F(125e-3));

  check(SV("answer is '***inf'"), SV("answer is '{:*>6f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf***'"), SV("answer is '{:*<6f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*inf**'"), SV("answer is '{:*^6f}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-inf'"), SV("answer is '{:#>7f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf###'"), SV("answer is '{:#<7f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-inf##'"), SV("answer is '{:#^7f}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^nan'"), SV("answer is '{:^>6f}'"), nan_pos);
  check(SV("answer is 'nan^^^'"), SV("answer is '{:^<6f}'"), nan_pos);
  check(SV("answer is '^nan^^'"), SV("answer is '{:^^6f}'"), nan_pos);

  check(SV("answer is '000-nan'"), SV("answer is '{:0>7f}'"), nan_neg);
  check(SV("answer is '-nan000'"), SV("answer is '{:0<7f}'"), nan_neg);
  check(SV("answer is '0-nan00'"), SV("answer is '{:0^7f}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   0.250000'"), SV("answer is '{:>011f}'"), F(0.25));
  check(SV("answer is '0.250000   '"), SV("answer is '{:<011f}'"), F(0.25));
  check(SV("answer is ' 0.250000  '"), SV("answer is '{:^011f}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0.000000'"), SV("answer is '{:f}'"), F(0));
  check(SV("answer is '0.000000'"), SV("answer is '{:-f}'"), F(0));
  check(SV("answer is '+0.000000'"), SV("answer is '{:+f}'"), F(0));
  check(SV("answer is ' 0.000000'"), SV("answer is '{: f}'"), F(0));

  check(SV("answer is '-0.000000'"), SV("answer is '{:f}'"), F(-0.));
  check(SV("answer is '-0.000000'"), SV("answer is '{:-f}'"), F(-0.));
  check(SV("answer is '-0.000000'"), SV("answer is '{:+f}'"), F(-0.));
  check(SV("answer is '-0.000000'"), SV("answer is '{: f}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'inf'"), SV("answer is '{:f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf'"), SV("answer is '{:-f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+inf'"), SV("answer is '{:+f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' inf'"), SV("answer is '{: f}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-inf'"), SV("answer is '{:f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:-f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:+f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{: f}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:f}'"), nan_pos);
  check(SV("answer is 'nan'"), SV("answer is '{:-f}'"), nan_pos);
  check(SV("answer is '+nan'"), SV("answer is '{:+f}'"), nan_pos);
  check(SV("answer is ' nan'"), SV("answer is '{: f}'"), nan_pos);

  check(SV("answer is '-nan'"), SV("answer is '{:f}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:-f}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:+f}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{: f}'"), nan_neg);

  // *** alternate form **
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0'"), SV("answer is '{:.0f}'"), F(0));
  check(SV("answer is '0.'"), SV("answer is '{:#.0f}'"), F(0));

  check(SV("answer is '0.000000'"), SV("answer is '{:#f}'"), F(0));
  check(SV("answer is '2.500000'"), SV("answer is '{:#f}'"), F(2.5));

  check(SV("answer is 'inf'"), SV("answer is '{:#f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:#f}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:#f}'"), nan_pos);
  check(SV("answer is '-nan'"), SV("answer is '{:#f}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '0.031250'"), SV("answer is '{:07f}'"), 0.03125);
  check(SV("answer is '+0.031250'"), SV("answer is '{:+07f}'"), 0.03125);
  check(SV("answer is '+0.031250'"), SV("answer is '{:+08f}'"), 0.03125);
  check(SV("answer is '+0.031250'"), SV("answer is '{:+09f}'"), 0.03125);

  check(SV("answer is '000.031250'"), SV("answer is '{:010f}'"), 0.03125);
  check(SV("answer is '000.031250'"), SV("answer is '{:-010f}'"), 0.03125);
  check(SV("answer is '+00.031250'"), SV("answer is '{:+010f}'"), 0.03125);
  check(SV("answer is ' 00.031250'"), SV("answer is '{: 010f}'"), 0.03125);

  check(SV("answer is '       inf'"), SV("answer is '{:010f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{:-010f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +inf'"), SV("answer is '{:+010f}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{: 010f}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -inf'"), SV("answer is '{:010f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:-010f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:+010f}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{: 010f}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       nan'"), SV("answer is '{:010f}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{:-010f}'"), nan_pos);
  check(SV("answer is '      +nan'"), SV("answer is '{:+010f}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{: 010f}'"), nan_pos);

  check(SV("answer is '      -nan'"), SV("answer is '{:010f}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:-010f}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:+010f}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{: 010f}'"), nan_neg);

  // *** precision ***
  check(SV("answer is '0'"), SV("answer is '{:.0f}'"), 0.03125);
  check(SV("answer is '0.0'"), SV("answer is '{:.1f}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.5f}'"), 0.03125);
  check(SV("answer is '0.0312500000'"), SV("answer is '{:.10f}'"), 0.03125);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_fixed_upper_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   0.250000'"), SV("answer is '{:11F}'"), F(0.25));
  check(SV("answer is '   0.250000'"), SV("answer is '{:>11F}'"), F(0.25));
  check(SV("answer is '0.250000   '"), SV("answer is '{:<11F}'"), F(0.25));
  check(SV("answer is ' 0.250000  '"), SV("answer is '{:^11F}'"), F(0.25));

  check(SV("answer is '---0.125000'"), SV("answer is '{:->11F}'"), F(125e-3));
  check(SV("answer is '0.125000---'"), SV("answer is '{:-<11F}'"), F(125e-3));
  check(SV("answer is '-0.125000--'"), SV("answer is '{:-^11F}'"), F(125e-3));

  check(SV("answer is '***INF'"), SV("answer is '{:*>6F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF***'"), SV("answer is '{:*<6F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*INF**'"), SV("answer is '{:*^6F}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-INF'"), SV("answer is '{:#>7F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF###'"), SV("answer is '{:#<7F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-INF##'"), SV("answer is '{:#^7F}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^NAN'"), SV("answer is '{:^>6F}'"), nan_pos);
  check(SV("answer is 'NAN^^^'"), SV("answer is '{:^<6F}'"), nan_pos);
  check(SV("answer is '^NAN^^'"), SV("answer is '{:^^6F}'"), nan_pos);

  check(SV("answer is '000-NAN'"), SV("answer is '{:0>7F}'"), nan_neg);
  check(SV("answer is '-NAN000'"), SV("answer is '{:0<7F}'"), nan_neg);
  check(SV("answer is '0-NAN00'"), SV("answer is '{:0^7F}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   0.250000'"), SV("answer is '{:>011F}'"), F(0.25));
  check(SV("answer is '0.250000   '"), SV("answer is '{:<011F}'"), F(0.25));
  check(SV("answer is ' 0.250000  '"), SV("answer is '{:^011F}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0.000000'"), SV("answer is '{:F}'"), F(0));
  check(SV("answer is '0.000000'"), SV("answer is '{:-F}'"), F(0));
  check(SV("answer is '+0.000000'"), SV("answer is '{:+F}'"), F(0));
  check(SV("answer is ' 0.000000'"), SV("answer is '{: F}'"), F(0));

  check(SV("answer is '-0.000000'"), SV("answer is '{:F}'"), F(-0.));
  check(SV("answer is '-0.000000'"), SV("answer is '{:-F}'"), F(-0.));
  check(SV("answer is '-0.000000'"), SV("answer is '{:+F}'"), F(-0.));
  check(SV("answer is '-0.000000'"), SV("answer is '{: F}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'INF'"), SV("answer is '{:F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF'"), SV("answer is '{:-F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+INF'"), SV("answer is '{:+F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' INF'"), SV("answer is '{: F}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-INF'"), SV("answer is '{:F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:-F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:+F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{: F}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:F}'"), nan_pos);
  check(SV("answer is 'NAN'"), SV("answer is '{:-F}'"), nan_pos);
  check(SV("answer is '+NAN'"), SV("answer is '{:+F}'"), nan_pos);
  check(SV("answer is ' NAN'"), SV("answer is '{: F}'"), nan_pos);

  check(SV("answer is '-NAN'"), SV("answer is '{:F}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:-F}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:+F}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{: F}'"), nan_neg);

  // *** alternate form **
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0'"), SV("answer is '{:.0F}'"), F(0));
  check(SV("answer is '0.'"), SV("answer is '{:#.0F}'"), F(0));

  check(SV("answer is '0.000000'"), SV("answer is '{:#F}'"), F(0));
  check(SV("answer is '2.500000'"), SV("answer is '{:#F}'"), F(2.5));

  check(SV("answer is 'INF'"), SV("answer is '{:#F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:#F}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:#F}'"), nan_pos);
  check(SV("answer is '-NAN'"), SV("answer is '{:#F}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '0.031250'"), SV("answer is '{:07F}'"), 0.03125);
  check(SV("answer is '+0.031250'"), SV("answer is '{:+07F}'"), 0.03125);
  check(SV("answer is '+0.031250'"), SV("answer is '{:+08F}'"), 0.03125);
  check(SV("answer is '+0.031250'"), SV("answer is '{:+09F}'"), 0.03125);

  check(SV("answer is '000.031250'"), SV("answer is '{:010F}'"), 0.03125);
  check(SV("answer is '000.031250'"), SV("answer is '{:-010F}'"), 0.03125);
  check(SV("answer is '+00.031250'"), SV("answer is '{:+010F}'"), 0.03125);
  check(SV("answer is ' 00.031250'"), SV("answer is '{: 010F}'"), 0.03125);

  check(SV("answer is '       INF'"), SV("answer is '{:010F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{:-010F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +INF'"), SV("answer is '{:+010F}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{: 010F}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -INF'"), SV("answer is '{:010F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:-010F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:+010F}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{: 010F}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       NAN'"), SV("answer is '{:010F}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{:-010F}'"), nan_pos);
  check(SV("answer is '      +NAN'"), SV("answer is '{:+010F}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{: 010F}'"), nan_pos);

  check(SV("answer is '      -NAN'"), SV("answer is '{:010F}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:-010F}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:+010F}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{: 010F}'"), nan_neg);

  // *** precision ***
  check(SV("answer is '0'"), SV("answer is '{:.0F}'"), 0.03125);
  check(SV("answer is '0.0'"), SV("answer is '{:.1F}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.5F}'"), 0.03125);
  check(SV("answer is '0.0312500000'"), SV("answer is '{:.10F}'"), 0.03125);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_general_lower_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   0.25'"), SV("answer is '{:7g}'"), F(0.25));
  check(SV("answer is '   0.25'"), SV("answer is '{:>7g}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<7g}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^7g}'"), F(0.25));

  check(SV("answer is '---0.125'"), SV("answer is '{:->8g}'"), F(125e-3));
  check(SV("answer is '0.125---'"), SV("answer is '{:-<8g}'"), F(125e-3));
  check(SV("answer is '-0.125--'"), SV("answer is '{:-^8g}'"), F(125e-3));

  check(SV("answer is '***inf'"), SV("answer is '{:*>6g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf***'"), SV("answer is '{:*<6g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*inf**'"), SV("answer is '{:*^6g}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-inf'"), SV("answer is '{:#>7g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf###'"), SV("answer is '{:#<7g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-inf##'"), SV("answer is '{:#^7g}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^nan'"), SV("answer is '{:^>6g}'"), nan_pos);
  check(SV("answer is 'nan^^^'"), SV("answer is '{:^<6g}'"), nan_pos);
  check(SV("answer is '^nan^^'"), SV("answer is '{:^^6g}'"), nan_pos);

  check(SV("answer is '000-nan'"), SV("answer is '{:0>7g}'"), nan_neg);
  check(SV("answer is '-nan000'"), SV("answer is '{:0<7g}'"), nan_neg);
  check(SV("answer is '0-nan00'"), SV("answer is '{:0^7g}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   0.25'"), SV("answer is '{:>07g}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<07g}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^07g}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0'"), SV("answer is '{:g}'"), F(0));
  check(SV("answer is '0'"), SV("answer is '{:-g}'"), F(0));
  check(SV("answer is '+0'"), SV("answer is '{:+g}'"), F(0));
  check(SV("answer is ' 0'"), SV("answer is '{: g}'"), F(0));

  check(SV("answer is '-0'"), SV("answer is '{:g}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:-g}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:+g}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{: g}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'inf'"), SV("answer is '{:g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf'"), SV("answer is '{:-g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+inf'"), SV("answer is '{:+g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' inf'"), SV("answer is '{: g}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-inf'"), SV("answer is '{:g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:-g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:+g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{: g}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:g}'"), nan_pos);
  check(SV("answer is 'nan'"), SV("answer is '{:-g}'"), nan_pos);
  check(SV("answer is '+nan'"), SV("answer is '{:+g}'"), nan_pos);
  check(SV("answer is ' nan'"), SV("answer is '{: g}'"), nan_pos);

  check(SV("answer is '-nan'"), SV("answer is '{:g}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:-g}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:+g}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{: g}'"), nan_neg);

  // *** alternate form **
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0'"), SV("answer is '{:.0g}'"), F(0));
  check(SV("answer is '0.'"), SV("answer is '{:#.0g}'"), F(0));

  check(SV("answer is '0.00000'"), SV("answer is '{:#g}'"), F(0));
  check(SV("answer is '2.50000'"), SV("answer is '{:#g}'"), F(2.5));

  check(SV("answer is 'inf'"), SV("answer is '{:#g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:#g}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:#g}'"), nan_pos);
  check(SV("answer is '-nan'"), SV("answer is '{:#g}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '0.03125'"), SV("answer is '{:06g}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+06g}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+07g}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+08g}'"), 0.03125);

  check(SV("answer is '000.03125'"), SV("answer is '{:09g}'"), 0.03125);
  check(SV("answer is '000.03125'"), SV("answer is '{:-09g}'"), 0.03125);
  check(SV("answer is '+00.03125'"), SV("answer is '{:+09g}'"), 0.03125);
  check(SV("answer is ' 00.03125'"), SV("answer is '{: 09g}'"), 0.03125);

  check(SV("answer is '       inf'"), SV("answer is '{:010g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{:-010g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +inf'"), SV("answer is '{:+010g}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{: 010g}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -inf'"), SV("answer is '{:010g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:-010g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:+010g}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{: 010g}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       nan'"), SV("answer is '{:010g}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{:-010g}'"), nan_pos);
  check(SV("answer is '      +nan'"), SV("answer is '{:+010g}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{: 010g}'"), nan_pos);

  check(SV("answer is '      -nan'"), SV("answer is '{:010g}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:-010g}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:+010g}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{: 010g}'"), nan_neg);

  // *** precision ***
  check(SV("answer is '0.03'"), SV("answer is '{:.0g}'"), 0.03125);
  check(SV("answer is '0.03'"), SV("answer is '{:.1g}'"), 0.03125);
  check(SV("answer is '0.031'"), SV("answer is '{:.2g}'"), 0.03125);
  check(SV("answer is '0.0312'"), SV("answer is '{:.3g}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.4g}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.5g}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.10g}'"), 0.03125);

  // *** precision & alternate form ***

  // Output validated with  printf("%#xg")
  check(SV("answer is '1.'"), SV("answer is '{:#.{}g}'"), 1.2, 0);
  check(SV("answer is '1.'"), SV("answer is '{:#.{}g}'"), 1.2, 1);
  check(SV("answer is '1.2'"), SV("answer is '{:#.{}g}'"), 1.2, 2);
  check(SV("answer is '1.20'"), SV("answer is '{:#.{}g}'"), 1.2, 3);
  check(SV("answer is '1.200'"), SV("answer is '{:#.{}g}'"), 1.2, 4);
  check(SV("answer is '1.2000'"), SV("answer is '{:#.{}g}'"), 1.2, 5);
  check(SV("answer is '1.20000'"), SV("answer is '{:#.{}g}'"), 1.2, 6);

  check(SV("answer is '1.e+03'"), SV("answer is '{:#.{}g}'"), 1200.0, 0);
  check(SV("answer is '1.e+03'"), SV("answer is '{:#.{}g}'"), 1200.0, 1);
  check(SV("answer is '1.2e+03'"), SV("answer is '{:#.{}g}'"), 1200.0, 2);
  check(SV("answer is '1.20e+03'"), SV("answer is '{:#.{}g}'"), 1200.0, 3);
  check(SV("answer is '1200.'"), SV("answer is '{:#.{}g}'"), 1200.0, 4);
  check(SV("answer is '1200.0'"), SV("answer is '{:#.{}g}'"), 1200.0, 5);
  check(SV("answer is '1200.00'"), SV("answer is '{:#.{}g}'"), 1200.0, 6);

  check(SV("answer is '1.e+06'"), SV("answer is '{:#.{}g}'"), 1200000.0, 0);
  check(SV("answer is '1.e+06'"), SV("answer is '{:#.{}g}'"), 1200000.0, 1);
  check(SV("answer is '1.2e+06'"), SV("answer is '{:#.{}g}'"), 1200000.0, 2);
  check(SV("answer is '1.20e+06'"), SV("answer is '{:#.{}g}'"), 1200000.0, 3);
  check(SV("answer is '1.200e+06'"), SV("answer is '{:#.{}g}'"), 1200000.0, 4);
  check(SV("answer is '1.2000e+06'"), SV("answer is '{:#.{}g}'"), 1200000.0, 5);
  check(SV("answer is '1.20000e+06'"), SV("answer is '{:#.{}g}'"), 1200000.0, 6);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_general_upper_case(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   0.25'"), SV("answer is '{:7G}'"), F(0.25));
  check(SV("answer is '   0.25'"), SV("answer is '{:>7G}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<7G}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^7G}'"), F(0.25));

  check(SV("answer is '---0.125'"), SV("answer is '{:->8G}'"), F(125e-3));
  check(SV("answer is '0.125---'"), SV("answer is '{:-<8G}'"), F(125e-3));
  check(SV("answer is '-0.125--'"), SV("answer is '{:-^8G}'"), F(125e-3));

  check(SV("answer is '***INF'"), SV("answer is '{:*>6G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF***'"), SV("answer is '{:*<6G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*INF**'"), SV("answer is '{:*^6G}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-INF'"), SV("answer is '{:#>7G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF###'"), SV("answer is '{:#<7G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-INF##'"), SV("answer is '{:#^7G}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^NAN'"), SV("answer is '{:^>6G}'"), nan_pos);
  check(SV("answer is 'NAN^^^'"), SV("answer is '{:^<6G}'"), nan_pos);
  check(SV("answer is '^NAN^^'"), SV("answer is '{:^^6G}'"), nan_pos);

  check(SV("answer is '000-NAN'"), SV("answer is '{:0>7G}'"), nan_neg);
  check(SV("answer is '-NAN000'"), SV("answer is '{:0<7G}'"), nan_neg);
  check(SV("answer is '0-NAN00'"), SV("answer is '{:0^7G}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   0.25'"), SV("answer is '{:>07G}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<07G}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^07G}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0'"), SV("answer is '{:G}'"), F(0));
  check(SV("answer is '0'"), SV("answer is '{:-G}'"), F(0));
  check(SV("answer is '+0'"), SV("answer is '{:+G}'"), F(0));
  check(SV("answer is ' 0'"), SV("answer is '{: G}'"), F(0));

  check(SV("answer is '-0'"), SV("answer is '{:G}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:-G}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:+G}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{: G}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'INF'"), SV("answer is '{:G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'INF'"), SV("answer is '{:-G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+INF'"), SV("answer is '{:+G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' INF'"), SV("answer is '{: G}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-INF'"), SV("answer is '{:G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:-G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:+G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{: G}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:G}'"), nan_pos);
  check(SV("answer is 'NAN'"), SV("answer is '{:-G}'"), nan_pos);
  check(SV("answer is '+NAN'"), SV("answer is '{:+G}'"), nan_pos);
  check(SV("answer is ' NAN'"), SV("answer is '{: G}'"), nan_pos);

  check(SV("answer is '-NAN'"), SV("answer is '{:G}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:-G}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{:+G}'"), nan_neg);
  check(SV("answer is '-NAN'"), SV("answer is '{: G}'"), nan_neg);

  // *** alternate form **
  // When precision is zero there's no decimal point except when the alternate form is specified.
  check(SV("answer is '0'"), SV("answer is '{:.0G}'"), F(0));
  check(SV("answer is '0.'"), SV("answer is '{:#.0G}'"), F(0));

  check(SV("answer is '0.00000'"), SV("answer is '{:#G}'"), F(0));
  check(SV("answer is '2.50000'"), SV("answer is '{:#G}'"), F(2.5));

  check(SV("answer is 'INF'"), SV("answer is '{:#G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-INF'"), SV("answer is '{:#G}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'NAN'"), SV("answer is '{:#G}'"), nan_pos);
  check(SV("answer is '-NAN'"), SV("answer is '{:#G}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '0.03125'"), SV("answer is '{:06G}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+06G}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+07G}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+08G}'"), 0.03125);

  check(SV("answer is '000.03125'"), SV("answer is '{:09G}'"), 0.03125);
  check(SV("answer is '000.03125'"), SV("answer is '{:-09G}'"), 0.03125);
  check(SV("answer is '+00.03125'"), SV("answer is '{:+09G}'"), 0.03125);
  check(SV("answer is ' 00.03125'"), SV("answer is '{: 09G}'"), 0.03125);

  check(SV("answer is '       INF'"), SV("answer is '{:010G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{:-010G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +INF'"), SV("answer is '{:+010G}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       INF'"), SV("answer is '{: 010G}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -INF'"), SV("answer is '{:010G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:-010G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{:+010G}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -INF'"), SV("answer is '{: 010G}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       NAN'"), SV("answer is '{:010G}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{:-010G}'"), nan_pos);
  check(SV("answer is '      +NAN'"), SV("answer is '{:+010G}'"), nan_pos);
  check(SV("answer is '       NAN'"), SV("answer is '{: 010G}'"), nan_pos);

  check(SV("answer is '      -NAN'"), SV("answer is '{:010G}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:-010G}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{:+010G}'"), nan_neg);
  check(SV("answer is '      -NAN'"), SV("answer is '{: 010G}'"), nan_neg);

  // *** precision ***
  check(SV("answer is '0.03'"), SV("answer is '{:.0G}'"), 0.03125);
  check(SV("answer is '0.03'"), SV("answer is '{:.1G}'"), 0.03125);
  check(SV("answer is '0.031'"), SV("answer is '{:.2G}'"), 0.03125);
  check(SV("answer is '0.0312'"), SV("answer is '{:.3G}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.4G}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.5G}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.10G}'"), 0.03125);

  // *** precision & alternate form ***

  // Output validated with  printf("%#xg")
  check(SV("answer is '1.'"), SV("answer is '{:#.{}G}'"), 1.2, 0);
  check(SV("answer is '1.'"), SV("answer is '{:#.{}G}'"), 1.2, 1);
  check(SV("answer is '1.2'"), SV("answer is '{:#.{}G}'"), 1.2, 2);
  check(SV("answer is '1.20'"), SV("answer is '{:#.{}G}'"), 1.2, 3);
  check(SV("answer is '1.200'"), SV("answer is '{:#.{}G}'"), 1.2, 4);
  check(SV("answer is '1.2000'"), SV("answer is '{:#.{}G}'"), 1.2, 5);
  check(SV("answer is '1.20000'"), SV("answer is '{:#.{}G}'"), 1.2, 6);

  check(SV("answer is '1.E+03'"), SV("answer is '{:#.{}G}'"), 1200.0, 0);
  check(SV("answer is '1.E+03'"), SV("answer is '{:#.{}G}'"), 1200.0, 1);
  check(SV("answer is '1.2E+03'"), SV("answer is '{:#.{}G}'"), 1200.0, 2);
  check(SV("answer is '1.20E+03'"), SV("answer is '{:#.{}G}'"), 1200.0, 3);
  check(SV("answer is '1200.'"), SV("answer is '{:#.{}G}'"), 1200.0, 4);
  check(SV("answer is '1200.0'"), SV("answer is '{:#.{}G}'"), 1200.0, 5);
  check(SV("answer is '1200.00'"), SV("answer is '{:#.{}G}'"), 1200.0, 6);

  check(SV("answer is '1.E+06'"), SV("answer is '{:#.{}G}'"), 1200000.0, 0);
  check(SV("answer is '1.E+06'"), SV("answer is '{:#.{}G}'"), 1200000.0, 1);
  check(SV("answer is '1.2E+06'"), SV("answer is '{:#.{}G}'"), 1200000.0, 2);
  check(SV("answer is '1.20E+06'"), SV("answer is '{:#.{}G}'"), 1200000.0, 3);
  check(SV("answer is '1.200E+06'"), SV("answer is '{:#.{}G}'"), 1200000.0, 4);
  check(SV("answer is '1.2000E+06'"), SV("answer is '{:#.{}G}'"), 1200000.0, 5);
  check(SV("answer is '1.20000E+06'"), SV("answer is '{:#.{}G}'"), 1200000.0, 6);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_default(TestFunction check) {
  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   0.25'"), SV("answer is '{:7}'"), F(0.25));
  check(SV("answer is '   0.25'"), SV("answer is '{:>7}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<7}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^7}'"), F(0.25));

  check(SV("answer is '---0.125'"), SV("answer is '{:->8}'"), F(125e-3));
  check(SV("answer is '0.125---'"), SV("answer is '{:-<8}'"), F(125e-3));
  check(SV("answer is '-0.125--'"), SV("answer is '{:-^8}'"), F(125e-3));

  check(SV("answer is '***inf'"), SV("answer is '{:*>6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf***'"), SV("answer is '{:*<6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*inf**'"), SV("answer is '{:*^6}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-inf'"), SV("answer is '{:#>7}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf###'"), SV("answer is '{:#<7}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-inf##'"), SV("answer is '{:#^7}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^nan'"), SV("answer is '{:^>6}'"), nan_pos);
  check(SV("answer is 'nan^^^'"), SV("answer is '{:^<6}'"), nan_pos);
  check(SV("answer is '^nan^^'"), SV("answer is '{:^^6}'"), nan_pos);

  check(SV("answer is '000-nan'"), SV("answer is '{:0>7}'"), nan_neg);
  check(SV("answer is '-nan000'"), SV("answer is '{:0<7}'"), nan_neg);
  check(SV("answer is '0-nan00'"), SV("answer is '{:0^7}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   0.25'"), SV("answer is '{:>07}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<07}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^07}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0'"), SV("answer is '{:}'"), F(0));
  check(SV("answer is '0'"), SV("answer is '{:-}'"), F(0));
  check(SV("answer is '+0'"), SV("answer is '{:+}'"), F(0));
  check(SV("answer is ' 0'"), SV("answer is '{: }'"), F(0));

  check(SV("answer is '-0'"), SV("answer is '{:}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:-}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:+}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{: }'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'inf'"), SV("answer is '{:}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf'"), SV("answer is '{:-}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+inf'"), SV("answer is '{:+}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' inf'"), SV("answer is '{: }'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-inf'"), SV("answer is '{:}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:-}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:+}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{: }'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:}'"), nan_pos);
  check(SV("answer is 'nan'"), SV("answer is '{:-}'"), nan_pos);
  check(SV("answer is '+nan'"), SV("answer is '{:+}'"), nan_pos);
  check(SV("answer is ' nan'"), SV("answer is '{: }'"), nan_pos);

  check(SV("answer is '-nan'"), SV("answer is '{:}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:-}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:+}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{: }'"), nan_neg);

  // *** alternate form ***
  check(SV("answer is '0.'"), SV("answer is '{:#}'"), F(0));
  check(SV("answer is '2.5'"), SV("answer is '{:#}'"), F(2.5));

  check(SV("answer is 'inf'"), SV("answer is '{:#}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:#}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:#}'"), nan_pos);
  check(SV("answer is '-nan'"), SV("answer is '{:#}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '0.03125'"), SV("answer is '{:07}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+07}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+08}'"), 0.03125);
  check(SV("answer is '+00.03125'"), SV("answer is '{:+09}'"), 0.03125);

  check(SV("answer is '0000.03125'"), SV("answer is '{:010}'"), 0.03125);
  check(SV("answer is '0000.03125'"), SV("answer is '{:-010}'"), 0.03125);
  check(SV("answer is '+000.03125'"), SV("answer is '{:+010}'"), 0.03125);
  check(SV("answer is ' 000.03125'"), SV("answer is '{: 010}'"), 0.03125);

  check(SV("answer is '       inf'"), SV("answer is '{:010}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{:-010}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +inf'"), SV("answer is '{:+010}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{: 010}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -inf'"), SV("answer is '{:010}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:-010}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:+010}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{: 010}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       nan'"), SV("answer is '{:010}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{:-010}'"), nan_pos);
  check(SV("answer is '      +nan'"), SV("answer is '{:+010}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{: 010}'"), nan_pos);

  check(SV("answer is '      -nan'"), SV("answer is '{:010}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:-010}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:+010}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{: 010}'"), nan_neg);

  // *** precision ***
  // See format_test_floating_point_default_precision

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_default_precision(TestFunction check) {

  auto nan_pos = std::numeric_limits<F>::quiet_NaN();                          // "nan"
  auto nan_neg = std::copysign(nan_pos, static_cast<decltype(nan_pos)>(-1.0)); // "-nan"

  // *** align-fill & width ***
  check(SV("answer is '   0.25'"), SV("answer is '{:7.6}'"), F(0.25));
  check(SV("answer is '   0.25'"), SV("answer is '{:>7.6}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<7.6}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^7.6}'"), F(0.25));

  check(SV("answer is '---0.125'"), SV("answer is '{:->8.6}'"), F(125e-3));
  check(SV("answer is '0.125---'"), SV("answer is '{:-<8.6}'"), F(125e-3));
  check(SV("answer is '-0.125--'"), SV("answer is '{:-^8.6}'"), F(125e-3));

  check(SV("answer is '***inf'"), SV("answer is '{:*>6.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf***'"), SV("answer is '{:*<6.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '*inf**'"), SV("answer is '{:*^6.6}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '###-inf'"), SV("answer is '{:#>7.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf###'"), SV("answer is '{:#<7.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '#-inf##'"), SV("answer is '{:#^7.6}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '^^^nan'"), SV("answer is '{:^>6.6}'"), nan_pos);
  check(SV("answer is 'nan^^^'"), SV("answer is '{:^<6.6}'"), nan_pos);
  check(SV("answer is '^nan^^'"), SV("answer is '{:^^6.6}'"), nan_pos);

  check(SV("answer is '000-nan'"), SV("answer is '{:0>7.6}'"), nan_neg);
  check(SV("answer is '-nan000'"), SV("answer is '{:0<7.6}'"), nan_neg);
  check(SV("answer is '0-nan00'"), SV("answer is '{:0^7.6}'"), nan_neg);

  // Test whether zero padding is ignored
  check(SV("answer is '   0.25'"), SV("answer is '{:>07.6}'"), F(0.25));
  check(SV("answer is '0.25   '"), SV("answer is '{:<07.6}'"), F(0.25));
  check(SV("answer is ' 0.25  '"), SV("answer is '{:^07.6}'"), F(0.25));

  // *** Sign ***
  check(SV("answer is '0'"), SV("answer is '{:.6}'"), F(0));
  check(SV("answer is '0'"), SV("answer is '{:-.6}'"), F(0));
  check(SV("answer is '+0'"), SV("answer is '{:+.6}'"), F(0));
  check(SV("answer is ' 0'"), SV("answer is '{: .6}'"), F(0));

  check(SV("answer is '-0'"), SV("answer is '{:.6}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:-.6}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{:+.6}'"), F(-0.));
  check(SV("answer is '-0'"), SV("answer is '{: .6}'"), F(-0.));

  // [format.string.std]/5 The sign option applies to floating-point infinity and NaN.
  check(SV("answer is 'inf'"), SV("answer is '{:.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is 'inf'"), SV("answer is '{:-.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '+inf'"), SV("answer is '{:+.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is ' inf'"), SV("answer is '{: .6}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '-inf'"), SV("answer is '{:.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:-.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:+.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{: .6}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:.6}'"), nan_pos);
  check(SV("answer is 'nan'"), SV("answer is '{:-.6}'"), nan_pos);
  check(SV("answer is '+nan'"), SV("answer is '{:+.6}'"), nan_pos);
  check(SV("answer is ' nan'"), SV("answer is '{: .6}'"), nan_pos);

  check(SV("answer is '-nan'"), SV("answer is '{:.6}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:-.6}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{:+.6}'"), nan_neg);
  check(SV("answer is '-nan'"), SV("answer is '{: .6}'"), nan_neg);

  // *** alternate form **
  // When precision is zero there's no decimal point except when the alternate form is specified.
  // Note unlike the g and G option the trailing zeros are still removed.
  check(SV("answer is '0'"), SV("answer is '{:.0}'"), F(0));
  check(SV("answer is '0.'"), SV("answer is '{:#.0}'"), F(0));

  check(SV("answer is '0.'"), SV("answer is '{:#.6}'"), F(0));
  check(SV("answer is '2.5'"), SV("answer is '{:#.6}'"), F(2.5));

  check(SV("answer is 'inf'"), SV("answer is '{:#.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '-inf'"), SV("answer is '{:#.6}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is 'nan'"), SV("answer is '{:#.6}'"), nan_pos);
  check(SV("answer is '-nan'"), SV("answer is '{:#.6}'"), nan_neg);

  // *** zero-padding & width ***
  check(SV("answer is '0.03125'"), SV("answer is '{:06.6}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+06.6}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+07.6}'"), 0.03125);
  check(SV("answer is '+0.03125'"), SV("answer is '{:+08.6}'"), 0.03125);

  check(SV("answer is '000.03125'"), SV("answer is '{:09.6}'"), 0.03125);
  check(SV("answer is '000.03125'"), SV("answer is '{:-09.6}'"), 0.03125);
  check(SV("answer is '+00.03125'"), SV("answer is '{:+09.6}'"), 0.03125);
  check(SV("answer is ' 00.03125'"), SV("answer is '{: 09.6}'"), 0.03125);

  check(SV("answer is '       inf'"), SV("answer is '{:010.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{:-010.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '      +inf'"), SV("answer is '{:+010.6}'"), std::numeric_limits<F>::infinity());
  check(SV("answer is '       inf'"), SV("answer is '{: 010.6}'"), std::numeric_limits<F>::infinity());

  check(SV("answer is '      -inf'"), SV("answer is '{:010.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:-010.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{:+010.6}'"), -std::numeric_limits<F>::infinity());
  check(SV("answer is '      -inf'"), SV("answer is '{: 010.6}'"), -std::numeric_limits<F>::infinity());

  check(SV("answer is '       nan'"), SV("answer is '{:010.6}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{:-010.6}'"), nan_pos);
  check(SV("answer is '      +nan'"), SV("answer is '{:+010.6}'"), nan_pos);
  check(SV("answer is '       nan'"), SV("answer is '{: 010.6}'"), nan_pos);

  check(SV("answer is '      -nan'"), SV("answer is '{:010.6}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:-010.6}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{:+010.6}'"), nan_neg);
  check(SV("answer is '      -nan'"), SV("answer is '{: 010.6}'"), nan_neg);

  // *** precision ***
  check(SV("answer is '0.03'"), SV("answer is '{:.0}'"), 0.03125);
  check(SV("answer is '0.03'"), SV("answer is '{:.1}'"), 0.03125);
  check(SV("answer is '0.031'"), SV("answer is '{:.2}'"), 0.03125);
  check(SV("answer is '0.0312'"), SV("answer is '{:.3}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.4}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.5}'"), 0.03125);
  check(SV("answer is '0.03125'"), SV("answer is '{:.10}'"), 0.03125);

  // *** precision & alternate form ***

  check(SV("answer is '1.'"), SV("answer is '{:#.{}}'"), 1.2, 0);
  check(SV("answer is '1.'"), SV("answer is '{:#.{}}'"), 1.2, 1);
  check(SV("answer is '1.2'"), SV("answer is '{:#.{}}'"), 1.2, 2);
  check(SV("answer is '1.2'"), SV("answer is '{:#.{}}'"), 1.2, 3);
  check(SV("answer is '1.2'"), SV("answer is '{:#.{}}'"), 1.2, 4);
  check(SV("answer is '1.2'"), SV("answer is '{:#.{}}'"), 1.2, 5);
  check(SV("answer is '1.2'"), SV("answer is '{:#.{}}'"), 1.2, 6);

  check(SV("answer is '1.e+03'"), SV("answer is '{:#.{}}'"), 1200.0, 0);
  check(SV("answer is '1.e+03'"), SV("answer is '{:#.{}}'"), 1200.0, 1);
  check(SV("answer is '1.2e+03'"), SV("answer is '{:#.{}}'"), 1200.0, 2);
  check(SV("answer is '1.2e+03'"), SV("answer is '{:#.{}}'"), 1200.0, 3);
  check(SV("answer is '1200.'"), SV("answer is '{:#.{}}'"), 1200.0, 4);
  check(SV("answer is '1200.'"), SV("answer is '{:#.{}}'"), 1200.0, 5);
  check(SV("answer is '1200.'"), SV("answer is '{:#.{}}'"), 1200.0, 6);

  check(SV("answer is '1.e+06'"), SV("answer is '{:#.{}}'"), 1200000.0, 0);
  check(SV("answer is '1.e+06'"), SV("answer is '{:#.{}}'"), 1200000.0, 1);
  check(SV("answer is '1.2e+06'"), SV("answer is '{:#.{}}'"), 1200000.0, 2);
  check(SV("answer is '1.2e+06'"), SV("answer is '{:#.{}}'"), 1200000.0, 3);
  check(SV("answer is '1.2e+06'"), SV("answer is '{:#.{}}'"), 1200000.0, 4);
  check(SV("answer is '1.2e+06'"), SV("answer is '{:#.{}}'"), 1200000.0, 5);
  check(SV("answer is '1.2e+06'"), SV("answer is '{:#.{}}'"), 1200000.0, 6);

  // *** locale-specific form ***
  // See locale-specific_form.pass.cpp
}

template <class F, class CharT, class TestFunction>
void format_test_floating_point_PR58714(TestFunction check) {
  check(SV("+1234"), SV("{:+}"), F(1234.0));
  check(SV("+1.348p+10"), SV("{:+a}"), F(1234.0));
  check(SV("+1.234000e+03"), SV("{:+e}"), F(1234.0));
  check(SV("+1234.000000"), SV("{:+f}"), F(1234.0));
  check(SV("+1234"), SV("{:+g}"), F(1234.0));

  check(SV("1234."), SV("{:#}"), F(1234.0));
  check(SV("1.348p+10"), SV("{:#a}"), F(1234.0));
  check(SV("1.234000e+03"), SV("{:#e}"), F(1234.0));
  check(SV("1234.000000"), SV("{:#f}"), F(1234.0));
  check(SV("1234.00"), SV("{:#g}"), F(1234.0));

  check(SV("4.e+30"), SV("{:#}"), F(4.0e+30));
  check(SV("1.p+102"), SV("{:#a}"), F(0x4.0p+100));
  check(SV("4.000000e+30"), SV("{:#e}"), F(4.0e+30));
  check(SV("5070602400912917605986812821504.000000"), SV("{:#f}"), F(0x4.0p+100));
  check(SV("4.00000e+30"), SV("{:#g}"), F(4.0e+30));

  check(SV("1234."), SV("{:#.6}"), F(1234.0)); // # does not restore zeros
  check(SV("1.348000p+10"), SV("{:#.6a}"), F(1234.0));
  check(SV("1.234000e+03"), SV("{:#.6e}"), F(1234.0));
  check(SV("1234.000000"), SV("{:#.6f}"), F(1234.0));
  check(SV("1234.00"), SV("{:#.6g}"), F(1234.0));

  check(SV("-1234."), SV("{:#}"), F(-1234.0));
  check(SV("-1.348p+10"), SV("{:#a}"), F(-1234.0));
  check(SV("-1.234000e+03"), SV("{:#e}"), F(-1234.0));
  check(SV("-1234.000000"), SV("{:#f}"), F(-1234.0));
  check(SV("-1234.00"), SV("{:#g}"), F(-1234.0));

  check(SV("-1234."), SV("{:#.6}"), F(-1234.0)); // # does not restore zeros
  check(SV("-1.348000p+10"), SV("{:#.6a}"), F(-1234.0));
  check(SV("-1.234000e+03"), SV("{:#.6e}"), F(-1234.0));
  check(SV("-1234.000000"), SV("{:#.6f}"), F(-1234.0));
  check(SV("-1234.00"), SV("{:#.6g}"), F(-1234.0));

  check(SV("+1234."), SV("{:+#}"), F(1234.0));
  check(SV("+1.348p+10"), SV("{:+#a}"), F(1234.0));
  check(SV("+1.234000e+03"), SV("{:+#e}"), F(1234.0));
  check(SV("+1234.000000"), SV("{:+#f}"), F(1234.0));
  check(SV("+1234.00"), SV("{:+#g}"), F(1234.0));

  check(SV("+1234."), SV("{:+#.6}"), F(1234.0)); // # does not restore zeros
  check(SV("+1.348000p+10"), SV("{:+#.6a}"), F(1234.0));
  check(SV("+1.234000e+03"), SV("{:+#.6e}"), F(1234.0));
  check(SV("+1234.000000"), SV("{:+#.6f}"), F(1234.0));
  check(SV("+1234.00"), SV("{:+#.6g}"), F(1234.0));
}

template <class F, class CharT, class TestFunction, class ExceptionTest>
void format_test_floating_point(TestFunction check, ExceptionTest check_exception) {
  format_test_floating_point_hex_lower_case<F, CharT>(check);
  format_test_floating_point_hex_upper_case<F, CharT>(check);
  format_test_floating_point_hex_lower_case_precision<F, CharT>(check);
  format_test_floating_point_hex_upper_case_precision<F, CharT>(check);

  format_test_floating_point_scientific_lower_case<F, CharT>(check);
  format_test_floating_point_scientific_upper_case<F, CharT>(check);

  format_test_floating_point_fixed_lower_case<F, CharT>(check);
  format_test_floating_point_fixed_upper_case<F, CharT>(check);

  format_test_floating_point_general_lower_case<F, CharT>(check);
  format_test_floating_point_general_upper_case<F, CharT>(check);

  format_test_floating_point_default<F, CharT>(check);
  format_test_floating_point_default_precision<F, CharT>(check);

  format_test_floating_point_PR58714<F, CharT>(check);

  // *** type ***
  for (const auto& fmt : invalid_types<CharT>("aAeEfFgG"))
    check_exception("The type option contains an invalid value for a floating-point formatting argument", fmt, F(1));
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_floating_point(TestFunction check, ExceptionTest check_exception) {
  format_test_floating_point<float, CharT>(check, check_exception);
  format_test_floating_point<double, CharT>(check, check_exception);
  format_test_floating_point<long double, CharT>(check, check_exception);
}

template <class P, class CharT, class TestFunction, class ExceptionTest>
void format_test_pointer(TestFunction check, ExceptionTest check_exception) {
  // *** align-fill & width ***
  check(SV("answer is '   0x0'"), SV("answer is '{:6}'"), P(nullptr));
  check(SV("answer is '   0x0'"), SV("answer is '{:>6}'"), P(nullptr));
  check(SV("answer is '0x0   '"), SV("answer is '{:<6}'"), P(nullptr));
  check(SV("answer is ' 0x0  '"), SV("answer is '{:^6}'"), P(nullptr));

  // The fill character ':' is allowed here (P0645) but not in ranges (P2286).
  check(SV("answer is ':::0x0'"), SV("answer is '{::>6}'"), P(nullptr));
  check(SV("answer is '0x0:::'"), SV("answer is '{::<6}'"), P(nullptr));
  check(SV("answer is ':0x0::'"), SV("answer is '{::^6}'"), P(nullptr));

  // Test whether zero padding is ignored
  check(SV("answer is ':::0x0'"), SV("answer is '{::>06}'"), P(nullptr));
  check(SV("answer is '0x0:::'"), SV("answer is '{::<06}'"), P(nullptr));
  check(SV("answer is ':0x0::'"), SV("answer is '{::^06}'"), P(nullptr));

  // *** Sign ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:-}"), P(nullptr));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:+}"), P(nullptr));
  check_exception("The format specifier should consume the input or end with a '}'", SV("{: }"), P(nullptr));

  // *** alternate form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:#}"), P(nullptr));

  // *** zero-padding ***
  check(SV("answer is '0x0000'"), SV("answer is '{:06}'"), P(nullptr));
  check(SV("answer is '0x0000'"), SV("answer is '{:06p}'"), P(nullptr));
  check(SV("answer is '0X0000'"), SV("answer is '{:06P}'"), P(nullptr));

  // *** precision ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.}"), nullptr);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.0}"), nullptr);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.42}"), nullptr);

  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}}"), nullptr);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}}"), nullptr, true);
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:.{}}"), nullptr, 1.0);

  // *** locale-specific form ***
  check_exception("The format specifier should consume the input or end with a '}'", SV("{:L}"), P(nullptr));

  // *** type ***
  for (const auto& fmt : invalid_types<CharT>("pP"))
    check_exception("The type option contains an invalid value for a pointer formatting argument", fmt, P(nullptr));
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_handle(TestFunction check, ExceptionTest check_exception) {
  // *** Valid permutations ***
  check(SV("answer is '0xaaaa'"), SV("answer is '{}'"), status::foo);
  check(SV("answer is '0xaaaa'"), SV("answer is '{:x}'"), status::foo);
  check(SV("answer is '0XAAAA'"), SV("answer is '{:X}'"), status::foo);
  check(SV("answer is 'foo'"), SV("answer is '{:s}'"), status::foo);

  check(SV("answer is '0x5555'"), SV("answer is '{}'"), status::bar);
  check(SV("answer is '0x5555'"), SV("answer is '{:x}'"), status::bar);
  check(SV("answer is '0X5555'"), SV("answer is '{:X}'"), status::bar);
  check(SV("answer is 'bar'"), SV("answer is '{:s}'"), status::bar);

  check(SV("answer is '0xaa55'"), SV("answer is '{}'"), status::foobar);
  check(SV("answer is '0xaa55'"), SV("answer is '{:x}'"), status::foobar);
  check(SV("answer is '0XAA55'"), SV("answer is '{:X}'"), status::foobar);
  check(SV("answer is 'foobar'"), SV("answer is '{:s}'"), status::foobar);

  // P2418 Changed the argument from a const reference to a forwarding reference.
  // This mainly affects handle classes, however since we use an abstraction
  // layer here it's "tricky" to verify whether this test would do the "right"
  // thing. So these tests are done separately.

  // *** type ***
  for (const auto& fmt : invalid_types<CharT>("xXs"))
    check_exception("The type option contains an invalid value for a status formatting argument", fmt, status::foo);
}

template <class CharT, class TestFunction, class ExceptionTest>
void format_test_pointer(TestFunction check, ExceptionTest check_exception) {
  format_test_pointer<std::nullptr_t, CharT>(check, check_exception);
  format_test_pointer<void*, CharT>(check, check_exception);
  format_test_pointer<const void*, CharT>(check, check_exception);
}

/// Tests special buffer functions with a "large" input.
///
/// This is a test specific for libc++, however the code should behave the same
/// on all implementations.
/// In \c __format::__output_buffer there are some special functions to optimize
/// outputting multiple characters, \c __copy, \c __transform, \c __fill. This
/// test validates whether the functions behave properly when the output size
/// doesn't fit in its internal buffer.
template <class CharT, class TestFunction>
void format_test_buffer_optimizations(TestFunction check) {
#ifdef _LIBCPP_VERSION
  // Used to validate our test sets are the proper size.
  // To test the chunked operations it needs to be larger than the internal
  // buffer. Picked a nice looking number.
  constexpr int minimum = 3 * 256;
#else
  constexpr int minimum = 1;
#endif

  // Copy
  std::basic_string<CharT> str = STR(
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog."
      "The quick brown fox jumps over the lazy dog.");
  assert(str.size() > minimum);
  check(std::basic_string_view<CharT>{str}, SV("{}"), str);

  // Fill
  std::basic_string<CharT> fill(minimum, CharT('*'));
  check(std::basic_string_view<CharT>{str + fill}, SV("{:*<{}}"), str, str.size() + minimum);
  check(std::basic_string_view<CharT>{fill + str + fill}, SV("{:*^{}}"), str, minimum + str.size() + minimum);
  check(std::basic_string_view<CharT>{fill + str}, SV("{:*>{}}"), str, minimum + str.size());
}

template <class CharT, execution_modus modus, class TestFunction, class ExceptionTest>
void format_tests(TestFunction check, ExceptionTest check_exception) {
  // *** Test escaping  ***
  check(SV("{"), SV("{{"));
  check(SV("}"), SV("}}"));
  check(SV("{:^}"), SV("{{:^}}"));
  check(SV("{: ^}"), SV("{{:{}^}}"), CharT(' '));
  check(SV("{:{}^}"), SV("{{:{{}}^}}"));
  check(SV("{:{ }^}"), SV("{{:{{{}}}^}}"), CharT(' '));

  // *** Test argument ID ***
  check(SV("hello false true"), SV("hello {0:} {1:}"), false, true);
  check(SV("hello true false"), SV("hello {1:} {0:}"), false, true);

  // *** Test many arguments ***

  // [format.args]/1
  // An instance of basic_format_args provides access to formatting arguments.
  // Implementations should optimize the representation of basic_format_args
  // for a small number of formatting arguments.
  //
  // These's no guidances what "a small number of formatting arguments" is.
  // - fmtlib uses a 15 elements
  // - libc++ uses 12 elements
  // - MSVC STL uses a different approach regardless of the number of arguments
  // - libstdc++ has no implementation yet
  // fmtlib and libc++ use a similar approach, this approach can support 16
  // elements (based on design choices both support less elements). This test
  // makes sure "the large number of formatting arguments" code path is tested.
  check(
      SV("1234567890\t1234567890"),
      SV("{}{}{}{}{}{}{}{}{}{}\t{}{}{}{}{}{}{}{}{}{}"),
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

  // *** Test buffer boundaries format strings ***
  if constexpr (modus == execution_modus::full) {
    format_test_buffer_copy<CharT>(check);
    format_test_buffer_full<CharT>(check);
  }

  // *** Test invalid format strings ***
  check_exception("The format string terminates at a '{'", SV("{"));
  check_exception("The argument index value is too large for the number of arguments supplied", SV("{:"));
  check_exception("The replacement field misses a terminating '}'", SV("{:"), 42);

  check_exception("The argument index should end with a ':' or a '}'", SV("{0"));
  check_exception("The argument index value is too large for the number of arguments supplied", SV("{0:"));
  check_exception("The replacement field misses a terminating '}'", SV("{0:"), 42);

  check_exception("The format string contains an invalid escape sequence", SV("}"));
  check_exception("The format string contains an invalid escape sequence", SV("{:}-}"), 42);

  check_exception("The format string contains an invalid escape sequence", SV("} "));
  check_exception("The argument index starts with an invalid character", SV("{-"), 42);
  check_exception("The argument index value is too large for the number of arguments supplied", SV("hello {}"));
  check_exception("The argument index value is too large for the number of arguments supplied", SV("hello {0}"));
  check_exception("The argument index value is too large for the number of arguments supplied", SV("hello {1}"), 42);

  // *** Test char format argument ***
  // The `char` to `wchar_t` formatting is tested separately.
  check(SV("hello 09azAZ!"),
        SV("hello {}{}{}{}{}{}{}"),
        CharT('0'),
        CharT('9'),
        CharT('a'),
        CharT('z'),
        CharT('A'),
        CharT('Z'),
        CharT('!'));
  if constexpr (modus == execution_modus::full) {
    format_test_char<CharT>(check, check_exception);
    format_test_char_as_integer<CharT>(check, check_exception);
  }

  // *** Test string format argument ***
  {
    CharT buffer[] = {CharT('0'), CharT('9'), CharT('a'), CharT('z'), CharT('A'), CharT('Z'), CharT('!'), 0};
    CharT* data = buffer;
    check(SV("hello 09azAZ!"), SV("hello {}"), data);
  }
  {
    CharT buffer[] = {CharT('0'), CharT('9'), CharT('a'), CharT('z'), CharT('A'), CharT('Z'), CharT('!'), 0};
    const CharT* data = buffer;
    check(SV("hello 09azAZ!"), SV("hello {}"), data);
  }
  {
    std::basic_string<CharT> data = STR("world");
    check(SV("hello world"), SV("hello {}"), data);
  }
  {
    std::basic_string<CharT> buffer = STR("world");
    std::basic_string_view<CharT> data = buffer;
    check(SV("hello world"), SV("hello {}"), data);
  }
  if constexpr (modus == execution_modus::full)
    format_string_tests<CharT>(check, check_exception);

  // *** Test Boolean format argument ***
  check(SV("hello false true"), SV("hello {} {}"), false, true);

  if constexpr (modus == execution_modus::full) {
    format_test_bool<CharT>(check, check_exception);
    format_test_bool_as_integer<CharT>(check, check_exception);
  }
  // *** Test signed integral format argument ***
  check(SV("hello 42"), SV("hello {}"), static_cast<signed char>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<short>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<int>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<long>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<long long>(42));
#ifndef TEST_HAS_NO_INT128
  check(SV("hello 42"), SV("hello {}"), static_cast<__int128_t>(42));
#endif

  if constexpr (modus == execution_modus::full)
    format_test_signed_integer<CharT>(check, check_exception);

  // ** Test unsigned integral format argument ***
  check(SV("hello 42"), SV("hello {}"), static_cast<unsigned char>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<unsigned short>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<unsigned>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<unsigned long>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<unsigned long long>(42));
#ifndef TEST_HAS_NO_INT128
  check(SV("hello 42"), SV("hello {}"), static_cast<__uint128_t>(42));
#endif
  if constexpr (modus == execution_modus::full)
    format_test_unsigned_integer<CharT>(check, check_exception);

  // *** Test floating point format argument ***
  check(SV("hello 42"), SV("hello {}"), static_cast<float>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<double>(42));
  check(SV("hello 42"), SV("hello {}"), static_cast<long double>(42));
  if constexpr (modus == execution_modus::full)
    format_test_floating_point<CharT>(check, check_exception);

  // *** Test pointer formatter argument ***
  check(SV("hello 0x0"), SV("hello {}"), nullptr);
  check(SV("hello 0x42"), SV("hello {}"), reinterpret_cast<void*>(0x42));
  check(SV("hello 0x42"), SV("hello {}"), reinterpret_cast<const void*>(0x42));
  if constexpr (modus == execution_modus::full)
    format_test_pointer<CharT>(check, check_exception);

  // *** Test handle formatter argument ***
  format_test_handle<CharT>(check, check_exception);

  // *** Test the internal buffer optimizations ***
  if constexpr (modus == execution_modus::full)
    format_test_buffer_optimizations<CharT>(check);
}

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
template <class TestFunction>
void format_tests_char_to_wchar_t(TestFunction check) {
  using CharT = wchar_t;
  check(SV("hello 09azA"), SV("hello {}{}{}{}{}"), '0', '9', 'a', 'z', 'A');
}
#endif

#endif // TEST_STD_UTILITIES_FORMAT_FORMAT_FUNCTIONS_FORMAT_TESTS_H
