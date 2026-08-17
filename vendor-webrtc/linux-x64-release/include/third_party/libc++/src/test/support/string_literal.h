//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_TEST_STRING_LITERAL_H
#define SUPPORT_TEST_STRING_LITERAL_H

#include "test_macros.h"

#include <algorithm>
#include <concepts>
#include <string_view>

#if TEST_STD_VER > 17

/// Helper class to "transfer" a string literal
///
/// The MAKE_STRING helper macros turn a string literal into a const char*.
/// This is an issue when testing std::format; its format-string needs a string
/// literal for compile-time validation. This class does the job.
///
/// \note The class assumes a wchar_t can be initialized from a char.
/// \note All members are public to avoid compilation errors.
template <std::size_t N>
struct string_literal {
  consteval /*implicit*/ string_literal(const char (&str)[N + 1]) {
    std::copy_n(str, N + 1, data_);
#  ifndef TEST_HAS_NO_WIDE_CHARACTERS
    std::copy_n(str, N + 1, wdata_);
#  endif
  }

  template <class CharT>
  consteval std::basic_string_view<CharT> sv() const {
    if constexpr (std::same_as<CharT, char>)
      return std::basic_string_view{data_};
#  ifndef TEST_HAS_NO_WIDE_CHARACTERS
    else
      return std::basic_string_view{wdata_};
#  endif
  }

  char data_[N + 1];
#  ifndef TEST_HAS_NO_WIDE_CHARACTERS
  wchar_t wdata_[N + 1];
#  endif
};

template <std::size_t N>
string_literal(const char (&str)[N]) -> string_literal<N - 1>;

#endif // TEST_STD_VER > 17

#endif // SUPPORT_TEST_STRING_LITERAL_H
