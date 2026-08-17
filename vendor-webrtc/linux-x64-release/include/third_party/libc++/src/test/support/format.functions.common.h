//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_FORMAT_FUNCTIONS_COMMON_H
#define TEST_SUPPORT_FORMAT_FUNCTIONS_COMMON_H

// Contains the common part of the formatter tests for different papers.

#include <algorithm>
#include <cctype>
#include <charconv>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <format>
#include <ranges>
#include <string_view>
#include <string>
#include <vector>

#include "make_string.h"

#define STR(S) MAKE_STRING(CharT, S)
#define SV(S) MAKE_STRING_VIEW(CharT, S)
#define CSTR(S) MAKE_CSTRING(CharT, S)

template <class T>
struct context {};

template <>
struct context<char> {
  using type = std::format_context;
};

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
template <>
struct context<wchar_t> {
  using type = std::wformat_context;
};
#endif

template <class T>
using context_t = typename context<T>::type;

// A user-defined type used to test the handle formatter.
enum class status : std::uint16_t { foo = 0xAAAA, bar = 0x5555, foobar = 0xAA55 };

// The formatter for a user-defined type used to test the handle formatter.
template <class CharT>
struct std::formatter<status, CharT> {
  // During the 2023 Issaquah meeting LEWG made it clear a formatter is
  // required to call its parse function. LWG3892 Adds the wording for that
  // requirement. Therefore this formatter is initialized in an invalid state.
  // A call to parse sets it in a valid state and a call to format validates
  // the state.
  int type = -1;

  constexpr auto parse(basic_format_parse_context<CharT>& parse_ctx) -> decltype(parse_ctx.begin()) {
    auto begin = parse_ctx.begin();
    auto end   = parse_ctx.end();
    type       = 0;
    if (begin == end)
      return begin;

    switch (*begin) {
    case CharT('x'):
      break;
    case CharT('X'):
      type = 1;
      break;
    case CharT('s'):
      type = 2;
      break;
    case CharT('}'):
      return begin;
    default:
      throw_format_error("The type option contains an invalid value for a status formatting argument");
    }

    ++begin;
    if (begin != end && *begin != CharT('}'))
      throw_format_error("The format specifier should consume the input or end with a '}'");

    return begin;
  }

  template <class Out>
  auto format(status s, basic_format_context<Out, CharT>& ctx) const -> decltype(ctx.out()) {
    const char* names[] = {"foo", "bar", "foobar"};
    char buffer[7];
    const char* begin = names[0];
    const char* end = names[0];
    switch (type) {
    case -1:
      throw_format_error("The formatter's parse function has not been called.");

    case 0:
      begin = buffer;
      buffer[0] = '0';
      buffer[1] = 'x';
      end = std::to_chars(&buffer[2], std::end(buffer), static_cast<std::uint16_t>(s), 16).ptr;
      buffer[6] = '\0';
      break;

    case 1:
      begin = buffer;
      buffer[0] = '0';
      buffer[1] = 'X';
      end = std::to_chars(&buffer[2], std::end(buffer), static_cast<std::uint16_t>(s), 16).ptr;
      std::transform(static_cast<const char*>(&buffer[2]), end, &buffer[2], [](char c) {
        return static_cast<char>(std::toupper(c)); });
      buffer[6] = '\0';
      break;

    case 2:
      switch (s) {
      case status::foo:
        begin = names[0];
        break;
      case status::bar:
        begin = names[1];
        break;
      case status::foobar:
        begin = names[2];
        break;
      }
      end = begin + strlen(begin);
      break;
    }

    return std::copy(begin, end, ctx.out());
  }

private:
  [[noreturn]] void throw_format_error([[maybe_unused]] const char* s) const {
#ifndef TEST_HAS_NO_EXCEPTIONS
    throw std::format_error(s);
#else
    std::abort();
#endif
  }
};

struct parse_call_validator {
  struct parse_function_not_called {};

  friend constexpr auto operator<=>(const parse_call_validator& lhs, const parse_call_validator& rhs) {
    return &lhs <=> &rhs;
  }
};

// The formatter for a user-defined type used to test the handle formatter.
//
// Like std::formatter<status, CharT> this formatter validates that parse is
// called. This formatter is intended to be used when the formatter's parse is
// called directly and not with format. In that case the format-spec does not
// require a terminating }. The tests must be written in a fashion where this
// formatter is always called with an empty format-spec. This requirement
// allows testing of certain code paths that are never reached by using a
// well-formed format-string in the format functions.
template <class CharT>
struct std::formatter<parse_call_validator, CharT> {
  bool parse_called{false};

  constexpr auto parse(basic_format_parse_context<CharT>& parse_ctx) -> decltype(parse_ctx.begin()) {
    auto begin = parse_ctx.begin();
    auto end   = parse_ctx.end();
    assert(begin == end);
    parse_called = true;
    return begin;
  }

  auto format(parse_call_validator, auto& ctx) const -> decltype(ctx.out()) {
    if (!parse_called)
      throw_error<parse_call_validator::parse_function_not_called>();
    return ctx.out();
  }

private:
  template <class T>
  [[noreturn]] void throw_error() const {
#ifndef TEST_HAS_NO_EXCEPTIONS
    throw T{};
#else
    std::abort();
#endif
  }
};

// Creates format string for the invalid types.
//
// valid contains a list of types that are valid.
// - The type ?s is the only type requiring 2 characters, use S for that type.
// - Whether n is a type or not depends on the context, is is always used.
//
// The return value is a collection of basic_strings, instead of
// basic_string_views since the values are temporaries.
namespace detail {
template <class CharT, std::size_t N>
std::basic_string<CharT> get_colons() {
  static std::basic_string<CharT> result(N, CharT(':'));
  return result;
}

constexpr std::string_view get_format_types() {
  return "aAbBcdeEfFgGopPsxX"
#if TEST_STD_VER > 20
         "?"
#endif
      ;
}

template <class CharT, /*format_types types,*/ size_t N>
std::vector<std::basic_string<CharT>> fmt_invalid_types(std::string_view valid) {
  // std::ranges::to is not available in C++20.
  std::vector<std::basic_string<CharT>> result;
  std::ranges::copy(
      get_format_types() | std::views::filter([&](char type) { return valid.find(type) == std::string_view::npos; }) |
          std::views::transform([&](char type) { return std::format(SV("{{{}{}}}"), get_colons<CharT, N>(), type); }),
      std::back_inserter(result));
  return result;
}

} // namespace detail

// Creates format string for the invalid types.
//
// valid contains a list of types that are valid.
//
// The return value is a collection of basic_strings, instead of
// basic_string_views since the values are temporaries.
template <class CharT>
std::vector<std::basic_string<CharT>> fmt_invalid_types(std::string_view valid) {
  return detail::fmt_invalid_types<CharT, 1>(valid);
}

// Like fmt_invalid_types but when the format spec is for an underlying formatter.
template <class CharT>
std::vector<std::basic_string<CharT>> fmt_invalid_nested_types(std::string_view valid) {
  return detail::fmt_invalid_types<CharT, 2>(valid);
}

#endif // TEST_SUPPORT_FORMAT_FUNCTIONS_COMMON_H
